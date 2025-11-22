//! # Workflow Configuration Domain
//!
//! This module contains all workflow-related configuration types, extracted from
//! the large `consolidated_domains.rs` file for better maintainability.

use crate::canonical::traits::{RetryStrategy, TimeoutPolicy};
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use crate::canonical::config::r#trait::BearDogConfig;

/// **CONSOLIDATED WORKFLOW CONFIGURATION** - Unifies all workflow configs
///
/// Consolidates: `WorkflowConfig`, `EscalationConfig`, etc.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConsolidatedWorkflowConfig {
    /// Enable workflow processing
    pub enabled: bool,

    /// Workflow engine configuration
    pub engine: WorkflowEngineConfig,

    /// Escalation configuration
    pub escalation: WorkflowEscalationConfig,

    /// Scheduling configuration
    pub scheduling: SchedulingConfig,

    /// Persistence configuration
    pub persistence: PersistenceConfig,
}

/// Workflow engine configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkflowEngineConfig {
    /// Engine type (`Arc<str>` for fast cloning in hot path)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub engine_type: Arc<str>,

    /// Worker pool size
    pub worker_pool_size: usize,

    /// Queue configuration
    pub queue: QueueConfig,

    /// Timeout configuration
    pub timeouts: TimeoutConfig,
}

/// Workflow escalation configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkflowEscalationConfig {
    /// Enable escalation
    pub enabled: bool,

    /// Escalation rules
    pub rules: Vec<EscalationRule>,

    /// Default escalation timeout
    pub default_timeout: Duration,

    /// Notification configuration
    pub notifications: NotificationConfig,
}

/// Escalation rule
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EscalationRule {
    /// Rule name (`Arc<str>` for fast cloning across workflow evaluations)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub name: Arc<str>,

    /// Condition expression (`Arc<str>` for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub condition: Arc<str>,

    /// Escalation target identifier (`Arc<str>` for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub target: Arc<str>,

    /// Timeout
    pub timeout: Duration,
}

/// Notification configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NotificationConfig {
    /// Notification channels
    pub channels: Vec<String>,

    /// Templates
    pub templates: HashMap<String, String>,

    /// Rate limiting
    pub rate_limit: super::network::RateLimitConfig,
}

/// Rate limiting configuration (DEPRECATED - use canonical network config)
///
/// **MIGRATION**: Use `super::network::RateLimitConfig` instead.
///
/// This type alias will be removed in v3.3.0.
#[deprecated(since = "3.1.0", note = "Use super::network::RateLimitConfig instead")]
// Re-export from network domain (consolidated Nov 7, 2025)
pub use super::network::RateLimitConfig;

/// Scheduling configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SchedulingConfig {
    /// Scheduler type (`Arc<str>` for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub scheduler_type: Arc<str>,

    /// Default schedule (`Arc<str>` for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub default_schedule: Arc<str>,

    /// Time zone (`Arc<str>` for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub timezone: Arc<str>,

    /// Concurrent execution limit
    pub max_concurrent: usize,
}

/// Queue configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QueueConfig {
    /// Queue type (`Arc<str>` for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub queue_type: Arc<str>,

    /// Queue capacity
    pub capacity: usize,

    /// Message TTL
    pub message_ttl: Duration,

    /// Dead letter queue
    pub dead_letter_queue: Option<String>,
}

/// Timeout configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TimeoutConfig {
    /// Default timeout
    pub default: Duration,

    /// Maximum timeout
    pub maximum: Duration,

    /// Connection timeout
    pub connection: Duration,

    /// Read timeout
    pub read: Duration,
}

// Implement TimeoutPolicy trait for workflow timeout configuration
impl TimeoutPolicy for TimeoutConfig {
    fn connection_timeout(&self) -> Duration {
        self.connection
    }

    fn operation_timeout(&self, operation: &str) -> Duration {
        match operation {
            "read" => self.read,
            "connect" | "connection" => self.connection,
            _ => self.default,
        }
    }

    fn should_timeout(&self, elapsed: Duration, operation: &str) -> bool {
        elapsed >= self.operation_timeout(operation)
    }

    fn global_timeout(&self) -> Option<Duration> {
        Some(self.maximum)
    }

    fn read_timeout(&self) -> Duration {
        self.read
    }

    fn write_timeout(&self) -> Duration {
        self.default // Workflow config doesn't have separate write timeout
    }

    fn idle_timeout(&self) -> Option<Duration> {
        None // Workflow config doesn't have idle timeout
    }

    fn remaining_time(&self, elapsed: Duration, operation: &str) -> Duration {
        let timeout = self.operation_timeout(operation);
        timeout.saturating_sub(elapsed)
    }

    fn validate(&self) -> Result<(), String> {
        if self.connection.is_zero() {
            return Err("Connection timeout cannot be zero".to_string());
        }
        if self.maximum < self.default {
            return Err("Maximum timeout must be >= default timeout".to_string());
        }
        Ok(())
    }

    fn is_production_ready(&self) -> bool {
        self.connection >= Duration::from_secs(1)
            && self.connection <= Duration::from_secs(60)
            && self.default >= Duration::from_secs(5)
            && self.maximum >= self.default
            && self.validate().is_ok()
    }
}

/// Persistence configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PersistenceConfig {
    /// Enable persistence
    pub enabled: bool,

    /// Storage backend type (`Arc<str>` for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub backend: Arc<str>,

    /// Connection configuration
    pub connection: ConnectionConfig,

    /// Retention configuration
    pub retention: RetentionConfig,

    /// Archive configuration
    pub archive: ArchiveConfig,
}

/// Connection configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectionConfig {
    /// Connection URL (`Arc<str>` for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub url: Arc<str>,

    /// Connection pool size
    pub pool_size: usize,

    /// Connection timeout
    pub timeout: Duration,

    /// Retry configuration
    pub retry: RetryConfig,
}

/// Retention configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RetentionConfig {
    /// Retention period
    pub period: Duration,

    /// Cleanup interval
    pub cleanup_interval: Duration,

    /// Archive before deletion
    pub archive_before_delete: bool,
}

/// Archive configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArchiveConfig {
    /// Enable archiving
    pub enabled: bool,

    /// Archive storage type (`Arc<str>` for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub storage: Arc<str>,

    /// Compression
    pub compression: bool,

    /// Encryption
    pub encryption: bool,
}

/// Retry configuration for workflows
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetryConfig {
    /// Maximum retry attempts
    pub max_attempts: usize,

    /// Initial delay
    pub initial_delay: Duration,

    /// Backoff multiplier
    pub backoff_multiplier: f64,

    /// Maximum delay
    pub max_delay: Duration,
}

// Default implementations
impl Default for ConsolidatedWorkflowConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            engine: WorkflowEngineConfig::default(),
            escalation: WorkflowEscalationConfig::default(),
            scheduling: SchedulingConfig::default(),
            persistence: PersistenceConfig::default(),
        }
    }
}

impl WorkflowEngineConfig {
    /// Create configuration from a config source (modern pattern)
    pub fn from_source(source: &dyn crate::canonical::config::source::ConfigSource) -> Self {
        use crate::canonical::config::source::get_parsed;

        Self {
            engine_type: Arc::from(
                source
                    .get_or("BEARDOG_WORKFLOW_ENGINE_TYPE", "tokio")
                    .as_str(),
            ),
            worker_pool_size: get_parsed(source, "BEARDOG_WORKFLOW_WORKER_POOL_SIZE", 10),
            queue: QueueConfig::default(),
            timeouts: TimeoutConfig::default(),
        }
    }
}

impl Default for WorkflowEngineConfig {
    fn default() -> Self {
        use crate::canonical::config::source::EnvConfigSource;
        Self::from_source(&EnvConfigSource::new())
    }
}

impl WorkflowEscalationConfig {
    /// Create configuration from a config source (modern pattern)
    pub fn from_source(source: &dyn crate::canonical::config::source::ConfigSource) -> Self {
        use crate::canonical::config::source::{get_bool, get_parsed};
        use std::time::Duration;

        Self {
            enabled: get_bool(source, "BEARDOG_WORKFLOW_POLICY_ENABLED", false),
            rules: vec![],
            default_timeout: Duration::from_secs(get_parsed(
                source,
                "BEARDOG_WORKFLOW_DEFAULT_TIMEOUT_SECS",
                300,
            )),
            notifications: NotificationConfig::default(),
        }
    }
}

impl Default for WorkflowEscalationConfig {
    fn default() -> Self {
        use crate::canonical::config::source::EnvConfigSource;
        Self::from_source(&EnvConfigSource::new())
    }
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            channels: vec!["email".to_string()],
            templates: HashMap::new(),
            rate_limit: super::network::RateLimitConfig::default(),
        }
    }
}

// RateLimitConfig Default implementation removed - using canonical version

impl SchedulingConfig {
    /// Create configuration from a config source (modern pattern)
    pub fn from_source(source: &dyn crate::canonical::config::source::ConfigSource) -> Self {
        use crate::canonical::config::source::get_parsed;

        Self {
            scheduler_type: Arc::from(
                source
                    .get_or("BEARDOG_WORKFLOW_SCHEDULER_TYPE", "cron")
                    .as_str(),
            ),
            default_schedule: Arc::from(
                source
                    .get_or("BEARDOG_WORKFLOW_DEFAULT_SCHEDULE", "0 0 * * *")
                    .as_str(),
            ),
            timezone: Arc::from(source.get_or("BEARDOG_WORKFLOW_TIMEZONE", "UTC").as_str()),
            max_concurrent: get_parsed(source, "BEARDOG_WORKFLOW_MAX_CONCURRENT", 5),
        }
    }
}

impl Default for SchedulingConfig {
    fn default() -> Self {
        use crate::canonical::config::source::EnvConfigSource;
        Self::from_source(&EnvConfigSource::new())
    }
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            queue_type: Arc::from("memory"),
            capacity: std::env::var("BEARDOG_WORKFLOW_QUEUE_CAPACITY")
                .ok()
                .and_then(|c| c.parse().ok())
                .unwrap_or(1000), // 1000 messages default
            message_ttl: Duration::from_secs(
                std::env::var("BEARDOG_WORKFLOW_MESSAGE_TTL_SECS")
                    .ok()
                    .and_then(|t| t.parse().ok())
                    .unwrap_or(3600), // 1 hour default
            ),
            dead_letter_queue: None,
        }
    }
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            default: Duration::from_secs(
                std::env::var("BEARDOG_WORKFLOW_TIMEOUT_DEFAULT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
            maximum: Duration::from_secs(
                std::env::var("BEARDOG_WORKFLOW_TIMEOUT_MAXIMUM_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(300), // 5 minutes
            ),
            connection: Duration::from_secs(
                std::env::var("BEARDOG_WORKFLOW_TIMEOUT_CONNECTION_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(10),
            ),
            read: Duration::from_secs(
                std::env::var("BEARDOG_WORKFLOW_TIMEOUT_READ_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
        }
    }
}

impl Default for PersistenceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            backend: Arc::from("sqlite"),
            connection: ConnectionConfig::default(),
            retention: RetentionConfig::default(),
            archive: ArchiveConfig::default(),
        }
    }
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            url: Arc::from(
                std::env::var("BEARDOG_WORKFLOW_DB_URL")
                    .unwrap_or_else(|_| "sqlite://workflows.db".to_string())
                    .as_str(),
            ),
            pool_size: std::env::var("BEARDOG_WORKFLOW_DB_POOL_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10),
            timeout: Duration::from_secs(
                std::env::var("BEARDOG_WORKFLOW_DB_TIMEOUT_SECS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(30),
            ),
            retry: RetryConfig::default(),
        }
    }
}

impl Default for RetentionConfig {
    fn default() -> Self {
        Self {
            period: Duration::from_secs(
                std::env::var("BEARDOG_WORKFLOW_RETENTION_PERIOD_SECS")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(86400 * 30), // 30 days default
            ),
            cleanup_interval: Duration::from_secs(
                std::env::var("BEARDOG_WORKFLOW_CLEANUP_INTERVAL_SECS")
                    .ok()
                    .and_then(|i| i.parse().ok())
                    .unwrap_or(3600), // 1 hour default
            ),
            archive_before_delete: true,
        }
    }
}

impl Default for ArchiveConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            storage: Arc::from("filesystem"),
            compression: true,
            encryption: false,
        }
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: std::env::var("BEARDOG_WORKFLOW_RETRY_MAX_ATTEMPTS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3),
            initial_delay: Duration::from_millis(
                std::env::var("BEARDOG_RETRY_INITIAL_DELAY_MS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(100),
            ),
            backoff_multiplier: std::env::var("BEARDOG_RETRY_BACKOFF_MULTIPLIER")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(2.0),
            max_delay: Duration::from_secs(
                std::env::var("BEARDOG_RETRY_MAX_DELAY_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
        }
    }
}

// Implement RetryStrategy trait for workflow retry configuration
impl RetryStrategy for RetryConfig {
    fn max_attempts(&self) -> u32 {
        self.max_attempts as u32 // Convert from usize
    }

    fn delay_for_attempt(&self, attempt: u32) -> Duration {
        // Exponential backoff
        // Note: Precision loss is acceptable for delay calculations (not cryptographic)
        #[allow(
            clippy::cast_precision_loss,
            clippy::cast_possible_wrap,
            clippy::cast_sign_loss
        )]
        let delay_ms = {
            let initial_ms = self.initial_delay.as_millis().min(u64::MAX as u128) as f64;
            let computed = initial_ms * self.backoff_multiplier.powi(attempt.min(30) as i32);
            computed.max(0.0) as u64
        };
        let delay = Duration::from_millis(delay_ms);
        delay.min(self.max_delay)
    }

    fn backoff_multiplier(&self) -> f64 {
        self.backoff_multiplier
    }

    fn should_retry_error(&self, _error: &(dyn std::error::Error + Send + Sync)) -> bool {
        // Workflow config: retry all errors by default
        true
    }

    fn is_limit_reached(&self, attempts: u32) -> bool {
        attempts >= self.max_attempts as u32
    }

    fn total_delay(&self, attempts: u32) -> Duration {
        let mut total = Duration::from_secs(0);
        for attempt in 0..attempts {
            total += self.delay_for_attempt(attempt);
        }
        total
    }
}

// BearDogConfig implementation for ConsolidatedWorkflowConfig
impl BearDogConfig for ConsolidatedWorkflowConfig {
    fn validate(&self) -> Result<(), BearDogError> {
        if self.enabled {
            if self.engine.worker_pool_size == 0 {
                return Err(BearDogError::validation(
                    "Worker pool size must be greater than 0",
                ));
            }

            if self.engine.queue.capacity == 0 {
                return Err(BearDogError::validation(
                    "Queue capacity must be greater than 0",
                ));
            }

            if self.scheduling.max_concurrent == 0 {
                return Err(BearDogError::validation(
                    "Max concurrent workflows must be greater than 0",
                ));
            }

            if self.persistence.enabled && self.persistence.connection.pool_size == 0 {
                return Err(BearDogError::validation(
                    "Connection pool size must be greater than 0 when persistence is enabled",
                ));
            }

            if self.escalation.enabled && self.escalation.rules.is_empty() {
                return Err(BearDogError::validation(
                    "Escalation rules cannot be empty when escalation is enabled",
                ));
            }
        }
        Ok(())
    }

    fn merge(&self, other: &Self) -> Result<Self, BearDogError> {
        Ok(Self {
            enabled: other.enabled,
            engine: if other.enabled {
                other.engine.clone()
            } else {
                self.engine.clone()
            },
            escalation: if other.escalation.enabled {
                other.escalation.clone()
            } else {
                self.escalation.clone()
            },
            scheduling: if other.enabled {
                other.scheduling.clone()
            } else {
                self.scheduling.clone()
            },
            persistence: if other.persistence.enabled {
                other.persistence.clone()
            } else {
                self.persistence.clone()
            },
        })
    }

    fn from_env() -> Result<Self, BearDogError> {
        let mut config = Self::default();

        if let Ok(enabled) = std::env::var("BEARDOG_WORKFLOW_ENABLED") {
            config.enabled = enabled.parse().unwrap_or(true);
        }

        if let Ok(worker_pool_size) = std::env::var("BEARDOG_WORKFLOW_WORKER_POOL_SIZE") {
            config.engine.worker_pool_size = worker_pool_size.parse().unwrap_or(10);
        }

        if let Ok(queue_capacity) = std::env::var("BEARDOG_WORKFLOW_QUEUE_CAPACITY") {
            config.engine.queue.capacity = queue_capacity.parse().unwrap_or(1000);
        }

        if let Ok(max_concurrent) = std::env::var("BEARDOG_WORKFLOW_MAX_CONCURRENT") {
            config.scheduling.max_concurrent = max_concurrent.parse().unwrap_or(5);
        }

        if let Ok(persistence_enabled) = std::env::var("BEARDOG_WORKFLOW_PERSISTENCE_ENABLED") {
            config.persistence.enabled = persistence_enabled.parse().unwrap_or(true);
        }

        if let Ok(backend) = std::env::var("BEARDOG_WORKFLOW_PERSISTENCE_BACKEND") {
            config.persistence.backend = Arc::from(backend.as_str());
        }

        config.validate()?;
        Ok(config)
    }

    fn to_toml(&self) -> Result<String, BearDogError> {
        toml::to_string(self).map_err(|e| {
            BearDogError::system(format!("Failed to serialize workflow config to TOML: {e}"))
        })
    }

    fn domain() -> &'static str {
        "workflows"
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::config::r#trait::BearDogConfig;
    use crate::canonical::traits::TimeoutPolicy;
    use std::sync::Arc;
    use std::time::Duration;

    // ============================================================================
    // ConsolidatedWorkflowConfig Tests
    // ============================================================================

    #[test]
    fn test_consolidated_workflow_config_default() {
        let config = ConsolidatedWorkflowConfig::default();

        assert!(config.enabled);
        assert_eq!(config.engine.worker_pool_size, 10);
        assert!(config.persistence.enabled);
        assert_eq!(config.scheduling.max_concurrent, 5);
    }

    #[test]
    fn test_consolidated_workflow_config_clone() {
        let config = ConsolidatedWorkflowConfig::default();
        let cloned = config.clone();

        assert_eq!(config, cloned);
        assert_eq!(config.enabled, cloned.enabled);
        assert_eq!(
            config.engine.worker_pool_size,
            cloned.engine.worker_pool_size
        );
    }

    #[test]
    fn test_consolidated_workflow_config_serialization() {
        let config = ConsolidatedWorkflowConfig::default();

        // Test serialization to TOML
        let toml_result = config.to_toml();
        assert!(toml_result.is_ok());

        let toml_str = toml_result.unwrap();
        assert!(!toml_str.is_empty());
        assert!(toml_str.contains("enabled"));
    }

    #[test]
    fn test_consolidated_workflow_config_domain() {
        assert_eq!(ConsolidatedWorkflowConfig::domain(), "workflows");
    }

    #[test]
    fn test_consolidated_workflow_config_validation() {
        let config = ConsolidatedWorkflowConfig::default();
        let result = config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_consolidated_workflow_config_validation_fails_zero_workers() {
        let mut config = ConsolidatedWorkflowConfig::default();
        config.engine.worker_pool_size = 0;

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_consolidated_workflow_config_validation_fails_zero_capacity() {
        let mut config = ConsolidatedWorkflowConfig::default();
        config.engine.queue.capacity = 0;

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_consolidated_workflow_config_from_env() {
        // Set environment variables
        std::env::set_var("BEARDOG_WORKFLOW_ENABLED", "true");
        std::env::set_var("BEARDOG_WORKFLOW_WORKER_POOL_SIZE", "20");
        std::env::set_var("BEARDOG_WORKFLOW_QUEUE_CAPACITY", "2000");

        let result = ConsolidatedWorkflowConfig::from_env();
        assert!(result.is_ok());

        let config = result.unwrap();
        assert!(config.enabled);
        assert_eq!(config.engine.worker_pool_size, 20);
        assert_eq!(config.engine.queue.capacity, 2000);

        // Cleanup
        std::env::remove_var("BEARDOG_WORKFLOW_ENABLED");
        std::env::remove_var("BEARDOG_WORKFLOW_WORKER_POOL_SIZE");
        std::env::remove_var("BEARDOG_WORKFLOW_QUEUE_CAPACITY");
    }

    #[test]
    fn test_consolidated_workflow_config_merge() {
        let config1 = ConsolidatedWorkflowConfig::default();
        let mut config2 = ConsolidatedWorkflowConfig::default();
        config2.engine.worker_pool_size = 20;
        config2.enabled = true;

        let merged = config1.merge(&config2);
        assert!(merged.is_ok());

        let result = merged.unwrap();
        assert_eq!(result.engine.worker_pool_size, 20);
    }

    // ============================================================================
    // WorkflowEngineConfig Tests
    // ============================================================================

    #[test]
    fn test_workflow_engine_config_default() {
        let config = WorkflowEngineConfig::default();

        assert_eq!(config.engine_type.as_ref(), "tokio");
        assert_eq!(config.worker_pool_size, 10);
        assert_eq!(config.queue.capacity, 1000);
    }

    #[test]
    fn test_workflow_engine_config_clone() {
        let config = WorkflowEngineConfig::default();
        let cloned = config.clone();

        assert_eq!(config, cloned);
        assert_eq!(config.engine_type, cloned.engine_type);
    }

    #[test]
    fn test_workflow_engine_config_custom_values() {
        let config = WorkflowEngineConfig {
            engine_type: Arc::from("sync"),
            worker_pool_size: 25,
            queue: QueueConfig::default(),
            timeouts: TimeoutConfig::default(),
        };

        assert_eq!(config.engine_type.as_ref(), "sync");
        assert_eq!(config.worker_pool_size, 25);
    }

    // ============================================================================
    // QueueConfig Tests
    // ============================================================================

    #[test]
    fn test_queue_config_default() {
        let config = QueueConfig::default();

        assert_eq!(config.queue_type.as_ref(), "memory");
        assert_eq!(config.capacity, 1000);
        assert_eq!(config.message_ttl, Duration::from_secs(3600));
    }

    #[test]
    fn test_queue_config_clone() {
        let config = QueueConfig::default();
        let cloned = config.clone();

        assert_eq!(config, cloned);
    }

    #[test]
    fn test_queue_config_custom_capacity() {
        let mut config = QueueConfig::default();
        config.capacity = 5000;

        assert_eq!(config.capacity, 5000);
    }

    #[test]
    fn test_queue_config_dead_letter_queue() {
        let mut config = QueueConfig::default();
        config.dead_letter_queue = Some("dlq-workflows".to_string());

        assert!(config.dead_letter_queue.is_some());
        assert_eq!(config.dead_letter_queue.unwrap(), "dlq-workflows");
    }

    // ============================================================================
    // TimeoutConfig Tests (with TimeoutPolicy trait)
    // ============================================================================

    #[test]
    fn test_timeout_config_default() {
        let config = TimeoutConfig::default();

        assert_eq!(config.default, Duration::from_secs(30));
        assert_eq!(config.maximum, Duration::from_secs(300));
        assert_eq!(config.connection, Duration::from_secs(10));
        assert_eq!(config.read, Duration::from_secs(30));
    }

    #[test]
    fn test_timeout_config_clone() {
        let config = TimeoutConfig::default();
        let cloned = config.clone();

        assert_eq!(config, cloned);
    }

    #[test]
    fn test_timeout_config_timeout_policy_connection() {
        let config = TimeoutConfig::default();
        assert_eq!(config.connection_timeout(), Duration::from_secs(10));
    }

    #[test]
    fn test_timeout_config_timeout_policy_operation() {
        let config = TimeoutConfig::default();

        assert_eq!(config.operation_timeout("read"), Duration::from_secs(30));
        assert_eq!(config.operation_timeout("connect"), Duration::from_secs(10));
        assert_eq!(
            config.operation_timeout("connection"),
            Duration::from_secs(10)
        );
        assert_eq!(config.operation_timeout("other"), Duration::from_secs(30));
    }

    #[test]
    fn test_timeout_config_should_timeout() {
        let config = TimeoutConfig::default();

        assert!(!config.should_timeout(Duration::from_secs(5), "read"));
        assert!(config.should_timeout(Duration::from_secs(35), "read"));
        assert!(config.should_timeout(Duration::from_secs(15), "connection"));
    }

    #[test]
    fn test_timeout_config_global_timeout() {
        let config = TimeoutConfig::default();

        let global = config.global_timeout();
        assert!(global.is_some());
        assert_eq!(global.unwrap(), Duration::from_secs(300));
    }

    #[test]
    fn test_timeout_config_read_timeout() {
        let config = TimeoutConfig::default();
        assert_eq!(config.read_timeout(), Duration::from_secs(30));
    }

    #[test]
    fn test_timeout_config_write_timeout() {
        let config = TimeoutConfig::default();
        assert_eq!(config.write_timeout(), Duration::from_secs(30));
    }

    #[test]
    fn test_timeout_config_idle_timeout() {
        let config = TimeoutConfig::default();
        assert!(config.idle_timeout().is_none());
    }

    #[test]
    fn test_timeout_config_remaining_time() {
        let config = TimeoutConfig::default();

        let remaining = config.remaining_time(Duration::from_secs(10), "read");
        assert_eq!(remaining, Duration::from_secs(20));

        let remaining_overtime = config.remaining_time(Duration::from_secs(35), "read");
        assert_eq!(remaining_overtime, Duration::ZERO);
    }

    #[test]
    fn test_timeout_config_validate_success() {
        let config = TimeoutConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_timeout_config_validate_fails_zero_connection() {
        let config = TimeoutConfig {
            default: Duration::from_secs(30),
            maximum: Duration::from_secs(300),
            connection: Duration::ZERO,
            read: Duration::from_secs(30),
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_timeout_config_validate_fails_maximum_less_than_default() {
        let config = TimeoutConfig {
            default: Duration::from_secs(100),
            maximum: Duration::from_secs(50),
            connection: Duration::from_secs(10),
            read: Duration::from_secs(30),
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_timeout_config_is_production_ready() {
        let config = TimeoutConfig::default();
        assert!(config.is_production_ready());
    }

    #[test]
    fn test_timeout_config_not_production_ready_connection_too_short() {
        let config = TimeoutConfig {
            default: Duration::from_secs(30),
            maximum: Duration::from_secs(300),
            connection: Duration::from_millis(500),
            read: Duration::from_secs(30),
        };

        assert!(!config.is_production_ready());
    }

    #[test]
    fn test_timeout_config_not_production_ready_connection_too_long() {
        let config = TimeoutConfig {
            default: Duration::from_secs(30),
            maximum: Duration::from_secs(300),
            connection: Duration::from_secs(120),
            read: Duration::from_secs(30),
        };

        assert!(!config.is_production_ready());
    }

    // ============================================================================
    // SchedulingConfig Tests
    // ============================================================================

    #[test]
    fn test_scheduling_config_default() {
        let config = SchedulingConfig::default();

        assert_eq!(config.scheduler_type.as_ref(), "cron");
        assert_eq!(config.max_concurrent, 5);
        assert_eq!(config.timezone.as_ref(), "UTC");
    }

    #[test]
    fn test_scheduling_config_clone() {
        let config = SchedulingConfig::default();
        let cloned = config.clone();

        assert_eq!(config, cloned);
    }

    #[test]
    fn test_scheduling_config_custom_values() {
        let config = SchedulingConfig {
            scheduler_type: Arc::from("interval"),
            default_schedule: Arc::from("@hourly"),
            timezone: Arc::from("America/New_York"),
            max_concurrent: 10,
        };

        assert_eq!(config.scheduler_type.as_ref(), "interval");
        assert_eq!(config.max_concurrent, 10);
        assert_eq!(config.timezone.as_ref(), "America/New_York");
    }

    // ============================================================================
    // WorkflowEscalationConfig Tests
    // ============================================================================

    #[test]
    fn test_escalation_config_default() {
        let config = WorkflowEscalationConfig::default();

        assert!(!config.enabled);
        assert!(config.rules.is_empty());
        assert_eq!(config.default_timeout, Duration::from_secs(300));
    }

    #[test]
    fn test_escalation_config_clone() {
        let config = WorkflowEscalationConfig::default();
        let cloned = config.clone();

        assert_eq!(config, cloned);
    }

    #[test]
    fn test_escalation_rule_creation() {
        let rule = EscalationRule {
            name: Arc::from("high-priority"),
            condition: Arc::from("priority > 8"),
            target: Arc::from("manager-on-call"),
            timeout: Duration::from_secs(600),
        };

        assert_eq!(rule.name.as_ref(), "high-priority");
        assert_eq!(rule.condition.as_ref(), "priority > 8");
        assert_eq!(rule.target.as_ref(), "manager-on-call");
    }

    #[test]
    fn test_notification_config_default() {
        let config = NotificationConfig::default();

        assert_eq!(config.channels, vec!["email".to_string()]);
        assert!(config.templates.is_empty());
    }

    // ============================================================================
    // PersistenceConfig Tests
    // ============================================================================

    #[test]
    fn test_persistence_config_default() {
        let config = PersistenceConfig::default();

        assert!(config.enabled);
        assert_eq!(config.backend.as_ref(), "sqlite");
    }

    #[test]
    fn test_persistence_config_clone() {
        let config = PersistenceConfig::default();
        let cloned = config.clone();

        assert_eq!(config.enabled, cloned.enabled);
        assert_eq!(config.backend, cloned.backend);
    }

    #[test]
    fn test_persistence_config_custom_backend() {
        let mut config = PersistenceConfig::default();
        config.backend = Arc::from("postgresql");

        assert_eq!(config.backend.as_ref(), "postgresql");
    }

    // ============================================================================
    // Environment Variable Integration Tests
    // ============================================================================

    #[test]
    fn test_from_env_workflow_enabled() {
        std::env::set_var("BEARDOG_WORKFLOW_ENABLED", "false");

        let config = ConsolidatedWorkflowConfig::from_env().unwrap();
        assert!(!config.enabled);

        std::env::remove_var("BEARDOG_WORKFLOW_ENABLED");
    }

    #[test]
    fn test_from_env_max_concurrent() {
        std::env::set_var("BEARDOG_WORKFLOW_MAX_CONCURRENT", "15");

        let config = ConsolidatedWorkflowConfig::from_env().unwrap();
        assert_eq!(config.scheduling.max_concurrent, 15);

        std::env::remove_var("BEARDOG_WORKFLOW_MAX_CONCURRENT");
    }

    #[test]
    fn test_from_env_persistence_enabled() {
        std::env::set_var("BEARDOG_WORKFLOW_PERSISTENCE_ENABLED", "false");

        let config = ConsolidatedWorkflowConfig::from_env().unwrap();
        assert!(!config.persistence.enabled);

        std::env::remove_var("BEARDOG_WORKFLOW_PERSISTENCE_ENABLED");
    }

    #[test]
    fn test_from_env_persistence_backend() {
        std::env::set_var("BEARDOG_WORKFLOW_PERSISTENCE_BACKEND", "mongodb");

        let config = ConsolidatedWorkflowConfig::from_env().unwrap();
        assert_eq!(config.persistence.backend.as_ref(), "mongodb");

        std::env::remove_var("BEARDOG_WORKFLOW_PERSISTENCE_BACKEND");
    }

    #[test]
    fn test_from_env_invalid_values_use_defaults() {
        std::env::set_var("BEARDOG_WORKFLOW_WORKER_POOL_SIZE", "invalid");
        std::env::set_var("BEARDOG_WORKFLOW_QUEUE_CAPACITY", "not_a_number");

        let config = ConsolidatedWorkflowConfig::from_env().unwrap();
        assert_eq!(config.engine.worker_pool_size, 10); // Falls back to default
        assert_eq!(config.engine.queue.capacity, 1000); // Falls back to default

        std::env::remove_var("BEARDOG_WORKFLOW_WORKER_POOL_SIZE");
        std::env::remove_var("BEARDOG_WORKFLOW_QUEUE_CAPACITY");
    }

    // ============================================================================
    // Integration Tests
    // ============================================================================

    #[test]
    fn test_full_config_lifecycle() {
        // Create config
        let config = ConsolidatedWorkflowConfig::default();

        // Validate
        assert!(config.validate().is_ok());

        // Serialize to TOML
        let toml_result = config.to_toml();
        assert!(toml_result.is_ok());

        // Clone
        let cloned = config.clone();
        assert_eq!(config, cloned);
    }

    #[test]
    fn test_merge_preserves_enabled_settings() {
        let mut config1 = ConsolidatedWorkflowConfig::default();
        config1.enabled = true;
        config1.escalation.enabled = false;

        let mut config2 = ConsolidatedWorkflowConfig::default();
        config2.enabled = true;
        config2.escalation.enabled = true;

        let merged = config1.merge(&config2).unwrap();
        assert!(merged.escalation.enabled);
    }

    #[test]
    fn test_timeout_policy_comprehensive() {
        let config = TimeoutConfig::default();

        // Test all timeout methods
        assert!(config.connection_timeout() > Duration::ZERO);
        assert!(config.read_timeout() > Duration::ZERO);
        assert!(config.write_timeout() > Duration::ZERO);
        assert!(config.global_timeout().is_some());
        assert!(config.idle_timeout().is_none());

        // Test validation
        assert!(config.validate().is_ok());
        assert!(config.is_production_ready());
    }

    #[test]
    fn test_arc_str_usage_for_zero_copy() {
        let config = WorkflowEngineConfig::default();

        // Arc<str> should allow cheap cloning
        let engine_type_1 = Arc::clone(&config.engine_type);
        let engine_type_2 = Arc::clone(&config.engine_type);

        // Should point to same allocation
        assert!(Arc::ptr_eq(&engine_type_1, &engine_type_2));
    }
}
