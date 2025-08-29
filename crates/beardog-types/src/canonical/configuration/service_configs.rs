/// Service Configuration Module
//!
//! Contains service-specific configuration structs for monitoring, workflows, performance, etc.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Monitoring and observability configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitoringConfig {
    pub enabled: bool,
    pub metrics_interval: Duration,
    pub enable_detailed_metrics: bool,
    pub retention_days: u32,
    pub alerting: AlertingConfig,
    pub health_checks: HealthCheckConfig,
    pub performance: PerformanceMonitoringConfig,
}

/// Workflow engine configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkflowConfig {
    pub engine_type: WorkflowEngineType,
    pub max_concurrent_workflows: usize,
    pub workflow_timeout: Duration,
    pub retry_policy: WorkflowRetryPolicy,
}

/// Performance tuning configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceConfig {
    pub max_threads: usize,
    pub buffer_size: usize,
    pub cache_size: usize,
    pub connection_pool_size: usize,
    pub enable_compression: bool,
    pub monitoring: PerformanceMonitoringConfig,
}

/// Service discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiscoveryConfig {
    pub enabled: bool,
    pub discovery_interval: Duration,
    pub service_timeout: Duration,
    pub max_services: usize,
    pub auto_registration: bool,
    pub health_check_interval: Duration,
}

/// Configuration management settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConfigManagerConfig {
    pub sources: ConfigSourcesConfig,
    pub validation: ConfigValidationConfig,
    pub caching: ConfigCachingConfig,
    pub reloading: ConfigReloadingConfig,
}

// Supporting types
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum WorkflowEngineType {
    #[default]
    Sequential,
    Parallel,
    Distributed,
    Genetic,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlertingConfig {
    pub enabled: bool,
    pub channels: Vec<String>,
    pub severity_levels: HashMap<String, u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HealthCheckConfig {
    pub interval: Duration,
    pub timeout: Duration,
    pub retries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceMonitoringConfig {
    pub enabled: bool,
    pub enable_detailed_metrics: bool,
    pub metrics_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkflowRetryPolicy {
    pub max_retries: u32,
    pub backoff_multiplier: f64,
    pub max_backoff: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConfigSourcesConfig {
    pub file_paths: Vec<String>,
    pub environment_prefix: String,
    pub watch_files: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConfigValidationConfig {
    pub strict_mode: bool,
    pub validate_on_load: bool,
    pub fail_on_unknown_fields: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConfigCachingConfig {
    pub enabled: bool,
    pub ttl: Duration,
    pub max_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConfigReloadingConfig {
    pub enabled: bool,
    pub watch_interval: Duration,
    pub auto_reload: bool,
}
