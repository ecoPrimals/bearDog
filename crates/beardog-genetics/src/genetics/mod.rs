//! BearDog Genetics Engine
//!
//! **Implements genetic algorithms for BearDog node reproduction and evolution.**
//!
//! This module was refactored from a large file to improve maintainability.
//! The genetics engine enables BearDog nodes to spawn offspring by combining their
//! cryptographic "genetics" - capabilities, security traits, and cryptographic material.

use beardog_auth::auth::BearDogGenetics;
use beardog_errors::BearDogResult;

// Module declarations
pub mod api;
pub mod entropy_hierarchy;
pub mod handlers;
pub mod human_entropy;
pub mod peer_to_peer_genetics;
pub mod spawning;

pub mod types;
pub mod zero_copy;

// Legacy support - keep the large file for now but re-export from modular version
pub mod zero_copy_spawning_legacy;

// Selective re-exports to avoid conflicts and warnings
pub use spawning::{GeneticSpawningEngine, SpawnRequest, SpawnResult};
pub use types::InMemoryGeneticsStore;

// Zero-copy spawning re-enabled - types are now aligned with current auth module
pub use zero_copy::{GeneticsPool, LineageStats, LineageTracker};
// Note: zero_copy_spawning_legacy remains disabled as it will be replaced by the modular version

// Configuration and core types
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsConfig {
    pub max_generations: u32,
    pub mutation_rate: f64,
    pub crossover_rate: f64,
    pub population_size: usize,
    pub selection_pressure: f64,
    pub diversity_threshold: f64,
    pub enable_adaptive_mutations: bool,
    pub parallel_processing: bool,
}

impl Default for GeneticsConfig {
    fn default() -> Self {
        Self {
            max_generations: 1000,
            mutation_rate: 0.1,
            crossover_rate: 0.8,
            population_size: 100,
            selection_pressure: 0.7,
            diversity_threshold: 0.3,
            enable_adaptive_mutations: true,
            parallel_processing: true,
        }
    }
}

// Define a simple GeneticsStore trait
pub trait GeneticsStore: Send + Sync {
    fn store_genetics(&self, genetics: &BearDogGenetics) -> BearDogResult<()>;
    fn get_genetics(&self, id: &str) -> BearDogResult<BearDogGenetics>;
    fn delete_genetics(&self, id: &str) -> BearDogResult<()>;
}

/// Simple genetics engine implementation
pub struct DefaultBearDogGeneticsEngine;

impl Default for DefaultBearDogGeneticsEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultBearDogGeneticsEngine {
    /// Create a new genetics engine
    pub fn new() -> Self {
        Self
    }

    /// Create genesis genetics for a node
    pub async fn create_genesis_genetics(&self, node_id: &str) -> BearDogResult<BearDogGenetics> {
        // Create basic genesis genetics
        Ok(BearDogGenetics {
            id: format!("genesis_{node_id}"),
            crypto_chromosomes: Vec::new(),
            security_traits: beardog_auth::auth::SecurityTraits::default(),
            capabilities: Vec::new(),
            spawn_restrictions: Vec::new(),
            generation: 0,
            parent_genetics: None,
            mutations: Vec::new(),
            fitness_score: 0.5,
            security_clearance: beardog_auth::auth::SecurityClearance::Basic,
            specializations: vec![beardog_auth::auth::NodeSpecialization::GeneralPurpose],
        })
    }

    /// Get genetics for a node
    pub async fn get_node_genetics(&self, _node_id: &str) -> BearDogResult<BearDogGenetics> {
        // Return default genetics
        Ok(BearDogGenetics::default())
    }
}

// Implement for DefaultBearDogGeneticsEngine instead of non-existent DefaultBearDogGeneticsStore
impl GeneticsStore for DefaultBearDogGeneticsEngine {
    fn store_genetics(&self, _genetics: &BearDogGenetics) -> BearDogResult<()> {
        // Placeholder implementation
        Ok(())
    }

    fn get_genetics(&self, _genetics_id: &str) -> BearDogResult<BearDogGenetics> {
        // Placeholder implementation
        Ok(BearDogGenetics::default())
    }

    fn delete_genetics(&self, _genetics_id: &str) -> BearDogResult<()> {
        // Placeholder implementation
        Ok(())
    }
}

/// Public API for genetic spawning operations
pub struct GeneticsAPI {
    spawning_engine: spawning::GeneticSpawningEngine,
    genetics_engine: DefaultBearDogGeneticsEngine,
    genetics_store: std::sync::Arc<dyn GeneticsStore>,
}

impl GeneticsAPI {
    /// Create a new genetics API instance
    pub fn new(genetics_store: std::sync::Arc<dyn GeneticsStore>, _config: GeneticsConfig) -> Self {
        let spawning_engine = spawning::GeneticSpawningEngine::new();

        Self {
            spawning_engine,
            genetics_engine: DefaultBearDogGeneticsEngine::new(),
            genetics_store: genetics_store.clone(),
        }
    }

    /// Create genesis genetics for a node
    pub async fn create_genesis_genetics(&self, node_id: &str) -> BearDogResult<BearDogGenetics> {
        self.genetics_engine.create_genesis_genetics(node_id).await
    }

    /// Get genetics for a node (alias for get_genetics)
    pub async fn get_node_genetics(&self, node_id: &str) -> BearDogResult<BearDogGenetics> {
        self.genetics_engine.get_node_genetics(node_id).await
    }

    /// Spawn a new node (alias for spawn)
    pub async fn spawn_node(
        &self,
        request: spawning::SpawnRequest,
    ) -> BearDogResult<spawning::SpawnResult> {
        self.spawn(request).await
    }

    /// Spawn a new genetics instance
    pub async fn spawn(
        &self,
        request: spawning::SpawnRequest,
    ) -> BearDogResult<spawning::SpawnResult> {
        self.spawning_engine.spawn_genetics(request).await
    }

    /// Get genetics by ID
    pub fn get_genetics(&self, genetics_id: &str) -> BearDogResult<BearDogGenetics> {
        self.genetics_store.get_genetics(genetics_id)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_genetics_api_creation() {
        let store = Arc::new(api::InMemoryGeneticsStore::new());
        let config = GeneticsConfig::default();
        let _api = GeneticsAPI::new(store, config);
    }
}
