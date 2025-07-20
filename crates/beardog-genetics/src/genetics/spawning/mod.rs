//! Spawning subsystem for BearDog genetics
//!
//! This module contains all functionality related to genetic spawning,
//! including validation, workflows, recombination, and core engine logic.

pub mod engine;
pub mod recombination;
pub mod validation;
pub mod workflows;

// Temporary types module to support compilation during refactor
pub mod types {
    use beardog_auth::auth::{
        BearDogGenetics, NodeCapability, ResourceLimits, SecurityClearance, SpawnPurpose, TaskType,
    };
    use beardog_errors::BearDogResult;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    /// Spawn request for genetic spawning operations
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SpawnRequest {
        /// Purpose for this spawn
        pub purpose: SpawnPurpose,
        /// Required capabilities
        pub required_capabilities: Vec<NodeCapability>,
        /// Resource requirements
        pub resource_requirements: ResourceLimits,
        /// Security clearance required
        pub security_clearance: SecurityClearance,
        /// Parent genetics (if any)
        pub parent_genetics: Vec<BearDogGenetics>,
        /// Metadata for the spawn
        pub metadata: HashMap<String, serde_json::Value>,
    }

    /// Result of a genetic spawning operation
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SpawnResult {
        /// The spawned genetics
        pub genetics: BearDogGenetics,
        /// Success status
        pub success: bool,
        /// Any warnings or notes
        pub messages: Vec<String>,
        /// Performance metrics
        pub metrics: HashMap<String, f64>,
    }

    impl Default for SpawnRequest {
        fn default() -> Self {
            Self {
                purpose: SpawnPurpose::LoadBalancing,
                required_capabilities: vec![],
                resource_requirements: ResourceLimits::default(),
                security_clearance: SecurityClearance::Basic,
                parent_genetics: vec![],
                metadata: HashMap::new(),
            }
        }
    }
}

// Re-export commonly used types
pub use engine::GeneticSpawningEngine;
pub use types::{SpawnRequest, SpawnResult};
pub use validation::SpawnValidation;
