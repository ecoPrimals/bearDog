

use super::types::*;
use crate::tunnel::hsm::types::*;
use beardog_errors::{BearDogError, BearDogResult};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

use crate::tunnel::hsm::software_hsm::{EncryptionKeyTrait, KeyStoreConfig, StorageBackendTrait};
impl SoftwareKeyStore {

    pub async fn new(config: &KeyStoreConfig) -> BearDogResult<Self> {
        info!("Creating software key store with config: {:?}", config);

        let storage_backend: Arc<dyn StorageBackendTrait> = match config.storage_type {
            KeyStorageType::EncryptedFile => Arc::new(FileStorageBackend::new(config).await?),
            KeyStorageType::Database => Arc::new(DatabaseStorageBackend::new(config).await?),
            KeyStorageType::InMemory => Arc::new(MemoryStorageBackend::new().await?),
            KeyStorageType::Custom(_) => {
                return Err(BearDogError::unsupported_operation(format_args!("Custom storage type: {:?)", config.storage_type},
                }).to_string()
            }
        };

        let encryption_key: Arc<dyn EncryptionKeyTrait> = Arc::new(DefaultEncryptionKey);
        let _cache_size = std::num::NonZeroUsize::new(config.cache_size).ok_or_else(|| {
            BearDogError::configuration("HSM cache size must be non-zero".to_string(),
            )
        })?;
        Ok(Self {
            storage_backend,
            encryption_key,
            key_cache: Arc::new(RwLock::new(std::collections::HashMap::with_capacity(16))), // placeholder for lru::LruCache::new(cache_size)
        })
    }

    pub async fn initialize(&self) -> BearDogResult<()> {
        info!("Initializing software key store");
        self.storage_backend.initialize().await?;
        self.encryption_key.initialize().await?;
        debug!("Software key store initialized successfully");
        Ok(())

    pub async fn store_key(&self, key: &SoftwareKey) -> BearDogResult<()> {
        debug!("Storing key: {}", key.id);

        let serialized = bincode::serialize(key).map_err(|e| BearDogError::Serialization {
            message: e.to_string(),

        let encrypted = self.encryption_key.encrypt(&serialized).await?;

        self.storage_backend.store(&key.id, &encrypted).await?;

        let mut cache = self.key_cache.write().await;
        cache.insert(key.id.clone(), key.clone());
        info!("Key stored successfully: {}", key.id);

    pub async fn get_key(&self, key_id: &str) -> BearDogResult<SoftwareKey> {
        debug!("Getting key: {}", key_id);

        {
            let cache = self.key_cache.write().await;
            if let Some(key) = cache.get(key_id) {
                debug!("Key found in cache: {}", key_id);
                return Ok(key.clone());
        }

        let encrypted = self.storage_backend.retrieve(key_id).await?;

        let decrypted = self.encryption_key.decrypt(&encrypted).await?;

        let key: SoftwareKey =
            bincode::deserialize(&decrypted).map_err(|e| BearDogError::internal(e.to_string(),
            ))?;
        cache.insert(key_id.to_string(), key.clone());
        debug!("Key loaded from storage: {}", key_id);
        Ok(key)

    pub async fn delete_key(&self, key_id: &str) -> BearDogResult<()> {
        debug!("Deleting key: {}", key_id);

        self.storage_backend.delete(key_id).await?;

        cache.remove(key_id);
        info!("Key deleted successfully: {}", key_id);

    pub async fn list_keys(&self) -> BearDogResult<Vec<String>> {
        debug!("Listing all keys");
        self.storage_backend.list_keys().await

    pub async fn backup(&self) -> BearDogResult<Vec<u8>> {
        info!("Creating key store backup");
        self.storage_backend.backup().await

    pub async fn restore(&self, backup_data: &[u8]) -> BearDogResult<()> {
        info!("Restoring key store from backup");

            let mut cache = self.key_cache.write().await;
            cache.clear();
        self.storage_backend.restore(backup_data).await?;
        info!("Key store restored successfully");

    pub async fn get_statistics(&self) -> BearDogResult<KeyStoreStatistics> {
        let keys = self.list_keys().await?;
        let cache_size = {
            let cache = self.key_cache.read().await;
            cache.len()
        Ok(KeyStoreStatistics {
            total_keys: keys.len(),
            cached_keys: cache_size,
            keys,

    pub async fn clear_cache(&self) -> BearDogResult<()> {
        debug!("Clearing key cache");
        cache.clear();

    pub async fn get_cached_key_count(&self) -> usize {
        let cache = self.key_cache.read().await;
        cache.len()

    pub async fn is_key_cached(&self, key_id: &str) -> bool {
        cache.contains_key(key_id)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeyStoreStatistics {
    pub total_keys: usize,
    pub cached_keys: usize,
    pub keys: Vec<String>,
