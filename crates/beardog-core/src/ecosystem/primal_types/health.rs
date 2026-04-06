// SPDX-License-Identifier: AGPL-3.0-or-later

//! Health and metrics types for primal services.

use beardog_types::canonical::HealthStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Primal health status
///
/// Comprehensive health information for a primal service, including
/// overall status, detailed health check results, and timing information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalHealth {
    /// Current health status
    pub status: HealthStatus,
    /// Timestamp of last health check
    pub last_check: DateTime<Utc>,
    /// Detailed health information for debugging
    pub details: HashMap<String, serde_json::Value>,
    /// Results of individual health checks
    pub checks: HashMap<String, bool>,
    /// Timestamp when this health report was generated
    pub timestamp: DateTime<Utc>,
}

impl Default for PrimalHealth {
    fn default() -> Self {
        Self {
            status: HealthStatus::Healthy,
            last_check: Utc::now(),
            details: HashMap::new(),
            checks: HashMap::new(),
            timestamp: Utc::now(),
        }
    }
}

/// Key operation status
///
/// Status information for cryptographic key operations, tracking
/// health, tested operations, and performance metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyOperationStatus {
    /// Whether key operations are healthy
    pub healthy: bool,
    /// List of operations that were tested
    pub operations_tested: Vec<String>,
    /// Response time metrics for operations
    pub response_times: serde_json::Value,
}

/// Endpoint health status
///
/// Health information for a specific service endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointHealth {
    /// Name of the endpoint
    pub name: String,
    /// Current health status of the endpoint
    pub status: String,
}

/// Response time metrics
///
/// Statistical metrics for measuring response time performance,
/// including average and percentile measurements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseTimeMetrics {
    /// Average response time in milliseconds
    pub average: f64,
    /// 95th percentile response time
    pub p95: f64,
    /// 99th percentile response time
    pub p99: f64,
}

/// Health status information for a primal service
///
/// Tracks overall health and component-level health for comprehensive monitoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalHealthStatus {
    /// Overall health status of the primal
    pub status: HealthStatus,
    /// Health status of individual components
    pub components: HashMap<String, HealthStatus>,
    /// Timestamp of last health check
    pub last_check: DateTime<Utc>,
    /// Scheduled timestamp for next health check
    pub next_check: DateTime<Utc>,
}

/// Resource usage information for monitoring
///
/// Tracks CPU, memory, network, and disk utilization metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsageInfo {
    /// CPU utilization as percentage (0.0-100.0)
    pub cpu_percent: f64,
    /// Memory usage in bytes
    pub memory_bytes: u64,
    /// Network throughput in bytes per second
    pub network_bytes_per_sec: u64,
    /// Disk I/O throughput in bytes per second
    pub disk_bytes_per_sec: u64,
}

/// Load and capacity metrics for services
///
/// Tracks resource utilization and capacity for load balancing decisions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadMetrics {
    /// Current CPU usage percentage
    pub cpu_usage: f64,
    /// The memory usage value
    pub memory_usage: f64,
    /// Number of `active_connections`
    pub active_connections: u32,
    /// The requests per second value
    pub requests_per_second: f64,
}

/// Error rate metrics for service monitoring
///
/// Tracks error rates, timeout rates, and categorizes failures for analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorRateMetrics {
    /// Overall error rate as percentage (0.0-100.0)
    pub error_rate: f64,
    /// Timeout rate as percentage (0.0-100.0)
    pub timeout_rate: f64,
    /// Count of failures by category
    #[serde(default)]
    pub failure_categories: HashMap<String, u32>,
}

/// Performance and health metrics for primal services
///
/// Tracks key operational metrics including response times, availability,
/// load/capacity, and error rates for monitoring and optimization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrimalMetrics {
    /// Response time percentiles (p50, p95, p99)
    pub response_times: ResponseTimeMetrics,
    /// Service availability percentage (0.0-100.0)
    pub availability: f64,
    /// Current load and capacity utilization
    pub load_metrics: LoadMetrics,
    /// Error rates across different categories
    pub error_rates: ErrorRateMetrics,
}
