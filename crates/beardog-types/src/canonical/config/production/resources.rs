// SPDX-License-Identifier: AGPL-3.0-only

// Resource Management Configuration
//
// This module contains system resource management configuration including
// memory, CPU, network, storage, and connection management settings.

use crate::constants::domains::network::limits::MAX_CONNECTIONS;
use crate::constants::time;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **RESOURCE MANAGEMENT CONFIGURATION** - System resource management
///
/// CPU, network connections, storage, and connection pooling.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceManagementConfig {
    /// Memory management
    /// The memory value
    pub memory: MemoryConfig,

    /// CPU management
    /// The cpu value
    pub cpu: CpuConfig,

    /// Network resource management
    /// The network value
    pub network: NetworkResourceConfig,

    /// Storage resource management
    /// The storage value
    pub storage: StorageResourceConfig,

    /// Connection management
    /// The connections value
    pub connections: ConnectionConfig,
}

/// Memory configuration and tuning
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryConfig {
    /// Maximum heap size in MB
    /// Optional max heap size mb
    pub max_heap_size_mb: Option<usize>,
    /// Initial heap size in MB
    /// Optional initial heap size mb
    pub initial_heap_size_mb: Option<usize>,
    /// Enable memory profiling
    /// Whether `enable_memory_profiling` is enabled
    pub enable_memory_profiling: bool,
    /// Garbage collection tuning
    /// The gc tuning value
    pub gc_tuning: GcTuningConfig,
}

/// Garbage collection tuning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcTuningConfig {
    /// GC strategy to use
    /// The strategy value
    pub strategy: GcStrategy,
    /// Target pause time in milliseconds
    /// Optional target pause ms
    pub target_pause_ms: Option<u64>,
    /// Throughput target percentage
    /// Optional throughput target percent
    pub throughput_target_percent: Option<u8>,
}

/// Garbage collection strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GcStrategy {
    /// Use system default GC
    Default,
    /// Optimize for maximal mutator CPU time (may increase pause times).
    Throughput,
    /// Optimize for shortest stop-the-world pauses (may reduce throughput).
    LowLatency,
    /// Balanced approach
    Balanced,
}

/// CPU configuration and affinity
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CpuConfig {
    /// Maximum number of cores to use
    /// Optional max cores
    pub max_cores: Option<usize>,
    /// CPU affinity mask
    /// Optional affinity
    pub affinity: Option<Vec<usize>>,
    /// Enable CPU profiling
    /// Whether `enable_cpu_profiling` is enabled
    pub enable_cpu_profiling: bool,
    /// Thread pool size
    /// Optional thread pool size
    pub thread_pool_size: Option<usize>,
}

/// Network resource configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkResourceConfig {
    /// Maximum concurrent connections
    /// Number of `max_connections`
    pub max_connections: usize,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Read timeout
    pub read_timeout: Duration,
    /// Write timeout
    pub write_timeout: Duration,
    /// Enable TCP keep-alive
    /// Whether `keep_alive` is enabled
    pub keep_alive: bool,
    /// Enable `TCP_NODELAY`
    /// Whether `tcp_nodelay` is enabled
    pub tcp_nodelay: bool,
}

impl NetworkResourceConfig {
    /// Default maximum connections
    pub const DEFAULT_MAX_CONNECTIONS: usize = 1000;

    /// Default connection timeout in seconds
    pub const DEFAULT_CONNECTION_TIMEOUT_SECS: u64 = 30;

    /// Default read timeout in seconds
    pub const DEFAULT_READ_TIMEOUT_SECS: u64 = 30;

    /// Default write timeout in seconds
    pub const DEFAULT_WRITE_TIMEOUT_SECS: u64 = 30;

    /// Create `NetworkResourceConfig` with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    pub const fn with_defaults() -> Self {
        Self {
            max_connections: Self::DEFAULT_MAX_CONNECTIONS,
            connection_timeout: Duration::from_secs(Self::DEFAULT_CONNECTION_TIMEOUT_SECS),
            read_timeout: Duration::from_secs(Self::DEFAULT_READ_TIMEOUT_SECS),
            write_timeout: Duration::from_secs(Self::DEFAULT_WRITE_TIMEOUT_SECS),
            keep_alive: true,
            tcp_nodelay: true,
        }
    }

    /// Create `NetworkResourceConfig` from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    ///
    /// # Environment Variables
    /// - `BEARDOG_PROD_MAX_CONNECTIONS`: Maximum connections (default: 1000)
    /// - `BEARDOG_PROD_CONNECTION_TIMEOUT_SECS`: Connection timeout (default: 30)
    /// - `BEARDOG_PROD_READ_TIMEOUT_SECS`: Read timeout (default: 30)
    /// - `BEARDOG_PROD_WRITE_TIMEOUT_SECS`: Write timeout (default: 30)
    pub fn from_env() -> Self {
        Self::from_env_provider(|k| std::env::var(k).ok())
    }

    /// Load from a custom environment provider (e.g. tests); production uses [`Self::from_env`].
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Self {
        Self {
            max_connections: get("BEARDOG_PROD_MAX_CONNECTIONS")
                .and_then(|c| c.parse().ok())
                .unwrap_or(Self::DEFAULT_MAX_CONNECTIONS),
            connection_timeout: Duration::from_secs(
                get("BEARDOG_PROD_CONNECTION_TIMEOUT_SECS")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(Self::DEFAULT_CONNECTION_TIMEOUT_SECS),
            ),
            read_timeout: Duration::from_secs(
                get("BEARDOG_PROD_READ_TIMEOUT_SECS")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(Self::DEFAULT_READ_TIMEOUT_SECS),
            ),
            write_timeout: Duration::from_secs(
                get("BEARDOG_PROD_WRITE_TIMEOUT_SECS")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(Self::DEFAULT_WRITE_TIMEOUT_SECS),
            ),
            keep_alive: true,
            tcp_nodelay: true,
        }
    }
}

/// Storage resource configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResourceConfig {
    /// Maximum disk usage percentage
    /// The max disk usage percent value
    pub max_disk_usage_percent: f64,
    /// Temporary directory cleanup interval
    /// The temp dir cleanup interval value
    pub temp_dir_cleanup_interval: Duration,
    /// Log rotation size in MB
    /// Number of `log_rotation_size_mb`
    pub log_rotation_size_mb: u64,
    /// Log retention in days
    /// Number of `log_retention_days`
    pub log_retention_days: u32,
}

/// Connection pooling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    /// Connection pool size
    /// Number of `pool_size`
    pub pool_size: usize,
    /// Maximum idle connections
    pub max_idle_connections: usize,
    /// Connection lifetime
    pub connection_lifetime: Duration,
    /// Health check interval
    /// The health check interval value
    pub health_check_interval: Duration,
}

impl StorageResourceConfig {
    /// Default maximum disk usage percentage
    pub const DEFAULT_MAX_DISK_USAGE_PERCENT: f64 = 80.0;

    /// Default temp cleanup interval in seconds
    pub const DEFAULT_TEMP_CLEANUP_INTERVAL_SECS: u64 = time::SECONDS_PER_HOUR;

    /// Default log rotation size in MB
    pub const DEFAULT_LOG_ROTATION_SIZE_MB: u64 = 100;

    /// Default log retention in days
    pub const DEFAULT_LOG_RETENTION_DAYS: u32 = 30;

    /// Create `StorageResourceConfig` with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    pub const fn with_defaults() -> Self {
        Self {
            max_disk_usage_percent: Self::DEFAULT_MAX_DISK_USAGE_PERCENT,
            temp_dir_cleanup_interval: Duration::from_secs(
                Self::DEFAULT_TEMP_CLEANUP_INTERVAL_SECS,
            ),
            log_rotation_size_mb: Self::DEFAULT_LOG_ROTATION_SIZE_MB,
            log_retention_days: Self::DEFAULT_LOG_RETENTION_DAYS,
        }
    }

    /// Create `StorageResourceConfig` from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    ///
    /// # Environment Variables
    /// - `BEARDOG_MAX_DISK_USAGE_PERCENT`: Max disk usage % (default: 80.0)
    /// - `BEARDOG_TEMP_CLEANUP_INTERVAL_SECS`: Cleanup interval (default: 3600)
    /// - `BEARDOG_LOG_ROTATION_SIZE_MB`: Log rotation size (default: 100)
    /// - `BEARDOG_LOG_RETENTION_DAYS`: Log retention days (default: 30)
    pub fn from_env() -> Self {
        Self::from_env_provider(|k| std::env::var(k).ok())
    }

    /// Load from a custom environment provider (e.g. tests); production uses [`Self::from_env`].
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Self {
        Self {
            max_disk_usage_percent: get("BEARDOG_MAX_DISK_USAGE_PERCENT")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_MAX_DISK_USAGE_PERCENT),
            temp_dir_cleanup_interval: Duration::from_secs(
                get("BEARDOG_TEMP_CLEANUP_INTERVAL_SECS")
                    .and_then(|i| i.parse().ok())
                    .unwrap_or(Self::DEFAULT_TEMP_CLEANUP_INTERVAL_SECS),
            ),
            log_rotation_size_mb: get("BEARDOG_LOG_ROTATION_SIZE_MB")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_LOG_ROTATION_SIZE_MB),
            log_retention_days: get("BEARDOG_LOG_RETENTION_DAYS")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_LOG_RETENTION_DAYS),
        }
    }
}

impl ConnectionConfig {
    /// Default connection pool size
    pub const DEFAULT_POOL_SIZE: usize = 10;

    /// Default max idle connections
    pub const DEFAULT_MAX_IDLE_CONNECTIONS: usize = 5;

    /// Default connection lifetime in seconds
    pub const DEFAULT_CONNECTION_LIFETIME_SECS: u64 = 1800;

    /// Default health check interval in seconds
    pub const DEFAULT_HEALTH_CHECK_INTERVAL_SECS: u64 = 60;

    /// Create `ConnectionConfig` with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    pub const fn with_defaults() -> Self {
        Self {
            pool_size: Self::DEFAULT_POOL_SIZE,
            max_idle_connections: Self::DEFAULT_MAX_IDLE_CONNECTIONS,
            connection_lifetime: Duration::from_secs(Self::DEFAULT_CONNECTION_LIFETIME_SECS),
            health_check_interval: Duration::from_secs(Self::DEFAULT_HEALTH_CHECK_INTERVAL_SECS),
        }
    }

    /// Create `ConnectionConfig` from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    ///
    /// # Environment Variables
    /// - `BEARDOG_CONNECTION_POOL_SIZE`: Pool size (default: 10)
    /// - `BEARDOG_MAX_IDLE_CONNECTIONS`: Max idle connections (default: 5)
    /// - `BEARDOG_CONNECTION_LIFETIME_SECS`: Connection lifetime (default: 1800)
    /// - `BEARDOG_CONNECTION_HEALTH_CHECK_INTERVAL_SECS`: Health check interval (default: 60)
    pub fn from_env() -> Self {
        Self::from_env_provider(|k| std::env::var(k).ok())
    }

    /// Load from a custom environment provider (e.g. tests); production uses [`Self::from_env`].
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Self {
        Self {
            pool_size: get("BEARDOG_CONNECTION_POOL_SIZE")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_POOL_SIZE),
            max_idle_connections: get("BEARDOG_MAX_IDLE_CONNECTIONS")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_MAX_IDLE_CONNECTIONS),
            connection_lifetime: Duration::from_secs(
                get("BEARDOG_CONNECTION_LIFETIME_SECS")
                    .and_then(|l| l.parse().ok())
                    .unwrap_or(Self::DEFAULT_CONNECTION_LIFETIME_SECS),
            ),
            health_check_interval: Duration::from_secs(
                get("BEARDOG_CONNECTION_HEALTH_CHECK_INTERVAL_SECS")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(Self::DEFAULT_HEALTH_CHECK_INTERVAL_SECS),
            ),
        }
    }
}

impl GcTuningConfig {
    /// Create `GcTuningConfig` with hardcoded defaults
    pub const fn with_defaults() -> Self {
        Self {
            strategy: GcStrategy::Default,
            target_pause_ms: None,
            throughput_target_percent: None,
        }
    }

    /// Create `GcTuningConfig` from environment variables
    pub fn from_env() -> Self {
        Self::from_env_provider(|k| std::env::var(k).ok())
    }

    /// Load from a custom environment provider (e.g. tests); production uses [`Self::from_env`].
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Self {
        let target_pause_ms = get("BEARDOG_GC_TARGET_PAUSE_MS").and_then(|s| s.parse().ok());
        let throughput_target_percent =
            get("BEARDOG_GC_THROUGHPUT_TARGET_PERCENT").and_then(|s| s.parse().ok());

        Self {
            strategy: GcStrategy::Default,
            target_pause_ms,
            throughput_target_percent,
        }
    }
}

impl Default for GcTuningConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl Default for NetworkResourceConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl Default for StorageResourceConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl ResourceManagementConfig {
    /// Returns [`Default`] materialized values (safe for tests).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Aggressive limits and profiling-friendly defaults suitable for large production clusters.
    #[must_use]
    pub fn production() -> Self {
        Self {
            memory: MemoryConfig {
                enable_memory_profiling: true,
                gc_tuning: GcTuningConfig {
                    strategy: GcStrategy::Balanced,
                    target_pause_ms: Some(10),
                    throughput_target_percent: Some(95),
                },
                ..Default::default()
            },
            cpu: CpuConfig {
                enable_cpu_profiling: true,
                ..Default::default()
            },
            network: NetworkResourceConfig {
                max_connections: std::env::var("BEARDOG_PROD_PRODUCTION_MAX_CONNECTIONS")
                    .ok()
                    .and_then(|c| c.parse().ok())
                    .unwrap_or(MAX_CONNECTIONS),
                connection_timeout: Duration::from_secs(
                    std::env::var("BEARDOG_PROD_PRODUCTION_CONNECTION_TIMEOUT_SECS")
                        .ok()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(10),
                ),
                ..Default::default()
            },
            storage: StorageResourceConfig {
                max_disk_usage_percent: std::env::var(
                    "BEARDOG_PROD_PRODUCTION_MAX_DISK_USAGE_PERCENT",
                )
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(70.0),
                temp_dir_cleanup_interval: Duration::from_secs(
                    std::env::var("BEARDOG_PROD_PRODUCTION_TEMP_CLEANUP_INTERVAL_SECS")
                        .ok()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(1800), // 30 minutes
                ),
                log_rotation_size_mb: std::env::var("BEARDOG_PRODUCTION_LOG_ROTATION_SIZE_MB")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(500),
                log_retention_days: std::env::var("BEARDOG_PROD_PRODUCTION_LOG_RETENTION_DAYS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(90),
            },
            connections: ConnectionConfig {
                pool_size: std::env::var("BEARDOG_PROD_PRODUCTION_POOL_SIZE")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(50),
                max_idle_connections: std::env::var("BEARDOG_PROD_PRODUCTION_MAX_IDLE_CONNECTIONS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(25),
                connection_lifetime: Duration::from_secs(
                    std::env::var("BEARDOG_PROD_PRODUCTION_CONNECTION_LIFETIME_SECS")
                        .ok()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(time::SECONDS_PER_HOUR), // 1 hour
                ),
                health_check_interval: Duration::from_secs(
                    std::env::var("BEARDOG_PROD_PRODUCTION_HEALTH_CHECK_INTERVAL_SECS")
                        .ok()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(30), // 30 seconds
                ),
            },
        }
    }

    /// Validate the resource configuration
    /// Validates input
    ///
    /// # Errors
    ///
    /// Returns an error if memory, network, storage, or connection limits are inconsistent or out of range.
    pub fn validate(&self) -> Result<(), BearDogError> {
        // Validate memory settings
        if let (Some(initial), Some(max)) = (
            self.memory.initial_heap_size_mb,
            self.memory.max_heap_size_mb,
        ) && initial > max
        {
            return Err(BearDogError::Business {
                message: "Initial heap size cannot be larger than maximum heap size".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }

        // Validate network settings
        if self.network.max_connections == 0 {
            return Err(BearDogError::Business {
                message: "Maximum connections must be greater than 0".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }

        // Validate storage settings
        if self.storage.max_disk_usage_percent <= 0.0 || self.storage.max_disk_usage_percent > 100.0
        {
            return Err(BearDogError::Business {
                message: "Disk usage percentage must be between 0 and 100".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }

        // Validate connection settings
        if self.connections.pool_size == 0 {
            return Err(BearDogError::Business {
                message: "Connection pool size must be greater than 0".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }

        if self.connections.max_idle_connections > self.connections.pool_size {
            return Err(BearDogError::Business {
                message: "Max idle connections cannot exceed pool size".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }

        Ok(())
    }
}
