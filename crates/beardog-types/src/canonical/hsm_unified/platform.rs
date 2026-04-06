// SPDX-License-Identifier: AGPL-3.0-or-later

// HSM Platform Configuration

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// Which mobile or embedded HSM platforms are enabled for this deployment.
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
    ///
    /// # Errors
    ///
    /// Never returns an error; reserved for future validation rules.
    pub const fn validate(&self) -> Result<(), BearDogError> {
        Ok(())
    }
}
