// SPDX-License-Identifier: AGPL-3.0-or-later

// Core Monitoring Configuration Types
//
// This module provides the foundational configuration types that are shared
// across all monitoring domains.

use crate::canonical::traits::{MonitoringConfig, RetryStrategy};
use crate::constants::defaults;
use crate::constants::time;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::Duration;

use super::MonitoringConfigValidation;
use beardog_config::env_keys;

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
    pub tags: BTreeMap<String, String>,
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
            tags: BTreeMap::new(),
            sampling_rate: std::env::var(env_keys::ENV_MONITORING_SAMPLING_RATE)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1.0),
            buffer_size: std::env::var(env_keys::ENV_MONITORING_BUFFER_SIZE)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1000),
            flush_interval: Duration::from_secs(
                std::env::var(env_keys::ENV_MONITORING_FLUSH_INTERVAL_SECS)
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(30),
            ),
        }
    }
}

// Implement MonitoringConfig trait for core monitoring configuration
impl MonitoringConfig for CoreMonitoringConfig {
    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn metrics_endpoint(&self) -> &str {
        // Core config doesn't store endpoint, use service name as identifier
        &self.service_name
    }

    fn reporting_interval(&self) -> Duration {
        self.flush_interval
    }

    fn detailed_metrics(&self) -> bool {
        // Core monitoring includes all metrics
        true
    }

    fn monitoring_level(&self) -> crate::canonical::traits::monitoring::MonitoringLevel {
        use crate::canonical::traits::monitoring::MonitoringLevel;
        // Determine level from sampling rate
        if self.sampling_rate >= 1.0 {
            MonitoringLevel::Standard
        } else if self.sampling_rate >= 0.5 {
            MonitoringLevel::Basic
        } else {
            MonitoringLevel::Minimal
        }
    }

    fn health_checks_enabled(&self) -> bool {
        self.enabled
    }

    fn sample_rate(&self) -> f64 {
        self.sampling_rate
    }

    fn validate(&self) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }
        if self.service_name.is_empty() {
            return Err("Service name cannot be empty".to_string());
        }
        if self.sampling_rate < 0.0 || self.sampling_rate > 1.0 {
            return Err("Sampling rate must be between 0.0 and 1.0".to_string());
        }
        if self.buffer_size == 0 {
            return Err("Buffer size must be > 0".to_string());
        }
        if self.flush_interval.is_zero() {
            return Err("Flush interval cannot be zero".to_string());
        }
        Ok(())
    }

    fn is_production_ready(&self) -> bool {
        use crate::constants::domains::validation::{
            MAX_FLUSH_INTERVAL_SECS, MIN_CACHE_SIZE, MIN_FLUSH_INTERVAL_SECS,
        };

        self.enabled
            && !self.service_name.is_empty()
            && self.sampling_rate >= 0.1
            && self.sampling_rate <= 1.0
            && self.flush_interval >= Duration::from_secs(MIN_FLUSH_INTERVAL_SECS)
            && self.flush_interval <= Duration::from_secs(MAX_FLUSH_INTERVAL_SECS)
            && self.buffer_size >= MIN_CACHE_SIZE
            && MonitoringConfig::validate(self).is_ok()
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

/// Persistence target for metric buffers, traces, or log archives.
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
        /// Max on-disk size of the active file before rotation.
        rotation_size: u64,
        /// Maximum number of rotated files to keep
        max_files: u32,
    },
    /// Database storage backend
    Database {
        /// Database connection string
        connection_string: String,
        /// Destination table or collection for metric rows.
        table_name: String,
    },
    /// Remote storage backend
    Remote {
        /// Remote endpoint URL
        endpoint: String,
        /// Optional bearer/API key for `SaaS` backends.
        api_key: Option<String>,
        /// Request timeout duration
        timeout: Duration,
    },
    /// Custom storage backend with configurable parameters
    Custom {
        /// Backend type identifier
        backend_type: String,
        /// Custom configuration parameters
        config: BTreeMap<String, serde_json::Value>,
    },
}

impl Default for StorageBackend {
    fn default() -> Self {
        Self::Memory {
            max_entries: defaults::DEFAULT_MAX_ENTRIES,
        }
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
    /// Apply a named transform pipeline to matching series or log lines.
    Transform {
        /// Built-in transform name (e.g. `drop_labels`, `sample`).
        operation: String,
        /// Transform-specific JSON parameters.
        parameters: BTreeMap<String, serde_json::Value>,
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

/// Operators used when evaluating [`FilterCondition`] predicates.
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
                std::env::var(env_keys::ENV_RETENTION_MAX_AGE_SECS)
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(time::SECONDS_PER_DAY * 30), // 30 days
            ),
            max_size: std::env::var(env_keys::ENV_RETENTION_MAX_SIZE_BYTES)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1024 * 1024 * 1024), // 1GB
            max_count: std::env::var(env_keys::ENV_RETENTION_MAX_COUNT)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1_000_000),
            cleanup_interval: Duration::from_secs(
                std::env::var(env_keys::ENV_RETENTION_CLEANUP_INTERVAL_SECS)
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(time::SECONDS_PER_HOUR), // 1 hour
            ),
            archive_policy: None,
        }
    }
}

/// Long-term archival settings applied after [`RetentionPolicy`] trimming.
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
            batch_size: std::env::var(env_keys::ENV_BATCH_SIZE)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
            flush_interval: Duration::from_secs(
                std::env::var(env_keys::ENV_BATCH_FLUSH_INTERVAL_SECS)
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
            max_wait_time: Duration::from_secs(
                std::env::var(env_keys::ENV_BATCH_MAX_WAIT_TIME_SECS)
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60),
            ),
            retry_policy: RetryPolicy::default(),
        }
    }
}

/// Exponential backoff for failed metric export batches.
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
            max_retries: std::env::var(env_keys::ENV_RETRY_POLICY_MAX_RETRIES)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3),
            initial_delay: Duration::from_millis(
                std::env::var(env_keys::ENV_RETRY_INITIAL_DELAY_MS)
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(100),
            ),
            max_delay: Duration::from_secs(
                std::env::var(env_keys::ENV_RETRY_MAX_DELAY_SECS)
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
            backoff_multiplier: std::env::var(env_keys::ENV_MONITORING_BACKOFF_MULTIPLIER)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(2.0),
        }
    }
}

// Implement RetryStrategy trait for monitoring retry policy
impl RetryStrategy for RetryPolicy {
    fn max_attempts(&self) -> u32 {
        self.max_retries
    }

    fn delay_for_attempt(&self, attempt: u32) -> Duration {
        // Exponential backoff with multiplier
        // Note: Precision loss is acceptable for delay calculations (not cryptographic)
        #[expect(
            clippy::cast_possible_truncation,
            reason = "monitoring retry delay clamped to millis as u64 for scheduling"
        )]
        #[expect(
            clippy::cast_precision_loss,
            reason = "acceptable imprecision for non-cryptographic backoff math"
        )]
        #[expect(
            clippy::cast_sign_loss,
            reason = "delay clamped non-negative before f64 to u64"
        )]
        #[expect(
            clippy::cast_possible_wrap,
            reason = "retry attempt capped at 30 for powi exponent"
        )]
        let delay_ms = {
            let initial_ms = self.initial_delay.as_millis().min(u128::from(u64::MAX)) as f64;
            let max_ms = self.max_delay.as_millis().min(u128::from(u64::MAX)) as u64;
            let computed = initial_ms * self.backoff_multiplier.powi(attempt.min(30) as i32);
            (computed.max(0.0) as u64).min(max_ms)
        };
        Duration::from_millis(delay_ms)
    }

    fn backoff_multiplier(&self) -> f64 {
        self.backoff_multiplier
    }

    fn should_retry_error(&self, _error: &(dyn std::error::Error + Send + Sync)) -> bool {
        // Monitoring: only retry if enabled
        self.enabled
    }

    fn is_limit_reached(&self, attempts: u32) -> bool {
        !self.enabled || attempts >= self.max_retries
    }

    fn total_delay(&self, attempts: u32) -> Duration {
        if !self.enabled {
            return Duration::from_secs(0);
        }
        let mut total = Duration::from_secs(0);
        for attempt in 0..attempts {
            total += self.delay_for_attempt(attempt);
        }
        total
    }
}

#[cfg(test)]
mod monitoring_core_coverage_tests {
    use super::*;
    use crate::canonical::monitoring::MonitoringConfigValidation;
    use crate::canonical::traits::monitoring::MonitoringLevel;
    use crate::canonical::traits::{MonitoringConfig, RetryStrategy};

    #[test]
    fn core_monitoring_config_monitoring_level_from_sampling_rate() {
        let mut c = CoreMonitoringConfig {
            sampling_rate: 1.0,
            ..CoreMonitoringConfig::default()
        };
        assert!(matches!(
            MonitoringConfig::monitoring_level(&c),
            MonitoringLevel::Standard
        ));
        c.sampling_rate = 0.75;
        assert!(matches!(
            MonitoringConfig::monitoring_level(&c),
            MonitoringLevel::Basic
        ));
        c.sampling_rate = 0.1;
        assert!(matches!(
            MonitoringConfig::monitoring_level(&c),
            MonitoringLevel::Minimal
        ));
    }

    #[test]
    fn core_monitoring_config_validate_trait_errors() {
        let mut c = CoreMonitoringConfig::default();
        c.enabled = true;
        c.service_name = String::new();
        assert!(MonitoringConfig::validate(&c).is_err());

        c.service_name = "svc".to_string();
        c.sampling_rate = 1.5;
        assert!(MonitoringConfig::validate(&c).is_err());

        c.sampling_rate = 1.0;
        c.buffer_size = 0;
        assert!(MonitoringConfig::validate(&c).is_err());

        c.buffer_size = 100;
        c.flush_interval = Duration::ZERO;
        assert!(MonitoringConfig::validate(&c).is_err());
    }

    #[test]
    fn core_monitoring_config_validation_business_errors() {
        let mut c = CoreMonitoringConfig::default();
        c.service_name.clear();
        assert!(MonitoringConfigValidation::validate(&c).is_err());

        c.service_name = "x".to_string();
        c.sampling_rate = 2.0;
        assert!(MonitoringConfigValidation::validate(&c).is_err());
    }

    #[test]
    fn retry_policy_delay_and_limits() {
        let mut p = RetryPolicy::default();
        p.enabled = true;
        p.max_retries = 3;
        p.initial_delay = Duration::from_millis(100);
        p.max_delay = Duration::from_secs(1);
        p.backoff_multiplier = 2.0;

        assert!(RetryStrategy::delay_for_attempt(&p, 0) > Duration::ZERO);
        assert!(RetryStrategy::is_limit_reached(&p, 3));
        assert!(RetryStrategy::should_retry_error(
            &p,
            &std::io::Error::other("x")
        ));
        let total = RetryStrategy::total_delay(&p, 2);
        assert!(total > Duration::ZERO);

        p.enabled = false;
        assert_eq!(RetryStrategy::total_delay(&p, 5), Duration::ZERO);
        assert!(RetryStrategy::is_limit_reached(&p, 0));
        assert!(!RetryStrategy::should_retry_error(
            &p,
            &std::io::Error::other("x")
        ));
    }

    #[test]
    fn storage_backend_and_filter_action_serde_roundtrip() {
        let b = StorageBackend::Memory { max_entries: 42 };
        let v = serde_json::to_value(&b).expect("ser");
        let back: StorageBackend = serde_json::from_value(v).expect("de");
        assert!(matches!(back, StorageBackend::Memory { max_entries: 42 }));

        let fa = FilterAction::Transform {
            operation: "sample".to_string(),
            parameters: std::collections::BTreeMap::new(),
        };
        let v2 = serde_json::to_value(&fa).expect("ser");
        let back2: FilterAction = serde_json::from_value(v2).expect("de");
        assert!(matches!(back2, FilterAction::Transform { .. }));
    }

    #[test]
    fn retention_policy_default_serializes() {
        let r = RetentionPolicy::default();
        let _ = serde_json::to_string(&r).expect("json");
    }
}
