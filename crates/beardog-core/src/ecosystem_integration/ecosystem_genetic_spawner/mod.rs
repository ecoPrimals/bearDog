// SPDX-License-Identifier: AGPL-3.0-only

//! # Ecosystem Genetic Spawner Module
//!
//! Provides genetic spawning capabilities for creating new primal instances
//! that inherit and evolve characteristics from their ancestors.
//!
//! ## Overview
//!
//! The genetic spawner uses evolutionary algorithms to:
//! - Spawn new primal instances optimized for specific tasks
//! - Inherit successful traits from parent primals
//! - Evolve and adapt to ecosystem conditions
//! - Manage primal lifecycle from genesis to retirement
//!
//! ## Key Components
//!
//! - Core spawning engine
//! - `traits` - Genetic traits and interfaces
//! - `types` - Type definitions for genetic patterns
//!
//! ## Example
//!
//! ```rust,ignore
//! use beardog_core::ecosystem_integration::ecosystem_genetic_spawner::EcosystemGeneticSpawner;
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! let spawner = EcosystemGeneticSpawner::new();
//!
//! // Spawn new primal with genetic inheritance
//! let new_primal = spawner.spawn_primal().await?;
//! # Ok(())
//! # }
//! ```

/// Core spawning engine
pub mod spawner;

/// Genetic traits and interfaces
pub mod traits;

/// Type definitions for genetic patterns
pub mod types;

pub use spawner::EcosystemGeneticSpawner;
pub use traits::{EcosystemGeneticTrait, EcosystemPrimalClient, EcosystemResourceAvailability};
pub use types::*;
