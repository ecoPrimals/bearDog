// SPDX-License-Identifier: AGPL-3.0-only

// HSM Key Management Configuration

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// High-level key lifecycle switches (rotation, backup) for the unified HSM profile.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HsmKeyManagementConfig {
    /// Key Rotation Enabled
    /// Whether key_rotation is enabled
    pub key_rotation_enabled: bool,
    /// Backup Enabled
    /// Whether backup is enabled
    pub backup_enabled: bool,
}

impl HsmKeyManagementConfig {
    /// Validate
    /// Validates input
    /// Validates input
    pub const fn validate(&self) -> Result<(), BearDogError> {
        Ok(())
    }
}
