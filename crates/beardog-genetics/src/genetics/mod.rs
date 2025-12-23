// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod entropy_hierarchy;
pub mod human_entropy; // Human entropy collection (keyboard, mouse, sensors)
pub mod key_exchange;
pub mod spawning;

// Re-export key types with explicit imports to avoid conflicts
pub use entropy_hierarchy::{
    BiometricHash, EntropyClass, EntropyHierarchyConfig, EntropyHierarchyManager,
    EntropyMixingEngine, EntropyMonitor, EntropySeed, EntropyValidator, HumanEntropyType,
    MachineEntropySource, OwnershipProof,
};
pub use human_entropy::*;
pub use key_exchange::{
    DelegatedKey, GeneticKeyExchange, KeyExchangeConfig, KeyExchangeResult, KeyLineage,
};
pub use spawning::{GeneticSpawningEngine, SpawnRequest, SpawnResult};
