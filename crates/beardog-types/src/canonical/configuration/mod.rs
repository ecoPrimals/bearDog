

pub mod integration;
pub mod monitoring;
pub mod network;
pub mod performance;
// Removed providers module - now using canonical::providers

pub mod security;
pub mod storage;
pub mod workflows;

pub mod compliance;
pub mod production;
pub mod encryption;

pub use integration::*;
pub use monitoring::{UnifiedMonitoringConfig, MetricCollectionConfig, AlertProcessingConfig, PrometheusConfig, SecurityMonitoringConfig, PerformanceMonitoringConfig};
pub use network::*;
pub use performance::*;
// Removed providers::* - now using canonical::providers
pub use security::*;
pub use storage::*;

pub use compliance::{ComplianceConfig, ComplianceStandard, ReportingConfig, PrivacyAuditConfig, DataSovereigntyConfig};
pub use production::{ProductionConfig, Environment, ClusterConfig, NodeConfig, BackupConfig, MaintenanceConfig, CircuitBreakerConfig, HealthMonitoringConfig, ResourceLimitsConfig};
pub use encryption::{EncryptionAlgorithm, EncryptionMode, KeyDerivationConfig, ContextAwareKeyConfig, EntropyAdjustmentConfig, KeyRotationConfig};

pub use workflows::{
    WorkflowConfig, WorkflowEngineConfig, WorkflowPolicyConfig, WorkflowProcessorConfig,
    WorkflowNotificationConfig, WorkflowRetryConfig, ZeroCostWorkflowConfig,
    WorkflowStorageConfig, WorkflowMonitoringConfig, ApprovalRequirements, ApprovalTier,
    WorkflowStorageBackend, WorkflowRetentionPolicy,
};

// Add consolidated adapter configurations
pub mod adapters;
pub use adapters::{
    BiomeOSConfig, SongBirdConfig, KubernetesConfig, VendorHsmConfig, 
    BearDogEcosystemConfig, UniversalAdapterConfig, ToadStoolClientConfig
};

// Add consolidated monitoring configurations  
pub mod monitoring_consolidated;
pub use monitoring_consolidated::{
    SecuritySentinelConfig, PrometheusConfigUnified, MetricCollectionConfigUnified,
    MonitoringConfigUnified
};

// Add routing and circuit breaker configurations
pub mod routing;
pub use routing::{
    RouterConfig, CircuitBreakerConfig as RoutingCircuitBreakerConfig, ModelConfig, CapabilityConfig, OAuth2Config
};

pub use crate::config::security_unified::UnifiedSecurityConfig as SecurityConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MfaMethod {

    Totp,

    Sms,

    Email,

    HardwareToken,

    Biometric,

    BackupCodes,
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct BearDogConfig {

    pub performance: PerformanceConfig,

    pub security: SecurityConfig,

    pub network: NetworkConfig,

    pub storage: StorageConfig,

    // Updated to use canonical provider config
    pub providers: Vec<super::providers::ProviderConfig>,

    pub workflows: WorkflowConfig,

    pub monitoring: UnifiedMonitoringConfig,

    pub compliance: ComplianceConfig,

    pub production: ProductionConfig,

    pub encryption: encryption::EncryptionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {

    pub name: String,

    pub version: String,

    pub environment: String,

    pub debug: bool,

    pub log_level: String,

    pub config_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeatureConfig {

    pub features: HashMap<String, bool>,

    pub rollout_percentages: HashMap<String, f64>,

    pub dependencies: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvironmentConfig {

    pub variables: HashMap<String, String>,

    pub overrides: HashMap<String, serde_json::Value>,

    pub validation_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[deprecated(since = "3.1.0", note = "Use UnifiedAuthConfig instead")]
pub struct SessionConfig {

    pub timeout: Duration,

    pub max_concurrent: u32,

    pub storage: String,

    pub max_failed_attempts: u32,

    pub lockout_duration: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RateLimitConfig {

    pub requests_per_minute: u32,

    pub burst_size: u32,

    pub window: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthenticationConfig {

    pub method: String,

    pub mfa: MfaConfig,

    pub password_policy: PasswordPolicyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EncryptionConfig {
    pub algorithm: String,
    pub key_size: u32,
    pub mode: String,
    pub padding: Option<String>,
    pub key_derivation_iterations: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MfaConfig {

    pub required: bool,

    pub methods: Vec<MfaMethod>,

    pub totp: TotpConfig,
}

// Unified configuration exports - use proper module paths
pub use crate::config::{
    UnifiedProcessorConfig, UnifiedRecoveryConfig, UnifiedTestingConfig, UnifiedAuthConfig,
    ProcessorType, RecoveryType, TestingType, AuthType,
};
