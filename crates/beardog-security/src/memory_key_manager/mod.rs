pub mod config;
pub mod metrics;

pub use config::*;
pub use metrics::*;

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadata {
    pub purpose: String,
}

#[derive(Debug)]
pub struct MemoryKeyManager {
    keys: HashMap<String, Vec<u8>>,
}

impl MemoryKeyManager {
    pub async fn new(_config: MemoryKeyConfig) -> Result<Self, BearDogError> {
        Ok(Self {
            keys: HashMap::new(),
        })
    }

    pub fn store_key(
        &mut self,
        data: &[u8],
        _metadata: KeyMetadata,
    ) -> Result<String, BearDogError> {
        let key_id = format!("key_{}", self.keys.len());
        self.keys.insert(key_id.clone(), data.to_vec());
        Ok(key_id)
    }

    pub fn get_key(&self, key_id: &str) -> Result<Vec<u8>, BearDogError> {
        self.keys
            .get(key_id)
            .cloned()
            .ok_or_else(|| BearDogError::internal("Key not found"))
    }

    pub fn delete_key(&mut self, key_id: &str) -> Result<(), BearDogError> {
        self.keys.remove(key_id);
        Ok(())
    }

    pub async fn list_keys(&self) -> Result<Vec<KeyMetadata>, BearDogError> {
        Ok(vec![]) // Simplified implementation
    }
}
