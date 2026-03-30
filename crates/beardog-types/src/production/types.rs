// SPDX-License-Identifier: AGPL-3.0-only

//! Production Domain Types
//!
//! Core domain types for production environments, including environment classification,
//! operational status, and performance metrics.
//!
//! # Module Organization
//!
//! - [`EnvironmentLevel`] - Environment classification (Development → Production)
//! - [`OperationalStatus`] - Service operational state (Initializing → Critical)
//! - [`PerformanceMetrics`] - Performance measurement and SLA monitoring
//!
//! # Design Philosophy
//!
//! These types are pure data structures with no business logic, making them:
//! - Easy to test in isolation
//! - Serializable for network transport
//! - Composable in larger structures
//! - Clear and self-documenting
//!
//! # Examples
//!
//! ```rust
//! use beardog_types::production::types::{
//!     EnvironmentLevel,
//!     OperationalStatus,
//!     PerformanceMetrics,
//! };
//!
//! // Environment-based configuration
//! let env = EnvironmentLevel::Production;
//! match env {
//!     EnvironmentLevel::Production | EnvironmentLevel::Critical => {
//!         // Enable strict monitoring
//!     }
//!     _ => {}
//! }
//!
//! // Status monitoring
//! let status = OperationalStatus::Healthy;
//! assert_eq!(status, OperationalStatus::Healthy);
//!
//! // Performance metrics
//! let metrics = PerformanceMetrics::default();
//! println!("P95 latency: {}ms", metrics.p95_response_time_ms);
//! ```

use serde::{Deserialize, Serialize};

// ============================================================================
// ENVIRONMENT CLASSIFICATION
// ============================================================================

/// Environment level classification for configuration and behavior tuning
///
/// Defines the deployment environment level, which determines logging verbosity,
/// validation strictness, monitoring depth, and feature availability. This enum
/// enables environment-specific configuration without hardcoding.
///
/// # Environment Characteristics
///
/// - **Development**: Local development, verbose logs, relaxed validation
/// - **Staging**: Production-like with debugging, full logging
/// - **`PreProduction`**: Final validation before production, strict monitoring
/// - **Production**: Optimized for performance and reliability
/// - **Critical**: Maximum reliability, strictest validation, 24/7 monitoring
///
/// # Usage Patterns
///
/// ## Configuration Loading
/// ```rust
/// use beardog_types::production::types::EnvironmentLevel;
///
/// let env = std::env::var("BEARDOG_ENV")
///     .unwrap_or_else(|_| "development".to_string());
///
/// let level = match env.to_lowercase().as_str() {
///     "development" | "dev" => EnvironmentLevel::Development,
///     "staging" | "stage" => EnvironmentLevel::Staging,
///     "preprod" | "pre-production" => EnvironmentLevel::PreProduction,
///     "production" | "prod" => EnvironmentLevel::Production,
///     "critical" | "crit" => EnvironmentLevel::Critical,
///     _ => EnvironmentLevel::Development,
/// };
/// ```
///
/// ## Environment-Specific Behavior
/// ```rust
/// use beardog_types::production::types::EnvironmentLevel;
///
/// fn configure_logging(env: &EnvironmentLevel) {
///     match env {
///         EnvironmentLevel::Development => {
///             // Verbose debug logging, pretty formatting
///         }
///         EnvironmentLevel::Production | EnvironmentLevel::Critical => {
///             // JSON structured logging, warn level, performance optimized
///         }
///         _ => {
///             // Balanced logging for staging/preprod
///         }
///     }
/// }
/// ```
///
/// ## Conditional Feature Enablement
/// ```rust
/// use beardog_types::production::types::EnvironmentLevel;
///
/// let env = EnvironmentLevel::Production;
///
/// match env {
///     EnvironmentLevel::Production | EnvironmentLevel::Critical => {
///         // Enable strict validation, comprehensive monitoring
///     }
///     _ => {}
/// }
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum EnvironmentLevel {
    /// Development environment - verbose logging, relaxed validation
    #[default]
    Development,
    /// Staging environment - production-like with additional debugging
    Staging,
    /// Pre-production environment - final validation before production
    #[serde(rename = "pre-production")]
    PreProduction,
    /// Production environment - optimized for performance and reliability
    Production,
    /// Critical production - highest reliability, strictest validation
    Critical,
}

impl std::fmt::Display for EnvironmentLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Development => write!(f, "Development"),
            Self::Staging => write!(f, "Staging"),
            Self::PreProduction => write!(f, "Pre-Production"),
            Self::Production => write!(f, "Production"),
            Self::Critical => write!(f, "Critical"),
        }
    }
}

impl EnvironmentLevel {
    /// Check if this environment is production-grade (Production or Critical)
    #[must_use]
    pub const fn is_production(&self) -> bool {
        matches!(self, Self::Production | Self::Critical)
    }

    /// Check if this environment allows debugging features
    #[must_use]
    pub const fn allows_debugging(&self) -> bool {
        matches!(
            self,
            Self::Development | Self::Staging | Self::PreProduction
        )
    }

    /// Get recommended log level for this environment
    #[must_use]
    pub const fn log_level(&self) -> &'static str {
        match self {
            Self::Development => "debug",
            Self::Staging | Self::PreProduction => "info",
            Self::Production => "warn",
            Self::Critical => "error",
        }
    }
}

// ============================================================================
// OPERATIONAL STATUS
// ============================================================================

/// Operational status of a production service
///
/// Represents the current health and operational state of a service, from initialization
/// through healthy operation to various failure states. This enum is used for:
/// - Health checks (readiness/liveness probes)
/// - Load balancer decisions
/// - Monitoring dashboards
/// - Alert routing and severity
///
/// # Status Transitions
///
/// ```text
/// Initializing → Healthy ⇄ Degraded → Unhealthy → Critical → Shutdown
///                   ↓
///               Shutdown (graceful)
/// ```
///
/// # Health Check Mapping
///
/// - `Initializing`: Readiness: ❌ Liveness: ✅
/// - `Healthy`: Readiness: ✅ Liveness: ✅
/// - `Degraded`: Readiness: ⚠️  Liveness: ✅ (reduced capacity)
/// - `Unhealthy`: Readiness: ❌ Liveness: ⚠️
/// - `Critical`: Readiness: ❌ Liveness: ❌ (immediate intervention needed)
/// - `Shutdown`: Readiness: ❌ Liveness: ❌ (graceful termination)
///
/// # Alert Severity
///
/// - `Initializing`: Info (expected during startup)
/// - `Healthy`: No alert
/// - `Degraded`: Warning (monitor closely)
/// - `Unhealthy`: Error (investigate immediately)
/// - `Critical`: Critical alert (page immediately, 24/7)
///
/// # Examples
///
/// ```rust
/// use beardog_types::production::types::OperationalStatus;
///
/// let status = OperationalStatus::Healthy;
///
/// match status {
///     OperationalStatus::Healthy => {
///         // All systems operational
///     }
///     OperationalStatus::Degraded => {
///         // Reduced capacity, but functional
///     }
///     OperationalStatus::Unhealthy | OperationalStatus::Critical => {
///         // Alert and investigate
///     }
///     _ => {}
/// }
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum OperationalStatus {
    /// System is starting up and not yet ready to serve traffic
    #[default]
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

impl std::fmt::Display for OperationalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Initializing => write!(f, "Initializing"),
            Self::Healthy => write!(f, "Healthy"),
            Self::Degraded => write!(f, "Degraded"),
            Self::Unhealthy => write!(f, "Unhealthy"),
            Self::Critical => write!(f, "Critical"),
            Self::Shutdown => write!(f, "Shutdown"),
        }
    }
}

impl OperationalStatus {
    /// Check if the service is ready to accept traffic
    #[must_use]
    pub const fn is_ready(&self) -> bool {
        matches!(self, Self::Healthy | Self::Degraded)
    }

    /// Check if the service is alive (not in critical failure)
    #[must_use]
    pub const fn is_alive(&self) -> bool {
        !matches!(self, Self::Critical | Self::Shutdown)
    }

    /// Check if the service requires immediate attention
    #[must_use]
    pub const fn requires_intervention(&self) -> bool {
        matches!(self, Self::Unhealthy | Self::Critical)
    }

    /// Get alert severity level for this status
    #[must_use]
    pub const fn alert_severity(&self) -> &'static str {
        match self {
            Self::Initializing => "info",
            Self::Healthy => "none",
            Self::Degraded => "warning",
            Self::Unhealthy => "error",
            Self::Critical => "critical",
            Self::Shutdown => "info",
        }
    }
}

// ============================================================================
// PERFORMANCE METRICS
// ============================================================================

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
/// ## Basic Usage
/// ```rust
/// use beardog_types::production::types::PerformanceMetrics;
///
/// let metrics = PerformanceMetrics {
///     avg_response_time_ms: 25.4,
///     p95_response_time_ms: 45.8,
///     p99_response_time_ms: 120.5,
///     requests_per_second: 1500.0,
///     error_rate_percent: 0.05, // 0.05% = 99.95% success rate
///     throughput_bytes_per_sec: 10 * 1024 * 1024, // 10 MB/s
/// };
/// ```
///
/// ## SLA Compliance Check
/// ```rust
/// use beardog_types::production::types::PerformanceMetrics;
///
/// fn check_sla(metrics: &PerformanceMetrics) -> bool {
///     metrics.p95_response_time_ms < 100.0
///         && metrics.p99_response_time_ms < 500.0
///         && metrics.error_rate_percent < 0.1
/// }
/// ```
///
/// ## Capacity Planning
/// ```rust
/// use beardog_types::production::types::PerformanceMetrics;
///
/// fn calculate_daily_volume(metrics: &PerformanceMetrics) -> f64 {
///     metrics.requests_per_second * 86400.0
/// }
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

impl PerformanceMetrics {
    /// Check if metrics meet typical production SLA requirements
    #[must_use]
    pub fn meets_sla(&self) -> bool {
        self.p95_response_time_ms < 100.0
            && self.p99_response_time_ms < 500.0
            && self.error_rate_percent < 0.1
    }

    /// Calculate success rate percentage from error rate
    #[must_use]
    pub fn success_rate(&self) -> f64 {
        100.0 - self.error_rate_percent
    }

    /// Calculate estimated daily request volume
    #[must_use]
    pub fn daily_request_volume(&self) -> f64 {
        self.requests_per_second * 86_400.0
    }

    /// Calculate estimated monthly request volume
    #[must_use]
    pub fn monthly_request_volume(&self) -> f64 {
        self.daily_request_volume() * 30.0
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_level_defaults() {
        assert_eq!(EnvironmentLevel::default(), EnvironmentLevel::Development);
    }

    #[test]
    fn test_environment_level_is_production() {
        assert!(EnvironmentLevel::Production.is_production());
        assert!(EnvironmentLevel::Critical.is_production());
        assert!(!EnvironmentLevel::Development.is_production());
        assert!(!EnvironmentLevel::Staging.is_production());
    }

    #[test]
    fn test_environment_level_log_level() {
        assert_eq!(EnvironmentLevel::Development.log_level(), "debug");
        assert_eq!(EnvironmentLevel::Production.log_level(), "warn");
        assert_eq!(EnvironmentLevel::Critical.log_level(), "error");
    }

    #[test]
    fn test_operational_status_defaults() {
        assert_eq!(
            OperationalStatus::default(),
            OperationalStatus::Initializing
        );
    }

    #[test]
    fn test_operational_status_is_ready() {
        assert!(OperationalStatus::Healthy.is_ready());
        assert!(OperationalStatus::Degraded.is_ready());
        assert!(!OperationalStatus::Unhealthy.is_ready());
    }

    #[test]
    fn test_operational_status_requires_intervention() {
        assert!(OperationalStatus::Unhealthy.requires_intervention());
        assert!(OperationalStatus::Critical.requires_intervention());
        assert!(!OperationalStatus::Healthy.requires_intervention());
    }

    #[test]
    fn test_performance_metrics_defaults() {
        let metrics = PerformanceMetrics::default();
        assert_eq!(metrics.avg_response_time_ms, 0.0);
        assert_eq!(metrics.requests_per_second, 0.0);
    }

    #[test]
    fn test_performance_metrics_sla() {
        let good_metrics = PerformanceMetrics {
            avg_response_time_ms: 25.0,
            p95_response_time_ms: 50.0,
            p99_response_time_ms: 200.0,
            requests_per_second: 1000.0,
            error_rate_percent: 0.05,
            throughput_bytes_per_sec: 1_000_000,
        };
        assert!(good_metrics.meets_sla());

        let bad_metrics = PerformanceMetrics {
            p95_response_time_ms: 150.0, // Exceeds 100ms SLA
            ..good_metrics
        };
        assert!(!bad_metrics.meets_sla());
    }

    #[test]
    fn test_performance_metrics_volume_calculations() {
        let metrics = PerformanceMetrics {
            requests_per_second: 100.0,
            ..Default::default()
        };

        assert_eq!(metrics.daily_request_volume(), 8_640_000.0);
        assert_eq!(metrics.monthly_request_volume(), 259_200_000.0);
    }
}
