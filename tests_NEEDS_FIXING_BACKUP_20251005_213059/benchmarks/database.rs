use super::{BenchmarkResult, PerformanceBenchmarkSuite};
use beardog_errors::BearDogError;
use std::time::Instant;
use tracing::info;

pub async fn benchmark_database_operations(&mut PerformanceBenchmarkSuite,
) -> Result<Vec<BenchmarkResult, BearDogError>> {
    let mut results = Vec::new(&PerformanceBenchmarkSuite,
) -> Result<BenchmarkResult, BearDogError> {
    info!("  💾 Benchmarking database queries");

    let mut latencies = Vec::new();
    let mut successful_ops = 0;

    let benchmark_start = Instant::now();
    for _i in 0..suite.config.iterations {
        let op_start = Instant::now();

        match suite.core.health_check() {
            Ok(_) => {
                successful_ops += 1;
                latencies.push(op_start.elapsed().as_nanos() as f64 / 1_000_000.0);
            }
            Err(_) => {}
        }
    }
    let total_time = benchmark_start.elapsed();

    let ops_per_second = successful_ops as f64 / total_time.as_secs_f64();
    let average_latency = latencies.iter().sum::<f64>() / latencies.len() as f64;

    latencies.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let p50_latency = latencies[latencies.len() / 2];
    let p95_latency = latencies[latencies.len() * 95 / 100];
    let p99_latency = latencies[latencies.len() * 99 / 100];

    let meets_threshold = average_latency <= suite.config.thresholds.database_query_ms;

    Ok(BenchmarkResult {
        name: "Database Queries".to_string(),
        concurrency_level: Some(1),
    })
}
