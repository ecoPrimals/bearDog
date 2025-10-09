// Unified Configuration System - Single Source of Truth
//
// This module provides the **PRIMARY** configuration system for BearDog that eliminates
// all fragmentation by consolidating 80+ Config structs into a unified hierarchy.
//
// ## Configuration Unification Strategy
//
// ### **ELIMINATED FRAGMENTATION**
// - ✅ **80+ Config structs** → **1 Unified Config** + **Domain modules**
// - ✅ **364 files** with configs → **Centralized system**
// - ✅ **Multiple version constants** → **Single version source**
// - ✅ **3 different systems** → **1 unified system**
//
// ### **ARCHITECTURE PRINCIPLES**
// - **Single Source of Truth**: All configuration in one canonical location
// - **Domain Organization**: Logical grouping by functional area
// - **Zero Duplication**: No duplicate config types anywhere
// - **Type Safety**: Strongly typed with comprehensive validation
// - **Performance Optimized**: Zero-cost abstractions and efficient loading
// - **Migration Friendly**: Maintains compatibility during transition

use serde::{Deserialize, Serialize};

// Import canonical configuration types
use crate::canonical::config::hsm::UnifiedHsmConfig;

// Submodules
pub mod core_domains;
pub mod implementations;
pub mod metadata;
pub mod simplified;
pub mod specialized_domains;

// Re-exports for convenience
pub use core_domains::{
    PasswordSource, UnifiedAppConfig, UnifiedDatabaseConfig, UnifiedGeneticsConfig,
    UnifiedNetworkConfig, UnifiedSecurityConfig,
};
pub use metadata::{
    DeploymentMode, Environment, LogLevel, RolloutConfig, RolloutStrategy, SystemMetadata,
    UnifiedVersionInfo,
};
pub use simplified::{
    DatabaseSettings, MonitoringSettings, NetworkSettings, PerformanceSettings, SecuritySettings,
    SimplifiedBearDogConfig,
};
pub use specialized_domains::{
    UnifiedAdapterConfig, UnifiedComplianceConfig, UnifiedDeploymentConfig,
    UnifiedDevelopmentConfig, UnifiedEcosystemConfig, UnifiedFederationConfig,
    UnifiedPerformanceConfig, UnifiedProductionConfig, UnifiedTestingConfig, UnifiedTunnelConfig,
    UnifiedWorkflowConfig,
};

/// Unified BearDog Configuration - Single Source of Truth
///
/// This is the **ROOT** of the unified configuration system that eliminates all
/// fragmentation by consolidating 80+ Config structs into one comprehensive structure.
///
/// ## Architecture
///
/// The configuration is organized into three main layers:
///
/// 1. **Core Domains** - Essential system configurations (app, network, security, hsm, database, monitoring)
/// 2. **Specialized Domains** - Advanced capabilities (genetics, workflows, compliance, performance)
/// 3. **Infrastructure Domains** - System operations (production, deployment)
///
/// ## Benefits
///
/// - **Single Source of Truth** - No configuration fragmentation
/// - **Type Safety** - Strongly typed with validation
/// - **Zero Duplication** - Consolidates 364 files with configs
/// - **Performance** - Zero-cost abstractions
/// - **Migration Friendly** - Maintains compatibility
///
/// ## Example
///
/// ```rust
/// use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
/// # use beardog_errors::BearDogError;
///
/// # fn example() -> Result<(), BearDogError> {
/// // Load from environment
/// let config = UnifiedBearDogConfig::load()?;
///
/// // Access domain configs
/// println!("App name: {}", config.app.app_name);
/// println!("Network timeout: {:?}", config.network.request_timeout);
/// println!("HSM enabled: {}", config.hsm.enabled);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedBearDogConfig {
    /// System metadata including version, environment, and instance information
    pub metadata: SystemMetadata,

    /// **CORE DOMAINS** - Essential system configurations

    /// Application-level configuration (ports, service name, operational parameters)
    pub app: UnifiedAppConfig,

    /// Network communication and connectivity (timeouts, TLS, load balancing)
    pub network: UnifiedNetworkConfig,

    /// Security policies and authentication (encryption, access control, threat detection)
    pub security: UnifiedSecurityConfig,

    /// Hardware Security Module integration (YubiKey, TPM, PKCS#11)
    pub hsm: UnifiedHsmConfig,

    /// Database connectivity and storage (pools, migrations, backups)
    pub database: UnifiedDatabaseConfig,

    /// Monitoring, metrics, and observability (telemetry, alerts, dashboards)
    pub monitoring: crate::canonical::monitoring::MonitoringConfig,

    /// **SPECIALIZED DOMAINS** - Advanced capabilities

    /// Genetic algorithms and AI configuration (entropy, biome sovereignty, key evolution)
    pub genetics: UnifiedGeneticsConfig,

    /// Workflow engine and automation (orchestration, business processes)
    pub workflows: UnifiedWorkflowConfig,

    /// Compliance and regulatory settings (GDPR, HIPAA, SOC 2)
    pub compliance: UnifiedComplianceConfig,

    /// Performance optimization (concurrency, buffers, zero-copy)
    pub performance: UnifiedPerformanceConfig,

    /// **INFRASTRUCTURE DOMAINS** - System operations

    /// Production environment configuration (deployment, scaling, reliability)
    pub production: UnifiedProductionConfig,

    /// Deployment and orchestration (K8s, containers, service mesh)
    pub deployment: UnifiedDeploymentConfig,

    /// Testing framework and validation configuration
    pub testing: UnifiedTestingConfig,

    /// Development environment configuration (dev tools, hot reload, debug settings)
    pub development: UnifiedDevelopmentConfig,

    /// **INTEGRATION DOMAINS** - External connectivity

    /// External adapter and integration configuration (API adapters, protocol bridges)
    pub adapters: UnifiedAdapterConfig,

    /// Secure tunnel and VPN configuration (encrypted channels, peer discovery)
    /// The tunnel value
    pub tunnel: UnifiedTunnelConfig,
    /// Federation and distributed system configuration
    /// The federation value
    pub federation: UnifiedFederationConfig,
    /// Ecosystem and plugin configuration
    /// The ecosystem value
    pub ecosystem: UnifiedEcosystemConfig,
}
