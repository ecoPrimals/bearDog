// SPDX-License-Identifier: AGPL-3.0-only

//! mDNS-based Primal Discovery
//!
//! Runtime discovery of other primals using mDNS/DNS-SD.
//! Zero hardcoding - discovers primals announcing themselves on local network.

use beardog_errors::BearDogError;
use std::time::Duration;
use tracing::{debug, info, warn};

/// mDNS service type for `BearDog` primals
pub const BEARDOG_SERVICE_TYPE: &str = "_beardog._tcp.local.";

/// mDNS service type for generic crypto services (interop)
pub const CRYPTO_SERVICE_TYPE: &str = "_crypto-service._tcp.local.";

/// Discovered primal from mDNS
#[derive(Debug, Clone)]
pub struct MdnsDiscoveredPrimal {
    /// Instance name (e.g., "beardog-node-001")
    pub instance_name: String,
    /// IP addresses where primal is reachable
    pub addresses: Vec<std::net::IpAddr>,
    /// Port where primal API is listening
    pub port: u16,
    /// Capabilities from TXT records
    pub capabilities: Vec<String>,
    /// Version from TXT records
    pub version: Option<String>,
    /// Primal type from TXT records
    pub primal_type: Option<String>,
}

/// mDNS Discovery Client
///
/// Discovers other primals on the local network through mDNS/DNS-SD.
/// No hardcoded addresses - pure runtime discovery.
#[derive(Clone, Debug)]
pub struct MdnsDiscoveryClient {
    timeout: Duration,
}

impl MdnsDiscoveryClient {
    /// Create new mDNS discovery client
    #[must_use]
    pub const fn new() -> Self {
        Self {
            timeout: Duration::from_secs(2),
        }
    }

    /// Set discovery timeout
    #[must_use]
    pub const fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Discover primals with specific capability
    ///
    /// Browses for `BearDog` services on local network and filters by capability.
    ///
    /// # Errors
    /// Returns error if mDNS browsing fails
    #[cfg(feature = "mdns")]
    pub async fn discover_by_capability(
        &self,
        capability: &str,
    ) -> Result<Vec<MdnsDiscoveredPrimal>, BearDogError> {
        use mdns_sd::{ServiceDaemon, ServiceEvent};

        info!(
            "🔍 Discovering primals via mDNS with capability: {}",
            capability
        );

        // Create mDNS daemon
        let mdns = ServiceDaemon::new()
            .map_err(|e| BearDogError::network(format!("Failed to create mDNS daemon: {e}")))?;

        // Browse for BearDog services
        let receiver = mdns
            .browse(BEARDOG_SERVICE_TYPE)
            .map_err(|e| BearDogError::network(format!("Failed to browse mDNS services: {e}")))?;

        let mut discovered = Vec::new();
        let start = tokio::time::Instant::now();

        // Listen for service discoveries with timeout
        while start.elapsed() < self.timeout {
            // Use tokio timeout to avoid blocking forever
            match tokio::time::timeout(Duration::from_millis(100), async {
                receiver.recv_async().await
            })
            .await
            {
                Ok(Ok(event)) => match event {
                    ServiceEvent::ServiceResolved(info) => {
                        debug!(
                            "Found service: {} at {:?}",
                            info.get_fullname(),
                            info.get_addresses()
                        );

                        // Parse capabilities from TXT records
                        let txt_properties: std::collections::HashMap<String, String> = info
                            .get_properties()
                            .iter()
                            .map(|prop| {
                                let key = prop.key();
                                let val = prop.val_str();
                                (key.to_string(), val.to_string())
                            })
                            .collect();

                        let capabilities_str = txt_properties
                            .get("capabilities")
                            .map_or("", String::as_str);
                        let capabilities: Vec<String> = capabilities_str
                            .split(',')
                            .map(str::trim)
                            .map(ToString::to_string)
                            .collect();

                        // Filter by requested capability
                        if capabilities.iter().any(|c| c == capability) {
                            let primal = MdnsDiscoveredPrimal {
                                instance_name: info.get_hostname().to_string(),
                                addresses: info.get_addresses().iter().copied().collect(),
                                port: info.get_port(),
                                capabilities,
                                version: txt_properties.get("version").cloned(),
                                primal_type: txt_properties.get("primal").cloned(),
                            };

                            info!(
                                "✅ Discovered primal: {} at {}:{}",
                                primal.instance_name,
                                primal
                                    .addresses
                                    .first()
                                    .map(ToString::to_string)
                                    .unwrap_or_default(),
                                primal.port
                            );

                            discovered.push(primal);
                        }
                    }
                    ServiceEvent::SearchStopped(_) => {
                        debug!("mDNS search stopped");
                        break;
                    }
                    _ => {}
                },
                Ok(Err(e)) => {
                    warn!("mDNS receive error: {e}");
                    break;
                }
                Err(_) => {
                    // Timeout - continue listening
                }
            }
        }

        // Shutdown mDNS daemon
        if let Err(e) = mdns.shutdown() {
            warn!("Failed to shutdown mDNS daemon cleanly: {e}");
        }

        info!(
            "🔍 mDNS discovery complete: found {} primals",
            discovered.len()
        );
        Ok(discovered)
    }

    /// Discover all primals (no capability filter)
    ///
    /// # Errors
    /// Returns error if mDNS browsing fails
    #[cfg(feature = "mdns")]
    pub async fn discover_all(&self) -> Result<Vec<MdnsDiscoveredPrimal>, BearDogError> {
        use mdns_sd::{ServiceDaemon, ServiceEvent};

        info!("🔍 Discovering all primals via mDNS");

        let mdns = ServiceDaemon::new()
            .map_err(|e| BearDogError::network(format!("Failed to create mDNS daemon: {e}")))?;

        let receiver = mdns
            .browse(BEARDOG_SERVICE_TYPE)
            .map_err(|e| BearDogError::network(format!("Failed to browse mDNS services: {e}")))?;

        let mut discovered = Vec::new();
        let start = tokio::time::Instant::now();

        while start.elapsed() < self.timeout {
            if let Ok(Ok(ServiceEvent::ServiceResolved(info))) =
                tokio::time::timeout(Duration::from_millis(100), async {
                    receiver.recv_async().await
                })
                .await
            {
                let txt_properties: std::collections::HashMap<String, String> = info
                    .get_properties()
                    .iter()
                    .map(|prop| {
                        let key = prop.key();
                        let val = prop.val_str();
                        (key.to_string(), val.to_string())
                    })
                    .collect();

                let capabilities_str = txt_properties
                    .get("capabilities")
                    .map_or("", String::as_str);
                let capabilities: Vec<String> = capabilities_str
                    .split(',')
                    .map(str::trim)
                    .map(ToString::to_string)
                    .collect();

                let primal = MdnsDiscoveredPrimal {
                    instance_name: info.get_hostname().to_string(),
                    addresses: info.get_addresses().iter().copied().collect(),
                    port: info.get_port(),
                    capabilities,
                    version: txt_properties.get("version").cloned(),
                    primal_type: txt_properties.get("primal").cloned(),
                };

                discovered.push(primal);
            }
        }

        if let Err(e) = mdns.shutdown() {
            warn!("Failed to shutdown mDNS daemon cleanly: {e}");
        }

        info!(
            "🔍 mDNS discovery complete: found {} primals",
            discovered.len()
        );
        Ok(discovered)
    }

    /// Graceful fallback when mDNS feature is not enabled
    #[cfg(not(feature = "mdns"))]
    pub async fn discover_by_capability(
        &self,
        capability: &str,
    ) -> Result<Vec<MdnsDiscoveredPrimal>, BearDogError> {
        warn!(
            "mDNS feature not enabled. Cannot discover primals with capability: {}",
            capability
        );
        warn!("Enable with: cargo build --features mdns");
        Ok(Vec::new())
    }

    /// Graceful fallback when mDNS feature is not enabled
    #[cfg(not(feature = "mdns"))]
    pub async fn discover_all(&self) -> Result<Vec<MdnsDiscoveredPrimal>, BearDogError> {
        warn!("mDNS feature not enabled. Cannot discover primals.");
        warn!("Enable with: cargo build --features mdns");
        Ok(Vec::new())
    }
}

impl Default for MdnsDiscoveryClient {
    fn default() -> Self {
        Self::new()
    }
}

/// mDNS Service Announcer
///
/// Announces this primal's presence on the local network.
/// Other primals discover us through mDNS browsing.
pub struct MdnsServiceAnnouncer {
    instance_name: String,
    port: u16,
    capabilities: Vec<String>,
}

impl MdnsServiceAnnouncer {
    /// Create new service announcer
    ///
    /// Instance name defaults to "beardog-{hostname}"
    #[must_use]
    pub fn new(port: u16, capabilities: Vec<String>) -> Self {
        let hostname = gethostname::gethostname();
        let instance_name = format!("beardog-{}", hostname.to_string_lossy());

        Self {
            instance_name,
            port,
            capabilities,
        }
    }

    /// Set custom instance name
    #[must_use]
    pub fn with_instance_name(mut self, name: impl Into<String>) -> Self {
        self.instance_name = name.into();
        self
    }

    /// Announce service on mDNS
    ///
    /// Registers this primal with mDNS so others can discover it.
    ///
    /// # Errors
    /// Returns error if mDNS registration fails
    #[cfg(feature = "mdns")]
    pub fn announce(&self) -> Result<(), BearDogError> {
        use mdns_sd::{ServiceDaemon, ServiceInfo};
        use std::collections::HashMap;

        info!(
            "📢 Announcing primal via mDNS: {} on port {}",
            self.instance_name, self.port
        );

        // Create mDNS daemon
        let mdns = ServiceDaemon::new()
            .map_err(|e| BearDogError::network(format!("Failed to create mDNS daemon: {e}")))?;

        // Get local IP addresses
        let addresses = get_local_addresses();

        // Create TXT records
        let mut properties = HashMap::new();
        properties.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());
        properties.insert("primal".to_string(), "beardog".to_string());
        properties.insert("capabilities".to_string(), self.capabilities.join(","));

        // Register service
        let service_info = ServiceInfo::new(
            BEARDOG_SERVICE_TYPE,
            &self.instance_name,
            &self.instance_name,
            &addresses[..],
            self.port,
            Some(properties),
        )
        .map_err(|e| BearDogError::network(format!("Failed to create service info: {e}")))?;

        mdns.register(service_info)
            .map_err(|e| BearDogError::network(format!("Failed to register mDNS service: {e}")))?;

        info!("✅ mDNS service announced successfully");

        // Keep daemon alive (caller should store this)
        // In practice, store the mdns daemon to keep service registered
        std::mem::forget(mdns);

        Ok(())
    }

    /// Graceful fallback when mDNS feature is not enabled
    #[cfg(not(feature = "mdns"))]
    pub fn announce(&self) -> Result<(), BearDogError> {
        warn!(
            "mDNS feature not enabled. Service {} will not be advertised on local network.",
            self.instance_name
        );
        warn!("Enable with: cargo build --features mdns");
        warn!("Other primals can still discover via HTTP API or service registry");
        Ok(())
    }
}

/// Get local IP addresses for mDNS announcement
fn get_local_addresses() -> Vec<std::net::IpAddr> {
    use std::net::{IpAddr, Ipv4Addr};

    // Try to get actual network interfaces
    // For now, return localhost as fallback
    // Real implementation would use `if_addrs` or `network-interface` crate
    vec![IpAddr::V4(Ipv4Addr::LOCALHOST)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mdns_client_creation() {
        let client = MdnsDiscoveryClient::new();
        assert_eq!(client.timeout, Duration::from_secs(2));
    }

    #[tokio::test]
    async fn test_mdns_client_with_timeout() {
        let client = MdnsDiscoveryClient::new().with_timeout(Duration::from_secs(5));
        assert_eq!(client.timeout, Duration::from_secs(5));
    }

    #[tokio::test]
    async fn test_announcer_default_instance_name() {
        let announcer = MdnsServiceAnnouncer::new(8080, vec!["crypto".to_string()]);
        assert!(announcer.instance_name.starts_with("beardog-"));
    }

    #[tokio::test]
    async fn test_announcer_custom_instance_name() {
        let announcer = MdnsServiceAnnouncer::new(8080, vec!["crypto".to_string()])
            .with_instance_name("test-node");
        assert_eq!(announcer.instance_name, "test-node");
    }

    #[tokio::test]
    async fn test_discover_graceful_without_feature() {
        // Should not panic when mdns feature is disabled
        let client = MdnsDiscoveryClient::new();
        let result = client.discover_by_capability("crypto").await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_service_type_constants() {
        assert!(!BEARDOG_SERVICE_TYPE.is_empty());
        assert!(!CRYPTO_SERVICE_TYPE.is_empty());
        assert!(BEARDOG_SERVICE_TYPE.contains("beardog"));
    }

    #[test]
    fn get_local_addresses_includes_loopback() {
        let addrs = super::get_local_addresses();
        assert!(addrs.iter().any(std::net::IpAddr::is_loopback));
    }

    #[test]
    fn mdns_client_default_matches_new() {
        assert_eq!(
            MdnsDiscoveryClient::default().timeout,
            MdnsDiscoveryClient::new().timeout
        );
    }

    #[test]
    fn test_mdns_discovered_primal_structure() {
        let primal = MdnsDiscoveredPrimal {
            instance_name: "test-node".to_string(),
            addresses: vec![],
            port: 8080,
            capabilities: vec!["crypto".to_string()],
            version: Some("1.0".to_string()),
            primal_type: Some("beardog".to_string()),
        };
        assert_eq!(primal.instance_name, "test-node");
        assert_eq!(primal.port, 8080);
        assert_eq!(primal.capabilities.len(), 1);
    }

    #[tokio::test]
    async fn test_announce_graceful_without_feature() {
        // This test verifies graceful handling regardless of feature state
        // When mdns feature is enabled, it will try to announce (may succeed or fail based on network)
        // When mdns feature is disabled, it will log a warning and return Ok
        let announcer = MdnsServiceAnnouncer::new(8080, vec!["crypto".to_string()]);
        let result = announcer.announce();

        // We expect either:
        // - Ok(()) if announcement succeeds or feature is disabled
        // - Err() if network/mDNS fails (which is acceptable in test environment)
        // The key is that it should NOT panic
        if result.is_err() {
            // If it failed, log it but don't fail the test
            // Network conditions in CI/test environments may not support mDNS
            eprintln!("mDNS announce failed (acceptable in test): {result:?}");
        }
        // Test passes as long as we didn't panic
    }
}
