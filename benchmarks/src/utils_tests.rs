// SPDX-License-Identifier: AGPL-3.0-only
//! Comprehensive tests for benchmark utilities
//!
//! These tests ensure our benchmark statistics calculations are accurate
//! and handle edge cases correctly.

use super::utils::*;
use std::time::Duration;

#[cfg(test)]
#[allow(clippy::float_cmp)] // Tests compare exact values
#[test]
fn test_benchmark_stats_default() {
    let stats = BenchmarkStats::default();

    assert_eq!(stats.mean_ns, 0.0);
    assert_eq!(stats.std_dev_ns, 0.0);
    assert_eq!(stats.min_ns, 0.0);
    assert_eq!(stats.max_ns, 0.0);
    assert_eq!(stats.count, 0);
}

#[test]
fn test_benchmark_stats_clone() {
    let stats = BenchmarkStats {
        mean_ns: 100.0,
        std_dev_ns: 10.0,
        min_ns: 90.0,
        max_ns: 110.0,
        count: 5,
    };

    let cloned = stats.clone();
    assert_eq!(stats.mean_ns, cloned.mean_ns);
    assert_eq!(stats.std_dev_ns, cloned.std_dev_ns);
    assert_eq!(stats.count, cloned.count);
}

#[test]
fn test_calculate_stats_empty_measurements() {
    let measurements: Vec<Duration> = vec![];
    let stats = calculate_stats(&measurements);

    assert_eq!(stats.mean_ns, 0.0);
    assert_eq!(stats.std_dev_ns, 0.0);
    assert_eq!(stats.min_ns, 0.0);
    assert_eq!(stats.max_ns, 0.0);
    assert_eq!(stats.count, 0);
}

#[test]
fn test_calculate_stats_single_measurement() {
    let measurements = vec![Duration::from_nanos(100)];
    let stats = calculate_stats(&measurements);

    assert_eq!(stats.mean_ns, 100.0);
    assert_eq!(stats.std_dev_ns, 0.0); // No variation with one sample
    assert_eq!(stats.min_ns, 100.0);
    assert_eq!(stats.max_ns, 100.0);
    assert_eq!(stats.count, 1);
}

#[test]
fn test_calculate_stats_multiple_identical_measurements() {
    let measurements = vec![
        Duration::from_nanos(100),
        Duration::from_nanos(100),
        Duration::from_nanos(100),
    ];
    let stats = calculate_stats(&measurements);

    assert_eq!(stats.mean_ns, 100.0);
    assert_eq!(stats.std_dev_ns, 0.0); // No variation
    assert_eq!(stats.min_ns, 100.0);
    assert_eq!(stats.max_ns, 100.0);
    assert_eq!(stats.count, 3);
}

#[test]
fn test_calculate_stats_varying_measurements() {
    let measurements = vec![
        Duration::from_nanos(100),
        Duration::from_nanos(200),
        Duration::from_nanos(300),
    ];
    let stats = calculate_stats(&measurements);

    assert_eq!(stats.mean_ns, 200.0);
    assert!(stats.std_dev_ns > 0.0); // Should have variation
    assert_eq!(stats.min_ns, 100.0);
    assert_eq!(stats.max_ns, 300.0);
    assert_eq!(stats.count, 3);
}

#[test]
fn test_calculate_stats_large_numbers() {
    let measurements = vec![
        Duration::from_millis(1), // 1,000,000 ns
        Duration::from_millis(2), // 2,000,000 ns
        Duration::from_millis(3), // 3,000,000 ns
    ];
    let stats = calculate_stats(&measurements);

    assert_eq!(stats.mean_ns, 2_000_000.0);
    assert_eq!(stats.min_ns, 1_000_000.0);
    assert_eq!(stats.max_ns, 3_000_000.0);
    assert_eq!(stats.count, 3);
}

#[test]
fn test_calculate_stats_microsecond_precision() {
    let measurements = vec![
        Duration::from_micros(100),
        Duration::from_micros(150),
        Duration::from_micros(200),
    ];
    let stats = calculate_stats(&measurements);

    assert_eq!(stats.mean_ns, 150_000.0);
    assert_eq!(stats.min_ns, 100_000.0);
    assert_eq!(stats.max_ns, 200_000.0);
    assert_eq!(stats.count, 3);
}

#[test]
fn test_calculate_stats_many_measurements() {
    let measurements: Vec<Duration> = (1..=100).map(|i| Duration::from_nanos(i * 10)).collect();

    let stats = calculate_stats(&measurements);

    assert_eq!(stats.count, 100);
    assert_eq!(stats.min_ns, 10.0);
    assert_eq!(stats.max_ns, 1000.0);
    assert!((400.0..600.0).contains(&stats.mean_ns)); // Should be around 505
    assert!(stats.std_dev_ns > 0.0);
}

#[test]
fn test_calculate_stats_standard_deviation_calculation() {
    // Known data set: [10, 20, 30]
    // Mean = 20
    // Variance = ((10-20)^2 + (20-20)^2 + (30-20)^2) / 3 = (100 + 0 + 100) / 3 = 66.67
    // Std Dev = sqrt(66.67) ≈ 8.16
    let measurements = vec![
        Duration::from_nanos(10),
        Duration::from_nanos(20),
        Duration::from_nanos(30),
    ];
    let stats = calculate_stats(&measurements);

    assert_eq!(stats.mean_ns, 20.0);
    assert!((stats.std_dev_ns - 8.16).abs() < 0.2); // Allow small floating point error
}

#[test]
fn test_calculate_stats_outliers() {
    let measurements = vec![
        Duration::from_nanos(100),
        Duration::from_nanos(100),
        Duration::from_nanos(100),
        Duration::from_nanos(1000), // Outlier
    ];
    let stats = calculate_stats(&measurements);

    assert_eq!(stats.min_ns, 100.0);
    assert_eq!(stats.max_ns, 1000.0);
    assert!(stats.std_dev_ns > 0.0); // Should show high variation
    assert_eq!(stats.count, 4);
}

#[test]
fn test_benchmark_stats_debug_format() {
    let stats = BenchmarkStats {
        mean_ns: 123.45,
        std_dev_ns: 12.34,
        min_ns: 100.0,
        max_ns: 150.0,
        count: 10,
    };

    let debug_str = format!("{stats:?}");
    assert!(debug_str.contains("mean_ns"));
    assert!(debug_str.contains("123.45"));
    assert!(debug_str.contains("count"));
    assert!(debug_str.contains("10"));
}

#[test]
fn test_calculate_stats_preserves_precision() {
    // Test that we don't lose precision with small durations
    let measurements = vec![
        Duration::from_nanos(1),
        Duration::from_nanos(2),
        Duration::from_nanos(3),
    ];
    let stats = calculate_stats(&measurements);

    assert_eq!(stats.mean_ns, 2.0);
    assert_eq!(stats.min_ns, 1.0);
    assert_eq!(stats.max_ns, 3.0);
}

#[test]
fn test_calculate_stats_handles_zero_durations() {
    let measurements = vec![
        Duration::from_nanos(0),
        Duration::from_nanos(100),
        Duration::from_nanos(200),
    ];
    let stats = calculate_stats(&measurements);

    assert_eq!(stats.min_ns, 0.0);
    assert_eq!(stats.max_ns, 200.0);
    assert_eq!(stats.count, 3);
}
