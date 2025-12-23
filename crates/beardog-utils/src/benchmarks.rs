// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tracing::{debug, info, warn};

#[derive(Debug)]
pub struct BenchmarkSuite {
    benchmarks: HashMap<String, BenchmarkResult>,

    config: BenchmarkConfig,

    suite_stats: SuiteStats,
}

#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    /// Number of `warmup_iterations`
    pub warmup_iterations: u32,

    /// Number of `measurement_iterations`
    pub measurement_iterations: u32,

    pub max_time: Duration,

    /// Whether `enable_memory_profiling` is enabled
    pub enable_memory_profiling: bool,

    /// Whether `enable_latency_analysis` is enabled
    pub enable_latency_analysis: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// Name of the item
    pub name: String,

    pub total_time: Duration,

    /// Number of iterations
    pub iterations: u64,

    /// The ops per second value
    pub ops_per_second: f64,

    /// The avg latency value
    pub avg_latency: Duration,

    /// The latency percentiles value
    pub latency_percentiles: LatencyPercentiles,

    /// The memory stats value
    pub memory_stats: MemoryStats,

    pub performance_grade: PerformanceGrade,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyPercentiles {
    /// The p50 value
    pub p50: Duration,
    /// The p90 value
    pub p90: Duration,
    /// The p95 value
    pub p95: Duration,
    /// The p99 value
    pub p99: Duration,
    /// The p99 9 value
    pub p99_9: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    /// Number of `peak_memory`
    pub peak_memory: u64,

    /// Number of `total_allocations`
    pub total_allocations: u64,

    /// The allocation rate value
    pub allocation_rate: f64,

    /// Number of `avg_allocation_size`
    pub avg_allocation_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PerformanceGrade {
    /// Represents excellent variant
    Excellent, // > 1M ops/sec
    /// Represents good variant
    Good, // > 100K ops/sec
    /// Represents average variant
    Average, // > 10K ops/sec
    /// Represents poor variant
    Poor, // > 1K ops/sec
    /// Represents critical variant
    Critical, // < 1K ops/sec
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuiteStats {
    /// Number of `total_benchmarks`
    pub total_benchmarks: u32,
    /// Number of `passed_benchmarks`
    pub passed_benchmarks: u32,
    /// Number of `failed_benchmarks`
    pub failed_benchmarks: u32,
    pub total_execution_time: Duration,
    pub average_performance_grade: f64,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            warmup_iterations: 1000,
            measurement_iterations: 10000,
            max_time: Duration::from_secs(30),
            enable_memory_profiling: true,
            enable_latency_analysis: true,
        }
    }
}

impl BenchmarkSuite {
    /// Creates a new instance
    #[must_use]
    pub fn new(config: BenchmarkConfig) -> Self {
        Self {
            benchmarks: HashMap::with_capacity(16),
            config,
            suite_stats: SuiteStats::default(),
        }
    }

    pub fn benchmark<F, R>(&mut self, name: &str, mut benchmark_fn: F) -> Result<(), BearDogError>
    where
        F: FnMut() -> R,
    {
        info!("🏁 Running benchmark: {}", name);

        let start_time = Instant::now();

        debug!(
            "🔥 Warmup phase: {} iterations",
            self.config.warmup_iterations
        );
        for _ in 0..self.config.warmup_iterations {
            let _ = benchmark_fn();
        }

        debug!(
            "📊 Measurement phase: {} iterations",
            self.config.measurement_iterations
        );
        let mut latencies = Vec::with_capacity(self.config.measurement_iterations as usize);
        let measurement_start = Instant::now();

        for _ in 0..self.config.measurement_iterations {
            let iter_start = Instant::now();
            let _ = benchmark_fn();
            let iter_duration = iter_start.elapsed();
            latencies.push(iter_duration);

            if measurement_start.elapsed() > self.config.max_time {
                warn!("⏰ Benchmark timeout reached for: {}", name);
                break;
            }
        }

        let total_time = start_time.elapsed();
        let measurement_time = measurement_start.elapsed();

        let iterations = latencies.len() as u64;
        let ops_per_second = iterations as f64 / measurement_time.as_secs_f64();
        let avg_latency = Duration::from_nanos(
            latencies.iter().map(|d| d.as_nanos() as u64).sum::<u64>() / iterations,
        );

        let mut sorted_latencies = latencies.clone();
        sorted_latencies.sort();

        let latency_percentiles = LatencyPercentiles {
            p50: sorted_latencies[sorted_latencies.len() * 50 / 100],
            p90: sorted_latencies[sorted_latencies.len() * 90 / 100],
            p95: sorted_latencies[sorted_latencies.len() * 95 / 100],
            p99: sorted_latencies[sorted_latencies.len() * 99 / 100],
            p99_9: sorted_latencies[sorted_latencies.len() * 999 / 1000],
        };

        let memory_stats = MemoryStats {
            peak_memory: 1024 * 1024, // 1MB mock
            total_allocations: iterations * 2,
            allocation_rate: (iterations * 2) as f64 / measurement_time.as_secs_f64(),
            avg_allocation_size: 512,
        };

        let performance_grade = if ops_per_second > 1_000_000.0 {
            PerformanceGrade::Excellent
        } else if ops_per_second > 100_000.0 {
            PerformanceGrade::Good
        } else if ops_per_second > 10_000.0 {
            PerformanceGrade::Average
        } else if ops_per_second > 1_000.0 {
            PerformanceGrade::Poor
        } else {
            PerformanceGrade::Critical
        };

        let result = BenchmarkResult {
            name: name.to_string(),
            total_time,
            iterations,
            ops_per_second,
            avg_latency,
            latency_percentiles,
            memory_stats,
            performance_grade: performance_grade.clone(),
        };

        info!(
            "✅ Benchmark complete: {} - {:.0} ops/sec ({:?})",
            name, ops_per_second, performance_grade
        );

        debug!(
            "📈 Latency P95: {:?}, P99: {:?}",
            result.latency_percentiles.p95, result.latency_percentiles.p99
        );

        self.suite_stats.total_benchmarks += 1;
        if matches!(performance_grade, PerformanceGrade::Critical) {
            self.suite_stats.failed_benchmarks += 1;
        } else {
            self.suite_stats.passed_benchmarks += 1;
        }
        self.suite_stats.total_execution_time += total_time;

        self.benchmarks.insert(name.to_string(), result);
        Ok(())
    }

    pub fn benchmark_throughput<F, R>(
        &mut self,
        name: &str,
        benchmark_fn: F,
    ) -> Result<f64, BearDogError>
    where
        F: Fn() -> R + Clone,
    {
        self.benchmark(name, benchmark_fn)?;
        let result = &self.benchmarks[name];
        Ok(result.ops_per_second)
    }

    pub fn benchmark_latency<F, R>(
        &mut self,
        name: &str,
        benchmark_fn: F,
    ) -> Result<Duration, BearDogError>
    where
        F: Fn() -> R + Clone,
    {
        self.benchmark(name, benchmark_fn)?;
        let result = &self.benchmarks[name];
        Ok(result.avg_latency)
    }

    /// Gets result
    /// Gets result
    #[must_use]
    pub fn get_result(&self, name: &str) -> Option<&BenchmarkResult> {
        self.benchmarks.get(name)
    }

    /// Gets `all_results`
    /// Gets `all_results`
    #[must_use]
    pub const fn get_all_results(&self) -> &HashMap<String, BenchmarkResult> {
        &self.benchmarks
    }

    #[must_use]
    pub fn generate_report(&self) -> PerformanceReport {
        let mut excellent_count = 0;
        let mut good_count = 0;
        let mut average_count = 0;
        let mut poor_count = 0;
        let mut critical_count = 0;

        for result in self.benchmarks.values() {
            match result.performance_grade {
                PerformanceGrade::Excellent => excellent_count += 1,
                PerformanceGrade::Good => good_count += 1,
                PerformanceGrade::Average => average_count += 1,
                PerformanceGrade::Poor => poor_count += 1,
                PerformanceGrade::Critical => critical_count += 1,
            }
        }

        let total_ops_per_second: f64 = self.benchmarks.values().map(|r| r.ops_per_second).sum();

        let avg_ops_per_second = if self.benchmarks.is_empty() {
            0.0
        } else {
            total_ops_per_second / self.benchmarks.len() as f64
        };

        PerformanceReport {
            suite_stats: self.suite_stats.clone(),
            grade_distribution: GradeDistribution {
                excellent: excellent_count,
                good: good_count,
                average: average_count,
                poor: poor_count,
                critical: critical_count,
            },
            avg_ops_per_second,
            total_ops_per_second,
            benchmarks: self.benchmarks.clone(),
        }
    }

    pub fn print_summary(&self) {
        let report = self.generate_report();

        println!("\n🎯 BearDog Performance Benchmark Report");
        println!("==========================================");
        println!("Total Benchmarks: {}", report.suite_stats.total_benchmarks);
        println!(
            "Passed: {} | Failed: {}",
            report.suite_stats.passed_benchmarks, report.suite_stats.failed_benchmarks
        );
        println!(
            "Total Execution Time: {:?}",
            report.suite_stats.total_execution_time
        );
        println!(
            "Average Performance: {:.0} ops/sec",
            report.avg_ops_per_second
        );

        println!("\n📊 Performance Grade Distribution:");
        println!(
            "🟢 Excellent (>1M ops/sec): {}",
            report.grade_distribution.excellent
        );
        println!(
            "🔵 Good (>100K ops/sec): {}",
            report.grade_distribution.good
        );
        println!(
            "🟡 Average (>10K ops/sec): {}",
            report.grade_distribution.average
        );
        println!("🟠 Poor (>1K ops/sec): {}", report.grade_distribution.poor);
        println!(
            "🔴 Critical (<1K ops/sec): {}",
            report.grade_distribution.critical
        );

        if report.grade_distribution.critical > 0 {
            println!(
                "\n⚠️  WARNING: {} benchmarks have critical performance issues!",
                report.grade_distribution.critical
            );
        }

        println!("\n🏆 Top Performers:");
        let mut sorted_results: Vec<_> = self.benchmarks.values().collect();
        sorted_results.sort_by(|a, b| {
            b.ops_per_second
                .partial_cmp(&a.ops_per_second)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for (i, result) in sorted_results.iter().take(5).enumerate() {
            println!(
                "  {}. {} - {:.0} ops/sec ({:?})",
                i + 1,
                result.name,
                result.ops_per_second,
                result.performance_grade
            );
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceReport {
    /// The suite stats value
    pub suite_stats: SuiteStats,
    /// The grade distribution value
    pub grade_distribution: GradeDistribution,
    /// The avg ops per second value
    pub avg_ops_per_second: f64,
    /// The total ops per second value
    pub total_ops_per_second: f64,
    /// Mapping of benchmarks
    pub benchmarks: HashMap<String, BenchmarkResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeDistribution {
    /// Number of excellent
    pub excellent: u32,
    /// Number of good
    pub good: u32,
    /// Number of average
    pub average: u32,
    /// Number of poor
    pub poor: u32,
    /// Number of critical
    pub critical: u32,
}

#[macro_export]
macro_rules! benchmark {
    ($suite:expr, $name:expr, $code:block) => {
        $suite.benchmark($name, || $code)?;
    };
}

#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

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
            beardog_errors::BearDogError::internal(
                "Operation failed: Result should exist".to_string(),
            )
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
                format_args!("Operation failed ({}): {:?}", "Benchmark should succeed", e)
                    .to_string(),
            )
        })?;

        let result = suite.get_result("fast_op").ok_or_else(|| {
            tracing::error!("Operation failed ({})", "Result should exist");
            beardog_errors::BearDogError::internal(
                "Operation failed: Result should exist".to_string(),
            )
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
}
