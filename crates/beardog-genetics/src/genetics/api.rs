//! Genetics API Module
//!
//! This module provides the main API interface for genetics operations.

use super::{DefaultBearDogGeneticsEngine, GeneticsResult, GeneticsStore};
use beardog_auth::auth::BearDogGenetics;
use beardog_types::canonical::genetics::GeneticsConfig;

/// Main genetics API interface
pub struct GeneticsAPI<S: GeneticsStore> {
    engine: DefaultBearDogGeneticsEngine<S>,
}

impl<S: GeneticsStore> GeneticsAPI<S> {
    pub fn new(store: S, config: GeneticsConfig) -> Self {
        Self {
            engine: DefaultBearDogGeneticsEngine::new(store, config),
        }
    }

    pub async fn create_node_genetics(&self, node_id: &str) -> GeneticsResult<BearDogGenetics> {
        self.engine.create_genesis_genetics(node_id).await
    }

    pub async fn get_node_genetics(&self, node_id: &str) -> GeneticsResult<BearDogGenetics> {
        self.engine.get_node_genetics(node_id).await
    }
}

// Simple in-memory store for API testing
#[derive(Debug, Default)]
pub struct InMemoryGeneticsStore {
    // Simplified for API compatibility
}

impl GeneticsStore for InMemoryGeneticsStore {
    fn store_genetics(&self, _genetics: &BearDogGenetics) -> GeneticsResult<()> {
        Ok(())
    }

    fn get_genetics(&self, _id: &str) -> GeneticsResult<BearDogGenetics> {
        Ok(BearDogGenetics::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_genetics_api() -> GeneticsResult<()> {
        let store = InMemoryGeneticsStore::default();
        let config = GeneticsConfig::default();
        let api = GeneticsAPI::new(store, config);

        let genetics = api.create_node_genetics("test-node").await?;
        assert!(!genetics.id.is_empty());

        Ok(())
    }
}
