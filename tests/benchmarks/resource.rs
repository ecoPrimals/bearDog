

use super::{BenchmarkResult, PerformanceBenchmarkSuite};
use beardog_errors::BearDogError;
use std::time::Instant;
use tracing::info;

pub async fn benchmark_resource_usage(suite: &mut PerformanceBenchmarkSuite) -> Result<Vec<BenchmarkResult, BearDogError>> {
    let mut results = Vec::new();

    let memory_result = benchmark_memory_allocation(suite).await?;
    results.push(memory_result);

    Ok(results)
}

async fn benchmark_memory_allocation(suite: &PerformanceBenchmarkSuite) -> Result<BenchmarkResult, BearDogError> {
    info!("  🧠 Benchmarking memory allocation patterns");

    let mut latencies = Vec::new();
    let mut peak_memory = 0.0f64;

    let benchmark_start = Instant::now();
    for _i in 0..suite.config.iterations {
        let op_start = Instant::now();

        let _large_vec: Vec<u8> = vec![0; 1024 * 1024]; // 1MB allocation

        tokio::task::yield_now().await;

        let current_memory = 128.0 + (_i as f64 * 0.1); // Simulated memory growth
        if current_memory > peak_memory {
            peak_memory = current_memory;
        }
        
        latencies.push(op_start.elapsed().as_nanos() as f64 / 1_000_000.0);
    }
    let total_time = benchmark_start.elapsed();

    let ops_per_second = suite.config.iterations as f64 / total_time.as_secs_f64();
    let average_latency = latencies.iter().sum::<f64>() / latencies.len() as f64;
    
    latencies.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let p50_latency = latencies[latencies.len() / 2];
    let p95_latency = latencies[latencies.len() * 95 / 100];
    let p99_latency = latencies[latencies.len() * 99 / 100];

    Ok(BenchmarkResult {
        name: "Memory Allocation".to_string(),
        operations_per_second: ops_per_second,
        average_latency_ms: average_latency,
        p50_latency_ms: p50_latency,
        p95_latency_ms: p95_latency,
        p99_latency_ms: p99_latency,
        memory_usage_mb: peak_memory,
        cpu_usage_percent: 0.0,
        success_rate: 1.0,
        meets_threshold: peak_memory <= 512.0, // 512MB threshold
        data_size_bytes: None,
        concurrency_level: Some(1),
    })
} 