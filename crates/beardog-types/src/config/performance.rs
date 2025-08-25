// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Performance Configuration - Canonical
///
/// **UNIFIED PERFORMANCE CONFIGURATION** for the BearDog ecosystem

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **CANONICAL** Async Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncConfig {
    pub max_concurrent_tasks: usize,
    pub task_queue_size: usize,
    pub worker_threads: Option<usize>,
    pub enable_work_stealing: bool,
    pub stack_size: Option<usize>,
}

impl Default for AsyncConfig {
    fn default() -> Self {
        Self {
            max_concurrent_tasks: 1000,
            task_queue_size: 10000,
            worker_threads: None, // Use system default
            enable_work_stealing: true,
            stack_size: None,
        }
    }
}

/// **CANONICAL** Memory Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub initial_heap_size: Option<usize>,
    pub max_heap_size: Option<usize>,
    pub enable_memory_mapping: bool,
    pub buffer_pool_size: usize,
    pub enable_zero_copy: bool,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            initial_heap_size: None,
            max_heap_size: None,
            enable_memory_mapping: true,
            buffer_pool_size: 1024 * 1024, // 1MB
            enable_zero_copy: true,
        }
    }
}

/// **CANONICAL** General Performance Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct GeneralPerformanceConfig {
    pub async_config: AsyncConfig,
    pub memory_config: MemoryConfig,
    pub io_config: IoOptimizationConfig,
    pub concurrency_config: ConcurrencyConfig,
    pub timeout_config: TimeoutConfig,
}


/// **CANONICAL** IO Optimization Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoOptimizationConfig {
    pub enable_async_io: bool,
    pub io_buffer_size: usize,
    pub enable_direct_io: bool,
    pub prefetch_size: usize,
}

impl Default for IoOptimizationConfig {
    fn default() -> Self {
        Self {
            enable_async_io: true,
            io_buffer_size: 64 * 1024, // 64KB
            enable_direct_io: false,
            prefetch_size: 256 * 1024, // 256KB
        }
    }
}

/// **CANONICAL** Concurrency Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrencyConfig {
    pub max_connections: usize,
    pub connection_pool_size: usize,
    pub enable_connection_pooling: bool,
    pub enable_multiplexing: bool,
}

impl Default for ConcurrencyConfig {
    fn default() -> Self {
        Self {
            max_connections: 1000,
            connection_pool_size: 100,
            enable_connection_pooling: true,
            enable_multiplexing: true,
        }
    }
}

/// **CANONICAL** Timeout Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutConfig {
    pub connection_timeout: Duration,
    pub request_timeout: Duration,
    pub idle_timeout: Duration,
    pub hsm_timeout: Duration,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            connection_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(60),
            idle_timeout: Duration::from_secs(300),
            hsm_timeout: Duration::from_secs(10),
        }
    }
}
