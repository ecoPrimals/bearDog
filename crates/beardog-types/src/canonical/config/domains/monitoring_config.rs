// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Monitoring Configuration Domain
//!
//! ⚠️  DEPRECATED: This domain-specific monitoring config is being consolidated.
//! Use `crate::canonical::monitoring::MonitoringConfig` instead for new code.
//!
//! This module contains all monitoring-related configuration types, extracted from
//! the large `consolidated_domains.rs` file for better maintainability.

use beardog_config::env_keys;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use crate::canonical::config::r#trait::BearDogConfig;
use crate::constants::time;

/// **CONSOLIDATED MONITORING CONFIGURATION** - Unifies all monitoring configs
///
/// Consolidates: `MetricsConfig`, `AnalysisConfig`, `HealthCheckConfig`, `AnomalyConfig`,
/// `TrendConfig`, `SecuritySentinelConfig`, etc.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConsolidatedMonitoringConfig {
    /// Enable monitoring
    pub enabled: bool,

    /// Metrics collection configuration
    pub metrics: MetricsCollectionConfig,

    /// Analysis and processing configuration
    pub analysis: AnalysisProcessingConfig,

    /// Health monitoring configuration
    pub health: HealthMonitoringConfig,

    /// Anomaly detection configuration
    pub anomaly: AnomalyDetectionConfig,

    /// Trend analysis configuration
    pub trends: TrendAnalysisConfig,

    /// Security monitoring configuration
    pub security: SecurityMonitoringConfig,

    /// Export and integration configuration
    pub export: ExportIntegrationConfig,
}

/// Metrics collection configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MetricsCollectionConfig {
    /// Collection interval
    pub interval: Duration,

    /// Metrics to collect
    pub metrics: Vec<String>,

    /// Collection strategy
    pub strategy: String,

    /// Buffer size for metrics
    pub buffer_size: usize,

    /// Retention period
    pub retention_period: Duration,
}

/// Analysis and processing configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AnalysisProcessingConfig {
    /// Enable real-time analysis
    pub real_time: bool,

    /// Analysis algorithms
    pub algorithms: Vec<String>,

    /// Processing window size
    pub window_size: Duration,

    /// Statistical methods
    pub statistical_methods: Vec<String>,

    /// Machine learning models
    pub ml_models: Vec<String>,
}

/// Health monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HealthMonitoringConfig {
    /// Enable health monitoring
    pub enabled: bool,

    /// Health check interval
    pub check_interval: Duration,

    /// Health endpoints
    pub endpoints: Vec<String>,

    /// Health thresholds
    pub thresholds: HashMap<String, f64>,

    /// Recovery actions
    pub recovery_actions: HashMap<String, String>,
}

/// Anomaly detection configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnomalyDetectionConfig {
    /// Enable anomaly detection
    pub enabled: bool,

    /// Detection algorithms
    pub algorithms: Vec<String>,

    /// Sensitivity level
    pub sensitivity: f64,

    /// Training window
    pub training_window: Duration,

    /// Alert thresholds
    pub alert_thresholds: HashMap<String, f64>,
}

/// Trend analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrendAnalysisConfig {
    /// Enable trend analysis
    pub enabled: bool,

    /// Analysis period
    pub analysis_period: Duration,

    /// Trend indicators
    pub indicators: Vec<String>,

    /// Forecasting models
    pub forecasting_models: Vec<String>,

    /// Prediction horizon
    pub prediction_horizon: Duration,
}

/// Security monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityMonitoringConfig {
    /// Enable security monitoring
    pub enabled: bool,

    /// Security event types to monitor
    pub event_types: Vec<String>,

    /// Threat detection rules
    pub threat_rules: Vec<String>,

    /// Alert thresholds
    pub alert_thresholds: HashMap<String, f64>,

    /// Response actions
    pub response_actions: HashMap<String, String>,
}

/// Export and integration configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExportIntegrationConfig {
    /// Export destinations
    pub destinations: Vec<ExportDestination>,

    /// Export format
    pub format: String,

    /// Export interval
    pub interval: Duration,

    /// Compression settings
    pub compression: CompressionConfig,
}

/// Export destination configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExportDestination {
    /// Destination type
    pub destination_type: String,

    /// Connection URL
    pub url: String,

    /// Authentication
    pub auth: HashMap<String, String>,

    /// Export filters
    pub filters: Vec<String>,
}

/// Compression configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CompressionConfig {
    /// Enable compression
    pub enabled: bool,

    /// Compression algorithm
    pub algorithm: String,

    /// Compression level
    pub level: u8,
}

/// Basic monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MonitoringConfig {
    /// Enable monitoring
    pub enabled: bool,

    /// Metrics collection interval
    pub metrics_interval: Duration,

    /// Alert configurations
    pub alerts: Vec<AlertConfig>,

    /// Dashboard configuration
    pub dashboard: DashboardConfig,
}

/// Alert configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AlertConfig {
    /// Alert name
    pub name: String,

    /// Alert condition
    pub condition: String,

    /// Alert threshold
    pub threshold: f64,

    /// Alert severity
    pub severity: String,

    /// Notification channels
    pub channels: Vec<String>,
}

/// Dashboard configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DashboardConfig {
    /// Enable dashboard
    pub enabled: bool,

    /// Refresh interval
    pub refresh_interval: Duration,

    /// Metrics to display
    pub metrics: Vec<String>,

    /// Dashboard layout
    pub layout: String,
}

// Default implementations
impl Default for ConsolidatedMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics: MetricsCollectionConfig::default(),
            analysis: AnalysisProcessingConfig::default(),
            health: HealthMonitoringConfig::default(),
            anomaly: AnomalyDetectionConfig::default(),
            trends: TrendAnalysisConfig::default(),
            security: SecurityMonitoringConfig::default(),
            export: ExportIntegrationConfig::default(),
        }
    }
}

impl MetricsCollectionConfig {
    /// Create configuration from a config source (modern pattern)
    pub fn from_source(source: &dyn crate::canonical::config::source::ConfigSource) -> Self {
        use crate::canonical::config::source::get_parsed;
        use std::time::Duration;

        Self {
            interval: Duration::from_secs(get_parsed(
                source,
                "BEARDOG_METRICS_COLLECTION_INTERVAL_SECS",
                60,
            )),
            metrics: vec!["cpu".to_string(), "memory".to_string()],
            strategy: source.get_or("BEARDOG_METRICS_STRATEGY", "push"),
            buffer_size: get_parsed(source, "BEARDOG_METRICS_BUFFER_SIZE", 1000),
            retention_period: Duration::from_secs(get_parsed(
                source,
                "BEARDOG_METRICS_RETENTION_PERIOD_SECS",
                time::SECONDS_PER_DAY * 7,
            )),
        }
    }
}

impl Default for MetricsCollectionConfig {
    fn default() -> Self {
        use crate::canonical::config::source::EnvConfigSource;
        Self::from_source(&EnvConfigSource::new())
    }
}

impl AnalysisProcessingConfig {
    /// Create configuration from a config source (modern pattern)
    pub fn from_source(source: &dyn crate::canonical::config::source::ConfigSource) -> Self {
        use crate::canonical::config::source::{get_bool, get_parsed};
        use std::time::Duration;

        Self {
            real_time: get_bool(source, "BEARDOG_ANALYSIS_REAL_TIME", true),
            algorithms: vec![
                "moving_average".to_string(),
                "exponential_smoothing".to_string(),
            ],
            window_size: Duration::from_secs(get_parsed(
                source,
                "BEARDOG_ANALYSIS_WINDOW_SIZE_SECS",
                300,
            )),
            statistical_methods: vec!["mean".to_string(), "stddev".to_string()],
            ml_models: vec!["linear_regression".to_string()],
        }
    }
}

impl Default for AnalysisProcessingConfig {
    fn default() -> Self {
        use crate::canonical::config::source::EnvConfigSource;
        Self::from_source(&EnvConfigSource::new())
    }
}

impl HealthMonitoringConfig {
    /// Create configuration from a config source (modern pattern)
    pub fn from_source(source: &dyn crate::canonical::config::source::ConfigSource) -> Self {
        use crate::canonical::config::source::{get_bool, get_parsed};
        use std::collections::HashMap;
        use std::time::Duration;

        let mut thresholds = HashMap::new();
        thresholds.insert(
            "response_time_ms".to_string(),
            get_parsed(source, "BEARDOG_RESPONSE_TIME_THRESHOLD_MS", 1000.0),
        );
        thresholds.insert("error_rate".to_string(), 0.05);

        Self {
            enabled: get_bool(source, "BEARDOG_HEALTH_MONITORING_ENABLED", true),
            check_interval: Duration::from_secs(get_parsed(
                source,
                "BEARDOG_HEALTH_CHECK_INTERVAL_SECS",
                30,
            )),
            endpoints: vec!["/health".to_string(), "/ready".to_string()],
            thresholds,
            recovery_actions: HashMap::new(),
        }
    }
}

impl Default for HealthMonitoringConfig {
    fn default() -> Self {
        use crate::canonical::config::source::EnvConfigSource;
        Self::from_source(&EnvConfigSource::new())
    }
}

impl AnomalyDetectionConfig {
    /// Create configuration from a config source (modern pattern)
    pub fn from_source(source: &dyn crate::canonical::config::source::ConfigSource) -> Self {
        use crate::canonical::config::source::get_parsed;
        use std::collections::HashMap;
        use std::time::Duration;

        let mut alert_thresholds = HashMap::new();
        alert_thresholds.insert("anomaly_score".to_string(), 0.9);

        Self {
            enabled: true,
            algorithms: vec![
                "isolation_forest".to_string(),
                "statistical_outlier".to_string(),
            ],
            sensitivity: get_parsed(source, "BEARDOG_ANOMALY_DETECTION_SENSITIVITY", 0.8),
            training_window: Duration::from_secs(get_parsed(
                source,
                "BEARDOG_ANOMALY_TRAINING_WINDOW_SECS",
                time::SECONDS_PER_DAY,
            )),
            alert_thresholds,
        }
    }
}

impl Default for AnomalyDetectionConfig {
    fn default() -> Self {
        use crate::canonical::config::source::EnvConfigSource;
        Self::from_source(&EnvConfigSource::new())
    }
}

impl Default for TrendAnalysisConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            analysis_period: Duration::from_secs(
                std::env::var(env_keys::ENV_TREND_ANALYSIS_PERIOD_SECS)
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(time::SECONDS_PER_DAY), // 1 day default
            ),
            indicators: vec!["cpu_trend".to_string(), "memory_trend".to_string()],
            forecasting_models: vec!["arima".to_string(), "linear_trend".to_string()],
            prediction_horizon: Duration::from_secs(
                std::env::var(env_keys::ENV_PREDICTION_HORIZON_SECS)
                    .ok()
                    .and_then(|h| h.parse().ok())
                    .unwrap_or(time::SECONDS_PER_HOUR), // 1 hour default
            ),
        }
    }
}

impl Default for SecurityMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            event_types: vec![
                "authentication_failure".to_string(),
                "unauthorized_access".to_string(),
                "suspicious_activity".to_string(),
            ],
            threat_rules: vec![
                "failed_login_attempts > 5".to_string(),
                "unusual_traffic_pattern".to_string(),
            ],
            alert_thresholds: {
                let mut thresholds = HashMap::new();
                thresholds.insert("threat_score".to_string(), 0.8);
                thresholds
            },
            response_actions: {
                let mut actions = HashMap::new();
                actions.insert("high_threat".to_string(), "block_ip".to_string());
                actions
            },
        }
    }
}

impl ExportIntegrationConfig {
    /// Create configuration from a config source (modern pattern)
    pub fn from_source(source: &dyn crate::canonical::config::source::ConfigSource) -> Self {
        use crate::canonical::config::source::get_parsed;
        use std::time::Duration;

        Self {
            destinations: vec![],
            format: source.get_or("BEARDOG_EXPORT_FORMAT", "json"),
            interval: Duration::from_secs(get_parsed(source, "BEARDOG_EXPORT_INTERVAL_SECS", 300)),
            compression: CompressionConfig::default(),
        }
    }
}

impl Default for ExportIntegrationConfig {
    fn default() -> Self {
        use crate::canonical::config::source::EnvConfigSource;
        Self::from_source(&EnvConfigSource::new())
    }
}

impl CompressionConfig {
    /// Create configuration from a config source (modern pattern)
    pub fn from_source(source: &dyn crate::canonical::config::source::ConfigSource) -> Self {
        use crate::canonical::config::source::{get_bool, get_parsed};

        Self {
            enabled: get_bool(source, "BEARDOG_COMPRESSION_ENABLED", true),
            algorithm: source.get_or("BEARDOG_COMPRESSION_ALGORITHM", "gzip"),
            level: get_parsed(source, "BEARDOG_COMPRESSION_LEVEL", 6),
        }
    }
}

impl Default for CompressionConfig {
    fn default() -> Self {
        use crate::canonical::config::source::EnvConfigSource;
        Self::from_source(&EnvConfigSource::new())
    }
}

impl MonitoringConfig {
    /// Create configuration from a config source (modern pattern)
    pub fn from_source(source: &dyn crate::canonical::config::source::ConfigSource) -> Self {
        use crate::canonical::config::source::{get_bool, get_parsed};
        use std::time::Duration;

        Self {
            enabled: get_bool(source, "BEARDOG_MONITORING_ENABLED", true),
            metrics_interval: Duration::from_secs(get_parsed(
                source,
                "BEARDOG_MONITORING_METRICS_INTERVAL_SECS",
                60,
            )),
            alerts: vec![],
            dashboard: DashboardConfig::default(),
        }
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        use crate::canonical::config::source::EnvConfigSource;
        Self::from_source(&EnvConfigSource::new())
    }
}

impl Default for AlertConfig {
    fn default() -> Self {
        Self {
            name: "default_alert".to_string(),
            condition: "cpu > 80%".to_string(),
            threshold: std::env::var(env_keys::ENV_ALERT_THRESHOLD)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(80.0),
            severity: "warning".to_string(),
            channels: vec!["email".to_string()],
        }
    }
}

impl DashboardConfig {
    /// Create configuration from a config source (modern pattern)
    pub fn from_source(source: &dyn crate::canonical::config::source::ConfigSource) -> Self {
        use crate::canonical::config::source::{get_bool, get_parsed};
        use std::time::Duration;

        Self {
            enabled: get_bool(source, "BEARDOG_DASHBOARD_ENABLED", true),
            refresh_interval: Duration::from_secs(get_parsed(
                source,
                "BEARDOG_DASHBOARD_REFRESH_INTERVAL_SECS",
                30,
            )),
            metrics: vec!["cpu".to_string(), "memory".to_string()],
            layout: source.get_or("BEARDOG_DASHBOARD_LAYOUT", "grid"),
        }
    }
}

impl Default for DashboardConfig {
    fn default() -> Self {
        use crate::canonical::config::source::EnvConfigSource;
        Self::from_source(&EnvConfigSource::new())
    }
}

// BearDogConfig implementation for ConsolidatedMonitoringConfig
impl BearDogConfig for ConsolidatedMonitoringConfig {
    fn validate(&self) -> Result<(), BearDogError> {
        if self.enabled {
            if self.metrics.interval.as_secs() == 0 {
                return Err(BearDogError::validation(
                    "Metrics collection interval cannot be zero",
                ));
            }

            if self.metrics.buffer_size == 0 {
                return Err(BearDogError::validation(
                    "Metrics buffer size cannot be zero",
                ));
            }

            if self.health.enabled && self.health.check_interval.as_secs() == 0 {
                return Err(BearDogError::validation(
                    "Health check interval cannot be zero",
                ));
            }

            if self.anomaly.enabled
                && (self.anomaly.sensitivity < 0.0 || self.anomaly.sensitivity > 1.0)
            {
                return Err(BearDogError::validation(
                    "Anomaly detection sensitivity must be between 0.0 and 1.0",
                ));
            }
        }
        Ok(())
    }

    fn merge(&self, other: &Self) -> Result<Self, BearDogError> {
        Ok(Self {
            enabled: other.enabled,
            metrics: if other.enabled {
                other.metrics.clone()
            } else {
                self.metrics.clone()
            },
            analysis: if other.enabled {
                other.analysis.clone()
            } else {
                self.analysis.clone()
            },
            health: if other.health.enabled {
                other.health.clone()
            } else {
                self.health.clone()
            },
            anomaly: if other.anomaly.enabled {
                other.anomaly.clone()
            } else {
                self.anomaly.clone()
            },
            trends: if other.trends.enabled {
                other.trends.clone()
            } else {
                self.trends.clone()
            },
            security: if other.security.enabled {
                other.security.clone()
            } else {
                self.security.clone()
            },
            export: other.export.clone(),
        })
    }

    fn from_env() -> Result<Self, BearDogError> {
        let mut config = Self::default();

        if let Ok(enabled) = std::env::var(env_keys::ENV_MONITORING_ENABLED) {
            config.enabled = enabled.parse().unwrap_or(true);
        }

        if let Ok(interval) = std::env::var(env_keys::ENV_MONITORING_INTERVAL)
            && let Ok(secs) = interval.parse::<u64>()
        {
            config.metrics.interval = Duration::from_secs(secs);
        }

        if let Ok(buffer_size) = std::env::var(env_keys::ENV_MONITORING_BUFFER_SIZE) {
            config.metrics.buffer_size = buffer_size.parse().unwrap_or(1000);
        }

        config.validate()?;
        Ok(config)
    }

    fn to_toml(&self) -> Result<String, BearDogError> {
        toml::to_string(self).map_err(|e| {
            BearDogError::system(format!(
                "Failed to serialize monitoring config to TOML: {e}"
            ))
        })
    }

    fn domain() -> &'static str {
        "monitoring"
    }
}

// Backward compatibility: redirect to canonical monitoring
