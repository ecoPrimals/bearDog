// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod spawner;
pub mod traits;
pub mod types;

pub use spawner::EcosystemGeneticSpawner;
pub use traits::{EcosystemGeneticTrait, EcosystemPrimalClient, EcosystemResourceAvailability};
pub use types::*;
