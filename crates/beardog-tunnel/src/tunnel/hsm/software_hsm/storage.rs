// SPDX-License-Identifier: AGPL-3.0-or-later

use aes_gcm::{
    Aes256Gcm, Key,
    aead::generic_array::GenericArray,
    aead::{Aead, KeyInit},
};
use beardog_config::env_keys;
use hkdf::Hkdf;
use sha2::Sha256;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

// Type alias for nonce size (96 bits / 12 bytes for AES-GCM)
type NonceSize = aes_gcm::aead::consts::U12;
use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// In-memory HSM storage backend
#[derive(Clone)]
pub struct MemoryHsmStorage {
    storage: Arc<RwLock<HashMap<String, StoredKey>>>,
    root_key: Arc<Key<Aes256Gcm>>,
    stats: Arc<RwLock<StorageStats>>,
}

/// Stored key with encryption
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoredKey {
    /// Unique key identifier
    pub key_id: String,
    /// AES-256-GCM encrypted key material
    pub encrypted_data: Vec<u8>,
    /// Nonce used for encryption
    pub nonce: Vec<u8>,
    /// Key metadata (type, algorithm, size)
    pub metadata: KeyMetadata,
    /// When the key was stored
    pub created_at: DateTime<Utc>,
    /// When the key was last accessed
    pub last_accessed: DateTime<Utc>,
}

/// Key metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyMetadata {
    /// Key type (e.g., "symmetric", "asymmetric")
    pub key_type: String,
    /// Algorithm name (e.g., "AES-256-GCM")
    pub algorithm: String,
    /// Key size in bytes
    pub size_bytes: usize,
}

/// Storage statistics
#[derive(Clone, Debug, Default)]
pub struct StorageStats {
    /// Total number of keys in storage
    pub total_keys: usize,
    /// Total encrypted data size in bytes
    pub total_size: usize,
    /// Number of operations performed
    pub operations_count: u64,
    /// Timestamp of the last operation
    pub last_operation: Option<DateTime<Utc>>,
}

impl MemoryHsmStorage {
    /// Create new in-memory HSM storage
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new() -> Result<Self, BearDogError> {
        info!("Creating in-memory HSM storage");

        // Generate a root encryption key
        let root_key_bytes: [u8; 32] = rand::random();
        let root_key = Key::<Aes256Gcm>::from_slice(&root_key_bytes);

        Ok(Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
            root_key: Arc::new(*root_key),
            stats: Arc::new(RwLock::new(StorageStats::default())),
        })
    }

    /// Initialize storage
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn initialize(&self) -> Result<(), BearDogError> {
        info!("Initializing memory HSM storage");
        Ok(())
    }

    /// Store a key
    ///
    /// # Errors
    /// Returns an error if storage fails
    pub async fn store_key(
        &self,
        key_id: String,
        key_data: &[u8],
        metadata: KeyMetadata,
    ) -> Result<(), BearDogError> {
        debug!("Storing key: {}", key_id);

        // Encrypt the key data
        let cipher = Aes256Gcm::new(&self.root_key);
        let nonce_bytes = rand::random::<[u8; 12]>();
        let nonce = GenericArray::<u8, NonceSize>::from_slice(&nonce_bytes);

        let encrypted_data =
            cipher
                .encrypt(nonce, key_data)
                .map_err(|e| BearDogError::Cryptographic {
                    message: format!("Encryption failed: {e}"),
                })?;

        let stored_key = StoredKey {
            key_id: key_id.clone(),
            encrypted_data,
            nonce: nonce.to_vec(),
            metadata,
            created_at: Utc::now(),
            last_accessed: Utc::now(),
        };

        let mut storage = self.storage.write().await;
        storage.insert(key_id.clone(), stored_key);

        // Update stats
        let mut stats = self.stats.write().await;
        stats.total_keys = storage.len();
        stats.total_size += key_data.len();
        stats.operations_count += 1;
        stats.last_operation = Some(Utc::now());

        debug!("Key stored successfully: {}", key_id);
        Ok(())
    }

    /// Retrieve a key
    ///
    /// # Errors
    /// Returns an error if key not found or decryption fails
    pub async fn get_key(&self, key_id: &str) -> Result<Vec<u8>, BearDogError> {
        debug!("Retrieving key: {}", key_id);

        let mut storage = self.storage.write().await;

        let stored_key = storage
            .get_mut(key_id)
            .ok_or_else(|| BearDogError::not_found(format!("Key not found: {key_id}")))?;

        // Update last accessed
        stored_key.last_accessed = Utc::now();

        // Decrypt the key data
        let cipher = Aes256Gcm::new(&self.root_key);
        let nonce = GenericArray::<u8, NonceSize>::from_slice(&stored_key.nonce);

        let decrypted = cipher
            .decrypt(nonce, stored_key.encrypted_data.as_ref())
            .map_err(|e| BearDogError::Cryptographic {
                message: format!("Decryption failed: {e}"),
            })?;

        // Update stats
        let mut stats = self.stats.write().await;
        stats.operations_count += 1;
        stats.last_operation = Some(Utc::now());

        debug!("Key retrieved successfully: {}", key_id);
        Ok(decrypted)
    }

    /// Delete a key
    ///
    /// # Errors
    /// Returns an error if deletion fails
    pub async fn delete_key(&self, key_id: &str) -> Result<(), BearDogError> {
        debug!("Deleting key: {}", key_id);

        let mut storage = self.storage.write().await;

        let removed = storage
            .remove(key_id)
            .ok_or_else(|| BearDogError::not_found(format!("Key not found: {key_id}")))?;

        // Update stats
        let mut stats = self.stats.write().await;
        stats.total_keys = storage.len();
        stats.total_size = stats
            .total_size
            .saturating_sub(removed.encrypted_data.len());
        stats.operations_count += 1;
        stats.last_operation = Some(Utc::now());

        debug!("Key deleted successfully: {}", key_id);
        Ok(())
    }

    /// List all key IDs
    ///
    /// # Errors
    /// Returns an error if listing fails
    pub async fn list_keys(&self) -> Result<Vec<String>, BearDogError> {
        let storage = self.storage.read().await;
        Ok(storage.keys().cloned().collect())
    }

    /// Check if a key exists
    pub async fn key_exists(&self, key_id: &str) -> bool {
        let storage = self.storage.read().await;
        storage.contains_key(key_id)
    }

    /// Get storage statistics
    pub async fn get_stats(&self) -> StorageStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// Clear all keys
    ///
    /// # Errors
    /// Returns an error if clear fails
    pub async fn clear_all(&self) -> Result<(), BearDogError> {
        info!("Clearing all keys from storage");

        let mut storage = self.storage.write().await;
        storage.clear();

        let mut stats = self.stats.write().await;
        stats.total_keys = 0;
        stats.total_size = 0;
        stats.operations_count += 1;
        stats.last_operation = Some(Utc::now());

        Ok(())
    }
}

/// Sentinel value for uninitialized keys. Using a zero key for encryption is a CRITICAL security
/// vulnerability. This marker forces explicit key setup via `from_env()`, `from_master_secret()`,
/// or `new()` before any encrypt/decrypt operations.
const UNINITIALIZED_KEY_MARKER: [u8; 32] = [0; 32];

/// Default encryption key implementation
pub struct DefaultEncryptionKey {
    cipher: Aes256Gcm,
    /// True if key was properly derived; false for Default (uninitialized marker)
    initialized: bool,
}

impl Default for DefaultEncryptionKey {
    /// Returns an uninitialized marker. Encrypt/decrypt will panic if called.
    /// Use `from_env()`, `from_master_secret()`, or `new()` for production.
    fn default() -> Self {
        let key = Key::<Aes256Gcm>::from_slice(&UNINITIALIZED_KEY_MARKER);
        Self {
            cipher: Aes256Gcm::new(key),
            initialized: false,
        }
    }
}

impl DefaultEncryptionKey {
    /// # Errors
    ///
    /// Returns an error if decryption fails.
    /// Derive key from `BEARDOG_HSM_MASTER_KEY` env var or generate random (dev/test).
    /// Production deployments MUST set `BEARDOG_HSM_MASTER_KEY` for deterministic key derivation.
    pub fn from_env() -> Result<Self, BearDogError> {
        if let Ok(master) = beardog_errors::process_env::var(env_keys::ENV_HSM_MASTER_KEY) {
            Self::from_master_secret(master.as_bytes())
        } else {
            warn!("BEARDOG_HSM_MASTER_KEY not set; using random key (dev/test only)");
            Self::new()
        }
    }

    /// # Errors
    ///
    /// Returns an error if decryption fails.
    /// Derive 256-bit key from master secret using HKDF-SHA256.
    pub fn from_master_secret(master_secret: &[u8]) -> Result<Self, BearDogError> {
        let hk = Hkdf::<Sha256>::new(None, master_secret);
        let mut key_bytes = [0u8; 32];
        hk.expand(b"beardog-hsm-storage-v1", &mut key_bytes)
            .map_err(|e| BearDogError::Cryptographic {
                message: format!("HKDF key derivation failed: {e}"),
            })?;
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        Ok(Self {
            cipher: Aes256Gcm::new(key),
            initialized: true,
        })
    }

    /// # Errors
    ///
    /// Returns an error if decryption fails.
    /// Create new encryption key with random material (dev/test). Prefer `from_env()` for production.
    pub fn new() -> Result<Self, BearDogError> {
        let key_bytes: [u8; 32] = rand::random();
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        Ok(Self {
            cipher: Aes256Gcm::new(key),
            initialized: true,
        })
    }

    fn ensure_initialized(&self) {
        assert!(
            self.initialized,
            "DefaultEncryptionKey used before initialization. \
             Call from_env(), from_master_secret(), or new() before encrypt/decrypt. \
             Set BEARDOG_HSM_MASTER_KEY for production."
        );
    }

    /// # Errors
    ///
    /// Returns an error if decryption fails.
    /// Initialize the encryption key
    pub const fn initialize(&self) -> Result<(), BearDogError> {
        Ok(())
    }

    /// # Errors
    ///
    /// Returns an error if decryption fails.
    /// Encrypt data
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        self.ensure_initialized();
        let nonce_bytes = rand::random::<[u8; 12]>();
        let nonce = GenericArray::<u8, NonceSize>::from_slice(&nonce_bytes);

        let mut ciphertext =
            self.cipher
                .encrypt(nonce, plaintext)
                .map_err(|e| BearDogError::Cryptographic {
                    message: format!("Encryption failed: {e}"),
                })?;

        // Prepend nonce to ciphertext
        let mut result = nonce.to_vec();
        result.append(&mut ciphertext);

        Ok(result)
    }

    /// # Errors
    ///
    /// Returns an error if key deletion fails in the underlying HSM provider.
    /// Decrypt data
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        self.ensure_initialized();
        if ciphertext.len() < 12 {
            return Err(BearDogError::Cryptographic {
                message: "Ciphertext too short".to_string(),
            });
        }

        let nonce = GenericArray::<u8, NonceSize>::from_slice(&ciphertext[..12]);
        let encrypted_data = &ciphertext[12..];

        self.cipher
            .decrypt(nonce, encrypted_data)
            .map_err(|e| BearDogError::Cryptographic {
                message: format!("Decryption failed: {e}"),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_storage_creation() -> Result<(), BearDogError> {
        let storage = MemoryHsmStorage::new()?;
        storage.initialize()?;
        Ok(())
    }

    #[tokio::test]
    async fn test_store_and_retrieve() -> Result<(), BearDogError> {
        let storage = MemoryHsmStorage::new()?;
        storage.initialize()?;

        let key_data = b"test-key-data";
        let metadata = KeyMetadata {
            key_type: "AES".to_string(),
            algorithm: "AES-256-GCM".to_string(),
            size_bytes: key_data.len(),
        };

        storage
            .store_key("test-key-1".to_string(), key_data, metadata)
            .await?;
        let retrieved = storage.get_key("test-key-1").await?;

        assert_eq!(key_data, &retrieved[..]);
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_key() -> Result<(), BearDogError> {
        let storage = MemoryHsmStorage::new()?;
        let metadata = KeyMetadata {
            key_type: "AES".to_string(),
            algorithm: "AES-256-GCM".to_string(),
            size_bytes: 32,
        };

        storage
            .store_key("test-key-2".to_string(), b"data", metadata)
            .await?;
        assert!(storage.key_exists("test-key-2").await);

        storage.delete_key("test-key-2").await?;
        assert!(!storage.key_exists("test-key-2").await);
        Ok(())
    }

    #[tokio::test]
    async fn test_encryption_key() -> Result<(), BearDogError> {
        let enc_key = DefaultEncryptionKey::new()?;
        enc_key.initialize()?;

        let plaintext = b"sensitive data";
        let ciphertext = enc_key.encrypt(plaintext)?;
        let decrypted = enc_key.decrypt(&ciphertext)?;

        assert_eq!(plaintext, &decrypted[..]);
        Ok(())
    }
}
