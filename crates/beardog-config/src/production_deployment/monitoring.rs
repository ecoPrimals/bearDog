//! Monitoring Configuration
//!
//! This module defines monitoring configurations for production deployments.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Deployment monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentMonitoring {
    /// Enable monitoring
    pub enabled: bool,
    /// Monitoring metrics
    pub metrics: Vec<MonitoringMetric>,
    /// Monitoring interval
    pub interval: Duration,
}

/// Monitoring metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringMetric {
    /// Metric name
    pub name: String,
    /// Metric type
    pub metric_type: String,
    /// Metric threshold
    pub threshold: f64,
}

impl Default for DeploymentMonitoring {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics: Vec::new(),
            interval: Duration::from_secs(60),
        }
    }
}

impl DeploymentMonitoring {
    /// Create production monitoring configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            metrics: vec![
                MonitoringMetric {
                    name: "cpu_usage".to_string(),
                    metric_type: "gauge".to_string(),
                    threshold: 80.0,
                },
                MonitoringMetric {
                    name: "memory_usage".to_string(),
                    metric_type: "gauge".to_string(),
                    threshold: 80.0,
                },
            ],
            interval: Duration::from_secs(30),
        }
    }

    /// Create development monitoring configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            metrics: Vec::new(),
            interval: Duration::from_secs(300),
        }
    }
}
