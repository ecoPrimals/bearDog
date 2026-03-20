// SPDX-License-Identifier: AGPL-3.0-only
#![forbid(unsafe_code)]

//! # `BearDog` Genetics - Advanced Genetic Algorithms and Entropy Management
//!
//! Comprehensive genetic algorithms, entropy hierarchy management, and ecosystem evolution
//! capabilities for the `BearDog` security system with human-centric entropy generation.
//!
//! ## Features
//!
//! - **Genetic Algorithms**: Advanced evolution and optimization algorithms
//! - **Entropy Hierarchy**: Human and machine entropy source management
//! - **Genetic Spawning**: Dynamic primal generation and evolution

#![cfg_attr(test, allow(clippy::expect_used))]
#![cfg_attr(test, allow(clippy::unwrap_used))]

//! - **Ecosystem Evolution**: Binary pattern elimination and relationship evolution
//! - **Biometric Entropy**: Human-owned entropy without corporate control
//!
//! ## Example
//!
//! ```rust,no_run
//! use beardog_genetics::{GeneticsManager, GeneticsConfig};
//!
//! # fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Initialize genetics manager
//! let genetics = GeneticsManager::new();
//!
//! // Use genetic algorithms for key evolution
//! // (implementation details)
//! # Ok(())
//! # }
//! ```
//!
//! ## Architecture
//!
//! The genetics system implements several key patterns:
//! - **Human Entropy Sovereignty**: Humans own their biometric entropy
//! - **Genetic Evolution**: Keys and algorithms evolve over time
//! - **Ecosystem Relationships**: Non-binary relationship modeling
//! - **Zero Corporate Control**: No extraction or surveillance
//!
//! ## Safety
//!
//! All genetic operations maintain memory safety with zero unsafe code.

use serde::{Deserialize, Serialize};

// Core genetics modules
/// Encrypted BirdSong discovery, lineage chains, and genesis enrollment.
pub mod birdsong;
/// Signed, self-enforcing key constraints and evolution.
pub mod constraints;
/// Non-binary trust models and ecosystem relationship evolution.
pub mod ecosystem_evolution;
/// Entropy hierarchy, human entropy, key exchange, and genetic spawning primitives.
pub mod genetics;

// Re-export constraint types
pub use constraints::{
    BehavioralConstraint, ConstraintEnforcementPolicy, ConstraintEnforcer,
    ConstraintEvolutionEngine, ConstraintViolationError, DataAccessConstraint, EvolutionTrigger,
    KeyConstraints, KeyOperation, LifetimeConstraint, ScopeConstraint, SignedConstraints,
};

// Re-export key types from genetics module
pub use genetics::entropy_hierarchy::{
    BiometricHash, EntropyClass, EntropyHierarchyConfig, EntropyHierarchyManager, EntropySeed,
    FusionAlgorithm, HumanEntropySource, HumanEntropyType, HumanIdentity, MachineEntropySource,
    MachineSourceType, MixingStrategy, OwnershipProof, SeedMetadata, VerificationLevel,
};

// Re-export human entropy collection types
pub use genetics::human_entropy::{
    HumanEntropyConfig, InteractionCaptureConfig, InteractionCaptureResult,
    InteractionEntropyCollector, InteractionEvent, InteractionMetrics, InteractionType,
    MultiModalHumanEntropyCollector,
};
pub use genetics::key_exchange::{
    DelegatedKey, GeneticKeyExchange, KeyExchangeConfig, KeyExchangeResult, KeyLineage,
};
pub use genetics::spawning::{GeneticSpawningEngine, SpawnRequest, SpawnResult};

// Re-export ecosystem evolution genetics
pub use ecosystem_evolution::{
    BinaryAccessPattern, BinaryTrust, CoordinationModel, EcosystemContext, EcosystemGeneticEngine,
    EcosystemMembership, HierarchicalPattern, RelationshipHistory, SymbiosisType, TrustEvolution,
};

// Re-export BirdSong types
pub use birdsong::{
    BirdSongEncryption, BirdSongManager, LineageChain, LineageChainManager, LineageHint,
    LineageKeyDerivation, LineageNode, LineageProof, LineageProofManager,
};

#[cfg(test)]
mod tests;

/// Coordinates genetics subsystems: entropy, spawning, ecosystem evolution, and BirdSong lineage.
#[derive(Debug, Clone)]
pub struct GeneticsManager {
    config: GeneticsConfig,
}

/// Feature flags and policy toggles for the genetics stack.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsConfig {
    /// Whether `entropy_collection` is enabled
    pub entropy_collection_enabled: bool,
    /// Whether `genetic_spawning` is enabled
    pub genetic_spawning_enabled: bool,
    /// Whether `ecosystem_evolution` is enabled
    pub ecosystem_evolution_enabled: bool,
    /// When true, human-sourced entropy must pass quality and live-feed checks before use.
    pub human_entropy_validation: bool,
    /// Whether `authorization_genetics` is enabled
    pub authorization_genetics_enabled: bool,
}

impl Default for GeneticsConfig {
    fn default() -> Self {
        Self {
            entropy_collection_enabled: true,
            genetic_spawning_enabled: true,
            ecosystem_evolution_enabled: true,
            human_entropy_validation: true,
            authorization_genetics_enabled: true,
        }
    }
}

impl GeneticsManager {
    /// Create a new genetics manager with default configuration
    /// Creates a new instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: GeneticsConfig::default(),
        }
    }

    /// Create a new genetics manager with custom configuration
    /// Creates instance with config
    #[must_use]
    pub const fn with_config(config: GeneticsConfig) -> Self {
        Self { config }
    }

    /// Get the current configuration
    #[must_use]
    pub const fn config(&self) -> &GeneticsConfig {
        &self.config
    }

    /// Update the configuration
    /// Updates config
    /// Updates config
    pub fn update_config(&mut self, config: GeneticsConfig) {
        self.config = config;
    }
}

impl Default for GeneticsManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Returns a short human-readable genetics subsystem status (e.g. `"Healthy"`).
///
/// Intended for diagnostics; a full deployment would aggregate real metrics from each subsystem.
#[must_use]
pub fn assess_genetics_health() -> String {
    // In a full implementation, this would check the health of all genetic systems
    "Healthy".to_string()
}
