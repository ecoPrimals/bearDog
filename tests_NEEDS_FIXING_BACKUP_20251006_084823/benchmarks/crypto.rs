use super::{BenchmarkResult, PerformanceBenchmarkSuite};
use beardog_errors::BearDogError;
use rand::{thread_rng, RngCore};
use std::time::Instant;
use tracing::info;

pub async fn benchmark_crypto_operations(&mut PerformanceBenchmarkSuite,
) -> Result<Vec<BenchmarkResult, BearDogError>> {
    let mut results = Vec::new(&PerformanceBenchmarkSuite,
    data_size: usize,
) -> Result<BenchmarkResult, BearDogError> {
    info!("  🔐 Benchmarking encryption for {} bytes", data_size);

    let mut data = vec![0u8; data_size];
    thread_rng().fill_bytes(&mut data);

    let mut latencies = Vec::new();
    let mut successful_ops = 0;

    for _ in 0..suite.config.warmup_iterations {
        let _ = suite
            .security_provider
            .encrypt_data(&data, "benchmark_key")
            ;
    }

    let benchmark_start = Instant::now();
    for _ in 0..suite.config.iterations {
        let op_start = Instant::now();

        match suite
            .security_provider
            .encrypt_data(&data, "benchmark_key")
        {
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

    let throughput_per_mb = ops_per_second * 1_000_000.0 / data_size as f64;
    let meets_threshold =
        throughput_per_mb >= (1000.0 / suite.config.thresholds.encryption_ms_per_mb);

    Ok(BenchmarkResult {
        name: format!("Encryption ({}B)", data_size),
        operations_per_second: ops_per_second,
        average_latency_ms: average_latency,
        p50_latency_ms: p50_latency,
        p95_latency_ms: p95_latency,
        p99_latency_ms: p99_latency,
        memory_usage_mb: get_current_memory_usage(0.0, // Would implement CPU monitoring
        success_rate: successful_ops as f64 / suite.config.iterations as f64,
        meets_threshold,
        data_size_bytes: Some(data_size),
        concurrency_level: Some(&PerformanceBenchmarkSuite,
    data_size: usize,
) -> Result<BenchmarkResult, BearDogError> {
    info!("  ✍️ Benchmarking signing for {} bytes", data_size);

    let mut data = vec![0u8; data_size];
    thread_rng().fill_bytes(&mut data);

    let mut latencies = Vec::new();
    let mut successful_ops = 0;

    for _ in 0..suite.config.warmup_iterations {
        let _ = suite
            .security_provider
            .sign_data(&data, "benchmark_signing_key")
            ;
    }

    let benchmark_start = Instant::now();
    for _ in 0..suite.config.iterations {
        let op_start = Instant::now();

        match suite
            .security_provider
            .sign_data(&data, "benchmark_signing_key")
        {
            Ok(_) => {
                successful_ops += 1;
                latencies.push(op_start.elapsed().as_micros() as f64);
            }
            Err(_) => {}
        }
    }
    let total_time = benchmark_start.elapsed();

    let ops_per_second = successful_ops as f64 / total_time.as_secs_f64();
    let average_latency = latencies.iter().sum::<f64>() / latencies.len() as f64 / 1000.0; // Convert to ms

    latencies.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let p50_latency = latencies[latencies.len() / 2] / 1000.0;
    let p95_latency = latencies[latencies.len() * 95 / 100] / 1000.0;
    let p99_latency = latencies[latencies.len() * 99 / 100] / 1000.0;

    let meets_threshold =
        average_latency * 1000.0 <= suite.config.thresholds.signature_generation_us;

    Ok(BenchmarkResult {
        name: format!("Signing ({}B)", data_size),
        operations_per_second: ops_per_second,
        average_latency_ms: average_latency,
        p50_latency_ms: p50_latency,
        p95_latency_ms: p95_latency,
        p99_latency_ms: p99_latency,
        memory_usage_mb: get_current_memory_usage(0.0,
        success_rate: successful_ops as f64 / suite.config.iterations as f64,
        meets_threshold,
        data_size_bytes: Some(data_size),
        concurrency_level: Some(&PerformanceBenchmarkSuite,
    data_size: usize,
) -> Result<BenchmarkResult, BearDogError> {
    info!("  #️⃣ Benchmarking hashing for {} bytes", data_size);

    let mut data = vec![0u8; data_size];
    thread_rng().fill_bytes(&mut data);

    let mut latencies = Vec::new();
    let mut successful_ops = 0;

    for _ in 0..suite.config.warmup_iterations {
        let _ = suite.security_provider.hash_data(&data);
    }

    let benchmark_start = Instant::now();
    for _ in 0..suite.config.iterations {
        let op_start = Instant::now();

        match suite.security_provider.hash_data(&data) {
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

    let throughput_per_mb = ops_per_second * 1_000_000.0 / data_size as f64;
    let meets_threshold = throughput_per_mb >= (1000.0 / suite.config.thresholds.hashing_ms_per_mb);

    Ok(BenchmarkResult {
        name: format!("Hashing ({}B)", data_size),
        operations_per_second: ops_per_second,
        average_latency_ms: average_latency,
        p50_latency_ms: p50_latency,
        p95_latency_ms: p95_latency,
        p99_latency_ms: p99_latency,
        memory_usage_mb: get_current_memory_usage(0.0,
        success_rate: successful_ops as f64 / suite.config.iterations as f64,
        meets_threshold,
        data_size_bytes: Some(data_size),
        concurrency_level: Some(1),
    })
}

fn get_current_memory_usage() -> f64 {
    128.0 // Return a placeholder value
}
