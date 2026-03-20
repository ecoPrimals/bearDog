// SPDX-License-Identifier: AGPL-3.0-only

//! Runtime HSM health snapshots, error rollups, and operation results.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// HSM operational status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmStatus {
    /// Current health status
    /// The health value
    pub health: HsmHealthStatus,
    /// Live throughput/latency counters for dashboards and autoscale hints.
    pub performance: PerformanceMetrics,
    /// Rolling error telemetry (recent messages, rates, last failure time).
    pub errors: ErrorInfo,
    /// Thresholds and probe list used by the health checker for this HSM.
    pub config: HsmHealthCheckConfig,
    /// Last status update
    /// The last updated value
    pub last_updated: SystemTime,
    /// Status metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

impl Default for HsmStatus {
    fn default() -> Self {
        Self {
            health: HsmHealthStatus::Healthy,
            performance: PerformanceMetrics::default(),
            errors: ErrorInfo::default(),
            config: HsmHealthCheckConfig::default(),
            last_updated: SystemTime::now(),
            metadata: HashMap::new(),
        }
    }
}

/// HSM health status enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum HsmHealthStatus {
    /// HSM is healthy and operational
    Healthy,
    /// HSM is degraded but functional
    Degraded,
    /// HSM is unhealthy
    Unhealthy,
    /// HSM status is unknown
    #[default]
    Unknown,
}

/// Rolling performance counters for a single HSM instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Operations per second
    /// The operations per second value
    pub operations_per_second: f64,
    /// Average latency in milliseconds
    /// The average latency ms value
    pub average_latency_ms: f64,
    /// Success rate percentage
    /// The success rate value
    pub success_rate: f64,
    /// Memory usage in MB
    /// The memory usage mb value
    pub memory_usage_mb: f64,
    /// CPU usage percentage
    /// The cpu usage percent value
    pub cpu_usage_percent: f64,
    /// Error count
    /// Number of error
    pub error_count: u64,
    /// Uptime in seconds
    pub uptime_seconds: u64,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            operations_per_second: 0.0,
            average_latency_ms: 0.0,
            success_rate: 100.0,
            memory_usage_mb: 0.0,
            cpu_usage_percent: 0.0,
            error_count: 0,
            uptime_seconds: 0,
        }
    }
}

/// Aggregated error telemetry (recent strings + rates) for an HSM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    /// Total error count
    /// Number of `total_errors`
    pub total_errors: u64,
    /// Recent errors
    /// Collection of recent errors
    pub recent_errors: Vec<String>,
    /// Error rate per hour
    /// The error rate per hour value
    pub error_rate_per_hour: f64,
    /// Last error timestamp
    /// Optional last error
    pub last_error: Option<SystemTime>,
}

impl Default for ErrorInfo {
    fn default() -> Self {
        Self {
            total_errors: 0,
            recent_errors: Vec::new(),
            error_rate_per_hour: 0.0,
            last_error: None,
        }
    }
}

/// HSM operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmOperationResult<T> {
    /// Operation success status
    /// Whether success is enabled
    pub success: bool,
    /// Result data
    /// Optional data
    pub data: Option<T>,
    /// Error message if failed
    /// Optional error
    pub error: Option<String>,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Operation metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

impl<T> Default for HsmOperationResult<T> {
    fn default() -> Self {
        Self {
            success: false,
            data: None,
            error: None,
            processing_time_ms: 0,
            metadata: HashMap::new(),
        }
    }
}

/// HSM health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmHealthCheckConfig {
    /// Check interval in seconds
    /// Number of `interval_seconds`
    pub interval_seconds: u64,
    /// Health thresholds
    /// The thresholds value
    pub thresholds: HealthThresholds,
    /// Enabled checks
    /// Whether `feature_checks` is enabled
    pub enabled_checks: Vec<String>,
}

impl Default for HsmHealthCheckConfig {
    fn default() -> Self {
        Self {
            interval_seconds: 60,
            thresholds: HealthThresholds::default(),
            enabled_checks: vec![
                "connectivity".to_string(),
                "performance".to_string(),
                "errors".to_string(),
            ],
        }
    }
}

/// Health check thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthThresholds {
    /// Maximum acceptable latency in ms
    /// The max latency ms value
    pub max_latency_ms: f64,
    /// Minimum success rate percentage
    /// The min success rate value
    pub min_success_rate: f64,
    /// Maximum error rate per hour
    /// The max error rate value
    pub max_error_rate: f64,
}

impl Default for HealthThresholds {
    fn default() -> Self {
        Self {
            max_latency_ms: 1000.0,
            min_success_rate: 95.0,
            max_error_rate: 10.0,
        }
    }
}

impl HsmStatus {
    /// Returns a fresh [`Default`] snapshot with [`SystemTime::now`] as [`Self::last_updated`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// True when [`HsmHealthStatus::Healthy`] (not merely unknown or degraded).
    #[must_use]
    pub const fn is_healthy(&self) -> bool {
        matches!(self.health, HsmHealthStatus::Healthy)
    }

    /// Sets [`Self::health`] and refreshes [`Self::last_updated`].
    pub fn update_health(&mut self, status: HsmHealthStatus) {
        self.health = status;
        self.last_updated = SystemTime::now();
    }

    /// Appends an error string, increments counters, and trims history to the last ten entries.
    pub fn record_error(&mut self, error: impl Into<String>) {
        self.errors.total_errors += 1;
        self.errors.recent_errors.push(error.into());
        self.errors.last_error = Some(SystemTime::now());

        // Keep only last 10 errors
        if self.errors.recent_errors.len() > 10 {
            self.errors.recent_errors.remove(0);
        }
    }

    /// Replaces [`Self::performance`] and bumps [`Self::last_updated`].
    pub fn update_performance(&mut self, metrics: PerformanceMetrics) {
        self.performance = metrics;
        self.last_updated = SystemTime::now();
    }

    /// Get status summary
    #[must_use]
    pub fn summary(&self) -> String {
        format!(
            "HSM Status: {:?}, Ops/sec: {:.2}, Latency: {:.2}ms, Success: {:.1}%",
            self.health,
            self.performance.operations_per_second,
            self.performance.average_latency_ms,
            self.performance.success_rate
        )
    }
}

impl<T> HsmOperationResult<T> {
    /// Create a successful result
    pub fn success(result: T) -> Self {
        Self {
            success: true,
            data: Some(result),
            error: None,
            processing_time_ms: 0,
            metadata: HashMap::new(),
        }
    }

    /// Create a failed result
    #[must_use]
    pub fn failure(error: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error.into()),
            processing_time_ms: 0,
            metadata: HashMap::new(),
        }
    }

    /// Set processing time
    #[must_use]
    /// Creates instance with processing time
    pub const fn with_processing_time(mut self, time_ms: u64) -> Self {
        self.processing_time_ms = time_ms;
        self
    }

    /// Add metadata
    #[must_use]
    /// Creates instance with metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Fine-grained vitals collected during deep health probes (fan, thermals, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMetrics {
    /// Overall health score (0-100)
    /// Number of `health_score`
    pub health_score: u8,
    /// CPU utilization percentage
    /// The cpu usage value
    pub cpu_usage: f32,
    /// Memory utilization percentage
    /// The memory usage value
    pub memory_usage: f32,
    /// Temperature in Celsius
    /// The temperature value
    pub temperature: f32,
    /// Number of active connections
    /// Number of `active_connections`
    pub active_connections: u32,
    /// Operations per second
    /// Number of `operations_per_second`
    pub operations_per_second: u32,
    /// Error rate percentage
    /// The error rate value
    pub error_rate: f32,
    /// Response time in milliseconds
    pub avg_response_time: u32,
    /// Last health check timestamp
    /// The last check value
    pub last_check: SystemTime,
}

impl Default for HealthMetrics {
    fn default() -> Self {
        Self {
            health_score: 100,
            cpu_usage: 0.0,
            memory_usage: 0.0,
            temperature: 25.0,
            active_connections: 0,
            operations_per_second: 0,
            error_rate: 0.0,
            avg_response_time: 0,
            last_check: SystemTime::now(),
        }
    }
}

/// Composite view combining [`HsmHealthStatus`], [`HealthMetrics`], and per-component breakdowns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmHealth {
    /// Overall health status
    /// Current status of the component
    pub status: HsmHealthStatus,
    /// Detailed health metrics
    /// The metrics value
    pub metrics: HealthMetrics,
    /// Health check configuration
    pub config: HsmHealthCheckConfig,
    /// Component health status
    /// Mapping of component health
    pub component_health: HashMap<String, HsmHealthStatus>,
    /// Recent health events
    /// Collection of recent events
    pub recent_events: Vec<String>,
    /// Health trends over time
    /// Mapping of trends
    pub trends: HashMap<String, Vec<f32>>,
}

impl Default for HsmHealth {
    fn default() -> Self {
        Self {
            status: HsmHealthStatus::Healthy,
            metrics: HealthMetrics::default(),
            config: HsmHealthCheckConfig::default(),
            component_health: HashMap::new(),
            recent_events: Vec::new(),
            trends: HashMap::new(),
        }
    }
}
