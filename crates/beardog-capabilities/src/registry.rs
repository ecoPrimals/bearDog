// SPDX-License-Identifier: AGPL-3.0-only

//! Capability registry
//!
//! Manages capability registration and advertisement for a primal instance.

use crate::metadata::{CapabilityAdvertisement, CapabilityMetadata, DiscoveryConfig, PrimalInfo};
use parking_lot::RwLock;
use std::any::Any;
use std::collections::HashMap;
use tracing::{debug, info};

/// Capability registry
///
/// BearDog uses this to register capabilities it provides.
/// The registry handles advertisement via mDNS and HTTP.
pub struct CapabilityRegistry {
    /// Primal information (self-knowledge)
    primal_info: PrimalInfo,

    /// Registered capabilities
    capabilities: RwLock<HashMap<String, RegisteredCapability>>,

    /// Discovery configuration
    discovery_config: DiscoveryConfig,
}

/// A registered capability (type-erased)
struct RegisteredCapability {
    /// Capability metadata
    metadata: CapabilityMetadata,

    /// Type-erased capability provider (Box<dyn Trait>)
    /// Note: Currently unused but reserved for future runtime capability queries
    _provider: Box<dyn Any + Send + Sync>,
}

impl CapabilityRegistry {
    /// Create a new capability registry
    ///
    /// # Arguments
    /// * `primal_id` - Unique identifier for this primal instance
    /// * `primal_type` - Type of primal (e.g., "cryptographic_services")
    /// * `base_url` - Base URL for HTTP endpoints
    pub fn new(
        primal_id: impl Into<String>,
        primal_type: impl Into<String>,
        base_url: impl Into<String>,
    ) -> Self {
        let primal_id = primal_id.into();
        let base_url = base_url.into();

        info!("Creating capability registry for primal: {}", primal_id);

        let primal_info = PrimalInfo {
            id: primal_id.clone(),
            primal_type: primal_type.into(),
            description: format!("BearDog instance: {}", primal_id),
            version: env!("CARGO_PKG_VERSION").to_string(),
        };

        let discovery_config = DiscoveryConfig {
            mdns: Some(format!("_beardog_{}._tcp.local", primal_id)),
            http: format!("{}/capabilities", base_url),
            ttl: 300,
        };

        Self {
            primal_info,
            capabilities: RwLock::new(HashMap::new()),
            discovery_config,
        }
    }

    /// Register a capability
    ///
    /// # Arguments
    /// * `capability_id` - Unique identifier (e.g., "secure_tunnel")
    /// * `provider` - Implementation of the capability trait
    /// * `metadata` - Capability metadata
    ///
    /// # Type Parameters
    /// * `T` - Capability provider type (must be Send + Sync + 'static)
    pub fn register<T: Send + Sync + 'static>(
        &self,
        capability_id: impl Into<String>,
        provider: T,
        metadata: CapabilityMetadata,
    ) {
        let capability_id = capability_id.into();

        debug!(
            "Registering capability: {} (version {})",
            capability_id, metadata.version
        );

        let registered = RegisteredCapability {
            metadata,
            _provider: Box::new(provider),
        };

        self.capabilities
            .write()
            .insert(capability_id.clone(), registered);

        info!("Capability registered: {}", capability_id);
    }

    /// Unregister a capability
    ///
    /// # Arguments
    /// * `capability_id` - Capability to remove
    ///
    /// # Returns
    /// True if capability was removed, false if not found
    pub fn unregister(&self, capability_id: &str) -> bool {
        let removed = self.capabilities.write().remove(capability_id).is_some();

        if removed {
            info!("Capability unregistered: {}", capability_id);
        }

        removed
    }

    /// Check if a capability is registered
    ///
    /// # Arguments
    /// * `capability_id` - Capability to check
    pub fn has_capability(&self, capability_id: &str) -> bool {
        self.capabilities.read().contains_key(capability_id)
    }

    /// Get list of registered capability IDs
    pub fn list_capabilities(&self) -> Vec<String> {
        self.capabilities.read().keys().cloned().collect()
    }

    /// Get capability metadata
    ///
    /// # Arguments
    /// * `capability_id` - Capability to query
    pub fn get_metadata(&self, capability_id: &str) -> Option<CapabilityMetadata> {
        self.capabilities
            .read()
            .get(capability_id)
            .map(|c| c.metadata.clone())
    }

    /// Build capability advertisement
    ///
    /// This creates the full advertisement with all registered capabilities.
    pub fn build_advertisement(&self) -> CapabilityAdvertisement {
        let capabilities: Vec<CapabilityMetadata> = self
            .capabilities
            .read()
            .values()
            .map(|c| c.metadata.clone())
            .collect();

        debug!(
            "Building advertisement with {} capabilities",
            capabilities.len()
        );

        CapabilityAdvertisement {
            primal: self.primal_info.clone(),
            capabilities,
            discovery: self.discovery_config.clone(),
        }
    }

    /// Advertise capabilities via mDNS and HTTP
    ///
    /// This should be called after all capabilities are registered.
    /// The advertisement will be served via HTTP and optionally mDNS.
    pub async fn advertise(&self) -> crate::Result<()> {
        let advertisement = self.build_advertisement();

        info!(
            "Advertising {} capabilities for primal {}",
            advertisement.capabilities.len(),
            advertisement.primal.id
        );

        // mDNS advertisement (Phase 3)
        // Environment-driven: Only advertise if ENABLE_MDNS=true
        // Graceful fallback: Log intent if mDNS unavailable

        let mdns_enabled = std::env::var("ENABLE_MDNS")
            .ok()
            .and_then(|v| v.parse::<bool>().ok())
            .unwrap_or(false);

        #[cfg(feature = "mdns")]
        {
            if mdns_enabled {
                if let Some(ref mdns_service) = advertisement.discovery.mdns {
                    info!(
                        "🔊 mDNS advertisement enabled for service: {}",
                        mdns_service
                    );
                    // Actual mDNS implementation would go here
                    // This requires the mdns-sd crate and platform-specific setup
                    // For now, we log the intent and delegate to ecosystem
                    debug!(
                        "mDNS service details: name={}, capabilities={}",
                        advertisement.primal.id,
                        advertisement.capabilities.len()
                    );
                } else {
                    debug!("mDNS enabled but no service name configured");
                }
            } else {
                debug!("mDNS advertisement disabled (set ENABLE_MDNS=true to enable)");
            }
        }

        #[cfg(not(feature = "mdns"))]
        {
            if mdns_enabled {
                warn!(
                    "⚠️  mDNS advertisement requested but 'mdns' feature not enabled. \
                     Rebuild with --features mdns to enable."
                );
            }
        }

        // HTTP advertisement is handled by the API server
        debug!(
            "HTTP advertisement endpoint: {}",
            advertisement.discovery.http
        );

        Ok(())
    }

    /// Get primal information
    pub fn primal_info(&self) -> &PrimalInfo {
        &self.primal_info
    }

    /// Get discovery configuration
    pub fn discovery_config(&self) -> &DiscoveryConfig {
        &self.discovery_config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockCapability;

    #[test]
    fn test_registry_creation() {
        let registry = CapabilityRegistry::new(
            "test-primal-1",
            "cryptographic_services",
            "http://localhost:8080",
        );

        assert_eq!(registry.primal_info().id, "test-primal-1");
        assert_eq!(registry.primal_info().primal_type, "cryptographic_services");
    }

    #[test]
    fn test_register_capability() {
        let registry = CapabilityRegistry::new(
            "test-primal-1",
            "cryptographic_services",
            "http://localhost:8080",
        );

        let metadata = CapabilityMetadata::new("test_capability", "1.0")
            .with_interface("TestCapability")
            .with_endpoint("http://localhost:8080/capabilities/test");

        registry.register("test_capability", MockCapability, metadata);

        assert!(registry.has_capability("test_capability"));
        assert!(!registry.has_capability("nonexistent"));
    }

    #[test]
    fn test_unregister_capability() {
        let registry = CapabilityRegistry::new(
            "test-primal-1",
            "cryptographic_services",
            "http://localhost:8080",
        );

        let metadata = CapabilityMetadata::new("test_capability", "1.0");
        registry.register("test_capability", MockCapability, metadata);

        assert!(registry.has_capability("test_capability"));
        assert!(registry.unregister("test_capability"));
        assert!(!registry.has_capability("test_capability"));
        assert!(!registry.unregister("test_capability")); // Already removed
    }

    #[test]
    fn test_list_capabilities() {
        let registry = CapabilityRegistry::new(
            "test-primal-1",
            "cryptographic_services",
            "http://localhost:8080",
        );

        let metadata1 = CapabilityMetadata::new("capability1", "1.0");
        let metadata2 = CapabilityMetadata::new("capability2", "1.0");

        registry.register("capability1", MockCapability, metadata1);
        registry.register("capability2", MockCapability, metadata2);

        let list = registry.list_capabilities();
        assert_eq!(list.len(), 2);
        assert!(list.contains(&"capability1".to_string()));
        assert!(list.contains(&"capability2".to_string()));
    }

    #[test]
    fn test_build_advertisement() {
        let registry = CapabilityRegistry::new(
            "test-primal-1",
            "cryptographic_services",
            "http://localhost:8080",
        );

        let metadata1 =
            CapabilityMetadata::new("secure_tunnel", "1.0").with_interface("SecureTunnelProvider");
        let metadata2 = CapabilityMetadata::new("lineage_signing", "1.0")
            .with_interface("LineageSigningProvider");

        registry.register("secure_tunnel", MockCapability, metadata1);
        registry.register("lineage_signing", MockCapability, metadata2);

        let advertisement = registry.build_advertisement();

        assert_eq!(advertisement.primal.id, "test-primal-1");
        assert_eq!(advertisement.capabilities.len(), 2);
        assert!(advertisement
            .capabilities
            .iter()
            .any(|c| c.id == "secure_tunnel"));
        assert!(advertisement
            .capabilities
            .iter()
            .any(|c| c.id == "lineage_signing"));
    }

    #[test]
    fn test_get_metadata() {
        let registry = CapabilityRegistry::new(
            "test-primal-1",
            "cryptographic_services",
            "http://localhost:8080",
        );

        let metadata = CapabilityMetadata::new("test_cap", "2.0")
            .with_description("Test capability")
            .with_rate_limit(100);

        registry.register("test_cap", MockCapability, metadata);

        let retrieved = registry.get_metadata("test_cap");
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.id, "test_cap");
        assert_eq!(retrieved.version, "2.0");
        assert_eq!(retrieved.description, "Test capability");
        assert_eq!(retrieved.rate_limit, Some(100));

        // Non-existent capability
        assert!(registry.get_metadata("nonexistent").is_none());
    }

    #[test]
    fn test_discovery_config() {
        let registry = CapabilityRegistry::new("my-primal", "storage", "http://localhost:9000");

        let config = registry.discovery_config();
        assert_eq!(config.http, "http://localhost:9000/capabilities");
        assert_eq!(config.ttl, 300);
        assert!(config.mdns.is_some());
        assert!(config.mdns.as_ref().unwrap().contains("my-primal"));
    }

    #[test]
    fn test_empty_registry() {
        let registry = CapabilityRegistry::new("empty-primal", "test", "http://localhost:8080");

        assert!(registry.list_capabilities().is_empty());
        let advertisement = registry.build_advertisement();
        assert!(advertisement.capabilities.is_empty());
        assert_eq!(advertisement.primal.id, "empty-primal");
    }

    #[test]
    fn test_register_multiple_and_query() {
        let registry =
            CapabilityRegistry::new("multi-cap-primal", "compute", "http://localhost:8080");

        for i in 1..=5 {
            let id = format!("capability_{}", i);
            let metadata = CapabilityMetadata::new(&id, "1.0");
            registry.register(&id, MockCapability, metadata);
        }

        assert_eq!(registry.list_capabilities().len(), 5);
        for i in 1..=5 {
            let id = format!("capability_{}", i);
            assert!(registry.has_capability(&id));
        }
    }

    #[tokio::test]
    async fn test_advertise() {
        let registry = CapabilityRegistry::new("advertise-test", "test", "http://localhost:8080");

        let metadata = CapabilityMetadata::new("test", "1.0");
        registry.register("test", MockCapability, metadata);

        // Advertise should succeed (no-op without mdns feature)
        let result = registry.advertise().await;
        assert!(result.is_ok());
    }
}
