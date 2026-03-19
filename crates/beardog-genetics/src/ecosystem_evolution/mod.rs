// SPDX-License-Identifier: AGPL-3.0-only

//! # Ecosystem Evolution Genetics
//!
//! This module implements horizontal gene transfer integration across ecosystem primals'
//! ecosystem evolution initiative. It provides the genetic foundation for evolving from
//! binary relationship patterns to spectrum-based ecosystem intelligence.
//!
//! ## Architecture
//!
//! The module is organized into cohesive submodules:
//!
//! - **types**: Core domain types (EcosystemMembership, TrustEvolution, CoordinationModel, etc.)
//! - **support**: Supporting types and helpers used throughout the system
//! - **genetics**: Specialized genetics modules for different evolution aspects
//! - **engine**: Main EcosystemGeneticEngine orchestrating evolution
//! - **tests**: Comprehensive test suite
//!
//! ## Example Usage
//!
//! ```rust
//! use beardog_genetics::ecosystem_evolution::{
//!     EcosystemGeneticEngine,
//!     BinaryAccessPattern,
//!     EcosystemContext,
//! };
//! use beardog_types::canonical::HealthStatus;
//!
//! # fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Create the genetic engine
//! let engine = EcosystemGeneticEngine::new()?;
//!
//! // Evolve a binary access pattern to ecosystem membership
//! let pattern = BinaryAccessPattern::Allowlist {
//!     allowed_entities: vec!["entity1".to_string()],
//! };
//!
//! let context = EcosystemContext {
//!     current_health: HealthStatus::Healthy,
//!     active_relationships: 5,
//!     ecosystem_load: 0.5,
//!     recent_events: vec![],
//! };
//!
//! let membership = engine.evolve_access_pattern(pattern, context)?;
//! # Ok(())
//! # }
//! ```

// Module declarations
pub mod engine;
pub mod genetics;
pub mod support;
pub mod types;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod engine_comprehensive_tests;

// Re-export key types for convenience
pub use engine::{migrate_from_binary_patterns, EcosystemGeneticEngine};
pub use support::{
    BinaryAccessPattern, BinaryTrust, EcosystemContext, EcosystemHealthReport, HierarchicalPattern,
    RelationshipHistory,
};
pub use types::{CoordinationModel, EcosystemMembership, SymbiosisType, TrustEvolution};
