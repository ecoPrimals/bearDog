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

// BearDogError no longer needed in this module after BearDogMasterConfig removal
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
/// 
/// NOTE: bootstrap.rs exists but needs proper module system integration
/// For now, domains/ uses simple directory structure
pub mod domains;

// UNIFIED CONFIGURATION SYSTEM - Single source of truth
/// Type Aliases module
pub mod type_aliases;
/// Unified configuration trait and validation utilities
#[path = "trait.rs"]
pub mod r#trait;
/// Unified module
pub mod unified;
// unified_simple module REMOVED in Phase 2 (October 2025)
// Functionality merged into unified.rs - use SimplifiedBearDogConfig
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
    CanonicalNetworkConfig as ConfigNetworkConfig, NetworkConfig as ConfigNetwork,
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

// NOTE: unified_simple module removed in Phase 2 cleanup (October 2025)
// All functionality migrated to unified.rs


// REMOVED: Deprecated BearDogMasterConfig (337 lines)
// Migrated to UnifiedBearDogConfig - October 2025


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
// REMOVED: Duplicate aliases (Phase 2 cleanup - October 2025)
// Use UnifiedBearDogConfig directly instead of MasterConfig or GlobalConfig  
// CLEANED: Removed ambiguous aliases - use UnifiedBearDogConfig or BearDogMasterConfig directly
