// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive Concurrent Operations Tests
//!
//! Real-world concurrent scenario tests for thread-safe operations,
//! race condition handling, and performance under concurrent load.
//!
//! Coverage expansion: December 10, 2025

use crate::concurrent_safe::{SafeConcurrentCache, SafeConcurrentHashMap};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use tokio::time::timeout;

// ============================================================================
// Atomic Operations Concurrent Tests (using standard AtomicU64)
// ============================================================================

#[tokio::test]
async fn test_atomic_concurrent_increments() {
    let counter = Arc::new(AtomicU64::new(0));
    let mut handles = vec![];

    // Spawn 100 tasks, each incrementing 100 times
    for _ in 0..100 {
        let counter = counter.clone();
        handles.push(tokio::spawn(async move {
            for _ in 0..100 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }

    // Wait for all tasks
    for handle in handles {
        handle.await.expect("task should complete");
    }

    // Should have exactly 10,000 increments
    assert_eq!(counter.load(Ordering::SeqCst), 10_000);
}

#[tokio::test]
async fn test_atomic_mixed_operations() {
    let counter = Arc::new(AtomicU64::new(1000));
    let mut handles = vec![];

    // Mix of increments and decrements
    for i in 0..100 {
        let counter = counter.clone();
        handles.push(tokio::spawn(async move {
            if i % 2 == 0 {
                counter.fetch_add(1, Ordering::SeqCst);
            } else {
                counter.fetch_sub(1, Ordering::SeqCst);
            }
        }));
    }

    for handle in handles {
        handle.await.expect("task should complete");
    }

    // Should have 50 increments - 50 decrements = 0 change
    assert_eq!(counter.load(Ordering::SeqCst), 1000);
}

// ============================================================================
// SafeConcurrentHashMap Tests
// ============================================================================

#[tokio::test]
async fn test_concurrent_hashmap_parallel_inserts() {
    let map: SafeConcurrentHashMap<String, i32> = SafeConcurrentHashMap::new();
    let map = Arc::new(map);
    let mut handles = vec![];

    // Each task inserts its own unique key
    for i in 0..100 {
        let map = map.clone();
        handles.push(tokio::spawn(async move {
            map.insert(format!("key_{i}"), i);
        }));
    }

    for handle in handles {
        handle.await.expect("task should complete");
    }

    // All 100 keys should be present
    assert_eq!(map.len(), 100);

    // Verify values
    for i in 0..100 {
        assert_eq!(map.get(&format!("key_{i}")), Some(i));
    }
}

#[tokio::test]
async fn test_concurrent_hashmap_read_write_mix() {
    let map: SafeConcurrentHashMap<String, i32> = SafeConcurrentHashMap::new();
    let map = Arc::new(map);

    // Pre-populate
    for i in 0..50 {
        map.insert(format!("key_{i}"), i);
    }

    let mut handles = vec![];

    // Mix of reads and writes
    for i in 0..100 {
        let map = map.clone();
        handles.push(tokio::spawn(async move {
            if i % 2 == 0 {
                // Read operation
                let _ = map.get(&format!("key_{}", i % 50));
            } else {
                // Write operation
                map.insert(format!("new_key_{i}"), i);
            }
        }));
    }

    for handle in handles {
        handle.await.expect("task should complete");
    }

    // Should have original 50 + 50 new keys
    assert_eq!(map.len(), 100);
}

#[tokio::test]
async fn test_concurrent_hashmap_remove_operations() {
    let map: SafeConcurrentHashMap<String, i32> = SafeConcurrentHashMap::new();
    let map = Arc::new(map);

    // Insert 100 items
    for i in 0..100 {
        map.insert(format!("key_{i}"), i);
    }

    let mut handles = vec![];

    // Concurrent removes
    for i in 0..50 {
        let map = map.clone();
        handles.push(tokio::spawn(async move {
            map.remove(&format!("key_{i}"));
        }));
    }

    for handle in handles {
        handle.await.expect("task should complete");
    }

    // Should have 50 items remaining
    assert_eq!(map.len(), 50);
}

// ============================================================================
// SafeConcurrentCache Tests
// ============================================================================

#[tokio::test]
async fn test_safe_cache_concurrent_gets() {
    let cache: SafeConcurrentCache<String, String> = SafeConcurrentCache::new(100);
    let cache = Arc::new(cache);

    // Pre-populate
    for i in 0..50 {
        let _ = cache.insert(format!("key_{i}"), format!("value_{i}"));
    }

    let mut handles = vec![];

    // Many concurrent reads
    for i in 0..200 {
        let cache = cache.clone();
        handles.push(tokio::spawn(async move {
            let key = format!("key_{}", i % 50);
            let _ = cache.get(&key);
        }));
    }

    for handle in handles {
        handle.await.expect("task should complete");
    }

    // Cache should still have all items
    assert_eq!(cache.len(), 50);
}

#[tokio::test]
async fn test_safe_cache_concurrent_inserts() {
    let cache: SafeConcurrentCache<String, String> = SafeConcurrentCache::new(1000);
    let cache = Arc::new(cache);
    let mut handles = vec![];

    // Concurrent writes
    for i in 0..100 {
        let cache = cache.clone();
        handles.push(tokio::spawn(async move {
            let _ = cache.insert(format!("key_{i}"), format!("value_{i}"));
        }));
    }

    for handle in handles {
        handle.await.expect("task should complete");
    }

    assert_eq!(cache.len(), 100);
}

#[tokio::test]
async fn test_safe_cache_eviction_under_pressure() {
    let cache: SafeConcurrentCache<String, String> = SafeConcurrentCache::new(50);
    let cache = Arc::new(cache);
    let mut handles = vec![];

    // Try to insert 200 items concurrently (4x capacity)
    for i in 0..200 {
        let cache = cache.clone();
        handles.push(tokio::spawn(async move {
            let _ = cache.insert(format!("key_{i}"), format!("value_{i}"));
        }));
    }

    for handle in handles {
        handle.await.expect("task should complete");
    }

    // Should respect capacity limit (with some tolerance for eviction timing)
    assert!(
        cache.len() <= 60,
        "Cache should respect capacity with tolerance"
    );
}

// ============================================================================
// Race Condition Tests
// ============================================================================

#[tokio::test]
async fn test_no_race_condition_on_same_key() {
    let map: SafeConcurrentHashMap<String, i32> = SafeConcurrentHashMap::new();
    let map = Arc::new(map);
    let mut handles = vec![];

    // Many tasks writing to same key
    for i in 0..100 {
        let map = map.clone();
        handles.push(tokio::spawn(async move {
            map.insert("shared_key".to_string(), i);
        }));
    }

    for handle in handles {
        handle.await.expect("task should complete");
    }

    // Should have exactly one value (last write wins)
    assert_eq!(map.len(), 1);
    assert!(map.get(&"shared_key".to_string()).is_some());
}

#[tokio::test]
async fn test_atomic_race_condition_safety() {
    let counter = Arc::new(AtomicU64::new(0));
    let mut handles = vec![];

    // Stress test: rapid concurrent modifications
    for _ in 0..1000 {
        let counter = counter.clone();
        handles.push(tokio::spawn(async move {
            counter.fetch_add(1, Ordering::SeqCst);
        }));
    }

    for handle in handles {
        handle.await.expect("task should complete");
    }

    // Must be exactly 1000 (no lost updates)
    assert_eq!(
        counter.load(Ordering::SeqCst),
        1000,
        "Counter should handle all increments atomically"
    );
}

// ============================================================================
// Timeout and Deadlock Prevention Tests
// ============================================================================

#[tokio::test]
async fn test_concurrent_operations_complete_promptly() {
    let map: SafeConcurrentHashMap<String, i32> = SafeConcurrentHashMap::new();
    let map = Arc::new(map);
    let mut handles = vec![];

    for i in 0..50 {
        let map = map.clone();
        handles.push(tokio::spawn(async move {
            map.insert(format!("key_{i}"), i);
        }));
    }

    // All operations should complete within 1 second
    let result = timeout(Duration::from_secs(1), async {
        for handle in handles {
            handle.await.expect("task should complete");
        }
    })
    .await;

    assert!(result.is_ok(), "Operations should complete without timeout");
}

#[tokio::test]
async fn test_no_deadlock_on_high_contention() {
    let map: SafeConcurrentHashMap<String, i32> = SafeConcurrentHashMap::new();
    let map = Arc::new(map);

    // Create high contention on small key set
    let mut handles = vec![];
    for _ in 0..100 {
        let map = map.clone();
        handles.push(tokio::spawn(async move {
            for i in 0..10 {
                map.insert(format!("key_{}", i % 5), i);
            }
        }));
    }

    let result = timeout(Duration::from_secs(2), async {
        for handle in handles {
            handle.await.expect("task should complete");
        }
    })
    .await;

    assert!(result.is_ok(), "Should not deadlock under high contention");
}

// ============================================================================
// Performance Under Load Tests
// ============================================================================

#[tokio::test]
async fn test_throughput_under_concurrent_load() {
    let counter = Arc::new(AtomicU64::new(0));
    let start = std::time::Instant::now();

    let mut handles = vec![];
    for _ in 0..10 {
        let counter = counter.clone();
        handles.push(tokio::spawn(async move {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }

    for handle in handles {
        handle.await.expect("task should complete");
    }

    let duration = start.elapsed();

    assert_eq!(counter.load(Ordering::SeqCst), 10_000);
    assert!(duration.as_millis() < 1000, "Should be fast: {duration:?}");
}

#[tokio::test]
async fn test_cache_performance_with_many_readers() {
    let cache: SafeConcurrentCache<String, String> = SafeConcurrentCache::new(100);
    let cache = Arc::new(cache);

    // Populate cache
    for i in 0..100 {
        let _ = cache.insert(format!("key_{i}"), format!("value_{i}"));
    }

    let start = std::time::Instant::now();
    let mut handles = vec![];

    // Many concurrent readers
    for i in 0..1000 {
        let cache = cache.clone();
        handles.push(tokio::spawn(async move {
            let key = format!("key_{}", i % 100);
            let _ = cache.get(&key);
        }));
    }

    for handle in handles {
        handle.await.expect("task should complete");
    }

    let duration = start.elapsed();

    assert!(
        duration.as_millis() < 2000,
        "Reads should be fast: {duration:?}"
    );
}

// ============================================================================
// Memory Safety Under Concurrent Access
// ============================================================================

#[tokio::test]
async fn test_memory_safety_with_drop() {
    let map: SafeConcurrentHashMap<String, i32> = SafeConcurrentHashMap::new();
    let map = Arc::new(map);

    for i in 0..100 {
        map.insert(format!("key_{i}"), i);
    }

    let mut handles = vec![];

    // Some tasks drop their reference immediately
    for i in 0..50 {
        let map = map.clone();
        handles.push(tokio::spawn(async move {
            let _ = map.get(&format!("key_{i}"));
            drop(map);
        }));
    }

    for handle in handles {
        handle.await.expect("task should complete");
    }

    // Original map should still be valid
    assert_eq!(map.len(), 100);
}

#[tokio::test]
async fn test_concurrent_clear_and_access() {
    let map: SafeConcurrentHashMap<String, i32> = SafeConcurrentHashMap::new();
    let map = Arc::new(map);

    for i in 0..100 {
        map.insert(format!("key_{i}"), i);
    }

    let mut handles = vec![];

    // One task clears, others try to access
    let map_clear = map.clone();
    handles.push(tokio::spawn(async move {
        tokio::task::yield_now().await;
        map_clear.clear();
    }));

    for i in 0..10 {
        let map = map.clone();
        handles.push(tokio::spawn(async move {
            for _ in 0..10 {
                let _ = map.get(&format!("key_{i}"));
                tokio::task::yield_now().await;
            }
        }));
    }

    for handle in handles {
        handle.await.expect("task should complete");
    }

    // Should be cleared by now
    assert_eq!(map.len(), 0);
}
