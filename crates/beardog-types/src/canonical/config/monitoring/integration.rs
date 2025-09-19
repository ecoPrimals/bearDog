// Unified Integration Monitoring Configuration

use super::MonitoringConfigValidation;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedIntegrationMonitoringConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Api Monitoring
    /// Whether `api_monitoring` is enabled
    pub api_monitoring: bool,
    /// Service Discovery Monitoring
    /// Whether `service_discovery_monitoring` is enabled
    pub service_discovery_monitoring: bool,
    /// Ecosystem Health
    /// Whether `ecosystem_health` is enabled
    pub ecosystem_health: bool,
}

impl Default for UnifiedIntegrationMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            api_monitoring: true,
            service_discovery_monitoring: true,
            ecosystem_health: true,
        }
    }
}

impl MonitoringConfigValidation for UnifiedIntegrationMonitoringConfig {
    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        Ok(())
    }
    /// Checks if compatible with
    fn is_compatible_with(&self, _other_version: u32) -> bool {
        true
    }
}
