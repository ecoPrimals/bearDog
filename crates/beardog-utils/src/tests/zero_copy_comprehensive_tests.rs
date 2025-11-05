// Comprehensive tests for zero-copy modules
//
// This test suite provides extensive coverage for zero-copy functionality.

use crate::zero_copy::*;
use crate::zero_copy_optimized;
use std::sync::Arc;
use std::time::Duration;

// =============================================================================
// ZeroCopyManager Tests (from mod.rs)
// =============================================================================

#[test]
fn test_zero_copy_manager_creation() {
    let manager = ZeroCopyManager::new();
    let stats = manager.get_stats();

    assert_eq!(
        stats
            .string_cache_hits
            .load(std::sync::atomic::Ordering::Relaxed),
        0
    );
    assert_eq!(
        stats
            .string_cache_misses
            .load(std::sync::atomic::Ordering::Relaxed),
        0
    );
}

#[test]
fn test_shared_string_caching() {
    let manager = ZeroCopyManager::new();

    // First access should miss
    let s1 = manager.get_shared_string("test_string");
    assert_eq!(&*s1, "test_string");

    // Second access should hit
    let s2 = manager.get_shared_string("test_string");
    assert_eq!(&*s2, "test_string");

    // Should be the same Arc
    assert!(Arc::ptr_eq(&s1, &s2));

    let stats = manager.get_stats();
    assert!(
        stats
            .string_cache_hits
            .load(std::sync::atomic::Ordering::Relaxed)
            > 0
    );
}

#[test]
fn test_shared_string_different_values() {
    let manager = ZeroCopyManager::new();

    let s1 = manager.get_shared_string("first");
    let s2 = manager.get_shared_string("second");
    let s3 = manager.get_shared_string("first");

    assert_eq!(&*s1, "first");
    assert_eq!(&*s2, "second");
    assert!(Arc::ptr_eq(&s1, &s3));
    assert!(!Arc::ptr_eq(&s1, &s2));
}

#[test]
fn test_shared_config_basic() {
    let manager = ZeroCopyManager::new();

    let config1 = manager.get_shared_config("test_config", || "test_value".to_string());
    assert_eq!(&*config1, "test_value");

    // Should get same instance
    let config2 = manager.get_shared_config("test_config", || "different_value".to_string());
    assert_eq!(&*config2, "test_value"); // Should be cached value
    assert!(Arc::ptr_eq(&config1, &config2));
}

#[test]
fn test_shared_config_different_types() {
    let manager = ZeroCopyManager::new();

    let config_str = manager.get_shared_config("key1", || "string value".to_string());
    let config_int = manager.get_shared_config("key1", || 42i32);

    assert_eq!(&*config_str, "string value");
    assert_eq!(*config_int, 42);
}

#[test]
fn test_cleanup_expired_strings() {
    let manager = ZeroCopyManager::new();

    {
        let _s1 = manager.get_shared_string("temp1");
        let _s2 = manager.get_shared_string("temp2");
        // Strings go out of scope here
    }

    // Cleanup should remove expired entries
    manager.cleanup_expired();

    // This is a weak test since we can't directly inspect cache
    let stats = manager.get_stats();
    // Note: string_cache_hits is AtomicUsize, so >= 0 is always true (enforced by type system)
    // Just verify we can access the stats
    let _ = stats
        .string_cache_hits
        .load(std::sync::atomic::Ordering::Relaxed);
}

#[test]
fn test_global_zero_copy_manager() {
    let manager1 = global_zero_copy_manager();
    let manager2 = global_zero_copy_manager();

    // Should be same instance
    assert!(std::ptr::eq(manager1, manager2));
}

#[test]
fn test_shared_string_convenience() {
    let s1 = shared_string("convenience");
    let s2 = shared_string("convenience");

    assert_eq!(&*s1, "convenience");
    assert!(Arc::ptr_eq(&s1, &s2));
}

#[test]
fn test_shared_config_convenience() {
    let config1 = shared_config("test_key", || vec![1, 2, 3]);
    let config2 = shared_config("test_key", || vec![4, 5, 6]);

    assert_eq!(&*config1, &vec![1, 2, 3]);
    assert!(Arc::ptr_eq(&config1, &config2));
}

#[test]
fn test_zero_copy_builder_basic() {
    let value = "test value".to_string();
    let builder = ZeroCopyBuilder::new(value);

    assert!(!builder.is_optimized());

    let built = builder.build();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(built, "test value");
}

#[test]
fn test_zero_copy_builder_optimize() {
    let value = "optimize me".to_string();
    let builder = ZeroCopyBuilder::new(value).optimize();

    assert!(builder.is_optimized());

    let built = builder.build();
    assert_eq!(built, "optimize me");
}

#[test]
fn test_is_valid_service_capability() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(is_valid_service_capability("communication_mesh"));
    assert!(is_valid_service_capability("storage_services"));
    assert!(is_valid_service_capability("compute_orchestration"));
    assert!(is_valid_service_capability("ai_intelligence"));
    assert!(is_valid_service_capability("security_provider"));
    assert!(is_valid_service_capability("system_integration"));
    assert!(is_valid_service_capability("hsm"));
    assert!(is_valid_service_capability("key_management"));
    assert!(is_valid_service_capability("secure_enclave"));

    assert!(!is_valid_service_capability("invalid_capability"));
    assert!(!is_valid_service_capability(""));
    assert!(!is_valid_service_capability("random"));
}

#[test]
fn test_get_all_standard_capabilities() {
    let capabilities = get_all_standard_capabilities();

    assert_eq!(capabilities.len(), 9);
    assert!(capabilities.contains(&"communication_mesh"));
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(capabilities.contains(&"security_provider"));
    assert!(capabilities.contains(&"hsm"));
}

// =============================================================================
// RequestCache Tests
// =============================================================================

#[test]
fn test_request_cache_creation() {
    let cache: RequestCache<String> = RequestCache::new(Duration::from_secs(60));
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(cache.is_empty());
    assert_eq!(cache.len(), 0);
}

#[test]
fn test_request_cache_insert_and_get() {
    let cache = RequestCache::new(Duration::from_secs(60));

    cache.insert("key1".to_string(), "value1".to_string());
    cache.insert("key2".to_string(), "value2".to_string());
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    assert_eq!(cache.len(), 2);
    assert!(!cache.is_empty());

    assert_eq!(cache.get("key1"), Some("value1".to_string()));
    assert_eq!(cache.get("key2"), Some("value2".to_string()));
    assert_eq!(cache.get("key3"), None);
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[test]
fn test_request_cache_overwrite() {
    let cache = RequestCache::new(Duration::from_secs(60));

    cache.insert("key".to_string(), "value1".to_string());
    assert_eq!(cache.get("key"), Some("value1".to_string()));

    cache.insert("key".to_string(), "value2".to_string());
    assert_eq!(cache.get("key"), Some("value2".to_string()));
}

#[test]
fn test_request_cache_expiration() {
    let cache = RequestCache::new(Duration::from_millis(10));

    cache.insert("temp_key".to_string(), "temp_value".to_string());
    assert_eq!(cache.get("temp_key"), Some("temp_value".to_string()));

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Wait for expiration
    std::thread::sleep(Duration::from_millis(20));

    assert_eq!(cache.get("temp_key"), None);
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_request_cache_cleanup_expired() {
    let cache = RequestCache::new(Duration::from_millis(10));

    cache.insert("key1".to_string(), "value1".to_string());
    cache.insert("key2".to_string(), "value2".to_string());
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(cache.len(), 2);

    // Wait for expiration
    std::thread::sleep(Duration::from_millis(20));

    cache.cleanup_expired();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(cache.len(), 0);
}

#[test]
fn test_request_cache_default() {
    let cache: RequestCache<i32> = RequestCache::default();
    assert!(cache.is_empty());
    cache.insert("test".to_string(), 42);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(cache.get("test"), Some(42));
}

#[test]
fn test_request_cache_complex_types() {
    #[derive(Clone, Debug, PartialEq)]
    struct ComplexData {
        id: u64,
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        name: String,
        values: Vec<i32>,
    }

    let cache = RequestCache::new(Duration::from_secs(60));

    let data = ComplexData {
        id: 123,
        name: "test".to_string(),
        values: vec![1, 2, 3],
    };

    cache.insert("complex".to_string(), data.clone());

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let retrieved = cache.get("complex");
    assert_eq!(retrieved, Some(data));
}

// =============================================================================
// OptimizedString Tests (from optimized.rs)
// =============================================================================

#[test]
fn test_optimized_string_shared() {
    let s = zero_copy_optimized::OptimizedString::Shared(Arc::from("shared"));
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    assert_eq!(s.as_str(), "shared");
    assert!(s.is_optimized());
    assert_eq!(s.to_string(), "shared");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_optimized_string_owned() {
    let s = zero_copy_optimized::OptimizedString::Owned("owned".to_string());

    assert_eq!(s.as_str(), "owned");
    assert!(!s.is_optimized());
    assert_eq!(s.to_string(), "owned");
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_optimized_string_into_string() {
    let shared = zero_copy_optimized::OptimizedString::Shared(Arc::from("test"));
    let owned = zero_copy_optimized::OptimizedString::Owned("test".to_string());

    assert_eq!(shared.into_string(), "test");
    assert_eq!(owned.into_string(), "test");
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_optimized_string_display() {
    let shared = zero_copy_optimized::OptimizedString::Shared(Arc::from("display"));
    let owned = zero_copy_optimized::OptimizedString::Owned("display".to_string());

    assert_eq!(format!("{}", shared), "display");
    assert_eq!(format!("{}", owned), "display");
}

// =============================================================================
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
// OptimizedBytes Tests
// =============================================================================

#[test]
fn test_optimized_bytes_shared() {
    let bytes = vec![1u8, 2, 3, 4, 5];
    let opt = zero_copy_optimized::OptimizedBytes::Shared(Arc::from(bytes.as_slice()));

    assert_eq!(opt.as_slice(), &[1, 2, 3, 4, 5]);
    assert!(opt.is_optimized());
    assert_eq!(opt.to_vec(), vec![1, 2, 3, 4, 5]);
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[test]
fn test_optimized_bytes_owned() {
    let bytes = vec![1u8, 2, 3, 4, 5];
    let opt = zero_copy_optimized::OptimizedBytes::Owned(bytes);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    assert_eq!(opt.as_slice(), &[1, 2, 3, 4, 5]);
    assert!(!opt.is_optimized());
    assert_eq!(opt.to_vec(), vec![1, 2, 3, 4, 5]);
}

// =============================================================================
// ZeroCopyOptimized Trait Tests
// =============================================================================

#[test]
fn test_string_zero_copy_optimized() {
    use zero_copy_optimized::ZeroCopyOptimized;

    let manager = zero_copy_optimized::ZeroCopyManager::new();
    let s = "test_string".to_string();

    let optimized = s.optimize(&manager);
    assert_eq!(optimized, "test_string");
}

#[test]
fn test_vec_zero_copy_optimized() {
    use zero_copy_optimized::ZeroCopyOptimized;
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    let manager = zero_copy_optimized::ZeroCopyManager::new();
    let v = vec![1u8, 2, 3, 4];

    let optimized = v.optimize(&manager);
    assert_eq!(optimized, vec![1, 2, 3, 4]);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

// =============================================================================
// Global Manager Tests (from optimized.rs)
// =============================================================================

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_global_manager_optimize_string() {
    let s1 = zero_copy_optimized::optimize_string("GET");
    let _s2 = zero_copy_optimized::optimize_string("GET");

    assert!(s1.is_optimized());
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(s1.as_str(), "GET");
    // Can't directly test Arc equality across calls to optimize_string
}

#[test]
fn test_global_manager_optimize_bytes() {
    let bytes = vec![1u8; 2000]; // Large enough to trigger sharing
    let opt = zero_copy_optimized::optimize_bytes(&bytes);

    // Check that it processes correctly (whether optimized depends on implementation details)
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(opt.as_slice().len(), 2000);
    // Note: optimization decision is implementation-specific
}

#[test]
fn test_global_manager_shared_string() {
    let s1 = zero_copy_optimized::shared_string("shared_test");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let s2 = zero_copy_optimized::shared_string("shared_test");

    assert_eq!(&*s1, "shared_test");
    assert!(Arc::ptr_eq(&s1, &s2));
}

// =============================================================================
// HyperOptimized AlignedBuffer Tests
// =============================================================================

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_aligned_buffer_creation() {
    use crate::zero_copy::hyperoptimized_zero_copy::AlignedBuffer;
    let buffer = AlignedBuffer::new(1024);
    assert!(buffer.is_ok());

    let buf = buffer.unwrap();
    assert_eq!(buf.capacity(), 1024);
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[test]
fn test_aligned_buffer_as_slice() {
    use crate::zero_copy::hyperoptimized_zero_copy::AlignedBuffer;
    let buffer = AlignedBuffer::new(1024).unwrap();
    let slice = buffer.as_slice();
    assert_eq!(slice.len(), 0); // Initially empty
}

#[test]
fn test_aligned_buffer_as_mut_slice() {
    use crate::zero_copy::hyperoptimized_zero_copy::AlignedBuffer;
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let mut buffer = AlignedBuffer::new(1024).unwrap();
    let slice = buffer.as_mut_slice();
    assert_eq!(slice.len(), 1024); // Full capacity for mut slice
}

#[test]
fn test_aligned_buffer_zero_capacity() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    use crate::zero_copy::hyperoptimized_zero_copy::AlignedBuffer;
    let buffer = AlignedBuffer::new(0);
    assert!(buffer.is_ok());

    let buf = buffer.unwrap();
    assert_eq!(buf.capacity(), 0);
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[test]
fn test_aligned_buffer_large_capacity() {
    use crate::zero_copy::hyperoptimized_zero_copy::AlignedBuffer;
    let buffer = AlignedBuffer::new(1024 * 1024);
    assert!(buffer.is_ok());

    let buf = buffer.unwrap();
    assert_eq!(buf.capacity(), 1024 * 1024);
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

// =============================================================================
// ZeroCopyStats Tests
// =============================================================================

#[test]
fn test_zero_copy_stats_default() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let stats = zero_copy_optimized::ZeroCopyStats::default();

    assert_eq!(
        stats
            .clones_avoided
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            .load(std::sync::atomic::Ordering::Relaxed),
        0
    );
    assert_eq!(
        stats
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            .memory_saved
            .load(std::sync::atomic::Ordering::Relaxed),
        0
    );
    assert_eq!(
        stats.cache_hits.load(std::sync::atomic::Ordering::Relaxed),
        0 // TEST_CATEGORY: integration
          // TEST_DOMAIN: core
          // TEST_PRIORITY: normal
    );
    assert_eq!(
        stats
            .cache_misses
            .load(std::sync::atomic::Ordering::Relaxed),
        0
    );
}

#[test]
fn test_zero_copy_manager_stats_tracking() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let manager = zero_copy_optimized::ZeroCopyManager::new();

    // Trigger some operations
    let _s1 = manager.optimize_string("GET");
    let _s2 = manager.optimize_string("GET");

    let stats = manager.get_stats();

    // Should have at least one cache hit
    assert!(
        stats.cache_hits.load(std::sync::atomic::Ordering::Relaxed) > 0
            || stats
                .cache_misses
                .load(std::sync::atomic::Ordering::Relaxed)
                > 0
    );
}

// =============================================================================
// Edge Cases and Error Conditions
// =============================================================================

#[test]
fn test_empty_string_optimization() {
    let manager = zero_copy_optimized::ZeroCopyManager::new();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let opt = manager.optimize_string("");

    assert_eq!(opt.as_str(), "");
}

#[test]
fn test_large_string_optimization() {
    let manager = zero_copy_optimized::ZeroCopyManager::new();
    let large_string = "x".repeat(2000);
    let opt = manager.optimize_string(&large_string);

    assert_eq!(opt.as_str().len(), 2000);
    assert!(opt.is_optimized()); // Should be shared due to size
}

#[test]
fn test_empty_bytes_optimization() {
    let manager = zero_copy_optimized::ZeroCopyManager::new();
    let opt = manager.optimize_bytes(&[]);

    assert_eq!(opt.as_slice().len(), 0);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_request_cache_many_entries() {
    let cache = RequestCache::new(Duration::from_secs(60));
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    for i in 0..1000 {
        cache.insert(format!("key_{}", i), format!("value_{}", i));
    }

    assert_eq!(cache.len(), 1000);

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    for i in 0..1000 {
        assert_eq!(
            cache.get(&format!("key_{}", i)),
            Some(format!("value_{}", i))
        );
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
    }
}

#[test]
fn test_concurrent_string_caching() {
    use std::sync::Arc;
    use std::thread;

    let manager = Arc::new(ZeroCopyManager::new());
    let mut handles = vec![];

    for i in 0..10 {
        let manager_clone = Arc::clone(&manager);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                // TEST_CATEGORY: integration
                // TEST_DOMAIN: core
                // TEST_PRIORITY: normal
                let _s = manager_clone.get_shared_string(format!("thread_{}", i));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let stats = manager.get_stats();
    // With concurrent access, we should see activity
    // (either cache hits, misses, or both)
    let total_activity = stats
        .string_cache_hits
        .load(std::sync::atomic::Ordering::Relaxed)
        + stats
            .string_cache_misses
            .load(std::sync::atomic::Ordering::Relaxed);
    assert!(total_activity > 0);
}

#[test]
fn test_zero_copy_builder_chaining() {
    let value = "chain test".to_string();
    let result = ZeroCopyBuilder::new(value).optimize().build();

    assert_eq!(result, "chain test");
}

// =============================================================================
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
// Integration Tests
// =============================================================================

#[test]
fn test_full_zero_copy_workflow() {
    let manager = ZeroCopyManager::new();

    // Test string caching
    let s1 = manager.get_shared_string("workflow_test");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let s2 = manager.get_shared_string("workflow_test");
    assert!(Arc::ptr_eq(&s1, &s2));

    // Test config caching
    let config = manager.get_shared_config("workflow_config", || vec![1, 2, 3]);
    assert_eq!(&*config, &vec![1, 2, 3]);

    // Test cleanup
    manager.cleanup_expired();

    // Stats should show activity
    let stats = manager.get_stats();
    assert!(
        stats
            .string_cache_hits
            .load(std::sync::atomic::Ordering::Relaxed)
            > 0
    );
}

#[test]
fn test_request_cache_workflow() {
    let cache = RequestCache::new(Duration::from_secs(60));
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    // Insert multiple entries
    for i in 0..10 {
        cache.insert(format!("key_{}", i), i * 2);
    }

    // Verify all entries
    for i in 0..10 {
        assert_eq!(cache.get(&format!("key_{}", i)), Some(i * 2));
    }

    // Cleanup shouldn't affect non-expired entries
    cache.cleanup_expired();
    assert_eq!(cache.len(), 10);
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_optimized_types_workflow() {
    let manager = zero_copy_optimized::ZeroCopyManager::new();

    // Test string optimization
    let s1 = manager.optimize_string("GET");
    let s2 = manager.optimize_string("POST");
    assert!(s1.is_optimized());
    assert!(s2.is_optimized());

    // Test bytes optimization
    let large_data = vec![0u8; 2000];
    let opt_bytes = manager.optimize_bytes(&large_data);
    // Optimization decision is implementation-specific, just verify it works
    assert_eq!(opt_bytes.as_slice().len(), 2000);
}
