// SPDX-License-Identifier: AGPL-3.0-or-later

//! Software HSM configuration

use beardog_types::constants::domains::system::defaults;
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
        // Use environment-driven key storage path
        // Priority: BEARDOG_KEY_STORAGE -> XDG_DATA_HOME/beardog/keys -> /tmp/beardog/keys
        let key_storage_path = defaults::resolve_key_storage_dir();
        
        Self {
            provider_id: "software-hsm".to_string(),
            enable_attestation: true,
            enable_entropy: true,
            key_storage_path,
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
