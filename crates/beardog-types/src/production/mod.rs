// Production-Grade BearDog Ecosystem
//
// This module provides advanced production capabilities including comprehensive
// monitoring, observability, performance optimization, and operational excellence
// features for the unified BearDog ecosystem.

/// Health module
pub mod health;
/// Metrics module
pub mod metrics;
/// Monitoring module
pub mod monitoring;
/// Observability module
pub mod observability;
/// Optimization module
pub mod optimization;
/// Telemetry module
pub mod telemetry;

// Comprehensive test modules
#[cfg(test)]
mod health_comprehensive_tests;
#[cfg(test)]
mod metrics_comprehensive_tests;

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Production-grade ecosystem configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductionConfig {
    /// Core production settings
    /// The core value
    pub core: ProductionCoreConfig,
    /// Monitoring configuration
    /// The monitoring value
    pub monitoring: monitoring::MonitoringConfig,
    /// Observability settings
    /// The observability value
    pub observability: observability::ObservabilityConfig,
    /// The optimization value
    pub optimization: optimization::OptimizationConfig,
    /// Health check configuration
    /// The health value
    pub health: health::HealthConfig,
    /// Metrics collection settings
    /// The metrics value
    pub metrics: metrics::MetricsConfig,
    /// Telemetry configuration
    /// The telemetry value
    pub telemetry: telemetry::TelemetryConfig,
}

/// Core production configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionCoreConfig {
    /// Production environment level
    /// The environment level value
    pub environment_level: EnvironmentLevel,
    /// Service name
    /// Name of the service
    pub service_name: String,
    /// Service version
    /// The service version value
    pub service_version: String,
    /// Deployment identifier
    pub deployment_id: String,
    /// Region/availability zone
    /// The region value
    pub region: String,
    /// Cluster identifier
    pub cluster_id: String,
    /// Node identifier
    pub node_id: String,
    /// Production flags
    /// The flags value
    pub flags: ProductionFlags,
}

/// Production environment levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EnvironmentLevel {
    /// Development environment level
    Development,
    /// Staging environment level
    Staging,
    /// Pre-production environment level
    PreProduction,
    /// Production environment level
    Production,
    /// Critical production environment level
    Critical,
}

/// Production feature flags
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct ProductionFlags {
    /// Enable advanced monitoring
    /// Whether `enable_advanced_monitoring` is enabled
    pub enable_advanced_monitoring: bool,
    /// Enable distributed tracing
    /// Whether `enable_distributed_tracing` is enabled
    pub enable_distributed_tracing: bool,
    pub enable_performance_profiling: bool,
    /// Enable security auditing
    /// Whether `enable_security_auditing` is enabled
    pub enable_security_auditing: bool,
    /// Enable auto-scaling
    /// Whether `enable_auto_scaling` is enabled
    pub enable_auto_scaling: bool,
    /// Enable circuit breakers
    /// Whether `enable_circuit_breakers` is enabled
    pub enable_circuit_breakers: bool,
    /// Enable rate limiting
    /// Whether `enable_rate_limiting` is enabled
    pub enable_rate_limiting: bool,
    /// Enable caching
    /// Whether `enable_caching` is enabled
    pub enable_caching: bool,
}

/// Production ecosystem manager
#[derive(Debug)]
pub struct ProductionEcosystem {
    /// Production configuration
    pub config: ProductionConfig,
    /// Runtime metrics collector
    /// The metrics collector value
    pub metrics_collector: metrics::ProductionMetricsCollector,
    /// Health checker
    /// The health checker value
    pub health_checker: health::HealthChecker,
    /// The optimizer value
    pub optimizer: optimization::PerformanceOptimizer,
    /// Observability engine
    /// The observability value
    pub observability: observability::ObservabilityEngine,
    start_time: Instant,
    /// Runtime state
    state: ProductionState,
}

/// Production runtime state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionState {
    /// Current operational status
    /// Current status of the component
    pub status: OperationalStatus,
    /// Active connections count
    /// Number of `active_connections`
    pub active_connections: u64,
    /// Total requests processed
    /// Number of `total_requests`
    pub total_requests: u64,
    /// Current memory usage (bytes)
    /// Number of `memory_usage_bytes`
    pub memory_usage_bytes: u64,
    /// Current CPU usage percentage
    /// The cpu usage percent value
    pub cpu_usage_percent: f64,
    /// Error count in last hour
    /// Number of `error_hourly`
    pub error_count_hourly: u64,
    pub performance: PerformanceMetrics,
}

/// Operational status levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OperationalStatus {
    /// System is initializing
    Initializing,
    /// System is healthy and operating normally
    Healthy,
    /// System is degraded but functional
    Degraded,
    /// System is unhealthy with significant issues
    Unhealthy,
    /// System is in critical state requiring immediate attention
    Critical,
    /// System is shutting down
    Shutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Average response time (milliseconds)
    pub avg_response_time_ms: f64,
    /// 95th percentile response time
    pub p95_response_time_ms: f64,
    /// 99th percentile response time
    pub p99_response_time_ms: f64,
    /// Requests per second
    /// The requests per second value
    pub requests_per_second: f64,
    /// Error rate percentage
    /// The error rate percent value
    pub error_rate_percent: f64,
    /// Throughput (bytes/second)
    /// Number of `throughput_bytes_per_sec`
    pub throughput_bytes_per_sec: u64,
}

impl Default for ProductionCoreConfig {
    fn default() -> Self {
        Self {
            environment_level: EnvironmentLevel::Development,
            service_name: "beardog-ecosystem".to_string(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            deployment_id: uuid::Uuid::new_v4().to_string(),
            region: "local".to_string(),
            cluster_id: "default-cluster".to_string(),
            node_id: std::env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string()),
            flags: ProductionFlags::default(),
        }
    }
}

impl Default for ProductionFlags {
    fn default() -> Self {
        Self {
            enable_advanced_monitoring: true,
            enable_distributed_tracing: true,
            enable_performance_profiling: true,
            enable_security_auditing: true,
            enable_auto_scaling: false,
            enable_circuit_breakers: true,
            enable_rate_limiting: true,
            enable_caching: true,
        }
    }
}

impl Default for ProductionState {
    fn default() -> Self {
        Self {
            status: OperationalStatus::Initializing,
            active_connections: 0,
            total_requests: 0,
            memory_usage_bytes: 0,
            cpu_usage_percent: 0.0,
            error_count_hourly: 0,
            performance: PerformanceMetrics::default(),
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            avg_response_time_ms: 0.0,
            p95_response_time_ms: 0.0,
            p99_response_time_ms: 0.0,
            requests_per_second: 0.0,
            error_rate_percent: 0.0,
            throughput_bytes_per_sec: 0,
        }
    }
}

impl ProductionEcosystem {
    /// Create a new production ecosystem
    /// Creates a new instance
    pub fn new(config: ProductionConfig) -> Result<Self, BearDogError> {
        let metrics_collector = metrics::ProductionMetricsCollector::new(config.metrics.clone());
        let health_checker = health::HealthChecker::new(&config.health)?;
        let optimizer = optimization::PerformanceOptimizer::new(&config.optimization)?;
        let observability = observability::ObservabilityEngine::new(&config.observability)?;

        Ok(Self {
            config,
            metrics_collector,
            health_checker,
            optimizer,
            observability,
            start_time: std::time::Instant::now(),
            state: ProductionState::default(),
        })
    }

    /// Initialize the production ecosystem
    /// Initializes componentialize
    /// Initializes componentialize
    pub fn initialize(&mut self) -> Result<(), BearDogError> {
        self.state.status = OperationalStatus::Initializing;

        // Initialize monitoring
        self.observability.start_monitoring()?;

        // Start health checks
        self.health_checker.start_health_monitoring()?;

        // Initialize performance optimization
        self.optimizer.initialize_optimizations()?;

        // Start metrics collection
        self.metrics_collector.start_collection()?;

        self.state.status = OperationalStatus::Healthy;
        Ok(())
    }

    /// Get current production status
    #[must_use]
    /// Gets status
    /// Gets status
    pub fn get_status(&self) -> &ProductionState {
        &self.state
    }

    /// Get system uptime
    #[must_use]
    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Updates metrics
    /// Updates metrics
    pub fn update_metrics(&mut self) -> Result<(), BearDogError> {
        // Collect current metrics
        let current_metrics = self.metrics_collector.collect_current_metrics()?;

        // Update state
        self.state.cpu_usage_percent = current_metrics.cpu_usage_percent;
        // Note: memory_usage_bytes field mapping would need to be added to state if needed

        // Trigger optimization if needed
        if self.config.core.flags.enable_auto_scaling {
            self.optimizer.evaluate_scaling_needs(&self.state)?;
        }

        Ok(())
    }

    pub fn health_check(&mut self) -> Result<health::HealthReport, BearDogError> {
        let report = self.health_checker.comprehensive_health_check()?;

        // Update operational status based on health
        self.state.status = match report.overall_status {
            health::HealthStatus::Healthy => OperationalStatus::Healthy,
            health::HealthStatus::Degraded => OperationalStatus::Degraded,
            health::HealthStatus::Unhealthy => OperationalStatus::Unhealthy,
            health::HealthStatus::Critical => OperationalStatus::Critical,
        };

        Ok(report)
    }

    /// Shutdown the production ecosystem gracefully
    pub fn shutdown(&mut self) -> Result<(), BearDogError> {
        self.state.status = OperationalStatus::Shutdown;

        // Stop metrics collection
        self.metrics_collector.stop_collection()?;

        // Stop health monitoring
        self.health_checker.stop_monitoring()?;

        // Stop observability
        self.observability.stop_monitoring()?;

        Ok(())
    }
}

pub struct ProductionEcosystemBuilder {
    config: ProductionConfig,
}

impl ProductionEcosystemBuilder {
    /// Create a new builder
    #[must_use]
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            config: ProductionConfig::default(),
        }
    }

    /// Set environment level
    #[must_use]
    pub fn environment_level(mut self, level: EnvironmentLevel) -> Self {
        self.config.core.environment_level = level;
        self
    }

    #[must_use]
    pub fn service(mut self, name: String, version: String) -> Self {
        self.config.core.service_name = name;
        self.config.core.service_version = version;
        self
    }

    #[must_use]
    pub fn deployment(mut self, id: String, region: String, cluster: String) -> Self {
        self.config.core.deployment_id = id;
        self.config.core.region = region;
        self.config.core.cluster_id = cluster;
        self
    }

    /// Enable advanced features
    #[must_use]
    pub fn enable_advanced_features(mut self) -> Self {
        self.config.core.flags.enable_advanced_monitoring = true;
        self.config.core.flags.enable_distributed_tracing = true;
        self.config.core.flags.enable_performance_profiling = true;
        self.config.core.flags.enable_security_auditing = true;
        self
    }

    /// Create production ecosystem from builder
    /// Builds component
    /// Builds component
    pub fn build(self) -> Result<ProductionEcosystem, BearDogError> {
        ProductionEcosystem::new(self.config)
    }
}

impl Default for ProductionEcosystemBuilder {
    fn default() -> Self {
        Self::new()
    }
}
