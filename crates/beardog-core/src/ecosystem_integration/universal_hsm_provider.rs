// Fixed universal_hsm_provider.rs - Universal HSM provider integration
use beardog_errors::BearDogError;
use crate::BearDogCore;
use beardog_types::canonical::{HealthStatus, HsmCapabilities, HsmKey, KeyMetadata};
use tracing::{debug, info, warn, error};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalHsmConfig {
    pub provider_id: String,
    pub provider_type: String,
    pub endpoint: Option<String>,
    pub authentication: Option<HashMap<String, String>>,
    pub capabilities: Vec<String>,
    pub security_level: String,
}

// MODERNIZED: Native async fn in traits (no async_trait needed)
#[allow(async_fn_in_trait)]
pub trait UniversalHsmProvider: Send + Sync {
    async fn initialize(&self, config: &UniversalHsmConfig) -> Result<(), BearDogError>;
    async fn generate_key(&self, key_type: &str, metadata: &KeyMetadata) -> Result<HsmKey, BearDogError>;
    async fn sign_data(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError>;
    async fn verify_signature(&self, key_id: &str, data: &[u8], signature: &[u8]) -> Result<bool, BearDogError>;
    async fn get_capabilities(&self) -> Result<HsmCapabilities, BearDogError>;
    async fn health_check(&self) -> Result<HealthStatus, BearDogError>;
}

pub struct SoftwareHsmProvider {
    config: Option<UniversalHsmConfig>,
    key_store: HashMap<String, HsmKey>,
}

impl SoftwareHsmProvider {
    pub fn new() -> Self {
        Self {
            config: None,
            key_store: HashMap::new(),
        }
    }
}

// MODERNIZED: Native async fn implementation (no async_trait needed)
impl UniversalHsmProvider for SoftwareHsmProvider {
    async fn initialize(&self, config: &UniversalHsmConfig) -> Result<(), BearDogError> {
        info!("🛠️ Initializing Software HSM provider: {}", config.provider_id);
        Ok(())
    }

    async fn generate_key(&self, key_type: &str, metadata: &KeyMetadata) -> Result<HsmKey, BearDogError> {
        info!("🔑 Generating {} key with metadata: {:?}", key_type, metadata);
        
        // In a real implementation, this would generate actual cryptographic keys
        let key = HsmKey {
            key_id: format!("sw-key-{}", uuid::Uuid::new_v4()),
            key_type: key_type.to_string(),
            public_key: vec![0u8; 32], // Mock public key
            metadata: metadata.clone(),
        };
        
        Ok(key)
    }

    async fn sign_data(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        debug!("✍️ Signing data with key: {}", key_id);
        
        // Mock signature - in reality would use actual cryptographic signing
        let mut signature = vec![0u8; 64];
        signature[0..8].copy_from_slice(&data.len().to_le_bytes());
        
        Ok(signature)
    }

    async fn verify_signature(&self, key_id: &str, data: &[u8], signature: &[u8]) -> Result<bool, BearDogError> {
        debug!("🔍 Verifying signature with key: {}", key_id);
        
        // Mock verification - always returns true for demo
        Ok(signature.len() == 64)
    }

    async fn get_capabilities(&self) -> Result<HsmCapabilities, BearDogError> {
        Ok(HsmCapabilities {
            supported_algorithms: vec![
                "ed25519".to_string(),
                "secp256r1".to_string(),
                "rsa2048".to_string(),
            ],
            key_storage: "memory".to_string(),
            attestation_support: false,
            fips_140_2_level: None,
        })
    }

    async fn health_check(&self) -> Result<HealthStatus, BearDogError> {
        Ok(HealthStatus::Healthy)
    }
}

pub struct UniversalHsmManager {
    providers: HashMap<String, Box<dyn UniversalHsmProvider>>,
    active_provider: Option<String>,
}

impl UniversalHsmManager {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            active_provider: None,
        }
    }

    pub fn register_provider(&mut self, provider_id: String, provider: Box<dyn UniversalHsmProvider>) {
        info!("📝 Registering HSM provider: {}", provider_id);
        self.providers.insert(provider_id, provider);
    }

    pub fn set_active_provider(&mut self, provider_id: String) -> Result<(), BearDogError> {
        if self.providers.contains_key(&provider_id) {
            self.active_provider = Some(provider_id);
            Ok(())
        } else {
            Err(BearDogError::business(format!("Provider not found: {}", provider_id)))
        }
    }

    pub async fn generate_key(&self, key_type: &str, metadata: &KeyMetadata) -> Result<HsmKey, BearDogError> {
        let provider_id = self.active_provider.as_ref()
            .ok_or_else(|| BearDogError::business("No active HSM provider".to_string()))?;

        let provider = self.providers.get(provider_id)
            .ok_or_else(|| BearDogError::business("Active provider not found".to_string()))?;

        provider.generate_key(key_type, metadata).await
    }

    pub async fn sign_data(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        let provider_id = self.active_provider.as_ref()
            .ok_or_else(|| BearDogError::business("No active HSM provider".to_string()))?;

        let provider = self.providers.get(provider_id)
            .ok_or_else(|| BearDogError::business("Active provider not found".to_string()))?;

        provider.sign_data(key_id, data).await
    }

    pub async fn health_check_all(&self) -> HashMap<String, HealthStatus> {
        let mut results = HashMap::new();
        
        for (provider_id, provider) in &self.providers {
            let health = provider.health_check().await.unwrap_or(HealthStatus::Unhealthy);
            results.insert(provider_id.clone(), health);
        }
        
        results
    }
}

impl Default for SoftwareHsmProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for UniversalHsmManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_software_hsm_provider() {
        let provider = SoftwareHsmProvider::new();
        let config = UniversalHsmConfig {
            provider_id: "test-sw-hsm".to_string(),
            provider_type: "software".to_string(),
            endpoint: None,
            authentication: None,
            capabilities: vec!["signing".to_string()],
            security_level: "software".to_string(),
        };

        assert!(provider.initialize(&config).await.is_ok());
        assert!(provider.health_check().await.is_ok());
    }

    #[test]
    fn test_universal_hsm_manager() {
        let mut manager = UniversalHsmManager::new();
        let provider = Box::new(SoftwareHsmProvider::new());
        
        manager.register_provider("test-provider".to_string(), provider);
        assert!(manager.set_active_provider("test-provider".to_string()).is_ok());
    }
}
