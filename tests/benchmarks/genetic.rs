

use super::{BenchmarkResult, PerformanceBenchmarkSuite};
use beardog::BearDogResult;
use std::time::Instant;
use tracing::info;

pub async fn benchmark_genetic_operations(suite: &mut PerformanceBenchmarkSuite) -> BearDogResult<Vec<BenchmarkResult>> {
    let mut results = Vec::new();

    let spawn_result = benchmark_genetic_spawning(suite).await?;
    results.push(spawn_result);

    let analysis_result = benchmark_genetic_analysis(suite).await?;
    results.push(analysis_result);

    Ok(results)
}

async fn benchmark_genetic_spawning(suite: &PerformanceBenchmarkSuite) -> BearDogResult<BenchmarkResult> {
    info!("  🧬 Benchmarking genetic spawning operations");

    let mut latencies = Vec::new();
    let mut successful_ops = 0;

    let benchmark_start = Instant::now();
    for i in 0..suite.config.iterations {
        let op_start = Instant::now();
        let node_id = format_args!("benchmark_spawn_{}", i).to_string();

        let spawn_genetics = beardog::genetics::BearDogGenetics::default();
        match suite.genetics_engine.spawn_with_genetics(&node_id, spawn_genetics).await {
            Ok(_) => {
                successful_ops += 1;
                latencies.push(op_start.elapsed().as_nanos() as f64 / 1_000_000.0);
            }
            Err(_) => {

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

    let meets_threshold = average_latency <= suite.config.thresholds.genetic_operation_ms;

    Ok(BenchmarkResult {
        name: "Genetic Spawning".to_string(),
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
        concurrency_level: Some(1),
    })
}

async fn benchmark_genetic_analysis(suite: &PerformanceBenchmarkSuite) -> BearDogResult<BenchmarkResult> {
    info!("  🧬 Benchmarking genetic analysis operations");

    let mut latencies = Vec::new();
    let mut successful_ops = 0;

    let benchmark_start = Instant::now();
    for i in 0..suite.config.iterations {
        let op_start = Instant::now();
        let node_id = format_args!("benchmark_node_{}", i).to_string();
        
        match suite.genetics_engine.analyze_genetic_fitness(&node_id).await {
            Ok(_) => {
                successful_ops += 1;
                latencies.push(op_start.elapsed().as_nanos() as f64 / 1_000_000.0);
            }
            Err(_) => {

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

    let meets_threshold = average_latency <= suite.config.thresholds.genetic_operation_ms;

    Ok(BenchmarkResult {
        name: "Genetic Analysis".to_string(),
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
        concurrency_level: Some(1),
    })
} 