// SPDX-License-Identifier: AGPL-3.0-or-later

// Unified Security Monitoring Configuration

use super::MonitoringConfigValidation;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// Unified Security Monitoring Configuration
///
/// ecosystem, including threat detection, audit logging, and compliance monitoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedSecurityMonitoringConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Threat Detection
    /// Whether `threat_detection` is enabled
    pub threat_detection: bool,
    /// Audit Logging
    /// Whether `audit_logging` is enabled
    pub audit_logging: bool,
    /// Compliance Monitoring
    /// Whether `compliance_monitoring` is enabled
    pub compliance_monitoring: bool,
}

impl Default for UnifiedSecurityMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            threat_detection: true,
            audit_logging: true,
            compliance_monitoring: true,
        }
    }
}

impl MonitoringConfigValidation for UnifiedSecurityMonitoringConfig {
    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        Ok(())
    }
    /// Checks if compatible with
    fn is_compatible_with(&self, _other_version: u32) -> bool {
        true
    }
}

// Deprecated ThreatDetectionConfig and SensitivityLevel removed
// Use beardog_types::canonical::config::domains::threat:: instead
