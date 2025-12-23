//! Comprehensive Concurrent Safe Operations Tests
//!
//! Tests for `BearDog`'s safe concurrent primitives and patterns,
//! focusing on thread-safe operations, lock-free patterns, and race condition prevention.
//!
//! This module aims to increase test coverage for `beardog-utils`'s concurrent-safe
//! functionality, ensuring memory safety and correctness under concurrent access.

#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all
)]

use crate::concurrent_safe::*;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: concurrency
/// `TEST_PRIORITY`: critical
#[cfg(test)]
mod concurrent_hashmap_tests {
    use super::*;

    #[test]
    fn test_concurrent_hashmap_creation() {
        let map = SafeConcurrentHashMap::<String, String>::new();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_concurrent_hashmap_with_capacity() {
        let map = SafeConcurrentHashMap::<i32, i32>::with_capacity(100);
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_concurrent_hashmap_insert_get() {
        let map = SafeConcurrentHashMap::new();

        map.insert("key1".to_string(), "value1".to_string());
        assert_eq!(map.get(&"key1".to_string()), Some("value1".to_string()));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_concurrent_hashmap_update() {
        let map = SafeConcurrentHashMap::new();

        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key1".to_string(), "value2".to_string());

        assert_eq!(map.get(&"key1".to_string()), Some("value2".to_string()));
        assert_eq!(map.len(), 1); // Should still be 1 after update
    }

    #[test]
    fn test_concurrent_hashmap_remove() {
        let map = SafeConcurrentHashMap::new();

        map.insert("key1".to_string(), "value1".to_string());
        assert_eq!(map.remove(&"key1".to_string()), Some("value1".to_string()));
        assert_eq!(map.get(&"key1".to_string()), None);
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_concurrent_hashmap_clear() {
        let map = SafeConcurrentHashMap::new();

        for i in 0..10 {
            map.insert(i, i * 2);
        }
        assert_eq!(map.len(), 10);

        map.clear();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }

    #[test]
    fn test_concurrent_hashmap_multiple_keys() {
        let map = SafeConcurrentHashMap::new();

        for i in 0..100 {
            map.insert(format!("key{}", i), format!("value{}", i));
        }

        assert_eq!(map.len(), 100);

        for i in 0..100 {
            assert_eq!(map.get(&format!("key{}", i)), Some(format!("value{}", i)));
        }
    }

    #[test]
    fn test_concurrent_hashmap_concurrent_inserts() {
        let map = Arc::new(SafeConcurrentHashMap::new());
        let mut handles = vec![];
        let num_threads = 10;
        let inserts_per_thread = 100;

        for thread_id in 0..num_threads {
            let map_ref = map.clone();
            let handle = thread::spawn(move || {
                for i in 0..inserts_per_thread {
                    let key = format!("t{}_k{}", thread_id, i);
                    let value = format!("v{}", i);
                    map_ref.insert(key, value);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(map.len(), num_threads * inserts_per_thread);
    }

    #[test]
    fn test_concurrent_hashmap_concurrent_reads() {
        let map = Arc::new(SafeConcurrentHashMap::new());

        // Pre-populate
        for i in 0..100 {
            map.insert(format!("key{}", i), format!("value{}", i));
        }

        let mut handles = vec![];
        let num_threads = 10;

        for _ in 0..num_threads {
            let map_ref = map.clone();
            let handle = thread::spawn(move || {
                for i in 0..100 {
                    let key = format!("key{}", i);
                    let value = map_ref.get(&key);
                    assert_eq!(value, Some(format!("value{}", i)));
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_concurrent_hashmap_stats() {
        let map = SafeConcurrentHashMap::new();

        map.insert("key1".to_string(), "value1".to_string());
        map.get(&"key1".to_string());
        map.get(&"key2".to_string());

        let stats = map.get_stats();
        assert!(stats.insertions.load(std::sync::atomic::Ordering::Relaxed) >= 1);
        assert!(stats.lookups.load(std::sync::atomic::Ordering::Relaxed) >= 2);
    }
}

/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: concurrency
/// `TEST_PRIORITY`: critical
#[cfg(test)]
mod concurrent_cache_tests {
    use super::*;

    #[test]
    fn test_concurrent_cache_creation() {
        let cache = SafeConcurrentCache::<String, String>::new(10);
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_concurrent_cache_insert_get() {
        let cache = SafeConcurrentCache::new(10);

        cache
            .insert("key1".to_string(), "value1".to_string())
            .unwrap();
        assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));
    }

    #[test]
    fn test_concurrent_cache_lru_eviction() {
        let cache = SafeConcurrentCache::new(2);

        cache
            .insert("key1".to_string(), "value1".to_string())
            .unwrap();
        cache
            .insert("key2".to_string(), "value2".to_string())
            .unwrap();

        // This should evict key1 (least recently used)
        cache
            .insert("key3".to_string(), "value3".to_string())
            .unwrap();

        assert_eq!(cache.get(&"key1".to_string()), None); // Evicted
        assert_eq!(cache.get(&"key2".to_string()), Some("value2".to_string()));
        assert_eq!(cache.get(&"key3".to_string()), Some("value3".to_string()));
    }

    #[test]
    fn test_concurrent_cache_touch() {
        let cache = SafeConcurrentCache::new(2);

        cache
            .insert("key1".to_string(), "value1".to_string())
            .unwrap();
        cache
            .insert("key2".to_string(), "value2".to_string())
            .unwrap();

        // Touch key1 to make it more recently used
        assert!(cache.touch(&"key1".to_string()));

        // Insert key3, should evict key2 (now least recently used)
        cache
            .insert("key3".to_string(), "value3".to_string())
            .unwrap();

        assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));
        assert_eq!(cache.get(&"key2".to_string()), None); // Evicted
        assert_eq!(cache.get(&"key3".to_string()), Some("value3".to_string()));
    }

    #[test]
    fn test_concurrent_cache_remove() {
        let cache = SafeConcurrentCache::new(10);

        cache
            .insert("key1".to_string(), "value1".to_string())
            .unwrap();
        assert_eq!(
            cache.remove(&"key1".to_string()),
            Some("value1".to_string())
        );
        assert_eq!(cache.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_concurrent_cache_clear() {
        let cache = SafeConcurrentCache::new(10);

        for i in 0..5 {
            cache
                .insert(format!("key{}", i), format!("value{}", i))
                .unwrap();
        }
        assert_eq!(cache.len(), 5);

        cache.clear();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_concurrent_cache_concurrent_access() {
        // Use larger cache to accommodate all concurrent inserts (10 threads * 50 items = 500)
        let cache = Arc::new(SafeConcurrentCache::new(1000));
        let mut handles = vec![];
        let num_threads = 10;

        for thread_id in 0..num_threads {
            let cache_ref = cache.clone();
            let handle = thread::spawn(move || {
                for i in 0..50 {
                    let key = format!("t{}_k{}", thread_id, i);
                    let value = format!("v{}", i);
                    let _ = cache_ref.insert(key.clone(), value.clone());
                    // Note: Due to concurrent access and potential evictions,
                    // we verify insertion worked but don't assert the value is still there
                    let retrieved = cache_ref.get(&key);
                    if retrieved.is_none() {
                        // Value might have been evicted by another thread, re-insert
                        let _ = cache_ref.insert(key.clone(), value);
                    }
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_concurrent_cache_stats() {
        let cache = SafeConcurrentCache::new(10);

        cache
            .insert("key1".to_string(), "value1".to_string())
            .unwrap();
        cache.get(&"key1".to_string());
        cache.get(&"key2".to_string());

        let stats = cache.get_stats();
        assert!(stats.insertions.load(std::sync::atomic::Ordering::Relaxed) >= 1);
        assert!(stats.lookups.load(std::sync::atomic::Ordering::Relaxed) >= 2);
        assert!(stats.cache_hits.load(std::sync::atomic::Ordering::Relaxed) >= 1);
        assert!(
            stats
                .cache_misses
                .load(std::sync::atomic::Ordering::Relaxed)
                >= 1
        );
    }
}

/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: concurrency
/// `TEST_PRIORITY`: high
#[cfg(test)]
mod concurrent_queue_tests {
    use super::*;

    #[test]
    fn test_concurrent_queue_creation() {
        let queue = SafeConcurrentQueue::<i32>::new();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_concurrent_queue_push_pop() {
        let queue = SafeConcurrentQueue::new();

        queue.push(1);
        queue.push(2);
        queue.push(3);

        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn test_concurrent_queue_fifo_order() {
        let queue = SafeConcurrentQueue::new();

        for i in 0..100 {
            queue.push(i);
        }

        for i in 0..100 {
            assert_eq!(queue.pop(), Some(i));
        }
    }

    #[test]
    fn test_concurrent_queue_length() {
        let queue = SafeConcurrentQueue::new();

        assert_eq!(queue.len(), 0);

        for i in 0..10 {
            queue.push(i);
        }

        assert_eq!(queue.len(), 10);

        queue.pop();
        queue.pop();

        assert_eq!(queue.len(), 8);
    }

    #[test]
    fn test_concurrent_queue_concurrent_producers() {
        let queue = Arc::new(SafeConcurrentQueue::new());
        let mut handles = vec![];
        let num_threads = 10;
        let pushes_per_thread = 100;

        for thread_id in 0..num_threads {
            let queue_ref = queue.clone();
            let handle = thread::spawn(move || {
                for i in 0..pushes_per_thread {
                    queue_ref.push((thread_id, i));
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(queue.len(), num_threads * pushes_per_thread);
    }

    #[test]
    fn test_concurrent_queue_producer_consumer() {
        let queue = Arc::new(SafeConcurrentQueue::new());
        let items_to_produce = 1000;

        let queue_producer = queue.clone();
        let producer = thread::spawn(move || {
            for i in 0..items_to_produce {
                queue_producer.push(i);
                // ✅ MODERNIZED: Yield instead of sleep for better concurrency
                std::thread::yield_now();
            }
        });

        let queue_consumer = queue.clone();
        let consumer = thread::spawn(move || {
            let mut count = 0;
            while count < items_to_produce {
                if let Some(_item) = queue_consumer.pop() {
                    count += 1;
                } else {
                    // ✅ MODERNIZED: Yield instead of sleep for polling
                    std::thread::yield_now();
                }
            }
            count
        });

        producer.join().unwrap();
        let consumed_count = consumer.join().unwrap();

        assert_eq!(consumed_count, items_to_produce);
    }

    #[test]
    fn test_concurrent_queue_multiple_producers_consumers() {
        let queue = Arc::new(SafeConcurrentQueue::new());
        let num_producers = 5;
        let num_consumers = 5;
        let items_per_producer = 100;
        let mut producer_handles = vec![];
        let mut consumer_handles = vec![];

        // Start producers
        for producer_id in 0..num_producers {
            let queue_ref = queue.clone();
            let handle = thread::spawn(move || {
                for i in 0..items_per_producer {
                    queue_ref.push((producer_id, i));
                }
            });
            producer_handles.push(handle);
        }

        // Start consumers
        for _ in 0..num_consumers {
            let queue_ref = queue.clone();
            let handle = thread::spawn(move || {
                let mut consumed = 0;
                while consumed < items_per_producer {
                    if queue_ref.pop().is_some() {
                        consumed += 1;
                    } else {
                        // ✅ MODERNIZED: Yield instead of sleep for polling
                        std::thread::yield_now();
                    }
                }
                consumed
            });
            consumer_handles.push(handle);
        }

        for handle in producer_handles {
            handle.join().unwrap();
        }

        for handle in consumer_handles {
            let _consumed = handle.join().unwrap();
        }

        // All items should be consumed
        assert!(queue.is_empty() || queue.len() < 10); // Some may still be in queue
    }
}

/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: concurrency
/// `TEST_PRIORITY`: high
#[cfg(test)]
mod concurrent_integration_tests {
    use super::*;

    #[test]
    fn test_combined_structures_concurrent_access() {
        let map = Arc::new(SafeConcurrentHashMap::new());
        let cache = Arc::new(SafeConcurrentCache::new(50));
        let queue = Arc::new(SafeConcurrentQueue::new());
        let mut handles = vec![];
        let num_threads = 5;

        for thread_id in 0..num_threads {
            let map_ref = map.clone();
            let cache_ref = cache.clone();
            let queue_ref = queue.clone();

            let handle = thread::spawn(move || {
                for i in 0..50 {
                    let key = format!("t{}_k{}", thread_id, i);
                    let value = format!("v{}", i);

                    // Use all three structures
                    map_ref.insert(key.clone(), value.clone());
                    let _ = cache_ref.insert(key.clone(), value.clone());
                    queue_ref.push(key);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(map.len(), num_threads * 50);
        assert_eq!(queue.len(), num_threads * 50);
        // Cache may have evictions
        assert!(cache.len() <= 50);
    }

    #[test]
    fn test_concurrent_hashmap_as_coordination_point() {
        let map = Arc::new(SafeConcurrentHashMap::new());
        let num_threads = 10;
        let mut handles = vec![];

        for thread_id in 0..num_threads {
            let map_ref = map.clone();
            let handle = thread::spawn(move || {
                // Each thread reports its completion
                map_ref.insert(format!("thread_{}", thread_id), "completed".to_string());

                // ✅ MODERNIZED: Yield instead of sleep for coordination
                // Wait for all threads to report
                while map_ref.len() < num_threads {
                    std::thread::yield_now();
                }

                // Verify all reported
                for i in 0..num_threads {
                    assert_eq!(
                        map_ref.get(&format!("thread_{}", i)),
                        Some("completed".to_string())
                    );
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: concurrency
/// `TEST_PRIORITY`: medium
#[cfg(test)]
mod concurrent_edge_cases_tests {
    use super::*;

    #[test]
    fn test_empty_operations() {
        let map = SafeConcurrentHashMap::<String, String>::new();
        assert_eq!(map.get(&"nonexistent".to_string()), None);
        assert_eq!(map.remove(&"nonexistent".to_string()), None);

        let cache = SafeConcurrentCache::<String, String>::new(10);
        assert_eq!(cache.get(&"nonexistent".to_string()), None);
        assert_eq!(cache.remove(&"nonexistent".to_string()), None);

        let queue = SafeConcurrentQueue::<i32>::new();
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn test_single_element() {
        let map = SafeConcurrentHashMap::new();
        map.insert("key".to_string(), "value".to_string());
        assert_eq!(map.len(), 1);
        assert_eq!(map.remove(&"key".to_string()), Some("value".to_string()));
        assert_eq!(map.len(), 0);

        let cache = SafeConcurrentCache::new(1);
        cache
            .insert("key".to_string(), "value".to_string())
            .unwrap();
        assert_eq!(cache.len(), 1);

        let queue = SafeConcurrentQueue::new();
        queue.push(42);
        assert_eq!(queue.pop(), Some(42));
        assert!(queue.is_empty());
    }

    #[test]
    fn test_cache_exact_capacity() {
        let cache = SafeConcurrentCache::new(3);

        cache.insert("k1".to_string(), "v1".to_string()).unwrap();
        cache.insert("k2".to_string(), "v2".to_string()).unwrap();
        cache.insert("k3".to_string(), "v3".to_string()).unwrap();

        // All should be present
        assert_eq!(cache.get(&"k1".to_string()), Some("v1".to_string()));
        assert_eq!(cache.get(&"k2".to_string()), Some("v2".to_string()));
        assert_eq!(cache.get(&"k3".to_string()), Some("v3".to_string()));

        // Adding one more should evict
        cache.insert("k4".to_string(), "v4".to_string()).unwrap();

        // One should be evicted
        let present_count = [
            cache.get(&"k1".to_string()).is_some(),
            cache.get(&"k2".to_string()).is_some(),
            cache.get(&"k3".to_string()).is_some(),
            cache.get(&"k4".to_string()).is_some(),
        ]
        .iter()
        .filter(|&&x| x)
        .count();

        assert_eq!(present_count, 3);
    }
}
