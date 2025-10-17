//! Cache Configuration
//!
//! Canonical cache configuration for managing in-memory caching and performance optimization.
//!
//! # Overview
//!
//! `CanonicalCacheConfig` provides comprehensive caching settings including:
//! - Cache enable/disable control
//! - Size limits and memory management
//! - TTL (Time To Live) for cache entries
//! - Eviction policies for cache management
//!
//! # Quick Start
//!
//! ```rust
//! use beardog_types::canonical::config::cache::{CanonicalCacheConfig, EvictionPolicy};
//! use std::time::Duration;
//!
//! // Create cache configuration
//! let config = CanonicalCacheConfig {
//!     enabled: true,
//!     max_size_mb: 512,  // 512 MB cache
//!     ttl: Duration::from_secs(3600),  // 1 hour
//!     eviction_policy: EvictionPolicy::Lru,
//! };
//!
//! assert!(config.enabled);
//! assert_eq!(config.max_size_mb, 512);
//! ```
//!
//! # Production Configuration
//!
//! ```rust
//! use beardog_types::canonical::config::cache::{CanonicalCacheConfig, EvictionPolicy};
//! use std::time::Duration;
//!
//! // High-performance production cache
//! let config = CanonicalCacheConfig {
//!     enabled: true,
//!     max_size_mb: 4096,  // 4 GB for production
//!     ttl: Duration::from_secs(1800),  // 30 minutes
//!     eviction_policy: EvictionPolicy::Lru,  // Most efficient
//! };
//! ```
//!
//! # Eviction Strategies
//!
//! ```rust
//! use beardog_types::canonical::config::cache::{CanonicalCacheConfig, EvictionPolicy};
//! use std::time::Duration;
//!
//! // LRU - Best for most use cases
//! let lru_config = CanonicalCacheConfig {
//!     enabled: true,
//!     max_size_mb: 1024,
//!     ttl: Duration::from_secs(3600),
//!     eviction_policy: EvictionPolicy::Lru,  // Least Recently Used
//! };
//!
//! // LFU - For frequently accessed data
//! let lfu_config = CanonicalCacheConfig {
//!     eviction_policy: EvictionPolicy::Lfu,  // Least Frequently Used
//!     ..lru_config
//! };
//! ```
//!
//! # Memory Management
//!
//! ```rust
//! use beardog_types::canonical::config::cache::{CanonicalCacheConfig, EvictionPolicy};
//! use std::time::Duration;
//!
//! // Calculate cache size based on available memory
//! let available_memory_mb = 16384;  // 16 GB
//! let cache_percentage = 0.25;  // Use 25% for cache
//! let cache_size = (available_memory_mb as f64 * cache_percentage) as u64;
//!
//! let config = CanonicalCacheConfig {
//!     enabled: true,
//!     max_size_mb: cache_size,  // 4 GB
//!     ttl: Duration::from_secs(3600),
//!     eviction_policy: EvictionPolicy::Lru,
//! };
//! ```
//!
//! # Performance Considerations
//!
//! - **LRU**: Best general-purpose eviction, O(1) access
//! - **LFU**: Better for hot data, but more complex
//! - **FIFO**: Simple but less efficient
//! - **Random**: Lowest overhead but unpredictable
//!
//! # TTL Best Practices
//!
//! - Short TTL (< 5 min): Frequently changing data
//! - Medium TTL (5-60 min): Semi-static data
//! - Long TTL (> 1 hour): Rarely changing data
//!
//! # Design Principles
//!
//! - **Flexibility**: Multiple eviction policies
//! - **Safety**: Size limits prevent memory exhaustion
//! - **Performance**: Fast access with appropriate eviction
//! - **Configurability**: All parameters tunable

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Canonical cache configuration
///
/// Comprehensive configuration for in-memory caching including size limits,
/// TTL, and eviction policies.
///
/// # Fields
///
/// * `enabled` - Whether caching is active
/// * `max_size_mb` - Maximum cache size in megabytes
/// * `ttl` - Time to live for cache entries (after this duration, entries expire)
/// * `eviction_policy` - Strategy for removing entries when cache is full
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::cache::{CanonicalCacheConfig, EvictionPolicy};
/// use std::time::Duration;
///
/// let config = CanonicalCacheConfig {
///     enabled: true,
///     max_size_mb: 1024,  // 1 GB
///     ttl: Duration::from_secs(3600),  // 1 hour
///     eviction_policy: EvictionPolicy::Lru,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalCacheConfig {
    /// Whether caching is enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Maximum cache size in megabytes
    /// Number of `max_size_mb`
    pub max_size_mb: u64,
    /// The ttl value
    pub ttl: Duration,
    /// Cache eviction policy
    /// The eviction policy value
    pub eviction_policy: EvictionPolicy,
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

pub type CacheConfig = CanonicalCacheConfig;

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
        };
        assert_eq!(config.max_size_mb, 4096);
    }

    #[test]
    fn test_eviction_policy_lru() {
        let policy = EvictionPolicy::Lru;
        assert!(matches!(policy, EvictionPolicy::Lru));
    }

    #[test]
    fn test_eviction_policy_lfu() {
        let policy = EvictionPolicy::Lfu;
        assert!(matches!(policy, EvictionPolicy::Lfu));
    }

    #[test]
    fn test_eviction_policy_fifo() {
        let policy = EvictionPolicy::Fifo;
        assert!(matches!(policy, EvictionPolicy::Fifo));
    }

    #[test]
    fn test_eviction_policy_random() {
        let policy = EvictionPolicy::Random;
        assert!(matches!(policy, EvictionPolicy::Random));
    }

    #[test]
    fn test_eviction_policy_default() {
        let policy = EvictionPolicy::default();
        assert!(matches!(policy, EvictionPolicy::Lru));
    }

    #[test]
    fn test_cache_ttl_short() {
        let config = CanonicalCacheConfig {
            ttl: Duration::from_secs(300), // 5 minutes
            ..Default::default()
        };
        assert_eq!(config.ttl, Duration::from_secs(300));
    }

    #[test]
    fn test_cache_ttl_long() {
        let config = CanonicalCacheConfig {
            ttl: Duration::from_secs(7200), // 2 hours
            ..Default::default()
        };
        assert_eq!(config.ttl, Duration::from_secs(7200));
    }

    #[test]
    fn test_type_alias() {
        let _config: CacheConfig = CanonicalCacheConfig::default();
    }
}
