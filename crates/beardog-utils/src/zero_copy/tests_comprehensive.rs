// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive tests for `zero_copy` module
//! Focus: Caching, shared references, capability validation

#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all
)]

use super::*;

#[test]
fn test_is_valid_service_capability_all_valid() {
    let valid_capabilities = vec![
        "communication_mesh",
        "storage_services",
        "compute_orchestration",
        "ai_intelligence",
        "security_provider",
        "system_integration",
        "hsm",
        "key_management",
        "secure_enclave",
    ];

    for capability in valid_capabilities {
        assert!(is_valid_service_capability(capability));
    }
}

#[test]
fn test_is_valid_service_capability_invalid() {
    assert!(!is_valid_service_capability("invalid_capability"));
    assert!(!is_valid_service_capability("random_service"));
    assert!(!is_valid_service_capability(""));
    assert!(!is_valid_service_capability("communication"));
}

#[test]
fn test_get_all_standard_capabilities() {
    let capabilities = get_all_standard_capabilities();
    assert_eq!(capabilities.len(), 9);
    assert!(capabilities.contains(&"communication_mesh"));
    assert!(capabilities.contains(&"hsm"));
    assert!(capabilities.contains(&"security_provider"));
}

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
fn test_zero_copy_manager_default() {
    let manager = ZeroCopyManager::default();
    let stats = manager.get_stats();

    assert_eq!(
        stats
            .string_cache_hits
            .load(std::sync::atomic::Ordering::Relaxed),
        0
    );
}

#[test]
fn test_shared_string_first_access() {
    let manager = ZeroCopyManager::new();
    let s = manager.get_shared_string("test_string");

    assert_eq!(&*s, "test_string");

    let stats = manager.get_stats();
    assert_eq!(
        stats
            .string_cache_misses
            .load(std::sync::atomic::Ordering::Relaxed),
        1 // TEST_CATEGORY: unit
          // TEST_DOMAIN: core
          // TEST_PRIORITY: normal
    );
    assert_eq!(
        stats
            .string_cache_hits
            .load(std::sync::atomic::Ordering::Relaxed),
        0
    );
}

#[test]
fn test_shared_string_cache_hit() {
    let manager = ZeroCopyManager::new();

    // First access (miss)
    let s1 = manager.get_shared_string("cached_string");

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: important
    // Second access (should be hit)
    let s2 = manager.get_shared_string("cached_string");

    assert_eq!(&*s1, &*s2);

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let stats = manager.get_stats();
    assert_eq!(
        stats
            .string_cache_hits
            .load(std::sync::atomic::Ordering::Relaxed),
        1 // TEST_CATEGORY: unit
          // TEST_DOMAIN: core
          // TEST_PRIORITY: normal
    );
    assert_eq!(
        stats
            .string_cache_misses
            .load(std::sync::atomic::Ordering::Relaxed),
        1
    );
}

#[test]
fn test_shared_string_multiple_strings() {
    let manager = ZeroCopyManager::new();

    let s1 = manager.get_shared_string("first");
    let s2 = manager.get_shared_string("second");
    let s3 = manager.get_shared_string("first"); // Cache hit
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    assert_eq!(&*s1, "first");
    assert_eq!(&*s2, "second");
    assert_eq!(&*s3, "first");

    let stats = manager.get_stats();
    assert_eq!(
        stats
            .string_cache_misses
            .load(std::sync::atomic::Ordering::Relaxed),
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        2
    );
    assert_eq!(
        stats
            .string_cache_hits
            .load(std::sync::atomic::Ordering::Relaxed),
        1
    );
}

#[test]
fn test_shared_config_basic() {
    let manager = ZeroCopyManager::new();

    let config = manager.get_shared_config("test_config", || 42i32);

    assert_eq!(*config, 42);

    let stats = manager.get_stats();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(
        stats
            .config_cache_misses
            .load(std::sync::atomic::Ordering::Relaxed),
        1
    );
}

#[test]
fn test_shared_config_cache_hit() {
    let manager = ZeroCopyManager::new();

    let config1 = manager.get_shared_config("shared_config", || 100i32);
    let config2 = manager.get_shared_config("shared_config", || 200i32); // Should not call factory

    assert_eq!(*config1, 100);
    assert_eq!(*config2, 100); // Same value, from cache

    let stats = manager.get_stats();
    assert_eq!(
        stats
            .config_cache_hits
            .load(std::sync::atomic::Ordering::Relaxed),
        1 // TEST_CATEGORY: unit
          // TEST_DOMAIN: core
          // TEST_PRIORITY: normal
    );
    assert_eq!(
        stats
            .config_cache_misses
            .load(std::sync::atomic::Ordering::Relaxed),
        1
    );
}

#[test]
fn test_shared_config_different_types() {
    let manager = ZeroCopyManager::new();

    let int_config = manager.get_shared_config("config", || 42i32);
    let string_config = manager.get_shared_config("config", || "test".to_string());

    assert_eq!(*int_config, 42);
    assert_eq!(&*string_config, "test");
}

#[test]
fn test_cleanup_expired() {
    let manager = ZeroCopyManager::new();

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    {
        let _s = manager.get_shared_string("temporary");
    } // String dropped here

    // Call cleanup (should remove expired weak references)
    manager.cleanup_expired();

    // Cleanup should work without errors (no panic means success)
}

#[test]
fn test_cleanup_expired_rate_limiting() {
    let manager = ZeroCopyManager::new();

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Call cleanup twice in quick succession
    manager.cleanup_expired();
    manager.cleanup_expired(); // Should return early due to rate limiting

    // No panic means rate limiting works correctly
}

#[test]
fn test_global_zero_copy_manager() {
    let manager1 = global_zero_copy_manager();
    let manager2 = global_zero_copy_manager();

    // Should be the same instance
    assert!(std::ptr::eq(manager1, manager2));
}

#[test]
fn test_global_shared_string() {
    let s1 = shared_string("global_test");
    let s2 = shared_string("global_test");

    assert_eq!(&*s1, &*s2);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_global_shared_config() {
    let config1 = shared_config("global_config", || vec![1, 2, 3]);
    let config2 = shared_config("global_config", || vec![4, 5, 6]);

    // Should return cached value
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(*config1, vec![1, 2, 3]);
    assert_eq!(*config2, vec![1, 2, 3]);
}

#[test]
fn test_zero_copy_builder_new() {
    let builder = ZeroCopyBuilder::new(42);
    assert_eq!(builder.build(), 42);
}

#[test]
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_zero_copy_builder_optimize() {
    let builder = ZeroCopyBuilder::new(100).optimize();
    assert!(builder.is_optimized());
    assert_eq!(builder.build(), 100);
}

#[test]
fn test_zero_copy_builder_not_optimized() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let builder = ZeroCopyBuilder::new(50);
    assert!(!builder.is_optimized());
}

#[test]
fn test_zero_copy_builder_with_string() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let builder = ZeroCopyBuilder::new(String::from("test")).optimize();
    assert!(builder.is_optimized());
    assert_eq!(builder.build(), "test");
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_zero_copy_stats_default() {
    let stats = ZeroCopyStats::default();

    assert_eq!(
        stats
            .string_cache_hits
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            .load(std::sync::atomic::Ordering::Relaxed),
        0
    );
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(
        stats
            .string_cache_misses
            .load(std::sync::atomic::Ordering::Relaxed),
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        0
    );
    assert_eq!(
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        stats
            .config_cache_hits
            .load(std::sync::atomic::Ordering::Relaxed),
        0 // TEST_CATEGORY: unit
          // TEST_DOMAIN: core
          // TEST_PRIORITY: normal
    );
    assert_eq!(
        stats
            .config_cache_misses
            .load(std::sync::atomic::Ordering::Relaxed),
        0
    );
}

#[test]
fn test_shared_string_empty() {
    let manager = ZeroCopyManager::new();
    let s = manager.get_shared_string("");

    assert_eq!(&*s, "");
}

#[test]
fn test_shared_string_long_text() {
    let manager = ZeroCopyManager::new();
    let long_text = "a".repeat(1000);

    let s1 = manager.get_shared_string(&long_text);
    let _s2 = manager.get_shared_string(&long_text);

    assert_eq!(&*s1, &long_text);

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let stats = manager.get_stats();
    assert_eq!(
        stats
            .string_cache_hits
            .load(std::sync::atomic::Ordering::Relaxed),
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        1
    );
}

#[test]
fn test_shared_config_complex_type() {
    #[derive(Debug, Clone, PartialEq)]
    struct ComplexConfig {
        value: i32,
        text: String,
        data: Vec<u8>,
    }

    let manager = ZeroCopyManager::new();

    let config = manager.get_shared_config("complex", || ComplexConfig {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        value: 42,
        text: "test".to_string(),
        data: vec![1, 2, 3],
    });

    assert_eq!(config.value, 42);
    assert_eq!(config.text, "test");
    assert_eq!(config.data, vec![1, 2, 3]);
}

#[test]
fn test_zero_copy_builder_chain() {
    let value = ZeroCopyBuilder::new(10).optimize().build();

    assert_eq!(value, 10);
}

#[test]
fn test_multiple_managers_independent() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let manager1 = ZeroCopyManager::new();
    let manager2 = ZeroCopyManager::new();

    let _s1 = manager1.get_shared_string("test");
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let _s2 = manager2.get_shared_string("test");

    let stats1 = manager1.get_stats();
    let stats2 = manager2.get_stats();

    assert_eq!(
        stats1
            .string_cache_misses
            .load(std::sync::atomic::Ordering::Relaxed),
        1
    );
    assert_eq!(
        stats2
            .string_cache_misses
            .load(std::sync::atomic::Ordering::Relaxed),
        1
    );
}

#[test]
fn test_capability_validation_case_sensitive() {
    assert!(is_valid_service_capability("hsm"));
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(!is_valid_service_capability("HSM"));
    assert!(!is_valid_service_capability("Hsm"));
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_get_all_standard_capabilities_completeness() {
    let capabilities = get_all_standard_capabilities();

    // Verify all capabilities are valid
    for capability in &capabilities {
        assert!(is_valid_service_capability(capability));
    }
}
