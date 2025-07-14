//! Monitoring and observability configuration types
//!
//! Contains all logging, metrics, audit, and monitoring configuration structures.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    pub level: String,
    /// Log format
    pub format: String,
    /// Log output configuration
    pub output: LoggingOutputConfig,
}

/// Logging output configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingOutputConfig {
    /// Console output
    pub console: bool,
    /// File output
    pub file: Option<String>,
    /// Syslog output
    pub syslog: bool,
}

/// Metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enable metrics collection
    pub enabled: bool,
    /// Metrics collection interval
    pub collection_interval: Duration,
    /// Prometheus metrics configuration
    pub prometheus: PrometheusConfig,
}

/// Prometheus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrometheusConfig {
    /// Enable Prometheus metrics
    pub enabled: bool,
    /// Metrics endpoint
    pub endpoint: String,
    /// Metrics port
    pub port: u16,
}

// Default implementations
impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "INFO".to_string(),
            format: "json".to_string(),
            output: LoggingOutputConfig::default(),
        }
    }
}

impl Default for LoggingOutputConfig {
    fn default() -> Self {
        Self {
            console: true,
            file: None,
            syslog: false,
        }
    }
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(60),
            prometheus: PrometheusConfig::default(),
        }
    }
}

impl Default for PrometheusConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoint: "/metrics".to_string(),
            port: 9090,
        }
    }
}
