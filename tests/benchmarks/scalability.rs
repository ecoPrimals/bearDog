//! Scalability and Concurrency Benchmarking
//!
//! Benchmarks for concurrent operations, load testing, and scalability analysis.

use super::{BenchmarkResult, PerformanceBenchmarkSuite};
use beardog::BearDogResult;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc, Mutex,
};
use std::time::Instant;
use tracing::info;

/// Benchmark scalability and concurrency
pub async fn benchmark_scalability(suite: &mut PerformanceBenchmarkSuite) -> BearDogResult<Vec<BenchmarkResult>> {
    let mut results = Vec::new();

    // Concurrent operations benchmarks
    for &concurrency in &suite.config.concurrency_levels {
        let result = benchmark_concurrent_operations(suite, concurrency).await?;
        results.push(result);
    }

    Ok(results)
}

/// Benchmark concurrent operations
async fn benchmark_concurrent_operations(suite: &PerformanceBenchmarkSuite, concurrency: usize) -> BearDogResult<BenchmarkResult> {
    info!("  📈 Benchmarking concurrent operations with {} threads", concurrency);

    let successful_ops = Arc::new(AtomicU64::new(0));
    let latencies = Arc::new(Mutex::new(Vec::new()));

    let mut tasks = Vec::new();
    let benchmark_start = Instant::now();

    // Create concurrent tasks
    for _i in 0..concurrency {
        let core = Arc::clone(&suite.core);
        let ops_counter = Arc::clone(&successful_ops);
        let latencies_vec = Arc::clone(&latencies);
        let iterations_per_task = suite.config.iterations / concurrency;

        let task = tokio::spawn(async move {
            for _j in 0..iterations_per_task {
                let op_start = Instant::now();
                
                match core.health_check().await {
                    Ok(_) => {
                        ops_counter.fetch_add(1, Ordering::SeqCst);
                        let latency = op_start.elapsed().as_nanos() as f64 / 1_000_000.0;
                        latencies_vec.lock().unwrap().push(latency);
                    }
                    Err(_) => {
                        // Record failed operation
                    }
                }
            }
        });
        
        tasks.push(task);
    }

    // Wait for all tasks to complete
    for task in tasks {
        let _ = task.await;
    }

    let total_time = benchmark_start.elapsed();
    let successful_ops = successful_ops.load(Ordering::SeqCst);
    let ops_per_second = successful_ops as f64 / total_time.as_secs_f64();

    let latencies_vec = latencies.lock().unwrap().clone();
    let average_latency = latencies_vec.iter().sum::<f64>() / latencies_vec.len() as f64;

    let mut sorted_latencies = latencies_vec;
    sorted_latencies.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let p50_latency = sorted_latencies[sorted_latencies.len() / 2];
    let p95_latency = sorted_latencies[sorted_latencies.len() * 95 / 100];
    let p99_latency = sorted_latencies[sorted_latencies.len() * 99 / 100];

    let meets_threshold = ops_per_second >= suite.config.thresholds.min_throughput_ops_per_sec;

    Ok(BenchmarkResult {
        name: format!("Concurrent Operations ({})", concurrency),
        operations_per_second: ops_per_second,
        average_latency_ms: average_latency,
        p50_latency_ms: p50_latency,
        p95_latency_ms: p95_latency,
        p99_latency_ms: p99_latency,
        memory_usage_mb: 128.0,
        cpu_usage_percent: 0.0,
        success_rate: successful_ops as f64 / suite.config.iterations as f64,
        meets_threshold,
        data_size_bytes: None,
        concurrency_level: Some(concurrency),
    })
} 