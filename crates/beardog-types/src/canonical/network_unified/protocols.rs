// SPDX-License-Identifier: AGPL-3.0-only

// Network Protocols Configuration

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProtocolsConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Supported Protocols
    /// Collection of supported protocols
    pub supported_protocols: Vec<String>,
}

impl ProtocolsConfig {
    /// Validate
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        Ok(())
    }
}
