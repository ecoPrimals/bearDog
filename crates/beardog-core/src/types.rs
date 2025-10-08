// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use serde::{Deserialize, Serialize};

/// Legacy `BearDog` configuration structure
///
/// **Note:** This is a legacy configuration type. New code should use
/// `UnifiedBearDogConfig` from `beardog-types::canonical::config::unified`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearDogConfig {
    /// Environment in which the system is running (development, production, etc.)
    /// The environment value
    pub environment: String,
    /// The version value
    pub version: String,
    /// Unique identifier for this node instance
    pub node_id: String,
}

impl Default for BearDogConfig {
    fn default() -> Self {
        Self {
            environment: "development".to_string(),
            version: "3.0.0".to_string(),
            node_id: uuid::Uuid::new_v4().to_string(),
        }
    }
}
