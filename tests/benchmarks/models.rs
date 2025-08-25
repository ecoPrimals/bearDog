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


//! Benchmark Models and Data Structures
//!
//! All data structures, configuration types, and result models
//! for the performance benchmark suite.

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{
        atomic::AtomicU64,
        Arc,
    },
};
use tokio::sync::RwLock;

/// Benchmark configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    /// Number of iterations for each benchmark
    pub iterations: usize,
    /// Warmup iterations before measurement
    pub warmup_iterations: usize,
    /// Maximum benchmark duration per test
    pub max_duration_seconds: u64,
    /// Concurrency levels to test
    pub concurrency_levels: Vec<usize>,
    /// Data sizes to test (in bytes)
    pub data_sizes: Vec<usize>,
    /// Enable memory profiling
    pub memory_profiling: bool,
    /// Enable CPU profiling
    pub cpu_profiling: bool,
    /// Target performance thresholds
    pub thresholds: PerformanceThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Maximum encryption time per MB (milliseconds)
    pub encryption_ms_per_mb: f64,
    /// Maximum signature generation time (microseconds)
    pub signature_generation_us: f64,
    /// Maximum signature verification time (microseconds)  
    pub signature_verification_us: f64,
    /// Maximum hash calculation time per MB (milliseconds)
    pub hashing_ms_per_mb: f64,
    /// Maximum genetic operation time (milliseconds)
    pub genetic_operation_ms: f64,
    /// Maximum workflow processing time (milliseconds)
    pub workflow_processing_ms: f64,
    /// Maximum database query time (milliseconds)
    pub database_query_ms: f64,
    /// Maximum API response time (milliseconds)
    pub api_response_ms: f64,
    /// Minimum throughput (operations per second)
    pub min_throughput_ops_per_sec: f64,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            iterations: 1000,
            warmup_iterations: 100,
            max_duration_seconds: 300, // 5 minutes
            concurrency_levels: vec![1, 2, 4, 8, 16, 32],
            data_sizes: vec![1024, 4096, 16384, 65536, 262144, 1048576], // 1KB to 1MB
            memory_profiling: true,
            cpu_profiling: true,
            thresholds: PerformanceThresholds {
                encryption_ms_per_mb: 50.0,
                signature_generation_us: 500.0,
                signature_verification_us: 200.0,
                hashing_ms_per_mb: 10.0,
                genetic_operation_ms: 100.0,
                workflow_processing_ms: 500.0,
                database_query_ms: 50.0,
                api_response_ms: 100.0,
                min_throughput_ops_per_sec: 1000.0,
            },
        }
    }
}

/// Performance metrics collector
pub struct PerformanceMetricsCollector {
    metrics: Arc<RwLock<PerformanceMetrics>>,
    operation_counters: HashMap<String, Arc<AtomicU64>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    // Crypto performance metrics
    pub crypto_encryption_ops_per_sec: f64,
    pub crypto_decryption_ops_per_sec: f64,
    pub crypto_signing_ops_per_sec: f64,
    pub crypto_verification_ops_per_sec: f64,
    pub crypto_hashing_ops_per_sec: f64,
    
    // Network performance metrics
    pub network_throughput_mbps: f64,
    pub network_latency_ms: f64,
    pub network_packet_loss: f64,
    
    // Genetic operations metrics
    pub genetic_spawn_ops_per_sec: f64,
    pub genetic_analysis_ops_per_sec: f64,
    pub genetic_mutation_ops_per_sec: f64,
    
    // Workflow performance metrics
    pub workflow_initiation_ops_per_sec: f64,
    pub workflow_approval_ops_per_sec: f64,
    pub workflow_completion_ops_per_sec: f64,
    
    // Database performance metrics
    pub database_read_ops_per_sec: f64,
    pub database_write_ops_per_sec: f64,
    pub database_query_latency_ms: f64,
    
    // System resource metrics
    pub peak_memory_usage_mb: f64,
    pub average_cpu_usage_percent: f64,
    pub gc_pressure_score: f64,
    
    // Overall system metrics
    pub overall_throughput_ops_per_sec: f64,
    pub system_scalability_score: f64,
    pub efficiency_score: f64,
}

/// Individual benchmark result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub name: String,
    pub operations_per_second: f64,
    pub average_latency_ms: f64,
    pub p50_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
    pub success_rate: f64,
    pub meets_threshold: bool,
    pub data_size_bytes: Option<usize>,
    pub concurrency_level: Option<usize>,
}

/// Benchmark suite report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkReport {
    pub suite_name: String,
    pub execution_time_seconds: f64,
    pub total_operations: u64,
    pub benchmark_results: Vec<BenchmarkResult>,
    pub performance_metrics: PerformanceMetrics,
    pub system_info: SystemInfo,
    pub performance_grade: String,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub cpu_model: String,
    pub cpu_cores: u32,
    pub memory_total_mb: u64,
    pub disk_type: String,
}

impl PerformanceMetricsCollector {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(PerformanceMetrics::default())),
            operation_counters: HashMap::new(),
        }
    }

    pub async fn calculate_final_metrics(&self, results: &[BenchmarkResult]) -> PerformanceMetrics {
        let mut metrics = PerformanceMetrics::default();

        // Calculate crypto metrics
        let crypto_results: Vec<_> = results.iter().filter(|r| 
            r.name.contains("Encryption") || r.name.contains("Signing") || r.name.contains("Hashing")
        ).collect();
        
        if !crypto_results.is_empty() {
            metrics.crypto_encryption_ops_per_sec = crypto_results.iter()
                .filter(|r| r.name.contains("Encryption"))
                .map(|r| r.operations_per_second)
                .fold(0.0, f64::max);
            
            metrics.crypto_signing_ops_per_sec = crypto_results.iter()
                .filter(|r| r.name.contains("Signing"))
                .map(|r| r.operations_per_second)
                .fold(0.0, f64::max);
            
            metrics.crypto_hashing_ops_per_sec = crypto_results.iter()
                .filter(|r| r.name.contains("Hashing"))
                .map(|r| r.operations_per_second)
                .fold(0.0, f64::max);
        }

        // Calculate overall throughput
        metrics.overall_throughput_ops_per_sec = results.iter()
            .map(|r| r.operations_per_second)
            .fold(0.0, f64::max);

        // Calculate system metrics
        metrics.peak_memory_usage_mb = results.iter()
            .map(|r| r.memory_usage_mb)
            .fold(0.0, f64::max);

        // Calculate scalability score
        let concurrent_results: Vec<_> = results.iter()
            .filter(|r| r.name.contains("Concurrent"))
            .collect();
        
        if concurrent_results.len() > 1 {
            let min_ops = concurrent_results.iter().map(|r| r.operations_per_second).fold(f64::INFINITY, f64::min);
            let max_ops = concurrent_results.iter().map(|r| r.operations_per_second).fold(0.0, f64::max);
            metrics.system_scalability_score = (max_ops / min_ops).min(10.0) * 10.0; // Scale 0-100
        }

        // Calculate efficiency score
        let pass_rate = results.iter().filter(|r| r.meets_threshold).count() as f64 / results.len() as f64;
        metrics.efficiency_score = pass_rate * 100.0;

        metrics
    }
} 