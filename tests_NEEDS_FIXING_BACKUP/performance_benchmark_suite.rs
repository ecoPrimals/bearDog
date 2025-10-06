use beardog_errors::BearDogError;

mod benchmarks;

pub use benchmarks::*;

#[tokio::test]
async fn test_benchmark_suite_initialization() -> beardog::Result<(), BearDogError> {
    let benchmark_suite = PerformanceBenchmarkSuite::new()?;

    assert!(!benchmark_suite.config.data_sizes.is_empty());
    assert!(!benchmark_suite.config.concurrency_levels.is_empty());
    assert!(benchmark_suite.config.iterations > 0);

    tracing::info!("✅ Benchmark suite initialization test passed");
    Ok(())
}

#[tokio::test]
async fn test_crypto_benchmark_basic() -> beardog::Result<(), BearDogError> {
    let mut benchmark_suite = PerformanceBenchmarkSuite::new()?;

    benchmark_suite.config.iterations = 10;
    benchmark_suite.config.data_sizes = vec![1024]; // 1KB only

    let crypto_results = benchmark_suite.benchmark_crypto_operations()?;

    assert!(!crypto_results.is_empty());
    assert!(crypto_results.iter().all(|r| r.operations_per_second > 0.0));
    assert!(crypto_results.iter().all(|r| r.success_rate > 0.0));

    tracing::info!("✅ Crypto benchmark basic test passed");
    Ok(())
}

#[tokio::test]
async fn test_scalability_benchmark_basic() -> beardog::Result<(), BearDogError> {
    let mut benchmark_suite = PerformanceBenchmarkSuite::new()?;

    benchmark_suite.config.iterations = 10;
    benchmark_suite.config.concurrency_levels = vec![1, 2];

    let scalability_results = benchmark_suite.benchmark_scalability()?;

    assert_eq!(scalability_results.len(), 2);
    assert!(scalability_results
        .iter()
        .all(|r| r.operations_per_second > 0.0));

    tracing::info!("✅ Scalability benchmark basic test passed");
    Ok(())
}

#[tokio::test]
async fn test_comprehensive_benchmark_suite_small() -> beardog::Result<(), BearDogError> {
    let mut benchmark_suite = PerformanceBenchmarkSuite::new()?;

    benchmark_suite.config.iterations = 5;
    benchmark_suite.config.warmup_iterations = 2;
    benchmark_suite.config.data_sizes = vec![1024];
    benchmark_suite.config.concurrency_levels = vec![1];

    let report = benchmark_suite.run_benchmark_suite()?;

    assert!(!report.benchmark_results.is_empty());
    assert!(!report.performance_grade.is_empty());
    assert!(report.execution_time_seconds > 0.0);
    assert!(report.total_operations > 0);

    tracing::info!("🎉 Comprehensive benchmark suite test completed!");
    tracing::info!("   Performance Grade: {}", report.performance_grade);
    tracing::info!("   Total Operations: {}", report.total_operations);
    tracing::info!("   Execution Time: {:.2}s", report.execution_time_seconds);

    Ok(())
}
