// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Number of l1_max_entries
    pub l1_max_entries: usize,
    /// Number of l2_max_size_mb
    pub l2_max_size_mb: usize,
    /// Number of l3_max_size_gb
    pub l3_max_size_gb: usize,
    /// The default ttl value
    pub default_ttl: Duration,
    /// Whether enable_compression is enabled
    pub enable_compression: bool,
    /// The eviction strategy value
    pub eviction_strategy: EvictionStrategy,
    /// The warming strategy value
    pub warming_strategy: WarmingStrategy,
    /// Whether enable_statistics is enabled
    pub enable_statistics: bool,
}

/// Cache eviction strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvictionStrategy {
    /// Represents l r u variant
    LRU,    // Least Recently Used
    /// Represents l f u variant
    LFU,    // Least Frequently Used
    /// Represents f i f o variant
    FIFO,   // First In First Out
    /// Represents random variant
    Random, // Random eviction
    /// Represents t t l variant
    TTL,    // Time To Live based
}

/// Cache warming strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WarmingStrategy {
    /// Represents lazy variant
    Lazy,       // Load on demand
    /// Represents eager variant
    Eager,      // Pre-load frequently accessed items
    /// State indicating scheduled
    Scheduled,  // Load at scheduled intervals
    /// Represents predictive variant
    Predictive, // AI-driven predictive loading
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            l1_max_entries: 10_000,
            l2_max_size_mb: 100,
            l3_max_size_gb: 1,
            default_ttl: Duration::from_secs(
                std::env::var("BEARDOG_CACHE_DEFAULT_TTL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(3600)
            ),
            enable_compression: true,
            eviction_strategy: EvictionStrategy::LRU,
            warming_strategy: WarmingStrategy::Lazy,
            enable_statistics: true,
        }
    }
}

impl CacheConfig {
    /// Create new cache configuration with custom settings
    /// Creates a new instance
    pub fn new(l1_entries: usize, l2_size_mb: usize, l3_size_gb: usize, ttl: Duration) -> Self {
        Self {
            l1_max_entries: l1_entries,
            l2_max_size_mb: l2_size_mb,
            l3_max_size_gb: l3_size_gb,
            default_ttl: ttl,
            enable_compression: true,
            eviction_strategy: EvictionStrategy::LRU,
            warming_strategy: WarmingStrategy::Lazy,
            enable_statistics: true,
        }
    }

    pub fn high_performance() -> Self {
        Self {
            l1_max_entries: 50_000,
            l2_max_size_mb: 500,
            l3_max_size_gb: 5,
            default_ttl: Duration::from_secs(7200), // 2 hours
            enable_compression: false,              // Disable for performance
            eviction_strategy: EvictionStrategy::LFU,
            warming_strategy: WarmingStrategy::Predictive,
            enable_statistics: true,
        }
    }

    /// Create memory-optimized cache configuration
    pub fn memory_optimized() -> Self {
        Self {
            l1_max_entries: 5_000,
            l2_max_size_mb: 50,
            l3_max_size_gb: 1,
            default_ttl: Duration::from_secs(1800), // 30 minutes
            enable_compression: true,
            eviction_strategy: EvictionStrategy::LRU,
            warming_strategy: WarmingStrategy::Lazy,
            enable_statistics: false, // Disable to save memory
        }
    }

    /// Validate cache configuration
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), String> {
        if self.l1_max_entries == 0 {
            return Err("L1 max entries must be greater than 0".to_string());
        }
        if self.l2_max_size_mb == 0 {
            return Err("L2 max size must be greater than 0".to_string());
        }
        if self.l3_max_size_gb == 0 {
            return Err("L3 max size must be greater than 0".to_string());
        }
        if self.default_ttl.as_secs() == 0 {
            return Err("Default TTL must be greater than 0".to_string());
        }
        Ok(())
    }

    /// Get total memory usage estimate in MB
    pub fn estimated_memory_usage_mb(&self) -> usize {
        let l1_estimate = self.l1_max_entries * 64 / 1024 / 1024; // Assume 64 bytes per entry
        let l2_estimate = self.l2_max_size_mb;
        let l3_estimate = self.l3_max_size_gb * 1024;

        l1_estimate + l2_estimate + l3_estimate
    }

    /// Set eviction strategy
    /// Creates instance with eviction strategy
    pub fn with_eviction_strategy(mut self, strategy: EvictionStrategy) -> Self {
        self.eviction_strategy = strategy;
        self
    }

    /// Set warming strategy
    /// Creates instance with warming strategy
    pub fn with_warming_strategy(mut self, strategy: WarmingStrategy) -> Self {
        self.warming_strategy = strategy;
        self
    }

    /// Enable or disable compression
    /// Creates instance with compression
    pub fn with_compression(mut self, enable: bool) -> Self {
        self.enable_compression = enable;
        self
    }

    /// Enable or disable statistics
    /// Creates instance with statistics
    pub fn with_statistics(mut self, enable: bool) -> Self {
        self.enable_statistics = enable;
        self
    }
}

impl std::fmt::Display for EvictionStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvictionStrategy::LRU => write!(f, "Least Recently Used"),
            EvictionStrategy::LFU => write!(f, "Least Frequently Used"),
            EvictionStrategy::FIFO => write!(f, "First In First Out"),
            EvictionStrategy::Random => write!(f, "Random"),
            EvictionStrategy::TTL => write!(f, "Time To Live"),
        }
    }
}

impl std::fmt::Display for WarmingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WarmingStrategy::Lazy => write!(f, "Lazy Loading"),
            WarmingStrategy::Eager => write!(f, "Eager Loading"),
            WarmingStrategy::Scheduled => write!(f, "Scheduled Loading"),
            WarmingStrategy::Predictive => write!(f, "Predictive Loading"),
        }
    }
}

#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = CacheConfig::default();
        assert_eq!(config.l1_max_entries, 10_000);
        assert_eq!(config.l2_max_size_mb, 100);
        assert_eq!(config.l3_max_size_gb, 1);
        assert!(config.enable_compression);
        assert!(config.enable_statistics);
    }

    #[test]
    fn test_high_performance_config() {
        let config = CacheConfig::high_performance();
        assert_eq!(config.l1_max_entries, 50_000);
        assert!(!config.enable_compression);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert_eq!(config.eviction_strategy, EvictionStrategy::LFU);
    }

    #[test]
    fn test_memory_optimized_config() {
        let config = CacheConfig::memory_optimized();
        assert_eq!(config.l1_max_entries, 5_000);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert!(config.enable_compression);
        assert!(!config.enable_statistics);
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    fn test_config_validation() {
        let config = CacheConfig::default();
        assert!(config.validate().is_ok());

        let invalid_config = CacheConfig {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            l1_max_entries: 0,
            ..Default::default()
        };
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_memory_usage_estimation() {
        let config = CacheConfig::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let estimated = config.estimated_memory_usage_mb();
        assert!(estimated > 0);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_builder_pattern() {
        let config = CacheConfig::default()
            .with_eviction_strategy(EvictionStrategy::FIFO)
            .with_warming_strategy(WarmingStrategy::Eager)
            .with_compression(false)
            .with_statistics(false);

        assert_eq!(config.eviction_strategy, EvictionStrategy::FIFO);
        assert_eq!(config.warming_strategy, WarmingStrategy::Eager);
        assert!(!config.enable_compression);
        assert!(!config.enable_statistics);
    }
}
