// SPDX-License-Identifier: AGPL-3.0-or-later

// Security Configuration

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
}

impl SecurityConfig {
    /// Validate
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        Ok(())
    }
}
