//! # Performance Benchmarking System - Modular Architecture
//!
//! This module provides comprehensive performance benchmarking for the unified
//! BearDog architecture, split into focused, maintainable modules.
//!
//! ## 🎯 **Modular Structure**
//!
//! The benchmarking system is organized into logical modules:
//! - **`core`**: Core types, configuration, and infrastructure (~240 lines)
//! - **`provider`**: Provider registry and discovery benchmarks (~240 lines)
//! - **`capability`**: Capability dispatch and matching benchmarks (~235 lines)
//! - **`metrics`**: Metrics collection, analysis, and reporting (~240 lines)
//!
//! ## 🏗️ **Architecture Benefits**
//!
//! - **Maintainability**: Each module under 250 lines
//! - **Single Responsibility**: Clear separation of benchmarking concerns
//! - **Canonical Integration**: Uses unified types and constants
//! - **Zero-Cost Abstractions**: Validates performance improvements
//! - **Comprehensive Coverage**: All performance aspects covered
//!
//! ## 🚀 **Usage Example**
//!
//! ```rust
//! use beardog_adapters::universal::benchmarks::{
//!     PerformanceBenchmarkSuite, BenchmarkConfig
//! };
//!
//! let benchmark_suite = PerformanceBenchmarkSuite::new(BenchmarkConfig::default());
//! let results = benchmark_suite.run_all_benchmarks().await?;
//! println!("Performance improvement: {:.2}%", 
//!     results.summary.memory_efficiency * 100.0);
//! ```

// Core benchmarking infrastructure
pub mod core;

// Provider benchmarking functionality
pub mod provider;

// Capability dispatch benchmarking
pub mod capability;

// Metrics collection and reporting
pub mod metrics;

// Re-export core types for easy access
pub use core::{
    PerformanceBenchmarkSuite, BenchmarkConfig, BenchmarkResult, BenchmarkCategory,
    BenchmarkSuiteResults, BenchmarkSummary, PerformanceRecommendation,
    RecommendationCategory, Priority, EffortLevel, ComparisonResult,
    StatisticalSignificance, MemoryStats
};

// Re-export main functionality
pub use metrics::*; // For the run_all_benchmarks implementation

/// Create a new benchmark suite with default configuration
pub fn create_benchmark_suite() -> PerformanceBenchmarkSuite {
    PerformanceBenchmarkSuite::new(BenchmarkConfig::default())
}

/// Create a new benchmark suite with custom configuration
pub fn create_custom_benchmark_suite(config: BenchmarkConfig) -> PerformanceBenchmarkSuite {
    PerformanceBenchmarkSuite::new(config)
}

/// Quick benchmark configuration for development testing
pub fn development_benchmark_config() -> BenchmarkConfig {
    BenchmarkConfig {
        iterations: 1000,
        concurrency: 10,
        warmup_iterations: 100,
        enable_memory_profiling: false,
        enable_cpu_profiling: false,
        timeout: std::time::Duration::from_secs(10),
        percentiles: vec![50.0, 95.0, 99.0],
    }
}

/// Production benchmark configuration for comprehensive analysis
pub fn production_benchmark_config() -> BenchmarkConfig {
    BenchmarkConfig {
        iterations: 100000,
        concurrency: 1000,
        warmup_iterations: 10000,
        enable_memory_profiling: true,
        enable_cpu_profiling: true,
        timeout: std::time::Duration::from_secs(300),
        percentiles: vec![50.0, 90.0, 95.0, 99.0, 99.9, 99.99],
    }
} 