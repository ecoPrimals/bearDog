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


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **CANONICAL MONITORING TYPES** - Unified type definitions for monitoring system
/// This module provides all the core types used throughout the BearDog monitoring
/// ecosystem with consistent serialization and validation.

/// Component health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    /// Name of the component being monitored
    pub name: String,
    /// Current health status
    pub status: beardog_types::canonical::health_status::HealthStatus,
    /// Optional health message
    pub message: Option<String>,
    /// When the health check was last performed
    pub last_check: DateTime<Utc>,
    /// Duration of the health check in milliseconds
    pub check_duration_ms: u64,
    /// Additional metadata about the component
    pub metadata: HashMap<String, String>,
}

/// System-wide health aggregation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    /// Overall system status
    pub status: beardog_types::canonical::health_status::HealthStatus,
    /// List of individual component health statuses
    pub components: Vec<ComponentHealth>,
    /// When the system health was last evaluated
    pub last_check: DateTime<Utc>,
    /// Summary message about system health
    pub summary: String,
}

/// Resource utilization metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    /// CPU usage percentage (0.0 to 100.0)
    pub cpu_usage_percent: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// Total available memory in bytes
    pub memory_total_bytes: u64,
    /// Disk usage in bytes
    pub disk_usage_bytes: u64,
    /// Total available disk space in bytes
    pub disk_total_bytes: u64,
    /// Network bytes sent
    pub network_bytes_sent: u64,
    /// Network bytes received
    pub network_bytes_received: u64,
    /// Number of active network connections
    pub active_connections: u32,
    /// System uptime in seconds
    pub uptime_seconds: u64,
}

impl Default for ResourceMetrics {
    fn default() -> Self {
        Self {
            cpu_usage_percent: 0.0,
            memory_usage_bytes: 0,
            memory_total_bytes: 0,
            disk_usage_bytes: 0,
            disk_total_bytes: 0,
            network_bytes_sent: 0,
            network_bytes_received: 0,
            active_connections: 0,
            uptime_seconds: 0,
        }
    }
}

/// Performance metrics for application monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Average request processing time in milliseconds
    pub avg_request_time_ms: f64,
    /// Number of requests processed per second
    pub requests_per_second: f64,
    /// Total number of requests processed
    pub total_requests: u64,
    /// Number of failed requests
    pub failed_requests: u64,
    /// Error rate as a percentage (0.0 to 100.0)
    pub error_rate_percent: f64,
    /// 95th percentile response time in milliseconds
    pub p95_response_time_ms: f64,
    /// 99th percentile response time in milliseconds
    pub p99_response_time_ms: f64,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            avg_request_time_ms: 0.0,
            requests_per_second: 0.0,
            total_requests: 0,
            failed_requests: 0,
            error_rate_percent: 0.0,
            p95_response_time_ms: 0.0,
            p99_response_time_ms: 0.0,
        }
    }
}

/// Comprehensive system metrics collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// When these metrics were collected
    pub timestamp: DateTime<Utc>,
    /// Resource utilization metrics
    pub resources: ResourceMetrics,
    /// Application performance metrics
    pub performance: PerformanceMetrics,
}

/// Alert threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    /// CPU usage threshold for alerts (percentage)
    pub cpu_threshold: f64,
    /// Memory usage threshold for alerts (percentage)
    pub memory_threshold: f64,
    /// Disk usage threshold for alerts (percentage)
    pub disk_threshold: f64,
    /// Error rate threshold for alerts (percentage)
    pub error_rate_threshold: f64,
    /// Response time threshold for alerts (milliseconds)
    pub response_time_threshold: f64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            cpu_threshold: 80.0,
            memory_threshold: 85.0,
            disk_threshold: 90.0,
            error_rate_threshold: 5.0,
            response_time_threshold: 1000.0,
        }
    }
}

/// Internal metrics summary for core BearDog operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalMetricsSummary {
    /// Number of security events processed
    pub security_events: u64,
    /// Number of encryption operations performed
    pub encryption_operations: u64,
    /// Number of threats detected
    pub threat_detections: u64,
    /// Number of compliance checks performed
    pub compliance_checks: u64,
    /// Number of API requests processed
    pub api_requests: u64,
    /// Number of errors encountered
    pub error_count: u64,
    /// Number of currently active sessions
    pub active_sessions: u64,
    /// When these metrics were last updated
    pub last_updated: DateTime<Utc>,
}

/// Prometheus configuration for metrics export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrometheusConfig {
    /// Whether Prometheus export is enabled
    pub enabled: bool,
    /// Prometheus server endpoint
    pub endpoint: String,
    /// Port for Prometheus metrics server
    pub port: u16,
    /// Path for metrics endpoint
    pub path: String,
    /// Update interval for metrics export in seconds
    pub update_interval_seconds: u64,
}

impl Default for PrometheusConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoint: "localhost".to_string(),
            port: 9090,
            path: "/metrics".to_string(),
            update_interval_seconds: 30,
        }
    }
}

/// Metric value types for flexible metrics collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricValue {
    /// Simple counter that only increases
    Counter(u64),
    /// Gauge that can increase or decrease
    Gauge(f64),
    /// Histogram with multiple sample values
    Histogram(Vec<f64>),
    /// Summary with sum and count
    Summary { sum: f64, count: u64 },
    /// String-based metric for labels or status
    Label(String),
}

impl MetricValue {
    /// Convert metric value to a numeric representation if possible
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            MetricValue::Counter(v) => Some(*v as f64),
            MetricValue::Gauge(v) => Some(*v),
            MetricValue::Summary { sum, count: _ } => Some(*sum),
            MetricValue::Histogram(values) => {
                if values.is_empty() {
                    Some(0.0)
                } else {
                    Some(values.iter().sum::<f64>() / values.len() as f64)
                }
            }
            MetricValue::Label(_) => None,
        }
    }
}

