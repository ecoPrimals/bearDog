//! Monitoring configuration
//!
//! This module contains monitoring-related configuration types.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Enable monitoring
    pub enabled: bool,
    /// Core monitoring settings
    pub core: MonitoringCoreConfig,
    /// Metrics settings
    pub metrics: MetricsConfig,
    /// Health check settings
    pub health: HealthConfig,
    /// Alerting settings
    pub alerting: AlertingConfig,
    /// Logging settings
    pub logging: MonitoringLoggingConfig,
    /// Tracing settings
    pub tracing: TracingConfig,
    /// Performance monitoring
    pub performance: PerformanceMonitoringConfig,
    /// System monitoring
    pub system: SystemMonitoringConfig,
    /// Security monitoring
    pub security: SecurityMonitoringConfig,
    /// Prometheus settings
    pub prometheus: PrometheusConfig,
}

/// Core monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringCoreConfig {
    /// Monitoring interval
    pub interval: Duration,
    /// Enable detailed monitoring
    pub detailed: bool,
}

/// Metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enable metrics collection
    pub enabled: bool,
    /// Metrics endpoint
    pub endpoint: String,
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthConfig {
    /// Enable health checks
    pub enabled: bool,
    /// Health check interval
    pub interval: Duration,
}

/// Alerting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertingConfig {
    /// Enable alerting
    pub enabled: bool,
    /// Alert rules
    pub rules: Vec<AlertRule>,
}

/// Alert rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    /// Rule name
    pub name: String,
    /// Rule condition
    pub condition: String,
    /// Severity level
    pub severity: String,
}

/// Monitoring logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringLoggingConfig {
    /// Enable monitoring logs
    pub enabled: bool,
    /// Log level
    pub level: String,
}

/// Tracing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfig {
    /// Enable tracing
    pub enabled: bool,
    /// Trace endpoint
    pub endpoint: String,
}

/// Performance monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMonitoringConfig {
    /// Enable performance monitoring
    pub enabled: bool,
    /// Sampling rate
    pub sampling_rate: f64,
}

/// System monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMonitoringConfig {
    /// Enable system monitoring
    pub enabled: bool,
    /// Monitor CPU
    pub monitor_cpu: bool,
    /// Monitor memory
    pub monitor_memory: bool,
}

/// Security monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMonitoringConfig {
    /// Enable security monitoring
    pub enabled: bool,
    /// Monitor authentication
    pub monitor_auth: bool,
}

/// Prometheus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrometheusConfig {
    /// Enable Prometheus
    pub enabled: bool,
    /// Prometheus endpoint
    pub endpoint: String,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            core: MonitoringCoreConfig::default(),
            metrics: MetricsConfig::default(),
            health: HealthConfig::default(),
            alerting: AlertingConfig::default(),
            logging: MonitoringLoggingConfig::default(),
            tracing: TracingConfig::default(),
            performance: PerformanceMonitoringConfig::default(),
            system: SystemMonitoringConfig::default(),
            security: SecurityMonitoringConfig::default(),
            prometheus: PrometheusConfig::default(),
        }
    }
}

impl Default for MonitoringCoreConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(60),
            detailed: false,
        }
    }
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: "/metrics".to_string(),
        }
    }
}

impl Default for HealthConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(30),
        }
    }
}

impl Default for AlertingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            rules: Vec::new(),
        }
    }
}

impl Default for MonitoringLoggingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            level: "info".to_string(),
        }
    }
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoint: "http://localhost:14268/api/traces".to_string(),
        }
    }
}

impl Default for PerformanceMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            sampling_rate: 0.1,
        }
    }
}

impl Default for SystemMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            monitor_cpu: true,
            monitor_memory: true,
        }
    }
}

impl Default for SecurityMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            monitor_auth: true,
        }
    }
}

impl Default for PrometheusConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoint: "http://localhost:9090".to_string(),
        }
    }
} 