use beardog_auth::auth::BearDogGenetics;
use beardog_errors::BearDogError;

// Type alias for genetics operations
pub type GeneticsResult<T> = Result<T, BearDogError>;

pub mod api;
// pub mod entropy_hierarchy; // Temporarily disabled - requires complete rewrite
// pub mod handlers; // Temporarily disabled - requires syntax fixes
pub mod human_entropy;
pub mod peer_to_peer_genetics; // Rebuilt with clean canonical patterns
pub mod spawning;
pub mod types;
pub mod zero_copy;

pub use spawning::{GeneticSpawningEngine, SpawnRequest, SpawnResult};
pub use types::InMemoryGeneticsStore;
pub use zero_copy::{GeneticsPool, LineageStats, LineageTracker};

// Re-export clean modules
pub use human_entropy::{EntropyFeatures, EntropySource, HumanEntropyCollector, PrivacyLevel};
pub use peer_to_peer_genetics::{
    DistributedSpawnRequest, GeneticsNode, NetworkConfig, NetworkStatus, P2PGeneticsNetwork,
    SpawnPriority,
};

pub use beardog_types::canonical::genetics::GeneticsConfig;

pub trait GeneticsStore: Send + Sync {
    fn store_genetics(&self, genetics: &BearDogGenetics) -> GeneticsResult<()>;
    fn get_genetics(&self, id: &str) -> GeneticsResult<BearDogGenetics>;
}

pub struct DefaultBearDogGeneticsEngine<S: GeneticsStore> {
    genetics_store: S,
    config: GeneticsConfig,
}

impl<S: GeneticsStore> DefaultBearDogGeneticsEngine<S> {
    pub fn new(genetics_store: S, config: GeneticsConfig) -> Self {
        Self {
            genetics_store,
            config,
        }
    }

    /// Get genetics engine configuration
    pub fn config(&self) -> &GeneticsConfig {
        &self.config
    }

    pub async fn create_genesis_genetics(&self, _node_id: &str) -> GeneticsResult<BearDogGenetics> {
        use uuid::Uuid;

        let genetics = BearDogGenetics {
            id: format!("genesis_{}", Uuid::new_v4()),
            generation: 0,
            fitness_score: self.config.fitness_threshold,
            security_clearance: beardog_auth::auth::SecurityClearance::Basic,
            ..Default::default()
        };

        self.genetics_store.store_genetics(&genetics)?;
        Ok(genetics)
    }

    pub async fn get_node_genetics(&self, node_id: &str) -> GeneticsResult<BearDogGenetics> {
        match self.genetics_store.get_genetics(node_id) {
            Ok(genetics) => Ok(genetics),
            Err(_) => self.create_genesis_genetics(node_id).await,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use beardog_errors::BearDogError;
    use std::sync::Arc;
    #[test]
    fn test_genetics_api_creation() {
        let store = Arc::new(api::InMemoryGeneticsStore::new());
        let config = GeneticsConfig::default();
        let _api = DefaultBearDogGeneticsEngine::new(store, config);
    }
}
