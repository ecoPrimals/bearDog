

use beardog_auth::auth::BearDogGenetics;
use beardog_errors::BearDogResult;

pub mod api;
pub mod entropy_hierarchy;
pub mod handlers;
pub mod human_entropy;
pub mod peer_to_peer_genetics;
pub mod spawning;
pub mod types;
pub mod zero_copy;

pub use spawning::{GeneticSpawningEngine, SpawnRequest, SpawnResult};
pub use types::InMemoryGeneticsStore;

pub use zero_copy::{GeneticsPool, LineageStats, LineageTracker};

use serde::{Deserialize, Serialize};

pub use beardog_types::canonical::genetics::GeneticsConfig;

pub trait GeneticsStore: Send + Sync {
    fn store_genetics(&self, genetics: &BearDogGenetics) -> GeneticsResult<()>;
}

pub struct DefaultBearDogGeneticsEngine<S: GeneticsStore> {
    genetics_store: S,
    config: GeneticsConfig,
}
impl<S: GeneticsStore> Default for DefaultBearDogGeneticsEngine<S> {
    fn default() -> Self {
        Self::new(S::default(), GeneticsConfig::default())
    }
}
impl<S: GeneticsStore> DefaultBearDogGeneticsEngine<S> {

    pub fn new(genetics_store: S, config: GeneticsConfig) -> Self {
        Self {
            genetics_store,
            config,
        }
    }

    pub async fn create_genesis_genetics(&self, node_id: &str) -> GeneticsResult<BearDogGenetics> {

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

    pub async fn get_node_genetics(&self, _node_id: &str) -> GeneticsResult<BearDogGenetics> {

        Ok(BearDogGenetics::default())
    }
}

pub struct GeneticsAPI<S: GeneticsStore> {
    spawning_engine: spawning::GeneticSpawningEngine,
    genetics_engine: DefaultBearDogGeneticsEngine<S>,
    genetics_store: S,
}

impl<S: GeneticsStore> GeneticsAPI<S> {

    pub fn new(genetics_store: S, config: GeneticsConfig) -> Self {
        let spawning_engine = spawning::GeneticSpawningEngine::new();
        let genetics_engine = DefaultBearDogGeneticsEngine::new(genetics_store.clone(), config);
        Self {
            spawning_engine,
            genetics_engine,
            genetics_store,
        }
    }

    pub async fn get_node_genetics(&self, node_id: &str) -> GeneticsResult<BearDogGenetics> {
        self.genetics_engine.get_node_genetics(node_id).await
    }

    pub async fn spawn_node(
        &self,
        request: spawning::SpawnRequest,
    ) -> GeneticsResult<spawning::SpawnResult> {
        self.spawn(request).await
    }

    pub async fn spawn(
        self,
        request: spawning::SpawnRequest,
    ) -> GeneticsResult<spawning::SpawnResult> {
        self.spawning_engine.spawn_genetics(request).await
    }

    pub fn get_genetics(&self, genetics_id: &str) -> GeneticsResult<BearDogGenetics> {
        self.genetics_store.get_genetics(genetics_id)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::sync::Arc;
use beardog_errors::{BearDogError, BearDogResult};
    #[test]
    fn test_genetics_api_creation() {
        let store = Arc::new(api::InMemoryGeneticsStore::new());
        let config = GeneticsConfig::default();
        let _api = GeneticsAPI::new(store, config);
