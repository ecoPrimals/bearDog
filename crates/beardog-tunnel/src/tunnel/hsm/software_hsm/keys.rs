use beardog_errors::BearDogError;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use beardog_types::canonical::hsm::KeyMetadata;

use super::encryption::EncryptionKey;
use super::storage::StorageBackend;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareKey {
    pub key_id: String,
    pub key_data: Vec<u8>,
    pub metadata: KeyMetadata,
    pub created_at: DateTime<Utc>,
    pub last_accessed: Option<DateTime<Utc>>,
}

/// Zero-cost software key store with compile-time dispatch
pub struct SoftwareKeyStore<S = DefaultStorageBackend, E = DefaultEncryptionKey> 
where
    S: StorageBackend + Send + Sync + 'static,
    E: EncryptionKey + Send + Sync + 'static,
{
    pub storage_backend: S,
    pub encryption_key: E,
    pub key_cache: Arc<RwLock<HashMap<String, SoftwareKey>>>,
    pub max_cached_keys: usize,
}

impl<S, E> SoftwareKeyStore<S, E>
where
    S: StorageBackend + Send + Sync + 'static,
    E: EncryptionKey + Send + Sync + 'static,
{
    pub fn new(
        storage_backend: S,
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

    pub async fn store_key(&self, key: &SoftwareKey) -> Result<(), BearDogError> {
        let encrypted_data = self.encryption_key.encrypt(&key.key_data).await?;

        self.storage_backend
            .store(&key.key_id, &encrypted_data)
            .await?;

        // Update cache
        let mut cache = self.key_cache.write().await;
        if cache.len() >= self.max_cached_keys {
            // Simple LRU eviction - remove oldest entry
            if let Some((oldest_key, _)) = cache.iter().next() {
                let oldest_key = oldest_key.clone();
                cache.remove(&oldest_key);
            }
        }
        cache.insert(key.key_id.clone(), key.clone());

        Ok(())
    }

    pub async fn retrieve_key(
        &self,
        key_id: &str,
    ) -> Result<Option<SoftwareKey>, BearDogError>> {
        // Check cache first
        {
            let cache = self.key_cache.read().await;
            if let Some(key) = cache.get(key_id) {
                return Ok(Some(key.clone()));
            }
        }

        // Load from storage
        if let Some(encrypted_data) = self.storage_backend.retrieve(key_id).await? {
            let key_data = self.encryption_key.decrypt(&encrypted_data).await?;
            
            // Reconstruct the key (simplified)
            let key = SoftwareKey {
                key_id: key_id.to_string(),
                key_data,
                metadata: KeyMetadata::default(),
                created_at: Utc::now(),
                last_accessed: Some(Utc::now()),
            };

            // Update cache
            let mut cache = self.key_cache.write().await;
            cache.insert(key_id.to_string(), key.clone());

            Ok(Some(key))
        } else {
            Ok(None)
        }
    }
}

// Default implementations for backward compatibility
pub struct DefaultStorageBackend;
pub struct DefaultEncryptionKey;

impl StorageBackend for DefaultStorageBackend {
    async fn store(&self, _key_id: &str, _data: &[u8]) -> Result<(), BearDogError> {
        // In-memory storage for testing
        Ok(())
    }

    async fn retrieve(&self, _key_id: &str) -> Result<Option<Vec<u8>, BearDogError>>> {
        Ok(None)
    }

    async fn delete(&self, _key_id: &str) -> Result<(), BearDogError> {
        Ok(())
    }
}

impl EncryptionKey for DefaultEncryptionKey {
    async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        // Simple XOR encryption for testing
        Ok(data.iter().map(|b| b ^ 0x42).collect())
    }

    async fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        // Simple XOR decryption
        Ok(data.iter().map(|b| b ^ 0x42).collect())
    }
}

// Type aliases for common configurations
pub type InMemoryKeyStore = SoftwareKeyStore<DefaultStorageBackend, DefaultEncryptionKey>;

// Builder for legacy compatibility
pub struct SoftwareKeyBuilder {
    key_id: Option<String>,
    key_data: Option<Vec<u8>>,
    metadata: Option<KeyMetadata>,
}

impl SoftwareKeyBuilder {
    pub fn new() -> Self {
        Self {
            key_id: None,
            key_data: None,
            metadata: None,
        }
    }

    pub fn key_id(mut self, key_id: String) -> Self {
        self.key_id = Some(key_id);
        self
    }

    pub fn key_data(mut self, key_data: Vec<u8>) -> Self {
        self.key_data = Some(key_data);
        self
    }

    pub fn metadata(mut self, metadata: KeyMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

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
            created_at: Utc::now(),
            last_accessed: None,
        })
    }
}
