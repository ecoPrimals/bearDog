// Safe request caching utilities
//
// This module provides request caching using safe Rust patterns.

use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, Instant};

/// Request cache entry
#[derive(Debug, Clone)]
pub struct CacheEntry<T> {
    /// The data value
    pub data: T,
    pub timestamp: Instant,
    /// The ttl value
    pub ttl: Duration,
}

/// Thread-safe request cache
#[derive(Debug)]
pub struct RequestCache<T> {
    cache: RwLock<HashMap<String, CacheEntry<T>>>,
    default_ttl: Duration,
}

impl<T: Clone> RequestCache<T> {
    /// Create new request cache
    /// Creates a new instance
    pub fn new(default_ttl: Duration) -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            default_ttl,
        }
    }

    /// Insert entry into cache
    pub fn insert(&self, key: String, data: T) {
        let entry = CacheEntry {
            data,
            timestamp: Instant::now(),
            ttl: self.default_ttl,
        };

        let mut cache = self.cache.write().unwrap();
        cache.insert(key, entry);
    }

    /// Get entry from cache
    /// Gets value
    /// Gets value
    pub fn get(&self, key: &str) -> Option<T> {
        let cache = self.cache.read().unwrap();
        if let Some(entry) = cache.get(key) {
            if entry.timestamp.elapsed() < entry.ttl {
                return Some(entry.data.clone());
            }
        }
        None
    }

    /// Remove expired entries
    /// Cleans up expired
    /// Cleans up expired
    pub fn cleanup_expired(&self) {
        let mut cache = self.cache.write().unwrap();
        let now = Instant::now();
        cache.retain(|_, entry| now.duration_since(entry.timestamp) < entry.ttl);
    }

    /// Get cache size
    pub fn len(&self) -> usize {
        self.cache.read().unwrap().len()
    }

    /// Check if cache is empty
    /// Checks if empty
    /// Checks if empty
    pub fn is_empty(&self) -> bool {
        self.cache.read().unwrap().is_empty()
    }
}

impl<T: Clone> Default for RequestCache<T> {
    fn default() -> Self {
        Self::new(Duration::from_secs(300)) // 5 minutes default TTL
    }
}
