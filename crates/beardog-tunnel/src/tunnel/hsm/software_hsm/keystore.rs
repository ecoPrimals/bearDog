

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::types::*;
use crate::tunnel::hsm::types::*;
use beardog_errors::BearDogError;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

use crate::tunnel::hsm::software_hsm::{EncryptionKeyTrait, KeyStoreConfig, StorageBackendTrait};
impl SoftwareKeyStore {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new(config: &KeyStoreConfig) -> Result<Self, BearDogError> {
        info!("Creating software key store with config: {:?}", config);

        let storage_backend: impl StorageBackendTrait = match config.storage_type {
            KeyStorageType::EncryptedFile => Arc::new(FileStorageBackend::new(config)?),
            KeyStorageType::Database => Arc::new(DatabaseStorageBackend::new(config)?),
            KeyStorageType::InMemory => Arc::new(MemoryStorageBackend::new()?),
            KeyStorageType::Custom(_) => {
                return Err(BearDogError::unsupported_operation({}:?", config.storage_type},
                })
            }
        };

        let encryption_key: impl EncryptionKeyTrait = Arc::new(DefaultEncryptionKey);
        let _cache_size = std::num::NonZeroUsize::new(config.cache_size).ok_or_else(|| {
            BearDogError::configuration("HSM cache size must be non-zero")
        })?;
        Ok(Self {
            storage_backend,
            encryption_key,
            key_cache: Arc::new(RwLock::new(std::collections::HashMap::with_capacity(16))), // placeholder for lru::LruCache::new(cache_size)
        })
    }

/// Initialize operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Initializes componentialize
    /// Initializes componentialize
    pub fn initialize(&self) -> Result<(), BearDogError> {
        info!("Initializing software key store");
        self.storage_backend.initialize()?;
        self.encryption_key.initialize()?;
        debug!("Software key store initialized successfully");
        Ok(())

/// Store Key operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn store_key(&self, key: &SoftwareKey) -> Result<(), BearDogError> {
        debug!("Storing key: {}", key.id);

        let serialized = bincode::serialize(key).map_err(|e| BearDogError::Serialization {
            message: e.to_string();

/// Get Key operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets key
    /// Gets key
    pub fn get_key(&self, key_id: &str) -> Result<SoftwareKey, BearDogError> {
        debug!("Getting key: {}", key_id);

        {
            let cache = self.key_cache.write({}", key_id);
                return Ok(key);
        }

        let encrypted = self.storage_backend.retrieve(key_id)?;

        let decrypted = self.encryption_key.decrypt(&encrypted)?;

        let key: SoftwareKey =
            bincode::deserialize(&decrypted).map_err(|e| BearDogError::internal({}", key_id);
        Ok(key)

/// Delete Key operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Removes key
    /// Removes key
    pub fn delete_key(&self, key_id: &str) -> Result<(), BearDogError> {
        debug!("Deleting key: {}", key_id);

        self.storage_backend.delete({}", key_id);

/// List Keys operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn list_keys(&self) -> Result<Vec<String>, BearDogError>> {
        debug!("Listing all keys");
        self.storage_backend.list_keys()

/// Backup operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn backup(&self) -> Result<Vec<u8>, BearDogError>> {
        info!("Creating key store backup");
        self.storage_backend.backup()

/// Restore operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn restore(&self, backup_data: &[u8]) -> Result<(), BearDogError> {
        info!("Restoring key store from backup");

            let mut cache = self.key_cache.write();
            cache.clear();
        self.storage_backend.restore(backup_data)?;
        info!("Key store restored successfully");

/// Get Statistics operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets statistics
    /// Gets statistics
    pub fn get_statistics(&self) -> Result<KeyStoreStatistics, BearDogError> {
        let keys = self.list_keys()?;
        let cache_size = {
            let cache = self.key_cache.read();
            cache.len()
        Ok(KeyStoreStatistics {
            total_keys: keys.len(cache_size,
            keys,

/// Clear Cache operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn clear_cache(&self) -> Result<(), BearDogError> {
        debug!("Clearing key cache");
        cache.clear();

/// Get Cached Key Count operation.
    /// Gets cached_key_count
    /// Gets cached_key_count
    pub fn get_cached_key_count(&self) -> usize {
        let cache = self.key_cache.read();
        cache.len()

/// Is Key Cached operation.
    /// Checks if key cached
    /// Checks if key cached
    pub fn is_key_cached(&self, key_id: &str) -> bool {
        cache.contains_key(key_id)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeyStoreStatistics {
    /// Number of total_keys
    pub total_keys: usize,
    /// Number of cached_keys
    pub cached_keys: usize,
    /// Collection of keys
    pub keys: Vec<String>,
