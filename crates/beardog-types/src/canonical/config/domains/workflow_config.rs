//! # Workflow Configuration Domain
//!
//! This module contains all workflow-related configuration types, extracted from
//! the large `consolidated_domains.rs` file for better maintainability.

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    /// Engine type
    pub engine_type: String,

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
    /// Rule name
    pub name: String,

    /// Condition
    pub condition: String,

    /// Escalation target
    pub target: String,

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
pub type RateLimitConfig = super::network::RateLimitConfig;

/// Scheduling configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SchedulingConfig {
    /// Scheduler type
    pub scheduler_type: String,

    /// Default schedule
    pub default_schedule: String,

    /// Time zone
    pub timezone: String,

    /// Concurrent execution limit
    pub max_concurrent: usize,
}

/// Queue configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QueueConfig {
    /// Queue type
    pub queue_type: String,

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

/// Persistence configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PersistenceConfig {
    /// Enable persistence
    pub enabled: bool,

    /// Storage backend
    pub backend: String,

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
    /// Connection URL
    pub url: String,

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

    /// Archive storage
    pub storage: String,

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

impl Default for WorkflowEngineConfig {
    fn default() -> Self {
        Self {
            engine_type: "tokio".to_string(),
            worker_pool_size: 10,
            queue: QueueConfig::default(),
            timeouts: TimeoutConfig::default(),
        }
    }
}

impl Default for WorkflowEscalationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            rules: vec![],
            default_timeout: Duration::from_secs(300), // 5 minutes
            notifications: NotificationConfig::default(),
        }
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

impl Default for SchedulingConfig {
    fn default() -> Self {
        Self {
            scheduler_type: "cron".to_string(),
            default_schedule: "0 0 * * *".to_string(), // Daily at midnight
            timezone: "UTC".to_string(),
            max_concurrent: 5,
        }
    }
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            queue_type: "memory".to_string(),
            capacity: 1000,
            message_ttl: Duration::from_secs(3600), // 1 hour
            dead_letter_queue: None,
        }
    }
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            default: Duration::from_secs(30),
            maximum: Duration::from_secs(300), // 5 minutes
            connection: Duration::from_secs(10),
            read: Duration::from_secs(30),
        }
    }
}

impl Default for PersistenceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            backend: "sqlite".to_string(),
            connection: ConnectionConfig::default(),
            retention: RetentionConfig::default(),
            archive: ArchiveConfig::default(),
        }
    }
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            url: "sqlite://workflows.db".to_string(),
            pool_size: 10,
            timeout: Duration::from_secs(30),
            retry: RetryConfig::default(),
        }
    }
}

impl Default for RetentionConfig {
    fn default() -> Self {
        Self {
            period: Duration::from_secs(86400 * 30),     // 30 days
            cleanup_interval: Duration::from_secs(3600), // 1 hour
            archive_before_delete: true,
        }
    }
}

impl Default for ArchiveConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            storage: "filesystem".to_string(),
            compression: true,
            encryption: false,
        }
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            backoff_multiplier: 2.0,
            max_delay: Duration::from_secs(30),
        }
    }
}

// BearDogConfig implementation for ConsolidatedWorkflowConfig
impl BearDogConfig for ConsolidatedWorkflowConfig {
    fn validate(&self) -> BearDogResult<()> {
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

    fn merge(&self, other: &Self) -> BearDogResult<Self> {
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

    fn from_env() -> BearDogResult<Self> {
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
            config.persistence.backend = backend;
        }

        config.validate()?;
        Ok(config)
    }

    fn to_toml(&self) -> BearDogResult<String> {
        toml::to_string(self).map_err(|e| {
            BearDogError::system(format!("Failed to serialize workflow config to TOML: {e}"))
        })
    }

    fn domain() -> &'static str {
        "workflows"
    }
}
