// CANONICAL CONFIGURATION - SINGLE SOURCE OF TRUTH
// Core configuration modules - focused and maintainable
pub mod app;
pub mod builder;
pub mod core;
pub mod network;
pub mod validation;

// Specialized configuration modules
pub mod adapters;
pub mod compliance;
pub mod deployment;
pub mod encryption;
pub mod genetics;
pub mod monitoring_consolidated;
pub mod production;
pub mod routing;
pub mod tunnel;

// Legacy consolidated module (deprecated - use focused modules above)
pub mod consolidated;

// PRIMARY EXPORTS - Use these for new code
pub use app::{AppConfig, Environment, LogLevel};
pub use builder::ConfigBuilder;
pub use core::BearDogCanonicalConfig;
pub use network::{
    LoadBalancingConfig, LoadBalancingStrategy, NetworkConfig, NetworkProtocol, PortRange,
};
pub use validation::{ConfigMigrator, ConfigValidator};

pub use compliance::{
    ComplianceConfig, ComplianceStandard, DataSovereigntyConfig, PrivacyAuditConfig,
    ReportingConfig,
};

pub use encryption::{
    ContextAwareKeyConfig, EncryptionAlgorithm, EncryptionMode, EntropyAdjustmentConfig,
    KeyDerivationConfig, KeyRotationConfig,
};

pub use production::{
    BackupConfig, ClusterConfig, HealthMonitoringConfig, MaintenanceConfig, NodeConfig,
    ResourceLimitsConfig,
};

pub use adapters::{
    BearDogEcosystemConfig, BiomeOSConfig, KubernetesConfig, SongBirdConfig, ToadStoolClientConfig,
    UniversalAdapterConfig, VendorHsmConfig,
};

pub use monitoring_consolidated::{
    MetricCollectionConfigUnified, MonitoringConfigUnified, PrometheusConfigUnified,
    SecuritySentinelConfig,
};

pub use routing::{CapabilityConfig, ModelConfig, OAuth2Config, RouterConfig};

pub use tunnel::{
    TunnelAlertThresholds, TunnelAuthConfig, TunnelAuthMethod, TunnelConfig,
    TunnelConnectionConfig, TunnelFailoverConfig, TunnelGamingConfig, TunnelHealthConfig,
    TunnelHsmConfig, TunnelHsmManagerConfig, TunnelHsmPerformanceConfig, TunnelHsmTier,
    TunnelKeyManagementConfig, TunnelMonitoringConfig, TunnelPerformanceConfig,
    TunnelSecurityConfig,
};

pub use genetics::{
    EntropyCollectionMethod, EntropyPrivacyLevel, GenesisConfig, GeneticsConfig,
    GeneticsEntropyConfig, GeneticsNetworkConfig, GeneticsSpawningConfig, GeneticsSystemConfig,
};

pub use deployment::{
    AutoScalingConfig,
    CDNConfig,
    CDNProvider,
    CloudProvider,
    DeploymentConfig,
    DeploymentMonitoringConfig,
    DeploymentOptimizationConfig,
    DeploymentSecurityConfig,
    GeographicalLocation,
    GlobalDeploymentConfig,
    LoadBalancingConfig as DeploymentLoadBalancingConfig,
    NodeConfig as DeploymentNodeConfig, // Renamed to avoid conflict
    NodeType,
    RegionConfig,
    RegionalDeploymentConfig,
    RoutingAlgorithm,
};

// Re-export the canonical CircuitBreakerConfig
pub use crate::canonical::providers::CircuitBreakerConfig;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MfaMethod {
    Totp,
    Sms,
    Email,
    HardwareToken,
    Biometric,
    BackupCodes,
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
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_size: u32,
    pub window: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthenticationConfig {
    pub method: String,
    pub mfa: MfaConfig,
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
}

// Re-export canonical types from consolidated module
pub use consolidated::{
    AuthType, ProcessorType, RecoveryType, TestingType, UnifiedAuthConfig, UnifiedProcessorConfig,
    UnifiedRecoveryConfig, UnifiedTestingConfig,
};
