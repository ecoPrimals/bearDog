/// Testing Configuration Module
//!
//! Contains all testing-related configuration structs and settings.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Unified testing configuration - consolidates all test config fragments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestExecutionMode {
    Sequential,
    Parallel,
    Adaptive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestingFeatures {
    pub performance_monitoring: bool,
    pub auto_cleanup: bool,
    pub verbose_logging: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestingConfig {
    pub enabled: bool,
    pub timeout: Duration,
    pub max_retries: u32,
    pub execution_mode: TestExecutionMode,
    pub max_concurrent_tests: usize,
    pub features: TestingFeatures,
    pub test_data_dir: String,

    // Consolidated from TestMetricsConfig
    pub metrics: TestMetricsConfig,

    // Consolidated from IntegrationTestConfig
    pub integration: IntegrationTestConfig,

    // Consolidated from TestHarnessConfig functionality
    pub harness: TestHarnessSettings,

    // Consolidated from chaos testing configs
    pub chaos: ChaosTestSettings,

    // Consolidated from benchmark configs
    pub benchmarks: BenchmarkSettings,
}

impl Default for TestExecutionMode {
    fn default() -> Self {
        Self::Parallel
    }
}

impl Default for TestingFeatures {
    fn default() -> Self {
        Self {
            performance_monitoring: true,
            auto_cleanup: true,
            verbose_logging: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestMetricsConfig {
    pub collect_metrics: bool,
    pub metrics_interval: Duration,
    pub store_results: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IntegrationTestConfig {
    pub enabled: bool,
    pub test_environment: String,
    pub cleanup_after_test: bool,
    pub reporting: ComplianceReportingConfig,
    pub monitoring_interval: Duration,
    pub audit_retention: Duration,
    pub data_sovereignty: DataSovereigntyConfig,
    pub privacy_audit: PrivacyAuditConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestHarnessSettings {
    pub auto_setup: bool,
    pub cleanup_timeout: Duration,
    pub resource_limits: TestResourceLimits,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChaosTestSettings {
    pub enabled: bool,
    pub failure_rate: f64,
    pub recovery_timeout: Duration,
    pub max_failures: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BenchmarkSettings {
    pub enabled: bool,
    pub iterations: usize,
    pub warmup_iterations: usize,
    pub measurement_time: Duration,
    pub baseline_comparison: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestResourceLimits {
    pub memory_mb: usize,
    pub cpu_percent: f64,
    pub disk_usage_mb: usize,
    pub network_connections: usize,
}

// Supporting compliance types for testing
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComplianceReportingConfig {
    pub enabled: bool,
    pub format: String,
    pub destination: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataSovereigntyConfig {
    pub enabled: bool,
    pub region: String,
    pub data_residency_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrivacyAuditConfig {
    pub enabled: bool,
    pub audit_interval: Duration,
    pub retention_period: Duration,
}
