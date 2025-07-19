//! Performance Configuration for SIMD Cryptography
//!
//! This module handles performance optimization configurations for SIMD crypto operations.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Crypto performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoPerformanceConfig {
    /// Performance targets
    pub targets: CryptoPerformanceTargets,
    /// Benchmarking configuration
    pub benchmarking: CryptoBenchmarkingConfig,
    /// Optimization triggers
    pub optimization_triggers: OptimizationTriggers,
}

/// Crypto performance targets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoPerformanceTargets {
    /// Target throughput in operations per second
    pub target_throughput_ops_per_sec: u64,
    /// Target latency in microseconds
    pub target_latency_us: u64,
    /// Target memory usage in bytes
    pub target_memory_usage_bytes: u64,
}

/// Crypto benchmarking configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoBenchmarkingConfig {
    /// Enable benchmarking
    pub enabled: bool,
    /// Benchmark interval
    pub interval: Duration,
    /// Benchmark duration
    pub duration: Duration,
    /// Warmup iterations
    pub warmup_iterations: u32,
}

/// Optimization triggers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationTriggers {
    /// CPU usage threshold
    pub cpu_usage_threshold: f64,
    /// Memory usage threshold
    pub memory_usage_threshold: f64,
    /// Latency threshold
    pub latency_threshold_us: u64,
}

impl Default for CryptoPerformanceConfig {
    fn default() -> Self {
        Self {
            targets: CryptoPerformanceTargets::default(),
            benchmarking: CryptoBenchmarkingConfig::default(),
            optimization_triggers: OptimizationTriggers::default(),
        }
    }
}

impl Default for CryptoPerformanceTargets {
    fn default() -> Self {
        Self {
            target_throughput_ops_per_sec: 100_000,
            target_latency_us: 100,
            target_memory_usage_bytes: 1024 * 1024, // 1MB
        }
    }
}

impl Default for CryptoBenchmarkingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            interval: Duration::from_secs(300),
            duration: Duration::from_secs(10),
            warmup_iterations: 100,
        }
    }
}

impl Default for OptimizationTriggers {
    fn default() -> Self {
        Self {
            cpu_usage_threshold: 80.0,
            memory_usage_threshold: 85.0,
            latency_threshold_us: 1000,
        }
    }
}

impl CryptoPerformanceConfig {
    /// Create production crypto performance configuration
    pub fn production() -> Self {
        Self {
            targets: CryptoPerformanceTargets::production(),
            benchmarking: CryptoBenchmarkingConfig::production(),
            optimization_triggers: OptimizationTriggers::production(),
        }
    }

    /// Create development crypto performance configuration
    pub fn development() -> Self {
        Self {
            targets: CryptoPerformanceTargets::development(),
            benchmarking: CryptoBenchmarkingConfig::development(),
            optimization_triggers: OptimizationTriggers::development(),
        }
    }
}

impl CryptoPerformanceTargets {
    /// Create production crypto performance targets
    pub fn production() -> Self {
        Self {
            target_throughput_ops_per_sec: 1_000_000,
            target_latency_us: 50,
            target_memory_usage_bytes: 10 * 1024 * 1024, // 10MB
        }
    }

    /// Create development crypto performance targets
    pub fn development() -> Self {
        Self {
            target_throughput_ops_per_sec: 10_000,
            target_latency_us: 1000,
            target_memory_usage_bytes: 512 * 1024, // 512KB
        }
    }
}

impl CryptoBenchmarkingConfig {
    /// Create production crypto benchmarking configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(3600),
            duration: Duration::from_secs(60),
            warmup_iterations: 1000,
        }
    }

    /// Create development crypto benchmarking configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            interval: Duration::from_secs(60),
            duration: Duration::from_secs(5),
            warmup_iterations: 10,
        }
    }
}

impl OptimizationTriggers {
    /// Create production optimization triggers
    pub fn production() -> Self {
        Self {
            cpu_usage_threshold: 90.0,
            memory_usage_threshold: 90.0,
            latency_threshold_us: 100,
        }
    }

    /// Create development optimization triggers
    pub fn development() -> Self {
        Self {
            cpu_usage_threshold: 50.0,
            memory_usage_threshold: 50.0,
            latency_threshold_us: 5000,
        }
    }
}
