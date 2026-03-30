// SPDX-License-Identifier: AGPL-3.0-only

//! Universal HSM registry for managing multiple HSM providers

use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Universal HSM registry
#[derive(Clone)]
pub struct UniversalHsmRegistry {
    providers: Arc<RwLock<HashMap<String, String>>>,
}

impl UniversalHsmRegistry {
    /// Create new HSM registry
    pub fn new() -> Self {
        Self {
            providers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// # Errors
    ///
    /// Returns an error if the provider cannot be registered.
    /// Register a provider
    pub async fn register_provider(
        &self,
        provider_id: String,
        provider_type: String,
    ) -> Result<(), BearDogError> {
        let mut providers = self.providers.write().await;
        providers.insert(provider_id, provider_type);
        Ok(())
    }

    /// # Errors
    ///
    /// Returns an error if the provider cannot be registered.
    /// Unregister a provider
    pub async fn unregister_provider(&self, provider_id: &str) -> Result<(), BearDogError> {
        let mut providers = self.providers.write().await;
        providers.remove(provider_id);
        Ok(())
    }

    /// # Errors
    ///
    /// Returns an error if the provider cannot be registered.
    /// List all registered providers
    pub async fn list_providers(&self) -> Result<Vec<String>, BearDogError> {
        let providers = self.providers.read().await;
        Ok(providers.keys().cloned().collect())
    }

    /// # Errors
    ///
    /// Returns an error if the provider cannot be registered.
    /// Get provider type by ID
    pub async fn get_provider_type(&self, provider_id: &str) -> Result<Option<String>, BearDogError> {
        let providers = self.providers.read().await;
        Ok(providers.get(provider_id).cloned())
    }
}

impl Default for UniversalHsmRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_registry_operations() {
        let registry = UniversalHsmRegistry::new();
        
        // Register a provider
        registry
            .register_provider("test-provider".to_string(), "software".to_string())
            .await
            ?;
        
        // List providers
        let providers = registry.list_providers().await?;
        assert_eq!(providers.len(), 1);
        
        // Get provider type
        let provider_type = registry.get_provider_type("test-provider").await?;
        assert_eq!(provider_type, Some("software".to_string()));
        
        // Unregister provider
        registry.unregister_provider("test-provider").await?;
        let providers = registry.list_providers().await?;
        assert_eq!(providers.len(), 0);
    }
}
