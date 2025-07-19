//! Dashboard Configuration
//!
//! This module defines dashboard configurations for monitoring.

use serde::{Deserialize, Serialize};

/// Dashboard configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    /// Enable dashboards
    pub enabled: bool,
    /// Dashboard definitions
    pub dashboards: Vec<String>,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            dashboards: vec!["system".to_string()],
        }
    }
}

impl DashboardConfig {
    /// Create production dashboard configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            dashboards: vec!["system".to_string(), "application".to_string()],
        }
    }

    /// Create development dashboard configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            dashboards: Vec::new(),
        }
    }
}
