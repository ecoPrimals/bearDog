// SPDX-License-Identifier: AGPL-3.0-only

// HSM Platform Configuration

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlatformConfig {
    /// Android Enabled
    pub android_enabled: bool,
    /// Ios Enabled
    /// Whether ios is enabled
    pub ios_enabled: bool,
}

impl PlatformConfig {
    /// Validate
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        Ok(())
    }
}
