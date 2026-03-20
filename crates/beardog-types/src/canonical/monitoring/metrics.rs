// SPDX-License-Identifier: AGPL-3.0-only

// Unified Metrics Configuration
//
// This module consolidates all metrics configuration patterns from across the codebase
// into a single, canonical system. It replaces fragmented metrics configs from:
// - beardog-monitoring/src/advanced_metrics.rs (MetricsConfig, AnalysisConfig, etc.)
// - beardog-monitoring/src/improved_monitoring.rs (MetricCollectionConfig)
// - beardog-types/src/canonical/monitoring.rs (MetricsConfig, MetricAggregationConfig)

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::{BatchConfig, MonitoringConfigValidation, RetentionPolicy, StorageBackend};

/// **UNIFIED METRICS CONFIGURATION** - Consolidates all metrics config patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedMetricsConfig {
    /// **CORE SETTINGS**
    /// Whether metrics collection is globally enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Interval between metric collection runs
    /// The collection interval value
    pub collection_interval: Duration,
    /// The export interval value
    pub export_interval: Duration,
    /// The retention policy value
    pub retention_policy: RetentionPolicy,

    /// **METRIC TYPES**
    /// Number of itemsers
    pub counters: CounterConfig,
    /// The gauges value
    pub gauges: GaugeConfig,
    /// The histograms value
    pub histograms: HistogramConfig,
    /// The summaries value
    pub summaries: SummaryConfig,

    /// **COLLECTION SETTINGS**
    /// Metric collection configuration
    /// The collection value
    pub collection: MetricCollectionConfig,
    /// Metric aggregation configuration
    /// The aggregation value
    pub aggregation: MetricAggregationConfig,
    /// Metric filtering configuration
    /// The filtering value
    pub filtering: MetricFilteringConfig,

    /// **ANALYSIS AND PROCESSING**
    /// Metric analysis configuration
    /// The analysis value
    pub analysis: MetricAnalysisConfig,
    /// Anomaly detection configuration
    /// The anomaly detection value
    pub anomaly_detection: AnomalyDetectionConfig,
    /// Trend analysis configuration
    /// The trend analysis value
    pub trend_analysis: TrendAnalysisConfig,

    /// **STORAGE AND EXPORT**
    /// Metric storage configuration
    /// The storage value
    pub storage: MetricStorageConfig,
    /// Metric export configuration
    /// The export value
    pub export: MetricExportConfig,

    /// **CUSTOM METRICS**
    /// Custom metric configurations by name
    /// Mapping of custom metrics
    pub custom_metrics: HashMap<String, CustomMetricConfig>,
}

impl Default for UnifiedMetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(15),
            export_interval: Duration::from_secs(60),
            retention_policy: RetentionPolicy::default(),
            counters: CounterConfig::default(),
            gauges: GaugeConfig::default(),
            histograms: HistogramConfig::default(),
            summaries: SummaryConfig::default(),
            collection: MetricCollectionConfig::default(),
            aggregation: MetricAggregationConfig::default(),
            filtering: MetricFilteringConfig::default(),
            analysis: MetricAnalysisConfig::default(),
            anomaly_detection: AnomalyDetectionConfig::default(),
            trend_analysis: TrendAnalysisConfig::default(),
            storage: MetricStorageConfig::default(),
            export: MetricExportConfig::default(),
            custom_metrics: HashMap::new(),
        }
    }
}

/// Counter metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterConfig {
    /// Whether counter metrics are enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// The prefix value
    pub prefix: String,
    /// Default labels to apply to all counter metrics
    /// Mapping of labels
    pub labels: HashMap<String, String>,
    /// Whether `rate_calculation` is enabled
    pub rate_calculation: bool,
}

impl Default for CounterConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            prefix: "beardog_counter".to_string(),
            labels: HashMap::new(),
            rate_calculation: true,
        }
    }
}

/// Gauge metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaugeConfig {
    /// Whether gauge metrics are enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// The prefix value
    pub prefix: String,
    /// Default labels to apply to all gauge metrics
    /// Mapping of labels
    pub labels: HashMap<String, String>,
    /// Whether to apply smoothing to gauge values
    /// Whether smoothing is enabled
    pub smoothing: bool,
    /// The smoothing factor value
    pub smoothing_factor: f64,
}

impl Default for GaugeConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            prefix: "beardog_gauge".to_string(),
            labels: HashMap::new(),
            smoothing: false,
            smoothing_factor: std::env::var("BEARDOG_METRICS_SMOOTHING_FACTOR")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.1),
        }
    }
}

/// Histogram metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistogramConfig {
    /// Whether histogram metrics are enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// The prefix value
    pub prefix: String,
    /// Default labels to apply to all histogram metrics
    /// Mapping of labels
    pub labels: HashMap<String, String>,
    /// Collection of buckets
    pub buckets: Vec<f64>,
    /// Maximum number of histogram buckets
    /// Number of `max_buckets`
    pub max_buckets: usize,
}

impl Default for HistogramConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            prefix: "beardog_histogram".to_string(),
            labels: HashMap::new(),
            buckets: vec![
                0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
            ],
            max_buckets: std::env::var("BEARDOG_HISTOGRAM_MAX_BUCKETS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(50),
        }
    }
}

/// Summary metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryConfig {
    /// Whether summary metrics are enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// The prefix value
    pub prefix: String,
    /// Default labels to apply to all summary metrics
    /// Mapping of labels
    pub labels: HashMap<String, String>,
    /// Collection of quantiles
    pub quantiles: Vec<f64>,
    /// The max age value
    pub max_age: Duration,
}

impl Default for SummaryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            prefix: "beardog_summary".to_string(),
            labels: HashMap::new(),
            quantiles: vec![0.5, 0.9, 0.95, 0.99],
            max_age: Duration::from_secs(600),
        }
    }
}

/// Metric collection configuration (consolidates from `improved_monitoring.rs`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricCollectionConfig {
    /// Whether metric collection is enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Flush sizing and retry policy for scrapes. **Default:** [`BatchConfig::default()`].
    pub batch_config: BatchConfig,
    /// Size of the metric collection buffer
    /// Number of `buffer_size`
    pub buffer_size: usize,
    /// Worker threads pulling from the scrape pipeline. **Default:** `4` (`BEARDOG_METRICS_COLLECTION_THREADS`).
    pub collection_threads: usize,
    /// Wall-clock budget for a full collection pass. **Default:** `30s`.
    pub collection_timeout: Duration,
    /// Whether to retry failed metric collections
    /// Whether `retry_failed_collections` is enabled
    pub retry_failed_collections: bool,
    /// Number of `max_collection_errors`
    /// Failures tolerated before the collector enters backoff. **Default:** `10`.
    pub max_collection_errors: u32,
}

impl Default for MetricCollectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            batch_config: BatchConfig::default(),
            buffer_size: std::env::var("BEARDOG_METRICS_BUFFER_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10000),
            collection_threads: std::env::var("BEARDOG_METRICS_COLLECTION_THREADS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(4),
            collection_timeout: Duration::from_secs(30),
            retry_failed_collections: true,
            max_collection_errors: std::env::var("BEARDOG_METRICS_MAX_COLLECTION_ERRORS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
        }
    }
}

/// Metric aggregation configuration (from canonical/monitoring.rs)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricAggregationConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Aggregation Interval
    /// The aggregation interval value
    pub aggregation_interval: Duration,
    /// Aggregation Functions
    /// Collection of aggregation functions
    pub aggregation_functions: Vec<AggregationFunction>,
    /// Grouping Keys
    /// Collection of grouping keys
    pub grouping_keys: Vec<String>,
    /// Sliding windows used when computing rollups (1m, 5m, …). **Default:** see [`MetricAggregationConfig::default`].
    pub time_windows: Vec<Duration>,
}

impl Default for MetricAggregationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            aggregation_interval: Duration::from_secs(60),
            aggregation_functions: vec![
                AggregationFunction::Sum,
                AggregationFunction::Average,
                AggregationFunction::Min,
                AggregationFunction::Max,
                AggregationFunction::Count,
            ],
            grouping_keys: vec!["service ".to_string(), "environment ".to_string()],
            time_windows: vec![
                Duration::from_secs(60),   // 1 minute
                Duration::from_secs(300),  // 5 minutes
                Duration::from_secs(900),  // 15 minutes
                Duration::from_secs(3600), // 1 hour
            ],
        }
    }
}

/// Aggregation functions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AggregationFunction {
    /// Sum variant
    Sum,
    /// Average variant
    Average,
    /// Min variant
    Min,
    /// Max variant
    Max,
    /// Count variant
    Count,
    /// Median variant
    Median,
    /// Percentile aggregation with specified percentile value (0.0-1.0)
    Percentile(f64),
    /// `StandardDeviation` variant
    StandardDeviation,
    /// Rate variant
    Rate,
    /// Increase variant
    Increase,
}

/// Metric filtering configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricFilteringConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Include Patterns
    /// Collection of include patterns
    pub include_patterns: Vec<String>,
    /// Exclude Patterns
    /// Collection of exclude patterns
    pub exclude_patterns: Vec<String>,
    /// Label Filters
    /// Mapping of label filters
    pub label_filters: HashMap<String, String>,
    /// Value Filters
    /// Collection of value filters
    pub value_filters: Vec<ValueFilter>,
}

/// Value filter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueFilter {
    /// Metric Name
    /// Name of the metric
    pub metric_name: String,
    /// Operator
    /// The operator value
    pub operator: ValueFilterOperator,
    /// Threshold
    /// The threshold value
    pub threshold: f64,
    /// Action
    /// The action value
    pub action: FilterAction,
}

/// Value filter operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValueFilterOperator {
    /// `GreaterThan` variant
    GreaterThan,
    /// `LessThan` variant
    LessThan,
    /// Equal variant
    Equal,
    /// `NotEqual` variant
    NotEqual,
    /// Filter values between min and max range (inclusive)
    Between {
        /// Lower inclusive bound.
        min: f64,
        /// Upper inclusive bound.
        max: f64,
    },
}

/// Filter actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterAction {
    /// Drop variant
    Drop,
    /// Cap values at maximum threshold
    Cap {
        /// Maximum allowed value
        max_value: f64,
    },
    /// Apply a registered transform to values matching the filter.
    Transform {
        /// Registered transform function name.
        function: String,
    },
}

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
            confidence_interval: std::env::var("BEARDOG_METRICS_CONFIDENCE_INTERVAL")
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
            sensitivity: std::env::var("BEARDOG_ANOMALY_DETECTION_SENSITIVITY")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.95),
            min_data_points: std::env::var("BEARDOG_ANOMALY_MIN_DATA_POINTS")
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
            trend_threshold: std::env::var("BEARDOG_METRICS_TREND_THRESHOLD")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.1),
            seasonal_adjustment: false,
        }
    }
}

/// Metric storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricStorageConfig {
    /// Backend
    /// The backend value
    pub backend: StorageBackend,
    /// Compression
    /// Whether compression is enabled
    pub compression: bool,
    /// Encryption
    /// Whether encryption is enabled
    pub encryption: bool,
    /// Partitioning
    /// The partitioning value
    pub partitioning: PartitioningConfig,
}

impl Default for MetricStorageConfig {
    fn default() -> Self {
        Self {
            backend: StorageBackend::default(),
            compression: true,
            encryption: false,
            partitioning: PartitioningConfig::default(),
        }
    }
}

/// Partitioning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitioningConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Partition By
    /// The partition by value
    pub partition_by: PartitionBy,
    /// Partition Size
    /// The partition size value
    pub partition_size: Duration,
}

impl Default for PartitioningConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            partition_by: PartitionBy::Time,
            partition_size: Duration::from_secs(86400), // 1 day
        }
    }
}

/// Partitioning strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PartitionBy {
    /// Time variant
    Time,
    /// `MetricName` variant
    MetricName,
    /// Labels variant
    Labels,
    /// Hash variant
    Hash,
}

/// Metric export configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricExportConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Exporters
    /// Collection of exporters
    pub exporters: Vec<MetricExporter>,
    /// Export Batch Size
    /// Number of `export_batch_size`
    pub export_batch_size: usize,
    /// Export Timeout
    pub export_timeout: Duration,
}

impl Default for MetricExportConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            exporters: vec![MetricExporter::Prometheus],
            export_batch_size: std::env::var("BEARDOG_METRIC_EXPORT_BATCH_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1000),
            export_timeout: Duration::from_secs(30),
        }
    }
}

/// Metric exporters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricExporter {
    /// Prometheus variant
    Prometheus,
    /// Grafana variant
    Grafana,
    /// `InfluxDB` variant
    InfluxDB,
    /// `CloudWatch` variant
    CloudWatch,
    /// Custom metrics backend with user-defined configuration
    Custom {
        /// Name of the custom backend
        name: String,
        /// Backend-specific configuration parameters
        config: HashMap<String, serde_json::Value>,
    },
}

/// Custom metric configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMetricConfig {
    /// Name
    /// Name of the item
    pub name: String,
    /// Metric Type
    /// The metric type value
    pub metric_type: CustomMetricType,
    /// Labels
    /// Mapping of labels
    pub labels: HashMap<String, String>,
    /// Collection Function
    /// The collection function value
    pub collection_function: String,
    /// Collection Interval
    /// The collection interval value
    pub collection_interval: Duration,
}

/// Custom metric types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of custom metric
pub enum CustomMetricType {
    /// Counter variant
    Counter,
    /// Gauge variant
    Gauge,
    /// Histogram variant
    Histogram,
    /// Summary variant
    Summary,
}

impl MonitoringConfigValidation for UnifiedMetricsConfig {
    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        if self.collection_interval.is_zero() {
            return Err(BearDogError::business(
                "Collection interval must be greater than zero".to_string(),
            ));
        }

        if self.export_interval < self.collection_interval {
            return Err(BearDogError::business(
                "Export interval must be >= collection interval".to_string(),
            ));
        }

        // Validate histogram buckets
        if self.histograms.enabled && self.histograms.buckets.is_empty() {
            return Err(BearDogError::business(
                "Histogram buckets cannot be empty when histograms are enabled".to_string(),
            ));
        }

        // Validate summary quantiles
        if self.summaries.enabled {
            for quantile in &self.summaries.quantiles {
                if *quantile < 0.0 || *quantile > 1.0 {
                    return Err(BearDogError::business(
                        "Summary quantiles must be between 0.0 and 1.0".to_string(),
                    ));
                }
            }
        }

        Ok(())
    }

    /// Checks if compatible with
    fn is_compatible_with(&self, _other_version: u32) -> bool {
        true
    }
}
