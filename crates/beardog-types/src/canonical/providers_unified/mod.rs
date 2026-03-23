// SPDX-License-Identifier: AGPL-3.0-only

//! Canonical provider configuration and re-exports (connection, health, performance, traits).

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
/// Monitoring module
pub mod monitoring;
/// Throughput limits, caching, and compression for provider implementations.
pub mod performance;
/// Resilience module
pub mod resilience;
/// Security module
pub mod security;
/// Service discovery (canonical)
pub mod service_discovery;
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

    /// Concurrency, cache, and wire-efficiency knobs. **Default:** [`PerformanceConfig::default()`].
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

/// Back-compat alias for [`CanonicalProviderConfig`].
pub type ProviderConfig = CanonicalProviderConfig;
