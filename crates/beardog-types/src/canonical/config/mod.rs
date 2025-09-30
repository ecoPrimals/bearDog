// Canonical Configuration System
//
// This module provides the **single source of truth** for all configuration types
// across the BearDog ecosystem. It consolidates 50+ fragmented Config structs
// into a unified, maintainable system.
//
// ## Unification Strategy
//
// This module replaces fragmented configurations from:
// - `beardog-types/src/configuration/` - Legacy configuration types
// - `beardog-types/src/production/` - Production-specific configs
// - Individual crate configurations scattered across 22 crates
//
// ## Modern Architecture Principles
//
// - **Single Source of Truth**: All config types in one canonical location
// - **Domain Organization**: Logical grouping by functional area
// - **Zero Fragmentation**: No duplicate config types across crates
// - **Compatibility Preservation**: Maintains existing APIs during migration
// - **Type Safety**: Strongly typed configuration with validation
// - **Performance Optimized**: Zero-cost abstractions and efficient loading

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
// use std::collections::HashMap; // Unused import

// Domain-specific configuration modules
/// App module
pub mod app;
/// Auth module
pub mod auth;
/// Cache module
pub mod cache;
/// Compliance module
pub mod compliance;
/// Database module
pub mod database;
/// Genetics module
pub mod genetics;
/// Hsm module
pub mod hsm;
/// Monitoring module
pub mod monitoring;
/// Monitoring Migration module
pub mod monitoring_migration; // MONITORING MIGRATION UTILITY - Consolidates fragmented monitoring configs
/// Network module
pub mod network;
pub mod performance;
/// Production module
pub mod production;
/// Security module
pub mod security;
/// Workflow module
pub mod workflow;

// Domain configuration submodules (consolidated types)
/// Domains module containing consolidated configuration types
pub mod domains;

// UNIFIED CONFIGURATION SYSTEM - Single source of truth
/// Type Aliases module
pub mod type_aliases;
/// Unified configuration trait and validation utilities
#[path = "trait.rs"]
pub mod r#trait;
/// Unified module
pub mod unified;
/// Unified Simple module
pub mod unified_simple; // WORKING IMPLEMENTATION // PEDANTIC: Type aliases for missing config types
/// Unified trait module (DEPRECATED - use r#trait)
#[deprecated(since = "3.1.0", note = "Use config::r#trait module instead")]
pub mod unified_trait;

// MODULAR PRODUCTION CONFIGURATION - Split from oversized production.rs
// This replaces the 1,436-line production.rs file with focused domain modules

// Re-export canonical config types (avoiding naming conflicts)
pub use app::{AppConfig, ApplicationConfig, CanonicalAppConfig};
pub use auth::*;
pub use cache::*;
pub use compliance::*;
pub use database::*;
pub use genetics::*;
pub use hsm::{UnifiedHsmConfig as ConfigHsmConfig, UnifiedHsmConfig as ConfigHsm};
pub use monitoring::{
    UnifiedAlertingConfig,
    // ProductionMonitoringConfig, // Removed - now part of UnifiedMonitoringConfig
    UnifiedHealthConfig,
    UnifiedLoggingConfig,
    UnifiedMetricsConfig,
    UnifiedMonitoringConfig,
    UnifiedTracingConfig,
};
pub use monitoring_migration::{
    create_configuration_legacy_monitoring, migrate_monitoring_configurations,
    LegacyMonitoringConfig, MonitoringMigrationReport, MonitoringMigrationResult,
    MonitoringMigrationService,
};
pub use network::{
    CanonicalNetworkConfig as ConfigNetworkConfig, EndpointConfig, NetworkConfig as ConfigNetwork,
};
pub use performance::*;
pub use production::*;
pub use production::{EnvironmentLevel, EnvironmentType}; // Explicit re-export for compatibility
pub use security::CanonicalSecurityConfig as ConfigSecurityConfig;
pub use workflow::*;

// Re-export with canonical names to avoid conflicts
pub use hsm::UnifiedHsmConfig;
// UnifiedMonitoringConfig already exported above
pub use network::CanonicalNetworkConfig;
pub use security::CanonicalSecurityConfig;

// Re-export the UNIFIED CONFIGURATION TRAIT SYSTEM
pub use r#trait::{
    BearDogConfig, ConfigBuilder, ConfigLoader, ConfigMetadata, ConfigSource, ValidationStatus,
    validation,
};

// Re-export the UNIFIED CONFIGURATION SYSTEM as primary interface
pub use unified::{
    // Comprehensive configuration
    UnifiedBearDogConfig,
    // Simplified configuration (NEW - merged from unified_simple.rs)
    SimplifiedBearDogConfig,
    // Supporting types
    DeploymentMode,
    Environment,
    LogLevel,
    PasswordSource,
    RolloutConfig,
    RolloutStrategy,
    SystemMetadata,
    UnifiedAppConfig,
    UnifiedDatabaseConfig,
    UnifiedGeneticsConfig,
    UnifiedNetworkConfig,
    UnifiedSecurityConfig,
    UnifiedVersionInfo,
    // Simplified config types (NEW - merged from unified_simple.rs)
    NetworkSettings,
    SecuritySettings,
    DatabaseSettings,
    MonitoringSettings,
    PerformanceSettings,
};

// Backward compatibility aliases
pub use unified::SimplifiedBearDogConfig as WorkingUnifiedConfig;
pub use UnifiedBearDogConfig as MasterUnifiedBearDogConfig;

// DEPRECATED: unified_simple module (functionality moved to unified.rs)
#[deprecated(since = "3.1.0", note = "Use unified module instead - types moved to unified.rs")]
pub use unified_simple::{
    ConfigurationMigrator,
};

/// Master configuration that unifies all domain configurations
///
/// It consolidates all domain-specific configurations into one coherent structure.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BearDogMasterConfig {
    /// Application configuration
    /// The app value
    pub app: CanonicalAppConfig,

    /// Network configuration
    /// The network value
    pub network: CanonicalNetworkConfig,

    /// Security configuration
    /// The security value
    pub security: CanonicalSecurityConfig,

    /// Monitoring configuration
    /// The monitoring value
    pub monitoring: UnifiedMonitoringConfig,

    /// HSM configuration
    /// The hsm value
    pub hsm: UnifiedHsmConfig,

    /// Production configuration
    /// The production value
    pub production: CanonicalProductionConfig,

    pub performance: CanonicalPerformanceConfig,

    /// Authentication configuration
    /// The auth value
    pub auth: CanonicalAuthConfig,

    /// Compliance configuration
    /// The compliance value
    pub compliance: CanonicalComplianceConfig,

    /// Genetics configuration
    /// The genetics value
    pub genetics: CanonicalGeneticsConfig,

    /// Workflow configuration
    /// The workflow value
    pub workflow: CanonicalWorkflowConfig,

    /// Cache configuration
    /// The cache value
    pub cache: CanonicalCacheConfig,

    /// Database configuration
    /// The database value
    pub database: CanonicalDatabaseConfig,
}

impl BearDogMasterConfig {
    /// Create a new master configuration with sensible defaults
    #[must_use]
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Load configuration from environment variables with BEARDOG_ prefix
    /// Creates instance from env
    pub fn from_env() -> Result<Self, BearDogError> {
        let mut config = Self::default();

        // Load environment-specific overrides
        if let Ok(env) = std::env::var("BEARDOG_ENVIRONMENT") {
            config.app.environment = env;
        }

        if let Ok(debug) = std::env::var("BEARDOG_DEBUG") {
            config.app.debug = debug.parse().unwrap_or(false);
        }

        if let Ok(port) = std::env::var("BEARDOG_PORT") {
            if let Ok(port_num) = port.parse::<u16>() {
                config.network.port = port_num;
            }
        }

        config.validate()?;
        Ok(config)
    }

    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        // Validate each domain configuration
        self.validate_app()?;
        self.validate_network()?;
        self.validate_security()?;
        self.validate_monitoring()?;
        self.validate_hsm()?;
        self.validate_production()?;
        self.validate_performance()?;
        self.validate_auth()?;
        self.validate_compliance()?;
        self.validate_genetics()?;
        self.validate_workflow()?;
        self.validate_cache()?;
        self.validate_database()?;

        // Cross-domain validation
        self.validate_cross_domain()?;

        Ok(())
    }

    /// Merge another configuration into this one, with the other taking precedence
    pub fn merge(&mut self, other: Self) -> Result<(), BearDogError> {
        // Only merge non-default values to preserve intentional settings
        if !other.app.name.is_empty() {
            self.app = other.app;
        }
        if other.network.port != 0 {
            self.network = other.network;
        }
        if other.security.enable_encryption || other.security.enable_hsm {
            self.security = other.security;
        }
        // ... continue for other domains

        self.validate()?;
        Ok(())
    }

    #[must_use]
    pub fn summary(&self) -> ConfigurationSummary {
        ConfigurationSummary {
            environment: self.app.environment.clone(),
            debug_mode: self.app.debug,
            network_enabled: self.network.enabled,
            security_level: if self.security.enable_encryption {
                "High"
            } else {
                "Standard"
            }
            .to_string(),
            monitoring_enabled: self.monitoring.enabled,
            hsm_enabled: self.hsm.enabled,
            production_mode: matches!(
                self.production.environment.environment_type,
                EnvironmentType::Production
            ),
            total_domains: 13,
        }
    }

    // Individual domain validation methods
    /// Validates app
    fn validate_app(&self) -> Result<(), BearDogError> {
        if self.app.name.is_empty() {
            return Err(BearDogError::validation("Application name cannot be empty"));
        }
        if self.app.version.is_empty() {
            return Err(BearDogError::validation(
                "Application version cannot be empty",
            ));
        }
        Ok(())
    }

    /// Validates network
    fn validate_network(&self) -> Result<(), BearDogError> {
        if self.network.enabled && self.network.port == 0 {
            return Err(BearDogError::validation(
                "Network port must be specified when network is enabled",
            ));
        }
        if self.network.max_connections == 0 {
            return Err(BearDogError::validation(
                "Max connections must be greater than 0",
            ));
        }
        Ok(())
    }

    /// Validates security
    fn validate_security(&self) -> Result<(), BearDogError> {
        if (self.security.enable_encryption || self.security.enable_hsm)
            && !self.security.authentication.jwt_secret.is_empty()
            && self.security.authentication.jwt_expiration_seconds == 0
        {
            return Err(BearDogError::validation(
                "At least one authentication method must be specified",
            ));
        }
        Ok(())
    }

    /// Validates monitoring
    fn validate_monitoring(&self) -> Result<(), BearDogError> {
        if self.monitoring.enabled
            && !self.monitoring.metrics.enabled
            && !self.monitoring.logging.enabled
        {
            return Err(BearDogError::validation(
                "At least metrics or logging must be enabled when monitoring is enabled",
            ));
        }
        Ok(())
    }

    /// Validates hsm
    fn validate_hsm(&self) -> Result<(), BearDogError> {
        // HSM validation updated for new unified structure
        if self.hsm.enabled
            && self.hsm.hardware.providers.is_empty()
            && !self.hsm.software.enabled
            && !self.hsm.mobile.enabled
            && !self.hsm.cloud.enabled
        {
            return Err(BearDogError::validation(
                "At least one HSM provider must be configured when HSM is enabled",
            ));
        }
        Ok(())
    }

    /// Validates production
    fn validate_production(&self) -> Result<(), BearDogError> {
        if matches!(
            self.production.environment.environment_type,
            EnvironmentType::Production
        ) {
            if !self.security.enable_encryption && !self.security.enable_hsm {
                return Err(BearDogError::validation(
                    "Security must be enabled in production environment",
                ));
            }
            if !self.monitoring.enabled {
                return Err(BearDogError::validation(
                    "Monitoring must be enabled in production environment",
                ));
            }
        }
        Ok(())
    }

    fn validate_performance(&self) -> Result<(), BearDogError> {
        if self.performance.enabled && self.performance.resource_limits.max_memory_mb == 0 {
            return Err(BearDogError::validation(
                "Memory limit must be specified when performance optimization is enabled",
            ));
        }
        Ok(())
    }

    /// Validates auth
    fn validate_auth(&self) -> Result<(), BearDogError> {
        if self.auth.enabled && self.auth.providers.is_empty() {
            return Err(BearDogError::validation(
                "At least one auth provider must be specified",
            ));
        }
        Ok(())
    }

    /// Validates compliance
    fn validate_compliance(&self) -> Result<(), BearDogError> {
        if self.compliance.status == crate::canonical::config::compliance::ComplianceStatus::Enabled
            && self.compliance.data_retention_days == 0
        {
            return Err(BearDogError::validation(
                "Data retention period must be specified for compliance",
            ));
        }
        Ok(())
    }

    /// Validates genetics
    fn validate_genetics(&self) -> Result<(), BearDogError> {
        if self.genetics.enabled && self.genetics.population_size == 0 {
            return Err(BearDogError::validation(
                "Population size must be greater than 0",
            ));
        }
        Ok(())
    }

    /// Validates workflow
    fn validate_workflow(&self) -> Result<(), BearDogError> {
        if self.workflow.enabled && self.workflow.max_concurrent_workflows == 0 {
            return Err(BearDogError::validation(
                "Max concurrent workflows must be greater than 0",
            ));
        }
        Ok(())
    }

    /// Validates cache
    fn validate_cache(&self) -> Result<(), BearDogError> {
        if self.cache.enabled && self.cache.max_size_mb == 0 {
            return Err(BearDogError::validation(
                "Cache size must be specified when caching is enabled",
            ));
        }
        Ok(())
    }

    /// Validates database
    fn validate_database(&self) -> Result<(), BearDogError> {
        if self.database.enabled && self.database.connection_string.is_empty() {
            return Err(BearDogError::validation(
                "Database connection string cannot be empty",
            ));
        }
        Ok(())
    }

    /// Validates `cross_domain`
    fn validate_cross_domain(&self) -> Result<(), BearDogError> {
        // Cross-domain validation logic
        if (self.security.enable_encryption || self.security.enable_hsm)
            && self.network.enabled
            && !self.network.tls.enabled
        {
            return Err(BearDogError::validation(
                "TLS must be enabled when both security and network are enabled",
            ));
        }

        if self.hsm.enabled && !self.security.enable_encryption && !self.security.enable_hsm {
            return Err(BearDogError::validation(
                "Security must be enabled when HSM is enabled",
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationSummary {
    /// Current deployment environment (dev, staging, prod)
    /// The environment value
    pub environment: String,
    /// Whether debug mode is enabled
    /// Whether `debug_mode` is enabled
    pub debug_mode: bool,
    /// Whether network functionality is enabled
    /// Whether network is enabled
    pub network_enabled: bool,
    /// Current security level setting
    /// The security level value
    pub security_level: String,
    /// Whether monitoring is enabled
    /// Whether monitoring is enabled
    pub monitoring_enabled: bool,
    /// Whether HSM functionality is enabled
    /// Whether hsm is enabled
    pub hsm_enabled: bool,
    /// Whether running in production mode
    /// Whether `production_mode` is enabled
    pub production_mode: bool,
    /// Total number of configured domains
    /// Number of `total_domains`
    pub total_domains: usize,
}

// Compatibility aliases for migration (DEPRECATED - use actual types from unified module)
#[deprecated(since = "3.1.0", note = "Use BearDogMasterConfig directly")]
pub type MasterConfig = BearDogMasterConfig;

#[deprecated(since = "3.1.0", note = "Use BearDogMasterConfig directly")]
pub type GlobalConfig = BearDogMasterConfig;
