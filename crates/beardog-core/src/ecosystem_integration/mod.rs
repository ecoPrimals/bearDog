//! # Ecosystem Integration Module
//!
//! Provides comprehensive integration capabilities for BearDog to interact
//! with the broader primal ecosystem through universal adapters and
//! genetic spawning patterns.
//!
//! ## Overview
//!
//! This module enables BearDog to:
//! - Discover and integrate with ecosystem services
//! - Spawn new primal instances using genetic patterns
//! - Manage licenses and service agreements
//! - Optimize performance across ecosystem boundaries
//! - Adapt to any service through universal adapters
//!
//! ## Key Components
//!
//! - `ecosystem_genetic_spawner` - Genetic spawning for new primals
//! - `integration_engine` - Core integration orchestration
//! - `license_manager` - License and agreement management
//! - `performance_optimizer` - Cross-ecosystem performance optimization
//! - `types` - Integration type definitions
//! - `universal_adapter` - Universal service adaptation
//!
//! ## Example
//!
//! ```rust,ignore
//! use beardog_core::ecosystem_integration::{IntegrationEngine, UniversalAdapter};
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! let engine = IntegrationEngine::new();
//!
//! // Discover and integrate with ecosystem services
//! let services = engine.discover_ecosystem_services().await?;
//! # Ok(())
//! # }
//! ```

/// Genetic spawning for new primal instances
pub mod ecosystem_genetic_spawner;

/// Core integration orchestration
pub mod integration_engine;

/// License and agreement management
pub mod license_manager;

/// Cross-ecosystem performance optimization
pub mod performance_optimizer;

/// Integration type definitions
pub mod types;

/// Deprecated songbird integration (kept for backward compatibility, do not use)
///
/// **Use `UniversalPrimalAdapter` instead** - see `beardog_adapters::UniversalPrimalAdapter`
#[deprecated(
    since = "3.3.0",
    note = "Use UniversalPrimalAdapter from beardog-adapters crate for capability-based discovery"
)]
#[allow(deprecated)]
pub mod songbird_integration;

/// Universal service adaptation
pub mod universal_adapter;
/// Universal compute client for distributed processing
pub mod universal_compute_client;

pub use integration_engine::*;
pub use license_manager::*;

// Tests
#[cfg(test)]
mod integration_engine_tests;

#[cfg(test)]
mod license_manager_tests;

#[cfg(test)]
mod performance_optimizer_tests;

// Avoid ambiguous re-exports by aliasing conflicting types
pub use performance_optimizer::{
    CapabilityConnectionPool, ComputeCache, ConnectionMetrics as PerformanceConnectionMetrics,
    EcosystemPerformanceOptimizer, PoolConfig as PerformancePoolConfig,
};

// ⚠️ DEPRECATED: Do not use these exports - they hardcode primal names
// Use UniversalPrimalAdapter from beardog-adapters crate instead
#[deprecated(
    since = "3.3.0",
    note = "Use UniversalPrimalAdapter from beardog-adapters for capability-based primal discovery"
)]
#[allow(deprecated)]
pub use songbird_integration::*;

pub use types::*;
pub use universal_adapter::*;
pub use universal_compute_client::*;
