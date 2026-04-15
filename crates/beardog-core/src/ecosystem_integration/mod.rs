// SPDX-License-Identifier: AGPL-3.0-or-later

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

/// Secure cross-primal messaging (capability-based, zero hardcoding)
pub mod secure_cross_primal_messaging;

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

#[cfg(test)]
#[path = "event_system_comprehensive_tests.rs"]
mod event_system_comprehensive_tests;

#[cfg(test)]
mod tests;

// Avoid ambiguous re-exports by aliasing conflicting types
pub use performance_optimizer::{
    CapabilityConnectionPool, ComputeCache, ConnectionMetrics as PerformanceConnectionMetrics,
    EcosystemPerformanceOptimizer, PoolConfig as PerformancePoolConfig,
};

pub use secure_cross_primal_messaging::{
    MessengerMetrics, PrimalDiscoveryService, SecureCrossPrimalMessenger, SecurePrimalMessage,
    SecurePrimalResponse, SecureSession,
};
pub use types::{EcosystemEvent, EcosystemNode, EventPriority, EventStatus, NodeType};
pub use universal_adapter::{
    AdapterEvent, AdapterMetrics, AdapterOperation, AdapterRequest, AdapterResponse,
    ConnectionInfo, ConnectionMetrics, ConnectionPool, ConnectionPoolConfig, ConnectionStatus,
    EventSubscriber, ProductionConfig, ProductionFeature, ProductionUniversalAdapter, ProtocolType,
    ServiceEndpoint, UniversalAdapter, UniversalAdapterConfig,
};
pub use universal_compute_client::{
    ComputeArchitecture, ComputeDiscoveryConfig, ComputeMetrics, ComputePriority,
    ComputeProviderInfo, OptimizationType, ProcessingCapability, ResourceUsageStats,
    UniversalComputeClient, UniversalComputeConfig, UniversalComputeRequest,
    UniversalComputeResponse,
};
