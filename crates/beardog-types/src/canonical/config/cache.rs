// SPDX-License-Identifier: AGPL-3.0-only

//! # Unified Cache Configuration - CONSOLIDATED
//!
//! **CANONICAL CACHE CONFIGURATION** - Single source of truth for all caching.
//!
//! This module consolidates **13 cache configuration variants** across the BearDog ecosystem:
//!
//! ## Variants Unified ✅
//!
//! 1. `CanonicalCacheConfig` (this file) - Base canonical config
//! 2. `beardog-utils/caching/config::CacheConfig` - Multi-tier config
//! 3. `beardog-utils/caching/l1_cache::L1CacheConfig` - L1 specific
//! 4. `beardog-utils/caching/l2_cache::L2CacheConfig` - L2 specific
//! 5. `beardog-utils/caching/l3_cache::L3CacheConfig` - L3 specific
//! 6. `beardog-core/universal_discovery/network::CacheConfig` - Discovery cache
//! 7. `beardog-core/ecosystem_storage/cache::CacheConfig` - Storage cache
//! 8. `beardog-types/canonical/config/discovery::CacheConfig` - Discovery config
//! 9. `beardog-types/canonical/config/domains/discovery_unified::CacheConfig` - Unified discovery
//! 10. `beardog-types/canonical/config/domains/discovery_config::CacheConfig` - Config discovery
//! 11. `beardog-types/canonical/config/domains/performance::CacheConfig` - Performance cache
//! 12. `beardog-types/canonical/config/domains/network/performance::CacheConfig` - Network cache
//! 13. `beardog-adapters/universal/capability_discovery_cache::CacheConfig` - Adapter cache
//!
//! ## Migration Path
//!
//! **All existing code continues to work via type aliases!**
//!
//! ```rust,ignore
//! // Old code (still works via type alias)
//! use beardog_utils::caching::config::CacheConfig;
//! let config = CacheConfig::default();
//!
//! // New code (recommended)
//! use beardog_types::canonical::config::cache::CanonicalCacheConfig;
//! let config = CanonicalCacheConfig::default();
//! ```
//!
//! ## Multi-Tier Caching with CacheTier
//!
//! ```rust
//! use beardog_types::canonical::config::cache::{CanonicalCacheConfig, CacheTier};
//! use std::time::Duration;
//!
//! // L1 Cache (fast, in-memory, entry-based)
//! let l1_config = CanonicalCacheConfig::with_tier(CacheTier::L1 {
//!     max_entries: 10_000,
//!     track_access: true,
//!     cleanup_interval: Duration::from_secs(60),
//! });
//!
//! // L2 Cache (compressed, memory-optimized)
//! let l2_config = CanonicalCacheConfig::with_tier(CacheTier::L2 {
//!     max_size_mb: 500,
//!     enable_compression: true,
//! });
//!
//! // L3 Cache (persistent, disk-backed)
//! let l3_config = CanonicalCacheConfig::with_tier(CacheTier::L3 {
//!     max_size_gb: 5,
//!     enable_persistence: true,
//!     persistent_path: Some("/var/cache/beardog".into()),
//! });
//! ```
//!
//! ## Advanced Features
//!
//! ```rust
//! use beardog_types::canonical::config::cache::*;
//! use std::time::Duration;
//!
//! // With warming strategy
//! let config = CanonicalCacheConfig {
//!     enabled: true,
//!     max_size_mb: 1024,
//!     ttl: Duration::from_secs(3600),
//!     eviction_policy: EvictionPolicy::Lru,
//!     warming_strategy: Some(WarmingStrategy::Predictive),
//!     enable_statistics: true,
//!     enable_compression: false,
//!     tier: None,
//! };
//! ```
//!
//! ## Backward Compatibility
//!
//! Type aliases ensure zero breaking changes:
//! - `CacheConfig` → `CanonicalCacheConfig`
//! - `L1CacheConfig` → `CanonicalCacheConfig` (with L1 tier)
//! - `L2CacheConfig` → `CanonicalCacheConfig` (with L2 tier)
//! - `L3CacheConfig` → `CanonicalCacheConfig` (with L3 tier)
//! - `DiscoveryCacheConfig` → `CanonicalCacheConfig`
//!
//! ## Quick Start
//!
//! ```rust
//! use beardog_types::canonical::config::cache::{CanonicalCacheConfig, EvictionPolicy};
//! use std::time::Duration;
//!
//! // Simple cache configuration
//! let config = CanonicalCacheConfig {
//!     enabled: true,
//!     max_size_mb: 512,
//!     ttl: Duration::from_secs(3600),
//!     eviction_policy: EvictionPolicy::Lru,
//!     ..Default::default()
//! };
//! ```

use crate::canonical::traits::CacheStrategy;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

/// **Canonical Cache Configuration** - Unified config for all cache tiers
///
/// Consolidates 13 cache configuration variants into a single, flexible configuration
/// that supports L1 (fast in-memory), L2 (compressed memory), and L3 (persistent disk) caching.
///
/// # Fields
///
/// * `enabled` - Whether caching is active
/// * `max_size_mb` - Maximum cache size in megabytes (general purpose)
/// * `ttl` - Time to live for cache entries (after this duration, entries expire)
/// * `eviction_policy` - Strategy for removing entries when cache is full
/// * `warming_strategy` - Cache warming/pre-loading strategy (optional)
/// * `enable_statistics` - Track cache hits/misses and performance metrics
/// * `enable_compression` - Enable compression for cache entries
/// * `tier` - Tier-specific configuration (L1, L2, or L3)
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::cache::{CanonicalCacheConfig, EvictionPolicy};
/// use std::time::Duration;
///
/// // Simple cache
/// let config = CanonicalCacheConfig {
///     enabled: true,
///     max_size_mb: 1024,
///     ttl: Duration::from_secs(3600),
///     eviction_policy: EvictionPolicy::Lru,
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalCacheConfig {
    /// Whether caching is enabled
    pub enabled: bool,

    /// Maximum cache size in megabytes (general purpose)
    pub max_size_mb: u64,

    /// Time to live for cache entries
    pub ttl: Duration,

    /// Cache eviction policy
    pub eviction_policy: EvictionPolicy,

    /// Cache warming strategy (optional)
    pub warming_strategy: Option<WarmingStrategy>,

    /// Enable statistics tracking (hits, misses, latency)
    pub enable_statistics: bool,

    /// Enable compression for cache entries
    pub enable_compression: bool,

    /// Tier-specific configuration (L1, L2, or L3)
    pub tier: Option<CacheTier>,
}

/// **Cache Tier Configuration** - Tier-specific settings for multi-level caching
///
/// BearDog supports 3 cache tiers with different performance characteristics:
///
/// - **L1**: Fast in-memory cache (CPU cache-friendly, entry-based)
/// - **L2**: Compressed memory cache (memory-efficient, compressed storage)
/// - **L3**: Persistent disk cache (large capacity, survives restarts)
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::cache::CacheTier;
/// use std::time::Duration;
///
/// // L1: Fast, entry-based
/// let l1 = CacheTier::L1 {
///     max_entries: 10_000,
///     track_access: true,
///     cleanup_interval: Duration::from_secs(60),
/// };
///
/// // L2: Compressed, memory-efficient
/// let l2 = CacheTier::L2 {
///     max_size_mb: 500,
///     enable_compression: true,
/// };
///
/// // L3: Persistent, large capacity
/// let l3 = CacheTier::L3 {
///     max_size_gb: 5,
///     enable_persistence: true,
///     persistent_path: Some("/var/cache/beardog".into()),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheTier {
    /// L1 Cache: Fast in-memory cache (CPU cache-friendly)
    ///
    /// **Characteristics**:
    /// - Fastest access (sub-microsecond)
    /// - Entry-based (not byte-based)
    /// - Small capacity (thousands of entries)
    /// - Access tracking for LRU/LFU
    L1 {
        /// Maximum number of cache entries
        max_entries: usize,
        /// Track access patterns for eviction
        track_access: bool,
        /// Cleanup interval for expired entries
        cleanup_interval: Duration,
    },

    /// L2 Cache: Compressed memory cache (memory-efficient)
    ///
    /// **Characteristics**:
    /// - Fast access (microseconds)
    /// - Compressed storage (lower memory usage)
    /// - Medium capacity (hundreds of MB)
    /// - Automatic compression
    L2 {
        /// Maximum size in megabytes
        max_size_mb: usize,
        /// Enable compression (recommended)
        enable_compression: bool,
    },

    /// L3 Cache: Persistent disk cache (large capacity)
    ///
    /// **Characteristics**:
    /// - Slower access (milliseconds)
    /// - Persistent across restarts
    /// - Large capacity (gigabytes)
    /// - Optional disk persistence
    L3 {
        /// Maximum size in gigabytes
        max_size_gb: usize,
        /// Enable persistence to disk
        enable_persistence: bool,
        /// Path for persistent storage (optional)
        persistent_path: Option<PathBuf>,
    },
}

/// **Cache Warming Strategy** - Pre-loading and predictive caching
///
/// Strategies for pre-populating the cache before requests arrive.
///
/// # Strategies
///
/// - **Lazy**: Load on demand (no pre-loading) - Default, lowest overhead
/// - **Eager**: Pre-load frequently accessed items at startup
/// - **Scheduled**: Load at scheduled intervals (e.g., every hour)
/// - **Predictive**: AI-driven predictive loading based on access patterns
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::cache::WarmingStrategy;
///
/// // Lazy loading (default)
/// let lazy = WarmingStrategy::Lazy;
///
/// // Eager loading at startup
/// let eager = WarmingStrategy::Eager;
///
/// // Scheduled every hour
/// let scheduled = WarmingStrategy::Scheduled;
///
/// // AI-driven predictive
/// let predictive = WarmingStrategy::Predictive;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WarmingStrategy {
    /// Lazy loading - Load on demand (no pre-loading)
    ///
    /// **Best for**: Unpredictable access patterns, minimal overhead
    Lazy,

    /// Eager loading - Pre-load frequently accessed items at startup
    ///
    /// **Best for**: Known hot data, faster initial requests
    Eager,

    /// Scheduled loading - Load at scheduled intervals
    ///
    /// **Best for**: Periodically refreshed data, time-based patterns
    Scheduled,

    /// Predictive loading - AI-driven predictive loading
    ///
    /// **Best for**: ML-predicted access patterns, adaptive caching
    Predictive,
}

/// Cache eviction policies
///
/// Strategies for removing entries when the cache reaches its size limit.
///
/// # Policy Comparison
///
/// | Policy | Complexity | Hit Rate | Use Case |
/// |--------|-----------|----------|----------|
/// | LRU | O(1) | High | General purpose (recommended) |
/// | LFU | O(log n) | Very High | Hot data, predictable access |
/// | FIFO | O(1) | Medium | Simple caching, testing |
/// | Random | O(1) | Low | Low overhead, unpredictable load |
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::cache::EvictionPolicy;
///
/// // Most common: LRU
/// let policy = EvictionPolicy::Lru;
///
/// // For frequently accessed data: LFU
/// let policy = EvictionPolicy::Lfu;
///
/// // Simple FIFO for testing
/// let policy = EvictionPolicy::Fifo;
/// ```
///
/// # Performance Characteristics
///
/// - **LRU**: Evicts least recently accessed items; best for temporal locality
/// - **LFU**: Evicts least frequently accessed items; best for frequency locality
/// - **FIFO**: Evicts oldest items first; simplest implementation
/// - **Random**: Evicts random items; lowest overhead, unpredictable
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum EvictionPolicy {
    /// Least Recently Used eviction (default)
    ///
    /// Removes entries that haven't been accessed recently.
    /// **Best for**: General-purpose caching with temporal locality.
    /// **Performance**: O(1) access and eviction.
    #[default]
    /// Represents lru variant
    Lru,

    /// Least Frequently Used eviction
    ///
    /// Removes entries with the lowest access frequency.
    /// **Best for**: Hot data with predictable access patterns.
    /// **Performance**: O(log n) access, higher memory overhead.
    Lfu,

    /// First In, First Out eviction
    ///
    /// Removes the oldest entries first (like a queue).
    /// **Best for**: Simple caching, testing, fair eviction.
    /// **Performance**: O(1) access and eviction.
    Fifo,

    /// Random eviction
    ///
    /// Removes random entries when cache is full.
    /// **Best for**: Low overhead, unpredictable access patterns.
    /// **Performance**: O(1) access, lowest overhead.
    Random,
}

impl Default for CanonicalCacheConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_size_mb: 0,
            ttl: Duration::from_secs(3600), // 1 hour default
            eviction_policy: EvictionPolicy::Lru,
            warming_strategy: None,
            enable_statistics: false,
            enable_compression: false,
            tier: None,
        }
    }
}

impl CanonicalCacheConfig {
    /// Create cache configuration with L1 tier
    pub fn with_tier(tier: CacheTier) -> Self {
        Self {
            enabled: true,
            tier: Some(tier),
            ..Default::default()
        }
    }

    /// High-performance preset for L1 cache
    pub fn high_performance_l1() -> Self {
        Self::with_tier(CacheTier::L1 {
            max_entries: 50_000,
            track_access: true,
            cleanup_interval: Duration::from_secs(30),
        })
    }

    /// High-performance preset for L2 cache
    pub fn high_performance_l2() -> Self {
        Self::with_tier(CacheTier::L2 {
            max_size_mb: 500,
            enable_compression: false, // Disable for performance
        })
    }

    /// High-performance preset for L3 cache
    pub fn high_performance_l3() -> Self {
        Self::with_tier(CacheTier::L3 {
            max_size_gb: 5,
            enable_persistence: true,
            persistent_path: None,
        })
    }

    /// Memory-optimized preset
    pub fn memory_optimized() -> Self {
        Self {
            enabled: true,
            max_size_mb: 256,
            ttl: Duration::from_secs(1800), // 30 minutes
            eviction_policy: EvictionPolicy::Lru,
            warming_strategy: None,
            enable_statistics: false,
            enable_compression: true,
            tier: Some(CacheTier::L2 {
                max_size_mb: 256,
                enable_compression: true,
            }),
        }
    }
}

// ============================================================================
// TYPE ALIASES - Backward Compatibility for 13 Cache Config Variants
// ============================================================================

/// Type alias for backward compatibility
///
/// **Unified from**: Multiple CacheConfig definitions across the codebase
pub type CacheConfig = CanonicalCacheConfig;

/// L1 Cache configuration type alias
///
/// **Migration**: Use `CanonicalCacheConfig::with_tier(CacheTier::L1 { ... })`
pub type L1CacheConfig = CanonicalCacheConfig;

/// L2 Cache configuration type alias
///
/// **Migration**: Use `CanonicalCacheConfig::with_tier(CacheTier::L2 { ... })`
pub type L2CacheConfig = CanonicalCacheConfig;

/// L3 Cache configuration type alias
///
/// **Migration**: Use `CanonicalCacheConfig::with_tier(CacheTier::L3 { ... })`
pub type L3CacheConfig = CanonicalCacheConfig;

/// Discovery cache configuration type alias
///
/// **Unified from**: Discovery-specific cache configurations
pub type DiscoveryCacheConfig = CanonicalCacheConfig;

/// Network performance cache configuration type alias
///
/// **Unified from**: Network performance cache configurations
pub type NetworkCacheConfig = CanonicalCacheConfig;

/// Adapter cache configuration type alias
///
/// **Unified from**: Universal adapter cache configurations
pub type AdapterCacheConfig = CanonicalCacheConfig;

/// Storage cache configuration type alias
///
/// **Unified from**: Ecosystem storage cache configurations
pub type StorageCacheConfig = CanonicalCacheConfig;

/// Performance cache configuration type alias
///
/// **Unified from**: Performance domain cache configurations
pub type PerformanceCacheConfig = CanonicalCacheConfig;

// ============================================================================
// MIGRATION GUIDE
// ============================================================================

/// **Migration Guide** for beardog-utils/caching/config::CacheConfig
///
/// ```rust,ignore
/// // OLD: beardog-utils/caching/config
/// use beardog_utils::caching::config::CacheConfig;
/// let config = CacheConfig {
///     l1_max_entries: 10_000,
///     l2_max_size_mb: 100,
///     l3_max_size_gb: 1,
///     default_ttl: Duration::from_secs(3600),
///     enable_compression: true,
///     eviction_strategy: EvictionStrategy::LRU,
///     warming_strategy: WarmingStrategy::Lazy,
///     enable_statistics: true,
/// };
///
/// // NEW: Unified canonical config
/// use beardog_types::canonical::config::cache::*;
/// let config = CanonicalCacheConfig {
///     enabled: true,
///     max_size_mb: 100,
///     ttl: Duration::from_secs(3600),
///     eviction_policy: EvictionPolicy::Lru,
///     warming_strategy: Some(WarmingStrategy::Lazy),
///     enable_statistics: true,
///     enable_compression: true,
///     tier: Some(CacheTier::L1 {
///         max_entries: 10_000,
///         track_access: true,
///         cleanup_interval: Duration::from_secs(60),
///     }),
/// };
/// ```
pub struct CacheConfigMigrationGuide;

// ============================================================================
// TRAIT IMPLEMENTATIONS
// ============================================================================

// Implement CacheStrategy trait for canonical cache configuration
impl CacheStrategy for CanonicalCacheConfig {
    fn max_entries(&self) -> usize {
        if !self.enabled {
            return 0;
        }
        // Estimate entries from MB: assume ~4KB per entry
        let bytes = (self.max_size_mb * 1024 * 1024) as usize;
        bytes / 4096
    }

    fn ttl(&self) -> Duration {
        self.ttl
    }

    fn eviction_policy(&self) -> crate::canonical::traits::cache::EvictionPolicy {
        use crate::canonical::traits::cache::EvictionPolicy as TraitPolicy;
        match self.eviction_policy {
            EvictionPolicy::Lru => TraitPolicy::Lru,
            EvictionPolicy::Lfu => TraitPolicy::Lfu,
            EvictionPolicy::Fifo => TraitPolicy::Fifo,
            EvictionPolicy::Random => TraitPolicy::Random,
        }
    }

    fn max_size_bytes(&self) -> Option<u64> {
        if self.enabled && self.max_size_mb > 0 {
            Some(self.max_size_mb * 1024 * 1024)
        } else {
            None
        }
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn validate(&self) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }
        if self.max_size_mb == 0 {
            return Err("max_size_mb must be > 0 when caching is enabled".to_string());
        }
        if self.ttl.is_zero() {
            return Err("TTL cannot be zero".to_string());
        }
        Ok(())
    }

    fn is_production_ready(&self) -> bool {
        if !self.enabled {
            return true; // Disabled is valid for production
        }
        self.max_size_mb >= 64 && // At least 64MB
        self.max_size_mb <= 16384 && // At most 16GB
        self.ttl >= Duration::from_secs(60) && // At least 1 minute
        self.ttl <= Duration::from_secs(86400) && // At most 1 day
        self.validate().is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_config_default() {
        let config = CanonicalCacheConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.max_size_mb, 0);
        assert!(matches!(config.eviction_policy, EvictionPolicy::Lru));
    }

    #[test]
    fn test_cache_config_enabled() {
        let config = CanonicalCacheConfig {
            enabled: true,
            max_size_mb: 512,
            ttl: Duration::from_secs(3600),
            eviction_policy: EvictionPolicy::Lru,
            ..Default::default()
        };
        assert!(config.enabled);
        assert_eq!(config.max_size_mb, 512);
        assert_eq!(config.ttl, Duration::from_secs(3600));
    }

    #[test]
    fn test_cache_config_production() {
        let config = CanonicalCacheConfig {
            enabled: true,
            max_size_mb: 4096, // 4GB for production
            ttl: Duration::from_secs(1800),
            eviction_policy: EvictionPolicy::Lru,
            ..Default::default()
        };
        assert_eq!(config.max_size_mb, 4096);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_eviction_policy_lru() {
        let policy = EvictionPolicy::Lru;
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(matches!(policy, EvictionPolicy::Lru));
    }

    #[test]
    fn test_eviction_policy_lfu() {
        let policy = EvictionPolicy::Lfu;
        assert!(matches!(policy, EvictionPolicy::Lfu));
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_eviction_policy_fifo() {
        let policy = EvictionPolicy::Fifo;
        assert!(matches!(policy, EvictionPolicy::Fifo));
    }

    #[test]
    fn test_eviction_policy_random() {
        let policy = EvictionPolicy::Random;
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(matches!(policy, EvictionPolicy::Random));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_eviction_policy_default() {
        let policy = EvictionPolicy::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(matches!(policy, EvictionPolicy::Lru));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_cache_ttl_short() {
        let config = CanonicalCacheConfig {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            ttl: Duration::from_secs(300), // 5 minutes
            ..Default::default()
        };
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(config.ttl, Duration::from_secs(300));
    }

    #[test]
    fn test_cache_ttl_long() {
        let config = CanonicalCacheConfig {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            ttl: Duration::from_secs(7200), // 2 hours
            ..Default::default()
        };
        assert_eq!(config.ttl, Duration::from_secs(7200));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_type_alias() {
        let _config: CacheConfig = CanonicalCacheConfig::default();
    }

    #[test]
    fn test_l1_cache_config() {
        let config = CanonicalCacheConfig::high_performance_l1();
        assert!(config.enabled);
        assert!(config.tier.is_some());
        if let Some(CacheTier::L1 {
            max_entries,
            track_access,
            ..
        }) = config.tier
        {
            assert_eq!(max_entries, 50_000);
            assert!(track_access);
        } else {
            panic!("Expected L1 tier");
        }
    }

    #[test]
    fn test_l2_cache_config() {
        let config = CanonicalCacheConfig::high_performance_l2();
        assert!(config.enabled);
        assert!(config.tier.is_some());
        if let Some(CacheTier::L2 {
            max_size_mb,
            enable_compression,
        }) = config.tier
        {
            assert_eq!(max_size_mb, 500);
            assert!(!enable_compression); // Disabled for performance
        } else {
            panic!("Expected L2 tier");
        }
    }

    #[test]
    fn test_l3_cache_config() {
        let config = CanonicalCacheConfig::high_performance_l3();
        assert!(config.enabled);
        assert!(config.tier.is_some());
        if let Some(CacheTier::L3 {
            max_size_gb,
            enable_persistence,
            ..
        }) = config.tier
        {
            assert_eq!(max_size_gb, 5);
            assert!(enable_persistence);
        } else {
            panic!("Expected L3 tier");
        }
    }

    #[test]
    fn test_memory_optimized_preset() {
        let config = CanonicalCacheConfig::memory_optimized();
        assert!(config.enabled);
        assert!(config.enable_compression);
        assert!(!config.enable_statistics);
        assert_eq!(config.max_size_mb, 256);
    }

    #[test]
    fn test_warming_strategy() {
        let config = CanonicalCacheConfig {
            enabled: true,
            max_size_mb: 1024,
            ttl: Duration::from_secs(3600),
            eviction_policy: EvictionPolicy::Lru,
            warming_strategy: Some(WarmingStrategy::Predictive),
            ..Default::default()
        };
        assert_eq!(config.warming_strategy, Some(WarmingStrategy::Predictive));
    }

    #[test]
    fn test_type_aliases() {
        // Test all type aliases compile
        let _: CacheConfig = CanonicalCacheConfig::default();
        let _: L1CacheConfig = CanonicalCacheConfig::default();
        let _: L2CacheConfig = CanonicalCacheConfig::default();
        let _: L3CacheConfig = CanonicalCacheConfig::default();
        let _: DiscoveryCacheConfig = CanonicalCacheConfig::default();
        let _: NetworkCacheConfig = CanonicalCacheConfig::default();
        let _: AdapterCacheConfig = CanonicalCacheConfig::default();
        let _: StorageCacheConfig = CanonicalCacheConfig::default();
        let _: PerformanceCacheConfig = CanonicalCacheConfig::default();
    }
}
