//! Configuration Module
//!
//! Re-exports canonical configuration for convenient access.
//! All configuration types are defined in `beardog_types::canonical::configuration::consolidated`.

// Re-export main canonical configuration types from focused modules
pub use crate::canonical::configuration::{
    AppConfig, BearDogCanonicalConfig, ConfigBuilder, ConfigMigrator, ConfigValidator, Environment,
    LoadBalancingConfig, LoadBalancingStrategy, LogLevel, NetworkConfig, NetworkProtocol,
    PortRange,
};

// Re-export from legacy consolidated module for compatibility
pub use crate::canonical::configuration::consolidated::{
    ComplianceConfig, ComplianceStandard, ConfigManagerConfig, DatabaseConfig, DiscoveryConfig,
    HsmConfig, HsmProvider, IntegrationConfig, LoadTestingConfig, MonitoringConfig,
    NodeRegistryConfig, PerformanceConfig, PlatformConfig, PlatformType, ProductionConfig,
    SecretsConfig, SecurityConfig, SecurityLevel, TunnelConfig, WorkflowConfig, WorkflowEngineType,
};

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
