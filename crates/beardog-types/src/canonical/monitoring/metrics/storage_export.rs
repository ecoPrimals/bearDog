// SPDX-License-Identifier: AGPL-3.0-or-later

use beardog_config::env_keys;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::Duration;

use super::super::StorageBackend;

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
            export_batch_size: std::env::var(env_keys::ENV_METRIC_EXPORT_BATCH_SIZE)
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
        config: BTreeMap<String, serde_json::Value>,
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
    pub labels: BTreeMap<String, String>,
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
