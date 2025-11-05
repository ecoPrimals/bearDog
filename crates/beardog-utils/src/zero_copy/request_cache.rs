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
    #[must_use]
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

        let mut cache = self.cache.write().unwrap_or_else(|poisoned| {
            tracing::warn!("Request cache lock poisoned on write, recovering");
            poisoned.into_inner()
        });
        cache.insert(key, entry);
    }

    /// Get entry from cache
    /// Gets value
    /// Gets value
    pub fn get(&self, key: &str) -> Option<T> {
        let cache = self.cache.read().unwrap_or_else(|poisoned| {
            tracing::warn!("Request cache lock poisoned on read, recovering");
            poisoned.into_inner()
        });
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
        let mut cache = self.cache.write().unwrap_or_else(|poisoned| {
            tracing::warn!("Request cache lock poisoned on cleanup, recovering");
            poisoned.into_inner()
        });
        let now = Instant::now();
        cache.retain(|_, entry| now.duration_since(entry.timestamp) < entry.ttl);
    }

    /// Get cache size
    pub fn len(&self) -> usize {
        self.cache
            .read()
            .unwrap_or_else(|poisoned| {
                tracing::warn!("Request cache lock poisoned on len, recovering");
                poisoned.into_inner()
            })
            .len()
    }

    /// Check if cache is empty
    /// Checks if empty
    /// Checks if empty
    pub fn is_empty(&self) -> bool {
        self.cache
            .read()
            .unwrap_or_else(|poisoned| {
                tracing::warn!("Request cache lock poisoned on is_empty, recovering");
                poisoned.into_inner()
            })
            .is_empty()
    }
}

impl<T: Clone> Default for RequestCache<T> {
    fn default() -> Self {
        Self::new(Duration::from_secs(300)) // 5 minutes default TTL
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_new() {
        let cache = RequestCache::<String>::new(Duration::from_secs(60));
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.default_ttl, Duration::from_secs(60));
    }

    #[test]
    fn test_cache_insert_and_get() {
        let cache = RequestCache::new(Duration::from_secs(60));

        cache.insert("key1".to_string(), "value1".to_string());
        assert_eq!(cache.get("key1"), Some("value1".to_string()));
        assert_eq!(cache.len(), 1);
        assert!(!cache.is_empty());
    }

    #[test]
    fn test_cache_get_nonexistent() {
        let cache = RequestCache::<String>::new(Duration::from_secs(60));
        assert_eq!(cache.get("nonexistent"), None);
    }

    #[test]
    fn test_cache_overwrite() {
        let cache = RequestCache::new(Duration::from_secs(60));

        cache.insert("key1".to_string(), "value1".to_string());
        assert_eq!(cache.get("key1"), Some("value1".to_string()));

        cache.insert("key1".to_string(), "value2".to_string());
        assert_eq!(cache.get("key1"), Some("value2".to_string()));
        assert_eq!(cache.len(), 1); // Still only one entry
    }

    #[test]
    fn test_cache_multiple_entries() {
        let cache = RequestCache::new(Duration::from_secs(60));

        cache.insert("key1".to_string(), 100);
        cache.insert("key2".to_string(), 200);
        cache.insert("key3".to_string(), 300);

        assert_eq!(cache.get("key1"), Some(100));
        assert_eq!(cache.get("key2"), Some(200));
        assert_eq!(cache.get("key3"), Some(300));
        assert_eq!(cache.len(), 3);
    }

    #[test]
    fn test_cache_expiration() {
        let cache = RequestCache::new(Duration::from_millis(50));

        cache.insert("key1".to_string(), "value1".to_string());
        assert_eq!(cache.get("key1"), Some("value1".to_string()));

        // Wait for TTL to expire
        std::thread::sleep(Duration::from_millis(60));

        // Should return None after expiration
        assert_eq!(cache.get("key1"), None);
        // But entry is still in cache until cleanup
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_cache_cleanup_expired() {
        let cache = RequestCache::new(Duration::from_millis(50));

        cache.insert("key1".to_string(), "value1".to_string());
        cache.insert("key2".to_string(), "value2".to_string());

        assert_eq!(cache.len(), 2);

        // Wait for TTL to expire
        std::thread::sleep(Duration::from_millis(60));

        // Cleanup expired entries
        cache.cleanup_expired();

        // Both entries should be removed
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_cache_cleanup_keeps_valid() {
        let cache = RequestCache::new(Duration::from_secs(60));

        cache.insert("key1".to_string(), "value1".to_string());
        cache.insert("key2".to_string(), "value2".to_string());

        assert_eq!(cache.len(), 2);

        // Cleanup should not remove valid entries
        cache.cleanup_expired();

        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get("key1"), Some("value1".to_string()));
        assert_eq!(cache.get("key2"), Some("value2".to_string()));
    }

    #[test]
    fn test_cache_default() {
        let cache = RequestCache::<String>::default();
        assert!(cache.is_empty());
        assert_eq!(cache.default_ttl, Duration::from_secs(300));
    }

    #[test]
    fn test_cache_entry_clone() {
        let entry = CacheEntry {
            data: "test".to_string(),
            timestamp: Instant::now(),
            ttl: Duration::from_secs(60),
        };

        let cloned = entry.clone();
        assert_eq!(entry.data, cloned.data);
        assert_eq!(entry.ttl, cloned.ttl);
    }

    #[test]
    fn test_cache_is_empty() {
        let cache = RequestCache::<String>::new(Duration::from_secs(60));
        assert!(cache.is_empty());

        cache.insert("key1".to_string(), "value1".to_string());
        assert!(!cache.is_empty());

        cache.cleanup_expired();
        assert!(!cache.is_empty()); // Still has valid entry
    }

    #[test]
    fn test_cache_len() {
        let cache = RequestCache::<u32>::new(Duration::from_secs(60));
        assert_eq!(cache.len(), 0);

        cache.insert("key1".to_string(), 1);
        assert_eq!(cache.len(), 1);

        cache.insert("key2".to_string(), 2);
        cache.insert("key3".to_string(), 3);
        assert_eq!(cache.len(), 3);

        cache.insert("key1".to_string(), 10); // Overwrite
        assert_eq!(cache.len(), 3); // Still 3 entries
    }

    #[test]
    fn test_cache_concurrent_access() {
        use std::sync::Arc;
        use std::thread;

        let cache = Arc::new(RequestCache::new(Duration::from_secs(60)));
        let mut handles = vec![];

        // Spawn threads that insert and read
        for i in 0..5 {
            let cache_clone = Arc::clone(&cache);
            handles.push(thread::spawn(move || {
                for j in 0..10 {
                    let key = format!("key_{}_{}", i, j);
                    let value = i * 10 + j;
                    cache_clone.insert(key.clone(), value);
                    let retrieved = cache_clone.get(&key);
                    assert_eq!(retrieved, Some(value));
                }
            }));
        }

        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(cache.len(), 50); // 5 threads * 10 entries each
    }

    #[test]
    fn test_cache_mixed_expiration() {
        let cache = RequestCache::new(Duration::from_millis(100));

        // Insert first batch
        cache.insert("short1".to_string(), "value1".to_string());
        cache.insert("short2".to_string(), "value2".to_string());

        // Wait a bit
        std::thread::sleep(Duration::from_millis(60));

        // Insert second batch (will have longer TTL remaining)
        cache.insert("long1".to_string(), "value3".to_string());
        cache.insert("long2".to_string(), "value4".to_string());

        assert_eq!(cache.len(), 4);

        // Wait for first batch to expire
        std::thread::sleep(Duration::from_millis(50));

        cache.cleanup_expired();

        // First batch should be gone, second batch still present
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get("long1"), Some("value3".to_string()));
        assert_eq!(cache.get("long2"), Some("value4".to_string()));
        assert_eq!(cache.get("short1"), None);
        assert_eq!(cache.get("short2"), None);
    }
}
