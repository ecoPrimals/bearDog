// SPDX-License-Identifier: AGPL-3.0-or-later

//! Software HSM provider core implementation

use super::config::SoftwareHsmConfig;
use crate::tunnel::hsm::types::{HsmCapability, KeyType};
use crate::universal_hsm::traits::{ProviderInfo, ProviderType, UniversalHsmProvider};
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

    /// Provider id from configuration (not part of [`UniversalHsmProvider`], but used by callers/tests).
    pub fn provider_id(&self) -> &str {
        &self.config.provider_id
    }
}

impl UniversalHsmProvider for SoftwareHsmProvider {
    fn get_provider_info(&self) -> ProviderInfo {
        ProviderInfo {
            name: self.config.provider_id.clone(),
            provider_type: ProviderType::Software,
            version: "1.0.0".to_string(),
            capabilities_verified: true,
        }
    }

    fn generate_key(&self, _key_type: KeyType) -> Result<Vec<u8>, BearDogError> {
        Err(BearDogError::unsupported_operation(
            "SoftwareHsmProvider::generate_key is not wired in this build",
        ))
    }

    fn sign(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Err(BearDogError::unsupported_operation(
            "SoftwareHsmProvider::sign is not wired in this build",
        ))
    }

    fn verify(
        &self,
        _key_id: &str,
        _data: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {
        Err(BearDogError::unsupported_operation(
            "SoftwareHsmProvider::verify is not wired in this build",
        ))
    }

    fn get_capabilities(&self) -> Vec<HsmCapability> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_software_provider_creation() {
        let config = SoftwareHsmConfig::default();
        let provider = SoftwareHsmProvider::new(config);

        assert_eq!(provider.provider_id(), "software-hsm");
    }

    #[test]
    fn test_provider_info() {
        let config = SoftwareHsmConfig::default();
        let provider = SoftwareHsmProvider::new(config);
        let info = provider.get_provider_info();
        assert_eq!(info.provider_type, ProviderType::Software);
        assert!(info.capabilities_verified);
    }
}
