//! # Performance Benchmarking System - Modular Architecture
//!
//! This module provides **comprehensive performance benchmarking** for the unified
//! BearDog architecture, validating the performance improvements from consolidation
//! and zero-cost optimizations.
//!
//! ## 🎯 **Benchmarking Strategy**
//!
//! This system benchmarks:
//! - **Provider registry operations** vs scattered provider access
//! - **Zero-cost enum dispatch** vs Box<dyn> dynamic dispatch
//! - **Configuration loading** unified vs fragmented systems
//! - **Memory allocation patterns** before and after optimization
//!
//! ## Modular Architecture
//!
//! The benchmarking system is organized into focused modules:
//! - [`core`] - Core benchmark suite and configuration
//! - [`metrics`] - Performance metrics and statistics
//! - [`comparisons`] - Benchmark comparisons and analysis
//! - [`recommendations`] - Performance recommendations system

pub mod core;
pub mod metrics;
pub mod comparisons;
pub mod recommendations;

// Re-export main types for backward compatibility
pub use core::{
    PerformanceBenchmarkSuite, BenchmarkConfig, BenchmarkCategory,
};

pub use metrics::{
    BenchmarkResult, MemoryStats, BenchmarkSuiteResults,
    BenchmarkSummary,
};

pub use comparisons::{
    ComparisonResult, StatisticalSignificance,
};

pub use recommendations::{
    PerformanceRecommendation, RecommendationCategory,
    Priority, EffortLevel,
}; 