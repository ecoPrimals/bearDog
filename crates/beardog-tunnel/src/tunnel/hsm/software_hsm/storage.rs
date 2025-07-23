//! # Software HSM Storage Backends
//!
//! This module provides different storage backend implementations for the Software HSM.
//! Each backend offers different persistence mechanisms and capabilities.

// use super::types::*;
// use crate::tunnel::hsm::types::*;
// use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use beardog_errors::{BearDogError, BearDogResult};
// use beardog_utils::crypto_utils::generate_secure_random_bytes;

/// In-memory storage for HSM keys
#[derive(Clone)]
pub struct MemoryHsmStorage {
    /// Storage for encrypted keys
    storage: Arc<RwLock<HashMap<String, StoredKey>>>,
    /// Master encryption key for storage
    master_key: Arc<Key<Aes256Gcm>>,
    /// Storage statistics
    stats: Arc<RwLock<StorageStats>>,
}

/// Stored key data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoredKey {
    /// Key identifier
    pub key_id: String,
    /// Encrypted key data
    pub encrypted_data: Vec<u8>,
    /// Encryption nonce
    pub nonce: Vec<u8>,
    /// Key metadata
    pub metadata: KeyMetadata,
    /// When the key was created
    pub created_at: DateTime<Utc>,
    /// When the key was last accessed
    pub last_accessed: DateTime<Utc>,
}

/// Key metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyMetadata {
    /// Key type (e.g., "aes256", "rsa2048")
    pub key_type: String,
    /// Key usage (e.g., "encryption", "signing")
    pub usage: String,
    /// Key size in bits
    pub key_size: u32,
    /// Whether the key is exportable
    pub exportable: bool,
    /// Custom attributes
    pub attributes: HashMap<String, String>,
}

/// Storage statistics
#[derive(Clone, Debug, Default)]
pub struct StorageStats {
    /// Total number of stored keys
    pub total_keys: usize,
    /// Total storage size in bytes
    pub total_size: usize,
    /// Number of operations performed
    pub operations_count: u64,
    /// Last operation timestamp
    pub last_operation: Option<DateTime<Utc>>,
}

/// Encryption key trait for HSM storage
pub trait EncryptionKey: Send + Sync {
    /// Initialize the encryption key
    fn initialize(&self) -> impl std::future::Future<Output = BearDogResult<()>> + Send;

    /// Encrypt data
    fn encrypt(
        &self,
        plaintext: &[u8],
    ) -> impl std::future::Future<Output = BearDogResult<Vec<u8>>> + Send;

    /// Decrypt data
    fn decrypt(
        &self,
        ciphertext: &[u8],
    ) -> impl std::future::Future<Output = BearDogResult<Vec<u8>>> + Send;
}

/// Default encryption key implementation using AES-256-GCM
pub struct DefaultEncryptionKey {
    cipher: Aes256Gcm,
}

impl MemoryHsmStorage {
    /// Create new memory HSM storage
    pub fn new() -> BearDogResult<Self> {
        // Generate a secure master key for storage encryption
        let master_key_bytes = rand::random::<[u8; 32]>();
        let master_key = Key::<Aes256Gcm>::from_slice(&master_key_bytes);

        info!("Initialized memory HSM storage with secure master key");

        Ok(Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
            master_key: Arc::new(*master_key),
            stats: Arc::new(RwLock::new(StorageStats::default())),
        })
    }

    /// Store a key in memory storage
    pub async fn store_key(
        &self,
        key_id: String,
        key_data: &[u8],
        metadata: KeyMetadata,
    ) -> BearDogResult<()> {
        debug!("Storing key in memory HSM: {}", key_id);

        // Encrypt the key data
        let cipher = Aes256Gcm::new(&self.master_key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let encrypted_data =
            cipher
                .encrypt(&nonce, key_data)
                .map_err(|e| BearDogError::Cryptographic {
                    operation: format!("Failed to encrypt key data: {e}"),
                })?;

        let encrypted_data_len = encrypted_data.len();

        let stored_key = StoredKey {
            key_id: key_id.clone(),
            encrypted_data,
            nonce: nonce.to_vec(),
            metadata,
            created_at: Utc::now(),
            last_accessed: Utc::now(),
        };

        // Store the encrypted key
        {
            let mut storage = self.storage.write().await;
            storage.insert(key_id.clone(), stored_key);
        }

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_keys += 1;
            stats.total_size += encrypted_data_len;
            stats.operations_count += 1;
            stats.last_operation = Some(Utc::now());
        }

        info!("Successfully stored key: {}", key_id);
        Ok(())
    }

    /// Retrieve a key from memory storage
    pub async fn retrieve_key(&self, key_id: &str) -> BearDogResult<Vec<u8>> {
        debug!("Retrieving key from memory HSM: {}", key_id);

        let stored_key = {
            let mut storage = self.storage.write().await;
            let stored_key = storage
                .get_mut(key_id)
                .ok_or_else(|| BearDogError::NotFound {
                    message: format!("Key not found: {key_id}"),
                })?;

            // Update last accessed time
            stored_key.last_accessed = Utc::now();
            stored_key.clone()
        };

        // Decrypt the key data
        let cipher = Aes256Gcm::new(&self.master_key);
        let nonce = Nonce::from_slice(&stored_key.nonce);

        let decrypted_data = cipher
            .decrypt(nonce, stored_key.encrypted_data.as_ref())
            .map_err(|e| BearDogError::Cryptographic {
                operation: format!("Failed to decrypt key data: {e}"),
            })?;

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.operations_count += 1;
            stats.last_operation = Some(Utc::now());
        }

        debug!("Successfully retrieved key: {}", key_id);
        Ok(decrypted_data)
    }

    /// Delete a key from memory storage
    pub async fn delete_key(&self, key_id: &str) -> BearDogResult<()> {
        debug!("Deleting key from memory HSM: {}", key_id);

        let removed_key = {
            let mut storage = self.storage.write().await;
            storage
                .remove(key_id)
                .ok_or_else(|| BearDogError::NotFound {
                    message: format!("Key not found: {key_id}"),
                })?
        };

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_keys -= 1;
            stats.total_size -= removed_key.encrypted_data.len();
            stats.operations_count += 1;
            stats.last_operation = Some(Utc::now());
        }

        info!("Successfully deleted key: {}", key_id);
        Ok(())
    }

    /// List all keys in storage
    pub async fn list_keys(&self) -> BearDogResult<Vec<String>> {
        let storage = self.storage.read().await;
        let keys: Vec<String> = storage.keys().cloned().collect();

        debug!("Listed {} keys from memory HSM", keys.len());
        Ok(keys)
    }

    /// Get key metadata
    pub async fn get_key_metadata(&self, key_id: &str) -> BearDogResult<KeyMetadata> {
        let storage = self.storage.read().await;
        let stored_key = storage.get(key_id).ok_or_else(|| BearDogError::NotFound {
            message: format!("Key not found: {key_id}"),
        })?;

        Ok(stored_key.metadata.clone())
    }

    /// Get storage statistics
    pub async fn get_statistics(&self) -> BearDogResult<StorageStats> {
        let stats = self.stats.read().await;
        Ok(stats.clone())
    }

    /// Clear all keys from storage
    pub async fn clear_storage(&self) -> BearDogResult<()> {
        warn!("Clearing all keys from memory HSM storage");

        {
            let mut storage = self.storage.write().await;
            storage.clear();
        }

        // Reset statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_keys = 0;
            stats.total_size = 0;
            stats.operations_count += 1;
            stats.last_operation = Some(Utc::now());
        }

        info!("Memory HSM storage cleared");
        Ok(())
    }
}

impl DefaultEncryptionKey {
    /// Create a new default encryption key
    pub fn new() -> BearDogResult<Self> {
        // Generate a secure key for the cipher
        let key_bytes = rand::random::<[u8; 32]>();
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);

        Ok(Self { cipher })
    }
}

impl EncryptionKey for DefaultEncryptionKey {
    /// Initialize encryption key
    async fn initialize(&self) -> BearDogResult<()> {
        info!("Initializing default encryption key");
        // Key is already initialized in constructor
        Ok(())
    }

    /// Encrypt data using AES-256-GCM
    async fn encrypt(&self, plaintext: &[u8]) -> BearDogResult<Vec<u8>> {
        debug!("Encrypting {} bytes with AES-256-GCM", plaintext.len());

        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext =
            self.cipher
                .encrypt(&nonce, plaintext)
                .map_err(|e| BearDogError::Cryptographic {
                    operation: format!("Failed to encrypt data: {e}"),
                })?;

        // Prepend nonce to ciphertext for decryption
        let mut encrypted_data = nonce.to_vec();
        encrypted_data.extend_from_slice(&ciphertext);

        debug!(
            "Successfully encrypted data, output size: {} bytes",
            encrypted_data.len()
        );
        Ok(encrypted_data)
    }

    /// Decrypt data using AES-256-GCM
    async fn decrypt(&self, ciphertext: &[u8]) -> BearDogResult<Vec<u8>> {
        debug!("Decrypting {} bytes with AES-256-GCM", ciphertext.len());

        if ciphertext.len() < 12 {
            return Err(BearDogError::Cryptographic {
                operation: "Ciphertext too short to contain nonce".to_string(),
            });
        }

        // Extract nonce and ciphertext
        let (nonce_bytes, encrypted_data) = ciphertext.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = self.cipher.decrypt(nonce, encrypted_data).map_err(|e| {
            BearDogError::Cryptographic {
                operation: format!("Failed to decrypt data: {e}"),
            }
        })?;

        debug!(
            "Successfully decrypted data, output size: {} bytes",
            plaintext.len()
        );
        Ok(plaintext)
    }
}

impl Default for MemoryHsmStorage {
    fn default() -> Self {
        // Create a safe fallback implementation without panicking
        // Use secure random generation for the master key
        let master_key_bytes = rand::random::<[u8; 32]>();
        let master_key = Arc::new(*Key::<Aes256Gcm>::from_slice(&master_key_bytes));
        
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
            master_key,
            stats: Arc::new(RwLock::new(StorageStats {
                total_keys: 0,
                total_size: 0,
                operations_count: 0,
                last_operation: None,
            })),
        }
    }
}

impl Default for DefaultEncryptionKey {
    fn default() -> Self {
        // Create a safe fallback implementation without panicking
        // Generate a secure key using the same pattern as new()
        let key_bytes = rand::random::<[u8; 32]>();
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        
        Self { cipher }
    }
}

/// Memory storage statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MemoryStorageStatistics {
    /// Number of keys stored in memory
    pub key_count: usize,
    /// Total memory usage in bytes
    pub total_size: usize,
    /// Average size per key in bytes
    pub average_key_size: usize,
}

/// Create storage backend factory
pub fn create_storage_backend() -> BearDogResult<MemoryHsmStorage> {
    MemoryHsmStorage::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_storage_operations() {
        let storage = MemoryHsmStorage::new().unwrap();

        let key_data = b"test_key_data_123";
        let metadata = KeyMetadata {
            key_type: "aes256".to_string(),
            usage: "encryption".to_string(),
            key_size: 256,
            exportable: true,
            attributes: HashMap::new(),
        };

        // Store key
        storage
            .store_key("test_key".to_string(), key_data, metadata)
            .await
            .unwrap();

        // Retrieve key
        let retrieved_data = storage.retrieve_key("test_key").await.unwrap();
        assert_eq!(retrieved_data, key_data);

        // List keys
        let keys = storage.list_keys().await.unwrap();
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0], "test_key");

        // Delete key
        storage.delete_key("test_key").await.unwrap();

        // Verify key is deleted
        let result = storage.retrieve_key("test_key").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_encryption_key() {
        let key = DefaultEncryptionKey::new().unwrap();
        key.initialize().await.unwrap();

        let plaintext = b"Hello, World!";
        let ciphertext = key.encrypt(plaintext).await.unwrap();
        let decrypted = key.decrypt(&ciphertext).await.unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[tokio::test]
    async fn test_storage_statistics() {
        let storage = MemoryHsmStorage::new().unwrap();

        let stats = storage.get_statistics().await.unwrap();
        assert_eq!(stats.total_keys, 0);
        assert_eq!(stats.total_size, 0);

        let key_data = b"test_key_data";
        let metadata = KeyMetadata {
            key_type: "aes256".to_string(),
            usage: "encryption".to_string(),
            key_size: 256,
            exportable: true,
            attributes: HashMap::new(),
        };

        storage
            .store_key("test_key".to_string(), key_data, metadata)
            .await
            .unwrap();

        let stats = storage.get_statistics().await.unwrap();
        assert_eq!(stats.total_keys, 1);
        assert!(stats.total_size > 0);
        assert!(stats.operations_count > 0);
    }
}
