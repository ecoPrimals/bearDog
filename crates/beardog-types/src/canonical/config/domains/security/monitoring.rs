//! Security Monitoring and Auditing Configuration
//!
//! This module provides audit, security monitoring, and SIEM integration
//! configuration structures for the BearDog security system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Audit configuration - consolidates `AuditConfig`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfiguration {
    /// Enable audit logging
    pub enable_audit_logging: bool,
    /// Audit log retention days
    pub retention_days: u32,
    /// Audit log format
    pub log_format: String,
    /// Enable real-time audit monitoring
    pub enable_realtime_monitoring: bool,
    /// Audit storage backend
    pub storage_backend: String,
    /// Compliance requirements
    pub compliance_standards: Vec<String>,
}

impl AuditConfiguration {
    /// Create configuration from a config source (modern pattern)
    pub fn from_source(source: &dyn crate::canonical::config::source::ConfigSource) -> Self {
        use crate::canonical::config::source::get_parsed;

        Self {
            enable_audit_logging: true,
            retention_days: get_parsed(source, "BEARDOG_AUDIT_RETENTION_DAYS", 2555), // 7 years default
            log_format: "json".to_string(),
            enable_realtime_monitoring: true,
            storage_backend: "encrypted_file".to_string(),
            compliance_standards: vec!["SOX".to_string(), "GDPR".to_string(), "CCPA".to_string()],
        }
    }
}

impl Default for AuditConfiguration {
    fn default() -> Self {
        use crate::canonical::config::source::EnvConfigSource;
        Self::from_source(&EnvConfigSource::new())
    }
}

/// Security monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMonitoringConfiguration {
    /// Enable security event monitoring
    pub enable_monitoring: bool,
    /// Monitoring interval seconds
    pub monitoring_interval_seconds: u64,
    /// Alert configuration
    pub alerts: SecurityAlertConfiguration,
    /// Metrics collection
    pub metrics_enabled: bool,
    /// Integration with external SIEM
    pub siem_integration: Option<SiemIntegrationConfiguration>,
}

/// Security alert configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAlertConfiguration {
    /// Enable security alerts
    pub enabled: bool,
    /// Alert channels (email, slack, webhook, etc.)
    pub channels: Vec<String>,
    /// Alert severity levels
    pub severity_levels: HashMap<String, u8>,
    /// Alert rate limiting
    pub rate_limit_per_hour: u32,
}

/// SIEM integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiemIntegrationConfiguration {
    /// SIEM system type
    pub siem_type: String,
    /// Connection endpoint
    pub endpoint: String,
    /// Authentication credentials
    pub credentials: HashMap<String, String>,
    /// Event format
    pub event_format: String,
}

/// Rate limiting configuration (DEPRECATED - use canonical network config)
///
/// **MIGRATION**: Use `super::super::network::RateLimitConfig` instead.
///
/// This type alias will be removed in v3.3.0.
#[deprecated(
    since = "3.1.0",
    note = "Use super::super::network::RateLimitConfig instead"
)]
pub type RateLimitConfiguration = super::super::network::RateLimitConfig;
