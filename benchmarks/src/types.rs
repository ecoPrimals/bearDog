// SPDX-License-Identifier: AGPL-3.0-or-later


use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(u32,
    pub burst_size: u32,
    pub window: Duration,
}

impl Default for RateLimitConfig {
    fn default(1000,
            burst_size: 100,
            window: Duration::from_secs(usize,
    pub max_buffer_size: usize,
    pub enable_zero_copy: bool,
}

impl Default for MemoryOptimizationConfig {
    fn default(1024,
            max_buffer_size: 64 * 1024,
            enable_zero_copy: true,
        }
    }
}

impl MemoryOptimizationConfig {
    #[must_use] pub const fn production(4096,
            max_buffer_size: 1024 * 1024,
            enable_zero_copy: true,
        }
    }

    #[must_use] pub const fn development(512,
            max_buffer_size: 32 * 1024,
            enable_zero_copy: false,
        }
    }
}

#[derive(usize,
    pub task_queue_size: usize,
    pub enable_work_stealing: bool,
}

impl Default for AsyncOptimizationConfig {
    fn default(100,
            task_queue_size: 1000,
            enable_work_stealing: true,
        }
    }
}

impl AsyncOptimizationConfig {
    #[must_use] pub const fn production(1000,
            task_queue_size: 10000,
            enable_work_stealing: true,
        }
    }

    #[must_use] pub const fn development(50,
            task_queue_size: 500,
            enable_work_stealing: false,
        }
    }
}

#[derive(u32,
    pub connection_timeout: Duration,
    pub query_timeout: Duration,
}

impl Default for OptimizedDatabaseConfig {
    fn default(10,
            connection_timeout: Duration::from_secs(30),
            query_timeout: Duration::from_secs(bool,
    pub allocations: u32,
    pub response_size_bytes: u64,
    pub zero_copy_ops: u32,
}
