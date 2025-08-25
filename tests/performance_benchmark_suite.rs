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