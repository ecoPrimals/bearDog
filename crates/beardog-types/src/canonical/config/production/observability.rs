// SPDX-License-Identifier: AGPL-3.0-only

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

impl ProductionMetricsConfig {
    /// Default metrics endpoint
    pub const DEFAULT_ENDPOINT: &'static str = "/metrics";

    /// Create ProductionMetricsConfig with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    pub fn with_defaults() -> Self {
        Self {
            enabled: true,
            endpoint: Self::DEFAULT_ENDPOINT.to_string(),
        }
    }

    /// Create ProductionMetricsConfig from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    ///
    /// # Environment Variables
    /// - `BEARDOG_METRICS_ENDPOINT`: Metrics endpoint (default: "/metrics")
    pub fn from_env() -> Self {
        Self {
            enabled: true,
            endpoint: std::env::var("BEARDOG_METRICS_ENDPOINT")
                .unwrap_or_else(|_| Self::DEFAULT_ENDPOINT.to_string()),
        }
    }
}

impl ProductionLoggingConfig {
    /// Default log level
    pub const DEFAULT_LEVEL: &'static str = "info";

    /// Default log format
    pub const DEFAULT_FORMAT: &'static str = "json";

    /// Create ProductionLoggingConfig with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    pub fn with_defaults() -> Self {
        Self {
            level: Self::DEFAULT_LEVEL.to_string(),
            format: Self::DEFAULT_FORMAT.to_string(),
        }
    }

    /// Create ProductionLoggingConfig from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    ///
    /// # Environment Variables
    /// - `BEARDOG_LOG_LEVEL`: Log level (default: "info")
    /// - `BEARDOG_LOG_FORMAT`: Log format (default: "json")
    pub fn from_env() -> Self {
        Self {
            level: std::env::var("BEARDOG_LOG_LEVEL")
                .unwrap_or_else(|_| Self::DEFAULT_LEVEL.to_string()),
            format: std::env::var("BEARDOG_LOG_FORMAT")
                .unwrap_or_else(|_| Self::DEFAULT_FORMAT.to_string()),
        }
    }
}

impl ProductionTracingConfig {
    /// Default tracing endpoint
    pub const DEFAULT_ENDPOINT: &'static str = "/traces";

    /// Create ProductionTracingConfig with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    pub fn with_defaults() -> Self {
        Self {
            enabled: false,
            endpoint: Self::DEFAULT_ENDPOINT.to_string(),
        }
    }

    /// Create ProductionTracingConfig from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    ///
    /// # Environment Variables
    /// - `BEARDOG_TRACING_ENDPOINT`: Tracing endpoint (default: "/traces")
    pub fn from_env() -> Self {
        Self {
            enabled: false,
            endpoint: std::env::var("BEARDOG_TRACING_ENDPOINT")
                .unwrap_or_else(|_| Self::DEFAULT_ENDPOINT.to_string()),
        }
    }
}

impl DashboardConfig {
    /// Default dashboard endpoint
    pub const DEFAULT_ENDPOINT: &'static str = "/dashboard";

    /// Create DashboardConfig with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    pub fn with_defaults() -> Self {
        Self {
            enabled: false,
            endpoint: Self::DEFAULT_ENDPOINT.to_string(),
        }
    }

    /// Create DashboardConfig from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    ///
    /// # Environment Variables
    /// - `BEARDOG_DASHBOARD_ENDPOINT`: Dashboard endpoint (default: "/dashboard")
    pub fn from_env() -> Self {
        Self {
            enabled: false,
            endpoint: std::env::var("BEARDOG_DASHBOARD_ENDPOINT")
                .unwrap_or_else(|_| Self::DEFAULT_ENDPOINT.to_string()),
        }
    }
}

impl Default for ProductionMetricsConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl Default for ProductionLoggingConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl Default for ProductionTracingConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self::with_defaults()
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
