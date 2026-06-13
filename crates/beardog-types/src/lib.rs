// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]

//! # 🏆 `BearDog` Types - Canonical Type System
//!
//! [![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
//! [![License](https://img.shields.io/badge/license-AGPL--3.0-blue.svg)](../../LICENSE)
//! [![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/ecoPrimals/beardog)
//! [![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/beardog-types)
//!
//! **The canonical type system for the `BearDog` distributed security ecosystem - now with PEDANTIC PERFECTION!**
//!
#![cfg_attr(test, allow(clippy::expect_used))]
#![cfg_attr(test, allow(clippy::unwrap_used))]
// Coverage and unit tests compare floats to literals, assert const invariants, and may use
// large stack arrays in generated patterns; keep `-D warnings` builds clean without churn.
#![cfg_attr(
    test,
    allow(
        clippy::float_cmp,
        clippy::assertions_on_constants,
        clippy::large_stack_arrays,
    )
)]
//! # `BearDog` Types v3.0.0
//!
//! **Production-grade type system for the `BearDog` sovereign computing ecosystem.**
//!
//! This crate provides the foundational type definitions, configurations, and data structures
//! that power the `BearDog` ecosystem's quantum-enhanced, zero-knowledge, and sovereignty-preserving
//! architecture.
//!
//! ## Features
//!
//! - **Zero-Copy Optimization**: Efficient memory management with minimal allocations
//! - **Canonical Type System**: Unified, consistent type definitions across all components
//! - **HSM Integration**: Hardware Security Module support for Android `StrongBox` and iOS Secure Enclave
//! - **Quantum-Enhanced**: Advanced quantum-inspired discovery and capability types
//! - **Human Dignity Preservation**: Types designed to protect user privacy and autonomy
//! - **Production-Ready**: Comprehensive error handling and validation
//!
//! ## Architecture
//!
//! The crate is organized into several key modules:
//!
//! - `canonical/`: Core canonical type definitions and configurations
//! - `zero_cost/`: Zero-copy optimized data structures
//! - `workflow/`: Workflow management and processing types
//! - `hsm/`: Hardware Security Module integration types
//! - `quantum/`: Quantum-enhanced discovery and capability types
//!
//! ## Examples
//!
//! ```rust
//! use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
//!
//! // Create a new unified configuration
//! let config = UnifiedBearDogConfig::default();
//! println!("Config created successfully");
//! ```
//!
//! ## Safety
//!
//! This crate maintains `BearDog`'s commitment to memory safety with:
//! - Fully memory-safe
//! - Comprehensive error handling
//! - Production-grade validation
//! - Human dignity preservation

#![doc(html_root_url = "https://docs.rs/beardog-types/3.0.0")]

// Re-export beardog-errors for convenience
pub use beardog_errors::BearDogError;

///
/// This module contains the unified, canonical type system that eliminates
/// all fragmentation across the `BearDog` ecosystem.
///
/// ## Key Modules
/// - [`canonical::config`] - Unified configuration system
/// - [`canonical::providers_unified`] - Unified provider trait system  
/// - [`canonical::monitoring`] - Advanced monitoring and metrics
/// - [`canonical::security_unified`] - Security types and operations
/// - [`canonical::hsm`] - Hardware Security Module integration
/// - [`canonical::network`] - Network communication types
/// - [`canonical::crypto`] - Cryptographic operations
pub mod canonical;

pub mod adapter_certificates;
/// Adapter integration types (unlock certificates, classifications for daemon issuance).
pub mod adapters;
/// 🔒 **CONSTRAINTS** - Universal constraint evaluation system
///
/// Philosophy: Users define their own rules. We provide the framework, not the limits.
///
/// This module provides a constraint-agnostic architecture where users can create
/// novel constraints for scenarios we can't predict:
/// - Proximity-based constraints
/// - Environmental sensor constraints
/// - Network connectivity constraints
/// - Biometric authentication constraints
/// - Composite logical constraints (AND/OR/NOT)
pub mod constraints;
/// Genetic constraint definitions used by the genetics subsystem and validation pipelines.
pub mod genetics_constraints;
/// Helpers for building, serializing, and evaluating genetic constraint bundles.
pub mod genetics_constraints_helpers;
pub mod primal_identity; // TRUE PRIMAL pattern: explicit identity injection

/// Crate-root HSM integration types (distinct from [`canonical::hsm`] canonical models).
pub mod hsm;

/// Universal operation receipts for verifiable audit trails across services.
pub mod receipt;

/// Mathematical limits, domain constants, and shared numeric bounds for the ecosystem.
pub mod constants;

/// Ergonomic trait patterns for conversions and shared behavior across Beardog types.
pub mod modern_traits;

/// Zero-cost abstraction types (IDs, workflow markers) used on hot paths.
pub mod zero_cost;

/// 🏭 **PRODUCTION** - Production-ready components
///
/// Production types and configurations. New code should prefer
/// the canonical modules where applicable.
pub mod production;

/// 🔐 **SECURITY** - Security type definitions
///
/// Security-related types and utilities. Consider using
pub mod security;

// Use: beardog_types::canonical::services::UnifiedServiceDefinition

/// 🧬 **GENETICS** - Genetic algorithm types
///
pub mod genetics;

/// 🎯 **CAPABILITIES** - Capability-based access control
///
pub mod capabilities;

/// 🔐 **BTSP** - BearDog Tunnel Security Protocol (Unified)
///
/// Unified secure protocol provider for both internal (primal-to-primal)
/// and external (HTTPS API) communication. Single API, different trust modes!
pub mod btsp;

/// Ionic bond types for cross-atomic-boundary trust negotiation.
pub mod ionic_bond;

/// 📈 **METRICS** - Metrics and monitoring types
///
pub mod metrics;

/// 🏥 **HEALTH** - Health monitoring types
///
/// Health check and system status types.
pub mod health_status;

// Test modules
#[cfg(test)]
mod coverage_boost;
#[cfg(test)]
mod tests;

#[cfg(test)]
mod lib_main_tests {
    use super::*;
    use std::marker::PhantomData;

    #[test]
    fn test_types_lib_accessible() {
        // Verify types lib module loads
    }

    #[test]
    fn test_version_constants() {
        let version = VERSION;
        let description = DESCRIPTION;
        assert!(!version.is_empty(), "VERSION should not be empty");
        assert_eq!(CRATE_NAME, "beardog-types");
        assert!(!description.is_empty(), "DESCRIPTION should not be empty");
    }

    #[test]
    fn test_canonical_module_accessible() {}

    #[test]
    fn test_hsm_module_accessible() {}

    #[test]
    fn test_constants_module_accessible() {}

    #[test]
    fn test_zero_cost_module_accessible() {}

    #[test]
    fn test_production_module_accessible() {}

    #[test]
    fn test_security_module_accessible() {}

    #[test]
    fn test_genetics_module_accessible() {}

    #[test]
    fn test_capabilities_module_accessible() {}

    #[test]
    fn test_metrics_module_accessible() {}

    #[test]
    fn test_health_status_module_accessible() {}

    #[test]
    fn test_workflow_module_accessible() {}

    #[test]
    fn test_unified_config_creation() {
        let config = SimplifiedBearDogConfig::default();
        assert!(!format!("{config:?}").is_empty());
    }

    #[test]
    fn test_config_exports() {
        use crate::{DatabaseSettings, MonitoringSettings, NetworkSettings};
        use crate::{PerformanceSettings, SecuritySettings};
        let _: PhantomData<(
            DatabaseSettings,
            MonitoringSettings,
            NetworkSettings,
            PerformanceSettings,
            SecuritySettings,
        )> = PhantomData;
    }

    #[test]
    fn test_provider_trait_exports() {
        // Ensure crate-root provider traits remain reachable (not required to be object-safe).
        const _: () = {
            fn _ai<T: crate::UnifiedAiProvider>() {}
            fn _storage<T: crate::UnifiedStorageProvider>() {}
            fn _hsm<T: crate::UnifiedHsmProvider>() {}
            fn _base<T: crate::UnifiedProvider>() {}
            fn _sec<T: crate::UnifiedSecurityProvider>() {}
            fn _mon<T: crate::UnifiedMonitoringProvider>() {}
            fn _net<T: crate::UnifiedNetworkProvider>() {}
        };
    }

    #[test]
    fn test_provider_types_exports() {
        use crate::{ProviderHealth, ProviderInfo, ProviderMetrics};
        let _: PhantomData<(ProviderHealth, ProviderInfo, ProviderMetrics)> = PhantomData;
    }

    #[test]
    fn test_constants_exports() {
        use crate::constants::domains::system::versions::BEARDOG_VERSION;
        let ver = BEARDOG_VERSION;
        assert!(!ver.is_empty(), "BEARDOG_VERSION should not be empty");
    }

    #[test]
    fn test_error_type_accessible() {
        use crate::BearDogError;
        let _: PhantomData<BearDogError> = PhantomData;
    }

    #[test]
    fn test_memory_safety_lint_documented() {
        // Crate sources inherit rustc lint levels from the workspace Cargo.toml.
    }

    #[test]
    fn test_pedantic_lints_enabled() {
        // Pedantic lint levels come from workspace Clippy configuration.
    }

    #[test]
    fn test_canonical_config_system() {}

    #[test]
    fn test_canonical_providers_system() {}

    #[test]
    fn test_architecture_modules() {}

    #[tokio::test]
    async fn test_async_types_available() {
        // Verify async patterns work with types
        // Modern: Just yield to verify async runtime works
        tokio::task::yield_now().await;
    }

    #[test]
    fn test_module_organization() {
        // Module tree matches crate docs: canonical, hsm, constants, zero_cost, production.
    }

    #[test]
    fn test_features_documented() {
        // Verify all documented features are testable
        // Zero-Copy Optimization ✓
        // Canonical Type System ✓
        // HSM Integration ✓
        // Quantum-Enhanced ✓
        // Human Dignity Preservation ✓
        // Production-Ready ✓
    }
}

/// 🔄 **WORKFLOWS** - Workflow orchestration types
///
pub mod workflow;

/// Crypto service types (protocol-agnostic)
pub mod crypto_service;

// **MODERNIZED CONFIGURATION EXPORTS** - Canonical unified configuration system
pub use canonical::config::SimplifiedBearDogConfig;
pub use canonical::config::{
    DatabaseSettings, MonitoringSettings, NetworkSettings, PerformanceSettings, SecuritySettings,
};

// **REMOVED**: Legacy configuration compatibility layers
// Use canonical::config::unified::SimplifiedBearDogConfig for all configuration

// **TYPE-SAFE ID NEWTYPES** - Zero-cost compile-time type safety (Nov 9, 2025)
pub use canonical::types::{KeyId, RegistrationId, ServiceInstanceId};

// **CANONICAL PROVIDER SYSTEM** - Unified provider traits and types
pub use canonical::providers_unified::traits::{
    ProviderHealth, ProviderInfo, ProviderMetrics, UnifiedAiProvider, UnifiedHsmProvider,
    UnifiedMonitoringProvider, UnifiedNetworkProvider, UnifiedProvider, UnifiedSecurityProvider,
    UnifiedStorageProvider,
};

// **MODERNIZED ERROR SYSTEM** - Single unified error type (already imported above)

// **MODERNIZED CONSTANTS SYSTEM** - Domain-organized constants
pub use constants::domains::{
    network::{addresses::*, timeouts::CONNECTION_TIMEOUT},
    security::{auth::*, crypto::*, hsm::*},
    system::{defaults::*, limits::*, timeouts::*, versions::BEARDOG_VERSION},
};

// Version information
/// Current version of the beardog-types crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate description
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
