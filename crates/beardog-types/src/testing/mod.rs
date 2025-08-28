use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    pub enabled: bool,
    pub timeout: Duration,
    pub max_retries: u32,
    pub parallel_execution: bool,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_secs(300),
            max_retries: 3,
            parallel_execution: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestMetricsConfig {
    pub collect_metrics: bool,
    pub metrics_interval: Duration,
    pub store_results: bool,
}

impl Default for TestMetricsConfig {
    fn default() -> Self {
        Self {
            collect_metrics: true,
            metrics_interval: Duration::from_secs(1),
            store_results: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationTestConfig {
    pub enabled: bool,
    pub test_environment: String,
    pub cleanup_after_test: bool,
}

impl Default for IntegrationTestConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            test_environment: "test".to_string(),
            cleanup_after_test: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestHarnessConfig {
    pub test_config: TestConfig,
    pub metrics: TestMetricsConfig,
    pub integration: IntegrationTestConfig,
}
