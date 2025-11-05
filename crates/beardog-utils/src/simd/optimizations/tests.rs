//! Tests for SIMD optimizations

use super::*;
use beardog_errors::BearDogError;

#[test]
fn test_simd_optimizer_creation() {
    let optimizer = AdvancedSIMDOptimizer::new();
    assert_eq!(optimizer.get_metrics().operations_count, 0);
    assert_eq!(optimizer.cache_hit_rate(), 0.0);
}

#[test]
fn test_fast_buffer_management() {
    let mut optimizer = AdvancedSIMDOptimizer::new();

    // Get buffer
    let buffer = optimizer.get_fast_buffer(1024);
    assert_eq!(buffer.len(), 1024);

    // Return buffer
    optimizer.return_fast_buffer(buffer);

    // Get buffer again (should reuse)
    let buffer2 = optimizer.get_fast_buffer(512);
    assert!(buffer2.capacity() >= 512);

    assert_eq!(optimizer.get_metrics().buffer_reuses, 1);
}

#[test]
fn test_simd_operations() -> Result<(), String> {
    let mut optimizer = AdvancedSIMDOptimizer::new();
    let mut data = vec![0xFF, 0x00, 0xAA, 0x55];

    // Test XOR operation
    optimizer
        .simd_process_data(&mut data, SIMDOperation::XorWithPattern(0xFF))?;
    assert_eq!(data, vec![0x00, 0xFF, 0x55, 0xAA]);

    // Test AND operation
    optimizer
        .simd_process_data(&mut data, SIMDOperation::BitwiseAnd(0x0F))?;
    assert_eq!(data, vec![0x00, 0x0F, 0x05, 0x0A]);
    
    Ok(())
}

#[test]
fn test_aligned_buffer_management() {
    let mut optimizer = AdvancedSIMDOptimizer::new();

    // Get aligned buffer
    let buffer = optimizer.get_aligned_buffer(2048);
    assert!(buffer.is_some());

    if let Some(buf) = buffer {
        assert!(buf.capacity >= 2048);
        assert!(buf.in_use);
    }

    // Release buffer
    optimizer.release_aligned_buffer(0);
    // Note: We can't directly access private fields to verify release,
    // but getting another buffer of same size should reuse it
    let buffer2 = optimizer.get_aligned_buffer(2048);
    assert!(buffer2.is_some());
    assert_eq!(optimizer.get_metrics().buffer_reuses, 1);
}

#[test]
fn test_performance_metrics() -> Result<(), String> {
    let mut optimizer = AdvancedSIMDOptimizer::new();
    let mut data = vec![0u8; 1024];

    // Perform operations to generate metrics
    optimizer
        .simd_process_data(&mut data, SIMDOperation::XorWithPattern(0xAA))?;
    optimizer
        .simd_process_data(&mut data, SIMDOperation::BitwiseAnd(0xFF))?;

    let metrics = optimizer.get_metrics();
    assert_eq!(metrics.operations_count, 2);
    assert_eq!(metrics.total_bytes_processed, 2048);
    assert!(metrics.avg_operation_time_ns > 0.0);
    
    Ok(())
}

// Additional tests for SafeSimdOptimizer
#[test]
fn test_safe_simd_optimizer_creation() {
    let config = SafeSimdConfig::default();
    let optimizer = SafeSimdOptimizer::new(config);
    assert_eq!(optimizer.get_stats().operations_completed, 0);
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
}

#[test]
fn test_safe_parallel_process() -> Result<(), BearDogError> {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let mut optimizer = SafeSimdOptimizer::default();
    let input = vec![1, 2, 3, 4, 5];
    
    let result = optimizer.safe_parallel_process(&input, |x| x.wrapping_add(1))?;
    assert_eq!(result, vec![2, 3, 4, 5, 6]);
    assert_eq!(optimizer.get_stats().operations_completed, 1);
    
    Ok(())
}

#[test]
fn test_safe_vectorized_transform() -> Result<(), BearDogError> {
    let mut optimizer = SafeSimdOptimizer::default();
    let input = vec![10, 20, 30];
    
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let result = optimizer.safe_vectorized_transform(&input, |&x| x * 2)?;
    assert_eq!(result, vec![20, 40, 60]);
    
    Ok(())
}

#[test]
fn test_safe_batch_process() -> Result<(), BearDogError> {
    let mut optimizer = SafeSimdOptimizer::default();
    let input = vec![1, 2, 3, 4, 5, 6];
    
    let result = optimizer.safe_batch_process(&input, 2, |batch| {
        batch.iter().map(|&x| x * 2).collect()
    })?;
    assert_eq!(result, vec![2, 4, 6, 8, 10, 12]);
     // TEST_CATEGORY: unit
     // TEST_DOMAIN: core
     // TEST_PRIORITY: normal
    
    Ok(())
}

#[test]
fn test_safe_filter() -> Result<(), BearDogError> {
    let mut optimizer = SafeSimdOptimizer::default();
    let input = vec![1, 2, 3, 4, 5, 6];
    
    let result = optimizer.safe_filter(&input, |&x| x % 2 == 0)?;
    assert_eq!(result, vec![2, 4, 6]);
    
    Ok(())
}

#[test]
fn test_safe_aggregate() -> Result<(), BearDogError> {
    let mut optimizer = SafeSimdOptimizer::default();
    let input = vec![1, 2, 3, 4, 5];
     // TEST_CATEGORY: unit
     // TEST_DOMAIN: core
     // TEST_PRIORITY: normal
    
    let result = optimizer.safe_aggregate(&input, 0, |acc, &x| acc + x)?;
    assert_eq!(result, 15);
    
    Ok(())
}

#[test]
fn test_safe_string_process() -> Result<(), BearDogError> {
    let mut optimizer = SafeSimdOptimizer::default();
    let input = vec!["hello", "world"];
    
    let result = optimizer.safe_string_process(&input, |s| s.to_uppercase())?;
    assert_eq!(result, vec!["HELLO", "WORLD"]);
    
    Ok(())
}
 // TEST_CATEGORY: unit
 // TEST_DOMAIN: core
 // TEST_PRIORITY: normal

#[test]
fn test_safe_numeric_ops() -> Result<(), BearDogError> {
    let mut optimizer = SafeSimdOptimizer::default();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let input = vec![1, 2, 3, 4, 5];
    
    let result = optimizer.safe_numeric_ops(&input, |x| x.checked_mul(2))?;
    assert_eq!(result, vec![2, 4, 6, 8, 10]);
    
    Ok(())
}

#[test]
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_safe_numeric_ops_overflow() {
    let mut optimizer = SafeSimdOptimizer::default();
    let input = vec![u64::MAX];
    
    let result = optimizer.safe_numeric_ops(&input, |x| x.checked_add(1));
    assert!(result.is_err());
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_benchmark_safe_ops() -> Result<(), BearDogError> {
    let mut optimizer = SafeSimdOptimizer::default();
    let test_data = vec![0u8; 1024];
    
    let duration = optimizer.benchmark_safe_ops(&test_data)?;
    assert!(duration.as_nanos() > 0);
    
    Ok(())
}
 // TEST_CATEGORY: unit
 // TEST_DOMAIN: core
 // TEST_PRIORITY: normal

#[test]
fn test_get_performance_info() {
    let optimizer = SafeSimdOptimizer::default();
    let info = optimizer.get_performance_info();
    
    assert!(info.contains_key("safety"));
    assert!(info.contains_key("performance"));
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(info.contains_key("parallelism"));
    assert_eq!(info.get("safety"), Some(&"100% - Zero unsafe blocks".to_string()));
}

// Tests for safe_utils module
#[test]
fn test_safe_parallel_sum() {
    let input = vec![1, 2, 3, 4, 5];
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let result = safe_utils::safe_parallel_sum(&input);
    assert_eq!(result, 15);
}

#[test]
fn test_safe_parallel_sum_overflow_protection() {
    let input = vec![u64::MAX, 1];
    let result = safe_utils::safe_parallel_sum(&input);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(result, u64::MAX); // saturating_add prevents overflow
}

#[test]
fn test_safe_parallel_max() {
    let input = vec![1, 5, 3, 9, 2];
    let result = safe_utils::safe_parallel_max(&input);
    assert_eq!(result, Some(9));
     // TEST_CATEGORY: unit
     // TEST_DOMAIN: core
     // TEST_PRIORITY: important
    
    let empty: Vec<u64> = vec![];
    let result_empty = safe_utils::safe_parallel_max(&empty);
    assert_eq!(result_empty, None);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_safe_parallel_min() {
    let input = vec![5, 1, 9, 3, 2];
    let result = safe_utils::safe_parallel_min(&input);
    assert_eq!(result, Some(1));
    
    let empty: Vec<u64> = vec![];
    let result_empty = safe_utils::safe_parallel_min(&empty);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(result_empty, None);
}

#[test]
fn test_safe_elementwise_add() -> Result<(), BearDogError> {
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let result = safe_utils::safe_elementwise_add(&a, &b)?;
    assert_eq!(result, vec![5, 7, 9]);
     // TEST_CATEGORY: unit
     // TEST_DOMAIN: core
     // TEST_PRIORITY: normal
    
    Ok(())
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: important
#[test]
fn test_safe_elementwise_add_length_mismatch() {
    let a = vec![1, 2, 3];
    let b = vec![4, 5];
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let result = safe_utils::safe_elementwise_add(&a, &b);
    assert!(result.is_err());
}

#[test]
fn test_safe_elementwise_add_overflow() {
    let a = vec![u64::MAX];
    let b = vec![1];
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let result = safe_utils::safe_elementwise_add(&a, &b);
    assert!(result.is_err());
}

#[test]
fn test_safe_pattern_match() {
    let data = vec![0, 1, 2, 3, 1, 2, 4, 5];
    let pattern = vec![1, 2];
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let result = safe_utils::safe_pattern_match(&data, &pattern);
    assert_eq!(result, vec![1, 4]); // Pattern found at indices 1 and 4
}

#[test]
fn test_safe_pattern_match_empty_pattern() {
    let data = vec![1, 2, 3];
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let pattern = vec![];
    let result = safe_utils::safe_pattern_match(&data, &pattern);
    assert!(result.is_empty());
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: important
#[test]
fn test_simd_memory_copy() -> Result<(), String> {
    let mut optimizer = AdvancedSIMDOptimizer::new();
    let src = vec![1, 2, 3, 4, 5];
    let mut dst = vec![0; 5];
     // TEST_CATEGORY: unit
     // TEST_DOMAIN: core
     // TEST_PRIORITY: normal
    
    optimizer.simd_memory_copy(&src, &mut dst)?;
    assert_eq!(dst, src);
    
    Ok(())
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
}

#[test]
fn test_simd_memory_copy_length_mismatch() {
    let mut optimizer = AdvancedSIMDOptimizer::new();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let src = vec![1, 2, 3];
    let mut dst = vec![0; 5];
    
    let result = optimizer.simd_memory_copy(&src, &mut dst);
    assert!(result.is_err());
}

#[test]
fn test_simd_byte_swap() -> Result<(), String> {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let mut optimizer = AdvancedSIMDOptimizer::new();
    let mut data = vec![0x12, 0x34, 0xAB, 0xCD];
    
    optimizer.simd_process_data(&mut data, SIMDOperation::ByteSwap)?;
    assert_eq!(data, vec![0x34, 0x12, 0xCD, 0xAB]);
    
    Ok(())
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
}

#[test]
fn test_optimize_pools() {
    let mut optimizer = AdvancedSIMDOptimizer::new();
    
    // Add some buffers
    for _ in 0..20 {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let buf = optimizer.get_fast_buffer(1024);
        optimizer.return_fast_buffer(buf);
    }
    
    // Optimize pools
    optimizer.optimize_pools();
    
    // Verify optimization worked by checking that we can still get buffers
    let buf = optimizer.get_fast_buffer(512);
    assert_eq!(buf.len(), 512);
}

#[test]
fn test_performance_report() {
    let mut optimizer = AdvancedSIMDOptimizer::new();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let mut data = vec![0u8; 100];
    
    let _ = optimizer.simd_process_data(&mut data, SIMDOperation::XorWithPattern(0xFF));
    
    let report = optimizer.performance_report();
    assert!(report.contains_key("operations_count"));
    assert!(report.contains_key("safety"));
    assert_eq!(report.get("safety"), Some(&"100% - Zero unsafe blocks".to_string()));
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_cache_hit_rate() {
    let optimizer = AdvancedSIMDOptimizer::new();
    assert_eq!(optimizer.cache_hit_rate(), 0.0);
}

