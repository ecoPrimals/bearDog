// SPDX-License-Identifier: AGPL-3.0-only

//! Universal Adapter for Ecosystem Service Discovery
//!
//! Provides zero-knowledge capability discovery and service coordination
//! across the `BearDog` ecosystem.

use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::CapabilityType;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Universal adapter for ecosystem service discovery
///
/// Provides zero-knowledge capability discovery and service coordination
/// across the `BearDog` ecosystem. Maintains a registry of available
/// capabilities and their endpoints.
#[derive(Debug, Clone)]
pub struct UniversalAdapter {
    /// Mapping of capability types to their service endpoints
    capabilities: Arc<RwLock<HashMap<CapabilityType, String>>>,
}

impl UniversalAdapter {
    /// Creates a new `UniversalAdapter` instance
    ///
    /// Initializes an empty adapter with no registered capabilities.
    /// Creates a new instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            capabilities: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    ///
    /// This is used to find where a particular service capability is hosted.
    ///
    /// # Arguments
    /// * `capability` - The type of capability to discover
    ///
    /// # Returns
    /// - `Ok(String)` containing the endpoint URL if the capability is registered
    ///
    /// # Errors
    /// Returns `Err(BearDogError)` if the capability is not available
    pub async fn discover_capability_endpoint(
        &self,
        capability: CapabilityType,
    ) -> Result<String, BearDogError> {
        let capabilities = self.capabilities.read().await;
        capabilities
            .get(&capability)
            .cloned()
            .ok_or_else(|| BearDogError::validation("Capability not available"))
    }

    /// Register a capability with its endpoint
    ///
    /// Adds a new capability to the registry, associating it with an endpoint URL.
    /// This allows other components to discover and use the capability.
    ///
    /// # Arguments
    /// * `capability` - The type of capability being registered
    /// * `endpoint` - The endpoint URL where the capability can be accessed
    ///
    /// # Returns
    /// - `Ok(())` if the capability was registered successfully
    ///
    /// # Errors
    /// Returns `Err(BearDogError)` if registration failed
    pub async fn register_capability(
        &self,
        capability: CapabilityType,
        endpoint: String,
    ) -> Result<(), BearDogError> {
        self.capabilities.write().await.insert(capability, endpoint);
        Ok(())
    }
}

impl Default for UniversalAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::canonical::capabilities::CapabilityType;

    #[tokio::test]
    async fn test_adapter_new() {
        let adapter = UniversalAdapter::new();
        let result = adapter
            .discover_capability_endpoint(CapabilityType::KeyManagement)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_adapter_register_and_discover() {
        let adapter = UniversalAdapter::new();
        adapter
            .register_capability(
                CapabilityType::KeyManagement,
                "http://localhost:8080/kms".to_string(),
            )
            .await
            .expect("register");
        let endpoint = adapter
            .discover_capability_endpoint(CapabilityType::KeyManagement)
            .await
            .expect("discover");
        assert_eq!(endpoint, "http://localhost:8080/kms");
    }

    #[tokio::test]
    async fn test_adapter_discover_unregistered() {
        let adapter = UniversalAdapter::new();
        let result = adapter
            .discover_capability_endpoint(CapabilityType::HardwareSecurityModule)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_adapter_default() {
        let adapter = UniversalAdapter::default();
        let result = adapter
            .discover_capability_endpoint(CapabilityType::Authentication)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_adapter_overwrite_capability() {
        let adapter = UniversalAdapter::new();
        adapter
            .register_capability(CapabilityType::KeyManagement, "http://old:8080".to_string())
            .await
            .expect("register");
        adapter
            .register_capability(CapabilityType::KeyManagement, "http://new:9090".to_string())
            .await
            .expect("overwrite");
        let endpoint = adapter
            .discover_capability_endpoint(CapabilityType::KeyManagement)
            .await
            .expect("discover");
        assert_eq!(endpoint, "http://new:9090");
    }
}
