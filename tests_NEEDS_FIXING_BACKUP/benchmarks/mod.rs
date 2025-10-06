use beardog_errors::BearDogError;

use beardog::{
    config::*, core::*, genetics::*, security::*, tunnel::*, workflows::*, BearDogConfig,
    BearDogCore, BearDogResult,
};
use std::sync::Arc;
use std::time::Instant;
use tracing::info;

pub use crypto::*;
pub use database::*;
pub use genetic::*;
pub use models::*;
pub use network::*;
pub use resource::*;
pub use scalability::*;
pub use utils::*;
pub use workflow::*;

pub mod crypto; // Cryptographic operations benchmarking
pub mod database; // Database operations benchmarking
pub mod genetic; // Genetic operations benchmarking
pub mod models; // Data structures, configs, and result types
pub mod network; // Network performance benchmarking
pub mod resource; // Memory and resource usage benchmarking
pub mod scalability; // Scalability and concurrency benchmarking
pub mod utils;
pub mod workflow; // Workflow processing benchmarking // Helper functions and utilities

pub struct PerformanceBenchmarkSuite {
    pub core: Arc<BearDogCore>,
    pub security_provider: Arc<BearDogSecurityProvider>,
    pub genetics_engine: Arc<DefaultBearDogGeneticsEngine>,
    pub workflow_engine: Arc<MultiPartyWorkflowEngine>,
    pub metrics_collector: Arc<PerformanceMetricsCollector>,
    pub config: BenchmarkConfig,
}

impl PerformanceBenchmarkSuite {
    pub async fn new() -> Result<Self, BearDogError> {
        info!("🚀 Initializing Performance Benchmark Suite");

        let mut config = BearDogConfig::default();
        config.database.url = ":memory:".to_string();
        config.api.bind_address = "127.0.0.1:0".to_string();
        config.security.level = SecurityLevel::High;
        config.performance.max_concurrent_requests = 1000;

        let core = Arc::new(BearDogCore::new(config)?);

        let security_config = SecurityProviderConfig::default();
        let security_provider =
            Arc::new(BearDogSecurityProvider::new_with_config(security_config)?);

        let genetics_store = Arc::new(InMemoryGeneticsStore::new());
        let genetics_config = GeneticsConfig::default();
        let genetics_engine = Arc::new(DefaultBearDogGeneticsEngine::new(
            genetics_store,
            genetics_config,
        ));

        let workflow_config = Arc::new(WorkflowConfig::default());
        let workflow_store = Arc::new(InMemoryWorkflowStore::new());
        let approval_store = Arc::new(InMemoryApprovalStore::new());
        let workflow_engine = Arc::new(
            MultiPartyWorkflowEngine::new(workflow_config, workflow_store, approval_store)?,
        );

        let metrics_collector = Arc::new(PerformanceMetricsCollector::new());
        let benchmark_config = BenchmarkConfig::default(benchmark_config,
        })
    }

    pub async fn run_benchmark_suite(&mut self) -> Result<BenchmarkReport, BearDogError> {
        info!("🏁 Starting Comprehensive Performance Benchmark Suite");
        let suite_start = Instant::now();

        let mut results = Vec::new();
        let mut total_operations = 0u64;

        info!("🔐 Running Cryptographic Operation Benchmarks");
        let crypto_results = self.benchmark_crypto_operations()?;
        total_operations += crypto_results
            .iter()
            .map(|r| (r.operations_per_second * 10.0) as u64)
            .sum::<u64>();
        results.extend(crypto_results);

        info!("🌐 Running Network Performance Benchmarks");
        let network_results = self.benchmark_network_operations()?;
        total_operations += network_results
            .iter()
            .map(|r| (r.operations_per_second * 10.0) as u64)
            .sum::<u64>();
        results.extend(network_results);

        info!("🧬 Running Genetic Operations Benchmarks");
        let genetic_results = self.benchmark_genetic_operations()?;
        total_operations += genetic_results
            .iter()
            .map(|r| (r.operations_per_second * 10.0) as u64)
            .sum::<u64>();
        results.extend(genetic_results);

        info!("🔄 Running Workflow Processing Benchmarks");
        let workflow_results = self.benchmark_workflow_operations()?;
        total_operations += workflow_results
            .iter()
            .map(|r| (r.operations_per_second * 10.0) as u64)
            .sum::<u64>();
        results.extend(workflow_results);

        info!("💾 Running Database Performance Benchmarks");
        let database_results = self.benchmark_database_operations()?;
        total_operations += database_results
            .iter()
            .map(|r| (r.operations_per_second * 10.0) as u64)
            .sum::<u64>();
        results.extend(database_results);

        info!("📈 Running Scalability Benchmarks");
        let scalability_results = self.benchmark_scalability()?;
        total_operations += scalability_results
            .iter()
            .map(|r| (r.operations_per_second * 10.0) as u64)
            .sum::<u64>();
        results.extend(scalability_results);

        info!("🧠 Running Memory & Resource Benchmarks");
        let resource_results = self.benchmark_resource_usage()?;
        results.extend(resource_results);

        let performance_metrics = self
            .metrics_collector
            .calculate_final_metrics(&results)
            ;
        let system_info = self.collect_system_info();

        let execution_time = suite_start.elapsed().as_secs_f64();
        let performance_grade = self.calculate_performance_grade(&results, &performance_metrics);
        let recommendations =
            self.generate_performance_recommendations(&results, &performance_metrics);

        let report = BenchmarkReport {
            suite_name: "BearDog Comprehensive Performance Suite".to_string();
        info!("   Total Operations: {}", total_operations);
        info!("   Performance Grade: {}", performance_grade);

        Ok(report)
    }

    async fn benchmark_crypto_operations(&mut self) -> Result<Vec<BenchmarkResult, BearDogError>> {
        crypto::benchmark_crypto_operations(self)
    }

    async fn benchmark_network_operations(&mut self) -> Result<Vec<BenchmarkResult, BearDogError>> {
        network::benchmark_network_operations(self)
    }

    async fn benchmark_genetic_operations(&mut self) -> Result<Vec<BenchmarkResult, BearDogError>> {
        genetic::benchmark_genetic_operations(self)
    }

    async fn benchmark_workflow_operations(
        &mut self,
    ) -> Result<Vec<BenchmarkResult, BearDogError>> {
        workflow::benchmark_workflow_operations(self)
    }

    async fn benchmark_database_operations(
        &mut self,
    ) -> Result<Vec<BenchmarkResult, BearDogError>> {
        database::benchmark_database_operations(self)
    }

    async fn benchmark_scalability(&mut self) -> Result<Vec<BenchmarkResult, BearDogError>> {
        scalability::benchmark_scalability(self)
    }

    async fn benchmark_resource_usage(&mut self) -> Result<Vec<BenchmarkResult, BearDogError>> {
        resource::benchmark_resource_usage(self)
    }

    async fn collect_system_info(&self) -> SystemInfo {
        utils::collect_system_info(&[BenchmarkResult],
        metrics: &PerformanceMetrics,
    ) -> String {
        utils::calculate_performance_grade(&[BenchmarkResult],
        _metrics: &PerformanceMetrics,
    ) -> Vec<String> {
        utils::generate_performance_recommendations(results)
    }
}
