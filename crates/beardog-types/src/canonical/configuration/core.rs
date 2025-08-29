//! Core Configuration Module
//!
//! Contains the main BearDogCanonicalConfig struct and core configuration types.
//! This is the primary entry point for all BearDog configuration.

use serde::{Deserialize, Serialize};

// Import all the specific config types from consolidated module for now
use super::consolidated::{
    HsmConfig, DatabaseConfig, MonitoringConfig, WorkflowConfig, ProductionConfig,
    PerformanceConfig, ConfigManagerConfig, DiscoveryConfig, ComplianceConfig,
    TunnelConfig, IntegrationConfig, NodeRegistryConfig, PlatformConfig,
    LoadTestingConfig, SecretsConfig, TestingConfig, SecurityConfig,
};

// Import from focused modules
use super::{
    app::AppConfig,
    network::NetworkConfig,
};

/// **THE** canonical BearDog configuration struct
/// Replaces: BearDogCanonicalConfig, UnifiedDeploymentConfig, BiomeConfig, etc.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BearDogCanonicalConfig {
    /// Core application configuration
    pub app: AppConfig,

    /// Network and connectivity settings
    pub network: NetworkConfig,

    /// Security and authentication settings
    pub security: SecurityConfig,

    /// HSM and cryptographic configuration
    pub hsm: HsmConfig,

    /// Database and storage configuration
    pub database: DatabaseConfig,

    /// Monitoring and observability configuration
    pub monitoring: MonitoringConfig,

    /// Workflow engine configuration
    pub workflows: WorkflowConfig,

    /// Production and deployment settings
    pub production: ProductionConfig,

    /// Performance tuning configuration
    pub performance: PerformanceConfig,

    /// Configuration management settings
    pub config_manager: ConfigManagerConfig,

    /// Service discovery configuration
    pub discovery: DiscoveryConfig,

    /// Compliance and audit configuration
    pub compliance: ComplianceConfig,

    /// Tunnel and VPN configuration
    pub tunnel: TunnelConfig,

    /// Integration with external systems
    pub integration: IntegrationConfig,

    /// Node registry configuration
    pub node_registry: NodeRegistryConfig,

    /// Platform-specific settings
    pub platform: PlatformConfig,

    /// Load testing configuration
    pub load_testing: LoadTestingConfig,

    /// Secrets management configuration
    pub secrets: SecretsConfig,

    /// Testing and development configuration
    pub testing: TestingConfig,
}

impl BearDogCanonicalConfig {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        super::validation::ConfigValidator::validate(self)
    }

    /// Create a configuration builder
    pub fn builder() -> super::builder::ConfigBuilder {
        super::builder::ConfigBuilder::new()
    }
} 