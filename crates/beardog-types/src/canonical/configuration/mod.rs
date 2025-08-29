// CONSOLIDATED CONFIGURATION - SINGLE SOURCE OF TRUTH
pub mod consolidated;

// Core configuration modules (kept for specific functionality)
pub mod compliance;
pub mod encryption;
pub mod production;

// Consolidated monitoring and adapter modules
pub mod adapters;
pub mod monitoring_consolidated;
pub mod routing;
pub mod tunnel;
pub mod genetics;
pub mod deployment;

// PRIMARY EXPORTS - Use these for new code
pub use consolidated::{
    AppConfig, BearDogCanonicalConfig, ConfigBuilder, ConfigMigrator, ConfigValidator,
    DatabaseConfig, Environment, HsmConfig, HsmProvider, LogLevel, MonitoringConfig, NetworkConfig,
    PerformanceConfig, ProductionConfig, SecurityConfig, WorkflowConfig, WorkflowEngineType,
};

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
    TunnelConfig, TunnelPerformanceConfig, TunnelKeyManagementConfig, TunnelGamingConfig,
    TunnelHsmManagerConfig, TunnelHealthConfig, TunnelFailoverConfig, TunnelHsmPerformanceConfig,
    TunnelHsmConfig, TunnelHsmTier, TunnelConnectionConfig, TunnelAuthConfig, TunnelAuthMethod,
    TunnelMonitoringConfig, TunnelAlertThresholds, TunnelSecurityConfig,
};

pub use genetics::{
    GeneticsConfig, GenesisConfig, GeneticsNetworkConfig, GeneticsSystemConfig,
    GeneticsSpawningConfig, GeneticsEntropyConfig, EntropyCollectionMethod, EntropyPrivacyLevel,
};

pub use deployment::{
    DeploymentConfig, GlobalDeploymentConfig, DeploymentOptimizationConfig,
    RegionalDeploymentConfig, RegionConfig, CloudProvider, GeographicalLocation,
    LoadBalancingConfig, RoutingAlgorithm, CDNConfig, CDNProvider, AutoScalingConfig,
    DeploymentMonitoringConfig, DeploymentSecurityConfig, NodeType,
    NodeConfig as DeploymentNodeConfig, // Renamed to avoid conflict
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
