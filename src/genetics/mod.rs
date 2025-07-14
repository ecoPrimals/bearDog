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
        let hsm_manager = std::sync::Arc::new(crate::tunnel::hsm::manager::HsmManager::new());

        let spawning_engine =
            spawning::GeneticSpawningEngine::new(genetics_store.clone(), hsm_manager, config.clone());
        let genetics_engine = DefaultBearDogGeneticsEngine::new(genetics_store, config);

        Self {
            spawning_engine,
            genetics_engine,
        }
    }

    /// Create a new genetics API instance with custom HSM manager
    pub fn with_hsm_manager(
        genetics_store: std::sync::Arc<dyn GeneticsStore>,
        hsm_manager: std::sync::Arc<crate::tunnel::hsm::manager::HsmManager>,
        config: GeneticsConfig,
    ) -> Self {
        let spawning_engine =
            spawning::GeneticSpawningEngine::new(genetics_store.clone(), hsm_manager, config.clone());
        let genetics_engine = DefaultBearDogGeneticsEngine::new(genetics_store, config);

        Self {
            spawning_engine,
            genetics_engine,
        }
    }

    /// Submit a spawn request for processing
    pub async fn spawn_node(&self, request: SpawnRequest) -> crate::BearDogResult<SpawnResult> {
        self.spawning_engine.process_spawn_request(request).await
    }

    /// Generate genesis genetics for a new node
    pub async fn create_genesis_node(
        &self,
        node_id: &str,
    ) -> crate::BearDogResult<crate::auth::BearDogGenetics> {
        self.genetics_engine
            .generate_genesis_genetics(node_id)
            .await
    }

    /// Get genetics for an existing node
    pub async fn get_node_genetics(
        &self,
        node_id: &str,
    ) -> crate::BearDogResult<crate::auth::BearDogGenetics> {
        self.genetics_engine.get_node_genetics(node_id).await
    }
}

/// Quick spawn helper for simple use cases
pub async fn quick_spawn_automated_consensus(
    genetics_api: &GeneticsAPI,
    requesting_parent: &str,
    co_parents: Vec<String>,
    purpose: crate::auth::SpawnPurpose,
    participating_nodes: Vec<String>,
) -> crate::BearDogResult<SpawnResult> {
    let request = SpawnRequest {
        request_id: uuid::Uuid::new_v4().to_string(),
        requesting_parent: requesting_parent.to_string(),
        co_parents,
        purpose,
        resource_requirements: ResourceLimits {
            max_cpu_percent: 50.0,
            max_memory_mb: 2048,
            max_storage_gb: 10,
            max_network_mbps: 100,
            allowed_jurisdictions: vec!["US".to_string(), "EU".to_string()],
            temporal_windows: vec![],
        },
        workflow_type: BearDogWorkflowType::AutomatedConsensus {
            participating_nodes,
            consensus_threshold: 0.67, // 2/3 majority
            max_decision_time: chrono::Duration::minutes(5),
        },
        created_at: chrono::Utc::now(),
        expires_at: chrono::Utc::now() + chrono::Duration::hours(24),
        metadata: std::collections::HashMap::new(),
    };

    genetics_api.spawn_node(request).await
}

/// Quick spawn helper for human approval workflows
pub async fn quick_spawn_human_approval(
    genetics_api: &GeneticsAPI,
    requesting_parent: &str,
    co_parents: Vec<String>,
    purpose: crate::auth::SpawnPurpose,
    approver_roles: Vec<String>,
    min_approvals: u32,
) -> crate::BearDogResult<SpawnResult> {
    let request = SpawnRequest {
        request_id: uuid::Uuid::new_v4().to_string(),
        requesting_parent: requesting_parent.to_string(),
        co_parents,
        purpose,
        resource_requirements: ResourceLimits {
            max_cpu_percent: 80.0,
            max_memory_mb: 4096,
            max_storage_gb: 50,
            max_network_mbps: 1000,
            allowed_jurisdictions: vec!["US".to_string()],
            temporal_windows: vec![],
        },
        workflow_type: BearDogWorkflowType::HumanApprovalRequired {
            approver_roles,
            min_approvals,
            approval_timeout: chrono::Duration::hours(8), // Business hours
        },
        created_at: chrono::Utc::now(),
        expires_at: chrono::Utc::now() + chrono::Duration::days(7),
        metadata: std::collections::HashMap::new(),
    };

    genetics_api.spawn_node(request).await
}
