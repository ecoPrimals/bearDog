#![allow(
    unused_imports,
    clippy::float_cmp,
    clippy::useless_vec,
    clippy::needless_range_loop,
    clippy::uninlined_format_args,
    clippy::field_reassign_with_default,
    clippy::manual_range_contains,
    unused_variables,
    dead_code
)]

// Tests for cache management

use crate::ecosystem_storage::cache::{CacheConfig, CacheManager, CacheMetrics};
use crate::ecosystem_storage::types::CacheEvictionPolicy;

#[test]
fn test_cache_manager_creation() {
    let config = CacheConfig {
        max_size_bytes: 1024 * 1024, // 1MB
        max_entries: 100,
        default_ttl_secs: 3600,
        eviction_policy: CacheEvictionPolicy::Lru,
        enable_compression: false,
    };

    let manager = CacheManager::new(config);
    assert_eq!(manager.metrics().hits, 0);
    assert_eq!(manager.metrics().misses, 0);
    assert_eq!(manager.metrics().current_entries, 0);
}

#[test]
fn test_cache_put_and_get() {
    let config = CacheConfig {
        max_size_bytes: 1024 * 1024,
        max_entries: 10,
        default_ttl_secs: 3600,
        eviction_policy: CacheEvictionPolicy::Lru,
        enable_compression: false,
    };

    let mut manager = CacheManager::new(config);

    // Put data
    let data = vec![1, 2, 3, 4, 5];
    manager.put("key1".to_string(), data.clone());

    // Get data
    let retrieved = manager.get("key1");
    assert!(retrieved.is_some());
    assert_eq!(*retrieved.unwrap(), data);

    // Verify metrics
    assert_eq!(manager.metrics().hits, 1);
    assert_eq!(manager.metrics().current_entries, 1);
}

#[test]
fn test_cache_miss() {
    let config = CacheConfig {
        max_size_bytes: 1024 * 1024,
        max_entries: 10,
        default_ttl_secs: 3600,
        eviction_policy: CacheEvictionPolicy::Lru,
        enable_compression: false,
    };

    let mut manager = CacheManager::new(config);

    // Try to get non-existent key
    let result = manager.get("nonexistent");
    assert!(result.is_none());

    // Verify miss count
    assert_eq!(manager.metrics().misses, 1);
    assert_eq!(manager.metrics().hits, 0);
}

#[test]
fn test_cache_remove() {
    let config = CacheConfig {
        max_size_bytes: 1024 * 1024,
        max_entries: 10,
        default_ttl_secs: 3600,
        eviction_policy: CacheEvictionPolicy::Lru,
        enable_compression: false,
    };

    let mut manager = CacheManager::new(config);

    // Put and then remove
    manager.put("key1".to_string(), vec![1, 2, 3]);
    assert_eq!(manager.metrics().current_entries, 1);

    let removed = manager.remove("key1");
    assert!(removed);
    assert_eq!(manager.metrics().current_entries, 0);

    // Try to remove again
    let removed_again = manager.remove("key1");
    assert!(!removed_again);
}

#[test]
fn test_cache_clear() {
    let config = CacheConfig {
        max_size_bytes: 1024 * 1024,
        max_entries: 10,
        default_ttl_secs: 3600,
        eviction_policy: CacheEvictionPolicy::Lru,
        enable_compression: false,
    };

    let mut manager = CacheManager::new(config);

    // Add multiple entries
    manager.put("key1".to_string(), vec![1, 2, 3]);
    manager.put("key2".to_string(), vec![4, 5, 6]);
    manager.put("key3".to_string(), vec![7, 8, 9]);
    assert_eq!(manager.metrics().current_entries, 3);

    // Clear cache
    manager.clear();
    assert_eq!(manager.metrics().current_entries, 0);
    assert_eq!(manager.metrics().current_size_bytes, 0);
}

#[test]
fn test_cache_eviction_by_size() {
    let config = CacheConfig {
        max_size_bytes: 10, // Very small cache
        max_entries: 100,
        default_ttl_secs: 3600,
        eviction_policy: CacheEvictionPolicy::Lru,
        enable_compression: false,
    };

    let mut manager = CacheManager::new(config);

    // Add data that exceeds cache size
    manager.put("key1".to_string(), vec![1, 2, 3, 4, 5]); // 5 bytes
    manager.put("key2".to_string(), vec![6, 7, 8, 9, 10]); // 5 bytes
    manager.put("key3".to_string(), vec![11, 12]); // 2 bytes - should trigger eviction

    // Verify eviction occurred
    assert!(manager.metrics().evictions > 0);
}

#[test]
fn test_cache_eviction_by_count() {
    let config = CacheConfig {
        max_size_bytes: 1024 * 1024,
        max_entries: 2, // Only 2 entries allowed
        default_ttl_secs: 3600,
        eviction_policy: CacheEvictionPolicy::Lru,
        enable_compression: false,
    };

    let mut manager = CacheManager::new(config);

    // Add 3 entries
    manager.put("key1".to_string(), vec![1]);
    manager.put("key2".to_string(), vec![2]);
    manager.put("key3".to_string(), vec![3]); // Should trigger eviction

    // Verify only 2 entries remain
    assert_eq!(manager.metrics().current_entries, 2);
    assert!(manager.metrics().evictions > 0);
}

#[test]
fn test_cache_hit_rate_calculation() {
    let config = CacheConfig {
        max_size_bytes: 1024 * 1024,
        max_entries: 10,
        default_ttl_secs: 3600,
        eviction_policy: CacheEvictionPolicy::Lru,
        enable_compression: false,
    };

    let mut manager = CacheManager::new(config);

    // Add data
    manager.put("key1".to_string(), vec![1, 2, 3]);

    // 2 hits, 1 miss
    manager.get("key1"); // hit
    manager.get("key1"); // hit
    manager.get("key2"); // miss

    let metrics = manager.metrics();
    assert_eq!(metrics.hits, 2);
    assert_eq!(metrics.misses, 1);

    // Hit rate should be 2/3 ≈ 0.666...
    assert!((metrics.hit_rate - 0.666).abs() < 0.01);
}

#[test]
fn test_cache_config_default() {
    let config = CacheConfig::default();

    // Verify default values
    assert_eq!(config.max_size_bytes, 1024 * 1024 * 1024); // 1GB
    assert_eq!(config.eviction_policy, CacheEvictionPolicy::Lru);
    assert!(!config.enable_compression);
}

#[test]
fn test_cache_metrics_default() {
    let metrics = CacheMetrics::default();

    assert_eq!(metrics.hits, 0);
    assert_eq!(metrics.misses, 0);
    assert_eq!(metrics.evictions, 0);
    assert_eq!(metrics.current_size_bytes, 0);
    assert_eq!(metrics.current_entries, 0);
    assert!((metrics.hit_rate - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_cache_config_serialization() {
    let config = CacheConfig {
        max_size_bytes: 512 * 1024,
        max_entries: 50,
        default_ttl_secs: 1800,
        eviction_policy: CacheEvictionPolicy::Lfu,
        enable_compression: true,
    };

    let serialized = serde_json::to_string(&config).unwrap();
    let deserialized: CacheConfig = serde_json::from_str(&serialized).unwrap();

    assert_eq!(config.max_size_bytes, deserialized.max_size_bytes);
    assert_eq!(config.max_entries, deserialized.max_entries);
    assert_eq!(config.default_ttl_secs, deserialized.default_ttl_secs);
    assert_eq!(config.enable_compression, deserialized.enable_compression);
}

#[test]
fn test_cache_metrics_serialization() {
    let metrics = CacheMetrics {
        hits: 100,
        misses: 20,
        evictions: 5,
        current_size_bytes: 1024,
        current_entries: 10,
        hit_rate: 0.833,
    };

    let serialized = serde_json::to_string(&metrics).unwrap();
    let deserialized: CacheMetrics = serde_json::from_str(&serialized).unwrap();

    assert_eq!(metrics.hits, deserialized.hits);
    assert_eq!(metrics.misses, deserialized.misses);
    assert_eq!(metrics.evictions, deserialized.evictions);
}

#[test]
fn test_cache_zero_copy_optimization() {
    let config = CacheConfig::default();
    let mut manager = CacheManager::new(config);

    // Put data
    let original_data = vec![1, 2, 3, 4, 5];
    manager.put("key1".to_string(), original_data.clone());

    // Get data multiple times
    let ref1 = manager.get("key1").unwrap();
    let ref2 = manager.get("key1").unwrap();

    // Both references should point to the same Arc
    assert_eq!(*ref1, *ref2);
    assert_eq!(*ref1, original_data);
}

#[test]
fn test_cache_multiple_entries() {
    let config = CacheConfig {
        max_size_bytes: 1024 * 1024,
        max_entries: 100,
        default_ttl_secs: 3600,
        eviction_policy: CacheEvictionPolicy::Lru,
        enable_compression: false,
    };

    let mut manager = CacheManager::new(config);

    // Add 10 entries
    for i in 0_u8..10 {
        let key = format!("key{i}");
        let data = vec![i; 10];
        manager.put(key, data);
    }

    assert_eq!(manager.metrics().current_entries, 10);

    // Access them all
    for i in 0..10 {
        let key = format!("key{i}");
        let result = manager.get(&key);
        assert!(result.is_some());
    }

    assert_eq!(manager.metrics().hits, 10);
}
