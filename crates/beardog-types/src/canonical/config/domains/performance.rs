// SPDX-License-Identifier: AGPL-3.0-only

//! Performance Domain Configuration

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Performance domain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceDomainConfig {
    /// Cache settings
    pub cache: CacheConfig,
    /// Thread pool settings
    pub threading: ThreadPoolConfig,
    /// Resource limits
    pub limits: ResourceLimits,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Enable caching
    pub enabled: bool,
    /// Cache size (number of entries)
    pub size: usize,
    /// Cache TTL
    pub ttl: Duration,
}

/// Thread pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadPoolConfig {
    /// Core thread count
    pub core_threads: usize,
    /// Maximum thread count
    pub max_threads: usize,
    /// Thread keep-alive time
    pub keep_alive: Duration,
}

/// Resource limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum memory usage (MB)
    pub max_memory_mb: Option<u64>,
    /// Maximum CPU usage (percentage)
    pub max_cpu_percent: Option<f64>,
    /// Request timeout
    pub request_timeout: Duration,
}

impl Default for PerformanceDomainConfig {
    fn default() -> Self {
        Self {
            cache: CacheConfig::default(),
            threading: ThreadPoolConfig::default(),
            limits: ResourceLimits::default(),
        }
    }
}

// Implement CacheStrategy trait for performance domain cache configuration
impl CacheStrategy for CacheConfig {
    fn max_entries(&self) -> usize {
        if !self.enabled {
            return 0;
        }
        self.size
    }

    fn ttl(&self) -> Duration {
        self.ttl
    }

    fn eviction_policy(&self) -> crate::canonical::traits::cache::EvictionPolicy {
        // Performance domain uses simple LRU
        crate::canonical::traits::cache::EvictionPolicy::Lru
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn validate(&self) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }
        if self.size == 0 {
            return Err("Cache size must be > 0 when enabled".to_string());
        }
        if self.ttl.is_zero() {
            return Err("TTL cannot be zero".to_string());
        }
        Ok(())
    }

    fn is_production_ready(&self) -> bool {
        use crate::constants::domains::validation::{MIN_CACHE_SIZE, MAX_PERFORMANCE_TTL_SECS};
        
        if !self.enabled {
            return true;
        }
        self.size >= MIN_CACHE_SIZE &&
        self.size <= 1_000_000 &&
        self.ttl >= Duration::from_secs(60) &&
        self.ttl <= Duration::from_secs(MAX_PERFORMANCE_TTL_SECS) &&
        self.validate().is_ok()
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            size: std::env::var("BEARDOG_CACHE_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10000), // 10K entries default
            ttl: Duration::from_secs(
                std::env::var("BEARDOG_CACHE_TTL")
                    .ok()
                    .and_then(|t| t.parse().ok())
                    .unwrap_or(3600) // 1 hour default
            ),
        }
    }
}

impl Default for ThreadPoolConfig {
    fn default() -> Self {
        let cores = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        Self {
            core_threads: cores,
            max_threads: cores * 2,
            keep_alive: Duration::from_secs(
                std::env::var("BEARDOG_PERFORMANCE_KEEP_ALIVE_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60)
            ),
        }
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_mb: None,
            max_cpu_percent: None,
            request_timeout: Duration::from_secs(
                std::env::var("BEARDOG_PERFORMANCE_REQUEST_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30)
            ),
        }
    }
}

impl PerformanceDomainConfig {
    /// Load from environment variables
    pub fn from_env() -> Result<Self, BearDogError> {
        let config = Self::default();
        Ok(config)
    }
    
    /// Validate configuration
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.threading.core_threads == 0 {
            return Err(BearDogError::validation("Core threads must be greater than 0"));
        }
        Ok(())
    }
} 