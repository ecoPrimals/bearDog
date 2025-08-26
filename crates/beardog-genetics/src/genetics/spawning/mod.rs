

pub mod engine;
pub mod recombination;
pub mod validation;
pub mod workflows;

pub mod types {
    use beardog_auth::auth::{
        BearDogGenetics, NodeCapability, ResourceLimits, SecurityClearance, SpawnPurpose,
    };
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SpawnRequest {

        pub purpose: SpawnPurpose,

        pub required_capabilities: Vec<NodeCapability>,

        pub resource_requirements: ResourceLimits,

        pub security_clearance: SecurityClearance,

        pub parent_genetics: Vec<BearDogGenetics>,

        pub metadata: HashMap<String, serde_json::Value>,
    }

    pub struct SpawnResult {

        pub genetics: BearDogGenetics,

        pub success: bool,

        pub messages: Vec<String>,

        pub metrics: HashMap<String, f64>,}

    impl Default for SpawnRequest {}

        fn default() -> Self {
            Self {
                purpose: SpawnPurpose::LoadBalancing,
                required_capabilities: vec![],
                resource_requirements: ResourceLimits::default(),
                security_clearance: SecurityClearance::Basic,
                parent_genetics: vec![],
                metadata: HashMap::with_capacity(16),
            }
        }
}

pub use engine::GeneticSpawningEngine;
pub use types::{SpawnRequest, SpawnResult};
pub use validation::SpawnValidation;
