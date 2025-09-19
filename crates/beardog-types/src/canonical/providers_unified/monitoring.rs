// Monitoring Configuration
//
// Provider monitoring, metrics, and observability configuration.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Provider monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderMonitoringConfig {
    /// Monitoring enabled
    /// Whether feature is enabled
    pub enabled: bool,

    /// Metrics collection enabled
    /// Whether metrics is enabled
    pub metrics_enabled: bool,

    /// Tracing enabled
    /// Whether tracing is enabled
    pub tracing_enabled: bool,

    /// Logging configuration
    /// The logging value
    pub logging: LoggingConfig,

    /// Metrics configuration
    /// The metrics value
    pub metrics: MetricsConfig,

    /// Alerting configuration
    /// The alerting value
    pub alerting: AlertingConfig,

    /// Custom monitoring hooks
    /// Collection of custom hooks
    pub custom_hooks: Vec<CustomMonitoringHook>,
}

impl Default for ProviderMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics_enabled: true,
            tracing_enabled: true,
            logging: LoggingConfig::default(),
            metrics: MetricsConfig::default(),
            alerting: AlertingConfig::default(),
            custom_hooks: Vec::new(),
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoggingConfig {
    /// Log level
    /// The level value
    pub level: String,
}

/// Metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricsConfig {
    /// Metrics collection interval
    /// The collection interval value
    pub collection_interval: Duration,
}

/// Alerting configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlertingConfig {
    /// Alerting enabled
    /// Whether feature is enabled
    pub enabled: bool,
}

/// Custom monitoring hook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMonitoringHook {
    /// Hook name
    /// Name of the item
    pub name: String,
    /// Hook enabled
    /// Whether feature is enabled
    pub enabled: bool,
}
