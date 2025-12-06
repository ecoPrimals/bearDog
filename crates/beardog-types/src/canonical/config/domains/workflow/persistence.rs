//! Workflow Persistence Configuration
//!
//! Configuration for workflow persistence, storage, retention, and archival.

use super::retry::RetryConfig;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

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

// ============================================================================
// Default Implementations
// ============================================================================

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
