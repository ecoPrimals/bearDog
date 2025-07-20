//! Type definitions for BearDog Core
//!
//! This module contains the core types, status enums, and data structures
//! used throughout the BearDog core system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Internal core state structure
#[derive(Debug)]
pub struct CoreState {
    /// When the core was started
    pub start_time: Option<DateTime<Utc>>,
    /// Status of individual components
    pub component_status: HashMap<String, ComponentStatus>,
    /// Overall system health
    pub health_status: HealthStatus,
    /// Performance metrics
    pub metrics: SystemMetrics,
}

/// Status of individual components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentStatus {
    /// Component name
    pub name: String,
    /// Whether the component is healthy
    pub healthy: bool,
    /// Optional error message if not healthy
    pub error_message: Option<String>,
    /// Last time this component was checked
    pub last_checked: DateTime<Utc>,
    /// Alias for last_checked (for compatibility)
    pub last_check: DateTime<Utc>,
    /// Component uptime duration
    pub uptime: Option<chrono::Duration>,
    /// Component-specific metadata
    pub metadata: HashMap<String, String>,
}

/// Overall health status of the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    /// All systems operational
    Healthy,
    /// Some non-critical issues detected
    Degraded,
    /// Critical issues detected
    Unhealthy,
    /// System is starting up
    Starting,
    /// System is shutting down
    Stopping,
}

/// System performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage in MB
    pub memory_usage_mb: u64,
    /// Number of active connections
    pub active_connections: u32,
    /// Request processing rate (requests per second)
    pub requests_per_second: f64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Timestamp when these metrics were collected
    pub collected_at: DateTime<Utc>,
    /// Custom metrics specific to BearDog operations
    pub custom_metrics: HashMap<String, f64>,
}

/// Health check result for individual components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    /// Name of the component being checked
    pub component_name: String,
    /// Whether the health check passed
    pub healthy: bool,
    /// Overall health status
    pub status: HealthStatus,
    /// System uptime
    pub uptime: Option<chrono::Duration>,
    /// Optional details about the health check
    pub details: Option<String>,
    /// Time taken to perform the health check (in milliseconds)
    pub check_duration_ms: u64,
    /// Individual component statuses
    pub components: Vec<ComponentStatus>,
    /// Performance metrics
    pub metrics: SystemMetrics,
    /// Timestamp when the health check was performed
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Default for CoreState {
    fn default() -> Self {
        Self {
            start_time: None,
            component_status: HashMap::new(),
            health_status: HealthStatus::Starting,
            metrics: SystemMetrics::default(),
        }
    }
}

impl Default for ComponentStatus {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            name: "unknown".to_string(),
            healthy: false,
            error_message: None,
            last_checked: now,
            last_check: now,
            uptime: None,
            metadata: HashMap::new(),
        }
    }
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage_mb: 0,
            active_connections: 0,
            requests_per_second: 0.0,
            avg_response_time_ms: 0.0,
            collected_at: Utc::now(),
            custom_metrics: HashMap::new(),
        }
    }
}

impl Default for HealthCheck {
    fn default() -> Self {
        Self {
            component_name: "unknown".to_string(),
            healthy: false,
            status: HealthStatus::Starting,
            uptime: None,
            details: None,
            check_duration_ms: 0,
            components: Vec::new(),
            metrics: SystemMetrics::default(),
            timestamp: chrono::Utc::now(),
        }
    }
}

impl ComponentStatus {
    /// Create a new healthy component status
    pub fn healthy(name: String) -> Self {
        let now = Utc::now();
        Self {
            name,
            healthy: true,
            error_message: None,
            last_checked: now,
            last_check: now,
            uptime: None,
            metadata: HashMap::new(),
        }
    }

    /// Create a new unhealthy component status with error message
    pub fn unhealthy(name: String, error_message: String) -> Self {
        let now = Utc::now();
        Self {
            name,
            healthy: false,
            error_message: Some(error_message),
            last_checked: now,
            last_check: now,
            uptime: None,
            metadata: HashMap::new(),
        }
    }

    /// Update the health status of this component
    pub fn update_health(&mut self, healthy: bool, error_message: Option<String>) {
        let now = Utc::now();
        self.healthy = healthy;
        self.error_message = error_message;
        self.last_checked = now;
        self.last_check = now;
    }
}

impl SystemMetrics {
    /// Create new system metrics with current timestamp
    pub fn new() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage_mb: 0,
            active_connections: 0,
            requests_per_second: 0.0,
            avg_response_time_ms: 0.0,
            collected_at: Utc::now(),
            custom_metrics: HashMap::new(),
        }
    }

    /// Update a custom metric
    pub fn set_custom_metric(&mut self, name: String, value: f64) {
        self.custom_metrics.insert(name, value);
        self.collected_at = Utc::now();
    }

    /// Get a custom metric by name
    pub fn get_custom_metric(&self, name: &str) -> Option<f64> {
        self.custom_metrics.get(name).copied()
    }
}

impl HealthCheck {
    /// Create a successful health check
    pub fn success(component_name: String, check_duration_ms: u64) -> Self {
        Self {
            component_name,
            healthy: true,
            status: HealthStatus::Healthy,
            uptime: None,
            details: None,
            check_duration_ms,
            components: Vec::new(),
            metrics: SystemMetrics::default(),
            timestamp: Utc::now(),
        }
    }

    /// Create a failed health check with details
    pub fn failure(component_name: String, details: String, check_duration_ms: u64) -> Self {
        Self {
            component_name,
            healthy: false,
            status: HealthStatus::Unhealthy,
            uptime: None,
            details: Some(details),
            check_duration_ms,
            components: Vec::new(),
            metrics: SystemMetrics::default(),
            timestamp: Utc::now(),
        }
    }
}
