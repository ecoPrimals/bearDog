// SPDX-License-Identifier: AGPL-3.0-or-later

//! Genetic spawning: instantiate new primals from capability requests and parent lineage.

/// Spawning engine: inheritance, fitness, and metrics for new [`BearDogGenetics`](beardog_auth::auth::BearDogGenetics).
pub mod engine;
/// Request/response types for spawn operations.
pub mod types;

pub use engine::GeneticSpawningEngine;
pub use types::{SpawnRequest, SpawnResult};
