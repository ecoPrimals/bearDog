// SPDX-License-Identifier: AGPL-3.0-or-later

use beardog_config::env_keys;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::Duration;

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
    pub labels: BTreeMap<String, String>,
    /// Whether `rate_calculation` is enabled
    pub rate_calculation: bool,
}

impl Default for CounterConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            prefix: "beardog_counter".to_string(),
            labels: BTreeMap::new(),
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
    pub labels: BTreeMap<String, String>,
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
            labels: BTreeMap::new(),
            smoothing: false,
            smoothing_factor: std::env::var(env_keys::ENV_METRICS_SMOOTHING_FACTOR)
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
    pub labels: BTreeMap<String, String>,
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
            labels: BTreeMap::new(),
            buckets: vec![
                0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
            ],
            max_buckets: std::env::var(env_keys::ENV_HISTOGRAM_MAX_BUCKETS)
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
    pub labels: BTreeMap<String, String>,
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
            labels: BTreeMap::new(),
            quantiles: vec![0.5, 0.9, 0.95, 0.99],
            max_age: Duration::from_secs(600),
        }
    }
}
