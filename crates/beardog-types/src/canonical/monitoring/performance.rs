// Unified Performance Monitoring Configuration

use super::MonitoringConfigValidation;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedPerformanceMonitoringConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Cpu Monitoring
    /// Whether `cpu_monitoring` is enabled
    pub cpu_monitoring: bool,
    /// Memory Monitoring
    /// Whether `memory_monitoring` is enabled
    pub memory_monitoring: bool,
    /// Network Monitoring
    /// Whether `network_monitoring` is enabled
    pub network_monitoring: bool,
    /// Profiling Enabled
    /// Whether profiling is enabled
    pub profiling_enabled: bool,
}

impl Default for UnifiedPerformanceMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cpu_monitoring: true,
            memory_monitoring: true,
            network_monitoring: true,
            profiling_enabled: false,
        }
    }
}

impl MonitoringConfigValidation for UnifiedPerformanceMonitoringConfig {
    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        Ok(())
    }
    /// Checks if compatible with
    fn is_compatible_with(&self, _other_version: u32) -> bool {
        true
    }
}
