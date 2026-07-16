// SPDX-License-Identifier: AGPL-3.0-or-later

use beardog_config::env_keys;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Metric analysis configuration (from `advanced_metrics.rs`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricAnalysisConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Analysis Interval
    /// The analysis interval value
    pub analysis_interval: Duration,
    /// Statistical Analysis
    /// Whether `statistical_analysis` is enabled
    pub statistical_analysis: bool,
    /// Correlation Analysis
    /// Whether `correlation_analysis` is enabled
    pub correlation_analysis: bool,
    /// Pattern Detection
    /// Whether `pattern_detection` is enabled
    pub pattern_detection: bool,
    /// Forecasting
    /// Optional horizon models for capacity planning. **Default:** [`ForecastingConfig::default()`].
    pub forecasting: ForecastingConfig,
}

impl Default for MetricAnalysisConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            analysis_interval: Duration::from_secs(300),
            statistical_analysis: true,
            correlation_analysis: false,
            pattern_detection: true,
            forecasting: ForecastingConfig::default(),
        }
    }
}

/// Forecasting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastingConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Algorithm
    /// The algorithm value
    pub algorithm: ForecastingAlgorithm,
    /// Horizon
    /// The horizon value
    pub horizon: Duration,
    /// Confidence Interval
    pub confidence_interval: f64,
}

impl Default for ForecastingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            algorithm: ForecastingAlgorithm::LinearRegression,
            horizon: Duration::from_secs(3600),
            confidence_interval: std::env::var(env_keys::ENV_METRICS_CONFIDENCE_INTERVAL)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.95),
        }
    }
}

/// Forecasting algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForecastingAlgorithm {
    /// `LinearRegression` variant
    LinearRegression,
    /// `ExponentialSmoothing` variant
    ExponentialSmoothing,
    /// ARIMA variant
    ARIMA,
    /// `SeasonalDecomposition` variant
    SeasonalDecomposition,
}

/// Anomaly detection configuration (from `advanced_metrics.rs`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyDetectionConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Algorithm
    /// The algorithm value
    pub algorithm: AnomalyDetectionAlgorithm,
    /// Sensitivity
    /// The sensitivity value
    pub sensitivity: f64,
    /// Min Data Points
    /// Number of `min_data_points`
    pub min_data_points: usize,
    /// Detection Interval
    /// The detection interval value
    pub detection_interval: Duration,
}

impl Default for AnomalyDetectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            algorithm: AnomalyDetectionAlgorithm::StatisticalOutlier,
            sensitivity: std::env::var(env_keys::ENV_ANOMALY_DETECTION_SENSITIVITY)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.95),
            min_data_points: std::env::var(env_keys::ENV_ANOMALY_MIN_DATA_POINTS)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
            detection_interval: Duration::from_secs(60),
        }
    }
}

/// Anomaly detection algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnomalyDetectionAlgorithm {
    /// `StatisticalOutlier` variant
    StatisticalOutlier,
    /// `IsolationForest` variant
    IsolationForest,
    /// `LocalOutlierFactor` variant
    LocalOutlierFactor,
    /// `OneClassSVM` variant
    OneClassSVM,
}

/// Trend analysis configuration (from `advanced_metrics.rs`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysisConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Time Window
    pub time_window: Duration,
    /// Trend Threshold
    /// The trend threshold value
    pub trend_threshold: f64,
    /// Seasonal Adjustment
    /// Whether `seasonal_adjustment` is enabled
    pub seasonal_adjustment: bool,
}

impl Default for TrendAnalysisConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            time_window: Duration::from_secs(3600),
            trend_threshold: std::env::var(env_keys::ENV_METRICS_TREND_THRESHOLD)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.1),
            seasonal_adjustment: false,
        }
    }
}
