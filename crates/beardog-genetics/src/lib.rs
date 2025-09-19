// BearDog Genetics - Advanced Genetic Algorithms and Entropy Management
//
// This crate provides comprehensive genetic algorithms, entropy hierarchy management,
// and ecosystem evolution capabilities for the BearDog security system.

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

/// Main genetics manager that coordinates all genetic operations
#[derive(Debug, Clone)]
pub struct GeneticsManager {
    config: GeneticsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsConfig {
    /// Whether entropy_collection is enabled
    pub entropy_collection_enabled: bool,
    /// Whether genetic_spawning is enabled
    pub genetic_spawning_enabled: bool,
    /// Whether ecosystem_evolution is enabled
    pub ecosystem_evolution_enabled: bool,
    pub human_entropy_validation: bool,
    /// Whether authorization_genetics is enabled
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
    pub fn new() -> Self {
        Self {
            config: GeneticsConfig::default(),
        }
    }

    /// Create a new genetics manager with custom configuration
    /// Creates instance with config
    pub fn with_config(config: GeneticsConfig) -> Self {
        Self { config }
    }

    /// Get the current configuration
    pub fn config(&self) -> &GeneticsConfig {
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

pub fn assess_genetics_health() -> String {
    // In a full implementation, this would check the health of all genetic systems
    "Healthy".to_string()
}

#[cfg(test)]
mod comprehensive_tests;
