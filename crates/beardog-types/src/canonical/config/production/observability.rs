// Observability Configuration
//
// This module contains production observability configuration including
// metrics, logging, tracing, alerting, and dashboard settings.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// **OBSERVABILITY CONFIGURATION** - Production observability
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ObservabilityConfig {
    /// Metrics configuration
    /// The metrics value
    pub metrics: ProductionMetricsConfig,

    /// Logging configuration
    /// The logging value
    pub logging: ProductionLoggingConfig,

    /// Tracing configuration
    /// The tracing value
    pub tracing: ProductionTracingConfig,

    /// Alerting configuration
    /// The alerting value
    pub alerting: ProductionAlertingConfig,

    /// Dashboards configuration
    /// The dashboards value
    pub dashboards: DashboardConfig,
}

/// Production metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionMetricsConfig {
    /// Enable metrics collection
    /// Whether feature is enabled
    pub enabled: bool,
    /// Metrics collection endpoint
    /// The endpoint value
    pub endpoint: String,
}

/// Production logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionLoggingConfig {
    /// Log level
    /// The level value
    pub level: String,
    pub format: String,
}

/// Production tracing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionTracingConfig {
    /// Enable distributed tracing
    /// Whether feature is enabled
    pub enabled: bool,
    /// Tracing endpoint
    /// The endpoint value
    pub endpoint: String,
}

/// Production alerting configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductionAlertingConfig {
    /// Enable alerting
    /// Whether feature is enabled
    pub enabled: bool,
    /// Alert channels
    /// Collection of channels
    pub channels: Vec<String>,
}

/// Dashboard configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    /// Enable dashboards
    /// Whether feature is enabled
    pub enabled: bool,
    /// Dashboard endpoint
    /// The endpoint value
    pub endpoint: String,
}

impl Default for ProductionMetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: "/metrics".to_string(),
        }
    }
}

impl Default for ProductionLoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "json".to_string(),
        }
    }
}

impl Default for ProductionTracingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoint: "/traces".to_string(),
        }
    }
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoint: "/dashboard".to_string(),
        }
    }
}

impl ObservabilityConfig {
    /// Validate
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        Ok(())
    }
}
