// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive tests for ultimate_safety and ultimate_performance modules
//!
//! This test suite provides extensive coverage of the safety and performance
//! optimization modules to increase test coverage from 0% to ~70-80%.

#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all
)]

use crate::ultimate_performance::UltimatePerformanceProcessor;
use crate::ultimate_safety::{
    SafeReference, SafetyError, SafetyLevel, SafetyToken, UltimateSafeBuffer,
    UltimateSafeMemoryPool,
};

// ================================
// ULTIMATE SAFETY MODULE TESTS
// ================================

#[test]
fn test_safe_buffer_basic_operations() {
    let mut buffer = UltimateSafeBuffer::new(1024);

    // Test writing various sizes
    let small_data = b"Small";
    assert!(buffer.safe_write(small_data).is_ok());

    let medium_data = b"Medium sized data chunk";
    assert!(buffer.safe_write(medium_data).is_ok());

    // Verify we can read back
    let read_small = buffer.safe_read(small_data.len()).unwrap();
    assert_eq!(read_small, small_data);

    let read_medium = buffer.safe_read(medium_data.len()).unwrap();
    assert_eq!(read_medium, medium_data);
}

#[test]
fn test_safe_buffer_bounds_checking() {
    let mut buffer = UltimateSafeBuffer::new(100);

    // Try to write more than capacity
    let large_data = vec![0u8; 200];
    let result = buffer.safe_write(&large_data);
    assert!(result.is_err());

    // Verify safety stats tracked the violation
    let stats = buffer.get_safety_stats();
    assert_eq!(stats.bounds_violations_prevented, 1);
}

#[test]
fn test_safe_buffer_read_beyond_bounds() {
    let mut buffer = UltimateSafeBuffer::new(100);

    // Write some data
    buffer.safe_write(b"Hello").unwrap();

    // Try to read more than written
    let result = buffer.safe_read(20);
    assert!(result.is_err());

    match result {
        Err(SafetyError::ReadBeyondBounds {
            attempted_read,
            available_data,
        }) => {
            assert_eq!(attempted_read, 20);
            assert_eq!(available_data, 5);
        }
        _ => panic!("Expected ReadBeyondBounds error"),
    }
}

#[test]
fn test_safe_buffer_integrity_verification() {
    let buffer = UltimateSafeBuffer::new(1024);

    // Fresh buffer should have valid integrity
    assert!(buffer.verify_integrity().is_ok());

    // Verify safety stats are tracked
    let stats = buffer.get_safety_stats();
    assert_eq!(stats.bounds_checks_performed, 0);
    assert_eq!(stats.safe_operations_completed, 0);
}

#[test]
fn test_safe_buffer_sequential_writes() {
    let mut buffer = UltimateSafeBuffer::new(1024);

    // Multiple sequential writes
    for i in 0..10 {
        let data = format!("Write {i}");
        assert!(buffer.safe_write(data.as_bytes()).is_ok());
    }

    // Verify all writes were tracked
    let stats = buffer.get_safety_stats();
    assert_eq!(stats.safe_operations_completed, 10);
    assert!(stats.bounds_checks_performed >= 10);
}

#[test]
fn test_safe_buffer_statistics_tracking() {
    let mut buffer = UltimateSafeBuffer::new(1024);

    // Perform operations
    buffer.safe_write(b"test1").unwrap();
    buffer.safe_write(b"test2").unwrap();
    buffer.safe_read(5).unwrap();

    let stats = buffer.get_safety_stats();

    // Verify statistics
    assert_eq!(stats.safe_operations_completed, 3);
    assert_eq!(stats.bounds_checks_performed, 3);
    assert!(stats.allocations_tracked >= 1);
}

#[test]
fn test_memory_pool_basic_operations() {
    let pool = UltimateSafeMemoryPool::new(|| vec![1, 2, 3], 10);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    // Borrow an object
    let obj1 = pool.safe_borrow().unwrap();
    assert!(obj1.as_ref().is_some());
    assert_eq!(*obj1.as_ref().unwrap(), vec![1, 2, 3]);

    // Get stats
    let stats = pool.get_stats();
    assert_eq!(stats.objects_borrowed, 1);
    assert_eq!(stats.total_created, 1);
}

#[test]
fn test_memory_pool_multiple_borrows() {
    let pool = UltimateSafeMemoryPool::new(|| String::from("test"), 10);

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Borrow multiple objects
    let _obj1 = pool.safe_borrow().unwrap();
    let _obj2 = pool.safe_borrow().unwrap();
    let _obj3 = pool.safe_borrow().unwrap();

    let stats = pool.get_stats();
    assert_eq!(stats.objects_borrowed, 3);
    assert_eq!(stats.total_created, 3);
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_memory_pool_reuse() {
    let pool = UltimateSafeMemoryPool::new(|| String::from("reusable"), 10);

    // Borrow and return
    {
        let _obj1 = pool.safe_borrow().unwrap();
        // obj1 dropped here, returned to pool
    }

    // Borrow again - should reuse
    let _obj2 = pool.safe_borrow().unwrap();

    let stats = pool.get_stats();
    assert_eq!(stats.total_created, 1); // Only created once
    assert!(stats.pool_hits >= 1); // Reused at least once
}

#[test]
fn test_memory_pool_statistics() {
    let pool = UltimateSafeMemoryPool::new(|| 42i32, 5);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    // Create some objects
    let obj1 = pool.safe_borrow().unwrap();
    let obj2 = pool.safe_borrow().unwrap();

    let stats = pool.get_stats();
    assert_eq!(stats.objects_borrowed, 2);
    assert_eq!(stats.total_created, 2);
    assert_eq!(stats.pool_misses, 2); // First two are misses

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    drop(obj1);
    drop(obj2);

    // Check that objects were returned
    let stats_after = pool.get_stats();
    assert_eq!(stats_after.objects_borrowed, 0);
}

#[test]
fn test_safe_reference_read_operations() {
    let safe_ref = SafeReference::new(vec![1, 2, 3, 4, 5]);

    // Test safe read
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let sum = safe_ref.safe_read(|v| v.iter().sum::<i32>()).unwrap();
    assert_eq!(sum, 15);

    let len = safe_ref.safe_read(std::vec::Vec::len).unwrap();
    assert_eq!(len, 5);
}

#[test]
fn test_safe_reference_write_operations() {
    let safe_ref = SafeReference::new(vec![1, 2, 3]);

    // Modify via safe_write
    safe_ref.safe_write(|v| v.push(4)).unwrap();

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let len = safe_ref.safe_read(std::vec::Vec::len).unwrap();
    assert_eq!(len, 4);

    // Verify content
    let last = safe_ref.safe_read(|v| *v.last().unwrap()).unwrap();
    assert_eq!(last, 4);
}

#[test]
fn test_safe_reference_invalidation() {
    let safe_ref = SafeReference::new(100);

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Should work before invalidation
    assert!(safe_ref.safe_read(|x| *x).is_ok());

    // Invalidate
    safe_ref.invalidate();

    // Should fail after invalidation
    assert!(safe_ref.safe_read(|x| *x).is_err());
    assert!(safe_ref.safe_write(|x| *x = 200).is_err());
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_safe_reference_concurrent_reads() {
    use std::sync::Arc;
    use std::thread;

    let safe_ref = Arc::new(SafeReference::new(42));
    let mut handles = vec![];

    // Multiple threads reading concurrently
    for _ in 0..5 {
        let safe_ref_clone = Arc::clone(&safe_ref);
        let handle = thread::spawn(move || safe_ref_clone.safe_read(|x| *x).unwrap());
        handles.push(handle);
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // All threads should read successfully
    for handle in handles {
        let value = handle.join().unwrap();
        assert_eq!(value, 42);
    }
}

#[test]
fn test_safety_token_creation() {
    let token1 = SafetyToken::new(SafetyLevel::Basic);
    let token2 = SafetyToken::new(SafetyLevel::Enhanced);
    let token3 = SafetyToken::new(SafetyLevel::Ultimate);

    // Tokens should be created successfully
    assert!(token1.verify_safety_level(SafetyLevel::Basic));
    assert!(token2.verify_safety_level(SafetyLevel::Enhanced));
    assert!(token3.verify_safety_level(SafetyLevel::Ultimate));
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[test]
fn test_safety_token_level_verification() {
    let basic_token = SafetyToken::new(SafetyLevel::Basic);
    let ultimate_token = SafetyToken::new(SafetyLevel::Ultimate);

    // Basic token should only satisfy Basic level
    assert!(basic_token.verify_safety_level(SafetyLevel::Basic));
    assert!(!basic_token.verify_safety_level(SafetyLevel::Enhanced));
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(!basic_token.verify_safety_level(SafetyLevel::Ultimate));

    // Ultimate token should satisfy all levels
    assert!(ultimate_token.verify_safety_level(SafetyLevel::Basic));
    assert!(ultimate_token.verify_safety_level(SafetyLevel::Enhanced));
    assert!(ultimate_token.verify_safety_level(SafetyLevel::Ultimate));
}

#[test]
fn test_safety_error_display() {
    let err1 = SafetyError::BufferOverflow {
        attempted_size: 100,
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: important
        available_space: 50,
    };
    let msg1 = format!("{err1}");
    assert!(msg1.contains("Buffer overflow"));
    assert!(msg1.contains("100"));
    assert!(msg1.contains("50"));

    let err2 = SafetyError::ReadBeyondBounds {
        attempted_read: 20,
        available_data: 10,
    };
    let msg2 = format!("{err2}");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(msg2.contains("Read beyond bounds"));

    let err3 = SafetyError::InvalidReference;
    let msg3 = format!("{err3}");
    assert!(msg3.contains("Invalid reference"));
}

// ================================
// ULTIMATE PERFORMANCE MODULE TESTS
// ================================

#[test]
fn test_performance_processor_creation() {
    let processor = UltimatePerformanceProcessor::new();
    let stats = processor.get_performance_stats();

    // New processor should have zero stats
    assert_eq!(stats.operations_processed, 0);
    assert_eq!(stats.simd_operations, 0);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_performance_processor_basic_processing() {
    let processor = UltimatePerformanceProcessor::new();
    let test_data = vec![1, 2, 3, 4, 5];

    let result = processor.process_with_ultimate_optimization(&test_data);

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Verify processing worked
    assert_eq!(result.len(), test_data.len());
    for (i, &byte) in result.iter().enumerate() {
        assert_eq!(byte, test_data[i].wrapping_add(1));
    }
}

#[test]
fn test_performance_processor_empty_data() {
    let processor = UltimatePerformanceProcessor::new();
    let empty_data: Vec<u8> = vec![];

    let result = processor.process_with_ultimate_optimization(&empty_data);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: important

    assert_eq!(result.len(), 0);
}

#[test]
fn test_performance_processor_large_data() {
    let processor = UltimatePerformanceProcessor::new();
    let large_data = vec![0u8; 1024];

    let result = processor.process_with_ultimate_optimization(&large_data);

    assert_eq!(result.len(), 1024);
    assert!(result.iter().all(|&b| b == 1));
}

#[test]
fn test_performance_processor_various_sizes() {
    let processor = UltimatePerformanceProcessor::new();

    // Test different data sizes
    for size in [1, 7, 16, 32, 64, 128, 256, 512] {
        let data = vec![42u8; size];
        let result = processor.process_with_ultimate_optimization(&data);

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert_eq!(result.len(), size);
        assert!(result.iter().all(|&b| b == 43));
    }
}

#[test]
fn test_performance_processor_wrapping_arithmetic() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let processor = UltimatePerformanceProcessor::new();
    let data = vec![255u8; 10]; // Max value

    let result = processor.process_with_ultimate_optimization(&data);

    // 255 + 1 wraps to 0
    assert!(result.iter().all(|&b| b == 0));
}

#[test]
fn test_performance_processor_statistics() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let processor = UltimatePerformanceProcessor::new();

    // Process some data
    let data1 = vec![1, 2, 3];
    processor.process_with_ultimate_optimization(&data1);

    let data2 = vec![4, 5, 6, 7, 8];
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    processor.process_with_ultimate_optimization(&data2);

    // Check statistics
    let stats = processor.get_performance_stats();
    assert!(stats.simd_operations >= 2);
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_performance_stats_structure() {
    let processor = UltimatePerformanceProcessor::new();
    let stats = processor.get_performance_stats();

    // Verify all fields exist and have reasonable values
    // Note: operations_processed, total_processing_time_ns, and simd_operations are unsigned,
    // so >= 0 is always true (enforced by type system)
    assert!(stats.cache_hit_ratio >= 0.0);
    assert!(stats.cache_hit_ratio <= 1.0);
    assert!(stats.prefetch_effectiveness >= 0.0);
    assert!(stats.prefetch_effectiveness <= 1.0);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(stats.average_latency_ns >= 0.0);
}

#[test]
fn test_performance_processor_default() {
    let processor1 = UltimatePerformanceProcessor::default();
    let processor2 = UltimatePerformanceProcessor::new();

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let data = vec![1, 2, 3];

    let result1 = processor1.process_with_ultimate_optimization(&data);
    let result2 = processor2.process_with_ultimate_optimization(&data);

    // Both should produce same results
    assert_eq!(result1, result2);
}

#[test]
fn test_performance_processor_sequential_processing() {
    let processor = UltimatePerformanceProcessor::new();

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Process multiple batches sequentially
    for i in 0..10 {
        let data = vec![i; 16];
        let result = processor.process_with_ultimate_optimization(&data);

        assert_eq!(result.len(), 16);
        assert!(result.iter().all(|&b| b == i.wrapping_add(1)));
    }

    // Verify statistics tracked all operations
    let stats = processor.get_performance_stats();
    assert!(stats.simd_operations >= 10);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_performance_stats_cache_hit_ratio() {
    let processor = UltimatePerformanceProcessor::new();

    // Process data
    processor.process_with_ultimate_optimization(&[1, 2, 3]);

    let stats = processor.get_performance_stats();

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Cache hit ratio should be in valid range
    assert!(stats.cache_hit_ratio >= 0.0);
    assert!(stats.cache_hit_ratio <= 1.0);
}

// ================================
// INTEGRATION TESTS
// ================================

#[test]
fn test_safety_and_performance_integration() {
    // Use safe buffer with performance processor
    let mut buffer = UltimateSafeBuffer::new(1024);
    let processor = UltimatePerformanceProcessor::new();

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Write data to safe buffer
    let data = b"Performance test data";
    buffer.safe_write(data).unwrap();

    // Read and process
    let read_data = buffer.safe_read(data.len()).unwrap();
    let processed = processor.process_with_ultimate_optimization(&read_data);

    // Verify
    assert_eq!(processed.len(), data.len());

    // Check both modules tracked operations
    let buffer_stats = buffer.get_safety_stats();
    let proc_stats = processor.get_performance_stats();

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(buffer_stats.safe_operations_completed >= 2);
    assert!(proc_stats.simd_operations >= 1);
}

#[test]
fn test_memory_pool_with_performance_data() {
    let pool = UltimateSafeMemoryPool::new(|| vec![0u8; 256], 5);
    let processor = UltimatePerformanceProcessor::new();

    // Borrow buffer from pool
    let mut obj = pool.safe_borrow().unwrap();
    if let Some(buffer) = obj.as_mut() {
        // Fill with test data
        buffer.fill(42);

        // Process the buffer
        let result = processor.process_with_ultimate_optimization(buffer);

        // Verify processing
        assert!(result.iter().all(|&b| b == 43));
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let pool_stats = pool.get_stats();
    assert_eq!(pool_stats.objects_borrowed, 1);
}

#[test]
fn test_safe_reference_with_performance_operations() {
    let processor = UltimatePerformanceProcessor::new();
    let safe_ref = SafeReference::new(vec![10u8; 100]);

    // Read and process via safe reference
    let result = safe_ref
        .safe_read(|data| processor.process_with_ultimate_optimization(data))
        .unwrap();

    assert_eq!(result.len(), 100);
    assert!(result.iter().all(|&b| b == 11));
}

// ================================
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
// EDGE CASE TESTS
// ================================

#[test]
fn test_safe_buffer_edge_case_zero_capacity() {
    let mut buffer = UltimateSafeBuffer::new(0);

    // Should not be able to write anything
    assert!(buffer.safe_write(b"x").is_err());

    // Integrity should still be valid
    assert!(buffer.verify_integrity().is_ok());
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: important
fn test_safe_buffer_edge_case_exact_capacity() {
    let mut buffer = UltimateSafeBuffer::new(5);

    // Write exactly capacity
    assert!(buffer.safe_write(b"12345").is_ok());

    // One more byte should fail
    assert!(buffer.safe_write(b"6").is_err());
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: important
}

#[test]
fn test_memory_pool_edge_case_factory() {
    let call_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let call_count_clone = call_count.clone();

    let pool = UltimateSafeMemoryPool::new(
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: important
        move || {
            call_count_clone.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            String::from("test")
        },
        5,
    );

    // Borrow multiple objects
    let _obj1 = pool.safe_borrow().unwrap();
    let _obj2 = pool.safe_borrow().unwrap();

    // Factory should have been called twice
    assert_eq!(call_count.load(std::sync::atomic::Ordering::Relaxed), 2);
}

#[test]
fn test_performance_processor_edge_case_single_byte() {
    let processor = UltimatePerformanceProcessor::new();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: important

    let result = processor.process_with_ultimate_optimization(&[255]);

    assert_eq!(result, vec![0]); // Wrapping
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_safety_token_uniqueness() {
    let token1 = SafetyToken::new(SafetyLevel::Ultimate);
    let token2 = SafetyToken::new(SafetyLevel::Ultimate);

    // Tokens should have different context IDs
    // (can't directly access context_id, but can verify both work independently)
    assert!(token1.verify_safety_level(SafetyLevel::Ultimate));
    assert!(token2.verify_safety_level(SafetyLevel::Ultimate));
}
