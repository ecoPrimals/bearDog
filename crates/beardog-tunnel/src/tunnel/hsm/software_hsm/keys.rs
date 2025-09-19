// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use beardog_types::canonical::hsm::KeyMetadata;

use super::encryption::EncryptionKey;
use super::storage::StorageBackend;

#[derive(Debug, Clone)]
    /// Collection of key data
    pub key_data: Vec<u8>,
    /// The metadata value
    pub metadata: KeyMetadata,
    /// The created at value
    pub created_at: DateTime<Utc>,
    /// Optional last accessed
    pub last_accessed: Option<DateTime<Utc>>,
}

pub struct SoftwareKeyStore<S = DefaultStorageBackend, E = DefaultEncryptionKey> 
where
    S: StorageBackend + Send + Sync + 'static,
    E: EncryptionKey + Send + Sync + 'static,
{
    /// The storage backend value
    pub storage_backend: S,
    /// The encryption key value
    pub encryption_key: E,
    /// The key cache value
    pub key_cache: Arc<RwLock<HashMap<String, SoftwareKey>>>,
    /// Number of max_cached_keys
    pub max_cached_keys: usize,
}

impl<S, E> SoftwareKeyStore<S, E>
where
    S: StorageBackend + Send + Sync + 'static,
    E: EncryptionKey + Send + Sync + 'static,
{
/// New operation.
    /// Creates a new instance
    pub fn new(S,
        encryption_key: E,
        max_cached_keys: usize,
    ) -> Self {
        Self {
            storage_backend,
            encryption_key,
            key_cache: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            max_cached_keys,
        }
    }

/// Store Key operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn store_key(&self, key: &SoftwareKey) -> Result<(), BearDogError> {
        let encrypted_data = self.encryption_key.encrypt(&str,
    ) -> Result<Option<SoftwareKey>, BearDogError>> {

        {
            let cache = self.key_cache.read();
            if let Some(key) = cache.get(key_id) {
                return Ok(Some(key));
            }
        }

        if let Some(encrypted_data) = self.storage_backend.retrieve(key_id)? {
            let key_data = self.encryption_key.decrypt(&encrypted_data)?;

            let key = SoftwareKey {
                key_id: key_id.to_string(),
                key_data,
                metadata: KeyMetadata::default(),
                created_at: Utc::now(),
                last_accessed: Some(Utc::now(&str, _data: &[u8]) -> Result<(), BearDogError> {

        Ok(())
    }


    fn retrieve(&self, _key_id: &str) -> Result<Option<Vec<u8>, BearDogError>>> {
        Ok(None)
    }

    /// Removes 
    fn delete(&self, _key_id: &str) -> Result<(), BearDogError> {
        Ok(())
    }
}

impl EncryptionKey for DefaultEncryptionKey {
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {

        Ok(data.iter().map(|b| b ^ 0x42).collect())
    }


    fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {

        Ok(Option<String>,
    key_data: Option<Vec<u8>>,
    metadata: Option<KeyMetadata>,
}

impl SoftwareKeyBuilder {
/// New operation.
    /// Creates a new instance
    pub fn new(None,
            key_data: None,
            metadata: None,
        }
    }

/// Key Id operation.
    pub fn key_id(mut self, key_id: &str) -> Self {
        self.key_id = Some(key_id);
        self
    }

/// Key Data operation.
    pub fn key_data(mut self, key_data: Vec<u8>) -> Self {
        self.key_data = Some(key_data);
        self
    }

/// Metadata operation.
    pub fn metadata(mut self, metadata: KeyMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

/// Build operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Builds component
    /// Builds component
    pub fn build(self) -> Result<SoftwareKey, BearDogError> {
        let key_id = self.key_id.ok_or_else(|| {
            beardog_errors::BearDogError::Validation {
                field: "key_id".to_string(),
                message: "Key ID is required".to_string(),
            }
        })?;

        let key_data = self.key_data.unwrap_or_default();
        let metadata = self.metadata.unwrap_or_default();

        Ok(SoftwareKey {
            key_id,
            key_data,
            metadata,
            created_at: Utc::now(None,
        })
    }
}
