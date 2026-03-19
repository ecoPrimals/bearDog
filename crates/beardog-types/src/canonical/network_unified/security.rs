// SPDX-License-Identifier: AGPL-3.0-only

// Network Security Configuration

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkSecurityConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Firewall Enabled
    /// Whether firewall is enabled
    pub firewall_enabled: bool,
}

impl NetworkSecurityConfig {
    /// Validate
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        Ok(())
    }
}
