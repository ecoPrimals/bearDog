// SPDX-License-Identifier: AGPL-3.0-only

// Access Control Configuration

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccessControlConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Policies
    /// Collection of policies
    pub policies: Vec<String>,
}

impl AccessControlConfig {
    /// Validate
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        Ok(())
    }
}
