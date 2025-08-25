// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Canonical Monitoring Types
///
/// **SINGLE SOURCE OF TRUTH** for all monitoring-related types in the `BearDog` ecosystem.
/// This module provides unified health check, monitoring, and observability types.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// **HEALTH CHECK CONFIG** - Configuration for health checks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Health check interval
    pub interval: Duration,
    /// Health check timeout
    pub timeout: Duration,
    /// Number of retries before marking unhealthy
    pub retries: u32,
    /// Threshold for degraded status
    pub degraded_threshold: f64,
    /// Threshold for unhealthy status
    pub unhealthy_threshold: f64,
    /// Custom health check endpoints
    pub endpoints: Vec<String>,
    /// Health check metadata
    pub metadata: HashMap<String, String>,
}

/// **SERVICE HEALTH MONITOR** - Service health monitoring system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealthMonitor {
    /// Monitor identifier
    pub monitor_id: String,
    /// Monitored service identifier
    pub service_id: String,
    /// Health check configuration
    pub config: HealthCheckConfig,
    /// Current health status
    pub current_status: HealthStatus,
    /// Health check history
    pub health_history: Vec<HealthCheckResult>,
    /// Last check timestamp
    pub last_check: DateTime<Utc>,
    /// Monitor metadata
    pub metadata: HashMap<String, String>,
}

/// **HEALTH STATUS** - Overall health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    /// Service is healthy
    Healthy,
    /// Service is degraded
    Degraded,
    /// Service is unhealthy
    Unhealthy,
    /// Health status unknown
    Unknown,
}

/// **HEALTH CHECK RESULT** - Result of a health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    /// Check timestamp
    pub timestamp: DateTime<Utc>,
    /// Check status
    pub status: HealthStatus,
    /// Response time in milliseconds
    pub response_time_ms: u64,
    /// Check details
    pub details: HashMap<String, serde_json::Value>,
    /// Error message (if any)
    pub error: Option<String>,
}

/// **MONITORING METRICS** - System monitoring metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringMetrics {
    /// Metric timestamp
    pub timestamp: DateTime<Utc>,
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage percentage
    pub memory_usage: f64,
    /// Disk usage percentage
    pub disk_usage: f64,
    /// Network usage metrics
    pub network_usage: NetworkUsage,
    /// Request metrics
    pub request_metrics: RequestMetrics,
    /// Custom metrics
    pub custom_metrics: HashMap<String, f64>,
}

/// **NETWORK USAGE** - Network usage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkUsage {
    /// Bytes received per second
    pub bytes_in_per_sec: f64,
    /// Bytes sent per second
    pub bytes_out_per_sec: f64,
    /// Packets received per second
    pub packets_in_per_sec: f64,
    /// Packets sent per second
    pub packets_out_per_sec: f64,
}

/// **REQUEST METRICS** - Request processing metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestMetrics {
    /// Total requests processed
    pub total_requests: u64,
    /// Successful requests
    pub successful_requests: u64,
    /// Failed requests
    pub failed_requests: u64,
    /// Average response time
    pub avg_response_time_ms: f64,
    /// Requests per second
    pub requests_per_second: f64,
}

/// **ALERT CONFIG** - Alert configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    /// Alert name
    pub name: String,
    /// Alert condition
    pub condition: String,
    /// Alert threshold
    pub threshold: f64,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert enabled
    pub enabled: bool,
}

/// **ALERT SEVERITY** - Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertSeverity {
    /// Low severity alert
    Low,
    /// Medium severity alert
    Medium,
    /// High severity alert
    High,
    /// Critical severity alert
    Critical,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
            retries: 3,
            degraded_threshold: 0.8,
            unhealthy_threshold: 0.5,
            endpoints: vec!["/health".to_string()],
            metadata: HashMap::new(),
        }
    }
}

impl Default for HealthStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Default for AlertSeverity {
    fn default() -> Self {
        Self::Medium
    }
}
