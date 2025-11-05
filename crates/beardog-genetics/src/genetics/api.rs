// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::{DefaultBearDogGeneticsEngine, GeneticsStore};
use beardog_auth::auth::BearDogGenetics;
use beardog_errors::BearDogError;
use beardog_types::canonical::config::GeneticsConfig;

pub struct GeneticsAPI<S: GeneticsStore> {
    engine: DefaultBearDogGeneticsEngine<S>,
}

impl<S: GeneticsStore> GeneticsAPI<S> {
    /// New operation.
    /// Creates a new instance
    pub fn new(store: S, config: GeneticsConfig) -> Self {
        Self {
            engine: DefaultBearDogGeneticsEngine::new(store, config),
        }
    }

    /// Create Genesis Genetics operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Creates genesis_genetics
    /// Creates genesis_genetics
    pub fn create_genesis_genetics(&self, _node_id: &str) -> Result<BearDogGenetics, BearDogError> {
        self.engine.generate_genesis_genetics()
    }

    /// Get Node Genetics operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Gets node_genetics
    /// Gets node_genetics
    pub fn get_node_genetics(&self, node_id: &str) -> Result<BearDogGenetics, BearDogError> {
        self.engine.get_node_genetics(node_id)
    }
}

#[derive(Debug, Default)]
pub struct InMemoryGeneticsStore {}

impl GeneticsStore for InMemoryGeneticsStore {
    fn store_genetics(&self, _genetics: &BearDogGenetics) -> Result<(), BearDogError> {
        Ok(())
    }

    /// Gets genetics
    fn get_genetics(&self, _id: &str) -> Result<BearDogGenetics, BearDogError> {
        Ok(BearDogGenetics::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[tokio::test]
    fn test_genetics_api() {
        let store = InMemoryGeneticsStore::default();
        let config = GeneticsConfig {
            enabled: true,
            ..GeneticsConfig::default()
        };
        let api = GeneticsAPI::new(store, config);

        let result = api.create_genesis_genetics("test_node");
        assert!(result.is_ok());
    }
}
