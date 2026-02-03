// Unified Canonical Provider Configuration
//
// This module consolidates all provider configuration patterns across BearDog into a single,
// comprehensive, modern system. It replaces fragmented provider configs with unified types.
//
// ## Smart Consolidation Strategy
//
// This unifies and modernizes:
// - `beardog-types/src/providers.rs` - Legacy provider types
// - `beardog-types/src/canonical/providers.rs` - Basic provider types
// - Various provider configs scattered across crates
//
// ## Modern Architecture Principles
//
// - **Single Source of Truth**: All provider config in one canonical place
// - **Hierarchical Organization**: Logical grouping by provider domain
// - **Zero Fragmentation**: No duplicate types across crates
// - **Modern Rust Patterns**: Latest idioms and BearDog conventions
// - **Production Compatibility**: Maintains all existing functionality

use serde::{Deserialize, Serialize};

// Modular organization
/// Connection module
pub mod connection;
/// Core module
/// Core functionality
/// Core functionality
pub mod core;
/// Discovery module (deprecated - use service_discovery instead)
pub mod discovery;
/// Health module
pub mod health;
/// Load Balancing module
pub mod load_balancing;
/// Migration module
pub mod migration;
/// Service discovery (canonical)
pub mod service_discovery;
// pub mod trait_migration; // Temporarily disabled - migration utility only
/// Monitoring module
pub mod monitoring;
pub mod performance;
/// Resilience module
pub mod resilience;
/// Security module
pub mod security;
// pub mod trait_migration; // NEW: Provider trait migration utilities - DISABLED
/// Traits module
pub mod traits; // NEW: Unified provider trait system // Provider migration utilities

// Re-export all types for easy access (avoiding ambiguous re-exports)
pub use connection::*;
pub use discovery::*;
pub use health::*;
pub use load_balancing::*;
pub use migration::*;
pub use monitoring::*;
pub use performance::*;
pub use resilience::*;
pub use security::*;

// Core types - explicit re-exports to avoid ambiguity
pub use core::{CoreProviderSettings, ProviderRegistryEntry, ProviderStatus};

// Unified traits - use these for new implementations
// pub use trait_migration::{
//     create_canonical_base_legacy_provider, create_canonical_hsm_legacy_provider,
//     create_canonical_security_legacy_provider, migrate_provider_traits, LegacyProviderTrait,
//     ProviderMigrationReport, ProviderMigrationResult, ProviderMigrationService,
// };
pub use traits::{
    KeyType, // Key type enumeration for HSM operations
    ProviderCapability as TraitProviderCapability,
    ProviderHealth as TraitProviderHealth,
    // Use trait-specific types to avoid conflicts
    ProviderInfo as TraitProviderInfo,
    ProviderType as TraitProviderType,
    UnifiedAiProvider,
    UnifiedHsmProvider,
    UnifiedMonitoringProvider,
    UnifiedNetworkProvider,
    UnifiedProvider,
    UnifiedSecurityProvider,
    UnifiedStorageProvider,
};

/// Unified canonical provider configuration - Single source of truth
///
/// This consolidates all provider configuration across `BearDog` into one comprehensive,
/// modern structure that eliminates fragmentation while maintaining full functionality.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalProviderConfig {
    /// Core provider settings
    /// The core value
    pub core: CoreProviderSettings,

    /// Connection management
    /// The connection value
    pub connection: ConnectionConfig,

    /// Health monitoring
    /// The health value
    pub health: HealthConfig,

    pub performance: PerformanceConfig,

    /// Security configuration
    /// The security value
    pub security: ProviderSecurityConfig,

    /// Retry and resilience
    /// The resilience value
    pub resilience: ResilienceConfig,

    /// Service discovery
    /// The discovery value
    pub discovery: DiscoveryConfig,

    /// Load balancing
    /// The load balancing value
    pub load_balancing: LoadBalancingConfig,

    /// Monitoring and observability
    /// The monitoring value
    pub monitoring: ProviderMonitoringConfig,
}

pub type ProviderConfig = CanonicalProviderConfig;
