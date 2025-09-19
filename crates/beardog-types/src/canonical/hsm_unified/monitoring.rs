// HSM Monitoring Configuration

use serde::{Deserialize, Serialize};

/// HSM monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HsmMonitoringConfig {
    /// Health checks enabled
    /// Whether `health_checks` is enabled
    pub health_checks_enabled: bool,

    /// Metrics collection enabled
    /// Whether metrics is enabled
    pub metrics_enabled: bool,

    /// Alerting enabled
    /// Whether alerting is enabled
    pub alerting_enabled: bool,
}
