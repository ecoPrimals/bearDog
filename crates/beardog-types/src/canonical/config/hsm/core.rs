// Core HSM Configuration Types

use super::HsmConfigValidation;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// Core HSM configuration shared across all HSM types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreHsmConfig {
    /// Whether HSM functionality is enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Default HSM provider to use (if not specified)
    pub default_provider: Option<String>,
    /// Whether to enable fallback to software HSM
    /// Whether fallback is enabled
    pub fallback_enabled: bool,
    /// Whether `strict_mode` is enabled
    pub strict_mode: bool,
}

impl Default for CoreHsmConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_provider: None,
            fallback_enabled: true,
            strict_mode: false,
        }
    }
}

impl HsmConfigValidation for CoreHsmConfig {
    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        Ok(())
    }
    /// Checks if compatible with
    fn is_compatible_with(&self, _other_version: u32) -> bool {
        true
    }
}
