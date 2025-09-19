// Resource Management Configuration
//
// This module contains system resource management configuration including
// memory, CPU, network, storage, and connection management settings.

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
    Throughput,
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

impl Default for GcTuningConfig {
    fn default() -> Self {
        Self {
            strategy: GcStrategy::Default,
            target_pause_ms: None,
            throughput_target_percent: None,
        }
    }
}

impl Default for NetworkResourceConfig {
    fn default() -> Self {
        Self {
            max_connections: 1000,
            connection_timeout: Duration::from_secs(30),
            read_timeout: Duration::from_secs(30),
            write_timeout: Duration::from_secs(30),
            keep_alive: true,
            tcp_nodelay: true,
        }
    }
}

impl Default for StorageResourceConfig {
    fn default() -> Self {
        Self {
            max_disk_usage_percent: 80.0,
            temp_dir_cleanup_interval: Duration::from_secs(3600), // 1 hour
            log_rotation_size_mb: 100,
            log_retention_days: 30,
        }
    }
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            pool_size: 10,
            max_idle_connections: 5,
            connection_lifetime: Duration::from_secs(1800), // 30 minutes
            health_check_interval: Duration::from_secs(60), // 1 minute
        }
    }
}

impl ResourceManagementConfig {
    /// Create a new resource management configuration
    #[must_use]
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }

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
                max_connections: 10000,
                connection_timeout: Duration::from_secs(10),
                ..Default::default()
            },
            storage: StorageResourceConfig {
                max_disk_usage_percent: 70.0,
                temp_dir_cleanup_interval: Duration::from_secs(1800), // 30 minutes
                log_rotation_size_mb: 500,
                log_retention_days: 90,
            },
            connections: ConnectionConfig {
                pool_size: 50,
                max_idle_connections: 25,
                connection_lifetime: Duration::from_secs(3600), // 1 hour
                health_check_interval: Duration::from_secs(30), // 30 seconds
            },
        }
    }

    /// Validate the resource configuration
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        // Validate memory settings
        if let (Some(initial), Some(max)) = (
            self.memory.initial_heap_size_mb,
            self.memory.max_heap_size_mb,
        ) {
            if initial > max {
                return Err(BearDogError::Business {
                    message: "Initial heap size cannot be larger than maximum heap size"
                        .to_string(),
                    category: beardog_errors::BusinessErrorCategory::Validation,
                });
            }
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
