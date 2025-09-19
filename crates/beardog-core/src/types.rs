// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearDogConfig {
    /// Environment in which the system is running (development, production, etc.)
    /// The environment value
    pub environment: String,
    /// The version value
    pub version: String,
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
