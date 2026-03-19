// SPDX-License-Identifier: AGPL-3.0-only

// Software HSM Configuration

use super::HsmConfigValidation;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// Software HSM configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedSoftwareHsmConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Implementation
    /// The implementation value
    pub implementation: String,
    /// Memory Protection
    /// Whether `memory_protection` is enabled
    pub memory_protection: bool,
    /// Key Caching
    /// Whether `key_caching` is enabled
    pub key_caching: bool,
}

impl Default for UnifiedSoftwareHsmConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            implementation: "BearDogSoftHSM".to_string(),
            memory_protection: true,
            key_caching: true,
        }
    }
}

impl HsmConfigValidation for UnifiedSoftwareHsmConfig {
    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        Ok(())
    }
    /// Checks if compatible with
    fn is_compatible_with(&self, _other_version: u32) -> bool {
        true
    }
}
