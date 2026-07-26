// SPDX-License-Identifier: AGPL-3.0-or-later

//! In-process software key store backed by storage and encryption backends.

use beardog_errors::BearDogError;
use beardog_types::hsm::InMemoryStorageBackend;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::debug;

use super::encryption::{DefaultEncryptionKey, EncryptionKeyBackend};
use super::keys::SoftwareKey;
use super::storage::StorageBackend;

/// Software key store
pub struct SoftwareKeyStore {
    /// Storage backend
    pub storage_backend: Arc<StorageBackend>,
    /// Encryption key
    pub encryption_key: Arc<EncryptionKeyBackend>,
    /// Key cache (LRU cache)
    pub key_cache: Arc<RwLock<HashMap<String, SoftwareKey>>>,
}

impl SoftwareKeyStore {
    /// Create a new software key store
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new(
        _config: &crate::tunnel::hsm::software_hsm::KeyStoreConfig,
    ) -> Result<Self, BearDogError> {
        Ok(Self {
            storage_backend: Arc::new(StorageBackend::InMemory(InMemoryStorageBackend)),
            encryption_key: Arc::new(EncryptionKeyBackend::Default(
                DefaultEncryptionKey::default(),
            )),
            key_cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Get key from store
    ///
    /// # Errors
    /// Returns an error if key not found or retrieval fails
    pub async fn get_key(&self, key_id: &str) -> Result<SoftwareKey, BearDogError> {
        let cache = self.key_cache.read().await;
        cache
            .get(key_id)
            .cloned()
            .ok_or_else(|| BearDogError::not_found(format!("Key not found: {key_id}")))
    }

    /// Store key in cache
    ///
    /// # Errors
    /// Returns an error if storage fails
    pub async fn store_key(&self, key: SoftwareKey) -> Result<(), BearDogError> {
        let mut cache = self.key_cache.write().await;
        cache.insert(key.id.clone(), key);
        Ok(())
    }

    /// Initialize the key store
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn initialize(&self) -> Result<(), BearDogError> {
        debug!("Initializing key store");
        Ok(())
    }

    /// Delete a key from the store
    ///
    /// # Errors
    /// Returns an error if deletion fails or key not found
    pub async fn delete_key(&mut self, key_id: &str) -> Result<(), BearDogError> {
        let mut cache = self.key_cache.write().await;
        if cache.remove(key_id).is_some() {
            debug!("Deleted key: {}", key_id);
            Ok(())
        } else {
            Err(BearDogError::not_found(format!("Key not found: {key_id}")))
        }
    }
}
