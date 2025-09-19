// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::config::CacheConfig;
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::time::Instant;

/// L2 (Level 2) cache - medium capacity, compressed storage
pub struct L2Cache {
    entries: HashMap<String, L2CacheEntry>,
    config: L2CacheConfig,
    stats: L2CacheStats,
}

#[derive(Debug, Clone)]
pub struct L2CacheConfig {
    /// Number of max_size_mb
    pub max_size_mb: usize,
    /// Whether enable_compression is enabled
    pub enable_compression: bool,
}

/// Entry in the L2 cache
#[derive(Debug, Clone)]
pub struct L2CacheEntry {
    /// Collection of buffer data
    pub buffer_data: Vec<u8>,
    /// The created at value
    pub created_at: Instant,
    /// Whether compressed is enabled
    pub compressed: bool,
}

#[derive(Debug, Clone, Default)]
pub struct L2CacheStats {
    /// Number of hits
    pub hits: u64,
    /// Number of misses
    pub misses: u64,
    /// Number of entries
    pub entries: usize,
    /// Number of memory_usage_mb
    pub memory_usage_mb: usize,
}

impl L2Cache {
    /// Create new L2 cache with configuration
    /// Creates a new instance
    pub fn new(config: &CacheConfig) -> Result<Self, BearDogError> {
        Ok(Self {
            entries: HashMap::with_capacity(1000),
            config: L2CacheConfig {
                max_size_mb: config.l2_max_size_mb,
                enable_compression: config.enable_compression,
            },
            stats: L2CacheStats::default(),
        })
    }

    /// Get value from cache
    /// Gets value
    /// Gets value
    pub fn get(&mut self, key: &str) -> Result<Option<Vec<u8>>, BearDogError> {
        if let Some(entry) = self.entries.get(key) {
            self.stats.hits += 1;
            let buffer_data = if entry.compressed {
                self.decompress_data(&entry.data)?
            } else {
                entry.data.clone()
            };
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
        let (data, compressed) = if self.config.enable_compression && value.len() > 1024 {
            (self.compress_data(value)?, true)
        } else {
            (value.to_vec(), false)
        };

        let entry = L2CacheEntry {
            data,
            created_at: Instant::now(),
            compressed,
        };

        self.entries.insert(key.to_string(), entry);
        self.update_stats();
        Ok(())
    }

    /// Remove entry from cache
    /// Removes item
    /// Removes item
    pub fn remove(&mut self, key: &str) -> Result<bool, BearDogError> {
        let removed = self.entries.remove(key).is_some();
        if removed {
            self.update_stats();
        }
        Ok(removed)
    }

    /// Clear all entries from cache
    pub fn clear(&mut self) -> Result<(), BearDogError> {
        self.entries.clear();
        self.stats = L2CacheStats::default();
        Ok(())
    }

    /// Get cache statistics
    /// Gets stats
    /// Gets stats
    pub fn get_stats(&self) -> &L2CacheStats {
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

    // Private helper methods
    fn compress_data(&self, buffer_data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        // Simplified compression (in production, use a real compression library)
        let mut compressed = Vec::with_capacity(data.len() / 2);
        let mut i = 0;
        while i < data.len() {
            let byte = data[i];
            let mut count = 1u8;

            // Simple run-length encoding
            while i + (count as usize) < data.len()
                && data[i + (count as usize)] == byte
                && count < 255
            {
                count += 1;
            }

            compressed.push(count);
            compressed.push(byte);
            i += count as usize;
        }
        Ok(compressed)
    }


    fn decompress_data(&self, compressed: &[u8]) -> Result<Vec<u8>, BearDogError> {
        // Decompress run-length encoded data
        let mut decompressed = Vec::new();
        let mut i = 0;

        while i + 1 < compressed.len() {
            let count = compressed[i];
            let byte = compressed[i + 1];

            for _ in 0..count {
                decompressed.push(byte);
            }

            i += 2;
        }

        Ok(decompressed)
    }

    /// Updates stats
    fn update_stats(&mut self) {
        self.stats.entries = self.entries.len();
        self.stats.memory_usage_mb = self
            .entries
            .values()
            .map(|entry| entry.data.len())
            .sum::<usize>()
            / (1024 * 1024);
    }
}

impl Default for L2CacheConfig {
    fn default() -> Self {
        Self {
            max_size_mb: 100,
            enable_compression: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_l2_cache_basic_operations() -> Result<(), BearDogError> {
        let config = CacheConfig::default();
        let mut cache = L2Cache::new(config)?;

        // Test set and get
        cache.set("key1", b"value1")?;
        let result = cache.get("key1")?;
        assert_eq!(result, Some(b"value1".to_vec()));

        // Test miss
        let result = cache.get("nonexistent")?;
        assert_eq!(result, None);

        Ok(())
    }

    #[test]
    fn test_l2_cache_compression() -> Result<(), BearDogError> {
        let config = CacheConfig::default();
        let mut cache = L2Cache::new(config)?;

        // Create data large enough to trigger compression
        let large_data = vec![42u8; 2048];
        cache.set("large_key", &large_data)?;

        let result = cache.get("large_key")?;
        assert_eq!(result, Some(large_data));

        Ok(())
    }

    #[test]
    fn test_l2_cache_stats() -> Result<(), BearDogError> {
        let config = CacheConfig::default();
        let mut cache = L2Cache::new(config)?;

        cache.set("key1", b"value1")?;
        cache.get("key1")?; // hit
        cache.get("key2")?; // miss

        let stats = cache.get_stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert!((cache.hit_ratio() - 0.5).abs() < f64::EPSILON);

        Ok(())
    }
}
