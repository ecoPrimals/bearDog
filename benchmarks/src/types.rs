

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_size: u32,
    pub window: Duration,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 1000,
            burst_size: 100,
            window: Duration::from_secs(60),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryOptimizationConfig {
    pub buffer_pool_size: usize,
    pub max_buffer_size: usize,
    pub enable_zero_copy: bool,
}

impl Default for MemoryOptimizationConfig {
    fn default() -> Self {
        Self {
            buffer_pool_size: 1024,
            max_buffer_size: 64 * 1024,
            enable_zero_copy: true,
        }
    }
}

impl MemoryOptimizationConfig {
    #[must_use] pub const fn production() -> Self {
        Self {
            buffer_pool_size: 4096,
            max_buffer_size: 1024 * 1024,
            enable_zero_copy: true,
        }
    }

    #[must_use] pub const fn development() -> Self {
        Self {
            buffer_pool_size: 512,
            max_buffer_size: 32 * 1024,
            enable_zero_copy: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncOptimizationConfig {
    pub max_concurrent_tasks: usize,
    pub task_queue_size: usize,
    pub enable_work_stealing: bool,
}

impl Default for AsyncOptimizationConfig {
    fn default() -> Self {
        Self {
            max_concurrent_tasks: 100,
            task_queue_size: 1000,
            enable_work_stealing: true,
        }
    }
}

impl AsyncOptimizationConfig {
    #[must_use] pub const fn production() -> Self {
        Self {
            max_concurrent_tasks: 1000,
            task_queue_size: 10000,
            enable_work_stealing: true,
        }
    }

    #[must_use] pub const fn development() -> Self {
        Self {
            max_concurrent_tasks: 50,
            task_queue_size: 500,
            enable_work_stealing: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizedDatabaseConfig {
    pub max_connections: u32,
    pub connection_timeout: Duration,
    pub query_timeout: Duration,
}

impl Default for OptimizedDatabaseConfig {
    fn default() -> Self {
        Self {
            max_connections: 10,
            connection_timeout: Duration::from_secs(30),
            query_timeout: Duration::from_secs(10),
        }
    }
}

impl OptimizedDatabaseConfig {

    pub fn validate(&self) -> Result<(), String> {
        if self.max_connections == 0 {
            return Err("max_connections must be greater than 0".to_string());
        }
        if self.connection_timeout.as_secs() == 0 {
            return Err("connection_timeout must be greater than 0".to_string());
        }
        if self.query_timeout.as_secs() == 0 {
            return Err("query_timeout must be greater than 0".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountStatus {
    Active,
    Inactive,
    Suspended,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCopyPerformanceMetrics {
    pub buffer_pool_hit: bool,
    pub allocations: u32,
    pub response_size_bytes: u64,
    pub zero_copy_ops: u32,
}
