//! Comprehensive tests for ultimate_safety module
//! Focus: Safety guarantees, edge cases, boundary conditions

use crate::ultimate_safety::*;

#[test]
fn test_ultimate_safe_buffer_basic_operations() {
    let mut buffer = UltimateSafeBuffer::new(1024);

    // Test successful write
    let data = b"Hello, BearDog!";
    let result = buffer.safe_write(data);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), data.len());

    // Test successful read
    let read_result = buffer.safe_read(data.len());
    assert!(read_result.is_ok());
    assert_eq!(read_result.unwrap(), data);
}

#[test]
fn test_safe_buffer_overflow_prevention() {
    let mut buffer = UltimateSafeBuffer::new(10);

    // Attempt to write more than capacity
    let large_data = vec![0u8; 20];
    let result = buffer.safe_write(&large_data);
    assert!(result.is_err());

    // Verify stats show prevention
    let stats = buffer.get_safety_stats();
    assert!(stats.bounds_violations_prevented > 0);
}

#[test]
fn test_safe_buffer_read_beyond_bounds_prevention() {
    let mut buffer = UltimateSafeBuffer::new(100);

    // Write some data
    buffer.safe_write(b"test").unwrap();

    // Try to read more than written
    let result = buffer.safe_read(10);
    assert!(result.is_err());

    // Verify stats
    let stats = buffer.get_safety_stats();
    assert!(stats.bounds_violations_prevented > 0);
}

#[test]
fn test_safe_buffer_integrity_verification() {
    let buffer = UltimateSafeBuffer::new(1024);
    let result = buffer.verify_integrity();
    assert!(result.is_ok());
}

#[test]
fn test_safe_buffer_multiple_writes() {
    let mut buffer = UltimateSafeBuffer::new(1024);

    // Multiple sequential writes
    buffer.safe_write(b"first").unwrap();
    buffer.safe_write(b" ").unwrap();
    buffer.safe_write(b"second").unwrap();

    // Read all data
    let data = buffer.safe_read(12).unwrap();
    assert_eq!(data, b"first second");
}

#[test]
fn test_safe_buffer_safety_statistics() {
    let mut buffer = UltimateSafeBuffer::new(100);

    buffer.safe_write(b"test").unwrap();
    buffer.safe_read(4).unwrap();

    let stats = buffer.get_safety_stats();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(stats.bounds_checks_performed >= 2);
    assert!(stats.safe_operations_completed >= 2);
}

#[test]
fn test_safe_buffer_clone() {
    let mut buffer1 = UltimateSafeBuffer::new(100);
    buffer1.safe_write(b"test").unwrap();

    let buffer2 = buffer1.clone();
    let stats = buffer2.get_safety_stats();
    assert!(stats.safe_operations_completed > 0);
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: important

#[test]
fn test_ultimate_safe_memory_pool_basic() {
    let pool = UltimateSafeMemoryPool::new(|| vec![1, 2, 3, 4, 5], 10);

    // Borrow object
    let obj = pool.safe_borrow();
    assert!(obj.is_ok());
    assert!(obj.unwrap().as_ref().is_some());
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_memory_pool_multiple_borrows() {
    let pool = UltimateSafeMemoryPool::new(|| String::from("test"), 5);

    let _obj1 = pool.safe_borrow().unwrap();
    let _obj2 = pool.safe_borrow().unwrap();
    let _obj3 = pool.safe_borrow().unwrap();

    let stats = pool.get_stats();
    assert_eq!(stats.objects_borrowed, 3);
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_memory_pool_object_return() {
    let pool = UltimateSafeMemoryPool::new(|| 42, 5);

    {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let _obj = pool.safe_borrow().unwrap();
        // Object should be returned when dropped
    }

    let stats = pool.get_stats();
    assert_eq!(stats.objects_borrowed, 0);
    assert_eq!(stats.objects_in_pool, 1);
}

#[test]
fn test_memory_pool_statistics() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let pool = UltimateSafeMemoryPool::new(|| vec![0u8; 100], 10);

    let _obj1 = pool.safe_borrow().unwrap();
    let _obj2 = pool.safe_borrow().unwrap();

    let stats = pool.get_stats();
    assert_eq!(stats.total_created, 2);
    assert_eq!(stats.pool_misses, 2);
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[test]
fn test_memory_pool_reuse() {
    let pool = UltimateSafeMemoryPool::new(|| String::from("pooled"), 5);

    {
        let _obj1 = pool.safe_borrow().unwrap();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
    } // obj1 returned to pool

    let _obj2 = pool.safe_borrow().unwrap(); // Should reuse obj1

    let stats = pool.get_stats();
    assert!(stats.pool_hits > 0);
    assert_eq!(stats.total_created, 1); // Only one object created
                                        // TEST_CATEGORY: integration
                                        // TEST_DOMAIN: core
                                        // TEST_PRIORITY: normal
}

#[test]
fn test_safe_reference_basic_operations() {
    let safe_ref = SafeReference::new(100);

    // Test safe read
    let value = safe_ref.safe_read(|x| *x);
    assert!(value.is_ok());
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(value.unwrap(), 100);
}

#[test]
fn test_safe_reference_safe_write() {
    let safe_ref = SafeReference::new(42);

    // Test safe write
    let result = safe_ref.safe_write(|x| *x = 200);
    assert!(result.is_ok());

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Verify write
    let value = safe_ref.safe_read(|x| *x).unwrap();
    assert_eq!(value, 200);
}

#[test]
fn test_safe_reference_invalidation() {
    let safe_ref = SafeReference::new(42);

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Invalidate reference
    safe_ref.invalidate();

    // Attempt to read should fail
    let result = safe_ref.safe_read(|x| *x);
    assert!(result.is_err());

    // Attempt to write should fail
    let result = safe_ref.safe_write(|x| *x = 100);
    assert!(result.is_err());
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_safety_token_creation() {
    let _token1 = SafetyToken::new(SafetyLevel::Basic);
    let _token2 = SafetyToken::new(SafetyLevel::Enhanced);
    let _token3 = SafetyToken::new(SafetyLevel::Ultimate);

    // Tokens are created successfully with different safety levels
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Note: context_id is private, so we test behavior instead
    // No panic means tokens were created successfully
}

#[test]
fn test_safety_level_verification() {
    let basic_token = SafetyToken::new(SafetyLevel::Basic);
    let enhanced_token = SafetyToken::new(SafetyLevel::Enhanced);
    let ultimate_token = SafetyToken::new(SafetyLevel::Ultimate);

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: important
    // Basic token doesn't meet Enhanced requirement
    assert!(!basic_token.verify_safety_level(SafetyLevel::Enhanced));

    // Enhanced token meets Basic requirement
    assert!(enhanced_token.verify_safety_level(SafetyLevel::Basic));

    // Ultimate token meets all requirements
    assert!(ultimate_token.verify_safety_level(SafetyLevel::Basic));
    assert!(ultimate_token.verify_safety_level(SafetyLevel::Enhanced));
    assert!(ultimate_token.verify_safety_level(SafetyLevel::Ultimate));
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_safety_level_ordering() {
    assert!(SafetyLevel::Ultimate > SafetyLevel::Enhanced);
    assert!(SafetyLevel::Enhanced > SafetyLevel::Basic);
    assert!(SafetyLevel::Ultimate > SafetyLevel::Basic);
}

#[test]
fn test_safety_error_display() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let error1 = SafetyError::BufferOverflow {
        attempted_size: 100,
        available_space: 50,
    };
    let display = format!("{}", error1);
    assert!(display.contains("Buffer overflow"));

    let error2 = SafetyError::InvalidReference;
    let display = format!("{}", error2);
    assert!(display.contains("Invalid reference"));
}

#[test]
fn test_pool_stats_clone() {
    let stats = PoolStats {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        objects_in_pool: 5,
        objects_borrowed: 2,
        total_created: 10,
        pool_hits: 15,
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: important
        pool_misses: 3,
        leaks_prevented: 0,
    };

    let cloned = stats.clone();
    assert_eq!(cloned.objects_in_pool, 5);
    assert_eq!(cloned.objects_borrowed, 2);
}

#[test]
fn test_safe_buffer_empty_write() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let mut buffer = UltimateSafeBuffer::new(100);
    let result = buffer.safe_write(&[]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_safe_buffer_empty_read() {
    let mut buffer = UltimateSafeBuffer::new(100);
    buffer.safe_write(b"test").unwrap();

    let result = buffer.safe_read(0);
    assert!(result.is_ok());
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn test_safe_reference_with_string() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let safe_ref = SafeReference::new(String::from("Hello"));

    let length = safe_ref.safe_read(|s| s.len()).unwrap();
    assert_eq!(length, 5);

    safe_ref.safe_write(|s| s.push_str(" World")).unwrap();

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let new_value = safe_ref.safe_read(|s| s.clone()).unwrap();
    assert_eq!(new_value, "Hello World");
}

#[test]
fn test_memory_pool_with_complex_type() {
    #[derive(Clone, Debug)]
    struct ComplexData {
        _id: u64,
        _data: Vec<u8>,
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
    }

    let pool = UltimateSafeMemoryPool::new(
        || ComplexData {
            _id: 0,
            _data: vec![0; 1024],
        },
        5,
    );

    let obj = pool.safe_borrow().unwrap();
    assert!(obj.as_ref().is_some());
}

#[test]
fn test_safety_statistics_defaults() {
    let stats = SafetyStatistics::default();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(stats.bounds_checks_performed, 0);
    assert_eq!(stats.bounds_violations_prevented, 0);
    assert_eq!(stats.safe_operations_completed, 0);
    assert_eq!(stats.allocations_tracked, 0);
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: important
#[test]
fn test_safe_buffer_capacity_boundary() {
    let mut buffer = UltimateSafeBuffer::new(10);

    // Write exactly to capacity
    let data = vec![1u8; 10];
    let result = buffer.safe_write(&data);
    assert!(result.is_ok());

    // Try to write one more byte (should fail)
    let result = buffer.safe_write(&[1]);
    assert!(result.is_err());
}
