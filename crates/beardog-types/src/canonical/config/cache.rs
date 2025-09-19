// Canonical Cache Configuration

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Canonical cache configuration
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum EvictionPolicy {
    /// Least Recently Used eviction (default)
    #[default]
    /// Represents lru variant
    Lru,
    /// Least Frequently Used eviction
    Lfu,
    /// First In, First Out eviction
    Fifo,
    /// Random eviction
    Random,
}

pub type CacheConfig = CanonicalCacheConfig;
