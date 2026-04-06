// SPDX-License-Identifier: AGPL-3.0-or-later

//! [`DashMap`](dashmap::DashMap) hash maps and LRU-ish caches with `parking_lot` backing locks.

use beardog_errors::BearDogError;
use dashmap::DashMap;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use tracing::{debug, info};

/// Sharded concurrent map with atomic usage counters.
#[derive(Debug)]
pub struct SafeConcurrentHashMap<K, V>
where
    K: Hash + Eq + Clone + Send + Sync,
    V: Clone + Send + Sync,
{
    inner: DashMap<K, V>,
    stats: SafeConcurrentStats,
}

/// Safe concurrent cache with LRU eviction
#[derive(Debug)]
pub struct SafeConcurrentCache<K, V>
where
    K: Hash + Eq + Clone + Send + Sync,
    V: Clone + Send + Sync,
{
    data: Arc<RwLock<HashMap<K, (V, u64)>>>, // (value, access_time)
    access_counter: AtomicU64,
    max_size: usize,
    stats: SafeConcurrentStats,
}

/// Atomics backing insert/lookup/eviction telemetry.
#[derive(Debug, Default)]
pub struct SafeConcurrentStats {
    /// The insertions value
    pub insertions: AtomicU64,
    /// The lookups value
    pub lookups: AtomicU64,
    /// The cache hits value
    pub cache_hits: AtomicU64,
    /// The cache misses value
    pub cache_misses: AtomicU64,
    /// The evictions value
    pub evictions: AtomicU64,
    /// The size value
    pub size: AtomicUsize,
}

impl<K, V> SafeConcurrentHashMap<K, V>
where
    K: Hash + Eq + Clone + Send + Sync,
    V: Clone + Send + Sync,
{
    /// Create a new safe concurrent hash map
    /// Creates a new instance
    pub fn new() -> Self {
        info!("🛡️ Creating SafeConcurrentHashMap - safe Rust only");

        Self {
            inner: DashMap::new(),
            stats: SafeConcurrentStats::default(),
        }
    }

    /// Create with capacity hint
    /// Creates instance with capacity
    pub fn with_capacity(capacity: usize) -> Self {
        info!(
            "🛡️ Creating SafeConcurrentHashMap with capacity {} - safe Rust only",
            capacity
        );

        Self {
            inner: DashMap::with_capacity(capacity),
            stats: SafeConcurrentStats::default(),
        }
    }

    /// Insert a key-value pair
    pub fn insert(&self, key: K, value: V) -> Option<V> {
        debug!("📝 Safe concurrent insert");

        self.stats.insertions.fetch_add(1, Ordering::Relaxed);
        let old_value = self.inner.insert(key, value);

        if old_value.is_none() {
            self.stats.size.fetch_add(1, Ordering::Relaxed);
        }

        old_value
    }

    /// Get a value by key
    /// Gets value
    pub fn get(&self, key: &K) -> Option<V> {
        debug!("🔍 Safe concurrent lookup");

        self.stats.lookups.fetch_add(1, Ordering::Relaxed);

        if let Some(entry) = self.inner.get(key) {
            self.stats.cache_hits.fetch_add(1, Ordering::Relaxed);
            Some(entry.value().clone())
        } else {
            self.stats.cache_misses.fetch_add(1, Ordering::Relaxed);
            None
        }
    }

    /// Remove a key-value pair
    /// Removes item
    pub fn remove(&self, key: &K) -> Option<V> {
        debug!("🗑️ Safe concurrent remove");

        if let Some((_, value)) = self.inner.remove(key) {
            self.stats.size.fetch_sub(1, Ordering::Relaxed);
            Some(value)
        } else {
            None
        }
    }

    /// Get current size
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if empty
    /// Checks if empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Clear all entries
    pub fn clear(&self) {
        self.inner.clear();
        self.stats.size.store(0, Ordering::Relaxed);
    }

    /// Gets stats
    pub fn get_stats(&self) -> SafeConcurrentStats {
        SafeConcurrentStats {
            insertions: AtomicU64::new(self.stats.insertions.load(Ordering::Relaxed)),
            lookups: AtomicU64::new(self.stats.lookups.load(Ordering::Relaxed)),
            cache_hits: AtomicU64::new(self.stats.cache_hits.load(Ordering::Relaxed)),
            cache_misses: AtomicU64::new(self.stats.cache_misses.load(Ordering::Relaxed)),
            evictions: AtomicU64::new(self.stats.evictions.load(Ordering::Relaxed)),
            size: AtomicUsize::new(self.stats.size.load(Ordering::Relaxed)),
        }
    }
}

impl<K, V> Default for SafeConcurrentHashMap<K, V>
where
    K: Hash + Eq + Clone + Send + Sync,
    V: Clone + Send + Sync,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> SafeConcurrentCache<K, V>
where
    K: Hash + Eq + Clone + Send + Sync,
    V: Clone + Send + Sync,
{
    /// Create a new safe concurrent cache
    /// Creates a new instance
    pub fn new(max_size: usize) -> Self {
        info!(
            "🛡️ Creating SafeConcurrentCache with max_size {} - safe Rust only",
            max_size
        );

        Self {
            data: Arc::new(RwLock::new(HashMap::with_capacity(max_size))),
            access_counter: AtomicU64::new(0),
            max_size,
            stats: SafeConcurrentStats::default(),
        }
    }

    /// Insert with LRU eviction
    ///
    /// # Errors
    ///
    /// Currently always returns `Ok` with the previous value if the key existed; the `Result` is
    /// reserved for future error paths.
    pub fn insert(&self, key: K, value: V) -> Result<Option<V>, BearDogError> {
        debug!("📝 Safe concurrent cache insert");

        let access_time = self.access_counter.fetch_add(1, Ordering::Relaxed);
        let mut data = self.data.write();

        self.stats.insertions.fetch_add(1, Ordering::Relaxed);

        // Check if we need to evict
        if data.len() >= self.max_size && !data.contains_key(&key) {
            self.evict_lru(&mut data);
        }

        let old_value = data.insert(key, (value, access_time)).map(|(v, _)| v);

        if old_value.is_none() {
            self.stats.size.fetch_add(1, Ordering::Relaxed);
        }

        Ok(old_value)
    }

    /// Get with access time update
    /// Gets value
    pub fn get(&self, key: &K) -> Option<V> {
        debug!("🔍 Safe concurrent cache lookup");

        self.stats.lookups.fetch_add(1, Ordering::Relaxed);

        // Try read lock first
        {
            let data = self.data.read();
            if let Some((value, _)) = data.get(key) {
                self.stats.cache_hits.fetch_add(1, Ordering::Relaxed);
                return Some(value.clone());
            }
        }

        self.stats.cache_misses.fetch_add(1, Ordering::Relaxed);
        None
    }

    /// Bumps LRU metadata for `key` if present.
    pub fn touch(&self, key: &K) -> bool {
        let access_time = self.access_counter.fetch_add(1, Ordering::Relaxed);
        let mut data = self.data.write();

        if let Some((value, _)) = data.get(key).cloned() {
            data.insert(key.clone(), (value, access_time));
            true
        } else {
            false
        }
    }

    /// Remove entry
    /// Removes item
    pub fn remove(&self, key: &K) -> Option<V> {
        let mut data = self.data.write();
        if let Some((value, _)) = data.remove(key) {
            self.stats.size.fetch_sub(1, Ordering::Relaxed);
            Some(value)
        } else {
            None
        }
    }

    /// Get current size
    pub fn len(&self) -> usize {
        self.data.read().len()
    }

    /// Check if empty
    /// Checks if empty
    pub fn is_empty(&self) -> bool {
        self.data.read().is_empty()
    }

    /// Clear all entries
    pub fn clear(&self) {
        self.data.write().clear();
        self.stats.size.store(0, Ordering::Relaxed);
    }

    /// Evict least recently used entry
    fn evict_lru(&self, data: &mut HashMap<K, (V, u64)>) {
        if let Some((lru_key, _)) = data
            .iter()
            .min_by_key(|(_, (_, access_time))| *access_time)
            .map(|(k, v)| (k.clone(), v.clone()))
        {
            data.remove(&lru_key);
            self.stats.evictions.fetch_add(1, Ordering::Relaxed);
            self.stats.size.fetch_sub(1, Ordering::Relaxed);

            debug!("♻️ Evicted LRU entry from safe cache");
        }
    }

    /// Gets stats
    pub fn get_stats(&self) -> SafeConcurrentStats {
        SafeConcurrentStats {
            insertions: AtomicU64::new(self.stats.insertions.load(Ordering::Relaxed)),
            lookups: AtomicU64::new(self.stats.lookups.load(Ordering::Relaxed)),
            cache_hits: AtomicU64::new(self.stats.cache_hits.load(Ordering::Relaxed)),
            cache_misses: AtomicU64::new(self.stats.cache_misses.load(Ordering::Relaxed)),
            evictions: AtomicU64::new(self.stats.evictions.load(Ordering::Relaxed)),
            size: AtomicUsize::new(self.stats.size.load(Ordering::Relaxed)),
        }
    }
}

/// Safe concurrent queue using crossbeam
#[derive(Debug)]
pub struct SafeConcurrentQueue<T>
where
    T: Send + Sync,
{
    inner: crossbeam::queue::SegQueue<T>,
    stats: SafeConcurrentStats,
}

impl<T> SafeConcurrentQueue<T>
where
    T: Send + Sync,
{
    /// Create a new safe concurrent queue
    /// Creates a new instance
    pub fn new() -> Self {
        info!("🛡️ Creating SafeConcurrentQueue - safe Rust only");

        Self {
            inner: crossbeam::queue::SegQueue::new(),
            stats: SafeConcurrentStats::default(),
        }
    }

    /// Push an item
    pub fn push(&self, item: T) {
        debug!("➡️ Safe concurrent queue push");

        self.inner.push(item);
        self.stats.insertions.fetch_add(1, Ordering::Relaxed);
        self.stats.size.fetch_add(1, Ordering::Relaxed);
    }

    /// Pop an item
    pub fn pop(&self) -> Option<T> {
        debug!("⬅️ Safe concurrent queue pop");

        if let Some(item) = self.inner.pop() {
            self.stats.cache_hits.fetch_add(1, Ordering::Relaxed);
            self.stats.size.fetch_sub(1, Ordering::Relaxed);
            Some(item)
        } else {
            self.stats.cache_misses.fetch_add(1, Ordering::Relaxed);
            None
        }
    }

    /// Check if empty
    /// Checks if empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Get approximate length
    pub fn len(&self) -> usize {
        self.stats.size.load(Ordering::Relaxed)
    }
}

impl<T> Default for SafeConcurrentQueue<T>
where
    T: Send + Sync,
{
    fn default() -> Self {
        Self::new()
    }
}

#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_concurrent_hashmap() {
        let map = SafeConcurrentHashMap::new();

        // Test insert and get
        assert_eq!(map.insert("key1".to_string(), "value1".to_string()), None);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert_eq!(map.get(&"key1".to_string()), Some("value1".to_string()));

        // Test update
        assert_eq!(
            map.insert("key1".to_string(), "value2".to_string()),
            Some("value1".to_string())
        );
        assert_eq!(map.get(&"key1".to_string()), Some("value2".to_string()));

        // Test remove
        assert_eq!(map.remove(&"key1".to_string()), Some("value2".to_string()));
        assert_eq!(map.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_safe_concurrent_cache() {
        let cache = SafeConcurrentCache::new(2);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal

        // Test insert and get
        assert!(
            cache
                .insert("key1".to_string(), "value1".to_string())
                .is_ok()
        );
        assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));

        // Test LRU eviction
        assert!(
            cache
                .insert("key2".to_string(), "value2".to_string())
                .is_ok()
        );
        assert!(
            cache
                .insert("key3".to_string(), "value3".to_string())
                .is_ok()
        ); // Should evict key1

        assert_eq!(cache.get(&"key1".to_string()), None); // Evicted
        assert_eq!(cache.get(&"key2".to_string()), Some("value2".to_string()));
        assert_eq!(cache.get(&"key3".to_string()), Some("value3".to_string()));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_safe_concurrent_queue() {
        let queue = SafeConcurrentQueue::new();

        // Test push and pop
        queue.push("item1".to_string());
        queue.push("item2".to_string());

        assert_eq!(queue.pop(), Some("item1".to_string()));
        assert_eq!(queue.pop(), Some("item2".to_string()));
        assert_eq!(queue.pop(), None);
    }
}
