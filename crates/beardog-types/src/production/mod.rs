// SPDX-License-Identifier: AGPL-3.0-or-later

//! Production-Grade BearDog Ecosystem
//!
//! This module provides advanced production capabilities including comprehensive
//! monitoring, observability, performance optimization, and operational excellence
//! features for the unified BearDog ecosystem.
//!
//! # Module Organization
//!
//! The production module is organized into focused submodules:
//!
//! ## Core Modules
//!
//! - [`types`](crate::production::types) - Domain types (EnvironmentLevel, OperationalStatus, PerformanceMetrics)
//! - [`config`](crate::production::config) - Configuration structures (ProductionConfig, ProductionCoreConfig, ProductionFlags)
//! - [`ecosystem`](crate::production::ecosystem) - Production orchestrator (ProductionEcosystem, ProductionState)
//! - [`builder`](crate::production::builder) - Builder pattern (ProductionEcosystemBuilder)
//!
//! ## Subsystem Modules
//!
//! - [`health`](crate::production::health) - Health monitoring and checks
//! - [`metrics`](crate::production::metrics) - Metrics collection and export
//! - [`monitoring`](crate::production::monitoring) - System monitoring
//! - [`observability`](crate::production::observability) - Distributed tracing and logging
//! - [`optimization`](crate::production::optimization) - Performance optimization
//! - [`telemetry`](crate::production::telemetry) - APM and telemetry
//!
//! # Architecture
//!
//! ```text
//! Production Module
//! ├── types.rs          - Domain types and enums
//! ├── config.rs         - Configuration structures
//! ├── ecosystem.rs      - Core orchestrator
//! ├── builder.rs        - Fluent builder API
//! └── Subsystems/
//!     ├── health/       - Health checks
//!     ├── metrics/      - Metrics collection
//!     ├── monitoring/   - System monitoring
//!     ├── observability/ - Tracing & logging
//!     ├── optimization/ - Performance tuning
//!     └── telemetry/    - APM integration
//! ```
//!
//! # Examples
//!
//! ## Basic Usage
//! ```rust,ignore
//! use beardog_types::production::{
//!     ProductionEcosystem,
//!     ProductionConfig,
//! };
//!
//! let config = ProductionConfig::default();
//! let mut ecosystem = ProductionEcosystem::new(config)?;
//! ecosystem.initialize()?;
//! ```
//!
//! ## Builder Pattern
//! ```rust,ignore
//! use beardog_types::production::{
//!     ProductionEcosystemBuilder,
//!     EnvironmentLevel,
//! };
//!
//! let ecosystem = ProductionEcosystemBuilder::new()
//!     .with_environment(EnvironmentLevel::Production)
//!     .with_service_name("beardog-api")
//!     .with_region("us-west-2")
//!     .with_monitoring_enabled(true)
//!     .build()?;
//! ```

// ============================================================================
// CORE MODULES
// ============================================================================

/// Domain types and enums
pub mod types;

/// Configuration structures
pub mod config;

/// Production ecosystem orchestrator
pub mod ecosystem;

/// Fluent builder pattern
pub mod builder;

// ============================================================================
// SUBSYSTEM MODULES
// ============================================================================

/// Health module
pub mod health;
/// Metrics module
pub mod metrics;
/// Monitoring module
pub mod monitoring;
/// Observability module
pub mod observability;
/// Optimization module
pub mod optimization;
/// Telemetry module
pub mod telemetry;

// ============================================================================
// RE-EXPORTS (Backwards Compatibility)
// ============================================================================

// Re-export types for convenient access
pub use types::{EnvironmentLevel, OperationalStatus, PerformanceMetrics};

// Re-export configuration
pub use config::{ProductionConfig, ProductionCoreConfig, ProductionFlags};

// Re-export ecosystem
pub use ecosystem::{ProductionEcosystem, ProductionState};

// Re-export builder
pub use builder::ProductionEcosystemBuilder;

// ============================================================================
// TEST MODULES
// ============================================================================

// Comprehensive test modules
#[cfg(test)]
mod health_comprehensive_tests;
#[cfg(test)]
mod metrics_comprehensive_tests;
#[cfg(test)]
mod tests_advanced;

// Test modules - October 24, 2025
#[cfg(test)]
mod ecosystem_tests;
#[cfg(test)]
mod monitoring_advanced_tests;
#[cfg(test)]
mod observability_tests;
#[cfg(test)]
mod optimization_tests;
#[cfg(test)]
mod telemetry_tests;

// October 25, 2025: Week 2 Test Expansion
#[cfg(test)]
mod production_types_validation_tests;

// October 26, 2025: Week 2 Day 3 - Lifecycle Tests
#[cfg(test)]
mod ecosystem_lifecycle_tests;
#[cfg(test)]
mod production_integration_tests;

// October 27, 2025: Week 1 Test Expansion - Production Core Tests
#[cfg(test)]
mod production_core_tests;

// October 28, 2025: Evening Test Expansion - Comprehensive Mod Tests
#[cfg(test)]
mod mod_comprehensive_tests;

// Extracted inline tests
#[cfg(test)]
mod tests;
