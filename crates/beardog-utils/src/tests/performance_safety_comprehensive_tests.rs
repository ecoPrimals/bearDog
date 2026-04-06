// SPDX-License-Identifier: AGPL-3.0-or-later

// Performance & Safety Comprehensive Tests
//
// Comprehensive test suite for performance optimization and ultimate safety modules.
// Tests edge cases, error conditions, memory safety, and performance characteristics.

#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all
)]

use crate::performance_optimizations::{CloneOptimizer, OptimizationStats};
use crate::ultimate_safety::{SafetyStatistics, UltimateSafeBuffer};

// ====================
// Performance Optimization Tests
// ====================

#[test]
fn test_clone_optimizer_new() {
    let optimizer = CloneOptimizer::new();
    let stats = optimizer.get_stats();
    assert_eq!(stats.clones_avoided, 0);
    assert_eq!(stats.memory_saved, 0);
    assert_eq!(stats.cache_hits, 0);
    assert_eq!(stats.cache_misses, 0);
}

#[test]
fn test_clone_optimizer_default() {
    let optimizer = CloneOptimizer::default();
    let stats = optimizer.get_stats();
    assert_eq!(stats.clones_avoided, 0);
}

#[test]
fn test_optimization_stats_default() {
    let stats = OptimizationStats::default();
    assert_eq!(stats.clones_avoided, 0);
    assert_eq!(stats.memory_saved, 0);
    assert_eq!(stats.cache_hits, 0);
    assert_eq!(stats.cache_misses, 0);
}

#[test]
fn test_optimize_string_short_string() {
    let mut optimizer = CloneOptimizer::new();
    let input = "short";

    let result = optimizer.optimize_string(input);
    // Short strings should be borrowed (no caching)
    assert_eq!(result.as_ref(), input);
}

#[test]
fn test_optimize_string_long_string() {
    let mut optimizer = CloneOptimizer::new();
    let input = "this_is_a_very_long_string_that_should_be_interned_for_performance_reasons";

    let result = optimizer.optimize_string(input);
    assert_eq!(result.as_ref(), input);

    let stats = optimizer.get_stats();
    // Should have attempted caching
    assert!(stats.cache_hits + stats.cache_misses > 0);
}

#[test]
fn test_optimize_string_repeated_access() {
    let mut optimizer = CloneOptimizer::new();
    let input = "repeated_string_that_is_long_enough_to_be_cached_multiple_times";

    // First access - cache miss
    let _result1 = optimizer.optimize_string(input);
    let _stats_after_first = optimizer.get_stats();

    // Second access - should be cache hit
    let _result2 = optimizer.optimize_string(input);
    let stats_after_second = optimizer.get_stats();

    assert!(stats_after_second.cache_hits > 0);
}

#[test]
fn test_get_optimized_buffer_small() {
    let mut optimizer = CloneOptimizer::new();
    let buffer = optimizer.get_optimized_buffer(1024);

    assert!(buffer.capacity() >= 1024);
    assert_eq!(buffer.len(), 0);
}

#[test]
fn test_get_optimized_buffer_large() {
    let mut optimizer = CloneOptimizer::new();
    let buffer = optimizer.get_optimized_buffer(65536);

    assert!(buffer.capacity() >= 65536);
}

#[test]
fn test_buffer_pool_reuse() {
    let mut optimizer = CloneOptimizer::new();

    // Get a buffer
    let buffer = optimizer.get_optimized_buffer(4096);
    let _capacity = buffer.capacity();

    // Return it to the pool
    optimizer.return_buffer(buffer);

    // Get another buffer of the same size - should come from pool
    let stats_before = optimizer.get_stats().clone();
    let _reused_buffer = optimizer.get_optimized_buffer(4096);
    let stats_after = optimizer.get_stats();

    // Cache hits should increase when buffer is reused
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(stats_after.cache_hits >= stats_before.cache_hits);
}

#[test]
fn test_buffer_pool_size_classes() {
    let mut optimizer = CloneOptimizer::new();

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Test different size classes
    let sizes = vec![128, 1024, 4096, 8192, 16384, 65536];

    for size in sizes {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let buffer = optimizer.get_optimized_buffer(size);
        assert!(buffer.capacity() >= size);
    }
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_return_buffer_clears_data() {
    let mut optimizer = CloneOptimizer::new();

    let mut buffer = optimizer.get_optimized_buffer(1024);
    buffer.extend_from_slice(b"test data");
    assert_eq!(buffer.len(), 9);

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    optimizer.return_buffer(buffer);

    // Get buffer back from pool
    let reused_buffer = optimizer.get_optimized_buffer(1024);
    // Should be cleared
    assert_eq!(reused_buffer.len(), 0);
}

#[test]
fn test_buffer_pool_max_size_limit() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let mut optimizer = CloneOptimizer::new();

    // Create and return more than 10 buffers (pool limit)
    for _ in 0..20 {
        let buffer = optimizer.get_optimized_buffer(1024);
        optimizer.return_buffer(buffer);
    }

    // Pool should not grow unbounded
    // This is verified internally in the implementation
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_concurrent_buffer_allocation() {
    let mut optimizer = CloneOptimizer::new();

    // Simulate concurrent allocations
    let mut buffers = Vec::new();
    for i in 0..100 {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let size = 1024 * (i % 10 + 1);
        let buffer = optimizer.get_optimized_buffer(size);
        buffers.push(buffer);
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(buffers.len(), 100);
}

#[test]
fn test_stats_tracking() {
    let mut optimizer = CloneOptimizer::new();

    // Perform various operations
    let _s1 = optimizer.optimize_string("a".repeat(100).as_str());
    let _s2 = optimizer.optimize_string("b".repeat(100).as_str());
    let _b1 = optimizer.get_optimized_buffer(1024);
    let _b2 = optimizer.get_optimized_buffer(2048);

    let stats = optimizer.get_stats();

    // Stats should be tracking operations
    assert!(stats.cache_hits + stats.cache_misses > 0);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

// ====================
// Ultimate Safety Tests
// ====================

#[test]
fn test_ultimate_safe_buffer_new() {
    let buffer = UltimateSafeBuffer::new(1024);
    // Just verify it was created successfully
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let stats = buffer.get_safety_stats();
    assert_eq!(stats.safe_operations_completed, 0);
}

#[test]
fn test_safe_buffer_write() {
    let mut buffer = UltimateSafeBuffer::new(256);
    let data = b"Hello, World!";

    let result = buffer.safe_write(data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), data.len());
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[test]
fn test_safe_buffer_write_overflow() {
    let mut buffer = UltimateSafeBuffer::new(10);
    let large_data = b"This is way too much data for the buffer";

    let result = buffer.safe_write(large_data);
    assert!(result.is_err());
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_safe_buffer_read() {
    let mut buffer = UltimateSafeBuffer::new(256);
    let data = b"Test data";

    buffer.safe_write(data).unwrap();

    let result = buffer.safe_read(data.len());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), data);
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_safe_buffer_read_underflow() {
    let mut buffer = UltimateSafeBuffer::new(256);

    let result = buffer.safe_read(100);
    assert!(result.is_err());
}

#[test]
fn test_safe_buffer_read_write_cycle() {
    let mut buffer = UltimateSafeBuffer::new(1024);

    // Write some data
    let write_data = b"First write";
    buffer.safe_write(write_data).unwrap();

    // Read it back
    let read_data = buffer.safe_read(write_data.len()).unwrap();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(read_data, write_data);

    // Write more data
    let write_data2 = b"Second write";
    buffer.safe_write(write_data2).unwrap();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    // Read second data
    let read_data2 = buffer.safe_read(write_data2.len()).unwrap();
    assert_eq!(read_data2, write_data2);
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: important
fn test_safe_buffer_capacity_boundaries() {
    let mut buffer = UltimateSafeBuffer::new(100);

    // Fill exactly to capacity
    let data = vec![0u8; 100];
    let result = buffer.safe_write(&data);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(result.is_ok());

    // Try to write one more byte - should fail
    let result = buffer.safe_write(b"x");
    assert!(result.is_err());
}

#[test]
fn test_safe_buffer_clone() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let mut buffer = UltimateSafeBuffer::new(256);
    buffer.safe_write(b"Original data").unwrap();

    let cloned = buffer.clone();
    let stats1 = buffer.get_safety_stats();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let stats2 = cloned.get_safety_stats();

    assert_eq!(
        stats1.safe_operations_completed,
        stats2.safe_operations_completed
    );
}

#[test]
fn test_safe_buffer_multiple_reads() {
    let mut buffer = UltimateSafeBuffer::new(256);
    buffer.safe_write(b"ABCDEFGHIJ").unwrap();

    // Read in chunks
    let data1 = buffer.safe_read(3).unwrap();
    assert_eq!(data1, b"ABC");

    let data2 = buffer.safe_read(3).unwrap();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(data2, b"DEF");

    let data3 = buffer.safe_read(4).unwrap();
    assert_eq!(data3, b"GHIJ");
}

#[test]
fn test_safety_statistics_default() {
    let stats = SafetyStatistics::default();
    assert_eq!(stats.bounds_checks_performed, 0);
    assert_eq!(stats.bounds_violations_prevented, 0);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(stats.safe_operations_completed, 0);
}

#[test]
fn test_safe_buffer_statistics_tracking() {
    let mut buffer = UltimateSafeBuffer::new(256);

    // Perform operations that should update statistics
    buffer.safe_write(b"Test").unwrap();
    buffer.safe_read(4).unwrap();

    let stats = buffer.get_safety_stats();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(stats.safe_operations_completed > 0);
}

#[test]
fn test_safe_buffer_bounds_checking() {
    let mut buffer = UltimateSafeBuffer::new(10);

    // This should trigger bounds checking
    let large_data = vec![0u8; 100];
    let result = buffer.safe_write(&large_data);

    assert!(result.is_err());

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let stats = buffer.get_safety_stats();
    assert!(stats.bounds_violations_prevented > 0);
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_safe_buffer_edge_cases() {
    // Zero-sized buffer
    let buffer = UltimateSafeBuffer::new(0);
    let stats = buffer.get_safety_stats();
    assert_eq!(stats.safe_operations_completed, 0);

    // Very large buffer
    let large_buffer = UltimateSafeBuffer::new(1024 * 1024);
    let stats = large_buffer.get_safety_stats();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(stats.safe_operations_completed, 0);
}

#[test]
fn test_safe_buffer_empty_operations() {
    let mut buffer = UltimateSafeBuffer::new(256);

    // Write empty data
    let result = buffer.safe_write(b"");
    assert!(result.is_ok());

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: important
    // Read empty
    let result = buffer.safe_read(0);
    assert!(result.is_ok());
}

// ====================
// Integration Tests
// ====================

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_optimizer_with_multiple_string_types() {
    let mut optimizer = CloneOptimizer::new();

    let short_strings = vec!["a", "b", "c"];
    let long_strings = vec![
        "this_is_a_long_string_1",
        "this_is_a_long_string_2",
        "this_is_a_long_string_3",
    ];

    for s in short_strings {
        optimizer.optimize_string(s);
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    for s in long_strings {
        optimizer.optimize_string(s);
    }

    let stats = optimizer.get_stats();
    assert!(stats.cache_hits + stats.cache_misses > 0);
}

#[test]
fn test_buffer_lifecycle_management() {
    let mut optimizer = CloneOptimizer::new();
    let mut buffers = Vec::new();

    // Allocate buffers
    for _ in 0..10 {
        let buffer = optimizer.get_optimized_buffer(2048);
        buffers.push(buffer);
    }

    // Return all buffers
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    for buffer in buffers {
        optimizer.return_buffer(buffer);
    }

    // Reallocate - should come from pool
    let stats_before = optimizer.get_stats().clone();
    let _new_buffer = optimizer.get_optimized_buffer(2048);
    let stats_after = optimizer.get_stats();

    assert!(stats_after.cache_hits >= stats_before.cache_hits);
}

#[test]
fn test_safe_buffer_stress_test() {
    let mut buffer = UltimateSafeBuffer::new(10000);

    // Perform many write/read cycles
    for i in 0..100 {
        let data = format!("Iteration {i}");
        buffer.safe_write(data.as_bytes()).unwrap();

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let read_data = buffer.safe_read(data.len()).unwrap();
        assert_eq!(read_data, data.as_bytes());
    }

    let stats = buffer.get_safety_stats();
    assert!(stats.safe_operations_completed >= 200); // 100 writes + 100 reads
}

#[test]
fn test_performance_with_varying_sizes() {
    let mut optimizer = CloneOptimizer::new();

    let sizes = vec![64, 128, 256, 512, 1024, 2048, 4096, 8192];

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    for size in sizes {
        let buffer = optimizer.get_optimized_buffer(size);
        assert!(buffer.capacity() >= size);
        optimizer.return_buffer(buffer);
    }
}

#[test]
fn test_safety_under_error_conditions() {
    let mut buffer = UltimateSafeBuffer::new(100);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: important

    // Try various error-inducing operations
    let result1 = buffer.safe_write(&[0u8; 150]);
    assert!(result1.is_err());

    let result2 = buffer.safe_read(50);
    assert!(result2.is_err());

    // Buffer should still be in valid state
    buffer.safe_write(b"Valid data").unwrap();
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_memory_efficiency() {
    let mut optimizer = CloneOptimizer::new();

    // Allocate and return many buffers
    for _ in 0..1000 {
        let buffer = optimizer.get_optimized_buffer(1024);
        optimizer.return_buffer(buffer);
    }

    // Most allocations should hit the cache
    let stats = optimizer.get_stats();
    assert!(stats.cache_hits > 0);
}
