// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::config::CacheConfig;
use beardog_errors::BearDogError;
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

/// L1 (Level 1) cache - fastest, smallest cache layer
pub struct L1Cache {
    entries: HashMap<String, L1CacheEntry>,
    access_order: VecDeque<String>,
    config: L1CacheConfig,
    stats: L1CacheStats,
}

#[derive(Debug, Clone)]
pub struct L1CacheConfig {
    /// Number of max_entries
    pub max_entries: usize,
    /// Whether track_access is enabled
    pub track_access: bool,
    /// The cleanup interval value
    pub cleanup_interval: Duration,
}

/// Entry in the L1 cache
#[derive(Debug, Clone)]
pub struct L1CacheEntry {
    /// Collection of buffer data
    pub buffer_data: Vec<u8>,
    /// The created at value
    pub created_at: Instant,
    /// The accessed at value
    pub accessed_at: Instant,
    /// Number of access
    pub access_count: u64,
    /// Optional ttl
    pub ttl: Option<Duration>,
}

#[derive(Debug, Clone, Default)]
pub struct L1CacheStats {
    /// Number of hits
    pub hits: u64,
    /// Number of misses
    pub misses: u64,
    /// Number of entries
    pub entries: usize,
    /// Number of memory_usage
    pub memory_usage: usize,
}

impl L1Cache {
    /// Create new L1 cache with configuration
    /// Creates a new instance
    pub fn new(config: &CacheConfig) -> Result<Self, BearDogError> {
        Ok(Self {
            entries: HashMap::with_capacity(config.l1_max_entries),
            access_order: VecDeque::with_capacity(config.l1_max_entries),
            config: L1CacheConfig {
                max_entries: config.l1_max_entries,
                track_access: true,
                cleanup_interval: Duration::from_secs(60),
            },
            stats: L1CacheStats::default(),
        })
    }

    /// Get value from cache
    /// Gets value
    /// Gets value
    pub fn get(&mut self, key: &str) -> Result<Option<Vec<u8>>, BearDogError> {
        // Check if entry exists and hasn't expired
        if let Some(entry) = self.entries.get(key) {
            if let Some(ttl) = entry.ttl {
                if entry.created_at.elapsed() > ttl {
                    // Entry expired, remove it
                    self.entries.remove(key);
                    self.remove_from_access_order(key);
                    self.stats.misses += 1;
                    return Ok(None);
                }
            }

            let buffer_data = entry.data.clone();

            // Now update the entry
            if let Some(entry) = self.entries.get_mut(key) {
                entry.accessed_at = Instant::now();
                entry.access_count += 1;
            }

            if self.config.track_access {
                self.update_access_order(key);
            }

            self.stats.hits += 1;
            Ok(Some(data))
        } else {
            self.stats.misses += 1;
            Ok(None)
        }
    }

    /// Set value in cache
    /// Sets value
    /// Sets value
    pub fn set(&mut self, key: &str, value: &[u8]) -> Result<(), BearDogError> {
        self.set_with_ttl(key, value, None)
    }

    /// Set value in cache with TTL
    /// Sets with_ttl
    /// Sets with_ttl
    pub fn set_with_ttl(
        &mut self,
        key: &str,
        value: &[u8],
        ttl: Option<Duration>,
    ) -> Result<(), BearDogError> {
        // Check if we need to evict entries
        if self.entries.len() >= self.config.max_entries && !self.entries.contains_key(key) {
            self.evict_lru()?;
        }

        let now = Instant::now();
        let entry = L1CacheEntry {
            data: value.to_vec(),
            created_at: now,
            accessed_at: now,
            access_count: 1,
            ttl,
        };

        // Insert or update entry
        if self.entries.insert(key.to_string(), entry).is_none() {
            // New entry
            self.access_order.push_back(key.to_string());
        } else {
            // Updated existing entry - move to back
            self.update_access_order(key);
        }

        self.update_stats();
        Ok(())
    }

    /// Remove entry from cache
    /// Removes item
    /// Removes item
    pub fn remove(&mut self, key: &str) -> Result<bool, BearDogError> {
        let removed = self.entries.remove(key).is_some();
        if removed {
            self.remove_from_access_order(key);
            self.update_stats();
        }
        Ok(removed)
    }

    /// Clear all entries from cache
    pub fn clear(&mut self) -> Result<(), BearDogError> {
        self.entries.clear();
        self.access_order.clear();
        self.stats = L1CacheStats::default();
        Ok(())
    }

    /// Get cache statistics
    /// Gets stats
    /// Gets stats
    pub fn get_stats(&self) -> &L1CacheStats {
        &self.stats
    }

    /// Get hit ratio
    pub fn hit_ratio(&self) -> f64 {
        let total = self.stats.hits + self.stats.misses;
        if total == 0 {
            0.0
        } else {
            self.stats.hits as f64 / total as f64
        }
    }

    /// Check if cache contains key
    pub fn contains_key(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }

    /// Get number of entries in cache
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if cache is empty
    /// Checks if empty
    /// Checks if empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Cleanup expired entries
    /// Cleans up expired
    /// Cleans up expired
    pub fn cleanup_expired(&mut self) -> Result<usize, BearDogError> {
        let mut removed_count = 0;
        let now = Instant::now();

        let expired_keys: Vec<String> = self
            .entries
            .iter()
            .filter_map(|(key, entry)| {
                if let Some(ttl) = entry.ttl {
                    if now.duration_since(entry.created_at) > ttl {
                        Some(key.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        for key in expired_keys {
            self.entries.remove(&key);
            self.remove_from_access_order(&key);
            removed_count += 1;
        }

        if removed_count > 0 {
            self.update_stats();
        }

        Ok(removed_count)
    }

    // Private helper methods
    fn evict_lru(&mut self) -> Result<(), BearDogError> {
        if let Some(lru_key) = self.access_order.pop_front() {
            self.entries.remove(&lru_key);
            self.update_stats();
        }
        Ok(())
    }

    /// Updates access_order
    fn update_access_order(&mut self, key: &str) {
        self.remove_from_access_order(key);
        self.access_order.push_back(key.to_string());
    }

    /// Removes from_access_order
    fn remove_from_access_order(&mut self, key: &str) {
        if let Some(pos) = self.access_order.iter().position(|value| x == key) {
            self.access_order.remove(pos);
        }
    }

    /// Updates stats
    fn update_stats(&mut self) {
        self.stats.entries = self.entries.len();
        self.stats.memory_usage = self
            .entries
            .values()
            .map(|entry| entry.data.len() + std::mem::size_of::<L1CacheEntry>())
            .sum();
    }
}

impl Default for L1CacheConfig {
    fn default() -> Self {
        Self {
            max_entries: 10_000,
            track_access: true,
            cleanup_interval: Duration::from_secs(60),
        }
    }
}

#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_l1_cache_basic_operations() -> Result<(), BearDogError> {
        let config = CacheConfig::default();
        let mut cache = L1Cache::new(config)?;

        // Test set and get
        cache.set("key1", b"value1")?;
        let result = cache.get("key1")?;
        assert_eq!(result, Some(b"value1".to_vec()));

        // Test miss
        let result = cache.get("nonexistent")?;
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert_eq!(result, None);

        // Test statistics
        let stats = cache.get_stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);

        Ok(())
    }

    #[test]
    fn test_l1_cache_eviction() -> Result<(), BearDogError> {
        let mut config = CacheConfig::default();
        config.l1_max_entries = 2;
        let mut cache = L1Cache::new(config)?;

        // Fill cache
        cache.set("key1", b"value1")?;
        cache.set("key2", b"value2")?;
 // TEST_CATEGORY: unit
 // TEST_DOMAIN: core
 // TEST_PRIORITY: normal

        // This should evict key1 (LRU)
        cache.set("key3", b"value3")?;

        assert_eq!(cache.get("key1")?, None);
        assert_eq!(cache.get("key2")?, Some(b"value2".to_vec()));
        assert_eq!(cache.get("key3")?, Some(b"value3".to_vec()));

        Ok(())
    }

    #[test]
    fn test_l1_cache_ttl() -> Result<(), BearDogError> {
        let config = CacheConfig::default();
        let mut cache = L1Cache::new(config)?;

        // Set with very short TTL
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        cache.set_with_ttl("key1", b"value1", Some(Duration::from_millis(1)))?;

        // Wait for expiration - minimal sleep (2ms > 1ms TTL)
        std::thread::sleep(Duration::from_millis(2));

        let result = cache.get("key1")?;
        assert_eq!(result, None);

        Ok(())
    }

    #[test]
    fn test_l1_cache_clear() -> Result<(), BearDogError> {
        let config = CacheConfig::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let mut cache = L1Cache::new(config)?;

        cache.set("key1", b"value1")?;
        cache.set("key2", b"value2")?;

        assert_eq!(cache.len(), 2);

        cache.clear()?;

        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());

        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_l1_cache_hit_ratio() -> Result<(), BearDogError> {
        let config = CacheConfig::default();
        let mut cache = L1Cache::new(config)?;

        cache.set("key1", b"value1")?;

        // 1 hit
        cache.get("key1")?;
        // 1 miss
        cache.get("key2")?;

        assert!((cache.hit_ratio() - 0.5).abs() < f64::EPSILON);

        Ok(())
    }
}
