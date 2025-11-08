use super::types::*;
use beardog_errors::BearDogError;
use beardog_types::hsm::KeyStoreConfig;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Software key store for managing cryptographic keys
pub struct SoftwareKeyStore {
    keys: Arc<RwLock<HashMap<String, SoftwareKey>>>,
    cache_size: usize,
}

impl SoftwareKeyStore {
    /// Create new software key store
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub async fn new(config: &KeyStoreConfig) -> Result<Self, BearDogError> {
        info!("Creating software key store with config: {:?}", config);

        // Validate storage type
        match config.storage_type {
            beardog_types::hsm::config::KeyStorageType::Memory
            | beardog_types::hsm::config::KeyStorageType::FileSystem
            | beardog_types::hsm::config::KeyStorageType::Database
            | beardog_types::hsm::config::KeyStorageType::Hardware
            | beardog_types::hsm::config::KeyStorageType::CloudKms => {}
        }

        let cache_size = if config.cache_size > 0 {
            config.cache_size
        } else {
            return Err(BearDogError::configuration(
                "HSM cache size must be non-zero",
            ));
        };

        Ok(Self {
            keys: Arc::new(RwLock::new(HashMap::with_capacity(cache_size))),
            cache_size,
        })
    }

    /// Initialize the key store
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub async fn initialize(&self) -> Result<(), BearDogError> {
        info!("Initializing software key store");
        debug!("Software key store initialized successfully");
        Ok(())
    }

    /// Store a key
    ///
    /// # Errors
    /// Returns an error if storage fails
    pub async fn store_key(&self, key: &SoftwareKey) -> Result<(), BearDogError> {
        debug!("Storing key: {}", key.id);

        let mut keys = self.keys.write().await;

        if keys.len() >= self.cache_size && !keys.contains_key(&key.id) {
            return Err(BearDogError::System {
                message: "Key cache is full".to_string(),
                category: beardog_errors::SystemErrorCategory::Resource,
            });
        }

        keys.insert(key.id.clone(), key.clone());
        debug!("Key stored successfully: {}", key.id);
        Ok(())
    }

    /// Retrieve a key
    ///
    /// # Errors
    /// Returns an error if key not found or retrieval fails
    pub async fn get_key(&self, key_id: &str) -> Result<SoftwareKey, BearDogError> {
        debug!("Retrieving key: {}", key_id);

        let keys = self.keys.read().await;

        keys.get(key_id)
            .cloned()
            .ok_or_else(|| BearDogError::not_found(format!("Key not found: {}", key_id)))
    }

    /// Delete a key
    ///
    /// # Errors
    /// Returns an error if deletion fails
    pub async fn delete_key(&self, key_id: &str) -> Result<(), BearDogError> {
        debug!("Deleting key: {}", key_id);

        let mut keys = self.keys.write().await;

        if keys.remove(key_id).is_some() {
            debug!("Key deleted successfully: {}", key_id);
            Ok(())
        } else {
            Err(BearDogError::not_found(format!(
                "Key not found: {}",
                key_id
            )))
        }
    }

    /// List all key IDs
    ///
    /// # Errors
    /// Returns an error if listing fails
    pub async fn list_keys(&self) -> Result<Vec<String>, BearDogError> {
        debug!("Listing all keys");

        let keys = self.keys.read().await;
        let key_ids: Vec<String> = keys.keys().cloned().collect();

        debug!("Found {} keys", key_ids.len());
        Ok(key_ids)
    }

    /// Check if a key exists
    pub async fn key_exists(&self, key_id: &str) -> bool {
        let keys = self.keys.read().await;
        keys.contains_key(key_id)
    }

    /// Get the number of stored keys
    pub async fn key_count(&self) -> usize {
        let keys = self.keys.read().await;
        keys.len()
    }

    /// Clear all keys (use with caution)
    ///
    /// # Errors
    /// Returns an error if clear operation fails
    pub async fn clear_all_keys(&self) -> Result<(), BearDogError> {
        info!("Clearing all keys from key store");

        let mut keys = self.keys.write().await;
        keys.clear();

        debug!("All keys cleared");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tunnel::hsm::types::{key::KeyMetadata, KeyType};

    fn create_test_config() -> KeyStoreConfig {
        // Use the canonical KeyStoreConfig's memory() constructor for tests
        KeyStoreConfig::memory()
    }

    fn create_test_key(id: &str) -> SoftwareKey {
        use crate::tunnel::hsm::software_hsm::types::ProtectedMemory;
        let key_type = KeyType::Aes;
        SoftwareKey {
            id: id.to_string(),
            key_type: key_type.clone(),
            key_material: ProtectedMemory::new(vec![0u8; 32], true),
            metadata: KeyMetadata::new(id.to_string(), key_type),
            created_at: chrono::Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_keystore_creation() -> Result<(), BearDogError> {
        let config = create_test_config();
        let keystore = SoftwareKeyStore::new(&config).await?;
        assert!(keystore.initialize().await.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_store_and_retrieve_key() -> Result<(), BearDogError> {
        let config = create_test_config();
        let keystore = SoftwareKeyStore::new(&config).await?;
        keystore.initialize().await?;

        let key = create_test_key("test-key-1");
        keystore.store_key(&key).await?;

        let retrieved = keystore.get_key("test-key-1").await?;
        assert_eq!(key.id, retrieved.id);
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_key() -> Result<(), BearDogError> {
        let config = create_test_config();
        let keystore = SoftwareKeyStore::new(&config).await?;
        keystore.initialize().await?;

        let key = create_test_key("test-key-2");
        keystore.store_key(&key).await?;
        assert!(keystore.key_exists("test-key-2").await);

        keystore.delete_key("test-key-2").await?;
        assert!(!keystore.key_exists("test-key-2").await);
        Ok(())
    }

    #[tokio::test]
    async fn test_list_keys() -> Result<(), BearDogError> {
        let config = create_test_config();
        let keystore = SoftwareKeyStore::new(&config).await?;
        keystore.initialize().await?;

        keystore.store_key(&create_test_key("key-1")).await?;
        keystore.store_key(&create_test_key("key-2")).await?;
        keystore.store_key(&create_test_key("key-3")).await?;

        let keys = keystore.list_keys().await?;
        assert_eq!(keys.len(), 3);
        Ok(())
    }

    #[tokio::test]
    async fn test_key_count() -> Result<(), BearDogError> {
        let config = create_test_config();
        let keystore = SoftwareKeyStore::new(&config).await?;
        keystore.initialize().await?;

        assert_eq!(keystore.key_count().await, 0);

        keystore.store_key(&create_test_key("key-1")).await?;
        assert_eq!(keystore.key_count().await, 1);

        keystore.store_key(&create_test_key("key-2")).await?;
        assert_eq!(keystore.key_count().await, 2);
        Ok(())
    }
}

/// Key store statistics for compatibility
#[derive(Debug, Clone)]
pub struct KeyStoreStatistics {
    /// Total number of keys
    pub total_keys: usize,
    /// Number of active keys
    pub active_keys: usize,
    /// Cache hit rate percentage
    pub cache_hit_rate: f64,
    /// Total operations performed
    pub total_operations: u64,
}

impl Default for KeyStoreStatistics {
    fn default() -> Self {
        Self {
            total_keys: 0,
            active_keys: 0,
            cache_hit_rate: 0.0,
            total_operations: 0,
        }
    }
}
