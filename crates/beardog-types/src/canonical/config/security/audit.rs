// Audit Configuration
//
// Canonical audit configuration for logging, compliance, and security event tracking.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **CANONICAL AUDIT CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalAuditConfig {
    /// Enable audit logging
    /// Whether feature is enabled
    pub enabled: bool,

    /// Audit log level
    /// The log level value
    pub log_level: String,

    pub log_format: String,

    /// Audit log destination
    /// The log destination value
    pub log_destination: String,

    /// Enable real-time alerting
    /// Whether `enable_alerts` is enabled
    pub enable_alerts: bool,

    /// Alert thresholds
    /// Mapping of alert thresholds
    pub alert_thresholds: HashMap<String, u32>,

    /// Retention policy in days
    /// Number of `retention_days`
    pub retention_days: u32,

    /// Enable compliance reporting
    /// Whether `enable_compliance_reporting` is enabled
    pub enable_compliance_reporting: bool,

    /// Compliance frameworks
    /// Collection of compliance frameworks
    pub compliance_frameworks: Vec<String>,
}

impl Default for CanonicalAuditConfig {
    fn default() -> Self {
        let mut alert_thresholds = HashMap::new();
        alert_thresholds.insert("failed_logins".to_string(), 5);
        alert_thresholds.insert("privilege_escalation".to_string(), 1);

        Self {
            enabled: true,
            log_level: "INFO".to_string(),
            log_format: "JSON".to_string(),
            log_destination: "file".to_string(),
            enable_alerts: true,
            alert_thresholds,
            retention_days: 365,
            enable_compliance_reporting: true,
            compliance_frameworks: vec!["SOC2".to_string(), "GDPR".to_string()],
        }
    }
}

impl CanonicalAuditConfig {
    /// Production
    #[must_use]
    pub fn production() -> Self {
        Self {
            retention_days: 2555, // 7 years for production compliance
            log_destination: "syslog".to_string(),
            ..Self::default()
        }
    }

    /// Validate
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.enabled && self.retention_days == 0 {
            return Err(BearDogError::security(
                "Audit retention days must be greater than 0".to_string(),
            ));
        }
        Ok(())
    }
}
