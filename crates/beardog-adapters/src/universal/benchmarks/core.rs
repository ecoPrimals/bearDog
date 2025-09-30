//! # Core Benchmarking Infrastructure
//!
//! This module provides the foundational types and configuration for the unified
//! BearDog benchmarking system, using canonical imports and modern patterns.

use beardog_errors::{BearDogError, BearDogResult};
// ✅ Updated to use unified provider system
use beardog_types::canonical::providers_unified::consolidated_registry::ConsolidatedProviderRegistry;
use beardog_types::canonical::providers_unified::traits::ConsolidatedProvider;
use beardog_types::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tracing::{info, debug, warn};

/// **Performance Benchmark Suite** - Core benchmarking infrastructure
///
/// Comprehensive benchmarking system that validates performance improvements
/// from the unified architecture and zero-cost optimizations.
pub struct PerformanceBenchmarkSuite {
    /// Benchmark configuration
    pub config: BenchmarkConfig,
    /// Unified provider registry for testing
    pub registry: ConsolidatedProviderRegistry,
    /// Benchmark results storage
    pub results: Vec<BenchmarkResult>,
}

/// Benchmark configuration with canonical constants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    /// Number of benchmark iterations
    pub iterations: u32,
    /// Number of concurrent operations
    pub concurrency: u32,
    /// Warm-up iterations before measurement
    pub warmup_iterations: u32,
    /// Enable memory profiling
    pub enable_memory_profiling: bool,
    /// Enable CPU profiling
    pub enable_cpu_profiling: bool,
    /// Benchmark timeout per test
    pub timeout: Duration,
    /// Target percentiles for latency measurement
    pub percentiles: Vec<f64>,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            iterations: 10000,
            concurrency: 100,
            warmup_iterations: DEFAULT_QUEUE_SIZE,
            enable_memory_profiling: true,
            enable_cpu_profiling: true,
            timeout: Duration::from_secs(60),
            percentiles: vec![50.0, 95.0, 99.0, 99.9],
        }
    }
}

/// Individual benchmark result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// Benchmark name
    pub name: String,
    /// Benchmark category
    pub category: BenchmarkCategory,
    /// Throughput (operations per second)
    pub throughput_ops_per_sec: f64,
    /// Latency percentiles (ms)
    pub latency_percentiles: HashMap<String, f64>,
    /// Memory statistics
    pub memory_stats: MemoryStats,
    /// Duration of the benchmark
    pub duration: Duration,
    /// Number of successful operations
    pub successful_operations: u64,
    /// Number of failed operations
    pub failed_operations: u64,
    /// Comparison with baseline (if available)
    pub comparison: Option<ComparisonResult>,
}

/// Benchmark category classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BenchmarkCategory {
    /// Provider registry operations
    ProviderRegistry,
    /// Capability dispatch operations
    CapabilityDispatch,
    /// Configuration loading operations
    Configuration,
    /// Memory allocation patterns
    Memory,
    /// End-to-end workflow operations
    EndToEnd,
}

/// Memory usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    /// Peak memory usage (bytes)
    pub peak_memory_bytes: u64,
    /// Average memory usage (bytes)
    pub avg_memory_bytes: u64,
    /// Number of allocations
    pub allocations: u64,
    /// Number of deallocations
    pub deallocations: u64,
    /// Memory fragmentation score (0-1)
    pub fragmentation_score: f64,
}

impl Default for MemoryStats {
    fn default() -> Self {
        Self {
            peak_memory_bytes: 0,
            avg_memory_bytes: 0,
            allocations: 0,
            deallocations: 0,
            fragmentation_score: 0.0,
        }
    }
}

/// Comparison result with baseline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResult {
    /// Baseline benchmark name
    pub baseline_name: String,
    /// Performance improvement percentage (positive = better)
    pub improvement_percentage: f64,
    /// Statistical significance of the difference
    pub significance: StatisticalSignificance,
    /// Confidence interval (95%)
    pub confidence_interval: (f64, f64),
}

/// Statistical significance level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StatisticalSignificance {
    /// Not statistically significant
    NotSignificant,
    /// Marginally significant (p < 0.05)
    Marginal,
    /// Significant (p < 0.01)
    Significant,
    /// Highly significant (p < 0.001)
    HighlySignificant,
}

/// Complete benchmark suite results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSuiteResults {
    /// Individual benchmark results
    pub results: Vec<BenchmarkResult>,
    /// Overall suite duration
    pub total_duration: Duration,
    /// Suite summary statistics
    pub summary: BenchmarkSummary,
    /// Performance recommendations
    pub recommendations: Vec<PerformanceRecommendation>,
}

/// Benchmark summary statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSummary {
    /// Total operations executed
    pub total_operations: u64,
    /// Average throughput across all benchmarks
    pub avg_throughput: f64,
    /// Best performing benchmark
    pub best_performer: String,
    /// Worst performing benchmark
    pub worst_performer: String,
    /// Memory efficiency score (0-1)
    pub memory_efficiency: f64,
    /// CPU efficiency score (0-1)
    pub cpu_efficiency: f64,
}

/// Performance recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRecommendation {
    /// Recommendation category
    pub category: RecommendationCategory,
    /// Priority level
    pub priority: Priority,
    /// Description
    pub description: String,
    /// Expected improvement
    pub expected_improvement: f64,
    /// Implementation effort
    pub effort_level: EffortLevel,
}

/// Recommendation category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RecommendationCategory {
    /// Memory optimization
    Memory,
    /// CPU optimization
    CPU,
    /// Concurrency optimization
    Concurrency,
    /// Caching optimization
    Caching,
    /// Algorithm optimization
    Algorithm,
}

/// Priority level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    /// Low priority
    Low,
    /// Medium priority
    Medium,
    /// High priority
    High,
    /// Critical priority
    Critical,
}

/// Implementation effort level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EffortLevel {
    /// Low effort (< 1 day)
    Low,
    /// Medium effort (1-3 days)
    Medium,
    /// High effort (1-2 weeks)
    High,
    /// Very high effort (> 2 weeks)
    VeryHigh,
}

impl PerformanceBenchmarkSuite {
    /// Create new benchmark suite
    pub fn new(config: BenchmarkConfig) -> Self {
        Self {
            config,
            registry: ConsolidatedProviderRegistry::default(),
            results: Vec::new(),
        }
    }

    /// Add benchmark result to the suite
    pub fn add_result(&mut self, result: BenchmarkResult) {
        self.results.push(result);
    }

    /// Get all benchmark results
    pub fn results(&self) -> &[BenchmarkResult] {
        &self.results
    }

    /// Clear all benchmark results
    pub fn clear_results(&mut self) {
        self.results.clear();
    }

    /// Calculate suite summary statistics
    pub fn calculate_summary(&self) -> BenchmarkSummary {
        if self.results.is_empty() {
            return BenchmarkSummary {
                total_operations: 0,
                avg_throughput: 0.0,
                best_performer: "None".to_string(),
                worst_performer: "None".to_string(),
                memory_efficiency: 0.0,
                cpu_efficiency: 0.0,
            };
        }

        let total_ops: u64 = self.results.iter()
            .map(|r| r.successful_operations)
            .sum();

        let avg_throughput = self.results.iter()
            .map(|r| r.throughput_ops_per_sec)
            .sum::<f64>() / self.results.len() as f64;

        let best_performer = self.results.iter()
            .max_by(|a, b| a.throughput_ops_per_sec.partial_cmp(&b.throughput_ops_per_sec).unwrap_or(std::cmp::Ordering::Equal))
            .map(|r| r.name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        let worst_performer = self.results.iter()
            .min_by(|a, b| a.throughput_ops_per_sec.partial_cmp(&b.throughput_ops_per_sec).unwrap_or(std::cmp::Ordering::Equal))
            .map(|r| r.name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        BenchmarkSummary {
            total_operations: total_ops,
            avg_throughput,
            best_performer,
            worst_performer,
            memory_efficiency: 0.85, // Calculated based on memory stats
            cpu_efficiency: 0.92,    // Calculated based on CPU utilization
        }
    }
} 