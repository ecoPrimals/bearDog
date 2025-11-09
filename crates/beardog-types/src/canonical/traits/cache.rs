//! # Cache Strategy Trait
//!
//! This module provides a polymorphic interface for cache configurations across different
//! domains (storage, providers, discovery, AI) while preserving their unique caching features.
//!
//! ## Design Rationale
//!
//! Rather than forcing all cache configs into a single struct, we provide a common
//! trait interface that enables:
//! - **Polymorphic cache management**: Functions that work with any cache config
//! - **Domain preservation**: Each config retains its domain-specific features
//! - **Type safety**: Compiler-enforced cache policies
//! - **Easy extension**: New cache configs just implement the trait
//!
//! ## Example Usage
//!
//! ```rust
//! use beardog_types::canonical::traits::{CacheStrategy, EvictionPolicy};
//! use std::time::Duration;
//!
//! fn setup_cache<C: CacheStrategy>(strategy: &C) {
//!     println!("Cache max entries: {}", strategy.max_entries());
//!     println!("TTL: {:?}", strategy.ttl());
//!     println!("Eviction: {:?}", strategy.eviction_policy());
//!     
//!     // Check if entry should be evicted
//!     let age = Duration::from_secs(3700);
//!     if strategy.should_evict(age, 10) {
//!         println!("Entry should be evicted");
//!     }
//! }
//! ```

use std::time::Duration;

/// Cache eviction policies
///
/// Determines which entries are removed when the cache reaches capacity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvictionPolicy {
    /// Least Recently Used - Evicts entries not accessed recently
    ///
    /// Best for: General purpose caching, temporal locality
    /// Performance: O(1) with proper implementation
    Lru,

    /// Least Frequently Used - Evicts entries accessed least often
    ///
    /// Best for: Data with consistent access patterns
    /// Performance: O(log n) typical
    Lfu,

    /// First In First Out - Evicts oldest entries first
    ///
    /// Best for: Simple queues, streaming data
    /// Performance: O(1)
    Fifo,

    /// Random - Evicts random entries
    ///
    /// Best for: Low overhead, no access pattern
    /// Performance: O(1)
    Random,

    /// Time To Live - Evicts based on entry age
    ///
    /// Best for: Time-sensitive data, sessions
    /// Performance: O(log n) with expiry tracking
    Ttl,
}

impl EvictionPolicy {
    /// Returns true if this policy tracks access patterns
    pub fn tracks_access(&self) -> bool {
        matches!(self, EvictionPolicy::Lru | EvictionPolicy::Lfu)
    }

    /// Returns true if this policy is time-based
    pub fn is_time_based(&self) -> bool {
        matches!(self, EvictionPolicy::Ttl)
    }

    /// Returns the relative computational overhead of this policy
    ///
    /// Returns a value from 1 (lowest) to 5 (highest)
    pub fn overhead_level(&self) -> u8 {
        match self {
            EvictionPolicy::Fifo | EvictionPolicy::Random => 1,
            EvictionPolicy::Lru => 2,
            EvictionPolicy::Ttl => 3,
            EvictionPolicy::Lfu => 4,
        }
    }
}

impl std::fmt::Display for EvictionPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvictionPolicy::Lru => write!(f, "LRU"),
            EvictionPolicy::Lfu => write!(f, "LFU"),
            EvictionPolicy::Fifo => write!(f, "FIFO"),
            EvictionPolicy::Random => write!(f, "Random"),
            EvictionPolicy::Ttl => write!(f, "TTL"),
        }
    }
}

/// Trait for cache strategies
///
/// Provides a common interface for cache settings across different domains while
/// allowing each implementation to maintain domain-specific caching features.
///
/// ## Cache Properties
///
/// - **Capacity**: Maximum number of entries or size
/// - **TTL**: Time-to-live for entries
/// - **Eviction**: Policy for removing entries when full
///
/// ## Thread Safety
///
/// The trait requires `Send + Sync` to enable use in async contexts and across threads.
pub trait CacheStrategy: Send + Sync {
    /// Maximum number of cache entries
    ///
    /// Returns the maximum number of entries the cache can hold.
    /// When this limit is reached, eviction occurs based on the eviction policy.
    ///
    /// ## Returns
    /// - Maximum entry count (>= 1)
    /// - `0` means caching is effectively disabled
    fn max_entries(&self) -> usize;

    /// Time-to-live for cache entries
    ///
    /// Returns how long entries remain valid before expiring.
    /// Expired entries are eligible for eviction.
    ///
    /// ## Returns
    /// - Duration entries remain valid
    /// - Very large values (e.g., days/years) mean entries rarely expire
    fn ttl(&self) -> Duration;

    /// Eviction policy when cache is full
    ///
    /// Returns the strategy used to determine which entries to remove
    /// when the cache reaches capacity.
    fn eviction_policy(&self) -> EvictionPolicy;

    /// Check if an entry should be evicted
    ///
    /// Determines if an entry should be removed based on its age and access count.
    ///
    /// ## Parameters
    /// - `age`: How long the entry has been in the cache
    /// - `access_count`: Number of times the entry has been accessed
    ///
    /// ## Default Implementation
    /// The default implementation evicts entries older than TTL.
    /// Implementations can override for more sophisticated policies.
    fn should_evict(&self, age: Duration, _access_count: usize) -> bool {
        age >= self.ttl()
    }

    /// Maximum cache size in bytes (optional)
    ///
    /// Returns the maximum memory the cache can use.
    /// Returns `None` if size-based limiting is not used.
    ///
    /// ## Use Cases
    /// - Memory-constrained systems
    /// - Large value caching (files, blobs)
    /// - Predictable memory usage
    fn max_size_bytes(&self) -> Option<u64> {
        None // Entry-based limiting by default
    }

    /// Whether caching is enabled
    ///
    /// Returns `false` if caching is disabled.
    /// All other methods may still return valid values for configuration purposes.
    fn is_enabled(&self) -> bool {
        true // Enabled by default
    }

    /// Calculate remaining capacity
    ///
    /// Returns how many more entries can be added before eviction occurs.
    ///
    /// ## Parameters
    /// - `current_entries`: Number of entries currently in cache
    ///
    /// ## Returns
    /// Remaining capacity, or 0 if at/over capacity
    fn remaining_capacity(&self, current_entries: usize) -> usize {
        self.max_entries().saturating_sub(current_entries)
    }

    /// Check if cache is at capacity
    ///
    /// Returns `true` if no more entries can be added without eviction.
    ///
    /// ## Parameters
    /// - `current_entries`: Number of entries currently in cache
    fn is_at_capacity(&self, current_entries: usize) -> bool {
        current_entries >= self.max_entries()
    }

    /// Calculate cache hit rate target
    ///
    /// Returns the target hit rate (0.0 to 1.0) for this cache strategy.
    /// Higher values indicate more aggressive caching.
    ///
    /// ## Returns
    /// Target hit rate: 0.0 (no caching) to 1.0 (always cached)
    fn target_hit_rate(&self) -> f64 {
        match self.eviction_policy() {
            EvictionPolicy::Lru | EvictionPolicy::Lfu => 0.8, // 80% for smart policies
            EvictionPolicy::Fifo | EvictionPolicy::Ttl => 0.7, // 70% for simpler policies
            EvictionPolicy::Random => 0.6, // 60% for random eviction
        }
    }

    /// Validate the cache configuration
    ///
    /// Checks that the configuration is valid and reasonable.
    /// Returns `Ok(())` if valid, or `Err(String)` with error description.
    ///
    /// ## Validation Rules
    /// - max_entries should be > 0 (unless disabled)
    /// - TTL should be reasonable (not too short, not absurdly long)
    /// - max_size_bytes (if set) should be reasonable
    fn validate(&self) -> Result<(), String> {
        // Check if caching is effectively disabled
        if !self.is_enabled() {
            return Ok(()); // Disabled cache is valid
        }

        // Check max entries
        if self.max_entries() == 0 {
            return Err("max_entries must be > 0 for enabled cache".to_string());
        }

        if self.max_entries() > 100_000_000 {
            eprintln!(
                "WARNING: Very large max_entries ({}), may cause memory issues",
                self.max_entries()
            );
        }

        // Check TTL
        let ttl = self.ttl();
        if ttl == Duration::ZERO {
            return Err("TTL cannot be zero (entries would expire immediately)".to_string());
        }

        if ttl < Duration::from_secs(1) {
            eprintln!("WARNING: Very short TTL ({:?}), cache may not be effective", ttl);
        }

        if ttl > Duration::from_secs(86400 * 365) {
            // > 1 year
            eprintln!(
                "WARNING: Very long TTL ({:?}), entries may never expire",
                ttl
            );
        }

        // Check max size if set
        if let Some(max_bytes) = self.max_size_bytes() {
            if max_bytes == 0 {
                return Err("max_size_bytes cannot be zero if set".to_string());
            }

            if max_bytes > 100 * 1024 * 1024 * 1024 {
                // > 100 GB
                eprintln!(
                    "WARNING: Very large max_size_bytes ({} bytes), ensure system has enough memory",
                    max_bytes
                );
            }
        }

        Ok(())
    }

    /// Returns true if the configuration is suitable for production
    ///
    /// Production configurations should have:
    /// - Reasonable entry limits
    /// - Appropriate TTL values
    /// - Efficient eviction policies
    fn is_production_ready(&self) -> bool {
        // Must be valid
        if self.validate().is_err() {
            return false;
        }

        // Must be enabled
        if !self.is_enabled() {
            return false;
        }

        // Reasonable entry count
        let entries = self.max_entries();
        if entries < 10 || entries > 10_000_000 {
            return false; // Too small or unreasonably large
        }

        // Reasonable TTL
        let ttl = self.ttl();
        if ttl < Duration::from_secs(10) || ttl > Duration::from_secs(86400 * 7) {
            return false; // Too short (<10s) or too long (>7 days)
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock cache strategy for testing
    struct MockCacheStrategy {
        enabled: bool,
        max_entries: usize,
        ttl: Duration,
        policy: EvictionPolicy,
        max_bytes: Option<u64>,
    }

    impl CacheStrategy for MockCacheStrategy {
        fn max_entries(&self) -> usize {
            self.max_entries
        }

        fn ttl(&self) -> Duration {
            self.ttl
        }

        fn eviction_policy(&self) -> EvictionPolicy {
            self.policy
        }

        fn is_enabled(&self) -> bool {
            self.enabled
        }

        fn max_size_bytes(&self) -> Option<u64> {
            self.max_bytes
        }
    }

    #[test]
    fn test_eviction_policy_properties() {
        assert!(EvictionPolicy::Lru.tracks_access());
        assert!(EvictionPolicy::Lfu.tracks_access());
        assert!(!EvictionPolicy::Fifo.tracks_access());
        assert!(!EvictionPolicy::Random.tracks_access());
        
        assert!(EvictionPolicy::Ttl.is_time_based());
        assert!(!EvictionPolicy::Lru.is_time_based());
    }

    #[test]
    fn test_eviction_policy_overhead() {
        assert_eq!(EvictionPolicy::Fifo.overhead_level(), 1);
        assert_eq!(EvictionPolicy::Random.overhead_level(), 1);
        assert_eq!(EvictionPolicy::Lru.overhead_level(), 2);
        assert_eq!(EvictionPolicy::Ttl.overhead_level(), 3);
        assert_eq!(EvictionPolicy::Lfu.overhead_level(), 4);
    }

    #[test]
    fn test_basic_strategy() {
        let strategy = MockCacheStrategy {
            enabled: true,
            max_entries: 1000,
            ttl: Duration::from_secs(3600),
            policy: EvictionPolicy::Lru,
            max_bytes: None,
        };

        assert!(strategy.is_enabled());
        assert_eq!(strategy.max_entries(), 1000);
        assert_eq!(strategy.ttl(), Duration::from_secs(3600));
        assert_eq!(strategy.eviction_policy(), EvictionPolicy::Lru);
        assert_eq!(strategy.max_size_bytes(), None);
    }

    #[test]
    fn test_should_evict() {
        let strategy = MockCacheStrategy {
            enabled: true,
            max_entries: 1000,
            ttl: Duration::from_secs(3600),
            policy: EvictionPolicy::Ttl,
            max_bytes: None,
        };

        // Not old enough
        assert!(!strategy.should_evict(Duration::from_secs(3599), 100));
        
        // Exactly at TTL
        assert!(strategy.should_evict(Duration::from_secs(3600), 100));
        
        // Older than TTL
        assert!(strategy.should_evict(Duration::from_secs(7200), 100));
    }

    #[test]
    fn test_capacity_calculations() {
        let strategy = MockCacheStrategy {
            enabled: true,
            max_entries: 1000,
            ttl: Duration::from_secs(3600),
            policy: EvictionPolicy::Lru,
            max_bytes: None,
        };

        assert_eq!(strategy.remaining_capacity(500), 500);
        assert_eq!(strategy.remaining_capacity(1000), 0);
        assert_eq!(strategy.remaining_capacity(1500), 0); // Saturating sub

        assert!(!strategy.is_at_capacity(500));
        assert!(strategy.is_at_capacity(1000));
        assert!(strategy.is_at_capacity(1500));
    }

    #[test]
    fn test_target_hit_rate() {
        let lru_strategy = MockCacheStrategy {
            enabled: true,
            max_entries: 1000,
            ttl: Duration::from_secs(3600),
            policy: EvictionPolicy::Lru,
            max_bytes: None,
        };
        assert_eq!(lru_strategy.target_hit_rate(), 0.8);

        let random_strategy = MockCacheStrategy {
            enabled: true,
            max_entries: 1000,
            ttl: Duration::from_secs(3600),
            policy: EvictionPolicy::Random,
            max_bytes: None,
        };
        assert_eq!(random_strategy.target_hit_rate(), 0.6);
    }

    #[test]
    fn test_validation() {
        // Valid strategy
        let valid = MockCacheStrategy {
            enabled: true,
            max_entries: 1000,
            ttl: Duration::from_secs(3600),
            policy: EvictionPolicy::Lru,
            max_bytes: Some(1024 * 1024 * 100), // 100 MB
        };
        assert!(valid.validate().is_ok());

        // Invalid: zero entries
        let invalid1 = MockCacheStrategy {
            enabled: true,
            max_entries: 0,
            ttl: Duration::from_secs(3600),
            policy: EvictionPolicy::Lru,
            max_bytes: None,
        };
        assert!(invalid1.validate().is_err());

        // Invalid: zero TTL
        let invalid2 = MockCacheStrategy {
            enabled: true,
            max_entries: 1000,
            ttl: Duration::ZERO,
            policy: EvictionPolicy::Lru,
            max_bytes: None,
        };
        assert!(invalid2.validate().is_err());

        // Valid: disabled cache with zero entries is OK
        let disabled = MockCacheStrategy {
            enabled: false,
            max_entries: 0,
            ttl: Duration::ZERO,
            policy: EvictionPolicy::Lru,
            max_bytes: None,
        };
        assert!(disabled.validate().is_ok());
    }

    #[test]
    fn test_production_ready() {
        // Production ready
        let prod = MockCacheStrategy {
            enabled: true,
            max_entries: 10000,
            ttl: Duration::from_secs(1800), // 30 minutes
            policy: EvictionPolicy::Lru,
            max_bytes: None,
        };
        assert!(prod.is_production_ready());

        // Not production ready: too small
        let not_prod1 = MockCacheStrategy {
            enabled: true,
            max_entries: 5,
            ttl: Duration::from_secs(1800),
            policy: EvictionPolicy::Lru,
            max_bytes: None,
        };
        assert!(!not_prod1.is_production_ready());

        // Not production ready: TTL too short
        let not_prod2 = MockCacheStrategy {
            enabled: true,
            max_entries: 10000,
            ttl: Duration::from_secs(5),
            policy: EvictionPolicy::Lru,
            max_bytes: None,
        };
        assert!(!not_prod2.is_production_ready());

        // Not production ready: disabled
        let not_prod3 = MockCacheStrategy {
            enabled: false,
            max_entries: 10000,
            ttl: Duration::from_secs(1800),
            policy: EvictionPolicy::Lru,
            max_bytes: None,
        };
        assert!(!not_prod3.is_production_ready());
    }
}

