// SPDX-License-Identifier: AGPL-3.0-or-later

//! Core Types - BearDog Configuration
//!
//! Provides legacy configuration types for backward compatibility.
//! New code should prefer `beardog-types` canonical types.

use serde::{Deserialize, Serialize};

/// Legacy `BearDog` configuration structure
///
/// **Note:** This is a legacy configuration type. New code should use
/// `UnifiedBearDogConfig` from `beardog-types::canonical::config::unified`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearDogConfig {
    /// Environment in which the system is running (development, production, etc.)
    pub environment: String,
    /// `BearDog` system version
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beardog_config_default() {
        let config = BearDogConfig::default();
        assert_eq!(config.environment, "development");
        assert_eq!(config.version, "3.0.0");
        assert!(!config.node_id.is_empty());
    }

    #[test]
    fn test_beardog_config_unique_node_id() {
        let config1 = BearDogConfig::default();
        let config2 = BearDogConfig::default();
        assert_ne!(config1.node_id, config2.node_id);
    }

    #[test]
    fn test_beardog_config_clone() {
        let config = BearDogConfig::default();
        let cloned = config.clone();
        assert_eq!(config.environment, cloned.environment);
        assert_eq!(config.version, cloned.version);
    }

    #[test]
    fn test_beardog_config_serialization() {
        let config = BearDogConfig::default();
        let serialized = serde_json::to_string(&config).expect("serialize");
        assert!(serialized.contains("development"));
        assert!(serialized.contains("3.0.0"));

        let deserialized: BearDogConfig = serde_json::from_str(&serialized).expect("deserialize");
        assert_eq!(config.environment, deserialized.environment);
    }

    #[test]
    fn test_beardog_config_custom() {
        let config = BearDogConfig {
            environment: "production".to_string(),
            version: "4.0.0".to_string(),
            node_id: "custom-node".to_string(),
        };
        assert_eq!(config.environment, "production");
        assert_eq!(config.version, "4.0.0");
        assert_eq!(config.node_id, "custom-node");
    }
}
