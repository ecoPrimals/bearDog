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

use beardog::{
    config::*,
    core::*,
    security::*,
    genetics::*,
    workflows::*,
    tunnel::*,
    BearDogConfig, BearDogCore, BearDogResult,
};
use std::sync::Arc;
use std::time::Instant;
use tracing::info;

// Re-export from sub-modules
pub use models::*;
pub use crypto::*;
pub use network::*;
pub use genetic::*;
pub use workflow::*;
pub use database::*;
pub use scalability::*;
pub use resource::*;
pub use utils::*;

// Sub-modules with focused responsibilities
pub mod models;      // Data structures, configs, and result types
pub mod crypto;      // Cryptographic operations benchmarking
pub mod network;     // Network performance benchmarking
pub mod genetic;     // Genetic operations benchmarking
pub mod workflow;    // Workflow processing benchmarking
pub mod database;    // Database operations benchmarking
pub mod scalability; // Scalability and concurrency benchmarking
pub mod resource;    // Memory and resource usage benchmarking
pub mod utils;       // Helper functions and utilities

/// Comprehensive performance benchmark suite
pub struct PerformanceBenchmarkSuite {
    pub core: Arc<BearDogCore>,
    pub security_provider: Arc<BearDogSecurityProvider>,
    pub genetics_engine: Arc<DefaultBearDogGeneticsEngine>,
    pub workflow_engine: Arc<MultiPartyWorkflowEngine>,
    pub metrics_collector: Arc<PerformanceMetricsCollector>,
    pub config: BenchmarkConfig,
}

impl PerformanceBenchmarkSuite {
    /// Initialize the performance benchmark suite
    pub async fn new() -> BearDogResult<Self> {
        info!("🚀 Initializing Performance Benchmark Suite");

        // Create optimized configuration for benchmarking
        let mut config = BearDogConfig::default();
        config.database.url = ":memory:".to_string();
        config.api.bind_address = "127.0.0.1:0".to_string();
        config.security.level = SecurityLevel::High;
        config.performance.max_concurrent_requests = 1000;

        // Initialize core components
        let core = Arc::new(BearDogCore::new(config).await?);
        
        let security_config = SecurityProviderConfig::default();
        let security_provider = Arc::new(
            BearDogSecurityProvider::new_with_config(security_config).await?
        );

        let genetics_store = Arc::new(InMemoryGeneticsStore::new());
        let genetics_config = GeneticsConfig::default();
        let genetics_engine = Arc::new(
            DefaultBearDogGeneticsEngine::new(genetics_store, genetics_config)
        );

        let workflow_config = Arc::new(WorkflowConfig::default());
        let workflow_store = Arc::new(InMemoryWorkflowStore::new());
        let approval_store = Arc::new(InMemoryApprovalStore::new());
        let workflow_engine = Arc::new(
            MultiPartyWorkflowEngine::new(workflow_config, workflow_store, approval_store).await?
        );

        let metrics_collector = Arc::new(PerformanceMetricsCollector::new());
        let benchmark_config = BenchmarkConfig::default();

        Ok(Self {
            core,
            security_provider,
            genetics_engine,
            workflow_engine,
            metrics_collector,
            config: benchmark_config,
        })
    }

    /// Run comprehensive benchmark suite
    pub async fn run_benchmark_suite(&mut self) -> BearDogResult<BenchmarkReport> {
        info!("🏁 Starting Comprehensive Performance Benchmark Suite");
        let suite_start = Instant::now();

        let mut results = Vec::new();
        let mut total_operations = 0u64;

        // Crypto benchmarks
        info!("🔐 Running Cryptographic Operation Benchmarks");
        let crypto_results = self.benchmark_crypto_operations().await?;
        total_operations += crypto_results.iter().map(|r| (r.operations_per_second * 10.0) as u64).sum::<u64>();
        results.extend(crypto_results);

        // Network benchmarks
        info!("🌐 Running Network Performance Benchmarks");
        let network_results = self.benchmark_network_operations().await?;
        total_operations += network_results.iter().map(|r| (r.operations_per_second * 10.0) as u64).sum::<u64>();
        results.extend(network_results);

        // Genetic operations benchmarks
        info!("🧬 Running Genetic Operations Benchmarks");
        let genetic_results = self.benchmark_genetic_operations().await?;
        total_operations += genetic_results.iter().map(|r| (r.operations_per_second * 10.0) as u64).sum::<u64>();
        results.extend(genetic_results);

        // Workflow benchmarks
        info!("🔄 Running Workflow Processing Benchmarks");
        let workflow_results = self.benchmark_workflow_operations().await?;
        total_operations += workflow_results.iter().map(|r| (r.operations_per_second * 10.0) as u64).sum::<u64>();
        results.extend(workflow_results);

        // Database benchmarks
        info!("💾 Running Database Performance Benchmarks");
        let database_results = self.benchmark_database_operations().await?;
        total_operations += database_results.iter().map(|r| (r.operations_per_second * 10.0) as u64).sum::<u64>();
        results.extend(database_results);

        // Scalability benchmarks
        info!("📈 Running Scalability Benchmarks");
        let scalability_results = self.benchmark_scalability().await?;
        total_operations += scalability_results.iter().map(|r| (r.operations_per_second * 10.0) as u64).sum::<u64>();
        results.extend(scalability_results);

        // Memory and resource benchmarks
        info!("🧠 Running Memory & Resource Benchmarks");
        let resource_results = self.benchmark_resource_usage().await?;
        results.extend(resource_results);

        // Collect final metrics
        let performance_metrics = self.metrics_collector.calculate_final_metrics(&results).await;
        let system_info = self.collect_system_info().await;

        let execution_time = suite_start.elapsed().as_secs_f64();
        let performance_grade = self.calculate_performance_grade(&results, &performance_metrics);
        let recommendations = self.generate_performance_recommendations(&results, &performance_metrics);

        let report = BenchmarkReport {
            suite_name: "BearDog Comprehensive Performance Suite".to_string(),
            execution_time_seconds: execution_time,
            total_operations,
            benchmark_results: results,
            performance_metrics,
            system_info,
            performance_grade: performance_grade.clone(),
            recommendations,
        };

        info!("🎉 Benchmark Suite Completed!");
        info!("   Execution Time: {:.2}s", execution_time);
        info!("   Total Operations: {}", total_operations);
        info!("   Performance Grade: {}", performance_grade);

        Ok(report)
    }

    /// Delegate crypto operations to crypto module
    async fn benchmark_crypto_operations(&mut self) -> BearDogResult<Vec<BenchmarkResult>> {
        crypto::benchmark_crypto_operations(self).await
    }

    /// Delegate network operations to network module
    async fn benchmark_network_operations(&mut self) -> BearDogResult<Vec<BenchmarkResult>> {
        network::benchmark_network_operations(self).await
    }

    /// Delegate genetic operations to genetic module
    async fn benchmark_genetic_operations(&mut self) -> BearDogResult<Vec<BenchmarkResult>> {
        genetic::benchmark_genetic_operations(self).await
    }

    /// Delegate workflow operations to workflow module
    async fn benchmark_workflow_operations(&mut self) -> BearDogResult<Vec<BenchmarkResult>> {
        workflow::benchmark_workflow_operations(self).await
    }

    /// Delegate database operations to database module
    async fn benchmark_database_operations(&mut self) -> BearDogResult<Vec<BenchmarkResult>> {
        database::benchmark_database_operations(self).await
    }

    /// Delegate scalability operations to scalability module
    async fn benchmark_scalability(&mut self) -> BearDogResult<Vec<BenchmarkResult>> {
        scalability::benchmark_scalability(self).await
    }

    /// Delegate resource operations to resource module
    async fn benchmark_resource_usage(&mut self) -> BearDogResult<Vec<BenchmarkResult>> {
        resource::benchmark_resource_usage(self).await
    }

    /// Delegate system info collection to utils module
    async fn collect_system_info(&self) -> SystemInfo {
        utils::collect_system_info().await
    }

    /// Delegate performance grade calculation to utils module
    fn calculate_performance_grade(
        &self,
        results: &[BenchmarkResult], 
        metrics: &PerformanceMetrics
    ) -> String {
        utils::calculate_performance_grade(results, metrics)
    }

    /// Delegate recommendations generation to utils module
    fn generate_performance_recommendations(
        &self,
        results: &[BenchmarkResult],
        _metrics: &PerformanceMetrics
    ) -> Vec<String> {
        utils::generate_performance_recommendations(results)
    }
} 