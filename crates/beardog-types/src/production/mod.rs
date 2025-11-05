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
#[cfg(test)]
mod tests_advanced;
// Test modules removed - migrated to production test suite

// New test modules - October 24, 2025
#[cfg(test)]
mod ecosystem_tests;
#[cfg(test)]
mod monitoring_advanced_tests;
#[cfg(test)]
mod observability_tests;
#[cfg(test)]
mod optimization_tests;
#[cfg(test)]
mod telemetry_tests;

// October 25, 2025: Week 2 Test Expansion
#[cfg(test)]
mod production_types_validation_tests;

// October 26, 2025: Week 2 Day 3 - Lifecycle Tests
#[cfg(test)]
mod ecosystem_lifecycle_tests;
#[cfg(test)]
mod production_integration_tests;

// October 27, 2025: Week 1 Test Expansion - Production Core Tests
#[cfg(test)]
mod production_core_tests;

// October 28, 2025: Evening Test Expansion - Comprehensive Mod Tests
#[cfg(test)]
mod mod_comprehensive_tests;

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Production-grade ecosystem configuration for BearDog
///
/// This is the primary configuration struct for production deployments, providing
/// comprehensive control over monitoring, observability, health checks, metrics,
/// and performance optimization.
///
/// # Architecture
///
/// The configuration is modular, with each domain (monitoring, observability, etc.)
/// having its own sub-configuration that can be customized independently.
///
/// # Environment Variables
///
/// Many settings can be overridden via environment variables:
/// - `BEARDOG_ENV` - Environment level (development, staging, production)
/// - `BEARDOG_SERVICE_NAME` - Service identifier
/// - `BEARDOG_METRICS_ENABLED` - Enable/disable metrics collection
///
/// # Examples
///
/// ## Basic Production Configuration
///
/// ```rust,ignore
/// use beardog_types::production::ProductionConfig;
///
/// // Start with defaults
/// let mut config = ProductionConfig::default();
///
/// // Customize for your environment
/// config.core.service_name = "beardog-api".to_string();
/// config.monitoring.enabled = true;
/// config.health.check_interval_secs = 30;
/// ```
///
/// ## Environment-Aware Configuration
///
/// ```rust,ignore
/// use beardog_types::production::{ProductionConfig, EnvironmentLevel};
///
/// let config = ProductionConfig {
///     core: beardog_types::production::ProductionCoreConfig {
///         environment_level: EnvironmentLevel::Production,
///         service_name: std::env::var("SERVICE_NAME")
///             .unwrap_or_else(|_| "beardog".to_string()),
///         ..Default::default()
///     },
///     ..Default::default()
/// };
/// ```
///
/// ## Loading from Environment
///
/// ```rust,ignore
/// use beardog_types::production::ProductionConfig;
///
/// // Load configuration with environment overrides
/// let config = ProductionConfig::from_env()
///     .expect("Failed to load production config");
/// ```
///
/// # See Also
///
/// - [`ProductionCoreConfig`] - Core production settings
/// - [`monitoring::MonitoringConfig`] - Monitoring configuration
/// - [`health::HealthConfig`] - Health check settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductionConfig {
    /// Core production settings including environment, service identity, and deployment info
    pub core: ProductionCoreConfig,
    /// Monitoring configuration for system and application metrics
    pub monitoring: monitoring::MonitoringConfig,
    /// Observability settings for tracing, logging, and debugging
    pub observability: observability::ObservabilityConfig,
    /// Performance optimization settings for zero-copy operations and caching
    pub optimization: optimization::OptimizationConfig,
    /// Health check configuration for readiness and liveness probes
    pub health: health::HealthConfig,
    /// Metrics collection settings for Prometheus, StatsD, and custom exporters
    pub metrics: metrics::MetricsConfig,
    /// Telemetry configuration for distributed tracing and APM integration
    pub telemetry: telemetry::TelemetryConfig,
}

/// Core production configuration settings
///
/// Contains essential identity and deployment information for production services.
/// These settings are typically set once during service initialization and rarely
/// change during runtime.
///
/// # Examples
///
/// ```rust,ignore
/// use beardog_types::production::{ProductionCoreConfig, EnvironmentLevel, ProductionFlags};
///
/// let core_config = ProductionCoreConfig {
///     environment_level: EnvironmentLevel::Production,
///     service_name: "beardog-security".to_string(),
///     service_version: env!("CARGO_PKG_VERSION").to_string(),
///     deployment_id: std::env::var("DEPLOYMENT_ID")
///         .unwrap_or_else(|_| "local".to_string()),
///     region: "us-west-2".to_string(),
///     cluster_id: "prod-cluster-01".to_string(),
///     node_id: hostname::get()
///         .unwrap_or_default()
///         .to_string_lossy()
///         .to_string(),
///     flags: ProductionFlags::production_defaults(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionCoreConfig {
    /// Production environment level (Development, Staging, Production, etc.)
    pub environment_level: EnvironmentLevel,
    /// Service name identifier (e.g., "beardog-security", "beardog-api")
    pub service_name: String,
    /// Semantic version of the service (e.g., "3.0.0")
    pub service_version: String,
    /// Unique identifier for this deployment instance
    pub deployment_id: String,
    /// Cloud region or availability zone (e.g., "us-west-2", "eu-central-1")
    pub region: String,
    /// Cluster identifier for orchestration platforms (K8s, ECS, etc.)
    pub cluster_id: String,
    /// Node/host identifier (typically hostname or instance ID)
    pub node_id: String,
    /// Production feature flags for enabling/disabling capabilities
    pub flags: ProductionFlags,
}

/// Production environment levels indicating deployment stage
///
/// Used to adjust behavior, logging verbosity, and safety checks based on
/// the deployment environment. Higher levels enable stricter validation and
/// more comprehensive monitoring.
///
/// # Environment Variable
///
/// Can be set via `BEARDOG_ENV` or `ENVIRONMENT`:
/// - `development` or `dev` → [`Development`](Self::Development)
/// - `staging` or `stage` → [`Staging`](Self::Staging)
/// - `preproduction` or `preprod` → [`PreProduction`](Self::PreProduction)
/// - `production` or `prod` → [`Production`](Self::Production)
/// - `critical` → [`Critical`](Self::Critical)
///
/// # Examples
///
/// ```rust,ignore
/// use beardog_types::production::EnvironmentLevel;
///
/// let env = EnvironmentLevel::from_env()
///     .unwrap_or(EnvironmentLevel::Development);
///
/// match env {
///     EnvironmentLevel::Development => {
///         // Enable debug logging, relaxed validation
///     }
///     EnvironmentLevel::Production | EnvironmentLevel::Critical => {
///         // Enable strict validation, comprehensive monitoring
///     }
///     _ => {}
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EnvironmentLevel {
    /// Development environment - verbose logging, relaxed validation
    Development,
    /// Staging environment - production-like with additional debugging
    Staging,
    /// Pre-production environment - final validation before production
    PreProduction,
    /// Production environment - optimized for performance and reliability
    Production,
    /// Critical production - highest reliability, strictest validation
    Critical,
}

/// Production feature flags for runtime capability control
///
/// Allows dynamic enabling/disabling of production features for gradual rollouts,
/// A/B testing, and emergency feature toggles without code deployment.
///
/// # Design Philosophy
///
/// - **Safe Defaults**: All flags default to safe values for the environment
/// - **Runtime Control**: Can be modified via environment variables or config management
/// - **Zero-Cost**: Flags are checked once at startup; no runtime overhead
///
/// # Examples
///
/// ```rust
/// use beardog_types::production::ProductionFlags;
///
/// // Production configuration with all safety features
/// let production_flags = ProductionFlags {
///     enable_advanced_monitoring: true,
///     enable_distributed_tracing: true,
///     enable_performance_profiling: false, // Only in staging
///     enable_security_auditing: true,
///     enable_auto_scaling: true,
///     enable_circuit_breakers: true,
///     enable_rate_limiting: true,
///     enable_caching: true,
/// };
///
/// // Development configuration for debugging
/// let dev_flags = ProductionFlags {
///     enable_advanced_monitoring: false,
///     enable_distributed_tracing: true, // Always useful
///     enable_performance_profiling: true,
///     enable_security_auditing: false,
///     enable_auto_scaling: false,
///     enable_circuit_breakers: false,
///     enable_rate_limiting: false,
///     enable_caching: false,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct ProductionFlags {
    /// Enable advanced monitoring with detailed metrics and alerting
    pub enable_advanced_monitoring: bool,
    /// Enable distributed tracing for request flow visualization (OpenTelemetry, Jaeger)
    pub enable_distributed_tracing: bool,
    /// Enable performance profiling and flame graph generation (development/staging only)
    pub enable_performance_profiling: bool,
    /// Enable comprehensive security audit logging for compliance
    pub enable_security_auditing: bool,
    /// Enable automatic horizontal scaling based on load metrics
    pub enable_auto_scaling: bool,
    /// Enable circuit breakers for fault tolerance and cascading failure prevention
    pub enable_circuit_breakers: bool,
    /// Enable request rate limiting to prevent abuse and ensure fair resource allocation
    pub enable_rate_limiting: bool,
    /// Enable response caching for improved performance and reduced load
    pub enable_caching: bool,
}

/// Production ecosystem manager for BearDog services
///
/// The main orchestrator for production deployments, managing monitoring, health checks,
/// performance optimization, and observability across the entire BearDog ecosystem.
///
/// # Architecture
///
/// `ProductionEcosystem` coordinates multiple subsystems:
/// - **Metrics Collection**: Real-time performance and business metrics
/// - **Health Checking**: Service readiness and liveness probes
/// - **Performance Optimization**: Zero-copy operations, caching, and resource management
/// - **Observability**: Distributed tracing, logging, and APM integration
///
/// # Lifecycle
///
/// 1. **Initialization**: Load configuration, start subsystems
/// 2. **Runtime**: Continuous monitoring and optimization
/// 3. **Shutdown**: Graceful cleanup and final metric flush
///
/// # Examples
///
/// ## Basic Setup
///
/// ```rust,ignore
/// use beardog_types::production::{ProductionEcosystem, ProductionConfig};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = ProductionConfig::default();
/// let ecosystem = ProductionEcosystem::new(config).await?;
///
/// // Start monitoring and health checks
/// ecosystem.start().await?;
///
/// // Check system health
/// let health = ecosystem.get_health_status().await?;
/// println!("System health: {:?}", health);
///
/// // Get current metrics
/// let metrics = ecosystem.get_metrics().await?;
/// println!("Requests/sec: {}", metrics.performance.requests_per_second);
/// # Ok(())
/// # }
/// ```
///
/// ## Production Deployment
///
/// ```rust,ignore
/// use beardog_types::production::{
///     ProductionEcosystem, ProductionConfig, EnvironmentLevel
/// };
///
/// # async fn production_example() -> Result<(), Box<dyn std::error::Error>> {
/// let mut config = ProductionConfig::default();
/// config.core.environment_level = EnvironmentLevel::Production;
/// config.monitoring.enabled = true;
/// config.health.check_interval_secs = 30;
///
/// let ecosystem = ProductionEcosystem::new(config).await?;
/// ecosystem.start().await?;
///
/// // Production monitoring loop
/// loop {
///     let state = ecosystem.get_state().await?;
///     
///     if state.status == beardog_types::production::OperationalStatus::Critical {
///         // Alert on-call team
///         eprintln!("CRITICAL: System in critical state!");
///     }
///     
///     tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
/// }
/// # }
/// ```
#[derive(Debug)]
pub struct ProductionEcosystem {
    /// Production configuration controlling all subsystem behavior
    pub config: ProductionConfig,
    /// Real-time metrics collector for Prometheus, StatsD, and custom exporters
    pub metrics_collector: metrics::ProductionMetricsCollector,
    /// Health checker for Kubernetes readiness/liveness probes
    pub health_checker: health::HealthChecker,
    /// Performance optimizer for zero-copy operations and resource management
    pub optimizer: optimization::PerformanceOptimizer,
    /// Observability engine for distributed tracing and APM
    pub observability: observability::ObservabilityEngine,
    /// Service start time for uptime calculations
    start_time: Instant,
    /// Current runtime state including metrics and operational status
    state: ProductionState,
}

/// Production runtime state snapshot
///
/// Provides a point-in-time view of system health, resource usage, and performance metrics.
/// This struct is designed for serialization and can be exported to monitoring dashboards,
/// log aggregators, or stored for historical analysis.
///
/// # Usage Patterns
///
/// - **Health Checks**: Return state in readiness/liveness probe responses
/// - **Monitoring**: Export to Prometheus, Grafana, or custom dashboards
/// - **Alerting**: Trigger alerts based on status or metric thresholds
/// - **Debugging**: Snapshot system state for incident analysis
///
/// # Examples
///
/// ```rust
/// use beardog_types::production::{ProductionState, OperationalStatus};
///
/// let state = ProductionState {
///     status: OperationalStatus::Healthy,
///     active_connections: 150,
///     total_requests: 1_000_000,
///     memory_usage_bytes: 512 * 1024 * 1024, // 512 MB
///     cpu_usage_percent: 45.2,
///     error_count_hourly: 5,
///     performance: Default::default(),
/// };
///
/// // Check if system is healthy
/// if state.status == OperationalStatus::Healthy {
///     println!("✅ System healthy");
/// }
///
/// // Monitor resource usage
/// if state.cpu_usage_percent > 80.0 {
///     eprintln!("⚠️ High CPU usage: {}%", state.cpu_usage_percent);
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionState {
    /// Current operational status (Healthy, Degraded, Critical, etc.)
    pub status: OperationalStatus,
    /// Number of currently active client connections
    pub active_connections: u64,
    /// Total number of requests processed since service start
    pub total_requests: u64,
    /// Current memory usage in bytes (RSS or working set)
    pub memory_usage_bytes: u64,
    /// Current CPU usage as percentage (0.0-100.0)
    pub cpu_usage_percent: f64,
    /// Number of errors in the last hour for error rate calculation
    pub error_count_hourly: u64,
    /// Detailed performance metrics including latency percentiles
    pub performance: PerformanceMetrics,
}

/// Operational status levels for production services
///
/// Represents the current health and operational state of a BearDog service.
/// Status levels determine readiness probe responses, alerting behavior, and
/// automatic remediation actions.
///
/// # Status Progression
///
/// Normal operation: `Initializing` → `Healthy`
/// Degradation: `Healthy` → `Degraded` → `Unhealthy` → `Critical`
/// Recovery: `Degraded` → `Healthy`
/// Shutdown: Any state → `Shutdown`
///
/// # Kubernetes Integration
///
/// - **Readiness Probe**: Only `Healthy` returns HTTP 200
/// - **Liveness Probe**: `Healthy` and `Degraded` return HTTP 200
/// - **Critical State**: Triggers pod restart after grace period
///
/// # Examples
///
/// ```rust
/// use beardog_types::production::OperationalStatus;
///
/// fn check_status(status: &OperationalStatus) -> Result<(), String> {
///     match status {
///         OperationalStatus::Healthy => {
///             println!("✅ All systems operational");
///             Ok(())
///         }
///         OperationalStatus::Degraded => {
///             eprintln!("⚠️ Service degraded - some features impaired");
///             Ok(())
///         }
///         OperationalStatus::Unhealthy => {
///             eprintln!("❌ Service unhealthy - significant issues");
///             Err("Service unhealthy".to_string())
///         }
///         OperationalStatus::Critical => {
///             eprintln!("🚨 CRITICAL - immediate action required");
///             Err("Critical failure".to_string())
///         }
///         OperationalStatus::Initializing => {
///             println!("🔄 Service starting...");
///             Err("Not ready".to_string())
///         }
///         OperationalStatus::Shutdown => {
///             println!("🛑 Service shutting down");
///             Err("Shutting down".to_string())
///         }
///     }
/// }
/// ```
///
/// # Alerting Thresholds
///
/// - `Healthy`: No alerts
/// - `Degraded`: Warning alert (non-critical)
/// - `Unhealthy`: Error alert (page during business hours)
/// - `Critical`: Critical alert (page immediately, 24/7)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OperationalStatus {
    /// System is starting up and not yet ready to serve traffic
    Initializing,
    /// System is fully operational with all health checks passing
    Healthy,
    /// System is operational but with reduced capacity or performance
    Degraded,
    /// System has significant issues affecting functionality
    Unhealthy,
    /// System is in critical failure state requiring immediate intervention
    Critical,
    /// System is gracefully shutting down
    Shutdown,
}

/// Performance metrics for latency, throughput, and reliability monitoring
///
/// Provides comprehensive performance statistics for SLA monitoring, capacity planning,
/// and performance regression detection. All latency metrics are in milliseconds.
///
/// # Metric Definitions
///
/// - **Average Response Time**: Mean latency across all requests in measurement window
/// - **P95**: 95th percentile latency - 95% of requests faster than this
/// - **P99**: 99th percentile latency - 99% of requests faster than this  
/// - **RPS**: Requests per second throughput
/// - **Error Rate**: Percentage of failed requests (0.0-100.0)
/// - **Throughput**: Network throughput in bytes per second
///
/// # SLA Monitoring
///
/// Typical production SLA thresholds:
/// - P95 < 100ms for interactive endpoints
/// - P99 < 500ms for all endpoints
/// - Error rate < 0.1% (99.9% success)
/// - Availability > 99.9% uptime
///
/// # Examples
///
/// ```rust
/// use beardog_types::production::PerformanceMetrics;
///
/// let metrics = PerformanceMetrics {
///     avg_response_time_ms: 25.4,
///     p95_response_time_ms: 45.8,
///     p99_response_time_ms: 120.5,
///     requests_per_second: 1500.0,
///     error_rate_percent: 0.05, // 0.05% = 99.95% success rate
///     throughput_bytes_per_sec: 10 * 1024 * 1024, // 10 MB/s
/// };
///
/// // Check SLA compliance
/// if metrics.p95_response_time_ms > 100.0 {
///     eprintln!("⚠️ P95 latency exceeds SLA: {}ms", metrics.p95_response_time_ms);
/// }
///
/// if metrics.error_rate_percent > 0.1 {
///     eprintln!("❌ Error rate exceeds SLA: {}%", metrics.error_rate_percent);
/// }
///
/// // Capacity planning
/// let requests_per_day = metrics.requests_per_second * 86400.0;
/// println!("Daily request volume: {:.0}", requests_per_day);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Average (mean) response time across all requests in milliseconds
    pub avg_response_time_ms: f64,
    /// 95th percentile response time - 95% of requests complete faster than this
    pub p95_response_time_ms: f64,
    /// 99th percentile response time - 99% of requests complete faster than this
    pub p99_response_time_ms: f64,
    /// Current request rate in requests per second
    pub requests_per_second: f64,
    /// Error rate as percentage (0.0-100.0), where 0.1 = 99.9% success rate
    pub error_rate_percent: f64,
    /// Network throughput in bytes per second for response payloads
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
    /// Creates a new production ecosystem with the given configuration
    ///
    /// This initializes all production subsystems including:
    /// - Metrics collection
    /// - Health monitoring
    /// - Performance optimization
    /// - Observability engine
    ///
    /// # Arguments
    ///
    /// * `config` - Production configuration defining monitoring, health, and optimization settings
    ///
    /// # Returns
    ///
    /// Returns a `ProductionEcosystem` instance ready for initialization
    ///
    /// # Errors
    ///
    /// Returns an error if any subsystem fails to initialize
    ///
    /// # Examples
    ///
    /// ```rust
    /// use beardog_types::production::{ProductionEcosystem, ProductionConfig};
    ///
    /// let config = ProductionConfig::default();
    /// let mut ecosystem = ProductionEcosystem::new(config)?;
    /// ecosystem.initialize()?;
    /// # Ok::<(), beardog_errors::BearDogError>(())
    /// ```
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

    /// Initializes all production subsystems and transitions to healthy state
    ///
    /// This method starts all production services in the correct order:
    /// 1. Observability and monitoring
    /// 2. Health checks
    /// 3. Performance optimization
    /// 4. Metrics collection
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all subsystems start successfully
    ///
    /// # Errors
    ///
    /// Returns an error if any subsystem fails to start. The ecosystem
    /// will remain in `Initializing` state on failure.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use beardog_types::production::{ProductionEcosystem, ProductionConfig};
    ///
    /// let config = ProductionConfig::default();
    /// let mut ecosystem = ProductionEcosystem::new(config)?;
    /// ecosystem.initialize()?;
    /// assert_eq!(ecosystem.get_status().status,
    ///            beardog_types::production::OperationalStatus::Healthy);
    /// # Ok::<(), beardog_errors::BearDogError>(())
    /// ```
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

    /// Performs a comprehensive health check of all production subsystems
    ///
    /// Evaluates the health of all components and updates the operational
    /// status accordingly. This should be called periodically to ensure
    /// the system is operating correctly.
    ///
    /// # Returns
    ///
    /// Returns a `HealthReport` with detailed status of all subsystems
    ///
    /// # Errors
    ///
    /// Returns an error if the health check itself fails (not if subsystems are unhealthy)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use beardog_types::production::{ProductionEcosystem, ProductionConfig};
    ///
    /// let config = ProductionConfig::default();
    /// let mut ecosystem = ProductionEcosystem::new(config)?;
    /// ecosystem.initialize()?;
    ///
    /// let health = ecosystem.health_check()?;
    /// println!("System health: {:?}", health.overall_status);
    /// # Ok::<(), beardog_errors::BearDogError>(())
    /// ```
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

    /// Gracefully shuts down all production subsystems
    ///
    /// Stops all services in the reverse order of initialization:
    /// 1. Observability
    /// 2. Health monitoring  
    /// 3. Metrics collection
    ///
    /// This ensures clean resource cleanup and proper shutdown.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all subsystems shut down cleanly
    ///
    /// # Errors
    ///
    /// Returns an error if any subsystem fails to shut down gracefully
    ///
    /// # Examples
    ///
    /// ```rust
    /// use beardog_types::production::{ProductionEcosystem, ProductionConfig};
    ///
    /// let config = ProductionConfig::default();
    /// let mut ecosystem = ProductionEcosystem::new(config)?;
    /// ecosystem.initialize()?;
    ///
    /// // Do work...
    ///
    /// ecosystem.shutdown()?; // Clean shutdown
    /// # Ok::<(), beardog_errors::BearDogError>(())
    /// ```
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
