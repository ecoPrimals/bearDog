// SPDX-License-Identifier: AGPL-3.0-only

//! Capability registry
//!
//! Manages capability registration and advertisement for a primal instance.

use crate::metadata::{CapabilityAdvertisement, CapabilityMetadata, DiscoveryConfig, PrimalInfo};
use parking_lot::RwLock;
use std::any::Any;
use std::collections::HashMap;
#[cfg(not(feature = "mdns"))]
use tracing::warn;
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
    /// Create a new capability registry using runtime discovery defaults.
    ///
    /// HTTP and mDNS advertisement targets are resolved from environment variables
    /// (see [`crate::metadata::resolve_capability_http_discovery_url`] and
    /// [`crate::metadata::resolve_capability_mdns_full_name`]). For explicit URLs in
    /// tests or integrations, use [`Self::with_http_base`].
    ///
    /// # Arguments
    /// * `instance_id` - Sovereign instance identifier (UUID or operator-assigned), not a hardcoded primal name
    /// * `capability_profile` - Declared service profile (e.g. `cryptographic_services`) for advertisement
    pub fn new(instance_id: impl Into<String>, capability_profile: impl Into<String>) -> Self {
        let instance_id = instance_id.into();
        let discovery_http = crate::metadata::resolve_capability_http_discovery_url();
        Self::from_discovery_http(instance_id, capability_profile.into(), discovery_http)
    }

    /// Create a registry with an explicit HTTP base URL; capability path still follows
    /// `BEARDOG_CAPABILITY_HTTP_PATH` or [`crate::metadata::DEFAULT_CAPABILITY_HTTP_PATH`].
    ///
    /// Prefer this in tests and when the HTTP base is known from configuration rather than env defaults.
    pub fn with_http_base(
        instance_id: impl Into<String>,
        capability_profile: impl Into<String>,
        http_base: impl Into<String>,
    ) -> Self {
        let instance_id = instance_id.into();
        let base = http_base.into().trim_end_matches('/').to_string();
        let path = std::env::var(crate::metadata::ENV_CAPABILITY_HTTP_PATH)
            .unwrap_or_else(|_| crate::metadata::DEFAULT_CAPABILITY_HTTP_PATH.to_string());
        let discovery_http = if path.starts_with('/') {
            format!("{base}{path}")
        } else {
            format!("{base}/{path}")
        };
        Self::from_discovery_http(instance_id, capability_profile.into(), discovery_http)
    }

    fn from_discovery_http(
        instance_id: String,
        capability_profile: String,
        discovery_http: String,
    ) -> Self {
        info!("Creating capability registry for instance: {}", instance_id);

        let primal_info = PrimalInfo {
            id: instance_id.clone(),
            primal_type: capability_profile,
            description: format!("Capability provider instance {instance_id}"),
            version: env!("CARGO_PKG_VERSION").to_string(),
        };

        let discovery_config = DiscoveryConfig {
            mdns: Some(crate::metadata::resolve_capability_mdns_full_name(
                &instance_id,
            )),
            http: discovery_http,
            ttl: std::env::var("BEARDOG_CAPABILITY_DISCOVERY_TTL")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(crate::metadata::DEFAULT_DISCOVERY_TTL_SECS),
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
    pub const fn primal_info(&self) -> &PrimalInfo {
        &self.primal_info
    }

    /// Get discovery configuration
    pub const fn discovery_config(&self) -> &DiscoveryConfig {
        &self.discovery_config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test-scoped HTTP base only; production URLs come from env / discovery.
    const TEST_HTTP_BASE: &str = "http://127.0.0.1:54321";
    const TEST_ALT_HTTP_BASE: &str = "http://127.0.0.1:59000";

    /// Type-erased placeholder provider for registry API tests only (not a real capability).
    struct MockCapability;

    #[test]
    fn test_registry_creation() {
        let registry = CapabilityRegistry::with_http_base(
            "550e8400-e29b-41d4-a716-446655440000",
            "cryptographic_services",
            TEST_HTTP_BASE,
        );

        assert_eq!(
            registry.primal_info().id,
            "550e8400-e29b-41d4-a716-446655440000"
        );
        assert_eq!(registry.primal_info().primal_type, "cryptographic_services");
    }

    #[test]
    fn test_register_capability() {
        let registry = CapabilityRegistry::with_http_base(
            "550e8400-e29b-41d4-a716-446655440000",
            "cryptographic_services",
            TEST_HTTP_BASE,
        );

        let metadata = CapabilityMetadata::new("test_capability", "1.0")
            .with_interface("TestCapability")
            .with_endpoint(format!("{TEST_HTTP_BASE}/capabilities/test"));

        registry.register("test_capability", MockCapability, metadata);

        assert!(registry.has_capability("test_capability"));
        assert!(!registry.has_capability("nonexistent"));
    }

    #[test]
    fn test_unregister_capability() {
        let registry = CapabilityRegistry::with_http_base(
            "550e8400-e29b-41d4-a716-446655440000",
            "cryptographic_services",
            TEST_HTTP_BASE,
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
        let registry = CapabilityRegistry::with_http_base(
            "550e8400-e29b-41d4-a716-446655440000",
            "cryptographic_services",
            TEST_HTTP_BASE,
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
        let registry = CapabilityRegistry::with_http_base(
            "550e8400-e29b-41d4-a716-446655440000",
            "cryptographic_services",
            TEST_HTTP_BASE,
        );

        let metadata1 =
            CapabilityMetadata::new("secure_tunnel", "1.0").with_interface("SecureTunnelProvider");
        let metadata2 = CapabilityMetadata::new("lineage_signing", "1.0")
            .with_interface("LineageSigningProvider");

        registry.register("secure_tunnel", MockCapability, metadata1);
        registry.register("lineage_signing", MockCapability, metadata2);

        let advertisement = registry.build_advertisement();

        assert_eq!(
            advertisement.primal.id,
            "550e8400-e29b-41d4-a716-446655440000"
        );
        assert_eq!(advertisement.capabilities.len(), 2);
        assert!(
            advertisement
                .capabilities
                .iter()
                .any(|c| c.id == "secure_tunnel")
        );
        assert!(
            advertisement
                .capabilities
                .iter()
                .any(|c| c.id == "lineage_signing")
        );
    }

    #[test]
    fn test_get_metadata() {
        let registry = CapabilityRegistry::with_http_base(
            "550e8400-e29b-41d4-a716-446655440000",
            "cryptographic_services",
            TEST_HTTP_BASE,
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
        let registry = CapabilityRegistry::with_http_base(
            "7c9e6679-7425-40de-944b-e07fc1f90ae7",
            "storage",
            TEST_ALT_HTTP_BASE,
        );

        let config = registry.discovery_config();
        assert_eq!(config.http, format!("{TEST_ALT_HTTP_BASE}/capabilities"));
        assert_eq!(config.ttl, crate::metadata::DEFAULT_DISCOVERY_TTL_SECS);
        assert!(config.mdns.is_some());
        let mdns = config.mdns.as_ref().unwrap();
        assert!(
            mdns.contains("7c9e6679-7425-40de-944b-e07fc1f90ae7"),
            "mDNS name should include sovereign instance id: {mdns}"
        );
        assert!(
            mdns.contains("_beardog-cap._tcp"),
            "mDNS should use capability service type: {mdns}"
        );
    }

    #[test]
    fn test_empty_registry() {
        let registry = CapabilityRegistry::with_http_base(
            "b0000000-0000-4000-8000-000000000001",
            "test",
            TEST_HTTP_BASE,
        );

        assert!(registry.list_capabilities().is_empty());
        let advertisement = registry.build_advertisement();
        assert!(advertisement.capabilities.is_empty());
        assert_eq!(
            advertisement.primal.id,
            "b0000000-0000-4000-8000-000000000001"
        );
    }

    #[test]
    fn test_register_multiple_and_query() {
        let registry = CapabilityRegistry::with_http_base(
            "a0000000-0000-4000-8000-000000000002",
            "compute",
            TEST_HTTP_BASE,
        );

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
        let registry = CapabilityRegistry::with_http_base(
            "c0000000-0000-4000-8000-000000000003",
            "test",
            TEST_HTTP_BASE,
        );

        let metadata = CapabilityMetadata::new("test", "1.0");
        registry.register("test", MockCapability, metadata);

        // Advertise should succeed (no-op without mdns feature)
        let result = registry.advertise().await;
        assert!(result.is_ok());
    }
}
