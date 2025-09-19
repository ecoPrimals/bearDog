// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod entropy_hierarchy;
pub mod human_entropy;
pub mod spawning;

// Re-export key types with explicit imports to avoid conflicts
pub use entropy_hierarchy::{
    BiometricHash, EntropyClass, EntropyHierarchyConfig, EntropyHierarchyManager,
    EntropyMixingEngine, EntropyMonitor, EntropySeed, EntropyValidator, HumanEntropyType,
    MachineEntropySource, OwnershipProof,
};
pub use human_entropy::*;
pub use spawning::{GeneticSpawningEngine, SpawnRequest, SpawnResult};
