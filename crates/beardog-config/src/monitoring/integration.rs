//! Integration Configuration
//!
//! This module defines integration configurations for monitoring.

use serde::{Deserialize, Serialize};

/// Integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    /// Enable integrations
    pub enabled: bool,
    /// Integration endpoints
    pub endpoints: Vec<String>,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoints: Vec::new(),
        }
    }
}

impl IntegrationConfig {
    /// Create production integration configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            endpoints: vec!["prometheus".to_string(), "grafana".to_string()],
        }
    }

    /// Create development integration configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            endpoints: Vec::new(),
        }
    }
}
