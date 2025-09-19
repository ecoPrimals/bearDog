// HSM Discovery Configuration

use super::HsmConfigValidation;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::time::Duration;

///
/// This struct manages how `BearDog` discovers and connects to available
/// HSM devices and services in the environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedHsmDiscoveryConfig {
    /// Whether HSM discovery is enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Enable automatic discovery of HSM devices
    /// Whether `auto_discovery` is enabled
    pub auto_discovery: bool,
    pub discovery_timeout: Duration,
    /// Whether to automatically detect HSM capabilities
    /// Whether `capability_detection` is enabled
    pub capability_detection: bool,
}

impl Default for UnifiedHsmDiscoveryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_discovery: true,
            discovery_timeout: Duration::from_secs(30),
            capability_detection: true,
        }
    }
}

impl HsmConfigValidation for UnifiedHsmDiscoveryConfig {
    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        Ok(())
    }
    /// Checks if compatible with
    fn is_compatible_with(&self, _other_version: u32) -> bool {
        true
    }
}
