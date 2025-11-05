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
//! - [`ecosystem_genetic_spawner`] - Genetic spawning for new primals
//! - [`integration_engine`] - Core integration orchestration
//! - [`license_manager`] - License and agreement management
//! - [`performance_optimizer`] - Cross-ecosystem performance optimization
//! - [`types`] - Integration type definitions
//! - [`universal_adapter`] - Universal service adaptation
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

/// Songbird integration for network discovery
pub mod songbird_integration;
/// Universal service adaptation
pub mod universal_adapter;
/// Universal compute client for distributed processing
pub mod universal_compute_client;

pub use integration_engine::*;
pub use license_manager::*;
// Avoid ambiguous re-exports by aliasing conflicting types
pub use performance_optimizer::{
    CapabilityConnectionPool, ComputeCache, ConnectionMetrics as PerformanceConnectionMetrics,
    EcosystemPerformanceOptimizer, PoolConfig as PerformancePoolConfig,
};
pub use songbird_integration::*;
pub use types::*;
pub use universal_adapter::*;
pub use universal_compute_client::*;
