// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive tests for `ultimate_performance` module
//! Focus: Performance optimizations, SIMD operations, statistics

use crate::float_eq;
use crate::ultimate_performance::*;

#[test]
fn test_ultimate_performance_processor_creation() {
    let processor = UltimatePerformanceProcessor::new();
    let stats = processor.get_performance_stats();
    assert_eq!(stats.operations_processed, 0);
}

#[test]
fn test_processor_default() {
    let processor = UltimatePerformanceProcessor::default();
    let stats = processor.get_performance_stats();
    assert_eq!(stats.operations_processed, 0);
}

#[test]
fn test_process_with_ultimate_optimization() {
    let processor = UltimatePerformanceProcessor::new();
    let test_data = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let result = processor.process_with_ultimate_optimization(&test_data);

    assert_eq!(result.len(), test_data.len());
    for (i, &byte) in result.iter().enumerate() {
        assert_eq!(byte, test_data[i].wrapping_add(1));
    }
}

#[test]
fn test_process_empty_data() {
    let processor = UltimatePerformanceProcessor::new();
    let result = processor.process_with_ultimate_optimization(&[]);
    assert_eq!(result.len(), 0);
}

#[test]
fn test_process_single_byte() {
    let processor = UltimatePerformanceProcessor::new();
    let result = processor.process_with_ultimate_optimization(&[5]);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], 6);
}

#[test]
fn test_process_large_buffer() {
    let processor = UltimatePerformanceProcessor::new();
    let large_data = vec![42u8; 10000];

    let result = processor.process_with_ultimate_optimization(&large_data);

    assert_eq!(result.len(), large_data.len());
    for &byte in &result {
        assert_eq!(byte, 43);
    }
}

#[test]
fn test_process_with_overflow_wrapping() {
    let processor = UltimatePerformanceProcessor::new();
    let data = vec![255u8; 8]; // Will wrap to 0

    let result = processor.process_with_ultimate_optimization(&data);

    for &byte in &result {
        assert_eq!(byte, 0); // 255 + 1 wraps to 0
    }
}

#[test]
fn test_performance_stats_tracking() {
    let processor = UltimatePerformanceProcessor::new();

    // Process some data
    let _result = processor.process_with_ultimate_optimization(&[1, 2, 3, 4]);

    let stats = processor.get_performance_stats();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(stats.simd_operations > 0);
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_cache_hit_ratio_calculation() {
    let processor = UltimatePerformanceProcessor::new();

    // Initial ratio should be 0
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let stats = processor.get_performance_stats();
    float_eq::f64(stats.cache_hit_ratio, 0.0);
}

#[test]
fn test_prefetch_effectiveness_calculation() {
    let processor = UltimatePerformanceProcessor::new();
    let stats = processor.get_performance_stats();

    // Initial effectiveness should be 0
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    float_eq::f64(stats.prefetch_effectiveness, 0.0);
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_average_latency_calculation() {
    let processor = UltimatePerformanceProcessor::new();
    let stats = processor.get_performance_stats();

    // Initial latency should be 0 (no operations yet)
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    float_eq::f64(stats.average_latency_ns, 0.0);
}

#[test]
fn test_performance_stats_clone() {
    let processor = UltimatePerformanceProcessor::new();
    let _result = processor.process_with_ultimate_optimization(&[1, 2, 3]);

    let stats1 = processor.get_performance_stats();
    let stats2 = stats1.clone();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: important

    assert_eq!(stats1.operations_processed, stats2.operations_processed);
    float_eq::f64(stats1.cache_hit_ratio, stats2.cache_hit_ratio);
}

#[test]
fn test_operation_type_variants() {
    let _op1 = OperationType::Cryptographic;
    let _op2 = OperationType::NetworkIO;
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let _op3 = OperationType::Memory;
    let _op4 = OperationType::Compute;

    // Test that all variants can be created (no panic means success)
}

#[test]
fn test_operation_type_copy() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let op1 = OperationType::Cryptographic;
    let op2 = op1; // Should be Copy

    // Both should be usable
    let _ = format!("{op1:?}");
    let _ = format!("{op2:?}");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_simd_capabilities_detection() {
    // SIMDCapabilities::detect() is private, so we test indirectly
    // by ensuring the processor can be created (which calls detect internally)
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let _processor = UltimatePerformanceProcessor::new();
    // No panic means processor creation (including CPU detection) succeeded
}

#[test]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "benchmark indices widened for synthetic load; in-range for test vectors"
)]
fn test_process_sequential_data() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let processor = UltimatePerformanceProcessor::new();
    let data: Vec<u8> = (0..255).collect();

    let result = processor.process_with_ultimate_optimization(&data);

    assert_eq!(result.len(), data.len());
    for (i, &value) in result.iter().enumerate().take(255) {
        assert_eq!(value, (i as u8).wrapping_add(1));
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_process_alternating_pattern() {
    let processor = UltimatePerformanceProcessor::new();
    let data = vec![0, 255, 0, 255, 0, 255];

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let result = processor.process_with_ultimate_optimization(&data);

    assert_eq!(result, vec![1, 0, 1, 0, 1, 0]);
}

#[test]
fn test_performance_processor_creation_and_usage() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // UltimatePerformanceProcessor doesn't implement Debug
    // Test that we can create and use it instead
    let processor = UltimatePerformanceProcessor::new();
    let result = processor.process_with_ultimate_optimization(&[1, 2, 3]);
    assert_eq!(result, vec![2, 3, 4]);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_multiple_sequential_operations() {
    let processor = UltimatePerformanceProcessor::new();

    let _result1 = processor.process_with_ultimate_optimization(&[1, 2, 3]);
    let _result2 = processor.process_with_ultimate_optimization(&[4, 5, 6]);
    let _result3 = processor.process_with_ultimate_optimization(&[7, 8, 9]);

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let stats = processor.get_performance_stats();
    assert!(stats.simd_operations >= 3);
}

#[test]
fn test_process_with_alignment_boundary() {
    let processor = UltimatePerformanceProcessor::new();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    // Test with 32-byte aligned data (AVX2 boundary)
    let data = vec![42u8; 32];
    let result = processor.process_with_ultimate_optimization(&data);

    assert_eq!(result.len(), 32);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    for &byte in &result {
        assert_eq!(byte, 43);
    }
}

#[test]
fn test_process_non_aligned_size() {
    let processor = UltimatePerformanceProcessor::new();

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: important
    // Test with non-aligned size (33 bytes)
    let data = vec![100u8; 33];
    let result = processor.process_with_ultimate_optimization(&data);

    assert_eq!(result.len(), 33);
    for &byte in &result {
        assert_eq!(byte, 101);
    }
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_stats_structure_fields() {
    let processor = UltimatePerformanceProcessor::new();
    let stats = processor.get_performance_stats();

    // Verify all fields are accessible
    let _ = stats.operations_processed;
    let _ = stats.total_processing_time_ns;
    let _ = stats.cache_hit_ratio;
    let _ = stats.simd_operations;
    let _ = stats.prefetch_effectiveness;
    let _ = stats.average_latency_ns;
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_zero_byte_values() {
    let processor = UltimatePerformanceProcessor::new();
    let data = vec![0u8; 100];

    let result = processor.process_with_ultimate_optimization(&data);

    for &byte in &result {
        assert_eq!(byte, 1);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
    }
}

#[test]
fn test_max_byte_values() {
    let processor = UltimatePerformanceProcessor::new();
    let data = vec![254u8; 100];

    let result = processor.process_with_ultimate_optimization(&data);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    for &byte in &result {
        assert_eq!(byte, 255);
    }
}

#[test]
fn test_performance_with_repeated_processing() {
    let processor = UltimatePerformanceProcessor::new();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let data = vec![50u8; 1000];

    // Process the same data multiple times
    for _ in 0..10 {
        let result = processor.process_with_ultimate_optimization(&data);
        assert_eq!(result.len(), 1000);
        assert_eq!(result[0], 51);
    }

    let stats = processor.get_performance_stats();
    assert!(stats.simd_operations >= 10);
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_various_buffer_sizes() {
    let processor = UltimatePerformanceProcessor::new();

    // Test with various sizes to ensure robustness
    for size in [1, 7, 15, 16, 31, 32, 63, 64, 127, 128, 255, 256, 1024] {
        let data = vec![100u8; size];
        let result = processor.process_with_ultimate_optimization(&data);

        assert_eq!(result.len(), size);
        for &byte in &result {
            assert_eq!(byte, 101);
        }
    }
}
