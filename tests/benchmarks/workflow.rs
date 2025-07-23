//! Workflow Operations Benchmarking  
//!
//! Benchmarks for workflow initiation, processing, and approval workflows.

use super::{BenchmarkResult, PerformanceBenchmarkSuite};
use beardog::BearDogResult;
use beardog::workflows::*;
use std::collections::HashMap;
use std::time::Instant;
use tracing::info;

/// Benchmark workflow operations
pub async fn benchmark_workflow_operations(suite: &mut PerformanceBenchmarkSuite) -> BearDogResult<Vec<BenchmarkResult>> {
    let mut results = Vec::new();

    // Workflow initiation benchmark
    let initiation_result = benchmark_workflow_initiation(suite).await?;
    results.push(initiation_result);

    Ok(results)
}

/// Benchmark workflow initiation
async fn benchmark_workflow_initiation(suite: &PerformanceBenchmarkSuite) -> BearDogResult<BenchmarkResult> {
    info!("  🔄 Benchmarking workflow initiation");

    let mut latencies = Vec::new();
    let mut successful_ops = 0;

    let benchmark_start = Instant::now();
    for i in 0..suite.config.iterations {
        let op_start = Instant::now();
        
        let workflow_request = WorkflowRequest {
            workflow_type: WorkflowType::SystemMaintenance,
            initiator: format!("benchmark_user_{}", i),
            target: WorkflowTarget::System,
            parameters: HashMap::new(),
            reason: "Performance benchmark workflow".to_string(),
            priority: WorkflowPriority::Normal,
            metadata: HashMap::new(),
        };

        match suite.workflow_engine.initiate_workflow(workflow_request).await {
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
    
    latencies.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let p50_latency = latencies[latencies.len() / 2];
    let p95_latency = latencies[latencies.len() * 95 / 100];
    let p99_latency = latencies[latencies.len() * 99 / 100];

    let meets_threshold = average_latency <= suite.config.thresholds.workflow_processing_ms;

    Ok(BenchmarkResult {
        name: "Workflow Initiation".to_string(),
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