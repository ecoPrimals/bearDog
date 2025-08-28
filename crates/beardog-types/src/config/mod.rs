//! Configuration Module
//!
//! Re-exports canonical configuration for convenient access.
//! All configuration types are defined in `beardog_types::canonical::configuration::consolidated`.

// Re-export main canonical configuration types
pub use crate::canonical::configuration::consolidated::{
    AppConfig, BearDogCanonicalConfig, ComplianceConfig, ComplianceStandard, ConfigBuilder,
    ConfigManagerConfig, ConfigMigrator, ConfigValidator, DatabaseConfig, DiscoveryConfig,
    Environment, HsmConfig, HsmProvider, IntegrationConfig, LoadTestingConfig, LogLevel,
    MonitoringConfig, NetworkConfig, NodeRegistryConfig, PerformanceConfig, PlatformConfig,
    PlatformType, ProductionConfig, SecretsConfig, SecurityConfig, SecurityLevel, TunnelConfig,
    WorkflowConfig, WorkflowEngineType,
};

// Type aliases for unified configs (use the canonical ones)
pub type UnifiedNetworkConfig = NetworkConfig;
pub type UnifiedPerformanceConfig = PerformanceConfig;
pub type UnifiedMonitoringConfig = MonitoringConfig;
pub type UnifiedSecurityConfig = SecurityConfig;

// Re-export specific types to avoid ambiguous re-exports
pub use crate::canonical::configuration::{
    compliance::{DataSovereigntyConfig, PrivacyAuditConfig, ReportingConfig},
    // Workflow configs now in consolidated module - removed duplicate WorkflowConfig
    encryption::{
        ContextAwareKeyConfig, EncryptionAlgorithm, EncryptionMode, EntropyAdjustmentConfig,
        KeyDerivationConfig, KeyRotationConfig,
    },
    production::{MaintenanceConfig, NodeConfig, ResourceLimitsConfig},
};

// Migration utilities
pub mod migration;

// Alias for the main configuration type
pub type BearDogConfig = BearDogCanonicalConfig;
