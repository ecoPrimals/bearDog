// SPDX-License-Identifier: AGPL-3.0-only

// Configuration for Advanced Metrics System

use super::types::{AnomalyAlgorithm, TrendAlgorithm};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Metrics collection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enable metrics collection
    /// Whether feature is enabled
    pub enabled: bool,
    /// Collection interval
    /// The collection interval value
    pub collection_interval: Duration,
    /// Maximum history size per metric
    /// Number of `max_history_size`
    pub max_history_size: usize,
    /// Enable real-time broadcasting
    /// Whether `enable_broadcasting` is enabled
    pub enable_broadcasting: bool,
    /// Analysis configuration
    /// The analysis value
    pub analysis: AnalysisConfig,
    /// Health check configuration
    /// The health checks value
    pub health_checks: MetricsHealthCheckConfig,
}

/// Analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    /// Enable anomaly detection
    /// Whether `enable_anomaly_detection` is enabled
    pub enable_anomaly_detection: bool,
    /// Enable trend analysis
    /// Whether `enable_trend_analysis` is enabled
    pub enable_trend_analysis: bool,
    /// Analysis window size
    /// The analysis window value
    pub analysis_window: Duration,
    /// Anomaly detection configuration
    /// The anomaly value
    pub anomaly: AnomalyConfig,
    /// Trend analysis configuration
    /// The trend value
    pub trend: TrendConfig,
}

/// Metrics system health check configuration
///
/// Domain-specific configuration for metrics health monitoring.
/// Renamed from `HealthCheckConfig` for clarity - tracks health metrics
/// with thresholds and degradation alerts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsHealthCheckConfig {
    /// Enable health monitoring
    /// Whether feature is enabled
    pub enabled: bool,
    /// Health check interval
    /// The check interval value
    pub check_interval: Duration,
    /// Health threshold (0.0 to 1.0)
    /// The health threshold value
    pub health_threshold: f64,
    /// Alert on degraded health
    /// Whether `alert_on_degraded` is enabled
    pub alert_on_degraded: bool,
}

/// Anomaly detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyConfig {
    /// Detection algorithm
    /// The algorithm value
    pub algorithm: AnomalyAlgorithm,
    /// Sensitivity threshold
    /// The sensitivity value
    pub sensitivity: f64,
    /// Minimum data points required
    /// Number of `min_data_points`
    pub min_data_points: usize,
}

/// Trend analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendConfig {
    /// Analysis algorithm
    /// The algorithm value
    pub algorithm: TrendAlgorithm,
    /// Prediction horizon
    /// The prediction horizon value
    pub prediction_horizon: Duration,
    /// Confidence interval
    pub confidence_interval: f64,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(
                std::env::var("BEARDOG_METRICS_COLLECTION_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60),
            ),
            max_history_size: std::env::var("BEARDOG_METRICS_HISTORY_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1000), // 1000 samples default
            enable_broadcasting: true,
            analysis: AnalysisConfig::default(),
            health_checks: MetricsHealthCheckConfig::default(),
        }
    }
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            enable_anomaly_detection: true,
            enable_trend_analysis: true,
            analysis_window: Duration::from_secs(
                std::env::var("BEARDOG_ANALYSIS_WINDOW_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(24 * 60 * 60), // 24 hours default
            ),
            anomaly: AnomalyConfig::default(),
            trend: TrendConfig::default(),
        }
    }
}

impl Default for MetricsHealthCheckConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_interval: Duration::from_secs(
                std::env::var("BEARDOG_HEALTH_CHECK_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(300),
            ),
            health_threshold: 0.8,
            alert_on_degraded: true,
        }
    }
}

impl Default for AnomalyConfig {
    fn default() -> Self {
        Self {
            algorithm: AnomalyAlgorithm::StatisticalOutlier,
            sensitivity: 0.95,
            min_data_points: 10,
        }
    }
}

impl Default for TrendConfig {
    fn default() -> Self {
        Self {
            algorithm: TrendAlgorithm::LinearRegression,
            prediction_horizon: Duration::from_secs(4 * 60 * 60), // 4 hours using stable API
            confidence_interval: 0.95,
        }
    }
}
