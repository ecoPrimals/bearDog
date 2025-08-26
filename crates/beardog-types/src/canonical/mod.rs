

use serde::{Deserialize, Serialize};

pub mod capabilities;
pub mod configuration;
pub mod constants;
pub mod crypto;
pub mod genetics;
pub mod health_status;
pub mod hsm;
pub mod metrics;
pub mod monitoring;
pub mod network;
pub mod providers;
pub mod security;
pub mod services;
pub mod workflow;

pub use capabilities::{
    CapabilityRequirements, HumanEntropyCapabilities, PerformanceCapabilities,
    SecurityLevel as CapabilitiesSecurityLevel,
};

pub use configuration::{
    BearDogConfig,
    DatabaseConfig,
    NetworkSecurityConfig,
    SecurityConfig,
};

pub use genetics::*;
pub use crate::constants::unified::network::limits::MAX_CONNECTIONS;
pub use crypto::{KeyType, EncryptionAlgorithm, KeyUsage, CryptoParams};
pub use health_status::*;
pub use hsm::{HsmCapabilities, HsmKey, KeyMetadata};

pub use metrics::*;
pub use monitoring::{
    AlertConfig, AlertSeverity, HealthCheckConfig, HealthCheckResult, MonitoringMetrics,
    NetworkUsage, RequestMetrics, ServiceHealthMonitor, MonitoringConfig,
    MetricsConfig, LoggingConfig, TracingConfig, PrometheusConfig,
    SecurityMonitoringConfig, PerformanceMonitoringConfig, IntegrationMonitoringConfig,
    PerformanceThresholds, ThreatDetectionConfig, SensitivityLevel,
};

pub use services::*;

pub use network::{
    CircuitBreakerConfig, ConnectionPoolConfig as NetworkConnectionPoolConfig,
    FailoverConfig as NetworkFailoverConfig, HealthCheckConfig as NetworkHealthCheckConfig,
    LoadBalancingConfig as NetworkLoadBalancingConfig, ServiceDiscoveryConfig,
};

pub use providers::{
    ProviderCapability, ProviderConfig, ProviderHealth,
    ProviderRegistryEntry, ProviderStatus, ProviderType,
    UniversalAdapterConfig, TargetPrimalConfig, ExternalServicesConfig,
    HsmKeyInfo, HsmHardwareStatus, HsmInfo,
    TimeoutConfig, RetryConfig,
};
pub use security::*;

pub use workflow::WorkflowRetryConfig as WorkflowRetryConfiguration;

pub struct CanonicalTypeRegistry;
impl CanonicalTypeRegistry {

    #[must_use]
    pub fn status_types() -> Vec<&'static str> {
        vec![
            "HealthStatus",
            "ComponentStatus",
            "OperationStatus",
            "WorkflowStatus",
            "KeyStatus",
        ]
    }

    pub fn config_types() -> Vec<&'static str> {
        vec![
            "SessionConfig",
            "RateLimitConfig",
            "AuthenticationConfig",
            "PasswordPolicyConfig",
            "MfaConfig",
            "EncryptionConfig",
            "ProviderConfig",
        ]
    }

    pub fn is_canonical_type(type_name: &str) -> bool {
        Self::status_types().contains(&type_name)
            || Self::config_types().contains(&type_name)
            || matches!(
                type_name,
                "SecurityContext"
                    | "SecurityAuditEvent"
                    | "PolicyDecision"
                    | "AuthorizationLevel"
                    | "SecurityFlags"
                    | "RiskLevel"
            )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCapabilities {

    pub authentication_methods: Vec<String>,

    pub rbac: bool,

    pub audit_logging: bool,

    pub secure_protocols: Vec<String>,

    pub compliance_certifications: Vec<String>,

    pub security_level: String,
}

impl Default for SecurityCapabilities {
    fn default() -> Self {
        Self {
            authentication_methods: vec!["password".to_string(), "mfa".to_string()],
            rbac: true,
            audit_logging: true,
            secure_protocols: vec!["TLS".to_string(), "HTTPS".to_string()],
            compliance_certifications: vec!["ISO27001".to_string()],
            security_level: "high".to_string(),
        }
    }
}
