// SPDX-License-Identifier: AGPL-3.0-only

// HSM Security Configuration

use super::HsmConfigValidation;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// HSM security configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedHsmSecurityConfig {
    /// Encryption Enabled
    /// Whether encryption is enabled
    pub encryption_enabled: bool,
    /// Audit Logging
    /// Whether `audit_logging` is enabled
    pub audit_logging: bool,
    /// Tamper Detection
    /// Whether `tamper_detection` is enabled
    pub tamper_detection: bool,
    /// Access Control
    /// Whether `access_control` is enabled
    pub access_control: bool,
}

impl Default for UnifiedHsmSecurityConfig {
    fn default() -> Self {
        Self {
            encryption_enabled: true,
            audit_logging: true,
            tamper_detection: true,
            access_control: true,
        }
    }
}

impl HsmConfigValidation for UnifiedHsmSecurityConfig {
    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        Ok(())
    }
    /// Checks if compatible with
    fn is_compatible_with(&self, _other_version: u32) -> bool {
        true
    }
}
