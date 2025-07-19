//! Alerting Configuration
//!
//! This module defines alerting configurations for monitoring.

use serde::{Deserialize, Serialize};

/// Alerting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertingConfig {
    /// Enable alerting
    pub enabled: bool,
    /// Alert rules
    pub rules: Vec<String>,
    /// Alert channels
    pub channels: Vec<String>,
}

impl Default for AlertingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: vec!["cpu_high".to_string()],
            channels: vec!["email".to_string()],
        }
    }
}

impl AlertingConfig {
    /// Create production alerting configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            rules: vec!["cpu_high".to_string(), "memory_high".to_string()],
            channels: vec!["email".to_string(), "slack".to_string()],
        }
    }

    /// Create development alerting configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            rules: Vec::new(),
            channels: Vec::new(),
        }
    }
}
