// SPDX-License-Identifier: AGPL-3.0-only

//! Genetic primitives: entropy classification, human collection, key exchange, and spawning.

/// Classifies and mixes human vs machine entropy for key derivation policy.
pub mod entropy_hierarchy;
/// Live and legacy human entropy capture (keyboard, mouse, multi-modal).
pub mod human_entropy;
/// Delegated keys and lineage-aware exchange between primals.
pub mod key_exchange;
/// Creates new genetic identities from parent lineage and capability requests.
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
