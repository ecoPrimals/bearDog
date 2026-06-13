// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive tests for `ultimate_safety` and `ultimate_performance` modules
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

