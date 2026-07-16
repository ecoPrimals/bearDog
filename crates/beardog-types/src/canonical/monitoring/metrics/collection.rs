// SPDX-License-Identifier: AGPL-3.0-or-later

use beardog_config::env_keys;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::Duration;

use super::super::BatchConfig;

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
            buffer_size: std::env::var(env_keys::ENV_METRICS_BUFFER_SIZE)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10000),
            collection_threads: std::env::var(env_keys::ENV_METRICS_COLLECTION_THREADS)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(4),
            collection_timeout: Duration::from_secs(30),
            retry_failed_collections: true,
            max_collection_errors: std::env::var(env_keys::ENV_METRICS_MAX_COLLECTION_ERRORS)
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
    pub label_filters: BTreeMap<String, String>,
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
