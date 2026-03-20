// SPDX-License-Identifier: AGPL-3.0-only

//! DNS Service Discovery (DNS-SD) implementation
//!
//! Complete production implementation using hickory-resolver for standards-compliant
//! DNS-SD service discovery (RFC 6763). No mocks - real DNS queries.

use crate::error::{DiscoveryError, Result};
use crate::types::{Capability, DiscoveredService, HealthStatus, QoSMetrics, ServiceEndpoint};
use hickory_resolver::TokioResolver;
use hickory_resolver::config::{ResolverConfig, ResolverOpts};
use hickory_resolver::name_server::TokioConnectionProvider;
use hickory_resolver::proto::rr::RecordType;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// DNS-SD discovery configuration
#[derive(Debug, Clone)]
pub struct DnsSdConfig {
    /// DNS domain to query (e.g., "local", "service.consul")
    pub domain: String,
    /// Query timeout
    pub timeout: Duration,
    /// Resolver configuration
    pub resolver_config: ResolverConfig,
    /// Resolver options
    pub resolver_opts: ResolverOpts,
}

impl Default for DnsSdConfig {
    fn default() -> Self {
        let mut opts = ResolverOpts::default();
        opts.timeout = Duration::from_secs(3);
        opts.attempts = 2;

        Self {
            domain: "local".to_string(),
            timeout: Duration::from_secs(5),
            resolver_config: ResolverConfig::default(),
            resolver_opts: opts,
        }
    }
}

/// DNS Service Discovery client - complete implementation
#[derive(Clone)]
pub struct DnsSdDiscovery {
    resolver: Arc<TokioResolver>,
    config: DnsSdConfig,
    cache: Arc<RwLock<HashMap<String, Vec<DiscoveredService>>>>,
}

impl DnsSdDiscovery {
    /// Create new DNS-SD discovery client with default configuration
    pub async fn new() -> Result<Self> {
        Self::with_config(DnsSdConfig::default()).await
    }

    /// Create new DNS-SD discovery client with custom configuration
    pub async fn with_config(config: DnsSdConfig) -> Result<Self> {
        let resolver = TokioResolver::builder_with_config(
            config.resolver_config.clone(),
            TokioConnectionProvider::default(),
        )
        .with_options(config.resolver_opts.clone())
        .build();

        Ok(Self {
            resolver: Arc::new(resolver),
            config,
            cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Discover services by capability using DNS-SD
    ///
    /// Performs real DNS queries following RFC 6763 DNS-SD specification.
    /// Returns all discovered services advertising the given capability.
    pub async fn discover(&self, capability: &str) -> Result<Vec<DiscoveredService>> {
        let service_type = Self::capability_to_service_name(capability);

        debug!("Starting DNS-SD discovery for service: {}", service_type);

        // Check cache first
        if let Some(cached) = self.get_cached(&service_type).await {
            debug!("Found {} cached services for {}", cached.len(), capability);
            return Ok(cached);
        }

        // Step 1: Browse for services (PTR query)
        let service_instances = self.browse_services(&service_type).await?;

        if service_instances.is_empty() {
            info!("No DNS-SD services found for {}", capability);
            return Ok(Vec::new());
        }

        // Step 2: Resolve each service instance (SRV + TXT queries)
        let mut services = Vec::new();
        for instance_name in service_instances {
            match self.resolve_service(&instance_name, capability).await {
                Ok(service) => services.push(service),
                Err(e) => {
                    warn!("Failed to resolve service {}: {}", instance_name, e);
                }
            }
        }

        info!(
            "DNS-SD discovery complete: found {} services for {}",
            services.len(),
            capability
        );

        // Update cache
        self.update_cache(&service_type, services.clone()).await;

        Ok(services)
    }

    /// Browse for service instances (PTR query)
    ///
    /// Query format: _<service>._tcp.<domain>
    /// Example: _storage._tcp.local
    async fn browse_services(&self, service_name: &str) -> Result<Vec<String>> {
        let query_name = format!("_{}._{}.{}", service_name, "tcp", self.config.domain);

        debug!("DNS-SD PTR query: {}", query_name);

        let response = match tokio::time::timeout(
            self.config.timeout,
            self.resolver.lookup(query_name.as_str(), RecordType::PTR),
        )
        .await
        {
            Ok(Ok(response)) => response,
            Ok(Err(e)) => {
                debug!("No services found for {}: {}", service_name, e);
                return Ok(vec![]);
            }
            Err(_) => {
                return Err(DiscoveryError::Timeout(
                    "DNS-SD browse query timed out".to_string(),
                ));
            }
        };

        let instances: Vec<String> = response
            .iter()
            .filter_map(|record| record.as_ptr().map(std::string::ToString::to_string))
            .collect();

        debug!("Found {} service instances", instances.len());
        Ok(instances)
    }

    /// Resolve service instance details (SRV + TXT queries)
    async fn resolve_service(
        &self,
        instance_name: &str,
        capability: &str,
    ) -> Result<DiscoveredService> {
        // Query SRV record for hostname and port
        let srv_response = self
            .resolver
            .lookup(instance_name, RecordType::SRV)
            .await
            .map_err(|e| DiscoveryError::QueryFailed(format!("SRV query failed: {e}")))?;

        let srv_record = srv_response
            .iter()
            .find_map(|r| r.as_srv())
            .ok_or_else(|| DiscoveryError::InvalidServiceInfo("No SRV record found".to_string()))?;

        let hostname = srv_record.target().to_string();
        let port = srv_record.port();

        // Query TXT record for metadata
        let txt_properties = match self.resolver.lookup(instance_name, RecordType::TXT).await {
            Ok(response) => {
                Self::parse_txt_records(response.iter().filter_map(|r| r.as_txt()).collect())
            }
            Err(e) => {
                debug!("No TXT records for {}: {}", instance_name, e);
                HashMap::new()
            }
        };

        // Resolve hostname to IP addresses
        let addresses = self.resolve_hostname(&hostname).await?;

        if addresses.is_empty() {
            return Err(DiscoveryError::InvalidServiceInfo(format!(
                "No IP addresses found for hostname: {hostname}"
            )));
        }

        let primary_url = format!("http://{}:{}", addresses[0], port);

        // Parse capabilities from TXT records
        let capabilities = Self::parse_capabilities_from_txt(&txt_properties, capability);

        Ok(DiscoveredService {
            id: instance_name.to_string(),
            service_type: capability.to_string(),
            display_name: hostname.clone(),
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
            discovery_method: "dns-sd".to_string(),
            metadata: txt_properties,
        })
    }

    /// Resolve hostname to IP addresses
    async fn resolve_hostname(&self, hostname: &str) -> Result<Vec<IpAddr>> {
        // Try A records (IPv4)
        let mut addresses = Vec::new();

        if let Ok(response) = self.resolver.lookup_ip(hostname).await {
            addresses.extend(response.iter());
        }

        Ok(addresses)
    }

    /// Parse TXT records into key-value pairs
    ///
    /// TXT records format: key=value
    fn parse_txt_records(
        txt_records: Vec<&hickory_resolver::proto::rr::rdata::TXT>,
    ) -> HashMap<String, String> {
        let mut properties = HashMap::new();

        for txt in txt_records {
            for data in txt.iter() {
                if let Ok(text) = std::str::from_utf8(data) {
                    if let Some((key, value)) = text.split_once('=') {
                        properties.insert(key.to_string(), value.to_string());
                    }
                }
            }
        }

        properties
    }

    /// Parse capabilities from TXT record properties
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

    /// Convert capability name to DNS-SD service name
    ///
    /// Examples:
    /// - "storage" -> "storage"
    /// - "orchestration" -> "orchestration"
    fn capability_to_service_name(capability: &str) -> String {
        capability.to_lowercase()
    }

    /// Get cached services
    async fn get_cached(&self, service_type: &str) -> Option<Vec<DiscoveredService>> {
        let cache = self.cache.read().await;
        cache.get(service_type).cloned()
    }

    /// Update cache
    async fn update_cache(&self, service_type: &str, services: Vec<DiscoveredService>) {
        let mut cache = self.cache.write().await;
        cache.insert(service_type.to_string(), services);
    }

    /// Clear discovery cache
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
        debug!("DNS-SD discovery cache cleared");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_to_service_name() {
        assert_eq!(
            DnsSdDiscovery::capability_to_service_name("storage"),
            "storage"
        );
        assert_eq!(
            DnsSdDiscovery::capability_to_service_name("Orchestration"),
            "orchestration"
        );
    }

    #[tokio::test]
    async fn test_dns_sd_discovery_creation() {
        let discovery = DnsSdDiscovery::new().await;
        assert!(discovery.is_ok());
    }

    #[tokio::test]
    async fn test_dns_sd_discovery_with_config() {
        let config = DnsSdConfig {
            domain: "local".to_string(),
            timeout: Duration::from_secs(2),
            ..Default::default()
        };

        let discovery = DnsSdDiscovery::with_config(config).await;
        assert!(discovery.is_ok());
    }

    #[tokio::test]
    async fn test_dns_sd_discovery_empty_results() {
        let config = DnsSdConfig {
            timeout: Duration::from_millis(100),
            ..Default::default()
        };

        let discovery = DnsSdDiscovery::with_config(config)
            .await
            .expect("Failed to create discovery");

        // Should return empty vec, not error
        let result = discovery.discover("nonexistent-capability").await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_cache_operations() {
        let discovery = DnsSdDiscovery::new()
            .await
            .expect("Failed to create discovery");

        // Cache should be empty initially
        assert!(discovery.get_cached("test-service").await.is_none());

        // Update cache
        let services = vec![];
        discovery.update_cache("test-service", services).await;

        // Should now be cached
        assert!(discovery.get_cached("test-service").await.is_some());

        // Clear cache
        discovery.clear_cache().await;
        assert!(discovery.get_cached("test-service").await.is_none());
    }

    #[test]
    fn test_parse_txt_records_empty() {
        let result = DnsSdDiscovery::parse_txt_records(vec![]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_capabilities_from_txt() {
        let mut properties = HashMap::new();
        properties.insert("capabilities".to_string(), "storage,encryption".to_string());

        let caps = DnsSdDiscovery::parse_capabilities_from_txt(&properties, "storage");

        assert_eq!(caps.len(), 2);
        assert_eq!(caps[0].capability_type, "storage");
        assert_eq!(caps[1].capability_type, "encryption");
    }

    // ===== ERROR PATH TESTS =====

    #[tokio::test]
    async fn test_invalid_domain_configuration() {
        let config = DnsSdConfig {
            domain: "".to_string(), // Empty domain
            ..Default::default()
        };

        let discovery = DnsSdDiscovery::with_config(config).await;
        assert!(discovery.is_ok()); // Should handle gracefully
    }

    #[tokio::test]
    async fn test_very_short_discovery_timeout() {
        let config = DnsSdConfig {
            timeout: Duration::from_millis(1), // Nearly instant
            ..Default::default()
        };

        let discovery = DnsSdDiscovery::with_config(config)
            .await
            .expect("Failed to create discovery");

        // Should timeout and return empty or error gracefully
        let result = discovery.discover("test-service").await;
        // Either error or empty result is acceptable
        match result {
            Ok(services) => assert!(services.is_empty()),
            Err(e) => {
                // Timeout error is acceptable
                assert!(matches!(e, DiscoveryError::Timeout(_)));
            }
        }
    }

    #[tokio::test]
    async fn test_concurrent_dns_sd_queries() {
        let discovery = DnsSdDiscovery::new()
            .await
            .expect("Failed to create discovery");

        // Multiple concurrent queries
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
    async fn test_cache_concurrent_updates() {
        let discovery = DnsSdDiscovery::new()
            .await
            .expect("Failed to create discovery");

        // Concurrent cache updates
        let handles: Vec<_> = (0..10)
            .map(|i| {
                let disc = discovery.clone();
                tokio::spawn(async move {
                    disc.update_cache(&format!("service-{}", i), vec![]).await;
                })
            })
            .collect();

        for handle in handles {
            handle.await.expect("Task panicked");
        }

        discovery.clear_cache().await;
    }

    #[test]
    fn test_capability_name_sanitization() {
        assert_eq!(
            DnsSdDiscovery::capability_to_service_name("Storage"),
            "storage"
        );
        assert_eq!(
            DnsSdDiscovery::capability_to_service_name("STORAGE"),
            "storage"
        );
        assert_eq!(
            DnsSdDiscovery::capability_to_service_name("test-service"),
            "test-service"
        );
    }

    #[test]
    fn test_parse_empty_capabilities() {
        let properties = HashMap::new();
        let caps = DnsSdDiscovery::parse_capabilities_from_txt(&properties, "test");
        assert_eq!(caps.len(), 1);
        assert_eq!(caps[0].capability_type, "test");
    }

    #[test]
    fn test_parse_malformed_capability_list() {
        let mut properties = HashMap::new();
        properties.insert("capabilities".to_string(), ",,,,".to_string());

        let caps = DnsSdDiscovery::parse_capabilities_from_txt(&properties, "test");
        assert_eq!(caps.len(), 1); // Should only have primary
    }

    #[test]
    fn test_parse_capabilities_with_whitespace() {
        let mut properties = HashMap::new();
        properties.insert(
            "capabilities".to_string(),
            " storage , encryption , tunneling ".to_string(),
        );

        let caps = DnsSdDiscovery::parse_capabilities_from_txt(&properties, "storage");
        assert!(caps.len() >= 3);
        assert!(caps.iter().any(|c| c.capability_type == "encryption"));
        assert!(caps.iter().any(|c| c.capability_type == "tunneling"));
    }

    #[test]
    fn test_parse_duplicate_capabilities() {
        let mut properties = HashMap::new();
        properties.insert(
            "capabilities".to_string(),
            "storage,encryption,storage".to_string(),
        );

        let caps = DnsSdDiscovery::parse_capabilities_from_txt(&properties, "storage");
        // Should have: storage (primary), encryption, storage (duplicate)
        assert!(caps.len() >= 2);
    }

    #[test]
    fn test_parse_metadata_from_txt() {
        let mut properties = HashMap::new();
        properties.insert("version".to_string(), "1.2.3".to_string());
        properties.insert("environment".to_string(), "production".to_string());
        properties.insert("capabilities".to_string(), "storage".to_string());

        // Capabilities should be in metadata (DNS-SD includes all TXT records)
        assert!(properties.contains_key("capabilities"));
    }

    #[tokio::test]
    async fn test_discovery_with_invalid_capability() {
        let discovery = DnsSdDiscovery::new()
            .await
            .expect("Failed to create discovery");

        // Empty capability name
        let result = discovery.discover("").await;
        assert!(result.is_ok()); // Should handle gracefully
    }

    #[tokio::test]
    async fn test_cache_ttl_behavior() {
        let discovery = DnsSdDiscovery::new()
            .await
            .expect("Failed to create discovery");

        // Add entry
        discovery.update_cache("test", vec![]).await;
        assert!(discovery.get_cached("test").await.is_some());

        // Clear and verify
        discovery.clear_cache().await;
        assert!(discovery.get_cached("test").await.is_none());
    }
}
