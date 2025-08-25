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


//! Cryptographic Operations Benchmarking
//!
//! Benchmarks for encryption, signing, hashing, and other cryptographic operations.

use super::{BenchmarkResult, PerformanceBenchmarkSuite};
use beardog::BearDogResult;
use rand::{thread_rng, RngCore};
use std::time::Instant;
use tracing::info;

/// Benchmark cryptographic operations
pub async fn benchmark_crypto_operations(suite: &mut PerformanceBenchmarkSuite) -> BearDogResult<Vec<BenchmarkResult>> {
    let mut results = Vec::new();

    // Encryption benchmarks
    for &data_size in &suite.config.data_sizes {
        let result = benchmark_encryption(suite, data_size).await?;
        results.push(result);
    }

    // Signing benchmarks
    for &data_size in &suite.config.data_sizes {
        let result = benchmark_signing(suite, data_size).await?;
        results.push(result);
    }

    // Hashing benchmarks
    for &data_size in &suite.config.data_sizes {
        let result = benchmark_hashing(suite, data_size).await?;
        results.push(result);
    }

    Ok(results)
}

/// Benchmark encryption operations
async fn benchmark_encryption(suite: &PerformanceBenchmarkSuite, data_size: usize) -> BearDogResult<BenchmarkResult> {
    info!("  🔐 Benchmarking encryption for {} bytes", data_size);

    let mut data = vec![0u8; data_size];
    thread_rng().fill_bytes(&mut data);

    let mut latencies = Vec::new();
    let mut successful_ops = 0;

    // Warmup
    for _ in 0..suite.config.warmup_iterations {
        let _ = suite.security_provider.encrypt_data(&data, "benchmark_key").await;
    }

    // Actual benchmark
    let benchmark_start = Instant::now();
    for _ in 0..suite.config.iterations {
        let op_start = Instant::now();
        
        match suite.security_provider.encrypt_data(&data, "benchmark_key").await {
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

    let throughput_per_mb = ops_per_second * 1_000_000.0 / data_size as f64;
    let meets_threshold = throughput_per_mb >= (1000.0 / suite.config.thresholds.encryption_ms_per_mb);

    Ok(BenchmarkResult {
        name: format!("Encryption ({}B)", data_size),
        operations_per_second: ops_per_second,
        average_latency_ms: average_latency,
        p50_latency_ms: p50_latency,
        p95_latency_ms: p95_latency,
        p99_latency_ms: p99_latency,
        memory_usage_mb: get_current_memory_usage(),
        cpu_usage_percent: 0.0, // Would implement CPU monitoring
        success_rate: successful_ops as f64 / suite.config.iterations as f64,
        meets_threshold,
        data_size_bytes: Some(data_size),
        concurrency_level: Some(1),
    })
}

/// Benchmark signing operations
async fn benchmark_signing(suite: &PerformanceBenchmarkSuite, data_size: usize) -> BearDogResult<BenchmarkResult> {
    info!("  ✍️ Benchmarking signing for {} bytes", data_size);

    let mut data = vec![0u8; data_size];
    thread_rng().fill_bytes(&mut data);

    let mut latencies = Vec::new();
    let mut successful_ops = 0;

    // Warmup
    for _ in 0..suite.config.warmup_iterations {
        let _ = suite.security_provider.sign_data(&data, "benchmark_signing_key").await;
    }

    // Actual benchmark
    let benchmark_start = Instant::now();
    for _ in 0..suite.config.iterations {
        let op_start = Instant::now();
        
        match suite.security_provider.sign_data(&data, "benchmark_signing_key").await {
            Ok(_) => {
                successful_ops += 1;
                latencies.push(op_start.elapsed().as_micros() as f64);
            }
            Err(_) => {
                // Record failed operation
            }
        }
    }
    let total_time = benchmark_start.elapsed();

    let ops_per_second = successful_ops as f64 / total_time.as_secs_f64();
    let average_latency = latencies.iter().sum::<f64>() / latencies.len() as f64 / 1000.0; // Convert to ms
    
    latencies.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let p50_latency = latencies[latencies.len() / 2] / 1000.0;
    let p95_latency = latencies[latencies.len() * 95 / 100] / 1000.0;
    let p99_latency = latencies[latencies.len() * 99 / 100] / 1000.0;

    let meets_threshold = average_latency * 1000.0 <= suite.config.thresholds.signature_generation_us;

    Ok(BenchmarkResult {
        name: format!("Signing ({}B)", data_size),
        operations_per_second: ops_per_second,
        average_latency_ms: average_latency,
        p50_latency_ms: p50_latency,
        p95_latency_ms: p95_latency,
        p99_latency_ms: p99_latency,
        memory_usage_mb: get_current_memory_usage(),
        cpu_usage_percent: 0.0,
        success_rate: successful_ops as f64 / suite.config.iterations as f64,
        meets_threshold,
        data_size_bytes: Some(data_size),
        concurrency_level: Some(1),
    })
}

/// Benchmark hashing operations
async fn benchmark_hashing(suite: &PerformanceBenchmarkSuite, data_size: usize) -> BearDogResult<BenchmarkResult> {
    info!("  #️⃣ Benchmarking hashing for {} bytes", data_size);

    let mut data = vec![0u8; data_size];
    thread_rng().fill_bytes(&mut data);

    let mut latencies = Vec::new();
    let mut successful_ops = 0;

    // Warmup
    for _ in 0..suite.config.warmup_iterations {
        let _ = suite.security_provider.hash_data(&data).await;
    }

    // Actual benchmark
    let benchmark_start = Instant::now();
    for _ in 0..suite.config.iterations {
        let op_start = Instant::now();
        
        match suite.security_provider.hash_data(&data).await {
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

    let throughput_per_mb = ops_per_second * 1_000_000.0 / data_size as f64;
    let meets_threshold = throughput_per_mb >= (1000.0 / suite.config.thresholds.hashing_ms_per_mb);

    Ok(BenchmarkResult {
        name: format!("Hashing ({}B)", data_size),
        operations_per_second: ops_per_second,
        average_latency_ms: average_latency,
        p50_latency_ms: p50_latency,
        p95_latency_ms: p95_latency,
        p99_latency_ms: p99_latency,
        memory_usage_mb: get_current_memory_usage(),
        cpu_usage_percent: 0.0,
        success_rate: successful_ops as f64 / suite.config.iterations as f64,
        meets_threshold,
        data_size_bytes: Some(data_size),
        concurrency_level: Some(1),
    })
}

/// Get current memory usage in MB (simplified implementation)
fn get_current_memory_usage() -> f64 {
    // This is a simplified implementation - in a real scenario you'd use 
    // system monitoring tools or memory profilers
    128.0 // Return a placeholder value
} 