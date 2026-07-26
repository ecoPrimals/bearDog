// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unified HSM Provider
//!
//! This module provides a unified interface for accessing different HSM providers.

use super::types::{HsmKey, HsmTier, KeyType};
use super::{GenerateKeyRequest, HsmProvider};
use beardog_errors::BearDogError;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info};

/// Unified HSM provider that routes to appropriate backends
pub struct UnifiedHsmProvider {
    providers: Arc<RwLock<HashMap<String, Arc<crate::tunnel::hsm::HsmProviderBackend>>>>,
    default_provider: Option<String>,
}

/// Provider registration info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    /// Unique provider identifier
    pub id: String,
    /// Human-readable provider name
    pub name: String,
    /// HSM security tier
    pub tier: HsmTier,
    /// Whether the provider is currently available
    pub is_available: bool,
}

impl UnifiedHsmProvider {
    /// Creates a new unified HSM provider
    pub fn new() -> Self {
        info!("🔗 Initializing unified HSM provider");
        Self {
            providers: Arc::new(RwLock::new(HashMap::new())),
            default_provider: None,
        }
    }

    /// Registers an HSM provider
    ///
    /// # Errors
    ///
    /// Returns an error if the provider cannot be registered.
    pub fn register_provider(
        &mut self,
        id: String,
        provider: Arc<crate::tunnel::hsm::HsmProviderBackend>,
    ) -> Result<(), BearDogError> {
        info!("📝 Registering HSM provider: {}", id);

        let mut providers = self.providers.write();
        providers.insert(id.clone(), provider);

        if self.default_provider.is_none() {
            self.default_provider = Some(id.clone());
            info!("✅ Set default provider: {}", id);
        }

        Ok(())
    }

    /// Unregisters an HSM provider
    ///
    /// # Errors
    ///
    /// Returns an error if the provider cannot be unregistered.
    pub fn unregister_provider(&mut self, id: &str) -> Result<(), BearDogError> {
        info!("🗑️ Unregistering HSM provider: {}", id);

        let mut providers = self.providers.write();
        providers.remove(id);

        if self.default_provider.as_deref() == Some(id) {
            self.default_provider = providers.keys().next().cloned();
        }

        Ok(())
    }

    /// Sets the default provider
    ///
    /// # Errors
    ///
    /// Returns an error if the provider id is not registered.
    pub fn set_default_provider(&mut self, id: String) -> Result<(), BearDogError> {
        let providers = self.providers.read();

        if !providers.contains_key(&id) {
            return Err(BearDogError::not_found(format!(
                "Provider '{id}' not found"
            )));
        }

        self.default_provider = Some(id.clone());
        info!("✅ Set default provider: {}", id);
        Ok(())
    }

    /// Gets a provider by ID
    ///
    /// # Errors
    ///
    /// Returns an error if the provider is not registered.
    pub fn get_provider(
        &self,
        id: &str,
    ) -> Result<Arc<crate::tunnel::hsm::HsmProviderBackend>, BearDogError> {
        let providers = self.providers.read();

        providers
            .get(id)
            .cloned()
            .ok_or_else(|| BearDogError::not_found(format!("Provider '{id}' not found")))
    }

    /// Gets the default provider
    ///
    /// # Errors
    ///
    /// Returns an error if no default provider is set or lookup fails.
    pub fn get_default_provider(
        &self,
    ) -> Result<Arc<crate::tunnel::hsm::HsmProviderBackend>, BearDogError> {
        let default_id = self
            .default_provider
            .as_ref()
            .ok_or_else(|| BearDogError::not_found("No default provider set".to_string()))?;

        self.get_provider(default_id)
    }
    /// Lists all registered providers
    #[must_use]
    pub fn list_providers(&self) -> Vec<String> {
        let providers = self.providers.read();
        providers.keys().cloned().collect()
    }

    /// Generates a key using a specific provider
    ///
    /// # Errors
    ///
    /// Returns an error if key generation fails in the underlying HSM provider.
    pub async fn generate_key_with_provider(
        &self,
        provider_id: &str,
        key_id: &str,
        key_type: &KeyType,
    ) -> Result<HsmKey, BearDogError> {
        debug!(
            "🔑 Generating key '{}' with provider '{}'",
            key_id, provider_id
        );

        let provider = self.get_provider(provider_id)?;
        let request = GenerateKeyRequest {
            key_id: key_id.to_string(),
            key_type: key_type.clone(),
        };
        provider.generate_key(request).await
    }

    /// Generates a key using the default provider
    ///
    /// # Errors
    ///
    /// Returns an error if key generation fails in the underlying HSM provider.
    pub async fn generate_key(
        &self,
        key_id: &str,
        key_type: &KeyType,
    ) -> Result<HsmKey, BearDogError> {
        debug!("🔑 Generating key '{}' with default provider", key_id);

        let provider = self.get_default_provider()?;
        let request = GenerateKeyRequest {
            key_id: key_id.to_string(),
            key_type: key_type.clone(),
        };
        provider.generate_key(request).await
    }

    /// Signs data using a specific provider
    ///
    /// # Errors
    ///
    /// Returns an error if signing fails in the underlying HSM provider.
    pub async fn sign_with_provider(
        &self,
        provider_id: &str,
        key_id: &str,
        data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        debug!("✍️ Signing with provider '{}'", provider_id);

        let provider = self.get_provider(provider_id)?;
        provider.sign(key_id, data).await
    }

    /// Signs data using the default provider
    ///
    /// # Errors
    ///
    /// Returns an error if signing fails in the underlying HSM provider.
    pub async fn sign(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        debug!("✍️ Signing with default provider");

        let provider = self.get_default_provider()?;
        provider.sign(key_id, data).await
    }

    /// Verifies a signature using a specific provider
    ///
    /// # Errors
    ///
    /// Returns an error if verification fails in the underlying HSM provider.
    pub async fn verify_with_provider(
        &self,
        provider_id: &str,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
        debug!("✅ Verifying with provider '{}'", provider_id);

        let provider = self.get_provider(provider_id)?;
        provider.verify(key_id, data, signature).await
    }

    /// Verifies a signature using the default provider
    ///
    /// # Errors
    ///
    /// Returns an error if verification fails in the underlying HSM provider.
    pub async fn verify(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
        debug!("✅ Verifying with default provider");

        let provider = self.get_default_provider()?;
        provider.verify(key_id, data, signature).await
    }

    /// Deletes a key using a specific provider
    ///
    /// # Errors
    ///
    /// Returns an error if key deletion fails in the underlying HSM provider.
    pub async fn delete_key_with_provider(
        &self,
        provider_id: &str,
        key_id: &str,
    ) -> Result<(), BearDogError> {
        debug!(
            "🗑️ Deleting key '{}' with provider '{}'",
            key_id, provider_id
        );

        let provider = self.get_provider(provider_id)?;
        provider.delete_key(key_id).await
    }

    /// Deletes a key using the default provider
    ///
    /// # Errors
    ///
    /// Returns an error if key deletion fails in the underlying HSM provider.
    pub async fn delete_key(&self, key_id: &str) -> Result<(), BearDogError> {
        debug!("🗑️ Deleting key '{}' with default provider", key_id);

        let provider = self.get_default_provider()?;
        provider.delete_key(key_id).await
    }
}

impl Default for UnifiedHsmProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::super::software_hsm::RustSoftwareHsm;
    use super::super::types::SoftwareHsmConfig;
    use super::*;

    #[test]
    fn test_provider_creation() -> Result<(), Box<dyn std::error::Error>> {
        let provider = UnifiedHsmProvider::new();
        assert!(provider.list_providers().is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_register_provider() -> Result<(), Box<dyn std::error::Error>> {
        let mut unified = UnifiedHsmProvider::new();

        let config = SoftwareHsmConfig::default();
        let software_hsm = RustSoftwareHsm::new(config).await?;

        let result = unified.register_provider(
            "software".to_string(),
            Arc::new(crate::tunnel::hsm::HsmProviderBackend::RustSoftware(
                software_hsm,
            )),
        );

        assert!(result.is_ok());
        assert_eq!(unified.list_providers().len(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_default_provider() -> Result<(), Box<dyn std::error::Error>> {
        let mut unified = UnifiedHsmProvider::new();

        let config = SoftwareHsmConfig::default();
        let software_hsm = RustSoftwareHsm::new(config).await?;

        unified.register_provider(
            "software".to_string(),
            Arc::new(crate::tunnel::hsm::HsmProviderBackend::RustSoftware(
                software_hsm,
            )),
        )?;

        let provider = unified.get_default_provider();
        assert!(provider.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_provider() -> Result<(), Box<dyn std::error::Error>> {
        let mut unified = UnifiedHsmProvider::new();

        let config = SoftwareHsmConfig::default();
        let software_hsm = RustSoftwareHsm::new(config).await?;

        unified.register_provider(
            "software".to_string(),
            Arc::new(crate::tunnel::hsm::HsmProviderBackend::RustSoftware(
                software_hsm,
            )),
        )?;

        let provider = unified.get_provider("software");
        assert!(provider.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_unregister_provider() -> Result<(), Box<dyn std::error::Error>> {
        let mut unified = UnifiedHsmProvider::new();

        let config = SoftwareHsmConfig::default();
        let software_hsm = RustSoftwareHsm::new(config).await?;

        unified.register_provider(
            "software".to_string(),
            Arc::new(crate::tunnel::hsm::HsmProviderBackend::RustSoftware(
                software_hsm,
            )),
        )?;

        assert_eq!(unified.list_providers().len(), 1);

        unified.unregister_provider("software")?;
        assert_eq!(unified.list_providers().len(), 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_set_default_provider() -> Result<(), Box<dyn std::error::Error>> {
        let mut unified = UnifiedHsmProvider::new();

        let config = SoftwareHsmConfig::default();
        let software_hsm1 = RustSoftwareHsm::new(config.clone()).await?;
        let software_hsm2 = RustSoftwareHsm::new(config).await?;

        unified.register_provider(
            "provider1".to_string(),
            Arc::new(crate::tunnel::hsm::HsmProviderBackend::RustSoftware(
                software_hsm1,
            )),
        )?;
        unified.register_provider(
            "provider2".to_string(),
            Arc::new(crate::tunnel::hsm::HsmProviderBackend::RustSoftware(
                software_hsm2,
            )),
        )?;

        let result = unified.set_default_provider("provider2".to_string());
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_generate_key() -> Result<(), Box<dyn std::error::Error>> {
        let mut unified = UnifiedHsmProvider::new();

        let config = SoftwareHsmConfig::default();
        let software_hsm = RustSoftwareHsm::new(config).await?;

        unified.register_provider(
            "software".to_string(),
            Arc::new(crate::tunnel::hsm::HsmProviderBackend::RustSoftware(
                software_hsm,
            )),
        )?;

        let result = unified.generate_key("test_key", &KeyType::Ed25519).await;
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_key_operations() -> Result<(), Box<dyn std::error::Error>> {
        let mut unified = UnifiedHsmProvider::new();

        let config = SoftwareHsmConfig::default();
        let software_hsm = RustSoftwareHsm::new(config).await?;

        unified.register_provider(
            "software".to_string(),
            Arc::new(crate::tunnel::hsm::HsmProviderBackend::RustSoftware(
                software_hsm,
            )),
        )?;

        // Generate key
        let key = unified.generate_key("test_key", &KeyType::Ed25519).await?;

        // Sign data
        let data = b"test data";
        let signature = unified.sign(&key.id, data).await?;

        // Verify signature
        let verified = unified.verify(&key.id, data, &signature).await?;
        assert!(verified);

        // Delete key
        let deleted = unified.delete_key(&key.id).await;
        assert!(deleted.is_ok());
        Ok(())
    }
}
