// SPDX-License-Identifier: AGPL-3.0-only

//! Production Configuration
//!
//! Configuration structures for production deployments, including core settings,
//! feature flags, and modular subsystem configurations.
//!
//! # Module Organization
//!
//! - [`ProductionConfig`] - Top-level configuration aggregating all subsystems
//! - [`ProductionCoreConfig`] - Core service identity and deployment settings
//! - [`ProductionFlags`] - Feature flags for runtime capability control
//!
//! # Design Philosophy
//!
//! - **Modular**: Each subsystem has independent configuration
//! - **Environment-Aware**: Adapts behavior based on deployment environment
//! - **Safe Defaults**: All settings have production-ready defaults
//! - **Override-Friendly**: Support environment variables and config files
//!
//! # Examples
//!
//! ## Basic Configuration
//! ```rust,ignore
//! use beardog_types::production::config::ProductionConfig;
//!
//! let config = ProductionConfig::default();
//! println!("Service: {}", config.core.service_name);
//! ```
//!
//! ## Environment-Specific Configuration
//! ```rust,ignore
//! use beardog_types::production::{
//!     config::{ProductionConfig, ProductionCoreConfig},
//!     types::EnvironmentLevel,
//! };
//!
//! let config = ProductionConfig {
//!     core: ProductionCoreConfig {
//!         environment_level: EnvironmentLevel::Production,
//!         service_name: "beardog-api".to_string(),
//!         ..Default::default()
//!     },
//!     ..Default::default()
//! };
//! ```

use super::types::EnvironmentLevel;
use crate::production::{health, metrics, monitoring, observability, optimization, telemetry};
use serde::{Deserialize, Serialize};

// ============================================================================
// TOP-LEVEL CONFIGURATION
// ============================================================================

/// Production-grade ecosystem configuration for BearDog
///
/// This is the primary configuration struct for production deployments, providing
/// comprehensive control over monitoring, observability, health checks, metrics,
/// and performance optimization.
///
/// # Architecture
///
/// The configuration is modular, with each domain (monitoring, observability, etc.)
/// having its own sub-configuration that can be customized independently:
///
/// ```text
/// ProductionConfig
/// ├── core: ProductionCoreConfig (service identity, environment)
/// ├── monitoring: MonitoringConfig (metrics, alerts, dashboards)
/// ├── observability: ObservabilityConfig (tracing, logging)
/// ├── optimization: OptimizationConfig (caching, zero-copy)
/// ├── health: HealthConfig (readiness, liveness)
/// ├── metrics: MetricsConfig (Prometheus, StatsD)
/// └── telemetry: TelemetryConfig (distributed tracing, APM)
/// ```
///
/// # Configuration Sources
///
/// Configuration can be loaded from multiple sources (in priority order):
/// 1. Environment variables (`BEARDOG_*`)
/// 2. Configuration file (TOML/JSON)
/// 3. Defaults (production-ready)
///
/// # Environment Variables
///
/// Common environment variable overrides:
/// - `BEARDOG_ENV` - Environment level (development, staging, production)
/// - `BEARDOG_SERVICE_NAME` - Service identifier
/// - `BEARDOG_REGION` - Cloud region/availability zone
/// - `BEARDOG_METRICS_ENABLED` - Enable/disable metrics collection
///
/// # Examples
///
/// ## Basic Production Configuration
/// ```rust,ignore
/// use beardog_types::production::config::ProductionConfig;
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
/// ```rust,ignore
/// use beardog_types::production::{
///     config::ProductionConfig,
///     types::EnvironmentLevel,
/// };
///
/// let mut config = ProductionConfig::default();
/// config.core.environment_level = EnvironmentLevel::Production;
///
/// // Production-specific tuning
/// if config.core.environment_level.is_production() {
///     config.monitoring.enabled = true;
///     config.health.check_interval_secs = 30;
///     config.metrics.prometheus_enabled = true;
/// }
/// ```
///
/// # See Also
///
/// - [`ProductionCoreConfig`] - Core production settings
/// - [`ProductionFlags`] - Feature flags
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

// ============================================================================
// CORE CONFIGURATION
// ============================================================================

/// Core production configuration settings
///
/// Contains essential identity and deployment information for production services.
/// These settings are typically set once during service initialization and rarely
/// change during runtime.
///
/// # Fields
///
/// - **environment_level**: Deployment environment (Dev, Staging, Production, Critical)
/// - **service_name**: Unique service identifier within the ecosystem
/// - **service_version**: Semantic version for deployment tracking
/// - **deployment_id**: Unique deployment instance identifier
/// - **region**: Cloud region or availability zone
/// - **cluster_id**: Orchestration cluster identifier (K8s, ECS, etc.)
/// - **node_id**: Node/host identifier (hostname or instance ID)
/// - **flags**: Feature flags for runtime capability control
///
/// # Examples
///
/// ## Basic Usage
/// ```rust,ignore
/// use beardog_types::production::{
///     config::ProductionCoreConfig,
///     types::EnvironmentLevel,
/// };
///
/// let core = ProductionCoreConfig {
///     environment_level: EnvironmentLevel::Production,
///     service_name: "beardog-security".to_string(),
///     service_version: env!("CARGO_PKG_VERSION").to_string(),
///     region: "us-west-2".to_string(),
///     ..Default::default()
/// };
/// ```
///
/// ## With Environment Variables
/// ```rust,ignore
/// use beardog_types::production::config::ProductionCoreConfig;
///
/// let core = ProductionCoreConfig {
///     service_name: std::env::var("SERVICE_NAME")
///         .unwrap_or_else(|_| "beardog".to_string()),
///     deployment_id: std::env::var("DEPLOYMENT_ID")
///         .unwrap_or_else(|_| uuid::Uuid::new_v4().to_string()),
///     region: std::env::var("AWS_REGION")
///         .unwrap_or_else(|_| "local".to_string()),
///     ..Default::default()
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

impl Default for ProductionCoreConfig {
    fn default() -> Self {
        Self {
            environment_level: EnvironmentLevel::Development,
            service_name: "beardog-ecosystem".to_string(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            deployment_id: uuid::Uuid::new_v4().to_string(),
            region: "local".to_string(),
            cluster_id: "default-cluster".to_string(),
            node_id: "unknown".to_string(),
            flags: ProductionFlags::default(),
        }
    }
}

impl ProductionCoreConfig {
    /// Load core identity fields from environment variables (e.g. `HOSTNAME` for `node_id`).
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            node_id: std::env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string()),
            ..Default::default()
        }
    }
}

// ============================================================================
// FEATURE FLAGS
// ============================================================================

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
/// - **Composable**: Mix and match flags for different deployment scenarios
///
/// # Flag Descriptions
///
/// - **advanced_monitoring**: Detailed metrics collection and alerting
/// - **distributed_tracing**: Request flow visualization (OpenTelemetry, Jaeger)
/// - **performance_profiling**: CPU/memory profiling and flame graphs (dev/staging only)
/// - **security_auditing**: Comprehensive security event logging
/// - **auto_scaling**: Horizontal scaling based on load metrics
/// - **circuit_breakers**: Fault tolerance and cascading failure prevention
/// - **rate_limiting**: Request throttling for abuse prevention
/// - **caching**: Response caching for improved performance
///
/// # Examples
///
/// ## Production Configuration
/// ```rust
/// use beardog_types::production::config::ProductionFlags;
///
/// // Production: All safety features, no profiling
/// let production_flags = ProductionFlags {
///     enable_advanced_monitoring: true,
///     enable_distributed_tracing: true,
///     enable_performance_profiling: false, // Performance overhead
///     enable_security_auditing: true,
///     enable_auto_scaling: true,
///     enable_circuit_breakers: true,
///     enable_rate_limiting: true,
///     enable_caching: true,
/// };
/// ```
///
/// ## Development Configuration
/// ```rust
/// use beardog_types::production::config::ProductionFlags;
///
/// // Development: Debugging tools, minimal overhead
/// let dev_flags = ProductionFlags {
///     enable_advanced_monitoring: false,
///     enable_distributed_tracing: true, // Useful for debugging
///     enable_performance_profiling: true, // Find bottlenecks
///     enable_security_auditing: false,
///     enable_auto_scaling: false,
///     enable_circuit_breakers: false,
///     enable_rate_limiting: false,
///     enable_caching: false, // Avoid stale data during development
/// };
/// ```
///
/// ## Environment-Based Configuration
/// ```rust,ignore
/// use beardog_types::production::{
///     config::ProductionFlags,
///     types::EnvironmentLevel,
/// };
///
/// fn flags_for_environment(env: EnvironmentLevel) -> ProductionFlags {
///     match env {
///         EnvironmentLevel::Production | EnvironmentLevel::Critical => {
///             ProductionFlags::production_defaults()
///         }
///         EnvironmentLevel::Development => {
///             ProductionFlags::development_defaults()
///         }
///         _ => ProductionFlags::default(),
///     }
/// }
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

impl ProductionFlags {
    /// Get production-optimized flags (strict, performance-focused)
    #[must_use]
    pub const fn production_defaults() -> Self {
        Self {
            enable_advanced_monitoring: true,
            enable_distributed_tracing: true,
            enable_performance_profiling: false, // Overhead in production
            enable_security_auditing: true,
            enable_auto_scaling: true,
            enable_circuit_breakers: true,
            enable_rate_limiting: true,
            enable_caching: true,
        }
    }

    /// Get development-optimized flags (debugging tools, minimal features)
    #[must_use]
    pub const fn development_defaults() -> Self {
        Self {
            enable_advanced_monitoring: false,
            enable_distributed_tracing: true, // Useful for debugging
            enable_performance_profiling: true, // Find bottlenecks
            enable_security_auditing: false,
            enable_auto_scaling: false,
            enable_circuit_breakers: false,
            enable_rate_limiting: false,
            enable_caching: false, // Avoid stale data
        }
    }

    /// Get staging-optimized flags (production-like with debugging)
    #[must_use]
    pub const fn staging_defaults() -> Self {
        Self {
            enable_advanced_monitoring: true,
            enable_distributed_tracing: true,
            enable_performance_profiling: true, // Profile before production
            enable_security_auditing: true,
            enable_auto_scaling: false, // Manual scaling in staging
            enable_circuit_breakers: true,
            enable_rate_limiting: true,
            enable_caching: true,
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
    fn test_production_config_defaults() {
        let config = ProductionConfig::default();
        assert_eq!(config.core.environment_level, EnvironmentLevel::Development);
        assert_eq!(config.core.service_name, "beardog-ecosystem");
    }

    #[test]
    fn test_production_core_config_defaults() {
        let core = ProductionCoreConfig::default();
        assert_eq!(core.environment_level, EnvironmentLevel::Development);
        assert_eq!(core.service_name, "beardog-ecosystem");
        assert_eq!(core.region, "local");
        assert_eq!(core.cluster_id, "default-cluster");
    }

    #[test]
    fn test_production_flags_defaults() {
        let flags = ProductionFlags::default();
        assert!(flags.enable_advanced_monitoring);
        assert!(flags.enable_distributed_tracing);
    }

    #[test]
    fn test_production_flags_production_defaults() {
        let flags = ProductionFlags::production_defaults();
        assert!(flags.enable_security_auditing);
        assert!(!flags.enable_performance_profiling); // Disabled in production
        assert!(flags.enable_auto_scaling);
    }

    #[test]
    fn test_production_flags_development_defaults() {
        let flags = ProductionFlags::development_defaults();
        assert!(!flags.enable_advanced_monitoring);
        assert!(flags.enable_performance_profiling); // Enabled for debugging
        assert!(!flags.enable_auto_scaling);
        assert!(!flags.enable_caching); // Avoid stale data
    }

    #[test]
    fn test_production_flags_staging_defaults() {
        let flags = ProductionFlags::staging_defaults();
        assert!(flags.enable_advanced_monitoring);
        assert!(flags.enable_performance_profiling); // Profile before prod
        assert!(!flags.enable_auto_scaling); // Manual scaling
    }
}
