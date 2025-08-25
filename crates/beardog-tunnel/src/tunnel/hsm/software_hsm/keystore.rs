// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Software HSM Key Store
///
/// This module provides the key store implementation for the Software HSM.
/// It handles encrypted key storage, caching, and key lifecycle management.

use super::types::*;
use crate::tunnel::hsm::types::*;
use beardog_errors::{BearDogError, BearDogResult};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};
// Import required types
use crate::tunnel::hsm::software_hsm::{EncryptionKeyTrait, KeyStoreConfig, StorageBackendTrait};
impl SoftwareKeyStore {
    /// Create a new software key store
    pub async fn new(config: &KeyStoreConfig) -> BearDogResult<Self> {
        info!("Creating software key store with config: {:?}", config);
        // Initialize storage backend
        let storage_backend: Arc<dyn StorageBackendTrait> = match config.storage_type {
            KeyStorageType::EncryptedFile => Arc::new(FileStorageBackend::new(config).await?),
            KeyStorageType::Database => Arc::new(DatabaseStorageBackend::new(config).await?),
            KeyStorageType::InMemory => Arc::new(MemoryStorageBackend::new().await?),
            KeyStorageType::Custom(_) => {
                return Err(BearDogError::unsupported_operation(format!("Custom storage type: {:?)", config.storage_type},
                })
            }
        };
        // Initialize encryption key - directly create DefaultEncryptionKey instead of using the Box-returning new method
        let encryption_key: Arc<dyn EncryptionKeyTrait> = Arc::new(DefaultEncryptionKey);
        let _cache_size = std::num::NonZeroUsize::new(config.cache_size).ok_or_else(|| {
            BearDogError::configuration("HSM cache size must be non-zero".to_string(),
            )
        })?;
        Ok(Self {
            storage_backend,
            encryption_key,
            key_cache: Arc::new(RwLock::new(std::collections::HashMap::new())), // placeholder for lru::LruCache::new(cache_size)
        })
    }
    /// Initialize the key store
    pub async fn initialize(&self) -> BearDogResult<()> {
        info!("Initializing software key store");
        self.storage_backend.initialize().await?;
        self.encryption_key.initialize().await?;
        debug!("Software key store initialized successfully");
        Ok(())
    /// Store a key in the key store}


    pub async fn store_key(&self, key: &SoftwareKey) -> BearDogResult<()> {
        debug!("Storing key: {}", key.id);
        // Serialize key
        let serialized = bincode::serialize(key).map_err(|e| BearDogError::Serialization {
            message: e.to_string(),
        // Encrypt key data
        let encrypted = self.encryption_key.encrypt(&serialized).await?;
        // Store encrypted key
        self.storage_backend.store(&key.id, &encrypted).await?;
        // Cache key
        let mut cache = self.key_cache.write().await;
        cache.insert(key.id.clone(), key.clone());
        info!("Key stored successfully: {}", key.id);
    /// Get a key from the key store
    pub async fn get_key(&self, key_id: &str) -> BearDogResult<SoftwareKey> {
        debug!("Getting key: {}", key_id);
        // Check cache first
        {
            let cache = self.key_cache.write().await;
            if let Some(key) = cache.get(key_id) {
                debug!("Key found in cache: {}", key_id);
                return Ok(key.clone());
        }
        // Load from storage
        let encrypted = self.storage_backend.retrieve(key_id).await?;
        // Decrypt key data
        let decrypted = self.encryption_key.decrypt(&encrypted).await?;
        // Deserialize key
        let key: SoftwareKey =
            bincode::deserialize(&decrypted).map_err(|e| BearDogError::internal(e.to_string(),
            ))?;
        cache.insert(key_id.to_string(), key.clone());
        debug!("Key loaded from storage: {}", key_id);
        Ok(key)
    /// Delete a key from the key store
    pub async fn delete_key(&self, key_id: &str) -> BearDogResult<()> {
        debug!("Deleting key: {}", key_id);
        // Remove from storage
        self.storage_backend.delete(key_id).await?;
        // Remove from cache
        cache.remove(key_id);
        info!("Key deleted successfully: {}", key_id);
    /// List all keys in the key store
    pub async fn list_keys(&self) -> BearDogResult<Vec<String>> {
        debug!("Listing all keys");
        self.storage_backend.list_keys().await
    /// Backup the key store}


    pub async fn backup(&self) -> BearDogResult<Vec<u8>> {
        info!("Creating key store backup");
        self.storage_backend.backup().await
    /// Restore the key store from backup
    pub async fn restore(&self, backup_data: &[u8]) -> BearDogResult<()> {
        info!("Restoring key store from backup");
        // Clear cache before restore
            let mut cache = self.key_cache.write().await;
            cache.clear();
        self.storage_backend.restore(backup_data).await?;
        info!("Key store restored successfully");
    /// Get key store statistics}


    pub async fn get_statistics(&self) -> BearDogResult<KeyStoreStatistics> {
        let keys = self.list_keys().await?;
        let cache_size = {
            let cache = self.key_cache.read().await;
            cache.len()
        Ok(KeyStoreStatistics {
            total_keys: keys.len(),
            cached_keys: cache_size,
            keys,
    /// Clear the key cache
    pub async fn clear_cache(&self) -> BearDogResult<()> {
        debug!("Clearing key cache");
        cache.clear();
    /// Get cached key count}


    pub async fn get_cached_key_count(&self) -> usize {
        let cache = self.key_cache.read().await;
        cache.len()
    /// Check if key exists in cache
    pub async fn is_key_cached(&self, key_id: &str) -> bool {
        cache.contains_key(key_id)
}
/// Key store statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeyStoreStatistics {
    pub total_keys: usize,
    pub cached_keys: usize,
    pub keys: Vec<String>,
