//! Performance Benchmark Suite for BearDog
//!
//! **Comprehensive Performance Testing & Optimization Validation**
//!
//! This benchmark suite provides:
//! - Crypto operations benchmarking (encryption, signing, hashing)  
//! - Network throughput and latency measurements
//! - Genetic algorithm performance validation
//! - Memory usage and allocation patterns
//! - Concurrent operation scalability
//! - Database query performance
//! - API endpoint response times

// Use the new modular benchmark structure
mod benchmarks;

// Re-export everything from the benchmarks module
pub use benchmarks::*;

// Performance benchmark test cases

#[tokio::test]
async fn test_benchmark_suite_initialization() -> beardog::BearDogResult<()> {
    let benchmark_suite = PerformanceBenchmarkSuite::new().await?;
    
    assert!(!benchmark_suite.config.data_sizes.is_empty());
    assert!(!benchmark_suite.config.concurrency_levels.is_empty());
    assert!(benchmark_suite.config.iterations > 0);
    
    tracing::info!("✅ Benchmark suite initialization test passed");
    Ok(())
}

#[tokio::test]
async fn test_crypto_benchmark_basic() -> beardog::BearDogResult<()> {
    let mut benchmark_suite = PerformanceBenchmarkSuite::new().await?;
    
    // Run a small subset for testing
    benchmark_suite.config.iterations = 10;
    benchmark_suite.config.data_sizes = vec![1024]; // 1KB only
    
    let crypto_results = benchmark_suite.benchmark_crypto_operations().await?;
    
    assert!(!crypto_results.is_empty());
    assert!(crypto_results.iter().all(|r| r.operations_per_second > 0.0));
    assert!(crypto_results.iter().all(|r| r.success_rate > 0.0));
    
    tracing::info!("✅ Crypto benchmark basic test passed");
    Ok(())
}

#[tokio::test]
async fn test_scalability_benchmark_basic() -> beardog::BearDogResult<()> {
    let mut benchmark_suite = PerformanceBenchmarkSuite::new().await?;
    
    // Test with small configuration
    benchmark_suite.config.iterations = 10;
    benchmark_suite.config.concurrency_levels = vec![1, 2];
    
    let scalability_results = benchmark_suite.benchmark_scalability().await?;
    
    assert_eq!(scalability_results.len(), 2);
    assert!(scalability_results.iter().all(|r| r.operations_per_second > 0.0));
    
    tracing::info!("✅ Scalability benchmark basic test passed");
    Ok(())
}

#[tokio::test]  
async fn test_comprehensive_benchmark_suite_small() -> beardog::BearDogResult<()> {
    let mut benchmark_suite = PerformanceBenchmarkSuite::new().await?;
    
    // Minimal configuration for testing
    benchmark_suite.config.iterations = 5;
    benchmark_suite.config.warmup_iterations = 2;
    benchmark_suite.config.data_sizes = vec![1024];
    benchmark_suite.config.concurrency_levels = vec![1];
    
    let report = benchmark_suite.run_benchmark_suite().await?;
    
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