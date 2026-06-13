// SPDX-License-Identifier: AGPL-3.0-or-later

//! Canonical Types for BearDog
//!
//! Single source of truth for shared types across the BearDog ecosystem: unified configuration,
//! security, monitoring, discovery, capabilities, and cross-cutting domain types.
//!
//! # Quick start
//!
//! ```rust
//! use beardog_types::canonical::{
//!     UnifiedBearDogConfig, HealthStatus, SecurityContext, ProviderConfig,
//! };
//! let config = UnifiedBearDogConfig::development();
//! let health = HealthStatus::Healthy;
//! assert_eq!(health, HealthStatus::Healthy);
//! ```
//!
//! # Submodules
//!
//! - [`config`](crate::canonical::config) — unified configuration
//! - [`capabilities`] — capability-based architecture
//! - [`security_unified`](crate::canonical::security_unified) — security configuration
//! - [`providers_unified`](crate::canonical::providers_unified) — provider adapters
//! - [`monitoring`](crate::canonical::monitoring) — observability
//! - [`hsm`] — hardware security modules
//! - [`network`](crate::canonical::network) — networking and discovery
//! - [`crypto`](crate::canonical::crypto) — cryptographic types
//!
//! # Design
//!
//! One canonical location per concept, strong typing, serde where appropriate, and a clear
//! migration path from legacy types. See submodule documentation for details.

// Core canonical modules
/// Biome configuration types
pub mod biome;
/// Capabilities module
pub mod capabilities;
/// Config module
/// Configuration management
pub mod config;
/// Constants module
pub mod constants;
/// Cryptographic types and operations
pub mod crypto;
/// Discovery module - Universal capability-based discovery
pub mod discovery;
/// Hardware Security Module integration
pub mod hsm;
/// Monitoring and observability types
pub mod monitoring;
/// Network communication types
pub mod network;

// NEW: Unified configurations (replaces fragmented configs)
/// Hsm Unified module
pub mod hsm_unified;
/// Network Unified module
pub mod network_unified;
/// Providers Unified module
pub mod providers_unified; // ✅ MODERNIZED: Split into modular structure
/// Rate limiting configuration - canonical implementation
pub mod rate_limiting; // ✅ MODERNIZED: Split into modular structure
/// Security Unified module
pub mod security_unified;
/// Configuration trait interfaces for polymorphic usage
pub mod traits; // ✅ NEW: Trait-based config interfaces (Nov 8, 2025) // ✅ MODERNIZED: Split into modular structure

/// Type-safe ID newtypes for compile-time safety (Nov 9, 2025)
pub mod types;

// Re-export type-safe ID newtypes for easy access
pub use types::{KeyId, RegistrationId, ServiceInstanceId};

mod canonical_core;
mod canonical_domain_types;
mod canonical_helpers;

pub use canonical_core::*;
pub use canonical_domain_types::*;
pub use canonical_helpers::migration;
pub use canonical_helpers::{canonical_type_info, validate_canonical_usage};

/// Utility functions for canonical types
pub mod utils;

// Legacy modules - REMOVED as part of modernization cleanup

// Re-export the unified configurations as canonical
pub use security_unified::CanonicalSecurityConfig;

// Compatibility alias - will be deprecated in v4.0.0
pub use security_unified::SecurityConfig;

pub use providers_unified::{
    CanonicalProviderConfig,
    ConnectionConfig,
    // Re-export all provider types for convenience
    CoreProviderSettings,
    DiscoveryConfig,
    HealthConfig,
    KeyType, // Key type enumeration for HSM operations
    LoadBalancingConfig,
    PerformanceConfig,
    ProviderConfig, // Primary provider config type
    ProviderMonitoringConfig,
    ProviderSecurityConfig,
    ResilienceConfig,
    // Re-export provider traits for Android StrongBox and other implementations
    UnifiedHsmProvider,
    UnifiedProvider,
    UnifiedSecurityProvider,
};

// Export the canonical MonitoringConfig
pub use monitoring::MonitoringConfig;

pub use monitoring::MonitoringConfig as CanonicalMonitoringConfig;

pub use network_unified::{
    CanonicalNetworkConfig,
    NetworkConfig, // Primary network config type
};

pub use hsm_unified::{
    CanonicalHsmConfig,
    HsmConfig, // Primary HSM config type
};

// Re-export the new unified configuration system
pub use config::{
    // Domain-specific canonical configs (avoiding conflicts with existing imports)
    CanonicalAppConfig,
    CanonicalAuthConfig,
    CanonicalCacheConfig,
    CanonicalComplianceConfig,
    CanonicalDatabaseConfig,
    CanonicalGeneticsConfig,
    CanonicalPerformanceConfig,
    // CanonicalProductionConfig removed - use UnifiedProductionConfig directly
    CanonicalWorkflowConfig,
    // Compatibility aliases
    UnifiedBearDogConfig,
};

// Specific re-exports to avoid ambiguous glob imports
// Capabilities
pub use capabilities::{
    CapabilityRequirements, HumanEntropyCapabilities, PerformanceCapabilities, SecurityLevel,
};

// Configuration - primary exports (from main configuration module, not canonical)
// Legacy config re-exports - use canonical::config instead
#[deprecated(
    since = "3.1.0",
    note = "Use canonical::config::app::UnifiedAppConfig instead"
)]
// Constants
pub use constants::genetics;

// Crypto - specific exports to avoid conflicts
pub use crypto::{
    CryptoAlgorithm,
    // KeyUsage conflicts with HSM, so we'll use crypto::KeyUsage explicitly when needed
};

// HSM - specific exports to avoid conflicts
pub use hsm::{
    HsmCapabilities,
    HsmKey,
    KeyMetadata,
    // HsmConfig conflicts with crypto, using hsm::HsmConfig when needed
    // KeyUsage conflicts with crypto, using hsm::KeyUsage when needed
};

// Monitoring - export the unified config (sub-configs accessed through MonitoringConfig fields)
// Old specific exports removed - all available through MonitoringConfig
// e.g., config.alerting, config.metrics, config.health, etc.

// Network - specific exports to avoid conflicts
pub use network::{
    // ConnectionPoolConfig, // Use config::domains::network::ConnectionPoolConfig instead
    LoadBalancerConfig,
    TimeoutConfig,
    // NetworkConfig conflicts with configuration module
    // HealthCheckConfig conflicts with monitoring module
    // LoadBalancingStrategy conflicts with hsm module
};

// Providers - now exported from providers_unified
// (These are exported through the unified module re-exports above)

/// Health status types and definitions
pub mod health_status;
pub use health_status::{
    ComponentStatus, HealthStatus as CanonicalHealthStatus, OperationStatus,
    WorkflowStatus as CanonicalWorkflowStatus,
};

// Tests extracted to separate module for better organization
#[cfg(test)]
#[path = "canonical_types_tests.rs"]
mod canonical_types_tests;
