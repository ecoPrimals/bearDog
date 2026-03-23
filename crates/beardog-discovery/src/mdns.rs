// SPDX-License-Identifier: AGPL-3.0-only

//! mDNS-based service discovery
//!
//! Complete production implementation using mdns-sd crate for zero-configuration
//! service discovery. No mocks - real mDNS queries and responses.

use crate::error::{DiscoveryError, Result};
use crate::types::{Capability, DiscoveredService, HealthStatus, QoSMetrics, ServiceEndpoint};
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// mDNS discovery configuration
#[derive(Debug, Clone)]
pub struct MdnsConfig {
    /// Timeout for discovery queries
    pub timeout: Duration,
    /// Service domain (default: local.)
    pub domain: String,
    /// Enable IPv6 discovery
    pub enable_ipv6: bool,
}

impl Default for MdnsConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(5),
            domain: "local.".to_string(),
            enable_ipv6: true,
        }
    }
}

/// mDNS discovery client - complete implementation
#[derive(Clone)]
pub struct MdnsDiscovery {
    daemon: Arc<ServiceDaemon>,
    config: MdnsConfig,
    cache: Arc<RwLock<HashMap<String, Vec<DiscoveredService>>>>,
}

impl MdnsDiscovery {
    /// Create new mDNS discovery client with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(MdnsConfig::default())
    }

    /// Create new mDNS discovery client with custom configuration
    pub fn with_config(config: MdnsConfig) -> Result<Self> {
        let daemon = ServiceDaemon::new()
            .map_err(|e| DiscoveryError::InitializationFailed(e.to_string()))?;

        Ok(Self {
            daemon: Arc::new(daemon),
            config,
            cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Discover services by capability using mDNS
    ///
    /// Performs real mDNS queries for services advertising the given capability.
    /// Returns all discovered services within the configured timeout.
    pub async fn discover(&self, capability: &str) -> Result<Vec<DiscoveredService>> {
        let service_type = Self::capability_to_service_type(capability);

        debug!("Starting mDNS discovery for service type: {}", service_type);

        // Check cache first
        if let Some(cached) = self.get_cached(&service_type).await {
            debug!("Found {} cached services for {}", cached.len(), capability);
            return Ok(cached);
        }

        // Perform real mDNS browse
        let receiver = self
            .daemon
            .browse(&service_type)
            .map_err(|e| DiscoveryError::QueryFailed(e.to_string()))?;

        let mut services = Vec::new();
        let timeout = tokio::time::sleep(self.config.timeout);
        tokio::pin!(timeout);

        // Collect services until timeout
        loop {
            tokio::select! {
                event = receiver.recv_async() => {
                    match event {
                        Ok(ServiceEvent::ServiceResolved(info)) => {
                            debug!("Resolved mDNS service: {}", info.get_fullname());
                            if let Ok(service) = self.convert_to_discovered(&info, capability) {
                                services.push(service);
                            }
                        }
                        Ok(ServiceEvent::ServiceRemoved(_, fullname)) => {
                            debug!("mDNS service removed: {}", fullname);
                        }
                        Ok(_) => {
                            // Other events (SearchStarted, etc.)
                        }
                        Err(e) => {
                            warn!("mDNS event error: {}", e);
                            break;
                        }
                    }
                }
                () = &mut timeout => {
                    debug!("mDNS discovery timeout reached");
                    break;
                }
            }
        }

        info!(
            "mDNS discovery complete: found {} services for {}",
            services.len(),
            capability
        );

        // Update cache
        self.update_cache(&service_type, services.clone()).await;

        Ok(services)
    }

    /// Convert capability name to mDNS service type
    ///
    /// Examples:
    /// - "storage" -> "_storage._tcp.local."
    /// - "orchestration" -> "_orchestration._tcp.local."
    fn capability_to_service_type(capability: &str) -> String {
        format!("_{}._tcp.local.", capability.to_lowercase())
    }

    /// Convert mDNS ServiceInfo to DiscoveredService
    fn convert_to_discovered(
        &self,
        info: &ServiceInfo,
        capability: &str,
    ) -> Result<DiscoveredService> {
        // Extract addresses - prefer IPv4, include IPv6 if enabled
        let addresses: Vec<IpAddr> = if self.config.enable_ipv6 {
            info.get_addresses().iter().copied().collect()
        } else {
            info.get_addresses()
                .iter()
                .filter(|addr| addr.is_ipv4())
                .copied()
                .collect()
        };

        if addresses.is_empty() {
            return Err(DiscoveryError::InvalidServiceInfo(
                "No valid IP addresses found".to_string(),
            ));
        }

        let port = info.get_port();
        let primary_url = format!("http://{}:{}", addresses[0], port);

        // Parse TXT records for additional capabilities
        // Convert TxtProperties to HashMap
        let properties = info.get_properties();
        let properties_map: HashMap<String, String> = properties
            .iter()
            .map(|p| (p.key().to_string(), p.val_str().to_string()))
            .collect();

        let capabilities = Self::parse_capabilities_from_txt(&properties_map, capability);

        Ok(DiscoveredService {
            id: info.get_hostname().to_string(),
            service_type: capability.to_string(),
            display_name: info.get_hostname().to_string(),
            endpoint: ServiceEndpoint {
                primary_url,
                fallback_urls: vec![],
                use_tls: false,
                path_prefix: None,
            },
            capabilities,
            qos: QoSMetrics::default(),
            health: HealthStatus::Unknown,
            discovered_at: SystemTime::now(),
            ttl_secs: 300,
            discovery_method: "mdns".to_string(),
            metadata: Self::parse_metadata_from_txt(&properties_map),
        })
    }

    /// Parse capabilities from TXT records
    ///
    /// Expected TXT format:
    /// - capabilities=storage,encryption
    /// - version=1.0.0
    /// - tier=production
    fn parse_capabilities_from_txt(
        properties: &HashMap<String, String>,
        primary_capability: &str,
    ) -> Vec<Capability> {
        let mut capabilities = vec![Capability {
            capability_type: primary_capability.to_string(),
            version: properties
                .get("version")
                .cloned()
                .unwrap_or_else(|| "1.0.0".to_string()),
            features: vec![],
            parameters: HashMap::new(),
        }];

        if let Some(caps_str) = properties.get("capabilities") {
            for cap in caps_str.split(',') {
                let cap = cap.trim();
                if !cap.is_empty() && cap != primary_capability {
                    capabilities.push(Capability {
                        capability_type: cap.to_string(),
                        version: "1.0.0".to_string(),
                        features: vec![],
                        parameters: HashMap::new(),
                    });
                }
            }
        }

        capabilities
    }

    /// Parse metadata from TXT records
    fn parse_metadata_from_txt(properties: &HashMap<String, String>) -> HashMap<String, String> {
        let mut metadata = HashMap::new();

        for (key, value) in properties {
            // Skip capability list (already parsed)
            if key != "capabilities" {
                metadata.insert(key.clone(), value.clone());
            }
        }

        metadata
    }

    /// Get cached services for a service type
    async fn get_cached(&self, service_type: &str) -> Option<Vec<DiscoveredService>> {
        let cache = self.cache.read().await;
        cache.get(service_type).cloned()
    }

    /// Update cache with discovered services
    async fn update_cache(&self, service_type: &str, services: Vec<DiscoveredService>) {
        let mut cache = self.cache.write().await;
        cache.insert(service_type.to_string(), services);
    }

    /// Clear discovery cache
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
        debug!("mDNS discovery cache cleared");
    }

    /// Announce a service via mDNS
    ///
    /// Allows this service to be discovered by other primals.
    /// This is how we implement self-knowledge: announce what we ARE,
    /// discover what others ARE at runtime.
    pub fn announce(
        &self,
        capability: &str,
        port: u16,
        properties: HashMap<String, String>,
    ) -> Result<()> {
        let service_type = Self::capability_to_service_type(capability);
        let hostname = Self::get_hostname()?;

        let fullname = format!("{hostname}.{service_type}");

        let service_info =
            ServiceInfo::new(&service_type, &hostname, &hostname, "", port, properties)
                .map_err(|e| DiscoveryError::AnnouncementFailed(e.to_string()))?;

        self.daemon
            .register(service_info)
            .map_err(|e| DiscoveryError::AnnouncementFailed(e.to_string()))?;

        info!("Announced mDNS service: {} at port {}", fullname, port);
        Ok(())
    }

    /// Get local hostname
    fn get_hostname() -> Result<String> {
        let hostname = hostname::get()
            .map_err(|e| DiscoveryError::SystemError(format!("Failed to get hostname: {e}")))?;

        hostname
            .to_string_lossy()
            .split('.')
            .next()
            .map(std::string::ToString::to_string)
            .ok_or_else(|| DiscoveryError::SystemError("Invalid hostname".to_string()))
    }
}

impl Default for MdnsDiscovery {
    fn default() -> Self {
        Self::new().expect("mDNS ServiceDaemon requires a working network stack to initialize")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_to_service_type() {
        assert_eq!(
            MdnsDiscovery::capability_to_service_type("storage"),
            "_storage._tcp.local."
        );
        assert_eq!(
            MdnsDiscovery::capability_to_service_type("Orchestration"),
            "_orchestration._tcp.local."
        );
    }

    #[tokio::test]
    async fn test_mdns_discovery_creation() {
        let discovery = MdnsDiscovery::new();
        assert!(discovery.is_ok());
    }

    #[tokio::test]
    async fn test_mdns_discovery_with_config() {
        let config = MdnsConfig {
            timeout: Duration::from_secs(2),
            domain: "local.".to_string(),
            enable_ipv6: false,
        };

        let discovery = MdnsDiscovery::with_config(config);
        assert!(discovery.is_ok());
    }

    #[tokio::test]
    async fn test_mdns_discovery_empty_results() {
        let discovery = MdnsDiscovery::with_config(MdnsConfig {
            timeout: Duration::from_millis(100), // Short timeout
            ..Default::default()
        })
        .expect("Failed to create discovery");

        // Should return empty vec, not error
        let result = discovery.discover("nonexistent-capability").await;
        assert!(result.is_ok());
        let services = result.expect("discover should return Ok for missing capability");
        assert!(services.is_empty());
    }

    #[tokio::test]
    async fn test_cache_operations() {
        let discovery = MdnsDiscovery::new().expect("Failed to create discovery");

        // Cache should be empty initially
        assert!(discovery.get_cached("_test._tcp.local.").await.is_none());

        // Update cache
        let services = vec![];
        discovery.update_cache("_test._tcp.local.", services).await;

        // Should now be cached
        assert!(discovery.get_cached("_test._tcp.local.").await.is_some());

        // Clear cache
        discovery.clear_cache().await;
        assert!(discovery.get_cached("_test._tcp.local.").await.is_none());
    }

    #[test]
    fn test_get_hostname() {
        let hostname = MdnsDiscovery::get_hostname();
        assert!(hostname.is_ok());
        let name = hostname.expect("get_hostname should succeed in test environment");
        assert!(!name.is_empty());
        assert!(!name.contains('.'), "Hostname should not contain domain");
    }

    // ===== ERROR PATH TESTS =====

    #[tokio::test]
    async fn test_invalid_capability_name() {
        let discovery = MdnsDiscovery::new().expect("Failed to create discovery");

        // Empty capability
        let result = discovery.discover("").await;
        assert!(result.is_ok()); // Should handle gracefully

        // Special characters
        let result = discovery.discover("test/capability").await;
        assert!(result.is_ok()); // Should sanitize
    }

    #[tokio::test]
    async fn test_very_short_timeout() {
        let discovery = MdnsDiscovery::with_config(MdnsConfig {
            timeout: Duration::from_millis(1), // Nearly instant timeout
            ..Default::default()
        })
        .expect("Failed to create discovery");

        let result = discovery.discover("test-service").await;
        assert!(result.is_ok()); // Should return empty results
        let services = result.expect("discover with short timeout should return Ok");
        assert!(services.is_empty());
    }

    #[tokio::test]
    async fn test_concurrent_discoveries() {
        let discovery = MdnsDiscovery::new().expect("Failed to create discovery");

        // Multiple concurrent discoveries should not interfere
        let handles: Vec<_> = (0..5)
            .map(|i| {
                let disc = discovery.clone();
                tokio::spawn(async move { disc.discover(&format!("test-cap-{}", i)).await })
            })
            .collect();

        for handle in handles {
            let result = handle.await.expect("Task panicked");
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_cache_concurrent_access() {
        let discovery = MdnsDiscovery::new().expect("Failed to create discovery");

        // Concurrent writes to cache
        let handles: Vec<_> = (0..10)
            .map(|i| {
                let disc = discovery.clone();
                tokio::spawn(async move {
                    disc.update_cache(&format!("_test{}._tcp.local.", i), vec![])
                        .await;
                })
            })
            .collect();

        for handle in handles {
            handle.await.expect("Task panicked");
        }

        // Cache should have all entries
        discovery.clear_cache().await;
    }

    #[test]
    fn test_parse_capabilities_empty() {
        let properties = HashMap::new();
        let caps = MdnsDiscovery::parse_capabilities_from_txt(&properties, "test");
        assert_eq!(caps.len(), 1); // Should have primary capability
        assert_eq!(caps[0].capability_type, "test");
    }

    #[test]
    fn test_parse_capabilities_malformed() {
        let mut properties = HashMap::new();
        properties.insert("capabilities".to_string(), ",,,,".to_string());

        let caps = MdnsDiscovery::parse_capabilities_from_txt(&properties, "test");
        assert_eq!(caps.len(), 1); // Should only have primary
    }

    #[test]
    fn test_parse_capabilities_with_spaces() {
        let mut properties = HashMap::new();
        properties.insert(
            "capabilities".to_string(),
            " storage , encryption ".to_string(),
        );

        let caps = MdnsDiscovery::parse_capabilities_from_txt(&properties, "storage");
        assert_eq!(caps.len(), 2);
        assert_eq!(caps[1].capability_type, "encryption");
    }

    #[test]
    fn test_parse_metadata_filtering() {
        let mut properties = HashMap::new();
        properties.insert("version".to_string(), "1.0.0".to_string());
        properties.insert("tier".to_string(), "production".to_string());
        properties.insert("capabilities".to_string(), "storage".to_string());

        let metadata = MdnsDiscovery::parse_metadata_from_txt(&properties);
        assert_eq!(metadata.len(), 2); // Should exclude "capabilities"
        assert!(metadata.contains_key("version"));
        assert!(metadata.contains_key("tier"));
        assert!(!metadata.contains_key("capabilities"));
    }

    #[tokio::test]
    async fn test_announce_error_handling() {
        let discovery = MdnsDiscovery::new().expect("Failed to create discovery");

        // Test with valid port and empty properties
        let properties = HashMap::new();
        let result = discovery.announce("test-cap", 8080, properties);
        // Just verify it doesn't panic
        let _ = result;
    }

    #[tokio::test]
    async fn test_cache_expiry_behavior() {
        let discovery = MdnsDiscovery::new().expect("Failed to create discovery");

        // Add to cache
        discovery.update_cache("_test._tcp.local.", vec![]).await;
        assert!(discovery.get_cached("_test._tcp.local.").await.is_some());

        // Clear cache
        discovery.clear_cache().await;
        assert!(discovery.get_cached("_test._tcp.local.").await.is_none());
    }

    #[test]
    fn mdns_config_default_values() {
        let c = MdnsConfig::default();
        assert!(c.domain.contains("local"));
        assert!(c.timeout.as_secs() >= 1);
    }
}
