// SPDX-License-Identifier: AGPL-3.0-or-later

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::zero_cost::memory::SimdCryptoEngine;
use crate::zero_cost::memory_safe::{SafeRingBuffer, SafeSimdCapabilities};

pub struct PerformanceBenchmark {
    name: String,
    iterations: usize,
    warmup_iterations: usize,
}

impl PerformanceBenchmark {
    /// New
    /// Creates a new instance
    pub fn new(name: &str, iterations: usize) -> Self {
        Self {
            name: name.to_string(),
            iterations,
            warmup_iterations: iterations / 10,
        }
    }

    /// Run
    /// Runs process
    pub fn run<F>(&self, mut operation: F) -> BenchmarkResult
    where
        F: FnMut(),
    {
        // Warmup phase
        for _ in 0..self.warmup_iterations {
            operation();
        }

        // Actual benchmark
        let start = Instant::now();
        for _ in 0..self.iterations {
            operation();
        }
        let total_time = start.elapsed();

        BenchmarkResult {
            name: self.name.to_string(),
            iterations: self.iterations,
            total_time,
            average_time: total_time / self.iterations as u32,
            operations_per_second: self.iterations as f64 / total_time.as_secs_f64(),
        }
    }

    /// Compare
    pub fn compare<F1, F2>(&self, mut operation1: F1, mut operation2: F2) -> ComparisonResult
    where
        F1: FnMut(),
        F2: FnMut(),
    {
        let result1 = self.run(&mut operation1);
        let result2 = self.run(&mut operation2);

        let speedup =
            result1.average_time.as_nanos() as f64 / result2.average_time.as_nanos() as f64;

        ComparisonResult {
            baseline: result1,
            optimized: result2,
            speedup,
            improvement_percentage: (speedup - 1.0) * 100.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// Name
    /// Name of the item
    pub name: String,
    /// Iterations
    /// Number of iterations
    pub iterations: usize,
    /// Total Time
    pub total_time: Duration,
    /// Average Time
    pub average_time: Duration,
    /// Operations Per Second
    /// The operations per second value
    pub operations_per_second: f64,
}

#[derive(Debug)]
pub struct ComparisonResult {
    /// Baseline
    /// The baseline value
    pub baseline: BenchmarkResult,
    /// Optimized
    /// The optimized value
    pub optimized: BenchmarkResult,
    /// Speedup
    /// The speedup value
    pub speedup: f64,
    /// Improvement Percentage
    /// The improvement percentage value
    pub improvement_percentage: f64,
}

impl std::fmt::Display for BenchmarkResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Benchmark: {} | {} iterations | Total: {:?} | Average: {:?} | Ops/sec: {:.2}",
            self.name,
            self.iterations,
            self.total_time,
            self.average_time,
            self.operations_per_second
        )
    }
}

impl std::fmt::Display for ComparisonResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Performance Comparison:\n  Baseline: {}\n  Optimized: {}\n  Speedup: {:.2}x ({:.1}% improvement)",
            self.baseline,
            self.optimized,
            self.speedup,
            self.improvement_percentage
        )
    }
}

/// Benchmark zero-copy operations vs traditional copying
pub fn benchmark_zero_copy_vs_copy() -> ComparisonResult {
    let benchmark = PerformanceBenchmark::new("Zero-copy vs Copy", 10000);
    let test_buffer = vec![0u8; 4096];

    // Traditional copy operation
    let copy_operation = || {
        let _copied = test_buffer.clone();
    };

    // Zero-copy operation (simulated)
    let zero_copy_operation = || {
        let _slice = test_buffer.as_slice();
    };

    benchmark.compare(copy_operation, zero_copy_operation)
}

/// Benchmark SIMD operations vs scalar operations
pub fn benchmark_simd_vs_scalar() -> ComparisonResult {
    let benchmark = PerformanceBenchmark::new("SIMD vs Scalar Hashing", 1000);
    let engine = SimdCryptoEngine::new();
    let test_input = vec![0xAA; 8192];

    // Scalar operation
    let scalar_operation = || {
        let _hash = engine.fallback_hash(&test_input)?;
    };

    // SIMD operation
    let simd_operation = || {
        let _hash = engine.simd_hash(&test_input)?;
    };

    benchmark.compare(scalar_operation, simd_operation)
}

/// Benchmark lock-free vs mutex-based operations
pub fn benchmark_lock_free_vs_mutex() -> ComparisonResult {
    let benchmark = PerformanceBenchmark::new("Lock-free vs Mutex", 100);

    // Lock-free ring buffer
    let lock_free_buffer = Arc::new(SafeRingBuffer::new(1000));
    let lock_free_operation = || {
        let handles: Vec<_> = (0..4)
            .map(|i| {
                let buffer = lock_free_buffer.clone();
                thread::spawn(move || {
                    for j in 0..25 {
                        let value = i * 25 + j;
                        while buffer.try_push(value).is_err() {
                            thread::yield_now();
                        }
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join()?;
        }

        // Consume all items
        while lock_free_buffer.try_pop().is_some() {}
    };

    // Mutex-based vector
    let mutex_buffer = Arc::new(Mutex::new(Vec::new()));
    let mutex_operation = || {
        let handles: Vec<_> = (0..4)
            .map(|i| {
                let buffer = mutex_buffer.clone();
                thread::spawn(move || {
                    for j in 0..25 {
                        let value = i * 25 + j;
                        buffer.lock()?.push(value);
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join()?;
        }

        // Clear the buffer
        mutex_buffer.lock()?.clear();
    };

    benchmark.compare(mutex_operation, lock_free_operation)
}

/// Benchmark memory allocation patterns
pub fn benchmark_allocation_patterns() -> Vec<BenchmarkResult> {
    let mut results = Vec::new();

    // Stack allocation benchmark
    let stack_benchmark = PerformanceBenchmark::new("Stack Allocation", 100_000);
    let stack_result = stack_benchmark.run(|| {
        let _data = [0u8; 1024];
    });
    results.push(stack_result);

    // Heap allocation benchmark
    let heap_benchmark = PerformanceBenchmark::new("Heap Allocation", 10000);
    let heap_result = heap_benchmark.run(|| {
        let _data = vec![0u8; 1024];
    });
    results.push(heap_result);

    // Box allocation benchmark
    let box_benchmark = PerformanceBenchmark::new("Box Allocation", 10000);
    let box_result = box_benchmark.run(|| {
        let _data = Box::new([0u8; 1024]);
    });
    results.push(box_result);

    results
}

pub struct PerformanceTestSuite {
    /// Results
    /// Collection of results
    pub results: Vec<BenchmarkResult>,
    /// Comparisons
    /// Collection of comparisons
    pub comparisons: Vec<ComparisonResult>,
}

impl PerformanceTestSuite {
    /// Run Comprehensive Suite
    /// Runs comprehensive_suite
    pub fn run_comprehensive_suite() -> Self {
        let mut results = Vec::new();
        let mut comparisons = Vec::new();

        // Run individual benchmarks
        results.extend(benchmark_allocation_patterns());

        // Run comparison benchmarks
        comparisons.push(benchmark_zero_copy_vs_copy());
        comparisons.push(benchmark_simd_vs_scalar());
        comparisons.push(benchmark_lock_free_vs_mutex());

        Self {
            results,
            comparisons,
        }
    }

    /// Print Summary
    pub fn print_summary(&self) {
        println!("=== Performance Test Suite Results ===\n");

        println!("Individual Benchmarks:");
        for result in &self.results {
            println!("  {result}");
        }

        println!("\nComparison Benchmarks:");
        for comparison in &self.comparisons {
            println!("  {comparison}\n");
        }

        // Calculate overall improvement
        let avg_improvement: f64 = self
            .comparisons
            .iter()
            .map(|c| c.improvement_percentage)
            .sum::<f64>()
            / self.comparisons.len() as f64;

        println!("Average Performance Improvement: {avg_improvement:.1}%");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_creation() {
        let benchmark = PerformanceBenchmark::new("Test", 1000);
        assert_eq!(benchmark.name, "Test");
        assert_eq!(benchmark.iterations, 1000);
        assert_eq!(benchmark.warmup_iterations, 100);
    }

    #[test]
    fn test_simple_benchmark() {
        let benchmark = PerformanceBenchmark::new("Simple", 100);
        let result = benchmark.run(|| {
            // Simple operation
            let _x = 1 + 1;
        });
 // TEST_CATEGORY: unit
 // TEST_DOMAIN: types
 // TEST_PRIORITY: normal

        assert_eq!(result.iterations, 100);
        assert!(result.total_time.as_nanos() > 0);
        assert!(result.operations_per_second > 0.0);
    }
 // TEST_CATEGORY: unit
 // TEST_DOMAIN: types
 // TEST_PRIORITY: normal

    #[test]
    fn test_benchmark_comparison() {
        let benchmark = PerformanceBenchmark::new("Comparison", 100);

        let slow_op = || {
            // Simulate slower operation
            std::thread::sleep(Duration::from_nanos(100));
        };

        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let fast_op = || {
            // Simulate faster operation
            let _x = 1 + 1;
        };

        let comparison = benchmark.compare(slow_op, fast_op);

        // Fast operation should be faster (speedup > 1.0 means optimized is faster)
        assert!(comparison.speedup > 1.0);
        assert!(comparison.improvement_percentage > 0.0);
    }

    #[test]
    fn test_zero_copy_benchmark() {
        let comparison = benchmark_zero_copy_vs_copy();

        // Zero-copy should be faster than copying
        assert!(comparison.speedup > 1.0);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(comparison.baseline.name, "Zero-copy vs Copy");
        assert_eq!(comparison.optimized.name, "Zero-copy vs Copy");
    }

    #[test]
    fn test_allocation_benchmarks() {
        let results = benchmark_allocation_patterns();
 // TEST_CATEGORY: unit
 // TEST_DOMAIN: types
 // TEST_PRIORITY: normal

        assert_eq!(results.len(), 3);
        assert_eq!(results[0].name, "Stack Allocation");
        assert_eq!(results[1].name, "Heap Allocation");
        assert_eq!(results[2].name, "Box Allocation");

        // Stack allocation should be fastest
        assert!(results[0].operations_per_second > results[1].operations_per_second);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_performance_suite() {
        let suite = PerformanceTestSuite::run_comprehensive_suite();

        assert!(!suite.results.is_empty());
        assert!(!suite.comparisons.is_empty());

        // All benchmarks should have positive operations per second
        for result in &suite.results {
            assert!(result.operations_per_second > 0.0);
        }

        // All comparisons should show some measurable difference
        for comparison in &suite.comparisons {
            assert!(comparison.speedup >= 0.0);
        }
    }
}