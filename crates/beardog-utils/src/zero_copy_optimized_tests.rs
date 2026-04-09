// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;

#[test]
fn test_string_optimization() {
    let manager = ZeroCopyManager::new();

    // Common strings should be cached
    let s1 = manager.optimize_string("GET");
    let _s2 = manager.optimize_string("GET");

    assert!(s1.is_optimized());
    assert_eq!(s1.as_str(), "GET");

    // Should have cache hits
    let stats = manager.get_stats();
    assert!(stats.cache_hits.load(std::sync::atomic::Ordering::Relaxed) > 0);
}

#[test]
fn test_zero_copy_builder() {
    let original = "application/json".to_string();
    let optimized = ZeroCopyBuilder::new(original).optimize().build();

    assert_eq!(optimized, "application/json");
}

#[test]
fn test_manager_creation() {
    let manager = ZeroCopyManager::new();
    let stats = manager.get_stats();

    assert_eq!(
        stats
            .clones_avoided
            .load(std::sync::atomic::Ordering::Relaxed),
        0
    );
    assert_eq!(
        stats
            .memory_saved
            .load(std::sync::atomic::Ordering::Relaxed),
        0
    );
}

#[test]
fn test_shared_string_cache() {
    let manager = ZeroCopyManager::new();

    // Get same string twice
    let s1 = manager.get_shared_string("test");
    let s2 = manager.get_shared_string("test");

    // Should be the same Arc (pointer equality)
    assert!(Arc::ptr_eq(&s1, &s2));

    // Should have cache hit
    let stats = manager.get_stats();
    assert_eq!(
        stats.cache_hits.load(std::sync::atomic::Ordering::Relaxed),
        1
    );
}

#[test]
fn test_optimize_string_common() {
    let manager = ZeroCopyManager::new();

    // Test common HTTP method
    let opt = manager.optimize_string("GET");
    assert!(opt.is_optimized());
    assert_eq!(opt.as_str(), "GET");

    // Test common content type
    let opt2 = manager.optimize_string("application/json");
    assert!(opt2.is_optimized());
}

#[test]
fn test_optimize_string_small() {
    let manager = ZeroCopyManager::new();

    // Small, uncommon string should not be optimized
    let opt = manager.optimize_string("xyz");
    assert!(!opt.is_optimized()); // Should be owned
    assert_eq!(opt.as_str(), "xyz");
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_optimize_string_large() {
    let manager = ZeroCopyManager::new();

    // Large string should be optimized
    let large_str = "a".repeat(2000);
    let opt = manager.optimize_string(&large_str);
    assert!(opt.is_optimized()); // Should be shared
    assert_eq!(opt.as_str().len(), 2000);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_optimize_bytes_small() {
    let manager = ZeroCopyManager::new();

    // Small byte array
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let data = vec![1, 2, 3, 4, 5];
    let opt = manager.optimize_bytes(&data);
    assert!(!opt.is_optimized()); // Should be owned
    assert_eq!(opt.as_slice(), &[1, 2, 3, 4, 5]);
}

#[test]
fn test_optimize_bytes_large() {
    let manager = ZeroCopyManager::new();

    // Large byte array (> 4KB)
    let data = vec![0u8; 5000];
    let opt = manager.optimize_bytes(&data);
    assert!(opt.is_optimized()); // Should be shared
    assert_eq!(opt.as_slice().len(), 5000);
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[test]
fn test_optimized_string_display() {
    let manager = ZeroCopyManager::new();
    let opt = manager.optimize_string("test");

    assert_eq!(format!("{opt}"), "test");
}

#[test]
fn test_optimized_string_into_string() {
    let manager = ZeroCopyManager::new();
    let opt = manager.optimize_string("test");
    let s = opt.into_string();

    assert_eq!(s, "test");
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_optimized_bytes_to_vec() {
    let manager = ZeroCopyManager::new();
    let data = vec![1, 2, 3];
    let opt = manager.optimize_bytes(&data);
    let vec = opt.to_vec();

    assert_eq!(vec, vec![1, 2, 3]);
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[test]
fn test_cleanup_expired() {
    let manager = ZeroCopyManager::new();

    // Add some strings
    let _s1 = manager.get_shared_string("test1");
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let _s2 = manager.get_shared_string("test2");

    // Cleanup (should not remove anything as strings are still referenced)
    manager.cleanup_expired();

    // Stats should remain
    let stats = manager.get_stats();
    assert!(
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        stats
            .cache_misses
            .load(std::sync::atomic::Ordering::Relaxed)
            >= 2
    );
}

#[test]
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_is_common_string() {
    let manager = ZeroCopyManager::new();

    // HTTP methods
    assert!(manager.is_common_string("GET"));
    assert!(manager.is_common_string("POST"));

    // Content types
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(manager.is_common_string("application/json"));
    assert!(manager.is_common_string("text/plain"));

    // Network
    assert!(manager.is_common_string("localhost"));
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(manager.is_common_string("127.0.0.1"));

    // Not common
    assert!(!manager.is_common_string("random_string_12345"));
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_optimization_report() {
    let manager = ZeroCopyManager::new();

    // Generate some activity
    let _s1 = manager.get_shared_string("test");
    let _s2 = manager.get_shared_string("test");
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    // Should not panic
    manager.print_optimization_report();
}

#[test]
fn test_zero_copy_builder_not_optimized() {
    let original = "abc".to_string(); // Small, not common
    let builder = ZeroCopyBuilder::new(original);

    assert!(!builder.is_optimized());
}

#[test]
fn test_zero_copy_builder_already_optimized() {
    let original = "ab".to_string(); // Very small, considered optimized
    let builder = ZeroCopyBuilder::new(original).optimize();

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Build returns the inner value
    let result = builder.build();
    assert_eq!(result, "ab");
}

#[test]
fn test_string_zero_copy_optimized_trait() {
    let manager = ZeroCopyManager::new();
    let original = "test".to_string();

    let optimized = original.optimize(&manager);
    assert_eq!(optimized, "test");
}

#[test]
fn test_vec_zero_copy_optimized_trait() {
    let manager = ZeroCopyManager::new();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let original = vec![1u8, 2, 3];

    let optimized = original.optimize(&manager);
    assert_eq!(optimized, vec![1, 2, 3]);
}

#[test]
fn test_global_zero_copy_manager() {
    let manager1 = global_zero_copy_manager();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let manager2 = global_zero_copy_manager();

    // Should be the same instance
    assert!(std::ptr::eq(manager1, manager2));
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[test]
fn test_shared_string_convenience() {
    let s = shared_string("test");
    assert_eq!(s.as_ref(), "test");
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_optimize_string_convenience() {
    let opt = optimize_string("test");
    assert_eq!(opt.as_str(), "test");
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_optimize_bytes_convenience() {
    let opt = optimize_bytes(&[1, 2, 3]);
    assert_eq!(opt.as_slice(), &[1, 2, 3]);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_memory_saved_tracking() {
    let manager = ZeroCopyManager::new();

    // Get same string twice
    let s1 = manager.get_shared_string("test_string");
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let _s2 = manager.get_shared_string("test_string");

    // Should track memory saved
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let stats = manager.get_stats();
    assert!(
        stats
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            .memory_saved
            .load(std::sync::atomic::Ordering::Relaxed)
            >= s1.len() as u64 // TEST_CATEGORY: unit
                               // TEST_DOMAIN: core
                               // TEST_PRIORITY: normal
    );
}

#[test]
fn test_stats_cloning() {
    let manager = ZeroCopyManager::new();
    let stats1 = manager.get_stats();
    let stats2 = manager.get_stats();

    // Both should point to same stats
    assert!(Arc::ptr_eq(&stats1, &stats2));
}

#[test]
fn test_optimized_string_as_str_shared() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let manager = ZeroCopyManager::new();
    let opt = OptimizedString::Shared(manager.get_shared_string("shared"));
    assert_eq!(opt.as_str(), "shared");
}

#[test]
fn test_optimized_string_as_str_owned() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let opt = OptimizedString::Owned("owned".to_string());
    assert_eq!(opt.as_str(), "owned");
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_optimized_bytes_as_slice_shared() {
    let bytes: Arc<[u8]> = Arc::from(&[1, 2, 3][..]);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let opt = OptimizedBytes::Shared(bytes);
    assert_eq!(opt.as_slice(), &[1, 2, 3]);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_optimized_bytes_as_slice_owned() {
    let opt = OptimizedBytes::Owned(vec![4, 5, 6]);
    assert_eq!(opt.as_slice(), &[4, 5, 6]);
}
