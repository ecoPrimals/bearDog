// SPDX-License-Identifier: AGPL-3.0-or-later

//! HSM Provider Implementation
//!
//! Default implementation of the HSM Provider trait

use super::super::types::HsmKey;
// GenerateKeyRequest imported in tests
use async_trait::async_trait;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Provider information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    /// Provider ID
    pub id: String,
    /// Provider name
    pub name: String,
    /// Security level (1-5, where 5 is highest)
    pub security_level: u8,
}

/// Key information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyInfo {
    /// Key ID
    pub key_id: String,
    /// Key type/algorithm
    pub key_type: String,
    /// Whether this key is backed by hardware security
    pub is_hardware_backed: bool,
}

/// Health status structure  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Whether the provider is healthy
    pub is_healthy: bool,
    /// Optional error message
    pub error_message: Option<String>,
}

/// HSM Provider trait
///
/// # Migration (v0.10.0)
///
/// Superseded by [`beardog_traits::hsm::HsmKeyProvider`] which is the
/// canonical, object-safe HSM abstraction.  Use `HsmProviderRegistry`
/// for runtime provider selection.
/// This trait will be removed in a future release.
#[async_trait]
pub trait HsmProvider: Send + Sync {
    /// Get provider information
    async fn get_info(&self) -> Result<ProviderInfo, BearDogError>;

    /// Generate a new key
    async fn generate_key(
        &self,
        request: crate::tunnel::hsm::GenerateKeyRequest,
    ) -> Result<HsmKey, BearDogError>;

    /// Sign data with a key
    async fn sign(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError>;

    /// Verify a signature
    async fn verify(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError>;

    /// Encrypt data
    async fn encrypt(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError>;

    /// Decrypt data
    async fn decrypt(&self, key_id: &str, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError>;

    /// Import a key
    async fn import_key(&self, key_data: &[u8], key_id: &str) -> Result<HsmKey, BearDogError>;

    /// Delete a key
    async fn delete_key(&self, key_id: &str) -> Result<(), BearDogError>;

    /// Get key information
    async fn get_key_info(&self, key_id: &str) -> Result<KeyInfo, BearDogError>;

    /// Perform a health check
    async fn health_check(&self) -> Result<HealthStatus, BearDogError>;

    /// Check if provider is available
    fn is_available(&self) -> bool;
}

/// Default HSM manager
pub struct DefaultHsmManager {
    /// Registered HSM providers
    pub hsm_providers: HashMap<String, Box<dyn HsmProvider>>,
}

impl DefaultHsmManager {
    /// Create a new HSM manager
    pub fn new() -> Self {
        Self {
            hsm_providers: HashMap::new(),
        }
    }

    /// # Errors
    ///
    /// Returns an error if the provider is not registered.
    /// Register an HSM provider
    pub fn register_provider(
        &mut self,
        id: String,
        provider: Box<dyn HsmProvider>,
    ) -> Result<(), BearDogError> {
        self.hsm_providers.insert(id, provider);
        Ok(())
    }

    /// # Errors
    ///
    /// Returns an error if the provider is not registered.
    /// Get a provider by ID
    pub fn get_provider(&self, id: &str) -> Result<&dyn HsmProvider, BearDogError> {
        self.hsm_providers
            .get(id)
            .map(std::convert::AsRef::as_ref)
            .ok_or_else(|| BearDogError::not_found(format!("Provider not found: {id}")))
    }

    /// List all registered providers
    pub fn list_providers(&self) -> Vec<String> {
        self.hsm_providers.keys().cloned().collect()
    }
}

impl Default for DefaultHsmManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Unit tests for [`DefaultHsmManager`] / [`HsmProvider`]. The in-memory `MockProvider` in
/// this module is not compiled into non-test builds.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tunnel::hsm::GenerateKeyRequest;
    use crate::tunnel::hsm::types::key::{
        KeyAttestation, KeyHealthStatus, KeyMaterial, KeyMetadata, KeyType,
    };
    use chrono::Utc;

    struct MockProvider {
        id: String,
    }

    #[async_trait]
    impl HsmProvider for MockProvider {
        async fn get_info(&self) -> Result<ProviderInfo, BearDogError> {
            Ok(ProviderInfo {
                id: self.id.clone(),
                name: "Mock Provider".to_string(),
                security_level: 1,
            })
        }

        fn is_available(&self) -> bool {
            true
        }

        async fn generate_key(&self, request: GenerateKeyRequest) -> Result<HsmKey, BearDogError> {
            Ok(HsmKey {
                id: request.key_id.clone(),
                hsm_type: "mock".to_string(),
                key_type: request.key_type.clone(),
                metadata: KeyMetadata::new(request.key_id.clone(), request.key_type),
                key_material: KeyMaterial::Encrypted {
                    encrypted_data: vec![0u8; 32],
                    encryption_algorithm: "AES-256-GCM".to_string(),
                    kdf_params: None,
                },
                hsm_tier: "Software".to_string(),
                health_status: KeyHealthStatus::Healthy,
                attestation: Some(KeyAttestation {
                    certificate_chain: vec![],
                    attestation_statement: vec![],
                    format: "mock".to_string(),
                    timestamp: Utc::now(),
                }),
                created_at: Utc::now(),
            })
        }

        async fn sign(&self, _key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
            Ok(data.to_vec())
        }

        async fn verify(
            &self,
            _key_id: &str,
            _data: &[u8],
            _signature: &[u8],
        ) -> Result<bool, BearDogError> {
            Ok(true)
        }

        async fn encrypt(&self, _key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
            Ok(data.to_vec())
        }

        async fn decrypt(&self, _key_id: &str, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError> {
            Ok(ciphertext.to_vec())
        }

        async fn import_key(&self, key_data: &[u8], key_id: &str) -> Result<HsmKey, BearDogError> {
            let _ = key_data; // Suppress warning
            Ok(HsmKey {
                id: key_id.to_string(),
                hsm_type: "mock".to_string(),
                key_type: KeyType::Ed25519,
                metadata: KeyMetadata::new(key_id.to_string(), KeyType::Ed25519),
                key_material: KeyMaterial::Encrypted {
                    encrypted_data: vec![0u8; 32],
                    encryption_algorithm: "AES-256-GCM".to_string(),
                    kdf_params: None,
                },
                hsm_tier: "Software".to_string(),
                health_status: KeyHealthStatus::Healthy,
                attestation: Some(KeyAttestation {
                    certificate_chain: vec![],
                    attestation_statement: vec![],
                    format: "mock".to_string(),
                    timestamp: Utc::now(),
                }),
                created_at: Utc::now(),
            })
        }

        async fn delete_key(&self, _key_id: &str) -> Result<(), BearDogError> {
            Ok(())
        }

        async fn get_key_info(&self, key_id: &str) -> Result<KeyInfo, BearDogError> {
            Ok(KeyInfo {
                key_id: key_id.to_string(),
                key_type: "Ed25519".to_string(),
                is_hardware_backed: false, // Software HSM
            })
        }

        async fn health_check(&self) -> Result<HealthStatus, BearDogError> {
            Ok(HealthStatus {
                is_healthy: true,
                error_message: None,
            })
        }
    }

    #[test]
    fn test_manager_creation() -> Result<(), Box<dyn std::error::Error>> {
        let manager = DefaultHsmManager::new();
        assert!(manager.hsm_providers.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_register_provider() -> Result<(), Box<dyn std::error::Error>> {
        let mut manager = DefaultHsmManager::new();
        let provider = Box::new(MockProvider {
            id: "mock-1".to_string(),
        });

        let result = manager.register_provider("mock-1".to_string(), provider);
        assert!(result.is_ok());
        assert_eq!(manager.list_providers().len(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_provider() -> Result<(), Box<dyn std::error::Error>> {
        let mut manager = DefaultHsmManager::new();
        let provider = Box::new(MockProvider {
            id: "mock-1".to_string(),
        });

        manager.register_provider("mock-1".to_string(), provider)?;

        let retrieved = manager.get_provider("mock-1");
        assert!(retrieved.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_provider_info() -> Result<(), Box<dyn std::error::Error>> {
        let provider = MockProvider {
            id: "mock-1".to_string(),
        };
        let info = provider.get_info().await?;

        assert_eq!(info.id, "mock-1");
        assert_eq!(info.name, "Mock Provider");
        assert_eq!(info.security_level, 1);
        Ok(())
    }
}
