// SPDX-License-Identifier: AGPL-3.0-or-later

// Module documentation
//
//! Memory-based key manager module
//!
//! Provides in-memory key management for BearDog with encryption and metrics.

/// Configuration module for memory key manager
pub mod config;
/// Metrics module for key manager operations
pub mod metrics;

pub use config::{KeyManagerConfig, KeyStorageConfig, MemoryKeyConfig};
pub use metrics::{ExtendedMetrics, KeyManagerMetrics, OperationMetrics};

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// Key metadata structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadata {
    /// Unique key identifier
    pub id: String,
    /// Timestamp when key was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Type of the key (e.g., "AES-256", "RSA-2048")
    pub key_type: String,
}

/// In-memory key manager
///
/// Provides secure in-memory key storage with encryption capabilities
#[derive(Debug, Clone)]
pub struct MemoryKeyManager {
    keys: Arc<RwLock<BTreeMap<String, Vec<u8>>>>,
}

impl MemoryKeyManager {
    /// New operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new(_config: MemoryKeyConfig) -> Result<Self, BearDogError> {
        Ok(Self {
            keys: Arc::new(RwLock::new(BTreeMap::new())),
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
            .write()
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
            .read()
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
            .write()
            .map_err(|_| BearDogError::internal("Failed to acquire lock".to_string()))?;
        keys.insert(key_id.clone(), payload.to_vec());
        Ok(key_id)
    }

    /// Get Key operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Gets key
    pub fn get_key(&self, key_id: &str) -> Result<Vec<u8>, BearDogError> {
        let keys = self
            .keys
            .read()
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
    pub fn delete_key(&mut self, key_id: &str) -> Result<(), BearDogError> {
        let mut keys = self
            .keys
            .write()
            .map_err(|_| BearDogError::internal("Failed to acquire lock".to_string()))?;
        keys.remove(key_id);
        Ok(())
    }

    /// List Keys operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn list_keys(&self) -> Result<Vec<KeyMetadata>, BearDogError> {
        let keys = self
            .keys
            .read()
            .map_err(|_| BearDogError::internal("Failed to acquire lock".to_string()))?;

        let mut listed: Vec<KeyMetadata> = keys
            .iter()
            .map(|(id, payload)| KeyMetadata {
                id: id.clone(),
                created_at: chrono::DateTime::UNIX_EPOCH,
                key_type: match payload.len() {
                    32 => "AES-256".to_string(),
                    _ => "symmetric".to_string(),
                },
            })
            .collect();
        listed.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(listed)
    }

    /// Generate random key data
    fn generate_random_key(size: usize) -> Result<Vec<u8>, BearDogError> {
        use rand::RngCore;
        let mut key = vec![0u8; size];
        rand::rng().fill_bytes(&mut key);
        Ok(key)
    }
}
