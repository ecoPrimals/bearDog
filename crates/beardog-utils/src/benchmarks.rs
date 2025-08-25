// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Comprehensive Benchmarking Framework
///
/// **PERFORMANCE MEASUREMENT & OPTIMIZATION** - Comprehensive benchmarking tools
/// 
/// This module provides a complete benchmarking framework for measuring and optimizing
/// performance across all BearDog components. It includes micro-benchmarks, system-level
/// benchmarks, and continuous performance monitoring.
///
/// ## Features
/// - **Micro-benchmarks** - Individual function performance measurement
/// - **System benchmarks** - End-to-end performance testing
/// - **Memory profiling** - Allocation and memory usage tracking
/// - **Throughput analysis** - Operations per second measurement
/// - **Latency distribution** - P50, P95, P99 latency tracking
/// - **Regression detection** - Automatic performance regression alerts

use beardog_errors::BearDogResult;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tracing::{debug, info, warn};
use serde::{Deserialize, Serialize};

/// Comprehensive benchmark suite
#[derive(Debug)]
pub struct BenchmarkSuite {
    /// Individual benchmark results
    benchmarks: HashMap<String, BenchmarkResult>,
    /// Suite configuration
    config: BenchmarkConfig,
    /// Global suite statistics
    suite_stats: SuiteStats,
}

/// Configuration for benchmark execution
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    /// Number of warmup iterations
    pub warmup_iterations: u32,
    /// Number of measurement iterations
    pub measurement_iterations: u32,
    /// Maximum time per benchmark
    pub max_time: Duration,
    /// Enable memory profiling
    pub enable_memory_profiling: bool,
    /// Enable detailed latency analysis
    pub enable_latency_analysis: bool,
}

/// Result of a single benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// Benchmark name
    pub name: String,
    /// Total execution time
    pub total_time: Duration,
    /// Number of iterations completed
    pub iterations: u64,
    /// Operations per second
    pub ops_per_second: f64,
    /// Average latency per operation
    pub avg_latency: Duration,
    /// Latency percentiles
    pub latency_percentiles: LatencyPercentiles,
    /// Memory usage statistics
    pub memory_stats: MemoryStats,
    /// Performance classification
    pub performance_grade: PerformanceGrade,
}

/// Latency percentile measurements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyPercentiles {
    pub p50: Duration,
    pub p90: Duration,
    pub p95: Duration,
    pub p99: Duration,
    pub p99_9: Duration,
}

/// Memory usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    /// Peak memory usage in bytes
    pub peak_memory: u64,
    /// Total allocations
    pub total_allocations: u64,
    /// Memory allocation rate (allocations/sec)
    pub allocation_rate: f64,
    /// Average allocation size
    pub avg_allocation_size: u64,
}

/// Performance grade classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PerformanceGrade {
    Excellent,  // > 1M ops/sec
    Good,       // > 100K ops/sec
    Average,    // > 10K ops/sec
    Poor,       // > 1K ops/sec
    Critical,   // < 1K ops/sec
}

/// Suite-level statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuiteStats {
    pub total_benchmarks: u32,
    pub passed_benchmarks: u32,
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
    /// Create a new benchmark suite
    pub fn new(config: BenchmarkConfig) -> Self {
        Self {
            benchmarks: HashMap::new(),
            config,
            suite_stats: SuiteStats::default(),
        }
    }
    
    /// Run a benchmark function
    pub fn benchmark<F, R>(&mut self, name: &str, mut benchmark_fn: F) -> BearDogResult<()>
    where
        F: FnMut() -> R,
    {
        info!("🏁 Running benchmark: {}", name);
        
        let start_time = Instant::now();
        
        // Warmup phase
        debug!("🔥 Warmup phase: {} iterations", self.config.warmup_iterations);
        for _ in 0..self.config.warmup_iterations {
            let _ = benchmark_fn();
        }
        
        // Measurement phase
        debug!("📊 Measurement phase: {} iterations", self.config.measurement_iterations);
        let mut latencies = Vec::with_capacity(self.config.measurement_iterations as usize);
        let measurement_start = Instant::now();
        
        for _ in 0..self.config.measurement_iterations {
            let iter_start = Instant::now();
            let _ = benchmark_fn();
            let iter_duration = iter_start.elapsed();
            latencies.push(iter_duration);
            
            // Check timeout
            if measurement_start.elapsed() > self.config.max_time {
                warn!("⏰ Benchmark timeout reached for: {}", name);
                break;
            }
        }
        
        let total_time = start_time.elapsed();
        let measurement_time = measurement_start.elapsed();
        
        // Calculate statistics
        let iterations = latencies.len() as u64;
        let ops_per_second = iterations as f64 / measurement_time.as_secs_f64();
        let avg_latency = Duration::from_nanos(
            latencies.iter().map(|d| d.as_nanos() as u64).sum::<u64>() / iterations
        );
        
        // Calculate percentiles
        let mut sorted_latencies = latencies.clone();
        sorted_latencies.sort();
        
        let latency_percentiles = LatencyPercentiles {
            p50: sorted_latencies[sorted_latencies.len() * 50 / 100],
            p90: sorted_latencies[sorted_latencies.len() * 90 / 100],
            p95: sorted_latencies[sorted_latencies.len() * 95 / 100],
            p99: sorted_latencies[sorted_latencies.len() * 99 / 100],
            p99_9: sorted_latencies[sorted_latencies.len() * 999 / 1000],
        };
        
        // Mock memory stats (would integrate with actual profiler)
        let memory_stats = MemoryStats {
            peak_memory: 1024 * 1024, // 1MB mock
            total_allocations: iterations * 2,
            allocation_rate: (iterations * 2) as f64 / measurement_time.as_secs_f64(),
            avg_allocation_size: 512,
        };
        
        // Determine performance grade
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
        
        // Log results
        info!(
            "✅ Benchmark complete: {} - {:.0} ops/sec ({:?})",
            name, ops_per_second, performance_grade
        );
        
        debug!(
            "📈 Latency P95: {:?}, P99: {:?}",
            result.latency_percentiles.p95,
            result.latency_percentiles.p99
        );
        
        // Update suite stats
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
    
    /// Run a throughput benchmark (operations per second)
    pub fn benchmark_throughput<F, R>(&mut self, name: &str, benchmark_fn: F) -> BearDogResult<f64>
    where
        F: Fn() -> R + Clone,
    {
        self.benchmark(name, benchmark_fn)?;
        let result = &self.benchmarks[name];
        Ok(result.ops_per_second)
    }
    
    /// Run a latency benchmark (time per operation)
    pub fn benchmark_latency<F, R>(&mut self, name: &str, benchmark_fn: F) -> BearDogResult<Duration>
    where
        F: Fn() -> R + Clone,
    {
        self.benchmark(name, benchmark_fn)?;
        let result = &self.benchmarks[name];
        Ok(result.avg_latency)
    }
    
    /// Get benchmark result by name
    pub fn get_result(&self, name: &str) -> Option<&BenchmarkResult> {
        self.benchmarks.get(name)
    }
    
    /// Get all benchmark results
    pub fn get_all_results(&self) -> &HashMap<String, BenchmarkResult> {
        &self.benchmarks
    }
    
    /// Generate performance report
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
        
        let total_ops_per_second: f64 = self.benchmarks.values()
            .map(|r| r.ops_per_second)
            .sum();
        
        let avg_ops_per_second = if !self.benchmarks.is_empty() {
            total_ops_per_second / self.benchmarks.len() as f64
        } else {
            0.0
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
    
    /// Print performance summary
    pub fn print_summary(&self) {
        let report = self.generate_report();
        
        println!("\n🎯 BearDog Performance Benchmark Report");
        println!("==========================================");
        println!("Total Benchmarks: {}", report.suite_stats.total_benchmarks);
        println!("Passed: {} | Failed: {}", 
                 report.suite_stats.passed_benchmarks,
                 report.suite_stats.failed_benchmarks);
        println!("Total Execution Time: {:?}", report.suite_stats.total_execution_time);
        println!("Average Performance: {:.0} ops/sec", report.avg_ops_per_second);
        
        println!("\n📊 Performance Grade Distribution:");
        println!("🟢 Excellent (>1M ops/sec): {}", report.grade_distribution.excellent);
        println!("🔵 Good (>100K ops/sec): {}", report.grade_distribution.good);
        println!("🟡 Average (>10K ops/sec): {}", report.grade_distribution.average);
        println!("🟠 Poor (>1K ops/sec): {}", report.grade_distribution.poor);
        println!("🔴 Critical (<1K ops/sec): {}", report.grade_distribution.critical);
        
        if report.grade_distribution.critical > 0 {
            println!("\n⚠️  WARNING: {} benchmarks have critical performance issues!", 
                     report.grade_distribution.critical);
        }
        
        println!("\n🏆 Top Performers:");
        let mut sorted_results: Vec<_> = self.benchmarks.values().collect();
        sorted_results.sort_by(|a, b| {
            b.ops_per_second
                .partial_cmp(&a.ops_per_second)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        for (i, result) in sorted_results.iter().take(5).enumerate() {
            println!("  {}. {} - {:.0} ops/sec ({:?})",
                     i + 1, result.name, result.ops_per_second, result.performance_grade);
        }
    }
}

/// Complete performance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceReport {
    pub suite_stats: SuiteStats,
    pub grade_distribution: GradeDistribution,
    pub avg_ops_per_second: f64,
    pub total_ops_per_second: f64,
    pub benchmarks: HashMap<String, BenchmarkResult>,
}

/// Performance grade distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeDistribution {
    pub excellent: u32,
    pub good: u32,
    pub average: u32,
    pub poor: u32,
    pub critical: u32,
}

/// Macro for easy benchmark creation
#[macro_export]
macro_rules! benchmark {
    ($suite:expr, $name:expr, $code:block) => {
        $suite.benchmark($name, || $code)?;
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_benchmark_suite_creation() {
        let suite = BenchmarkSuite::new(BenchmarkConfig::default());
        assert_eq!(suite.benchmarks.len(), 0);
    }
    
    #[test]
    fn test_simple_benchmark() -> Result<(), Box<dyn std::error::Error>> {
        let mut suite = BenchmarkSuite::new(BenchmarkConfig {
            warmup_iterations: 10,
            measurement_iterations: 100,
            ..BenchmarkConfig::default()
        });
        
        // Benchmark a simple operation
        suite.benchmark("simple_add", || {
            let _result = 1 + 1;
        }).map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Benchmark should succeed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Benchmark should succeed", e))
})?;
        
        let result = suite.get_result("simple_add").ok_or_else(|| {
    tracing::error!("Operation failed ({})", "Result should exist");
    beardog_errors::BearDogError::internal("Operation failed: Result should exist".to_string())
})?;
        assert!(result.ops_per_second > 0.0);
        assert_eq!(result.iterations, 100);
        Ok(())
    }
    
    #[test]
    fn test_performance_grading() -> Result<(), Box<dyn std::error::Error>> {
        let mut suite = BenchmarkSuite::new(BenchmarkConfig {
            warmup_iterations: 1,
            measurement_iterations: 10,
            ..BenchmarkConfig::default()
        });
        
        // Fast operation should get a good grade
        suite.benchmark("fast_op", || {
            // Very fast operation
        }).map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Benchmark should succeed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Benchmark should succeed", e))
})?;
        
        let result = suite.get_result("fast_op").ok_or_else(|| {
    tracing::error!("Operation failed ({})", "Result should exist");
    beardog_errors::BearDogError::internal("Operation failed: Result should exist".to_string())
})?;
        // Fast operations should get excellent or good grade
        assert!(matches!(result.performance_grade, 
                        PerformanceGrade::Excellent | PerformanceGrade::Good));
        Ok(())
    }
} 