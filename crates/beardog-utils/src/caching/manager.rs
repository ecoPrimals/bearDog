// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::{config::CacheConfig, l1_cache::L1Cache, l2_cache::L2Cache, l3_cache::L3Cache};
use beardog_errors::BearDogError;
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;

/// Advanced multi-level cache manager
pub struct AdvancedCacheManager {
    l1_cache: Arc<Mutex<L1Cache>>,
    l2_cache: Arc<Mutex<L2Cache>>,
    l3_cache: Arc<Mutex<L3Cache>>,
    config: CacheConfig,
    stats: Arc<RwLock<CacheManagerStats>>,
}

#[derive(Debug, Clone, Default)]
pub struct CacheManagerStats {
    /// Number of total_requests
    pub total_requests: u64,
    /// Number of l1_hits
    pub l1_hits: u64,
    /// Number of l2_hits
    pub l2_hits: u64,
    /// Number of l3_hits
    pub l3_hits: u64,
    /// Number of total_misses
    pub total_misses: u64,
}

impl AdvancedCacheManager {
    /// Create new cache manager with configuration
    /// Creates a new instance
    pub fn new(config: CacheConfig) -> Result<Self, BearDogError> {
        let l1_cache = Arc::new(Mutex::new(L1Cache::new(&config)?));
        let l2_cache = Arc::new(Mutex::new(L2Cache::new(&config)?));
        let l3_cache = Arc::new(Mutex::new(L3Cache::new(&config)?));

        Ok(Self {
            l1_cache,
            l2_cache,
            l3_cache,
            config,
            stats: Arc::new(RwLock::new(CacheManagerStats::default())),
        })
    }

    /// Get value from multi-level cache
    /// Gets value
    /// Gets value
    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>, BearDogError> {
        let mut stats = self.stats.write();
        stats.total_requests += 1;
        drop(stats);

        // Try L1 cache first
        if let Some(value) = self.try_l1_get(key)? {
            let mut stats = self.stats.write();
            stats.l1_hits += 1;
            return Ok(Some(value));
        }

        // Try L2 cache
        if let Some(value) = self.try_l2_get(key)? {
            let mut stats = self.stats.write();
            stats.l2_hits += 1;
            // Promote to L1
            self.promote_to_l1(key, &value)?;
            return Ok(Some(value));
        }

        // Try L3 cache
        if let Some(value) = self.try_l3_get(key)? {
            let mut stats = self.stats.write();
            stats.l3_hits += 1;
            // Promote to L2 and L1
            self.promote_to_l2(key, &value)?;
            self.promote_to_l1(key, &value)?;
            return Ok(Some(value));
        }

        // Cache miss
        let mut stats = self.stats.write();
        stats.total_misses += 1;
        Ok(None)
    }

    /// Set value in multi-level cache
    /// Sets value
    /// Sets value
    pub fn set(&self, key: &str, value: &[u8]) -> Result<(), BearDogError> {
        // Set in all levels
        self.set_l1(key, value)?;
        self.set_l2(key, value)?;
        self.set_l3(key, value)?;
        Ok(())
    }

    /// Remove value from all cache levels
    /// Removes item
    /// Removes item
    pub fn remove(&self, key: &str) -> Result<bool, BearDogError> {
        let l1_removed = self.remove_l1(key)?;
        let l2_removed = self.remove_l2(key)?;
        let l3_removed = self.remove_l3(key)?;

        Ok(l1_removed || l2_removed || l3_removed)
    }

    /// Clear all cache levels
    pub fn clear(&self) -> Result<(), BearDogError> {
        self.clear_l1()?;
        self.clear_l2()?;
        self.clear_l3()?;

        let mut stats = self.stats.write();
        *stats = CacheManagerStats::default();
        Ok(())
    }

    /// Get cache manager statistics
    /// Gets stats
    /// Gets stats
    pub fn get_stats(&self) -> CacheManagerStats {
        self.stats.read().clone()
    }

    /// Get cache configuration
    /// Gets config
    /// Gets config
    pub fn get_config(&self) -> &CacheConfig {
        &self.config
    }

    /// Check if compression is enabled
    /// Checks if compression enabled
    /// Checks if compression enabled
    pub fn is_compression_enabled(&self) -> bool {
        self.config.enable_compression
    }

    /// Get default TTL
    /// Gets default_ttl
    /// Gets default_ttl
    pub fn get_default_ttl(&self) -> std::time::Duration {
        self.config.default_ttl
    }

    /// Get overall hit ratio
    pub fn hit_ratio(&self) -> f64 {
        let stats = self.stats.read();
        let total_hits = stats.l1_hits + stats.l2_hits + stats.l3_hits;
        let total_requests = stats.total_requests;

        if total_requests == 0 {
            0.0
        } else {
            total_hits as f64 / total_requests as f64
        }
    }

    // Private helper methods for L1 operations
    fn try_l1_get(&self, key: &str) -> Result<Option<Vec<u8>>, BearDogError> {
        let mut cache = self
            .l1_cache
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock L1 cache: {e}")))?;
        cache.get(key)
    }


    fn promote_to_l1(&self, key: &str, value: &[u8]) -> Result<(), BearDogError> {
        let mut cache = self
            .l1_cache
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock L1 cache: {e}")))?;
        cache.set(key, value)
    }

    /// Sets l1
    fn set_l1(&self, key: &str, value: &[u8]) -> Result<(), BearDogError> {
        let mut cache = self
            .l1_cache
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock L1 cache: {e}")))?;
        cache.set(key, value)
    }

    /// Removes l1
    fn remove_l1(&self, key: &str) -> Result<bool, BearDogError> {
        let mut cache = self
            .l1_cache
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock L1 cache: {e}")))?;
        cache.remove(key)
    }


    fn clear_l1(&self) -> Result<(), BearDogError> {
        let mut cache = self
            .l1_cache
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock L1 cache: {e}")))?;
        cache.clear()
    }

    // Private helper methods for L2 operations
    fn try_l2_get(&self, key: &str) -> Result<Option<Vec<u8>>, BearDogError> {
        let mut cache = self
            .l2_cache
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock L2 cache: {e}")))?;
        cache.get(key)
    }


    fn promote_to_l2(&self, key: &str, value: &[u8]) -> Result<(), BearDogError> {
        let mut cache = self
            .l2_cache
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock L2 cache: {e}")))?;
        cache.set(key, value)
    }

    /// Sets l2
    fn set_l2(&self, key: &str, value: &[u8]) -> Result<(), BearDogError> {
        let mut cache = self
            .l2_cache
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock L2 cache: {e}")))?;
        cache.set(key, value)
    }

    /// Removes l2
    fn remove_l2(&self, key: &str) -> Result<bool, BearDogError> {
        let mut cache = self
            .l2_cache
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock L2 cache: {e}")))?;
        cache.remove(key)
    }


    fn clear_l2(&self) -> Result<(), BearDogError> {
        let mut cache = self
            .l2_cache
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock L2 cache: {e}")))?;
        cache.clear()
    }

    // Private helper methods for L3 operations
    fn try_l3_get(&self, key: &str) -> Result<Option<Vec<u8>>, BearDogError> {
        let mut cache = self
            .l3_cache
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock L3 cache: {e}")))?;
        cache.get(key)
    }

    /// Sets l3
    fn set_l3(&self, key: &str, value: &[u8]) -> Result<(), BearDogError> {
        let mut cache = self
            .l3_cache
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock L3 cache: {e}")))?;
        cache.set(key, value)
    }

    /// Removes l3
    fn remove_l3(&self, key: &str) -> Result<bool, BearDogError> {
        let mut cache = self
            .l3_cache
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock L3 cache: {e}")))?;
        cache.remove(key)
    }


    fn clear_l3(&self) -> Result<(), BearDogError> {
        let mut cache = self
            .l3_cache
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to lock L3 cache: {e}")))?;
        cache.clear()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    fn test_cache_manager_basic_operations() -> Result<(), BearDogError> {
        let config = CacheConfig::default();
        let manager = AdvancedCacheManager::new(config)?;

        // Test set and get
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        manager.set("key1", b"value1")?;
        let result = manager.get("key1")?;
        assert_eq!(result, Some(b"value1".to_vec()));

        // Test miss
        let result = manager.get("nonexistent")?;
        assert_eq!(result, None);

        Ok(())
    }

    #[tokio::test]
    fn test_cache_manager_multilevel() -> Result<(), BearDogError> {
        let config = CacheConfig::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let manager = AdvancedCacheManager::new(config)?;

        // Set in all levels
        manager.set("key1", b"value1")?;

        // Should hit L1 first
        let result = manager.get("key1")?;
        assert_eq!(result, Some(b"value1".to_vec()));

        let stats = manager.get_stats();
        assert_eq!(stats.l1_hits, 1);
        assert_eq!(stats.total_requests, 1);

        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[tokio::test]
    fn test_cache_manager_stats() -> Result<(), BearDogError> {
        let config = CacheConfig::default();
        let manager = AdvancedCacheManager::new(config)?;

        manager.set("key1", b"value1")?;
        manager.get("key1")?; // hit
        manager.get("key2")?; // miss

        let stats = manager.get_stats();
        assert_eq!(stats.total_requests, 2);
        assert_eq!(stats.l1_hits, 1);
        assert_eq!(stats.total_misses, 1);

        let hit_ratio = manager.hit_ratio();
        assert!((hit_ratio - 0.5).abs() < f64::EPSILON);

        Ok(())
    }
}
