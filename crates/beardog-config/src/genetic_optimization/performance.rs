//! Performance Optimization for Genetic Algorithms
//!
//! This module handles performance optimization configurations.

use serde::{Deserialize, Serialize};

/// Genetic performance optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticPerformanceOptimizationConfig {
    /// Memory optimization enabled
    pub memory_optimization: bool,
    /// CPU optimization enabled
    pub cpu_optimization: bool,
    /// I/O optimization enabled
    pub io_optimization: bool,
    /// Network optimization enabled
    pub network_optimization: bool,
    /// Profiling enabled
    pub profiling: bool,
    /// Benchmarking enabled
    pub benchmarking: bool,
}

impl Default for GeneticPerformanceOptimizationConfig {
    fn default() -> Self {
        Self {
            memory_optimization: true,
            cpu_optimization: true,
            io_optimization: false,
            network_optimization: false,
            profiling: false,
            benchmarking: false,
        }
    }
}

impl GeneticPerformanceOptimizationConfig {
    /// Create production performance optimization configuration
    pub fn production() -> Self {
        Self {
            memory_optimization: true,
            cpu_optimization: true,
            io_optimization: true,
            network_optimization: true,
            profiling: true,
            benchmarking: false,
        }
    }

    /// Create development performance optimization configuration
    pub fn development() -> Self {
        Self {
            memory_optimization: false,
            cpu_optimization: false,
            io_optimization: false,
            network_optimization: false,
            profiling: false,
            benchmarking: true,
        }
    }
}
