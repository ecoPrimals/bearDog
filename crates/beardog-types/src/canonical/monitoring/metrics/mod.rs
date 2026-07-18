// SPDX-License-Identifier: AGPL-3.0-or-later

// Unified Metrics Configuration
//
// This module consolidates all metrics configuration patterns from across the codebase
// into a single, canonical system. It replaces fragmented metrics configs from:
// - beardog-monitoring/src/advanced_metrics.rs (MetricsConfig, AnalysisConfig, etc.)
// - beardog-monitoring/src/improved_monitoring.rs (MetricCollectionConfig)
// - beardog-types/src/canonical/monitoring.rs (MetricsConfig, MetricAggregationConfig)

mod analysis;
mod collection;
mod storage_export;
mod types;

pub use analysis::{
    AnomalyDetectionAlgorithm, AnomalyDetectionConfig, ForecastingAlgorithm, ForecastingConfig,
    MetricAnalysisConfig, TrendAnalysisConfig,
};
pub use collection::{
    AggregationFunction, FilterAction, MetricAggregationConfig, MetricCollectionConfig,
    MetricFilteringConfig, ValueFilter, ValueFilterOperator,
};
pub use storage_export::{
    CustomMetricConfig, CustomMetricType, MetricExportConfig, MetricExporter, MetricStorageConfig,
    PartitionBy, PartitioningConfig,
};
pub use types::{CounterConfig, GaugeConfig, HistogramConfig, SummaryConfig};

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::Duration;

use super::{MonitoringConfigValidation, RetentionPolicy};

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
    pub custom_metrics: BTreeMap<String, CustomMetricConfig>,
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
            custom_metrics: BTreeMap::new(),
        }
    }
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
