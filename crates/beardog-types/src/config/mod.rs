

pub mod performance;

pub mod monitoring;

pub mod security_unified;

pub mod network_unified;

pub mod app;

pub mod database;

pub mod compliance;

pub mod discovery;

pub mod load_testing;

pub mod manager;

pub mod node_registry;

pub mod platform;

pub mod production;

pub mod secrets;

pub mod tunnel;

pub mod integration;

pub mod migration;

pub mod canonical_consolidation;

// New unified consolidation module
pub mod unified_consolidation;

pub use integration::IntegrationConfig;
pub use network_unified::UnifiedNetworkConfig;
pub use performance::GeneralPerformanceConfig as UnifiedPerformanceConfig;
pub use monitoring::BasicMonitoringConfig as UnifiedMonitoringConfig;
pub use production::UnifiedProductionConfig as ProductionConfig;
pub use security_unified::UnifiedSecurityConfig;

pub use app::AppConfig;
pub use database::UnifiedDatabaseConfig as DatabaseConfig;
pub use compliance::UnifiedComplianceConfig as ComplianceConfig;
pub use discovery::UnifiedDiscoveryConfig as DiscoveryConfig;
pub use load_testing::UnifiedLoadTestConfig as LoadTestingConfig;
pub use manager::UnifiedConfigManager as ConfigManagerConfig;
pub use node_registry::UnifiedNodeRegistryConfig as NodeRegistryConfig;
pub use platform::UnifiedPlatformConfig as PlatformConfig;

pub use canonical_consolidation::{
    CanonicalHsmConfig, CanonicalTunnelConfig, CanonicalAdapterConfig,
    ConfigurationConsolidator, ConsolidationReport
};

// New unified consolidation exports
pub use unified_consolidation::{
    UnifiedProcessorConfig, UnifiedRecoveryConfig, UnifiedTestingConfig, UnifiedAuthConfig,
    ProcessorType, RecoveryType, TestingType, AuthType,
    ProcessorCoreConfig, ProcessorSecurityConfig,
    RecoveryCoreConfig, BackupRecoveryConfig, RecoverySecurityConfig,
    TestingCoreConfig, TestingSecurityConfig,
    AuthCoreConfig, AuthSecurityConfig,
    UserManagementProcessorConfig, EmailNotificationSettings,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[derive(Default)]
pub struct BearDogCanonicalConfig {

    pub app: AppConfig,

    pub performance: UnifiedPerformanceConfig,

    pub security: UnifiedSecurityConfig,

    pub database: DatabaseConfig,

    pub monitoring: UnifiedMonitoringConfig,

    pub network: UnifiedNetworkConfig,

    pub production: ProductionConfig,

    pub integration: IntegrationConfig,

    pub tunnel: tunnel::UnifiedTunnelConfig,
    
    // New consolidated configurations
    pub processors: Option<UnifiedProcessorConfig>,
    
    pub recovery: Option<UnifiedRecoveryConfig>,
    
    pub testing: Option<UnifiedTestingConfig>,
    
    pub auth: Option<UnifiedAuthConfig>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConfigPerformanceMetrics;

impl ConfigPerformanceMetrics {

    pub fn validate_performance(_config: &BearDogCanonicalConfig) -> Vec<String> {

        vec![]
    }

    pub fn get_optimization_suggestions(_config: &BearDogCanonicalConfig) -> Vec<String> {

        vec![]
    }
}
