// SPDX-License-Identifier: AGPL-3.0-or-later

//! Serde-friendly monitoring domain types (metrics, alerts, summaries).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Types of metrics that can be collected
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MetricType {
    /// Counter metric that only increases
    Counter,
    /// Gauge metric that can increase or decrease
    Gauge,
    /// Histogram for latency or size distributions
    Histogram,
    /// Summary metric with quantiles
    Summary,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertSeverity {
    /// Informational alert (no immediate action)
    Info,
    /// Warning alert - attention recommended
    Warning,
    /// Critical alert - immediate action required
    Critical,
    /// Emergency alert - system failure imminent
    Emergency,
}

/// Alert status tracking
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertStatus {
    /// Alert is active and requires attention
    Active,
    /// Alert has been acknowledged by operators
    Acknowledged,
    /// Alert has been resolved
    Resolved,
}

/// System operation status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OperationStatus {
    /// Operation is currently processing
    Processing,
    /// Operation completed successfully
    Completed,
    /// Operation failed with error
    Failed,
    /// Operation timed out
    Timeout,
}

/// Alert thresholds configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    /// CPU utilization threshold (percentage)
    /// The cpu threshold percent value
    pub cpu_threshold_percent: f64,
    /// Memory utilization threshold (percentage)
    /// The memory threshold percent value
    pub memory_threshold_percent: f64,
    /// Disk utilization threshold (percentage)
    /// The disk threshold percent value
    pub disk_threshold_percent: f64,
    /// Network utilization threshold (bytes per second)
    /// Number of `network_threshold_bps`
    pub network_threshold_bps: u64,
    /// Error rate threshold (percentage)
    /// The error rate threshold percent value
    pub error_rate_threshold_percent: f64,
    /// Response time threshold (milliseconds)
    pub response_time_threshold_ms: f64,
}

/// System alert representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// Unique alert identifier
    pub id: Uuid,
    /// Alert severity level
    /// The severity value
    pub severity: AlertSeverity,
    /// Current status of the alert
    /// Current status of the component
    pub status: AlertStatus,
    /// Human-readable alert message
    /// The message value
    pub message: String,
    /// Source component that generated the alert
    /// The source value
    pub source: String,
    /// Timestamp when alert was created
    /// The created at value
    pub created_at: DateTime<Utc>,
    /// Timestamp when alert was last updated
    /// The updated at value
    pub updated_at: DateTime<Utc>,
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

/// System metrics summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSummary {
    /// Overall system health score (0.0 to 1.0)
    /// The health score value
    pub health_score: f64,
    /// Number of active alerts
    /// Number of `active_alerts`
    pub active_alerts_count: u32,
    /// Average CPU utilization over monitoring period
    /// The avg cpu utilization value
    pub avg_cpu_utilization: f64,
    /// Average memory utilization over monitoring period
    /// The avg memory utilization value
    pub avg_memory_utilization: f64,
    /// Average response time over monitoring period
    pub avg_response_time_ms: f64,
    /// Total number of requests processed
    /// Number of `total_requests`
    pub total_requests: u64,
    /// Error rate percentage
    /// The error rate percent value
    pub error_rate_percent: f64,
    /// Timestamp of this summary
    pub timestamp: DateTime<Utc>,
}

/// Aggregated performance summary for trend analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSummary {
    /// Number of `trend_indicator`
    pub trend_indicator: i8,
    /// Bottleneck identification
    /// Optional primary bottleneck
    pub primary_bottleneck: Option<String>,
    /// Collection of recommendations
    pub recommendations: Vec<String>,
    /// Resource utilization efficiency score
    /// The efficiency score value
    pub efficiency_score: f64,
    /// Timestamp of this analysis
    pub timestamp: DateTime<Utc>,
}

/// System overview summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemOverview {
    /// Overall system status
    /// Current status of the component
    pub status: String,
    /// System uptime in seconds
    pub uptime_seconds: u64,
    /// Total number of active services
    /// Number of `active_services`
    pub active_services: u32,
    /// Number of failed services
    /// Number of `failed_services`
    pub failed_services: u32,
    /// Current resource utilization summary
    /// The resource utilization value
    pub resource_utilization: ResourceUtilization,
    /// Timestamp of this overview
    pub timestamp: DateTime<Utc>,
}

/// Resource utilization summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUtilization {
    /// CPU utilization percentage
    /// The cpu percent value
    pub cpu_percent: f64,
    /// Memory utilization percentage
    /// The memory percent value
    pub memory_percent: f64,
    /// Disk utilization percentage
    /// The disk percent value
    pub disk_percent: f64,
    /// Network utilization percentage
    /// The network percent value
    pub network_percent: f64,
}

/// System metrics data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// CPU utilization percentage
    /// The cpu utilization value
    pub cpu_utilization: f64,
    /// Memory utilization percentage
    /// The memory utilization value
    pub memory_utilization: f64,
    /// Disk utilization percentage
    /// The disk utilization value
    pub disk_utilization: f64,
    /// Network utilization in bytes per second
    /// Number of `network_utilization_bps`
    pub network_utilization_bps: u64,
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            cpu_utilization: 0.0,
            memory_utilization: 0.0,
            disk_utilization: 0.0,
            network_utilization_bps: 0,
        }
    }
}

/// Health status enumeration
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    /// Component is healthy and operating normally
    Healthy,
    /// Component is degraded but still functional
    Degraded,
    /// Component is unhealthy and may not function properly
    Unhealthy,
    /// Component health status is unknown
    #[default]
    Unknown,
}
