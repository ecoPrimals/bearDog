//! Software HSM configuration

use serde::{Deserialize, Serialize};

/// Software HSM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareHsmConfig {
    /// Provider identifier
    pub provider_id: String,
    /// Whether to enable attestation
    pub enable_attestation: bool,
    /// Whether to enable entropy collection
    pub enable_entropy: bool,
    /// Key storage path
    pub key_storage_path: String,
}

impl Default for SoftwareHsmConfig {
    fn default() -> Self {
        Self {
            provider_id: "software-hsm".to_string(),
            enable_attestation: true,
            enable_entropy: true,
            key_storage_path: "/tmp/beardog/keys".to_string(),
        }
    }
}

impl SoftwareHsmConfig {
    /// Create new configuration
    pub fn new(provider_id: impl Into<String>) -> Self {
        Self {
            provider_id: provider_id.into(),
            ..Default::default()
        }
    }

    /// Set key storage path
    pub fn with_key_storage_path(mut self, path: impl Into<String>) -> Self {
        self.key_storage_path = path.into();
        self
    }
}
