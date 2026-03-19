// SPDX-License-Identifier: AGPL-3.0-only

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod engine;
pub mod types;

pub use engine::GeneticSpawningEngine;
pub use types::{SpawnRequest, SpawnResult};
