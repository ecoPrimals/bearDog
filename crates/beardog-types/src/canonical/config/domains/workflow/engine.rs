// SPDX-License-Identifier: AGPL-3.0-or-later

//! Workflow Engine Configuration
//!
//! Configuration for workflow execution engines, queues, and timeouts.

use crate::canonical::traits::TimeoutPolicy;
use beardog_config::env_keys;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

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

// ============================================================================
// Trait Implementations
// ============================================================================

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

// ============================================================================
// Default Implementations
// ============================================================================

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

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            queue_type: Arc::from("memory"),
            capacity: std::env::var(env_keys::ENV_WORKFLOW_QUEUE_CAPACITY)
                .ok()
                .and_then(|c| c.parse().ok())
                .unwrap_or(1000), // 1000 messages default
            message_ttl: Duration::from_secs(
                std::env::var(env_keys::ENV_WORKFLOW_MESSAGE_TTL_SECS)
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
                std::env::var(env_keys::ENV_WORKFLOW_TIMEOUT_DEFAULT_SECS)
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
            maximum: Duration::from_secs(
                std::env::var(env_keys::ENV_WORKFLOW_TIMEOUT_MAXIMUM_SECS)
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(300), // 5 minutes
            ),
            connection: Duration::from_secs(
                std::env::var(env_keys::ENV_WORKFLOW_TIMEOUT_CONNECTION_SECS)
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(10),
            ),
            read: Duration::from_secs(
                std::env::var(env_keys::ENV_WORKFLOW_TIMEOUT_READ_SECS)
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60),
            ),
        }
    }
}
