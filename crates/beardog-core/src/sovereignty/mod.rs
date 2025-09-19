

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


pub mod ecosystem;
pub mod genesis;
pub mod types;

pub use crate::ecosystem_simple::{
    SimpleEcosystemConfig as EcosystemConfig, SimpleEcosystemManager as EcosystemManager,
};
pub use genesis::{GenesisConfig, GenesisManager};
pub use types::{SovereigntyConfig, SovereigntyLevel};
