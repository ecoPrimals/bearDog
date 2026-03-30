// SPDX-License-Identifier: AGPL-3.0-only

//! Production Ecosystem Orchestration
//!
//! Core orchestrator for production deployments, managing the lifecycle of all
//! production subsystems including monitoring, health checks, observability, and
//! performance optimization.
//!
//! # Module Organization
//!
//! - [`ProductionEcosystem`] - Main orchestrator managing all subsystems
//! - [`ProductionState`] - Runtime state snapshot for monitoring
//!
//! # Architecture
//!
//! The `ProductionEcosystem` coordinates multiple subsystems:
//! ```text
//! ProductionEcosystem
//! ├── MetricsCollector (Prometheus, StatsD, custom)
//! ├── HealthChecker (readiness/liveness probes)
//! ├── PerformanceOptimizer (zero-copy, caching)
//! └── ObservabilityEngine (tracing, APM)
//! ```
//!
//! # Lifecycle
//!
//! 1. **Creation**: `ProductionEcosystem::new(config)`
//! 2. **Initialization**: `ecosystem.initialize()` - starts all subsystems
//! 3. **Operation**: Periodic health checks and metric updates
//! 4. **Shutdown**: `ecosystem.shutdown()` - graceful cleanup
//!
//! # Examples
//!
//! ## Basic Usage
//! ```rust,ignore
//! use beardog_types::production::{
//!     ecosystem::ProductionEcosystem,
//!     config::ProductionConfig,
//! };
//!
//! let config = ProductionConfig::default();
//! let mut ecosystem = ProductionEcosystem::new(config)?;
//! ecosystem.initialize()?;
//!
//! // Check health
//! let health = ecosystem.health_check()?;
//! println!("Status: {:?}", health.overall_status);
//!
//! // Get uptime
//! println!("Uptime: {:?}", ecosystem.uptime());
//!
//! // Shutdown
//! ecosystem.shutdown()?;
//! ```

use super::{
    config::ProductionConfig,
    types::{OperationalStatus, PerformanceMetrics},
};
use crate::production::{health, metrics, observability, optimization};
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

// ============================================================================
// PRODUCTION ECOSYSTEM
// ============================================================================

/// Production ecosystem orchestrator for BearDog services
///
/// The main coordinator for production deployments, managing monitoring, health checks,
/// performance optimization, and observability across the entire BearDog ecosystem.
///
/// # Responsibilities
///
/// - **Lifecycle Management**: Initialize and shutdown all subsystems in correct order
/// - **Health Monitoring**: Continuous health checks and status updates
/// - **Metrics Collection**: Aggregate metrics from all subsystems
/// - **Performance Optimization**: Automatic scaling and resource management
/// - **Observability**: Distributed tracing and APM integration
///
/// # Subsystems
///
/// - **Metrics Collector**: Real-time metrics for Prometheus, StatsD, custom exporters
/// - **Health Checker**: Kubernetes readiness/liveness probe implementation
/// - **Performance Optimizer**: Zero-copy operations, caching, resource management
/// - **Observability Engine**: Distributed tracing, logging, APM integration
///
/// # State Management
///
/// The ecosystem maintains runtime state including:
/// - Operational status (Healthy, Degraded, Critical, etc.)
/// - Active connections and request counts
/// - Resource usage (CPU, memory)
/// - Performance metrics (latency, throughput, errors)
///
/// # Examples
///
/// ## Basic Initialization
/// ```rust,ignore
/// use beardog_types::production::{
///     ecosystem::ProductionEcosystem,
///     config::ProductionConfig,
/// };
///
/// let config = ProductionConfig::default();
/// let mut ecosystem = ProductionEcosystem::new(config)?;
/// ecosystem.initialize()?;
/// ```
///
/// ## Health Monitoring
/// ```rust,ignore
/// use beardog_types::production::ecosystem::ProductionEcosystem;
///
/// let mut ecosystem = ProductionEcosystem::new(config)?;
/// ecosystem.initialize()?;
///
/// // Periodic health check
/// tokio::spawn(async move {
///     loop {
///         if let Ok(health) = ecosystem.health_check() {
///             println!("Health: {:?}", health.overall_status);
///         }
///         tokio::time::sleep(Duration::from_secs(30)).await;
///     }
/// });
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
    /// ```rust,ignore
    /// use beardog_types::production::{
    ///     ecosystem::ProductionEcosystem,
    ///     config::ProductionConfig,
    /// };
    ///
    /// let config = ProductionConfig::default();
    /// let ecosystem = ProductionEcosystem::new(config)?;
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
            start_time: Instant::now(),
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
    /// ```rust,ignore
    /// use beardog_types::production::{
    ///     ecosystem::ProductionEcosystem,
    ///     config::ProductionConfig,
    ///     types::OperationalStatus,
    /// };
    ///
    /// let config = ProductionConfig::default();
    /// let mut ecosystem = ProductionEcosystem::new(config)?;
    /// ecosystem.initialize()?;
    /// assert_eq!(ecosystem.get_status().status, OperationalStatus::Healthy);
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

    /// Get current production state snapshot
    ///
    /// Returns a read-only reference to the current runtime state including
    /// operational status, resource usage, and performance metrics.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use beardog_types::production::ecosystem::ProductionEcosystem;
    ///
    /// let ecosystem = ProductionEcosystem::new(config)?;
    /// let state = ecosystem.get_status();
    /// println!("Status: {:?}", state.status);
    /// println!("Active connections: {}", state.active_connections);
    /// ```
    #[must_use]
    pub const fn get_status(&self) -> &ProductionState {
        &self.state
    }

    /// Get system uptime duration
    ///
    /// Returns the time elapsed since the ecosystem was created.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use beardog_types::production::ecosystem::ProductionEcosystem;
    ///
    /// let ecosystem = ProductionEcosystem::new(config)?;
    /// let uptime = ecosystem.uptime();
    /// println!("Uptime: {:?}", uptime);
    /// ```
    #[must_use]
    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Update current metrics from all subsystems
    ///
    /// Collects current metrics and updates the ecosystem state. Should be called
    /// periodically (e.g., every 10-30 seconds) to keep state current.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if metrics are collected successfully
    ///
    /// # Errors
    ///
    /// Returns an error if metric collection fails
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use beardog_types::production::ecosystem::ProductionEcosystem;
    ///
    /// let mut ecosystem = ProductionEcosystem::new(config)?;
    /// ecosystem.initialize()?;
    ///
    /// // Update metrics every 30 seconds
    /// ecosystem.update_metrics()?;
    /// ```
    pub fn update_metrics(&mut self) -> Result<(), BearDogError> {
        // Collect current metrics
        let current_metrics = self.metrics_collector.collect_current_metrics()?;

        // Update state
        self.state.cpu_usage_percent = current_metrics.cpu_usage_percent;

        // Trigger optimization if enabled
        if self.config.core.flags.enable_auto_scaling {
            self.optimizer.evaluate_scaling_needs(&self.state)?;
        }

        Ok(())
    }

    /// Perform comprehensive health check of all subsystems
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
    /// ```rust,ignore
    /// use beardog_types::production::ecosystem::ProductionEcosystem;
    ///
    /// let mut ecosystem = ProductionEcosystem::new(config)?;
    /// ecosystem.initialize()?;
    ///
    /// let health = ecosystem.health_check()?;
    /// println!("System health: {:?}", health.overall_status);
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

    /// Gracefully shut down all production subsystems
    ///
    /// Stops all services in the reverse order of initialization:
    /// 1. Metrics collection
    /// 2. Health monitoring  
    /// 3. Performance optimization
    /// 4. Observability
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
    /// ```rust,ignore
    /// use beardog_types::production::ecosystem::ProductionEcosystem;
    ///
    /// let mut ecosystem = ProductionEcosystem::new(config)?;
    /// ecosystem.initialize()?;
    ///
    /// // Do work...
    ///
    /// ecosystem.shutdown()?; // Clean shutdown
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

// ============================================================================
// PRODUCTION STATE
// ============================================================================

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
/// # Kubernetes Integration
///
/// ```text
/// Readiness Probe:
///   GET /health/ready
///   Returns 200 if status == Healthy
///   Returns 503 otherwise
///
/// Liveness Probe:
///   GET /health/live
///   Returns 200 if status == Healthy || Degraded
///   Returns 503 if status == Critical || Shutdown
/// ```
///
/// # Examples
///
/// ## Basic Usage
/// ```rust
/// use beardog_types::production::{
///     ecosystem::ProductionState,
///     types::OperationalStatus,
/// };
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
/// assert_eq!(state.status, OperationalStatus::Healthy);
/// ```
///
/// ## Monitoring
/// ```rust,no_run
/// use beardog_types::production::ecosystem::ProductionState;
///
/// fn check_state(state: &ProductionState) {
///     if state.cpu_usage_percent > 80.0 {
///         eprintln!("⚠️ High CPU usage: {}%", state.cpu_usage_percent);
///     }
///     
///     if state.error_count_hourly > 100 {
///         eprintln!("⚠️ High error rate: {} errors/hour", state.error_count_hourly);
///     }
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

impl ProductionState {
    /// Check if the system is ready to accept traffic (readiness probe)
    #[must_use]
    pub const fn is_ready(&self) -> bool {
        self.status.is_ready()
    }

    /// Check if the system is alive (liveness probe)
    #[must_use]
    pub const fn is_alive(&self) -> bool {
        self.status.is_alive()
    }

    /// Check if system requires immediate intervention
    #[must_use]
    pub const fn requires_intervention(&self) -> bool {
        self.status.requires_intervention()
    }

    /// Calculate current error rate percentage
    #[must_use]
    #[expect(
        clippy::cast_precision_loss,
        reason = "u64 counts converted to f64 for approximate error-rate percentage"
    )]
    pub fn error_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            (self.error_count_hourly as f64 / self.total_requests as f64) * 100.0
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_production_state_defaults() {
        let state = ProductionState::default();
        assert_eq!(state.status, OperationalStatus::Initializing);
        assert_eq!(state.active_connections, 0);
        assert_eq!(state.total_requests, 0);
    }

    #[test]
    fn test_production_state_is_ready() {
        let mut state = ProductionState::default();
        assert!(!state.is_ready()); // Initializing

        state.status = OperationalStatus::Healthy;
        assert!(state.is_ready());

        state.status = OperationalStatus::Degraded;
        assert!(state.is_ready()); // Degraded but ready

        state.status = OperationalStatus::Critical;
        assert!(!state.is_ready());
    }

    #[test]
    fn test_production_state_is_alive() {
        let mut state = ProductionState::default();
        assert!(state.is_alive()); // Initializing is alive

        state.status = OperationalStatus::Healthy;
        assert!(state.is_alive());

        state.status = OperationalStatus::Critical;
        assert!(!state.is_alive());

        state.status = OperationalStatus::Shutdown;
        assert!(!state.is_alive());
    }

    #[test]
    fn test_production_state_error_rate() {
        let mut state = ProductionState::default();
        state.total_requests = 10000;
        state.error_count_hourly = 50;

        let error_rate = state.error_rate();
        assert!((error_rate - 0.5).abs() < 0.01); // ~0.5% error rate
    }
}
