// SPDX-License-Identifier: AGPL-3.0-or-later

//! Performance tuning: connection pools, caching, timeouts, and retries.

use serde::{Deserialize, Serialize};

use crate::canonical::config::domains::network::ConnectionPoolConfig;
use crate::canonical::traits::RetryStrategy;

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfiguration {
    /// Connection pool settings
    pub connection_pool: ConnectionPoolConfig,
    
    /// Caching configuration
    pub caching: CachingConfiguration,
    
    /// Timeout settings
    pub timeouts: TimeoutConfiguration,
    
    /// Retry configuration
    pub retry: RetryConfiguration,
}

/// Caching configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachingConfiguration {
    /// Enable caching
    pub enabled: bool,
    
    /// Cache size limit
    pub max_size: u64,
    
    /// Cache TTL in seconds
    pub ttl: u64,
    
    /// Cache eviction policy
    pub eviction_policy: EvictionPolicy,
}

/// Cache eviction policies
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EvictionPolicy {
    Lru,
    Lfu,
    Fifo,
    Random,
    Custom(String),
}

/// Timeout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutConfiguration {
    /// Request timeout in seconds
    pub request_timeout: u64,
    
    /// Connection timeout in seconds
    pub connection_timeout: u64,
    
    /// Read timeout in seconds
    pub read_timeout: u64,
    
    /// Write timeout in seconds
    pub write_timeout: u64,
}

/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfiguration {
    /// Maximum number of retries
    pub max_retries: u32,
    
    /// Base delay between retries in milliseconds
    pub base_delay_ms: u64,
    
    /// Maximum delay between retries in milliseconds
    pub max_delay_ms: u64,
    
    /// Backoff strategy
    pub backoff_strategy: BackoffStrategy,
}

/// Backoff strategies for retries
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BackoffStrategy {
    Fixed,
    Linear,
    Exponential,
    Jittered,
    Custom(String),
}

// Implement RetryStrategy trait for RetryConfiguration
impl RetryStrategy for RetryConfiguration {
    fn max_attempts(&self) -> u32 {
        self.max_retries.max(1) // Ensure at least 1 attempt
    }

    fn delay_for_attempt(&self, attempt: u32) -> std::time::Duration {
        if attempt == 0 {
            return std::time::Duration::ZERO;
        }

        let base_delay = std::time::Duration::from_millis(self.base_delay_ms);
        let max_delay = std::time::Duration::from_millis(self.max_delay_ms);

        let calculated_delay = match self.backoff_strategy {
            BackoffStrategy::Fixed => base_delay,
            BackoffStrategy::Linear => {
                let delay_ms = self.base_delay_ms * (attempt as u64);
                std::time::Duration::from_millis(delay_ms)
            }
            BackoffStrategy::Exponential => {
                let delay_ms = (self.base_delay_ms as f64 * 2.0_f64.powi((attempt - 1) as i32)) as u64;
                std::time::Duration::from_millis(delay_ms)
            }
            BackoffStrategy::Jittered => {
                // Jittered exponential backoff with deterministic jitter
                let base = (self.base_delay_ms as f64 * 2.0_f64.powi((attempt - 1) as i32)) as u64;
                let jitter_percent = (attempt % 10) as f64 * 0.01; // 0-10% variation
                let jitter_factor = 0.95 + jitter_percent; // 95-105%
                std::time::Duration::from_millis((base as f64 * jitter_factor) as u64)
            }
            BackoffStrategy::Custom(_) => {
                // Fallback to exponential for custom strategies
                let delay_ms = (self.base_delay_ms as f64 * 2.0_f64.powi((attempt - 1) as i32)) as u64;
                std::time::Duration::from_millis(delay_ms)
            }
        };

        // Cap at max_delay
        calculated_delay.min(max_delay)
    }

    fn backoff_multiplier(&self) -> f64 {
        match self.backoff_strategy {
            BackoffStrategy::Exponential | BackoffStrategy::Jittered => 2.0,
            BackoffStrategy::Linear => 1.0,
            BackoffStrategy::Fixed => 1.0,
            BackoffStrategy::Custom(_) => 2.0,
        }
    }
}
