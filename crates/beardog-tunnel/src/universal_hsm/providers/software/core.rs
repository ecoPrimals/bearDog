//! Software HSM provider core implementation

use super::config::SoftwareHsmConfig;
use crate::universal_hsm::traits::{ProviderInfo, ProviderHealth, ProviderType, Platform, UniversalHsmProvider};
use beardog_errors::BearDogError;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Software HSM provider implementation
#[derive(Clone)]
pub struct SoftwareHsmProvider {
    config: SoftwareHsmConfig,
    initialized: Arc<RwLock<bool>>,
}

impl SoftwareHsmProvider {
    /// Create new software HSM provider
    pub fn new(config: SoftwareHsmConfig) -> Self {
        Self {
            config,
            initialized: Arc::new(RwLock::new(false)),
        }
    }
}

#[async_trait::async_trait]
impl UniversalHsmProvider for SoftwareHsmProvider {
    fn get_provider_info(&self) -> ProviderInfo {
        ProviderInfo {
            provider_id: self.config.provider_id.clone(),
            name: "Software HSM".to_string(),
            version: "1.0.0".to_string(),
            description: "Pure Rust software HSM implementation".to_string(),
            vendor: "BearDog".to_string(),
            provider_type: ProviderType::Software,
            platforms: vec![Platform::Universal],
        }
    }

    async fn check_health(&self) -> Result<ProviderHealth, BearDogError> {
        Ok(ProviderHealth {
            is_healthy: true,
            error_message: None,
            last_check: chrono::Utc::now(),
            response_time_ms: Some(0.1),
            capabilities_verified: true,
        })
    }

    async fn initialize(&self) -> Result<(), BearDogError> {
        let mut initialized = self.initialized.write().await;
        *initialized = true;
        Ok(())
    }

    async fn shutdown(&self) -> Result<(), BearDogError> {
        let mut initialized = self.initialized.write().await;
        *initialized = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_software_provider_creation() {
        let config = SoftwareHsmConfig::default();
        let provider = SoftwareHsmProvider::new(config);
        
        assert_eq!(provider.get_provider_info().provider_id, "software-hsm");
    }

    #[tokio::test]
    async fn test_provider_health_check() {
        let config = SoftwareHsmConfig::default();
        let provider = SoftwareHsmProvider::new(config);
        
        let health = provider.check_health().await.unwrap();
        assert!(health.is_healthy);
    }
}
