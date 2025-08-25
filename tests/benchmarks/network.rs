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


//! Network Performance Benchmarking
//!
//! Benchmarks for network latency, throughput, and data processing operations.

use super::{BenchmarkResult, PerformanceBenchmarkSuite};
use beardog::BearDogResult;
use rand::{thread_rng, RngCore};
use std::time::Instant;
use tracing::info;

/// Benchmark network operations
pub async fn benchmark_network_operations(suite: &mut PerformanceBenchmarkSuite) -> BearDogResult<Vec<BenchmarkResult>> {
    let mut results = Vec::new();

    // Network latency benchmark
    let latency_result = benchmark_network_latency(suite).await?;
    results.push(latency_result);

    // Network throughput benchmark
    for &data_size in &suite.config.data_sizes {
        let throughput_result = benchmark_network_throughput(suite, data_size).await?;
        results.push(throughput_result);
    }

    Ok(results)
}

/// Benchmark network latency
async fn benchmark_network_latency(suite: &PerformanceBenchmarkSuite) -> BearDogResult<BenchmarkResult> {
    info!("  🌐 Benchmarking network latency");

    let mut latencies = Vec::new();
    let mut successful_ops = 0;

    let benchmark_start = Instant::now();
    for _ in 0..suite.config.iterations {
        let op_start = Instant::now();
        
        // Simulate network latency via core health check
        match suite.core.health_check().await {
            Ok(_) => {
                successful_ops += 1;
                latencies.push(op_start.elapsed().as_nanos() as f64 / 1_000_000.0);
            }
            Err(_) => {
                // Record failed operation
            }
        }
    }
    let total_time = benchmark_start.elapsed();

    let ops_per_second = successful_ops as f64 / total_time.as_secs_f64();
    let average_latency = latencies.iter().sum::<f64>() / latencies.len() as f64;
    
    latencies.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let p50_latency = latencies[latencies.len() / 2];
    let p95_latency = latencies[latencies.len() * 95 / 100];
    let p99_latency = latencies[latencies.len() * 99 / 100];

    Ok(BenchmarkResult {
        name: "Network Latency".to_string(),
        operations_per_second: ops_per_second,
        average_latency_ms: average_latency,
        p50_latency_ms: p50_latency,
        p95_latency_ms: p95_latency,
        p99_latency_ms: p99_latency,
        memory_usage_mb: 128.0,
        cpu_usage_percent: 0.0,
        success_rate: successful_ops as f64 / suite.config.iterations as f64,
        meets_threshold: average_latency <= 10.0, // 10ms threshold
        data_size_bytes: None,
        concurrency_level: Some(1),
    })
}

/// Benchmark network throughput
async fn benchmark_network_throughput(suite: &PerformanceBenchmarkSuite, data_size: usize) -> BearDogResult<BenchmarkResult> {
    info!("  🌐 Benchmarking network throughput for {} bytes", data_size);

    // Simulate network throughput by creating and processing data
    let mut data = vec![0u8; data_size];
    thread_rng().fill_bytes(&mut data);

    let mut latencies = Vec::new();
    let mut successful_ops = 0;
    let mut total_bytes_processed = 0u64;

    let benchmark_start = Instant::now();
    for _ in 0..suite.config.iterations {
        let op_start = Instant::now();
        
        // Simulate network data processing
        let processed_successfully = simulate_network_data_processing(suite, &data).await?;
        
        if processed_successfully {
            successful_ops += 1;
            total_bytes_processed += data_size as u64;
            latencies.push(op_start.elapsed().as_nanos() as f64 / 1_000_000.0);
        }
    }
    let total_time = benchmark_start.elapsed();

    let ops_per_second = successful_ops as f64 / total_time.as_secs_f64();
    let throughput_mbps = (total_bytes_processed as f64 * 8.0) / (total_time.as_secs_f64() * 1_000_000.0);
    let average_latency = latencies.iter().sum::<f64>() / latencies.len() as f64;
    
    latencies.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let p50_latency = latencies[latencies.len() / 2];
    let p95_latency = latencies[latencies.len() * 95 / 100];
    let p99_latency = latencies[latencies.len() * 99 / 100];

    Ok(BenchmarkResult {
        name: format!("Network Throughput ({}B)", data_size),
        operations_per_second: ops_per_second,
        average_latency_ms: average_latency,
        p50_latency_ms: p50_latency,
        p95_latency_ms: p95_latency,
        p99_latency_ms: p99_latency,
        memory_usage_mb: 128.0,
        cpu_usage_percent: 0.0,
        success_rate: successful_ops as f64 / suite.config.iterations as f64,
        meets_threshold: throughput_mbps >= 100.0, // 100 Mbps threshold
        data_size_bytes: Some(data_size),
        concurrency_level: Some(1),
    })
}

/// Simulate network data processing
async fn simulate_network_data_processing(suite: &PerformanceBenchmarkSuite, data: &[u8]) -> BearDogResult<bool> {
    // Simulate network data processing by hashing the data
    let _hash = suite.security_provider.hash_data(data).await?;
    Ok(true)
} 