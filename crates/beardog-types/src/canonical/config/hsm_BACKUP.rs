// Canonical HSM Configuration

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Canonical HSM configuration - consolidates all HSM-related configs
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalHsmConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Provider
    pub provider: HsmProviderConfig,
    /// Connection
    /// The connection value
    pub connection: HsmConnectionConfig,
    /// Security
    /// The security value
    pub security: HsmSecurityConfig,
    pub performance: HsmPerformanceConfig,
    /// Monitoring
    /// The monitoring value
    pub monitoring: HsmMonitoringConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HsmProviderConfig {
    /// Provider Type
    pub provider_type: HsmProviderType,
    /// Endpoint
    /// Optional endpoint
    pub endpoint: Option<String>,
    /// Credentials
    /// Optional credentials
    pub credentials: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
/// Types of hsm provider
pub enum HsmProviderType {
    #[default]
    /// Represents software variant
    Software,
    /// Hardware variant
    Hardware,
    /// Cloud variant
    Cloud,
    /// Mobile variant
    Mobile,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HsmConnectionConfig {
    /// Timeout
    pub timeout: Duration,
    /// Max Connections
    /// Number of max_connections
    pub max_connections: u32,
    /// Retry Attempts
    /// Number of retry_attempts
    pub retry_attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HsmSecurityConfig {
    /// Authentication Required
    /// Whether authentication_required is enabled
    pub authentication_required: bool,
    /// Access Control Enabled
    /// Whether access_control is enabled
    pub access_control_enabled: bool,
    /// Audit Logging Enabled
    /// Whether audit_logging is enabled
    pub audit_logging_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HsmPerformanceConfig {
    /// Batch Operations Enabled
    /// Whether batch_operations is enabled
    pub batch_operations_enabled: bool,
    /// Connection Pooling Enabled
    /// Whether connection_pooling is enabled
    pub connection_pooling_enabled: bool,
    /// Cache Enabled
    /// Whether cache is enabled
    pub cache_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HsmMonitoringConfig {
    /// Health Checks Enabled
    /// Whether health_checks is enabled
    pub health_checks_enabled: bool,
    /// Metrics Enabled
    /// Whether metrics is enabled
    pub metrics_enabled: bool,
    /// Alerting Enabled
    /// Whether alerting is enabled
    pub alerting_enabled: bool,
}

// Compatibility aliases
pub type HsmConfig = CanonicalHsmConfig;
