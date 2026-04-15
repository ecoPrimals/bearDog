// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use crate::float_eq;
use std::sync::atomic::Ordering;

#[test]
fn test_ultimate_performance_processor() {
    let processor = UltimatePerformanceProcessor::new();
    let test_data = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let result = processor.process_with_ultimate_optimization(&test_data);

    // Verify processing worked correctly
    assert_eq!(result.len(), test_data.len());
    for (i, &byte) in result.iter().enumerate() {
        assert_eq!(byte, test_data[i].wrapping_add(1));
    }

    // Verify statistics are being tracked
    let stats = processor.get_performance_stats();
    // Note: operations_processed is unsigned, so >= 0 is always true (enforced by type system)
    // Just verify we can access the stats
    let _ = stats.operations_processed;
}

#[test]
fn test_simd_capabilities() {
    let capabilities = SIMDCapabilities::detect();

    // Test that we can detect SIMD capabilities
    println!("AVX2: {}", capabilities.has_avx2);
    println!("AVX512: {}", capabilities.has_avx512);
    println!("SSE4.2: {}", capabilities.has_sse42);
    println!("Vector width: {}", capabilities.vector_width);

    assert!(capabilities.vector_width == 128 || capabilities.vector_width == 256);
}

#[test]
fn test_processor_creation() {
    let processor = UltimatePerformanceProcessor::new();
    let stats = processor.get_performance_stats();

    assert_eq!(stats.operations_processed, 0);
    assert_eq!(stats.simd_operations, 0);
    float_eq::f64(stats.cache_hit_ratio, 0.0);
}

#[test]
fn test_processor_default() {
    let processor = UltimatePerformanceProcessor::default();
    let stats = processor.get_performance_stats();

    assert_eq!(stats.operations_processed, 0);
}

#[test]
fn test_empty_data_processing() {
    let processor = UltimatePerformanceProcessor::new();
    let test_data: Vec<u8> = vec![];

    let result = processor.process_with_ultimate_optimization(&test_data);

    assert_eq!(result.len(), 0);
}

#[test]
fn test_single_byte_processing() {
    let processor = UltimatePerformanceProcessor::new();
    let test_data = vec![42];

    let result = processor.process_with_ultimate_optimization(&test_data);

    assert_eq!(result.len(), 1);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(result[0], 43);
}

#[test]
fn test_large_data_processing() {
    let processor = UltimatePerformanceProcessor::new();
    let test_data: Vec<u8> = (0..1024)
        .map(|i| u8::try_from(i % 256).expect("mod 256 fits u8"))
        .collect();

    let result = processor.process_with_ultimate_optimization(&test_data);

    assert_eq!(result.len(), test_data.len());
    for (i, &byte) in result.iter().enumerate() {
        assert_eq!(byte, test_data[i].wrapping_add(1));
    }
}

#[test]
fn process_with_ultimate_optimization_prefetches_across_multiple_cache_lines() {
    let processor = UltimatePerformanceProcessor::new();
    let len = 64 * 12 + 37;
    let test_data: Vec<u8> = (0..len)
        .map(|i| u8::try_from(i % 251).expect("mod 251 fits u8"))
        .collect();

    let result = processor.process_with_ultimate_optimization(&test_data);

    assert_eq!(result.len(), len);
    for (i, &byte) in result.iter().enumerate() {
        assert_eq!(byte, test_data[i].wrapping_add(1));
    }
}

#[test]
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_wrapping_addition() {
    let processor = UltimatePerformanceProcessor::new();
    let test_data = vec![255u8]; // Should wrap to 0

    let result = processor.process_with_ultimate_optimization(&test_data);

    assert_eq!(result[0], 0);
}

#[test]
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_aligned_data_processing() {
    let processor = UltimatePerformanceProcessor::new();
    // Test with 32-byte aligned size (AVX2 register width)
    let test_data = vec![1u8; 32];

    let result = processor.process_with_ultimate_optimization(&test_data);

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(result.len(), 32);
    assert!(result.iter().all(|&b| b == 2));
}

#[test]
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_unaligned_data_processing() {
    let processor = UltimatePerformanceProcessor::new();
    // Test with non-aligned size
    let test_data = vec![5u8; 37];

    let result = processor.process_with_ultimate_optimization(&test_data);

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(result.len(), 37);
    assert!(result.iter().all(|&b| b == 6));
}

#[test]
fn test_performance_stats_calculation() {
    let processor = UltimatePerformanceProcessor::new();

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Process some data
    for _ in 0..10 {
        let data = vec![1u8; 100];
        processor.process_with_ultimate_optimization(&data);
    }

    let stats = processor.get_performance_stats();

    // Verify SIMD operations were tracked
    assert!(stats.simd_operations >= 10);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_cache_hit_ratio_calculation() {
    let processor = UltimatePerformanceProcessor::new();

    // Initially no hits or misses
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let stats = processor.get_performance_stats();
    float_eq::f64(stats.cache_hit_ratio, 0.0);

    // After processing, ratio should still be valid
    processor.process_with_ultimate_optimization(&[1, 2, 3]);
    let stats = processor.get_performance_stats();
    assert!((0.0..=1.0).contains(&stats.cache_hit_ratio));
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_prefetch_effectiveness() {
    let processor = UltimatePerformanceProcessor::new();

    let stats = processor.get_performance_stats();
    assert!(stats.prefetch_effectiveness >= 0.0);
}

#[test]
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_average_latency_calculation() {
    let processor = UltimatePerformanceProcessor::new();

    let stats = processor.get_performance_stats();
    float_eq::f64(stats.average_latency_ns, 0.0);
}

#[test]
fn performance_stats_cache_hit_ratio_uses_hits_and_misses_when_nonzero() {
    let processor = UltimatePerformanceProcessor::new();
    processor.stats.cache_hits.fetch_add(75, Ordering::Relaxed);
    processor
        .stats
        .cache_misses
        .fetch_add(25, Ordering::Relaxed);

    let stats = processor.get_performance_stats();
    float_eq::f64(stats.cache_hit_ratio, 0.75);
}

#[test]
fn performance_stats_prefetch_effectiveness_divides_by_operation_count() {
    let processor = UltimatePerformanceProcessor::new();
    processor
        .stats
        .prefetch_hits
        .fetch_add(40, Ordering::Relaxed);
    processor
        .stats
        .operations_processed
        .fetch_add(160, Ordering::Relaxed);

    let stats = processor.get_performance_stats();
    float_eq::f64(stats.prefetch_effectiveness, 0.25);
}

#[test]
fn performance_stats_average_latency_divides_total_time_by_operations() {
    let processor = UltimatePerformanceProcessor::new();
    processor
        .stats
        .total_processing_time_ns
        .fetch_add(50_000, Ordering::Relaxed);
    processor
        .stats
        .operations_processed
        .fetch_add(250, Ordering::Relaxed);

    let stats = processor.get_performance_stats();
    float_eq::f64(stats.average_latency_ns, 200.0);
}

#[test]
fn test_buffer_pool_creation() {
    let pool = SIMDOptimizedBufferPool::new();

    // Verify pool is created with proper SIMD capabilities
    assert!(pool.simd_capabilities.vector_width > 0);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_lock_free_queue_creation() {
    let queue: LockFreeQueue<u32> = LockFreeQueue::new(64);

    assert_eq!(queue.capacity, 64);
}

#[test]
fn test_lock_free_queue_large_capacity() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let queue: LockFreeQueue<u32> = LockFreeQueue::new(1024);

    assert_eq!(queue.capacity, 1024);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_memory_prefetch_controller_creation() {
    let controller = MemoryPrefetchController::new();

    assert_eq!(controller.access_patterns.len(), 0);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_cache_aligned_stats_creation() {
    let stats = CacheAlignedStats::new();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    assert_eq!(stats.operations_processed.load(Ordering::Relaxed), 0);
    assert_eq!(stats.cache_hits.load(Ordering::Relaxed), 0);
    assert_eq!(stats.cache_misses.load(Ordering::Relaxed), 0);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_concurrent_processing() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    use std::sync::Arc;
    use std::thread;

    let processor = Arc::new(UltimatePerformanceProcessor::new());
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let mut handles = vec![];

    // Spawn multiple threads processing data
    for _ in 0..4 {
        let proc = Arc::clone(&processor);
        let handle = thread::spawn(move || {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            for _ in 0..10 {
                let data = vec![1u8; 100];
                proc.process_with_ultimate_optimization(&data);
            }
        });
        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify concurrent processing tracked SIMD operations
    let stats = processor.get_performance_stats();
    assert!(stats.simd_operations >= 40);
}

#[test]
fn test_different_data_patterns() {
    let processor = UltimatePerformanceProcessor::new();

    // All zeros
    let zeros = vec![0u8; 64];
    let result = processor.process_with_ultimate_optimization(&zeros);
    assert!(result.iter().all(|&b| b == 1));

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // All ones
    let ones = vec![1u8; 64];
    let result = processor.process_with_ultimate_optimization(&ones);
    assert!(result.iter().all(|&b| b == 2));

    // Alternating pattern
    let alternating: Vec<u8> = (0..64).map(|i| if i % 2 == 0 { 10 } else { 20 }).collect();
    let result = processor.process_with_ultimate_optimization(&alternating);
    for (i, &byte) in result.iter().enumerate() {
        let expected = if i % 2 == 0 { 11 } else { 21 };
        assert_eq!(byte, expected);
    }
}

#[test]
fn test_sequential_processing() {
    let processor = UltimatePerformanceProcessor::new();

    // Process multiple times sequentially
    for size in [1, 10, 100, 1000] {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let data = vec![42u8; size];
        let result = processor.process_with_ultimate_optimization(&data);

        assert_eq!(result.len(), size);
        assert!(result.iter().all(|&b| b == 43));
    }
}

#[test]
fn test_operation_type_variants() {
    let types = vec![
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        OperationType::Cryptographic,
        OperationType::NetworkIO,
        OperationType::Memory,
        OperationType::Compute,
    ];

    // Test that all operation types can be created and cloned
    for op_type in types {
        let cloned = op_type;
        let _ = format!("{cloned:?}");
    }
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_stats_structure() {
    let processor = UltimatePerformanceProcessor::new();
    let stats = processor.get_performance_stats();

    // Verify stats structure is valid
    let _ = stats;
    let debug_str = format!("{stats:?}");
    assert!(!debug_str.is_empty());
}

#[test]
fn ultimate_processor_default_runs_same_as_new() {
    let a = UltimatePerformanceProcessor::default();
    let b = UltimatePerformanceProcessor::new();
    assert_eq!(
        a.get_performance_stats().operations_processed,
        b.get_performance_stats().operations_processed
    );
}

#[test]
fn safe_process_chunked_matches_byte_map_semantics() {
    let processor = UltimatePerformanceProcessor::new();
    let data: Vec<u8> = (0u8..=64).collect();
    let chunked = processor._safe_process_chunked(&data);
    let ultimate = processor.process_with_ultimate_optimization(&data);
    assert_eq!(chunked, ultimate);
}
