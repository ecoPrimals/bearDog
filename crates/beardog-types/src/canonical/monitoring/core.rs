// Core Monitoring Configuration Types
//
// This module provides the foundational configuration types that are shared
// across all monitoring domains.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::MonitoringConfigValidation;

/// Core monitoring configuration that applies to all monitoring domains
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMonitoringConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Service Name
    /// Name of the service
    pub service_name: String,
    /// Environment
    /// The environment value
    pub environment: String,
    /// Version
    /// The version value
    pub version: String,
    /// Instance Id
    pub instance_id: String,
    /// Tags
    /// Mapping of tags
    pub tags: HashMap<String, String>,
    /// Sampling Rate
    /// The sampling rate value
    pub sampling_rate: f64,
    /// Buffer Size
    /// Number of `buffer_size`
    pub buffer_size: usize,
    /// Flush Interval
    /// The flush interval value
    pub flush_interval: Duration,
}

impl Default for CoreMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            service_name: "beardog ".to_string(),
            environment: "development".to_string(),
            version: "4.0.0".to_string(),
            instance_id: "beardog-instance".to_string(),
            tags: HashMap::new(),
            sampling_rate: std::env::var("BEARDOG_MONITORING_SAMPLING_RATE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1.0),
            buffer_size: std::env::var("BEARDOG_MONITORING_BUFFER_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1000),
            flush_interval: Duration::from_secs(
                std::env::var("BEARDOG_MONITORING_FLUSH_INTERVAL_SECS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(30),
            ),
        }
    }
}

impl MonitoringConfigValidation for CoreMonitoringConfig {
    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        if self.service_name.is_empty() {
            return Err(BearDogError::business(
                "Service name cannot be empty".to_string(),
            ));
        }

        if self.sampling_rate < 0.0 || self.sampling_rate > 1.0 {
            return Err(BearDogError::business(
                "Sampling rate must be between 0.0 and 1.0".to_string(),
            ));
        }

        if self.buffer_size == 0 {
            return Err(BearDogError::business(
                "Buffer size must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }

    /// Checks if compatible with
    fn is_compatible_with(&self, _other_version: u32) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageBackend {
    /// In-memory storage with configurable limits
    Memory {
        /// Maximum number of entries to store in memory
        max_entries: usize,
    },
    /// File-based storage with rotation
    File {
        /// Path to the storage file
        path: String,
        rotation_size: u64,
        /// Maximum number of rotated files to keep
        max_files: u32,
    },
    /// Database storage backend
    Database {
        /// Database connection string
        connection_string: String,
        table_name: String,
    },
    /// Remote storage backend
    Remote {
        /// Remote endpoint URL
        endpoint: String,
        api_key: Option<String>,
        /// Request timeout duration
        timeout: Duration,
    },
    /// Custom storage backend with configurable parameters
    Custom {
        /// Backend type identifier
        backend_type: String,
        /// Custom configuration parameters
        config: HashMap<String, serde_json::Value>,
    },
}

impl Default for StorageBackend {
    fn default() -> Self {
        Self::Memory { max_entries: 10000 }
    }
}

/// Metric filter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricFilter {
    /// Name
    /// Name of the item
    pub name: String,
    /// Pattern
    /// The pattern value
    pub pattern: String,
    /// Action
    /// The action value
    pub action: FilterAction,
    /// Conditions
    /// Collection of conditions
    pub conditions: Vec<FilterCondition>,
}

/// Filter actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterAction {
    /// Include variant
    Include,
    /// Exclude variant
    Exclude,
    Transform {
        operation: String,
        parameters: HashMap<String, serde_json::Value>,
    },
}

/// Filter conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCondition {
    /// Field
    /// The field value
    pub field: String,
    /// Operator
    /// The operator value
    pub operator: ComparisonOperator,
    /// Value
    /// The value value
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperator {
    /// Equals variant
    Equals,
    /// `NotEquals` variant
    NotEquals,
    /// `GreaterThan` variant
    GreaterThan,
    /// `LessThan` variant
    LessThan,
    /// `GreaterThanOrEqual` variant
    GreaterThanOrEqual,
    /// `LessThanOrEqual` variant
    LessThanOrEqual,
    /// Contains variant
    Contains,
    /// `NotContains` variant
    NotContains,
    /// Matches variant
    Matches,
    /// `NotMatches` variant
    NotMatches,
}

/// Retention policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Max Age
    /// The max age value
    pub max_age: Duration,
    /// Max Size
    /// Number of `max_size`
    pub max_size: u64,
    /// Max Count
    /// Number of max
    pub max_count: u64,
    /// Cleanup Interval
    /// The cleanup interval value
    pub cleanup_interval: Duration,
    /// Archive Policy
    /// Optional archive policy
    pub archive_policy: Option<ArchivePolicy>,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            enabled: true,
            max_age: Duration::from_secs(
                std::env::var("BEARDOG_RETENTION_MAX_AGE_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(86400 * 30), // 30 days
            ),
            max_size: std::env::var("BEARDOG_RETENTION_MAX_SIZE_BYTES")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1024 * 1024 * 1024), // 1GB
            max_count: std::env::var("BEARDOG_RETENTION_MAX_COUNT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1_000_000),
            cleanup_interval: Duration::from_secs(
                std::env::var("BEARDOG_RETENTION_CLEANUP_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(3600), // 1 hour
            ),
            archive_policy: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchivePolicy {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Archive After
    /// The archive after value
    pub archive_after: Duration,
    /// Compression
    /// The compression value
    pub compression: CompressionType,
    /// Storage Backend
    /// The storage backend value
    pub storage_backend: StorageBackend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of compression
pub enum CompressionType {
    /// None variant
    None,
    /// Gzip compression
    Gzip,
    /// LZ4 compression
    Lz4,
    /// Zstd compression
    Zstd,
}

/// Batch processing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Batch Size
    /// Number of `batch_size`
    pub batch_size: usize,
    /// Flush Interval
    /// The flush interval value
    pub flush_interval: Duration,
    /// Max Wait Time
    pub max_wait_time: Duration,
    /// Retry Policy
    /// The retry policy value
    pub retry_policy: RetryPolicy,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            batch_size: std::env::var("BEARDOG_BATCH_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
            flush_interval: Duration::from_secs(
                std::env::var("BEARDOG_BATCH_FLUSH_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
            max_wait_time: Duration::from_secs(
                std::env::var("BEARDOG_BATCH_MAX_WAIT_TIME_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60),
            ),
            retry_policy: RetryPolicy::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Max Retries
    /// Number of `max_retries`
    pub max_retries: u32,
    /// Initial Delay
    /// The initial delay value
    pub initial_delay: Duration,
    /// Max Delay
    /// The max delay value
    pub max_delay: Duration,
    /// Backoff Multiplier
    /// The backoff multiplier value
    pub backoff_multiplier: f64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            enabled: true,
            max_retries: std::env::var("BEARDOG_RETRY_POLICY_MAX_RETRIES")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3),
            initial_delay: Duration::from_millis(
                std::env::var("BEARDOG_RETRY_INITIAL_DELAY_MS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(100),
            ),
            max_delay: Duration::from_secs(
                std::env::var("BEARDOG_RETRY_MAX_DELAY_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
            backoff_multiplier: std::env::var("BEARDOG_MONITORING_BACKOFF_MULTIPLIER")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(2.0),
        }
    }
}
