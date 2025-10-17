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

#![deny(unsafe_code)]
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
pub mod ecosystem_evolution;
pub mod genetics;

// Re-export key types from genetics module
pub use genetics::entropy_hierarchy::{
    BiometricHash, EntropyClass, EntropyHierarchyConfig, EntropyHierarchyManager, EntropySeed,
    FusionAlgorithm, HumanEntropySource, HumanEntropyType, HumanIdentity, MachineEntropySource,
    MachineSourceType, MixingStrategy, OwnershipProof, SeedMetadata, VerificationLevel,
};
pub use genetics::spawning::{GeneticSpawningEngine, SpawnRequest, SpawnResult};

// Re-export ecosystem evolution genetics
pub use ecosystem_evolution::{
    BinaryAccessPattern, BinaryTrust, CoordinationModel, EcosystemContext, EcosystemGeneticEngine,
    EcosystemMembership, HierarchicalPattern, RelationshipHistory, SymbiosisType, TrustEvolution,
};

#[cfg(test)]
mod tests;

/// Main genetics manager that coordinates all genetic operations
#[derive(Debug, Clone)]
pub struct GeneticsManager {
    config: GeneticsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsConfig {
    /// Whether `entropy_collection` is enabled
    pub entropy_collection_enabled: bool,
    /// Whether `genetic_spawning` is enabled
    pub genetic_spawning_enabled: bool,
    /// Whether `ecosystem_evolution` is enabled
    pub ecosystem_evolution_enabled: bool,
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

#[must_use]
pub fn assess_genetics_health() -> String {
    // In a full implementation, this would check the health of all genetic systems
    "Healthy".to_string()
}
