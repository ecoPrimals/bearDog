//! Metrics Configuration
//!
//! This module defines metrics collection and processing configurations.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Metrics collection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enable metrics collection
    pub enabled: bool,
    /// Collection interval
    pub collection_interval: Duration,
    /// Metrics collectors
    pub collectors: Vec<String>,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(30),
            collectors: vec!["cpu".to_string(), "memory".to_string()],
        }
    }
}

impl MetricsConfig {
    /// Create production metrics configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(10),
            collectors: vec![
                "cpu".to_string(),
                "memory".to_string(),
                "network".to_string(),
                "disk".to_string(),
            ],
        }
    }

    /// Create development metrics configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            collection_interval: Duration::from_secs(60),
            collectors: vec!["cpu".to_string()],
        }
    }
}
