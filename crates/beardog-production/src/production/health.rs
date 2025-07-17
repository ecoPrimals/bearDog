//! Health monitoring and system status tracking
//!
//! This module provides comprehensive health monitoring capabilities,
//! including system health checks, component status tracking, liveness
//! and readiness probes, and alerting systems.

use std::collections::HashMap;
use std::time::SystemTime;

/// System health status enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    /// System is operating normally
    Healthy,
    /// System has minor issues but is functional
    Warning,
    /// System has serious issues requiring attention
    Critical,
    /// System health status is unknown
    Unknown,
}

/// Overall system health status
#[derive(Debug, Clone)]
pub struct SystemHealthStatus {
    /// Overall system health status
    pub overall_status: HealthStatus,
    /// CPU utilization percentage (0.0-1.0)
    pub cpu_utilization: f64,
    /// Memory utilization percentage (0.0-1.0)
    pub memory_utilization: f64,
    /// Disk utilization percentage (0.0-1.0)
    pub disk_utilization: f64,
    /// Whether network connectivity is available
    pub network_connectivity: bool,
}

/// Component health status collection
#[derive(Debug, Clone)]
pub struct ComponentHealthStatus {
    /// Health status for each system component
    pub components: HashMap<String, ComponentStatus>,
}

/// Individual component status
#[derive(Debug, Clone)]
pub struct ComponentStatus {
    /// Health status of the component
    pub health: ComponentHealth,
    /// Component uptime in seconds
    pub uptime_seconds: u64,
    /// Last time this component was checked
    pub last_check: SystemTime,
}

/// Component health enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum ComponentHealth {
    /// Component is healthy and functioning normally
    Healthy,
    /// Component has warnings but is still functional
    Warning,
    /// Component has critical issues
    Critical,
    /// Component is down or not responding
    Down,
}

/// Health endpoint test results
#[derive(Debug, Clone)]
pub struct HealthEndpointTest {
    /// HTTP status code returned by health endpoint
    pub status_code: u16,
    /// Response time in milliseconds
    pub response_time_ms: u64,
    /// Whether the response format is valid
    pub response_valid: bool,
}

/// Liveness probe for basic system availability
#[derive(Debug, Clone)]
pub struct LivenessProbe {
    /// Whether the system is alive
    pub is_alive: bool,
    /// Whether the core system is responding
    pub core_responding: bool,
    /// Whether critical processes are running
    pub critical_processes_running: bool,
}

/// Readiness probe for service availability
#[derive(Debug, Clone)]
pub struct ReadinessProbe {
    /// Whether the system is ready to serve requests
    pub is_ready: bool,
    /// Whether the system is accepting requests
    pub accepting_requests: bool,
    /// Whether all dependencies are available
    pub dependencies_available: bool,
}

/// Monitoring configuration for alerts and thresholds
#[derive(Debug, Clone)]
pub struct MonitoringConfiguration {
    /// CPU usage threshold for alerts (0.0-1.0)
    pub cpu_threshold: f64,
    /// Memory usage threshold for alerts (0.0-1.0)
    pub memory_threshold: f64,
    /// Disk usage threshold for alerts (0.0-1.0)
    pub disk_threshold: f64,
    /// Response time threshold for alerts in milliseconds
    pub response_time_threshold_ms: u64,
    /// Error rate threshold for alerts (0.0-1.0)
    pub error_rate_threshold: f64,
    /// Cooldown period between alerts in seconds
    pub alert_cooldown_seconds: u64,
}

/// Alert system configuration and status
#[derive(Debug, Clone)]
pub struct AlertSystem {
    /// Whether alerts are configured
    pub alerts_configured: bool,
    /// Available notification channels
    pub notification_channels: Vec<String>,
    /// Whether escalation procedures are defined
    pub escalation_procedures_defined: bool,
}

impl SystemHealthStatus {
    /// Create a new system health status
    pub fn new() -> Self {
        Self {
            overall_status: HealthStatus::Unknown,
            cpu_utilization: 0.0,
            memory_utilization: 0.0,
            disk_utilization: 0.0,
            network_connectivity: false,
        }
    }

    /// Update the overall status based on component metrics
    pub fn update_overall_status(&mut self) {
        // Determine overall status based on component metrics
        if self.cpu_utilization > 0.9
            || self.memory_utilization > 0.9
            || self.disk_utilization > 0.95
        {
            self.overall_status = HealthStatus::Critical;
        } else if self.cpu_utilization > 0.8
            || self.memory_utilization > 0.8
            || self.disk_utilization > 0.9
        {
            self.overall_status = HealthStatus::Warning;
        } else if self.network_connectivity {
            self.overall_status = HealthStatus::Healthy;
        } else {
            self.overall_status = HealthStatus::Critical;
        }
    }

    /// Check if the system is healthy
    pub fn is_healthy(&self) -> bool {
        matches!(self.overall_status, HealthStatus::Healthy)
    }

    /// Check if the system has warnings
    pub fn has_warnings(&self) -> bool {
        matches!(self.overall_status, HealthStatus::Warning)
    }

    /// Check if the system is in critical state
    pub fn is_critical(&self) -> bool {
        matches!(self.overall_status, HealthStatus::Critical)
    }

    /// Get a summary of resource utilization
    pub fn resource_summary(&self) -> String {
        format!(
            "CPU: {:.1}%, Memory: {:.1}%, Disk: {:.1}%, Network: {}",
            self.cpu_utilization * 100.0,
            self.memory_utilization * 100.0,
            self.disk_utilization * 100.0,
            if self.network_connectivity {
                "OK"
            } else {
                "DOWN"
            }
        )
    }
}

impl ComponentHealthStatus {
    /// Create a new component health status collection
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    /// Add or update a component status
    pub fn update_component(&mut self, name: String, status: ComponentStatus) {
        self.components.insert(name, status);
    }

    /// Get the status of a specific component
    pub fn get_component(&self, name: &str) -> Option<&ComponentStatus> {
        self.components.get(name)
    }

    /// Get all healthy components
    pub fn healthy_components(&self) -> Vec<(&String, &ComponentStatus)> {
        self.components
            .iter()
            .filter(|(_, status)| matches!(status.health, ComponentHealth::Healthy))
            .collect()
    }

    /// Get all components with warnings
    pub fn warning_components(&self) -> Vec<(&String, &ComponentStatus)> {
        self.components
            .iter()
            .filter(|(_, status)| matches!(status.health, ComponentHealth::Warning))
            .collect()
    }

    /// Get all critical components
    pub fn critical_components(&self) -> Vec<(&String, &ComponentStatus)> {
        self.components
            .iter()
            .filter(|(_, status)| matches!(status.health, ComponentHealth::Critical))
            .collect()
    }

    /// Get all down components
    pub fn down_components(&self) -> Vec<(&String, &ComponentStatus)> {
        self.components
            .iter()
            .filter(|(_, status)| matches!(status.health, ComponentHealth::Down))
            .collect()
    }

    /// Check if all components are healthy
    pub fn all_healthy(&self) -> bool {
        self.components
            .values()
            .all(|status| matches!(status.health, ComponentHealth::Healthy))
    }

    /// Get the overall health based on worst component
    pub fn overall_health(&self) -> ComponentHealth {
        if self
            .components
            .values()
            .any(|s| matches!(s.health, ComponentHealth::Down))
        {
            ComponentHealth::Down
        } else if self
            .components
            .values()
            .any(|s| matches!(s.health, ComponentHealth::Critical))
        {
            ComponentHealth::Critical
        } else if self
            .components
            .values()
            .any(|s| matches!(s.health, ComponentHealth::Warning))
        {
            ComponentHealth::Warning
        } else {
            ComponentHealth::Healthy
        }
    }
}

impl ComponentStatus {
    /// Create a new component status
    pub fn new(health: ComponentHealth) -> Self {
        Self {
            health,
            uptime_seconds: 0,
            last_check: SystemTime::now(),
        }
    }

    /// Create a healthy component status
    pub fn healthy() -> Self {
        Self::new(ComponentHealth::Healthy)
    }

    /// Create a warning component status
    pub fn warning() -> Self {
        Self::new(ComponentHealth::Warning)
    }

    /// Create a critical component status
    pub fn critical() -> Self {
        Self::new(ComponentHealth::Critical)
    }

    /// Create a down component status
    pub fn down() -> Self {
        Self::new(ComponentHealth::Down)
    }

    /// Update the last check time to now
    pub fn update_check_time(&mut self) {
        self.last_check = SystemTime::now();
    }

    /// Check if this component is healthy
    pub fn is_healthy(&self) -> bool {
        matches!(self.health, ComponentHealth::Healthy)
    }

    /// Check if this component is down
    pub fn is_down(&self) -> bool {
        matches!(self.health, ComponentHealth::Down)
    }

    /// Get the time since last check in seconds
    pub fn seconds_since_last_check(&self) -> u64 {
        self.last_check.elapsed().unwrap_or_default().as_secs()
    }
}

impl LivenessProbe {
    /// Create a new liveness probe
    pub fn new() -> Self {
        Self {
            is_alive: false,
            core_responding: false,
            critical_processes_running: false,
        }
    }

    /// Check if the probe indicates the system is alive
    pub fn is_alive(&self) -> bool {
        self.is_alive && self.core_responding && self.critical_processes_running
    }

    /// Update the liveness status
    pub fn update(
        &mut self,
        is_alive: bool,
        core_responding: bool,
        critical_processes_running: bool,
    ) {
        self.is_alive = is_alive;
        self.core_responding = core_responding;
        self.critical_processes_running = critical_processes_running;
    }
}

impl ReadinessProbe {
    /// Create a new readiness probe
    pub fn new() -> Self {
        Self {
            is_ready: false,
            accepting_requests: false,
            dependencies_available: false,
        }
    }

    /// Check if the probe indicates the system is ready
    pub fn is_ready(&self) -> bool {
        self.is_ready && self.accepting_requests && self.dependencies_available
    }

    /// Update the readiness status
    pub fn update(
        &mut self,
        is_ready: bool,
        accepting_requests: bool,
        dependencies_available: bool,
    ) {
        self.is_ready = is_ready;
        self.accepting_requests = accepting_requests;
        self.dependencies_available = dependencies_available;
    }
}

impl HealthEndpointTest {
    /// Create a new health endpoint test result
    pub fn new(status_code: u16, response_time_ms: u64, response_valid: bool) -> Self {
        Self {
            status_code,
            response_time_ms,
            response_valid,
        }
    }

    /// Check if the endpoint test passed
    pub fn is_healthy(&self) -> bool {
        self.status_code == 200 && self.response_valid
    }

    /// Check if the response time is acceptable
    pub fn is_fast_enough(&self, threshold_ms: u64) -> bool {
        self.response_time_ms <= threshold_ms
    }
}

impl MonitoringConfiguration {
    /// Create default monitoring configuration
    pub fn default() -> Self {
        Self {
            cpu_threshold: 0.8,
            memory_threshold: 0.8,
            disk_threshold: 0.9,
            response_time_threshold_ms: 1000,
            error_rate_threshold: 0.05,
            alert_cooldown_seconds: 300,
        }
    }

    /// Create production monitoring configuration with stricter thresholds
    pub fn production() -> Self {
        Self {
            cpu_threshold: 0.7,
            memory_threshold: 0.7,
            disk_threshold: 0.85,
            response_time_threshold_ms: 500,
            error_rate_threshold: 0.01,
            alert_cooldown_seconds: 600,
        }
    }

    /// Create development monitoring configuration with relaxed thresholds
    pub fn development() -> Self {
        Self {
            cpu_threshold: 0.9,
            memory_threshold: 0.9,
            disk_threshold: 0.95,
            response_time_threshold_ms: 2000,
            error_rate_threshold: 0.1,
            alert_cooldown_seconds: 60,
        }
    }

    /// Check if CPU usage exceeds threshold
    pub fn cpu_exceeds_threshold(&self, usage: f64) -> bool {
        usage > self.cpu_threshold
    }

    /// Check if memory usage exceeds threshold
    pub fn memory_exceeds_threshold(&self, usage: f64) -> bool {
        usage > self.memory_threshold
    }

    /// Check if disk usage exceeds threshold
    pub fn disk_exceeds_threshold(&self, usage: f64) -> bool {
        usage > self.disk_threshold
    }

    /// Check if response time exceeds threshold
    pub fn response_time_exceeds_threshold(&self, response_time_ms: u64) -> bool {
        response_time_ms > self.response_time_threshold_ms
    }

    /// Check if error rate exceeds threshold
    pub fn error_rate_exceeds_threshold(&self, error_rate: f64) -> bool {
        error_rate > self.error_rate_threshold
    }
}

impl AlertSystem {
    /// Create a new alert system
    pub fn new() -> Self {
        Self {
            alerts_configured: false,
            notification_channels: Vec::new(),
            escalation_procedures_defined: false,
        }
    }

    /// Configure the alert system
    pub fn configure(&mut self, notification_channels: Vec<String>, escalation_procedures: bool) {
        self.notification_channels = notification_channels;
        self.escalation_procedures_defined = escalation_procedures;
        self.alerts_configured = !self.notification_channels.is_empty();
    }

    /// Check if alerts are properly configured
    pub fn is_configured(&self) -> bool {
        self.alerts_configured && !self.notification_channels.is_empty()
    }

    /// Add a notification channel
    pub fn add_notification_channel(&mut self, channel: String) {
        if !self.notification_channels.contains(&channel) {
            self.notification_channels.push(channel);
            self.alerts_configured = true;
        }
    }

    /// Remove a notification channel
    pub fn remove_notification_channel(&mut self, channel: &str) {
        self.notification_channels.retain(|c| c != channel);
        self.alerts_configured = !self.notification_channels.is_empty();
    }
}

impl Default for SystemHealthStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ComponentHealthStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for LivenessProbe {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ReadinessProbe {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for MonitoringConfiguration {
    fn default() -> Self {
        Self::default()
    }
}

impl Default for AlertSystem {
    fn default() -> Self {
        Self::new()
    }
}
