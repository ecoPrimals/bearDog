// SPDX-License-Identifier: AGPL-3.0-only

//! # Production Health Monitoring
//!
//! This module provides types for production health monitoring,
//! including system health, component status, and alerting.

use beardog_types::canonical::HealthStatus;
use std::collections::HashMap;
use std::time::SystemTime;

// ============================================================
// System Health
// ============================================================

/// System health status
#[derive(Debug, Clone)]
pub struct SystemHealthStatus {
    /// Overall status
    pub overall_status: HealthStatus,

    /// CPU utilization (0.0 - 1.0)
    pub cpu_utilization: f64,

    /// Memory utilization (0.0 - 1.0)
    pub memory_utilization: f64,

    /// Disk utilization (0.0 - 1.0)
    pub disk_utilization: f64,

    /// Network connectivity
    pub network_connectivity: bool,
}

impl SystemHealthStatus {
    /// Create new health status
    pub fn new() -> Self {
        Self {
            overall_status: HealthStatus::Unknown,
            cpu_utilization: 0.0,
            memory_utilization: 0.0,
            disk_utilization: 0.0,
            network_connectivity: false,
        }
    }

    /// Update overall status based on metrics
    pub fn update_overall_status(&mut self) {
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
            self.overall_status = HealthStatus::Warning;
        }
    }

    /// Check if healthy
    pub fn is_healthy(&self) -> bool {
        matches!(self.overall_status, HealthStatus::Healthy)
    }

    /// Check if has warnings
    pub fn has_warnings(&self) -> bool {
        matches!(self.overall_status, HealthStatus::Warning)
    }

    /// Check if critical
    pub fn is_critical(&self) -> bool {
        matches!(self.overall_status, HealthStatus::Critical)
    }

    /// Get resource summary
    pub fn resource_summary(&self) -> String {
        format!(
            "CPU: {:.1}%, Memory: {:.1}%, Disk: {:.1}%, Network: {}",
            self.cpu_utilization * 100.0,
            self.memory_utilization * 100.0,
            self.disk_utilization * 100.0,
            if self.network_connectivity { "OK" } else { "DOWN" }
        )
    }
}

impl Default for SystemHealthStatus {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// Component Health
// ============================================================

/// Component health enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentHealth {
    /// Healthy
    Healthy,
    /// Warning
    Warning,
    /// Critical
    Critical,
    /// Down
    Down,
}

impl Default for ComponentHealth {
    fn default() -> Self {
        Self::Healthy
    }
}

/// Component status
#[derive(Debug, Clone)]
pub struct ComponentStatus {
    /// Health status
    pub health: ComponentHealth,

    /// Uptime in seconds
    pub uptime_seconds: u64,

    /// Last check time
    pub last_check: SystemTime,
}

impl ComponentStatus {
    /// Create new status
    pub fn new(health: ComponentHealth) -> Self {
        Self {
            health,
            uptime_seconds: 0,
            last_check: SystemTime::now(),
        }
    }

    /// Create healthy status
    pub fn healthy() -> Self {
        Self::new(ComponentHealth::Healthy)
    }

    /// Create warning status
    pub fn warning() -> Self {
        Self::new(ComponentHealth::Warning)
    }

    /// Create critical status
    pub fn critical() -> Self {
        Self::new(ComponentHealth::Critical)
    }

    /// Create down status
    pub fn down() -> Self {
        Self::new(ComponentHealth::Down)
    }

    /// Update check time
    pub fn update_check_time(&mut self) {
        self.last_check = SystemTime::now();
    }

    /// Check if healthy
    pub fn is_healthy(&self) -> bool {
        matches!(self.health, ComponentHealth::Healthy)
    }

    /// Check if down
    pub fn is_down(&self) -> bool {
        matches!(self.health, ComponentHealth::Down)
    }

    /// Get seconds since last check
    pub fn seconds_since_last_check(&self) -> u64 {
        self.last_check
            .elapsed()
            .map(|d| d.as_secs())
            .unwrap_or(0)
    }
}

impl Default for ComponentStatus {
    fn default() -> Self {
        Self::healthy()
    }
}

/// Component health status collection
#[derive(Debug, Clone, Default)]
pub struct ComponentHealthStatus {
    /// Components by name
    pub components: HashMap<String, ComponentStatus>,
}

impl ComponentHealthStatus {
    /// Create new status
    pub fn new() -> Self {
        Self {
            components: HashMap::with_capacity(16),
        }
    }

    /// Add component
    pub fn add_component(&mut self, name: &str, status: ComponentStatus) {
        self.components.insert(name.to_string(), status);
    }

    /// Get component
    pub fn get_component(&self, name: &str) -> Option<&ComponentStatus> {
        self.components.get(name)
    }

    /// Get healthy components
    pub fn healthy_components(&self) -> Vec<(&String, &ComponentStatus)> {
        self.components
            .iter()
            .filter(|(_, status)| matches!(status.health, ComponentHealth::Healthy))
            .collect()
    }

    /// Get warning components
    pub fn warning_components(&self) -> Vec<(&String, &ComponentStatus)> {
        self.components
            .iter()
            .filter(|(_, status)| matches!(status.health, ComponentHealth::Warning))
            .collect()
    }

    /// Get critical components
    pub fn critical_components(&self) -> Vec<(&String, &ComponentStatus)> {
        self.components
            .iter()
            .filter(|(_, status)| matches!(status.health, ComponentHealth::Critical))
            .collect()
    }

    /// Get down components
    pub fn down_components(&self) -> Vec<(&String, &ComponentStatus)> {
        self.components
            .iter()
            .filter(|(_, status)| matches!(status.health, ComponentHealth::Down))
            .collect()
    }

    /// Check if all healthy
    pub fn all_healthy(&self) -> bool {
        self.components
            .values()
            .all(|status| matches!(status.health, ComponentHealth::Healthy))
    }

    /// Get overall health
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

// ============================================================
// Probes
// ============================================================

/// Liveness probe
#[derive(Debug, Clone)]
pub struct LivenessProbe {
    /// Is alive
    pub is_alive: bool,

    /// Core responding
    pub core_responding: bool,

    /// Critical processes running
    pub critical_processes_running: bool,
}

impl LivenessProbe {
    /// Create new probe
    pub fn new() -> Self {
        Self {
            is_alive: false,
            core_responding: false,
            critical_processes_running: false,
        }
    }

    /// Set alive status
    pub fn set_alive(&mut self, is_alive: bool, core_responding: bool, critical_processes_running: bool) {
        self.is_alive = is_alive;
        self.core_responding = core_responding;
        self.critical_processes_running = critical_processes_running;
    }

    /// Check if fully alive
    pub fn is_fully_alive(&self) -> bool {
        self.is_alive && self.core_responding && self.critical_processes_running
    }
}

impl Default for LivenessProbe {
    fn default() -> Self {
        Self::new()
    }
}

/// Readiness probe
#[derive(Debug, Clone)]
pub struct ReadinessProbe {
    /// Is ready
    pub is_ready: bool,

    /// Accepting requests
    pub accepting_requests: bool,

    /// Dependencies available
    pub dependencies_available: bool,
}

impl ReadinessProbe {
    /// Create new probe
    pub fn new() -> Self {
        Self {
            is_ready: false,
            accepting_requests: false,
            dependencies_available: false,
        }
    }

    /// Set ready status
    pub fn set_ready(&mut self, is_ready: bool, accepting_requests: bool, dependencies_available: bool) {
        self.is_ready = is_ready;
        self.accepting_requests = accepting_requests;
        self.dependencies_available = dependencies_available;
    }

    /// Check if fully ready
    pub fn is_fully_ready(&self) -> bool {
        self.is_ready && self.accepting_requests && self.dependencies_available
    }
}

impl Default for ReadinessProbe {
    fn default() -> Self {
        Self::new()
    }
}

/// Health endpoint test
#[derive(Debug, Clone)]
pub struct HealthEndpointTest {
    /// Status code
    pub status_code: u16,

    /// Response time in ms
    pub response_time_ms: u64,

    /// Response valid
    pub response_valid: bool,
}

impl HealthEndpointTest {
    /// Create new test result
    pub fn new(status_code: u16, response_time_ms: u64, response_valid: bool) -> Self {
        Self {
            status_code,
            response_time_ms,
            response_valid,
        }
    }

    /// Check if passed
    pub fn passed(&self) -> bool {
        self.status_code == 200 && self.response_valid
    }

    /// Check if fast enough
    pub fn is_fast_enough(&self, threshold_ms: u64) -> bool {
        self.response_time_ms <= threshold_ms
    }
}

// ============================================================
// Monitoring Configuration
// ============================================================

/// Monitoring configuration
#[derive(Debug, Clone)]
pub struct MonitoringConfiguration {
    /// CPU threshold (0.0 - 1.0)
    pub cpu_threshold: f64,

    /// Memory threshold (0.0 - 1.0)
    pub memory_threshold: f64,

    /// Disk threshold (0.0 - 1.0)
    pub disk_threshold: f64,

    /// Response time threshold in ms
    pub response_time_threshold_ms: u64,

    /// Error rate threshold (0.0 - 1.0)
    pub error_rate_threshold: f64,

    /// Alert cooldown in seconds
    pub alert_cooldown_seconds: u64,
}

impl MonitoringConfiguration {
    /// Create default configuration
    pub fn new_default() -> Self {
        Self {
            cpu_threshold: 0.8,
            memory_threshold: 0.8,
            disk_threshold: 0.9,
            response_time_threshold_ms: 1000,
            error_rate_threshold: 0.05,
            alert_cooldown_seconds: 300,
        }
    }

    /// Create production configuration
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

    /// Create development configuration
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

    /// Check if CPU exceeds threshold
    pub fn cpu_exceeds_threshold(&self, usage: f64) -> bool {
        usage > self.cpu_threshold
    }

    /// Check if memory exceeds threshold
    pub fn memory_exceeds_threshold(&self, usage: f64) -> bool {
        usage > self.memory_threshold
    }

    /// Check if disk exceeds threshold
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

impl Default for MonitoringConfiguration {
    fn default() -> Self {
        Self::new_default()
    }
}

// ============================================================
// Alert System
// ============================================================

/// Alert system
#[derive(Debug, Clone)]
pub struct AlertSystem {
    /// Alerts configured
    pub alerts_configured: bool,

    /// Notification channels
    pub notification_channels: Vec<String>,

    /// Escalation procedures defined
    pub escalation_procedures_defined: bool,
}

impl AlertSystem {
    /// Create new alert system
    pub fn new() -> Self {
        Self {
            alerts_configured: false,
            notification_channels: Vec::new(),
            escalation_procedures_defined: false,
        }
    }

    /// Configure alert system
    pub fn configure(&mut self, notification_channels: Vec<String>, escalation_procedures: bool) {
        self.notification_channels = notification_channels;
        self.escalation_procedures_defined = escalation_procedures;
        self.alerts_configured = !self.notification_channels.is_empty();
    }

    /// Check if configured
    pub fn is_configured(&self) -> bool {
        self.alerts_configured && !self.notification_channels.is_empty()
    }

    /// Add notification channel
    pub fn add_notification_channel(&mut self, channel: &str) {
        if !self.notification_channels.contains(&channel.to_string()) {
            self.notification_channels.push(channel.to_string());
            self.alerts_configured = true;
        }
    }

    /// Remove notification channel
    pub fn remove_notification_channel(&mut self, channel: &str) {
        self.notification_channels.retain(|c| c != channel);
        self.alerts_configured = !self.notification_channels.is_empty();
    }
}

impl Default for AlertSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_health_status() {
        let mut status = SystemHealthStatus::new();
        status.cpu_utilization = 0.5;
        status.memory_utilization = 0.5;
        status.disk_utilization = 0.5;
        status.network_connectivity = true;
        status.update_overall_status();
        assert!(status.is_healthy());
    }

    #[test]
    fn test_system_health_critical() {
        let mut status = SystemHealthStatus::new();
        status.cpu_utilization = 0.95;
        status.update_overall_status();
        assert!(status.is_critical());
    }

    #[test]
    fn test_component_status() {
        let status = ComponentStatus::healthy();
        assert!(status.is_healthy());
        assert!(!status.is_down());
    }

    #[test]
    fn test_component_health_status() {
        let mut health = ComponentHealthStatus::new();
        health.add_component("service1", ComponentStatus::healthy());
        health.add_component("service2", ComponentStatus::warning());
        assert!(!health.all_healthy());
        assert_eq!(health.overall_health(), ComponentHealth::Warning);
    }

    #[test]
    fn test_liveness_probe() {
        let mut probe = LivenessProbe::new();
        assert!(!probe.is_fully_alive());
        probe.set_alive(true, true, true);
        assert!(probe.is_fully_alive());
    }

    #[test]
    fn test_readiness_probe() {
        let mut probe = ReadinessProbe::new();
        assert!(!probe.is_fully_ready());
        probe.set_ready(true, true, true);
        assert!(probe.is_fully_ready());
    }

    #[test]
    fn test_health_endpoint_test() {
        let test = HealthEndpointTest::new(200, 50, true);
        assert!(test.passed());
        assert!(test.is_fast_enough(100));
    }

    #[test]
    fn test_monitoring_configuration() {
        let config = MonitoringConfiguration::default();
        assert!(!config.cpu_exceeds_threshold(0.5));
        assert!(config.cpu_exceeds_threshold(0.9));
    }

    #[test]
    fn test_alert_system() {
        let mut alerts = AlertSystem::new();
        assert!(!alerts.is_configured());
        alerts.add_notification_channel("email");
        assert!(alerts.is_configured());
    }
}
