// Cache Management Module

use super::types::{CacheEntry, CacheEvictionPolicy};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Cache manager
#[derive(Debug)]
pub struct CacheManager {
    /// Cache entries
    entries: HashMap<String, CacheEntry>,
    /// Cache configuration
    config: CacheConfig,
    /// Cache metrics
    metrics: CacheMetrics,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Maximum cache size in bytes
    /// Number of `max_size_bytes`
    pub max_size_bytes: u64,
    /// Maximum number of entries
    /// Number of `max_entries`
    pub max_entries: usize,
    /// Default TTL in seconds
    /// Number of `default_ttl_secs`
    pub default_ttl_secs: u64,
    /// Eviction policy
    /// The eviction policy value
    pub eviction_policy: CacheEvictionPolicy,
    /// Enable cache compression
    /// Whether `enable_compression` is enabled
    pub enable_compression: bool,
}

/// Cache metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetrics {
    /// Total cache hits
    /// Number of hits
    pub hits: u64,
    /// Total cache misses
    /// Number of misses
    pub misses: u64,
    /// Total evictions
    /// Number of evictions
    pub evictions: u64,
    /// Current cache size in bytes
    /// Number of `current_size_bytes`
    pub current_size_bytes: u64,
    /// Current number of entries
    /// Number of `current_entries`
    pub current_entries: usize,
    /// Cache hit rate (0.0 to 1.0)
    /// The hit rate value
    pub hit_rate: f64,
}

impl CacheManager {
    /// Create new cache manager
    /// Creates a new instance
    #[must_use]
    pub fn new(config: CacheConfig) -> Self {
        Self {
            entries: HashMap::new(),
            config,
            metrics: CacheMetrics::default(),
        }
    }

    /// Get cached data with zero-copy optimization
    /// Gets value
    /// Gets value
    pub fn get(&mut self, key: &str) -> Option<std::sync::Arc<Vec<u8>>> {
        // Check TTL first
        let should_remove = self
            .entries
            .get(key)
            .is_some_and(|entry| entry.ttl.is_some_and(|ttl| Utc::now() > ttl));

        if should_remove {
            self.entries.remove(key);
            self.metrics.misses += 1;
            self.update_hit_rate();
            return None;
        }

        if let Some(entry) = self.entries.get_mut(key) {
            // Update access statistics
            entry.last_accessed = Utc::now();
            entry.access_count += 1;

            self.metrics.hits += 1;
            // ⚡ ZERO-COPY OPTIMIZATION: Return Arc reference directly (no data clone!)
            let data = entry.data.clone();
            self.update_hit_rate();

            Some(data)
        } else {
            self.metrics.misses += 1;
            self.update_hit_rate();
            None
        }
    }

    /// Put data in cache
    pub fn put(&mut self, key: String, data: Vec<u8>) {
        let size_bytes = data.len() as u64;

        // Check if we need to evict entries
        while self.should_evict(size_bytes) {
            self.evict_one();
        }

        let entry = CacheEntry {
            key: key.clone(),
            data: std::sync::Arc::new(data),
            created_at: Utc::now(),
            last_accessed: Utc::now(),
            access_count: 0,
            size_bytes,
            ttl: Some(
                Utc::now()
                    + chrono::Duration::seconds(
                        i64::try_from(self.config.default_ttl_secs).unwrap_or(3600),
                    ),
            ),
        };

        self.entries.insert(key, entry);
        self.metrics.current_size_bytes += size_bytes;
        self.metrics.current_entries += 1;
    }

    /// Remove entry from cache
    /// Removes item
    /// Removes item
    pub fn remove(&mut self, key: &str) -> bool {
        if let Some(entry) = self.entries.remove(key) {
            self.metrics.current_size_bytes -= entry.size_bytes;
            self.metrics.current_entries -= 1;
            true
        } else {
            false
        }
    }

    /// Clear all cache entries
    pub fn clear(&mut self) {
        self.entries.clear();
        self.metrics.current_size_bytes = 0;
        self.metrics.current_entries = 0;
    }

    /// Get cache metrics
    #[must_use]
    pub const fn metrics(&self) -> &CacheMetrics {
        &self.metrics
    }

    const fn should_evict(&self, new_item_size: u64) -> bool {
        self.metrics.current_size_bytes + new_item_size > self.config.max_size_bytes
            || self.metrics.current_entries >= self.config.max_entries
    }

    fn evict_one(&mut self) {
        // Simple LRU eviction for now
        if let Some(key_to_evict) = self
            .entries
            .iter()
            .min_by_key(|(_, entry)| entry.last_accessed)
            .map(|(k, _)| k.clone())
        {
            self.remove(&key_to_evict);
            self.metrics.evictions += 1;
        }
    }

    /// Updates `hit_rate`
    #[allow(clippy::cast_precision_loss)]
    fn update_hit_rate(&mut self) {
        let total_requests = self.metrics.hits + self.metrics.misses;
        if total_requests > 0 {
            self.metrics.hit_rate = self.metrics.hits as f64 / total_requests as f64;
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_size_bytes: 1024 * 1024 * 1024, // 1GB
            max_entries: std::env::var("BEARDOG_STORAGE_CACHE_MAX_ENTRIES")
                .ok()
                .and_then(|e| e.parse().ok())
                .unwrap_or(10000), // 10K entries default
            default_ttl_secs: std::env::var("BEARDOG_STORAGE_CACHE_DEFAULT_TTL_SECS")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(3600), // 1 hour default
            eviction_policy: CacheEvictionPolicy::Lru,
            enable_compression: false,
        }
    }
}

impl Default for CacheMetrics {
    fn default() -> Self {
        Self {
            hits: 0,
            misses: 0,
            evictions: 0,
            current_size_bytes: 0,
            current_entries: 0,
            hit_rate: 0.0,
        }
    }
}
