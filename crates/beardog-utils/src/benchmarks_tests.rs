#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]

// TEST_CATEGORY: unit
// TEST_DOMAIN: performance
// TEST_PRIORITY: normal

use super::*;
use crate::float_eq;

#[test]
fn test_benchmark_suite_creation() {
    let suite = BenchmarkSuite::new(BenchmarkConfig::default());
    assert_eq!(suite.benchmarks.len(), 0);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_simple_benchmark() -> Result<(), Box<dyn std::error::Error>> {
    let mut suite = BenchmarkSuite::new(BenchmarkConfig {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        warmup_iterations: 10,
        measurement_iterations: 100,
        ..BenchmarkConfig::default()
    });

    suite
        .benchmark("simple_add", || {
            let _result = 1 + 1;
        })
        .map_err(|e| {
            tracing::error!("Operation failed ({}): {:?}", "Benchmark should succeed", e);
            beardog_errors::BearDogError::internal(
                format_args!("Operation failed ({}): {:?}", "Benchmark should succeed", e)
                    .to_string(),
            )
        })?;

    let result = suite.get_result("simple_add").ok_or_else(|| {
        tracing::error!("Operation failed ({})", "Result should exist");
        beardog_errors::BearDogError::internal("Operation failed: Result should exist".to_string())
    })?;
    assert!(result.ops_per_second > 0.0);
    assert_eq!(result.iterations, 100);
    Ok(())
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_performance_grading() -> Result<(), Box<dyn std::error::Error>> {
    let mut suite = BenchmarkSuite::new(BenchmarkConfig {
        warmup_iterations: 10,
        measurement_iterations: 100_000, // Increased to ensure sufficient ops/sec
        ..BenchmarkConfig::default()
    });

    suite.benchmark("fast_op", || {}).map_err(|e| {
        tracing::error!("Operation failed ({}): {:?}", "Benchmark should succeed", e);
        beardog_errors::BearDogError::internal(
            format_args!("Operation failed ({}): {:?}", "Benchmark should succeed", e).to_string(),
        )
    })?;

    let result = suite.get_result("fast_op").ok_or_else(|| {
        tracing::error!("Operation failed ({})", "Result should exist");
        beardog_errors::BearDogError::internal("Operation failed: Result should exist".to_string())
    })?;

    assert!(matches!(
        result.performance_grade,
        PerformanceGrade::Excellent | PerformanceGrade::Good
    ));
    Ok(())
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_benchmark_config_default() {
    let config = BenchmarkConfig::default();
    assert_eq!(config.warmup_iterations, 1000); // Actual default from code
    assert_eq!(config.measurement_iterations, 10000); // Actual default from code
    assert_eq!(config.max_time, Duration::from_secs(30)); // Actual default from code
    assert!(config.enable_memory_profiling); // Actual default from code
    assert!(config.enable_latency_analysis); // Actual default from code
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_latency_percentiles() {
    let percentiles = LatencyPercentiles {
        p50: Duration::from_millis(1),
        p90: Duration::from_millis(2),
        p95: Duration::from_millis(3),
        p99: Duration::from_millis(5),
        p99_9: Duration::from_millis(10),
    };

    assert!(percentiles.p50 < percentiles.p90);
    assert!(percentiles.p90 < percentiles.p95);
    assert!(percentiles.p95 < percentiles.p99);
    assert!(percentiles.p99 < percentiles.p99_9);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_memory_stats() {
    let stats = MemoryStats {
        peak_memory: 1024 * 1024, // 1MB
        total_allocations: 100,
        allocation_rate: 10.0,
        avg_allocation_size: 10240,
    };

    assert_eq!(stats.peak_memory, 1024 * 1024);
    assert_eq!(stats.total_allocations, 100);
    assert!(stats.allocation_rate > 0.0);
    assert!(stats.avg_allocation_size > 0);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_performance_grade_ordering() {
    assert!(PerformanceGrade::Excellent != PerformanceGrade::Good);
    assert!(PerformanceGrade::Good != PerformanceGrade::Average);
    assert!(PerformanceGrade::Average != PerformanceGrade::Poor);
    assert!(PerformanceGrade::Poor != PerformanceGrade::Critical);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_suite_stats() {
    let suite = BenchmarkSuite::new(BenchmarkConfig::default());
    let stats = &suite.suite_stats;

    assert_eq!(stats.total_benchmarks, 0);
    assert_eq!(stats.passed_benchmarks, 0);
    assert_eq!(stats.failed_benchmarks, 0);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_benchmark_result_serialization() -> Result<(), Box<dyn std::error::Error>> {
    let result = BenchmarkResult {
        name: "test".to_string(),
        total_time: Duration::from_secs(1),
        iterations: 100,
        ops_per_second: 100.0,
        avg_latency: Duration::from_millis(10),
        latency_percentiles: LatencyPercentiles {
            p50: Duration::from_millis(5),
            p90: Duration::from_millis(10),
            p95: Duration::from_millis(15),
            p99: Duration::from_millis(20),
            p99_9: Duration::from_millis(30),
        },
        memory_stats: MemoryStats {
            peak_memory: 1024,
            total_allocations: 10,
            allocation_rate: 1.0,
            avg_allocation_size: 102,
        },
        performance_grade: PerformanceGrade::Good,
    };

    let serialized = serde_json::to_string(&result)?;
    let deserialized: BenchmarkResult = serde_json::from_str(&serialized)?;

    assert_eq!(result.name, deserialized.name);
    assert_eq!(result.iterations, deserialized.iterations);
    assert_eq!(result.performance_grade, deserialized.performance_grade);
    Ok(())
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_multiple_benchmarks() -> Result<(), Box<dyn std::error::Error>> {
    let mut suite = BenchmarkSuite::new(BenchmarkConfig {
        warmup_iterations: 5,
        measurement_iterations: 50,
        ..BenchmarkConfig::default()
    });

    suite.benchmark("test1", || {
        std::hint::black_box(1 + 1);
    })?;

    suite.benchmark("test2", || {
        std::hint::black_box(2 * 2);
    })?;

    assert!(suite.get_result("test1").is_some());
    assert!(suite.get_result("test2").is_some());
    assert!(suite.get_result("nonexistent").is_none());
    Ok(())
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_generate_report() -> Result<(), Box<dyn std::error::Error>> {
    let mut suite = BenchmarkSuite::new(BenchmarkConfig {
        warmup_iterations: 5,
        measurement_iterations: 100,
        ..BenchmarkConfig::default()
    });

    suite.benchmark("test_op", || {
        std::hint::black_box(42);
    })?;

    let report = suite.generate_report();
    assert!(report.suite_stats.total_benchmarks > 0);
    assert!(!report.benchmarks.is_empty());
    Ok(())
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: performance
// TEST_PRIORITY: normal
#[test]
fn test_generate_report_empty_suite() {
    let suite = BenchmarkSuite::new(BenchmarkConfig::default());
    let report = suite.generate_report();
    assert_eq!(report.suite_stats.total_benchmarks, 0);
    float_eq::f64(report.avg_ops_per_second, 0.0);
    float_eq::f64(report.total_ops_per_second, 0.0);
    assert!(report.benchmarks.is_empty());
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: performance
// TEST_PRIORITY: normal
#[test]
fn test_benchmark_throughput_and_latency_helpers() -> Result<(), Box<dyn std::error::Error>> {
    let mut suite = BenchmarkSuite::new(BenchmarkConfig {
        warmup_iterations: 2,
        measurement_iterations: 20,
        ..BenchmarkConfig::default()
    });

    let ops = suite.benchmark_throughput("tp", || std::hint::black_box(0u8))?;
    assert!(ops > 0.0);

    let mut suite2 = BenchmarkSuite::new(BenchmarkConfig {
        warmup_iterations: 2,
        measurement_iterations: 20,
        ..BenchmarkConfig::default()
    });
    let lat = suite2.benchmark_latency("lat", || std::hint::black_box(0u8))?;
    assert!(!lat.is_zero());

    Ok(())
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: performance
// TEST_PRIORITY: normal
#[test]
fn test_benchmark_measurement_stops_on_max_time() -> Result<(), Box<dyn std::error::Error>> {
    let mut suite = BenchmarkSuite::new(BenchmarkConfig {
        warmup_iterations: 0,
        measurement_iterations: 1_000_000,
        max_time: Duration::from_millis(1),
        ..BenchmarkConfig::default()
    });

    suite.benchmark("timeout_branch", || {
        std::thread::sleep(Duration::from_millis(5));
    })?;

    let result = suite.get_result("timeout_branch").expect("result");
    assert!(
        result.iterations < 1_000_000,
        "expected early exit from max_time"
    );
    Ok(())
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: performance
// TEST_PRIORITY: normal
#[test]
fn test_critical_grade_and_failed_suite_counter() -> Result<(), Box<dyn std::error::Error>> {
    let mut suite = BenchmarkSuite::new(BenchmarkConfig {
        warmup_iterations: 0,
        measurement_iterations: 5,
        max_time: Duration::from_secs(30),
        ..BenchmarkConfig::default()
    });

    suite.benchmark("deliberately_slow", || {
        std::thread::sleep(Duration::from_millis(5));
    })?;

    let result = suite.get_result("deliberately_slow").expect("result");
    assert_eq!(result.performance_grade, PerformanceGrade::Critical);
    assert!(suite.suite_stats.failed_benchmarks >= 1);
    Ok(())
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: performance
// TEST_PRIORITY: normal
#[test]
fn test_benchmark_macro_expands() -> Result<(), Box<dyn std::error::Error>> {
    let mut suite = BenchmarkSuite::new(BenchmarkConfig {
        warmup_iterations: 0,
        measurement_iterations: 3,
        ..BenchmarkConfig::default()
    });
    crate::benchmark! {
        suite,
        "macro_named",
        {
            std::hint::black_box(7u8.wrapping_mul(3));
        }
    };
    assert!(suite.get_result("macro_named").is_some());
    Ok(())
}

#[cfg(test)]
mod print_summary_smoke_tests {
    #![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: performance
    // TEST_PRIORITY: normal

    use super::{BenchmarkConfig, BenchmarkSuite, PerformanceGrade};

    #[test]
    fn print_summary_smoke_runs() {
        let mut suite = BenchmarkSuite::new(BenchmarkConfig {
            warmup_iterations: 0,
            measurement_iterations: 2,
            ..BenchmarkConfig::default()
        });
        suite.benchmark("smoke", || {}).expect("benchmark");
        suite
            .benchmark("crit", || {
                std::thread::sleep(std::time::Duration::from_millis(4));
            })
            .expect("slow bench");
        suite.print_summary();
        let r = suite.get_result("crit").expect("crit");
        assert_eq!(r.performance_grade, PerformanceGrade::Critical);
    }
}
