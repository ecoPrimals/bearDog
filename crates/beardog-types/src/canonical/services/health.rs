//! Unified Service Health Status Definitions
//!
//! This module provides consolidated health status types for service monitoring,
//! replacing scattered health definitions across the codebase.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unified health status for services
///
/// Consolidates health status types from various locations across the codebase
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UnifiedHealthStatus {
    /// Service is healthy and operating normally
    Healthy,
    /// Service is degraded but still functional
    Degraded,
    /// Service is unhealthy and may not function properly
    Unhealthy,
    /// Service health status is unknown
    Unknown,
    /// Service is starting up
    Starting,
    /// Service is shutting down
    Stopping,
    /// Service is temporarily unavailable for maintenance
    Maintenance,
}

/// Comprehensive health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    /// Overall health status
    pub status: UnifiedHealthStatus,
    
    /// Timestamp of the health check
    pub timestamp: DateTime<Utc>,
    
    /// Detailed health information
    pub details: HealthDetails,
    
    /// Individual component health checks
    pub components: HashMap<String, ComponentHealth>,
    
    /// Health check duration in milliseconds
    pub check_duration_ms: u64,
    
    /// Health check version/revision
    pub version: String,
}

/// Detailed health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthDetails {
    /// Human-readable status message
    pub message: String,
    
    /// Error details if unhealthy
    pub error: Option<String>,
    
    /// Performance metrics
    pub metrics: Option<HealthMetrics>,
    
    /// Additional diagnostic information
    pub diagnostics: HashMap<String, serde_json::Value>,
}

/// Individual component health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    /// Component health status
    pub status: UnifiedHealthStatus,
    
    /// Component-specific message
    pub message: Option<String>,
    
    /// Error details if component is unhealthy
    pub error: Option<String>,
    
    /// Component metrics
    pub metrics: Option<ComponentMetrics>,
    
    /// Last successful check timestamp
    pub last_success: Option<DateTime<Utc>>,
    
    /// Last failure timestamp
    pub last_failure: Option<DateTime<Utc>>,
}

/// Health-related performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMetrics {
    /// Response time in milliseconds
    pub response_time_ms: f64,
    
    /// CPU utilization percentage
    pub cpu_utilization_percent: f64,
    
    /// Memory utilization percentage
    pub memory_utilization_percent: f64,
    
    /// Disk utilization percentage
    pub disk_utilization_percent: f64,
    
    /// Network throughput in bytes per second
    pub network_throughput_bps: f64,
    
    /// Active connections count
    pub active_connections: u64,
    
    /// Error rate (0.0 to 1.0)
    pub error_rate: f64,
    
    /// Requests per second
    pub requests_per_second: f64,
}

/// Component-specific metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentMetrics {
    /// Component response time in milliseconds
    pub response_time_ms: f64,
    
    /// Component availability (0.0 to 1.0)
    pub availability: f64,
    
    /// Component error count
    pub error_count: u64,
    
    /// Component success count
    pub success_count: u64,
    
    /// Component-specific metrics
    pub custom_metrics: HashMap<String, f64>,
}

impl Default for UnifiedHealthStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Default for HealthCheckResult {
    fn default() -> Self {
        Self {
            status: UnifiedHealthStatus::Unknown,
            timestamp: Utc::now(),
            details: HealthDetails::default(),
            components: HashMap::new(),
            check_duration_ms: 0,
            version: "1.0.0".to_string(),
        }
    }
}

impl Default for HealthDetails {
    fn default() -> Self {
        Self {
            message: "Health status unknown".to_string(),
            error: None,
            metrics: None,
            diagnostics: HashMap::new(),
        }
    }
}

impl UnifiedHealthStatus {
    /// Check if the status represents a healthy state
    pub fn is_healthy(&self) -> bool {
        matches!(self, Self::Healthy)
    }
    
    /// Check if the status represents an unhealthy state
    pub fn is_unhealthy(&self) -> bool {
        matches!(self, Self::Unhealthy | Self::Unknown)
    }
    
    /// Check if the status represents a degraded but functional state
    pub fn is_degraded(&self) -> bool {
        matches!(self, Self::Degraded)
    }
    
    /// Check if the status represents a transitional state
    pub fn is_transitional(&self) -> bool {
        matches!(self, Self::Starting | Self::Stopping | Self::Maintenance)
    }
    
    /// Get a human-readable description of the status
    pub fn description(&self) -> &'static str {
        match self {
            Self::Healthy => "Service is healthy and operating normally",
            Self::Degraded => "Service is degraded but still functional",
            Self::Unhealthy => "Service is unhealthy and may not function properly",
            Self::Unknown => "Service health status is unknown",
            Self::Starting => "Service is starting up",
            Self::Stopping => "Service is shutting down",
            Self::Maintenance => "Service is temporarily unavailable for maintenance",
        }
    }
    
    /// Get the severity level of this status (0 = best, 6 = worst)
    pub fn severity_level(&self) -> u8 {
        match self {
            Self::Healthy => 0,
            Self::Degraded => 2,
            Self::Starting => 3,
            Self::Maintenance => 3,
            Self::Stopping => 4,
            Self::Unknown => 5,
            Self::Unhealthy => 6,
        }
    }
}

impl HealthCheckResult {
    /// Create a new healthy health check result
    pub fn healthy(message: String) -> Self {
        Self {
            status: UnifiedHealthStatus::Healthy,
            details: HealthDetails {
                message,
                error: None,
                metrics: None,
                diagnostics: HashMap::new(),
            },
            ..Default::default()
        }
    }
    
    /// Create a new unhealthy health check result
    pub fn unhealthy(message: String, error: String) -> Self {
        Self {
            status: UnifiedHealthStatus::Unhealthy,
            details: HealthDetails {
                message,
                error: Some(error),
                metrics: None,
                diagnostics: HashMap::new(),
            },
            ..Default::default()
        }
    }
    
    /// Create a new degraded health check result
    pub fn degraded(message: String) -> Self {
        Self {
            status: UnifiedHealthStatus::Degraded,
            details: HealthDetails {
                message,
                error: None,
                metrics: None,
                diagnostics: HashMap::new(),
            },
            ..Default::default()
        }
    }
    
    /// Add a component health check result
    pub fn add_component(&mut self, name: String, component: ComponentHealth) -> &mut Self {
        self.components.insert(name, component);
        self
    }
    
    /// Set health metrics
    pub fn with_metrics(mut self, metrics: HealthMetrics) -> Self {
        self.details.metrics = Some(metrics);
        self
    }
    
    /// Add diagnostic information
    pub fn add_diagnostic(&mut self, key: String, value: serde_json::Value) -> &mut Self {
        self.details.diagnostics.insert(key, value);
        self
    }
    
    /// Calculate overall health status based on component statuses
    pub fn calculate_overall_status(&mut self) {
        if self.components.is_empty() {
            return;
        }
        
        let mut max_severity = 0u8;
        for component in self.components.values() {
            let severity = component.status.severity_level();
            if severity > max_severity {
                max_severity = severity;
            }
        }
        
        self.status = match max_severity {
            0 => UnifiedHealthStatus::Healthy,
            1..=2 => UnifiedHealthStatus::Degraded,
            3..=4 => UnifiedHealthStatus::Degraded,
            _ => UnifiedHealthStatus::Unhealthy,
        };
    }
    
    /// Check if all components are healthy
    pub fn all_components_healthy(&self) -> bool {
        self.components
            .values()
            .all(|component| component.status.is_healthy())
    }
    
    /// Get unhealthy components
    pub fn unhealthy_components(&self) -> Vec<(&String, &ComponentHealth)> {
        self.components
            .iter()
            .filter(|(_, component)| component.status.is_unhealthy())
            .collect()
    }
}

impl ComponentHealth {
    /// Create a healthy component health status
    pub fn healthy(message: Option<String>) -> Self {
        Self {
            status: UnifiedHealthStatus::Healthy,
            message,
            error: None,
            metrics: None,
            last_success: Some(Utc::now()),
            last_failure: None,
        }
    }
    
    /// Create an unhealthy component health status
    pub fn unhealthy(message: Option<String>, error: String) -> Self {
        Self {
            status: UnifiedHealthStatus::Unhealthy,
            message,
            error: Some(error),
            metrics: None,
            last_success: None,
            last_failure: Some(Utc::now()),
        }
    }
    
    /// Create a degraded component health status
    pub fn degraded(message: Option<String>) -> Self {
        Self {
            status: UnifiedHealthStatus::Degraded,
            message,
            error: None,
            metrics: None,
            last_success: Some(Utc::now()),
            last_failure: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_methods() {
        assert!(UnifiedHealthStatus::Healthy.is_healthy());
        assert!(!UnifiedHealthStatus::Healthy.is_unhealthy());
        assert!(!UnifiedHealthStatus::Healthy.is_degraded());
        
        assert!(UnifiedHealthStatus::Degraded.is_degraded());
        assert!(!UnifiedHealthStatus::Degraded.is_healthy());
        
        assert!(UnifiedHealthStatus::Unhealthy.is_unhealthy());
        assert!(!UnifiedHealthStatus::Unhealthy.is_healthy());
    }
     // TEST_CATEGORY: unit
     // TEST_DOMAIN: types
     // TEST_PRIORITY: normal
    
    #[test]
    fn test_health_status_severity() {
        assert_eq!(UnifiedHealthStatus::Healthy.severity_level(), 0);
        assert_eq!(UnifiedHealthStatus::Degraded.severity_level(), 2);
        assert_eq!(UnifiedHealthStatus::Unhealthy.severity_level(), 6);
    }
    
    #[test]
    fn test_health_check_result_creation() {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let result = HealthCheckResult::healthy("All systems operational".to_string());
        assert_eq!(result.status, UnifiedHealthStatus::Healthy);
        assert_eq!(result.details.message, "All systems operational");
        assert!(result.details.error.is_none());
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    }
    
    #[test]
    fn test_component_health_management() {
        let mut result = HealthCheckResult::default();
         // TEST_CATEGORY: unit
         // TEST_DOMAIN: types
         // TEST_PRIORITY: normal
        
        result.add_component(
            "database".to_string(),
            ComponentHealth::healthy(Some("Database connection active".to_string())),
        );
        
        result.add_component(
            "cache".to_string(),
            ComponentHealth::unhealthy(
                Some("Cache connection failed".to_string()),
                "Connection timeout".to_string(),
            ),
        );
        
        assert_eq!(result.components.len(), 2);
        assert!(!result.all_components_healthy());
        
        let unhealthy = result.unhealthy_components();
        assert_eq!(unhealthy.len(), 1);
        assert_eq!(unhealthy[0].0, "cache");
    }
    
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_overall_status_calculation() {
        let mut result = HealthCheckResult::default();
        
        // All healthy components should result in healthy overall status
        result.add_component(
            "db".to_string(),
            ComponentHealth::healthy(None),
        );
        result.add_component(
            "cache".to_string(),
            ComponentHealth::healthy(None),
        );
        
        result.calculate_overall_status();
        assert_eq!(result.status, UnifiedHealthStatus::Healthy);
        
        // Add an unhealthy component
        result.add_component(
            "queue".to_string(),
            ComponentHealth::unhealthy(None, "Connection failed".to_string()),
        );
        
        result.calculate_overall_status();
        assert_eq!(result.status, UnifiedHealthStatus::Unhealthy);
    }
} 