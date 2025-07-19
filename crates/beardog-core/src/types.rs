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
    /// Optional details about the health check
    pub details: Option<String>,
    /// Time taken to perform the health check (in milliseconds)
    pub check_duration_ms: u64,
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
        Self {
            name: "unknown".to_string(),
            healthy: false,
            error_message: None,
            last_checked: Utc::now(),
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
            details: None,
            check_duration_ms: 0,
        }
    }
}

impl ComponentStatus {
    /// Create a new healthy component status
    pub fn healthy(name: String) -> Self {
        Self {
            name,
            healthy: true,
            error_message: None,
            last_checked: Utc::now(),
            metadata: HashMap::new(),
        }
    }

    /// Create a new unhealthy component status with error message
    pub fn unhealthy(name: String, error_message: String) -> Self {
        Self {
            name,
            healthy: false,
            error_message: Some(error_message),
            last_checked: Utc::now(),
            metadata: HashMap::new(),
        }
    }

    /// Update the health status of this component
    pub fn update_health(&mut self, healthy: bool, error_message: Option<String>) {
        self.healthy = healthy;
        self.error_message = error_message;
        self.last_checked = Utc::now();
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
            details: None,
            check_duration_ms,
        }
    }

    /// Create a failed health check with details
    pub fn failure(component_name: String, details: String, check_duration_ms: u64) -> Self {
        Self {
            component_name,
            healthy: false,
            details: Some(details),
            check_duration_ms,
        }
    }
} 