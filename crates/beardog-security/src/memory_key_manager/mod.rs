// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod config;
pub mod metrics;

pub use config::*;
pub use metrics::*;

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadata {
    pub id: String,
    /// The created at value
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The key type value
    pub key_type: String,
}

#[derive(Debug, Clone)]
pub struct MemoryKeyManager {
    keys: Arc<Mutex<HashMap<String, Vec<u8>>>>,
}

impl MemoryKeyManager {
    /// New operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new(_config: MemoryKeyConfig) -> Result<Self, BearDogError> {
        Ok(Self {
            keys: Arc::new(Mutex::new(HashMap::with_capacity(100))),
        })
    }

    /// Generate a new key and return its ID
    ///
    /// # Errors
    /// Returns an error if key generation fails.
    pub fn generate_key(&self) -> Result<String, BearDogError> {
        let key_id = format!("key_{}", Uuid::new_v4());
        let key_data = Self::generate_random_key(32)?;

        let mut keys = self
            .keys
            .lock()
            .map_err(|_| BearDogError::internal("Failed to acquire lock".to_string()))?;
        keys.insert(key_id.clone(), key_data);

        Ok(key_id)
    }

    /// Check if a key exists
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn key_exists(&self, key_id: &str) -> Result<bool, BearDogError> {
        let keys = self
            .keys
            .lock()
            .map_err(|_| BearDogError::internal("Failed to acquire lock".to_string()))?;
        Ok(keys.contains_key(key_id))
    }

    /// Store Key operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn store_key(
        &mut self,
        payload: &[u8],
        _metadata: KeyMetadata,
    ) -> Result<String, BearDogError> {
        let key_id = format!("key_{}", Uuid::new_v4());
        let mut keys = self
            .keys
            .lock()
            .map_err(|_| BearDogError::internal("Failed to acquire lock".to_string()))?;
        keys.insert(key_id.clone(), payload.to_vec());
        Ok(key_id)
    }

    /// Get Key operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Gets key
    /// Gets key
    pub fn get_key(&self, key_id: &str) -> Result<Vec<u8>, BearDogError> {
        let keys = self
            .keys
            .lock()
            .map_err(|_| BearDogError::internal("Failed to acquire lock".to_string()))?;
        keys.get(key_id)
            .cloned()
            .ok_or_else(|| BearDogError::internal("Key not found".to_string()))
    }

    /// Delete Key operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Removes key
    /// Removes key
    pub fn delete_key(&mut self, key_id: &str) -> Result<(), BearDogError> {
        let mut keys = self
            .keys
            .lock()
            .map_err(|_| BearDogError::internal("Failed to acquire lock".to_string()))?;
        keys.remove(key_id);
        Ok(())
    }

    /// List Keys operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub const fn list_keys(&self) -> Result<Vec<KeyMetadata>, BearDogError> {
        Ok(vec![]) // Simplified implementation
    }

    /// Generate random key data
    fn generate_random_key(size: usize) -> Result<Vec<u8>, BearDogError> {
        use rand::RngCore;
        let mut key = vec![0u8; size];
        rand::thread_rng().fill_bytes(&mut key);
        Ok(key)
    }
}
