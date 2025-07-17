//! BearDog Genetics Engine
//!
//! **Implements genetic algorithms for BearDog node reproduction and evolution.**
//!
//! This module was refactored from a large file to improve maintainability.
//! The genetics engine enables BearDog nodes to spawn offspring by combining their
//! cryptographic "genetics" - capabilities, security traits, and cryptographic material.
//!
//! ## Key Features
//!
//! * **Genetic Recombination**: Combining cryptographic "genetics" from multiple parent nodes
//! * **Mutation Operations**: Introducing controlled variations to maintain genetic diversity
//! * **Capability Evolution**: Allowing nodes to adapt their security capabilities over time
//! * **Spawn Restrictions**: Enforcing genetic lineage rules and security policies
//! * **Genesis Generation**: Creating foundational genetics for new nodes
//! * **Multi-party Workflows**: Human approval, automated consensus, and hybrid workflows
//! * **Cryptographic Lineage**: Verifiable parent-child relationships with Ed25519 signatures
//! * **RESTful API**: HTTP endpoints for external integration and management

use beardog_errors::BearDogResult;
use beardog_auth::auth::{SpawnPurpose, SecurityTraits, CryptoChromosome, NodeCapability, BearDogGenetics};
// use beardog_tunnel::tunnel::hsm::manager::HsmManager;

// Re-export public types and functions from submodules
pub use handlers::*;
pub use spawning::*;
pub use types::*;

// Module declarations
pub mod api;
pub mod entropy_hierarchy;
pub mod handlers;
pub mod human_entropy;
pub mod spawning;
pub mod types;

#[cfg(test)]
mod tests;

/// Public API for genetic spawning operations
pub struct GeneticsAPI {
    spawning_engine: crate::genetics::spawning::GeneticSpawningEngine,
    genetics_engine: crate::genetics::DefaultBearDogGeneticsEngine,
}

impl GeneticsAPI {
    /// Create a new genetics API instance
    pub fn new(genetics_store: std::sync::Arc<dyn GeneticsStore>, config: GeneticsConfig) -> Self {
        // Create a basic HSM manager for testing/development
        // let hsm_manager = std::sync::Arc::new(HsmManager::new());

        let spawning_engine = spawning::GeneticSpawningEngine::new(
            genetics_store.clone(),
            // hsm_manager,
            config.clone(),
        );

        let genetics_engine = handlers::DefaultBearDogGeneticsEngine::new(
            genetics_store,
            config,
        );

        Self {
            spawning_engine,
            genetics_engine,
        }
    }

    /// Spawn a new node with genetic recombination
    pub async fn spawn_node(&self, request: SpawnRequest) -> BearDogResult<SpawnResult> {
        self.spawning_engine.process_spawn_request(request).await
    }

    /// Create genesis genetics for a new node
    pub async fn create_genesis_genetics(
        &self,
        node_id: &str,
    ) -> BearDogResult<BearDogGenetics> {
        self.genetics_engine.create_genesis_genetics(node_id).await
    }

    /// Get genetics for a specific node
    pub async fn get_node_genetics(
        &self,
        node_id: &str,
    ) -> BearDogResult<BearDogGenetics> {
        self.genetics_engine.get_node_genetics(node_id).await
    }

    /// Breed genetics from multiple parents
    pub async fn breed_genetics(
        parent_genetics: &[BearDogGenetics],
        purpose: SpawnPurpose,
    ) -> BearDogResult<SpawnResult> {
        // Create a temporary engine for breeding
        let config = GeneticsConfig::default();
        let genetics_store = std::sync::Arc::new(spawning::InMemoryGeneticsStore::new());
        // let hsm_manager = std::sync::Arc::new(HsmManager::new());
        
        let spawning_engine = spawning::GeneticSpawningEngine::new(
            genetics_store.clone(),
            // hsm_manager,
            config.clone(),
        );

        let request = SpawnRequest {
            request_id: format!("spawn_{}", uuid::Uuid::new_v4()),
            requesting_parent: "breeder".to_string(),
            co_parents: vec![],
            purpose,
            resource_requirements: types::ResourceLimits::default(),
            workflow_type: types::BearDogWorkflowType::AutomatedConsensus {
                participating_nodes: vec!["breeder".to_string()],
                consensus_threshold: 0.5,
                max_decision_time: chrono::Duration::minutes(5),
            },
            created_at: chrono::Utc::now(),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(24),
            metadata: std::collections::HashMap::new(),
        };

        spawning_engine.process_spawn_request(request).await
    }

    /// Batch breed genetics from multiple parent sets
    pub async fn batch_breed_genetics(
        parent_sets: &[Vec<BearDogGenetics>],
        purpose: SpawnPurpose,
    ) -> BearDogResult<SpawnResult> {
        // Similar to breed_genetics but for multiple sets
        let config = GeneticsConfig::default();
        let genetics_store = std::sync::Arc::new(spawning::InMemoryGeneticsStore::new());
        // let hsm_manager = std::sync::Arc::new(HsmManager::new());
        
        let spawning_engine = spawning::GeneticSpawningEngine::new(
            genetics_store.clone(),
            // hsm_manager,
            config.clone(),
        );

        // For now, just use the first parent set
        let parent_genetics = parent_sets.first().cloned().unwrap_or_default();

        let request = SpawnRequest {
            request_id: format!("batch_spawn_{}", uuid::Uuid::new_v4()),
            requesting_parent: "batch_breeder".to_string(),
            co_parents: vec![],
            purpose,
            resource_requirements: types::ResourceLimits::default(),
            workflow_type: types::BearDogWorkflowType::AutomatedConsensus {
                participating_nodes: vec!["batch_breeder".to_string()],
                consensus_threshold: 0.5,
                max_decision_time: chrono::Duration::minutes(5),
            },
            created_at: chrono::Utc::now(),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(24),
            metadata: std::collections::HashMap::new(),
        };

        spawning_engine.process_spawn_request(request).await
    }
}

// Re-export types from submodules for convenience
pub use types::*;
pub use spawning::{SpawnRequest, SpawnResult, GeneticsStore, GeneticsConfig, ResourceLimits, BearDogWorkflowType};
pub use handlers::DefaultBearDogGeneticsEngine;
