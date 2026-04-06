// SPDX-License-Identifier: AGPL-3.0-or-later

// Network Routing Configuration

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RoutingConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Routes
    /// Collection of routes
    pub routes: Vec<String>,
}

impl RoutingConfig {
    /// Validate
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        Ok(())
    }
}
