

use crate::common::{TestContext, TestResult, TestMetrics, TestPhase};
use beardog_errors::BearDogError;
use beardog::{BearDogConfig, BearDogCore};
use serde_json::{json, Value as JsonValue};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{Mutex, RwLock};
use tracing::{debug, info, warn};
use uuid::Uuid;

pub struct BearDogTestHarness {

    pub config: TestHarnessConfig,

    pub core: Option<Arc<BearDogCore>>,

    pub test_configs: Arc<RwLock<HashMap<String, JsonValue>>>,

    pub active_contexts: Arc<Mutex<HashMap<String, TestContext>>>,

    pub shared_resources: Arc<RwLock<TestResourcePool>>,

    pub metrics_collector: Arc<TestMetricsCollector>,

    pub environment_state: Arc<RwLock<TestEnvironmentState>>,
}

#[derive(Debug, Clone)]
pub struct TestHarnessConfig {

    pub environment: TestEnvironment,

    pub enable_performance_monitoring: bool,

    pub enable_auto_cleanup: bool,

    pub default_timeout: Duration,

    pub enable_parallel_execution: bool,

    pub max_concurrent_tests: usize,

    pub enable_verbose_logging: bool,

    pub test_data_dir: String,

    pub enable_chaos_testing: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TestEnvironment {

    Unit,

    Integration,

    E2E,

    Performance,

    Chaos,

    Production,
}

#[derive(Debug, Default)]
pub struct TestResourcePool {

    pub database_connections: HashMap<String, String>,

    pub test_datasets: HashMap<String, JsonValue>,

    pub mock_services: HashMap<String, Arc<dyn MockService>>,

    pub temp_directories: Vec<String>,

    pub test_artifacts: HashMap<String, TestArtifact>,
}

#[derive(Debug, Clone)]
pub struct TestArtifact {
    pub artifact_id: String,
    pub artifact_type: String,
    pub content: JsonValue,
    pub created_at: Instant,
    pub metadata: HashMap<String, String>,
}

pub trait MockService: Send + Sync {
    fn service_name(&self) -> &str;
    fn reset(&self) -> Result<(), BearDogError>;
    fn configure(&self, config: JsonValue) -> Result<(), BearDogError>;
}

pub struct TestMetricsCollector {

    execution_metrics: Arc<RwLock<HashMap<String, TestExecutionMetrics>>>,

    performance_benchmarks: Arc<RwLock<Vec<PerformanceBenchmark>>>,

    resource_usage: Arc<RwLock<ResourceUsageMetrics>>,
}

#[derive(Debug, Clone)]
pub struct TestExecutionMetrics {
    pub test_name: String,
    pub execution_count: u64,
    pub success_count: u64,
    pub failure_count: u64,
    pub average_duration: Duration,
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub last_execution: Instant,
    pub error_patterns: HashMap<String, u64>,
}

#[derive(Debug, Clone)]
pub struct PerformanceBenchmark {
    pub benchmark_name: String,
    pub test_name: String,
    pub duration: Duration,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
    pub operations_per_second: f64,
    pub timestamp: Instant,
    pub metadata: HashMap<String, JsonValue>,
}

#[derive(Debug, Clone, Default)]
pub struct ResourceUsageMetrics {
    pub peak_memory_mb: f64,
    pub total_cpu_time_ms: u64,
    pub network_requests: u64,
    pub database_queries: u64,
    pub file_operations: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

#[derive(Debug, Default)]
pub struct TestEnvironmentState {
    pub is_initialized: bool,
    pub active_tests: u64,
    pub completed_tests: u64,
    pub failed_tests: u64,
    pub environment_errors: Vec<String>,
    pub last_cleanup: Option<Instant>,
}

impl BearDogTestHarness {

    pub fn new() -> Self {
        Self::with_config(TestHarnessConfig::default())
    }

    pub fn with_config(config: TestHarnessConfig) -> Self {
        Self {
            config,
            core: None,
            test_configs: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            active_contexts: Arc::new(Mutex::new(HashMap::with_capacity(16))),
            shared_resources: Arc::new(RwLock::new(TestResourcePool::default())),
            metrics_collector: Arc::new(TestMetricsCollector::new()),
            environment_state: Arc::new(RwLock::new(TestEnvironmentState::default())),
        }
    }

    pub async fn initialize(&mut self) -> TestResult<()> {
        info!("🚀 Initializing BearDog test harness for {} environment", 
              format_args!("{:?}", self.config.environment).to_string());

        let mut state = self.environment_state.write().await;
        
        if state.is_initialized {
            return Ok(());
        }

        let core_config = self.create_test_config().await?;
        self.core = Some(Arc::new(BearDogCore::new(core_config).await
            .map_err(|e| BearDogError::enhanced(
                "TEST_HARNESS_INIT",
                format_args!("Failed to initialize BearDog core for testing: {}", e).to_string(),
                beardog_errors::ErrorSeverity::High,
                beardog_errors::ErrorCategory::Initialization,
                "test_harness",
                "core_initialization",
                vec![
                    "Verify test configuration is valid".to_string(),
                    "Check test environment dependencies".to_string(),
                    "Review test harness setup parameters".to_string(),
                ],
            ))?));

        self.initialize_shared_resources().await?;

        self.setup_test_data().await?;

        self.metrics_collector.initialize().await?;

        state.is_initialized = true;
        info!("✅ Test harness initialized successfully");
        Ok(())
    }

    pub async fn create_test_context(&self, test_name: impl Into<&str>) -> TestResult<TestContext> {
        let test_name = test_name.into();
        let test_id = Uuid::new_v4().to_string();
        
        info!("🧪 Creating test context for: {}", test_name);

        let mut context = TestContext::new(&test_name);

        context.add_metadata("test_id", json!(test_id));
        context.add_metadata("environment", json!(format_args!("{:?}", self.config.environment).to_string()));
        context.add_metadata("harness_version", json!("2.0"));
        context.add_metadata("created_at", json!(chrono::Utc::now().to_rfc3339()));

        {
            let mut active_contexts = self.active_contexts.lock().await;
            active_contexts.insert(test_id.clone(), context.clone());
        }

        {
            let mut state = self.environment_state.write().await;
            state.active_tests += 1;
        }

        debug!("Created test context with ID: {}", test_id);
        Ok(context)
    }

    pub async fn run_test<F, Fut, T>(
        &self,
        test_name: impl Into<String>,
        test_fn: F,
    ) -> TestResult<T>
    where
        F: FnOnce(TestContext, Arc<BearDogCore>) -> Fut,
        Fut: std::future::Future<Output = TestResult<T>>,
    {
        let test_name = test_name.into();
        let context = self.create_test_context(&test_name).await?;
        
        let core = self.core.as_ref()
            .ok_or_else(|| BearDogError::configuration("Test harness not initialized - call initialize() first"))?
            .clone();

        let start_time = Instant::now();

        let result = if self.config.default_timeout > Duration::ZERO {
            tokio::time::timeout(self.config.default_timeout, test_fn(context.clone(), core)).await
                .map_err(|_| BearDogError::timeout(&format_args!("Test '{}' exceeded timeout of {:?}", test_name, self.config.default_timeout).to_string()))?
        } else {
            test_fn(context.clone(), core).await
        };

        let duration = start_time.elapsed();

        self.record_test_metrics(&test_name, &result, duration).await;

        {
            let mut state = self.environment_state.write().await;
            state.active_tests -= 1;
            if result.is_ok() {
                state.completed_tests += 1;
            } else {
                state.failed_tests += 1;
            }
        }

        {
            let mut active_contexts = self.active_contexts.lock().await;
            if let Some(test_context) = context.get_metadata("test_id").and_then(|v| v.as_str()) {
                active_contexts.remove(test_context);
            }
        }

        info!("🏁 Test '{}' completed in {:?} with result: {}", 
              test_name, duration, if result.is_ok() { "SUCCESS" } else { "FAILURE" });

        result
    }

    pub async fn get_statistics(&self) -> TestHarnessStatistics {
        let state = self.environment_state.read().await;
        let metrics = self.metrics_collector.get_summary().await;
        
        TestHarnessStatistics {
            environment: self.config.environment.clone(),
            total_tests_run: state.completed_tests + state.failed_tests,
            successful_tests: state.completed_tests,
            failed_tests: state.failed_tests,
            active_tests: state.active_tests,
            average_test_duration: metrics.average_duration,
            peak_memory_usage_mb: metrics.peak_memory_mb,
            total_cpu_time_ms: metrics.total_cpu_time_ms,
            is_initialized: state.is_initialized,
        }
    }

    pub async fn cleanup(&self) -> TestResult<()> {
        info!("🧹 Cleaning up test harness environment");

        {
            let mut resources = self.shared_resources.write().await;
            self.cleanup_shared_resources(&mut resources).await?;
        }

        {
            let mut active_contexts = self.active_contexts.lock().await;
            active_contexts.clear();
        }

        {
            let mut state = self.environment_state.write().await;
            state.last_cleanup = Some(Instant::now());
        }

        info!("✅ Test harness cleanup completed");
        Ok(())
    }

    async fn create_test_config(&self) -> TestResult<BearDogConfig> {
        let mut config = BearDogConfig::default();

        match self.config.environment {
            TestEnvironment::Unit => {
                config.enable_networking = false;
                config.enable_database = false;
                config.enable_metrics = self.config.enable_performance_monitoring;
            }
            TestEnvironment::Integration => {
                config.enable_networking = true;
                config.enable_database = true;
                config.enable_metrics = true;
            }
            TestEnvironment::E2E => {
                config.enable_networking = true;
                config.enable_database = true;
                config.enable_metrics = true;
                config.enable_api_server = true;
            }
            TestEnvironment::Performance => {
                config.enable_metrics = true;
                config.enable_performance_monitoring = true;
            }
            TestEnvironment::Chaos => {
                config.enable_chaos_testing = true;
                config.enable_fault_injection = true;
            }
            TestEnvironment::Production => {

                config.enable_all_features = true;
            }
        }

        Ok(config)
    }

    async fn initialize_shared_resources(&self) -> TestResult<()> {
        info!("🔧 Initializing shared test resources");

        let mut resources = self.shared_resources.write().await;

        std::fs::create_dir_all(&self.config.test_data_dir)
            .map_err(|e| BearDogError::io(&format_args!("Failed to create test data directory: {}", e).to_string()))?;

        match self.config.environment {
            TestEnvironment::Integration | TestEnvironment::E2E => {

                resources.database_connections.insert(
                    "test_db".to_string(),
                    "sqlite::memory:".to_string()
                );
            }
            _ => {}
        }

        Ok(())
    }

    async fn setup_test_data(&self) -> TestResult<()> {
        debug!("📁 Setting up test data");
        
        let mut resources = self.shared_resources.write().await;

        resources.test_datasets.insert(
            "sample_config".to_string(),
            json!({
                "test_mode": true,
                "environment": format_args!("{:?}", self.config.environment).to_string(),
                "created_at": chrono::Utc::now().to_rfc3339()
            })
        );

        Ok(())
    }

    async fn record_test_metrics<T>(&self, test_name: &str, result: &TestResult<T>, duration: Duration) {
        self.metrics_collector.record_test_execution(
            test_name, 
            result.is_ok(), 
            duration
        ).await;
    }

    async fn cleanup_shared_resources(&self, resources: &mut TestResourcePool) -> TestResult<()> {

        for temp_dir in &resources.temp_directories {
            if let Err(e) = std::fs::remove_dir_all(temp_dir) {
                warn!("Failed to cleanup temporary directory {}: {}", temp_dir, e);
            }
        }
        resources.temp_directories.clear();

        for (name, mock_service) in &resources.mock_services {
            if let Err(e) = mock_service.reset() {
                warn!("Failed to reset mock service {}: {}", name, e);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct TestHarnessStatistics {
    pub environment: TestEnvironment,
    pub total_tests_run: u64,
    pub successful_tests: u64,
    pub failed_tests: u64,
    pub active_tests: u64,
    pub average_test_duration: Duration,
    pub peak_memory_usage_mb: f64,
    pub total_cpu_time_ms: u64,
    pub is_initialized: bool,
}

impl TestMetricsCollector {
    pub fn new() -> Self {
        Self {
            execution_metrics: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            performance_benchmarks: Arc::new(RwLock::new(Vec::new())),
            resource_usage: Arc::new(RwLock::new(ResourceUsageMetrics::default())),
        }
    }

    pub async fn initialize(&self) -> TestResult<()> {
        debug!("📊 Initializing test metrics collector");
        Ok(())
    }

    pub async fn record_test_execution(&self, test_name: &str, success: bool, duration: Duration) {
        let mut metrics = self.execution_metrics.write().await;
        
        let entry = metrics.entry(test_name.to_string()).or_insert_with(|| TestExecutionMetrics {
            test_name: test_name.to_string(),
            execution_count: 0,
            success_count: 0,
            failure_count: 0,
            average_duration: Duration::ZERO,
            min_duration: duration,
            max_duration: duration,
            last_execution: Instant::now(),
            error_patterns: HashMap::with_capacity(16),
        });

        entry.execution_count += 1;
        if success {
            entry.success_count += 1;
        } else {
            entry.failure_count += 1;
        }

        entry.average_duration = Duration::from_nanos(
            (entry.average_duration.as_nanos() as u64 * (entry.execution_count - 1) + duration.as_nanos() as u64) 
            / entry.execution_count
        );
        entry.min_duration = entry.min_duration.min(duration);
        entry.max_duration = entry.max_duration.max(duration);
        entry.last_execution = Instant::now();
    }

    pub async fn get_summary(&self) -> TestMetricsSummary {
        let metrics = self.execution_metrics.read().await;
        let resource_usage = self.resource_usage.read().await;
        
        let total_executions: u64 = metrics.values().map(|m| m.execution_count).sum();
        let total_successes: u64 = metrics.values().map(|m| m.success_count).sum();
        
        let average_duration = if !metrics.is_empty() {
            let total_duration: Duration = metrics.values()
                .map(|m| m.average_duration)
                .sum();
            Duration::from_nanos(total_duration.as_nanos() as u64 / metrics.len() as u64)
        } else {
            Duration::ZERO
        };

        TestMetricsSummary {
            total_tests: metrics.len(),
            total_executions,
            success_rate: if total_executions > 0 { 
                (total_successes as f64 / total_executions as f64) * 100.0 
            } else { 
                0.0 
            },
            average_duration,
            peak_memory_mb: resource_usage.peak_memory_mb,
            total_cpu_time_ms: resource_usage.total_cpu_time_ms,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TestMetricsSummary {
    pub total_tests: usize,
    pub total_executions: u64,
    pub success_rate: f64,
    pub average_duration: Duration,
    pub peak_memory_mb: f64,
    pub total_cpu_time_ms: u64,
}

impl Default for TestHarnessConfig {
    fn default() -> Self {
        Self {
            environment: TestEnvironment::Unit,
            enable_performance_monitoring: true,
            enable_auto_cleanup: true,
            default_timeout: Duration::from_secs(30),
            enable_parallel_execution: true,
            max_concurrent_tests: 10,
            enable_verbose_logging: false,
            test_data_dir: "test_data".to_string(),
            enable_chaos_testing: false,
        }
    }
}

#[macro_export]
macro_rules! beardog_test {
    ($name:ident, $env:expr, $test:expr) => {
        #[tokio::test]
        async fn $name() -> $crate::common::TestResult<()> {
            let mut harness = $crate::common::harness::BearDogTestHarness::with_config(
                $crate::common::harness::TestHarnessConfig {
                    environment: $env,
                    ..Default::default()
                }
            );
            
            harness.initialize().await?;
            
            let result = harness.run_test(stringify!($name), $test).await;
            
            harness.cleanup().await?;
            
            result
        }
    };
}

#[macro_export]
macro_rules! unit_test {
    ($name:ident, $test:expr) => {
        beardog_test!($name, $crate::common::harness::TestEnvironment::Unit, $test);
    };
}

#[macro_export]
macro_rules! integration_test {
    ($name:ident, $test:expr) => {
        beardog_test!($name, $crate::common::harness::TestEnvironment::Integration, $test);
    };
}

#[macro_export]
macro_rules! e2e_test {
    ($name:ident, $test:expr) => {
        beardog_test!($name, $crate::common::harness::TestEnvironment::E2E, $test);
    };
} 