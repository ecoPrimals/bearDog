// SPDX-License-Identifier: AGPL-3.0-only

//! Canonical Configuration System
//!
//! This module provides the **single source of truth** for all configuration types
//! across the BearDog ecosystem. It consolidates 50+ fragmented Config structs
//! into a unified, maintainable system.
//!
//! ## Unification Strategy
//!
//! This module replaces fragmented configurations from:
//! - `beardog-types/src/configuration/` - Legacy configuration types
//! - `beardog-types/src/production/` - Production-specific configs
//! - Individual crate configurations scattered across 22 crates
//!
//! ## Modern Architecture Principles
//!
//! - **Single Source of Truth**: All config types in one canonical location
//! - **Domain Organization**: Logical grouping by functional area
//! - **Zero Fragmentation**: No duplicate config types across crates
//! - **Compatibility Preservation**: Maintains existing APIs during migration
//! - **Type Safety**: Strongly typed configuration with validation
//! - **Performance Optimized**: Zero-cost abstractions and efficient loading
//!
//! ## Example
//!
//! ```rust
//! use beardog_types::canonical::config::UnifiedBearDogConfig;
//!
//! // Load unified configuration
//! let config = UnifiedBearDogConfig::default();
//!
//! // Access domain-specific configs
//! let network_config = &config.network;
//! let security_config = &config.security;
//! ```

// BearDogError no longer needed in this module after deprecated config removal
use serde::{Deserialize, Serialize};

// Domain-specific configuration modules

/// Application configuration
///
/// Core application settings including service name, version, environment,
/// and operational parameters.
pub mod app;

/// Authentication and authorization configuration
///
/// Settings for authentication mechanisms, JWT tokens, OAuth, and
/// authorization policies.
pub mod auth;

/// Caching configuration
///
/// Settings for caching layers, TTLs, eviction policies, and
/// distributed caching strategies.
pub mod cache;

/// Compliance and regulatory configuration
///
/// Settings for GDPR, HIPAA, SOC 2, and other regulatory compliance
/// requirements.
pub mod compliance;

/// Database configuration
///
/// Database connection settings, pool configuration, migration settings,
/// and backup strategies.
pub mod database;

/// Genetics and entropy configuration
///
/// Settings for genetic algorithms, human entropy collection, biome
/// sovereignty, and key evolution.
pub mod genetics;

/// Configuration utilities and helpers
///
/// Utility functions for config loading, validation, and transformations.
/// Includes serde helpers for `Arc<str>` serialization.
pub mod utils;

/// Hardware Security Module (HSM) configuration
///
/// Settings for HSM integration including YubiKey, TPM, PKCS#11,
/// and software HSM configurations.
pub mod hsm;

// Monitoring moved to: canonical/monitoring/ (now the canonical location)

/// Monitoring migration utilities
///
/// Consolidates fragmented monitoring configs into the unified system.
/// Used during the migration from old monitoring configurations.
pub mod monitoring_migration;

/// Network configuration
///
/// Network communication settings including ports, timeouts, TLS,
/// load balancing, and circuit breakers.
pub mod network;

/// Performance configuration
///
/// Performance optimization settings including concurrency limits,
/// buffer sizes, and zero-copy optimizations.
pub mod performance;

/// Production environment configuration
///
/// Production-specific settings for deployment, scaling, reliability,
/// and operational excellence.
pub mod production;

/// Runtime configuration
///
/// Runtime settings with environment variable overrides for network endpoints,
/// ports, and other deployment-specific values.
pub mod runtime_config;
pub mod source;
pub mod test_fixtures;

/// Security configuration
///
/// Core security settings including encryption, key management,
/// access control, and threat detection.
pub mod security;

/// Workflow configuration
///
/// Workflow engine settings for automation, orchestration, and
/// business process management.
pub mod workflow;

// Domain configuration submodules (consolidated types)
/// Domains module containing consolidated configuration types
///
/// NOTE: bootstrap.rs exists but needs proper module system integration
/// For now, domains/ uses simple directory structure
pub mod domains;

// UNIFIED CONFIGURATION SYSTEM - Single source of truth
/// Unified configuration trait and validation utilities
#[path = "trait.rs"]
pub mod r#trait;
/// Type Aliases module
pub mod type_aliases;
/// Unified module
pub mod unified;

// Comprehensive test modules
#[cfg(test)]
mod tests;
#[cfg(test)]
mod validation_comprehensive_tests;
// unified_simple module REMOVED in Phase 2 (October 2025)
// Functionality merged into unified.rs - use SimplifiedBearDogConfig
// REMOVED: unified_trait module (Nov 8, 2025)
// This module was deprecated and replaced by trait.rs
// Migration: Use `use beardog_types::canonical::config::BearDogConfig;`

// MODULAR PRODUCTION CONFIGURATION - Split from oversized production.rs
// This replaces the 1,436-line production.rs file with focused domain modules

// Re-export canonical config types (avoiding naming conflicts)
pub use crate::canonical::monitoring::{
    MonitoringConfig, // The canonical monitoring configuration
    UnifiedAlertingConfig,
    UnifiedHealthConfig,
    UnifiedLoggingConfig,
    UnifiedMetricsConfig,
    UnifiedTracingConfig,
};
pub use app::{AppConfig, ApplicationConfig, CanonicalAppConfig};
pub use auth::*;
pub use cache::*;
pub use compliance::*;
pub use database::*;
pub use genetics::*;
pub use hsm::{UnifiedHsmConfig as ConfigHsmConfig, UnifiedHsmConfig as ConfigHsm};
pub use monitoring_migration::{
    LegacyMonitoringConfig, MonitoringMigrationReport, MonitoringMigrationResult,
    MonitoringMigrationService, create_configuration_legacy_monitoring,
    migrate_monitoring_configurations,
};
pub use network::{CanonicalNetworkConfig as ConfigNetworkConfig, NetworkConfig as ConfigNetwork};
pub use performance::*;
pub use production::*;
pub use production::{EnvironmentLevel, EnvironmentType}; // Explicit re-export for compatibility
pub use runtime_config::{RuntimeConfig, RuntimeHsmConfig, RuntimeNetworkConfig};
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
    DatabaseSettings,
    // Supporting types
    DeploymentMode,
    Environment,
    LogLevel,
    MonitoringSettings,
    // Simplified config types (NEW - merged from unified_simple.rs)
    NetworkSettings,
    PasswordSource,
    PerformanceSettings,
    RolloutConfig,
    RolloutStrategy,
    SecuritySettings,
    // Simplified configuration (NEW - merged from unified_simple.rs)
    SimplifiedBearDogConfig,
    SystemMetadata,
    UnifiedAppConfig,
    // Comprehensive configuration
    UnifiedBearDogConfig,
    UnifiedDatabaseConfig,
    UnifiedGeneticsConfig,
    UnifiedNetworkConfig,
    UnifiedSecurityConfig,
    UnifiedVersionInfo,
};

// Backward compatibility aliases
pub use UnifiedBearDogConfig as PrimaryUnifiedBearDogConfig;
pub use unified::SimplifiedBearDogConfig as WorkingUnifiedConfig;

// NOTE: unified_simple module removed in Phase 2 cleanup (October 2025)
// All functionality migrated to unified.rs

// REMOVED: Deprecated legacy config types (337 lines)
// Migrated to UnifiedBearDogConfig - October 2025

/// High-level flags describing which major subsystems are enabled for the loaded configuration.
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
// Use UnifiedBearDogConfig directly instead of deprecated legacy config types
// CLEANED: Removed ambiguous aliases - use UnifiedBearDogConfig directly
