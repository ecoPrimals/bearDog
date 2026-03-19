// SPDX-License-Identifier: AGPL-3.0-only

//! HSM Key Persistence and Public Key Management (Phase 2)
//!
//! Provides secure key storage, retrieval, and lifecycle management for cryptographic keys.
//! Supports both ephemeral keys and persistent storage (HSM-backed or file-based).

use beardog_errors::BearDogError;
use ed25519_dalek::{SigningKey, VerifyingKey, PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::RwLock;
use zeroize::Zeroizing;

// ============================================================================
// Key Types and Metadata
// ============================================================================

/// Cryptographic key type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyType {
    /// Ed25519 signing key
    Ed25519,
    /// AES-256-GCM encryption key
    Aes256Gcm,
    /// X25519 key exchange key
    X25519,
    /// RSA-2048 key
    Rsa2048,
    /// RSA-4096 key
    Rsa4096,
}

/// Key usage flags
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyUsage {
    /// Key can be used for signing
    Sign,
    /// Key can be used for verification
    Verify,
    /// Key can be used for encryption
    Encrypt,
    /// Key can be used for decryption
    Decrypt,
    /// Key can be used for key agreement
    KeyAgreement,
}

/// Key storage location
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyStorage {
    /// Ephemeral key (in-memory only)
    Ephemeral,
    /// File-based storage
    File(PathBuf),
    /// Hardware Security Module
    Hsm(String), // HSM slot/token identifier
}

/// Key metadata for cryptographic key management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadata {
    /// Unique identifier for this cryptographic key
    pub key_id: String,
    /// Type of cryptographic key (symmetric, asymmetric, etc.)
    pub key_type: KeyType,
    /// Permitted usages for this key (encrypt, sign, etc.)
    pub usage: Vec<KeyUsage>,
    /// Storage location/method for this key (HSM, software, etc.)
    pub storage: KeyStorage,
    /// Timestamp when this key was created (UTC)
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Optional expiration timestamp for key rotation (UTC)
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Optional human-readable description of key purpose
    pub description: Option<String>,
}

// ============================================================================
// Key Store
// ============================================================================

/// In-memory key store for private keys
struct KeyStoreInner {
    /// Private keys (Ed25519 signing keys)
    signing_keys: HashMap<String, SigningKey>,
    /// Public keys (Ed25519 verifying keys)
    verifying_keys: HashMap<String, VerifyingKey>,
    /// AES-256-GCM symmetric keys
    symmetric_keys: HashMap<String, Zeroizing<Vec<u8>>>,
    /// Key metadata
    metadata: HashMap<String, KeyMetadata>,
}

/// Thread-safe key store
pub struct KeyStore {
    inner: Arc<RwLock<KeyStoreInner>>,
    storage_dir: Option<PathBuf>,
}

impl KeyStore {
    /// Create a new key store
    ///
    /// # Arguments
    /// * `storage_dir` - Optional directory for persistent key storage
    #[must_use]
    pub fn new(storage_dir: Option<PathBuf>) -> Self {
        Self {
            inner: Arc::new(RwLock::new(KeyStoreInner {
                signing_keys: HashMap::new(),
                verifying_keys: HashMap::new(),
                symmetric_keys: HashMap::new(),
                metadata: HashMap::new(),
            })),
            storage_dir,
        }
    }

    /// Generate a new Ed25519 signing key
    ///
    /// # Arguments
    /// * `key_id` - Unique identifier for the key
    /// * `usage` - Key usage flags
    /// * `storage` - Storage location
    /// * `description` - Optional description
    ///
    /// # Returns
    /// Public key bytes
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn generate_signing_key(
        &self,
        key_id: String,
        usage: Vec<KeyUsage>,
        storage: KeyStorage,
        description: Option<String>,
    ) -> Result<Vec<u8>, BearDogError> {
        use rand::rngs::OsRng;

        let mut rng = OsRng;
        let signing_key = SigningKey::from_bytes(&rand::Rng::gen(&mut rng));
        let verifying_key = signing_key.verifying_key();
        let public_key_bytes = verifying_key.to_bytes().to_vec();

        let metadata = KeyMetadata {
            key_id: key_id.clone(),
            key_type: KeyType::Ed25519,
            usage,
            storage: storage.clone(),
            created_at: chrono::Utc::now(),
            expires_at: None,
            description,
        };

        // Store in memory
        let mut inner = self.inner.write().await;
        inner.signing_keys.insert(key_id.clone(), signing_key);
        inner.verifying_keys.insert(key_id.clone(), verifying_key);
        inner.metadata.insert(key_id.clone(), metadata.clone());
        drop(inner);

        // Persist to storage if configured
        if let KeyStorage::File(_) = &storage {
            self.persist_signing_key(&key_id).await?;
        }

        Ok(public_key_bytes)
    }

    /// Generate a symmetric encryption key (AES-256-GCM)
    ///
    /// # Arguments
    /// * `key_id` - Unique identifier for the key
    /// * `usage` - Key usage flags
    /// * `storage` - Storage location
    /// * `description` - Optional description
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn generate_symmetric_key(
        &self,
        key_id: String,
        usage: Vec<KeyUsage>,
        storage: KeyStorage,
        description: Option<String>,
    ) -> Result<(), BearDogError> {
        use rand::RngCore;

        let mut key_bytes = Zeroizing::new(vec![0u8; 32]); // 256 bits
        rand::rngs::OsRng.fill_bytes(&mut key_bytes);

        let metadata = KeyMetadata {
            key_id: key_id.clone(),
            key_type: KeyType::Aes256Gcm,
            usage,
            storage: storage.clone(),
            created_at: chrono::Utc::now(),
            expires_at: None,
            description,
        };

        // Store in memory
        let mut inner = self.inner.write().await;
        inner.symmetric_keys.insert(key_id.clone(), key_bytes);
        inner.metadata.insert(key_id.clone(), metadata.clone());
        drop(inner);

        // Persist to storage if configured
        if let KeyStorage::File(_) = &storage {
            self.persist_symmetric_key(&key_id).await?;
        }

        Ok(())
    }

    /// Import an existing Ed25519 signing key
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn import_signing_key(
        &self,
        key_id: String,
        private_key_bytes: &[u8],
        usage: Vec<KeyUsage>,
        storage: KeyStorage,
        description: Option<String>,
    ) -> Result<(), BearDogError> {
        if private_key_bytes.len() != SECRET_KEY_LENGTH {
            return Err(BearDogError::security(format!(
                "Invalid Ed25519 private key length: expected {}, got {}",
                SECRET_KEY_LENGTH,
                private_key_bytes.len()
            )));
        }

        let signing_key = SigningKey::from_bytes(
            private_key_bytes
                .try_into()
                .map_err(|_| BearDogError::security("Failed to parse Ed25519 key".to_string()))?,
        );
        let verifying_key = signing_key.verifying_key();

        let metadata = KeyMetadata {
            key_id: key_id.clone(),
            key_type: KeyType::Ed25519,
            usage,
            storage: storage.clone(),
            created_at: chrono::Utc::now(),
            expires_at: None,
            description,
        };

        let mut inner = self.inner.write().await;
        inner.signing_keys.insert(key_id.clone(), signing_key);
        inner.verifying_keys.insert(key_id.clone(), verifying_key);
        inner.metadata.insert(key_id.clone(), metadata);
        drop(inner);

        if let KeyStorage::File(_) = &storage {
            self.persist_signing_key(&key_id).await?;
        }

        Ok(())
    }

    /// Import a public key for verification
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn import_public_key(
        &self,
        key_id: String,
        public_key_bytes: &[u8],
        key_type: KeyType,
        description: Option<String>,
    ) -> Result<(), BearDogError> {
        match key_type {
            KeyType::Ed25519 => {
                if public_key_bytes.len() != PUBLIC_KEY_LENGTH {
                    return Err(BearDogError::security(format!(
                        "Invalid Ed25519 public key length: expected {}, got {}",
                        PUBLIC_KEY_LENGTH,
                        public_key_bytes.len()
                    )));
                }

                let verifying_key =
                    VerifyingKey::from_bytes(public_key_bytes.try_into().map_err(|_| {
                        BearDogError::security("Failed to parse Ed25519 public key".to_string())
                    })?)
                    .map_err(|e| {
                        BearDogError::security(format!("Invalid Ed25519 public key: {e}"))
                    })?;

                let metadata = KeyMetadata {
                    key_id: key_id.clone(),
                    key_type: KeyType::Ed25519,
                    usage: vec![KeyUsage::Verify],
                    storage: KeyStorage::Ephemeral,
                    created_at: chrono::Utc::now(),
                    expires_at: None,
                    description,
                };

                let mut inner = self.inner.write().await;
                inner.verifying_keys.insert(key_id.clone(), verifying_key);
                inner.metadata.insert(key_id.clone(), metadata);
                Ok(())
            }
            _ => Err(BearDogError::security(format!(
                "Unsupported key type for public key import: {key_type:?}"
            ))),
        }
    }

    /// Get a signing key by ID
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn get_signing_key(&self, key_id: &str) -> Result<SigningKey, BearDogError> {
        let inner = self.inner.read().await;
        inner
            .signing_keys
            .get(key_id)
            .cloned()
            .ok_or_else(|| BearDogError::security(format!("Signing key not found: {key_id}")))
    }

    /// Get a verifying key by ID
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn get_verifying_key(&self, key_id: &str) -> Result<VerifyingKey, BearDogError> {
        let inner = self.inner.read().await;
        inner
            .verifying_keys
            .get(key_id)
            .copied()
            .ok_or_else(|| BearDogError::security(format!("Verifying key not found: {key_id}")))
    }

    /// Get a symmetric key by ID
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn get_symmetric_key(
        &self,
        key_id: &str,
    ) -> Result<Zeroizing<Vec<u8>>, BearDogError> {
        let inner = self.inner.read().await;
        inner
            .symmetric_keys
            .get(key_id)
            .cloned()
            .ok_or_else(|| BearDogError::security(format!("Symmetric key not found: {key_id}")))
    }

    /// Get key metadata
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn get_metadata(&self, key_id: &str) -> Result<KeyMetadata, BearDogError> {
        let inner = self.inner.read().await;
        inner
            .metadata
            .get(key_id)
            .cloned()
            .ok_or_else(|| BearDogError::security(format!("Key metadata not found: {key_id}")))
    }

    /// List all key IDs
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn list_keys(&self) -> Result<Vec<String>, BearDogError> {
        let inner = self.inner.read().await;
        Ok(inner.metadata.keys().cloned().collect())
    }

    /// Delete a key
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn delete_key(&self, key_id: &str) -> Result<(), BearDogError> {
        let mut inner = self.inner.write().await;
        let metadata = inner.metadata.remove(key_id);
        inner.signing_keys.remove(key_id);
        inner.verifying_keys.remove(key_id);
        inner.symmetric_keys.remove(key_id);
        drop(inner);

        // Delete from persistent storage if applicable
        if let Some(KeyMetadata {
            storage: KeyStorage::File(_),
            ..
        }) = metadata
        {
            self.delete_persisted_key(key_id).await?;
        }

        Ok(())
    }

    /// Persist signing key to file
    async fn persist_signing_key(&self, key_id: &str) -> Result<(), BearDogError> {
        let storage_dir = self.storage_dir.as_ref().ok_or_else(|| {
            BearDogError::security("Key storage directory not configured".to_string())
        })?;

        let inner = self.inner.read().await;
        let signing_key = inner
            .signing_keys
            .get(key_id)
            .ok_or_else(|| BearDogError::security(format!("Signing key not found: {key_id}")))?;
        let metadata = inner
            .metadata
            .get(key_id)
            .ok_or_else(|| BearDogError::security(format!("Key metadata not found: {key_id}")))?
            .clone();

        let key_bytes = signing_key.to_bytes();
        drop(inner);

        // Create storage directory if it doesn't exist
        fs::create_dir_all(storage_dir).await.map_err(|e| {
            BearDogError::security(format!("Failed to create key storage directory: {e}"))
        })?;

        // Write key file
        let key_path = storage_dir.join(format!("{key_id}.key"));
        fs::write(&key_path, &key_bytes)
            .await
            .map_err(|e| BearDogError::security(format!("Failed to write signing key: {e}")))?;

        // Write metadata file
        let metadata_path = storage_dir.join(format!("{key_id}.meta.json"));
        let metadata_json = serde_json::to_string_pretty(&metadata).map_err(|e| {
            BearDogError::security(format!("Failed to serialize key metadata: {e}"))
        })?;
        fs::write(&metadata_path, metadata_json)
            .await
            .map_err(|e| BearDogError::security(format!("Failed to write key metadata: {e}")))?;

        Ok(())
    }

    /// Persist symmetric key to file
    async fn persist_symmetric_key(&self, key_id: &str) -> Result<(), BearDogError> {
        let storage_dir = self.storage_dir.as_ref().ok_or_else(|| {
            BearDogError::security("Key storage directory not configured".to_string())
        })?;

        let inner = self.inner.read().await;
        let symmetric_key = inner
            .symmetric_keys
            .get(key_id)
            .ok_or_else(|| BearDogError::security(format!("Symmetric key not found: {key_id}")))?;
        let metadata = inner
            .metadata
            .get(key_id)
            .ok_or_else(|| BearDogError::security(format!("Key metadata not found: {key_id}")))?
            .clone();

        let key_bytes = symmetric_key.clone();
        drop(inner);

        // Create storage directory if it doesn't exist
        fs::create_dir_all(storage_dir).await.map_err(|e| {
            BearDogError::security(format!("Failed to create key storage directory: {e}"))
        })?;

        // Write key file
        let key_path = storage_dir.join(format!("{key_id}.key"));
        fs::write(&key_path, &*key_bytes)
            .await
            .map_err(|e| BearDogError::security(format!("Failed to write symmetric key: {e}")))?;

        // Write metadata file
        let metadata_path = storage_dir.join(format!("{key_id}.meta.json"));
        let metadata_json = serde_json::to_string_pretty(&metadata).map_err(|e| {
            BearDogError::security(format!("Failed to serialize key metadata: {e}"))
        })?;
        fs::write(&metadata_path, metadata_json)
            .await
            .map_err(|e| BearDogError::security(format!("Failed to write key metadata: {e}")))?;

        Ok(())
    }

    /// Delete persisted key from file
    async fn delete_persisted_key(&self, key_id: &str) -> Result<(), BearDogError> {
        if let Some(storage_dir) = &self.storage_dir {
            let key_path = storage_dir.join(format!("{key_id}.key"));
            let metadata_path = storage_dir.join(format!("{key_id}.meta.json"));

            if key_path.exists() {
                fs::remove_file(&key_path).await.map_err(|e| {
                    BearDogError::security(format!("Failed to delete key file: {e}"))
                })?;
            }

            if metadata_path.exists() {
                fs::remove_file(&metadata_path).await.map_err(|e| {
                    BearDogError::security(format!("Failed to delete metadata file: {e}"))
                })?;
            }
        }

        Ok(())
    }

    /// Load keys from persistent storage
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn load_from_storage(&self) -> Result<usize, BearDogError> {
        let storage_dir = self.storage_dir.as_ref().ok_or_else(|| {
            BearDogError::security("Key storage directory not configured".to_string())
        })?;

        if !storage_dir.exists() {
            return Ok(0);
        }

        let mut entries = fs::read_dir(storage_dir).await.map_err(|e| {
            BearDogError::security(format!("Failed to read key storage directory: {e}"))
        })?;

        let mut loaded_count = 0;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| BearDogError::security(format!("Failed to read directory entry: {e}")))?
        {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("key") {
                let key_id = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .ok_or_else(|| BearDogError::security("Invalid key filename".to_string()))?;

                let metadata_path = storage_dir.join(format!("{key_id}.meta.json"));
                if !metadata_path.exists() {
                    continue; // Skip if metadata is missing
                }

                // Load metadata
                let metadata_json = fs::read_to_string(&metadata_path).await.map_err(|e| {
                    BearDogError::security(format!("Failed to read key metadata: {e}"))
                })?;
                let metadata: KeyMetadata = serde_json::from_str(&metadata_json).map_err(|e| {
                    BearDogError::security(format!("Failed to parse key metadata: {e}"))
                })?;

                // Load key based on type
                match metadata.key_type {
                    KeyType::Ed25519 => {
                        let key_bytes = fs::read(&path).await.map_err(|e| {
                            BearDogError::security(format!("Failed to read signing key: {e}"))
                        })?;
                        self.import_signing_key(
                            key_id.to_string(),
                            &key_bytes,
                            metadata.usage.clone(),
                            metadata.storage.clone(),
                            metadata.description.clone(),
                        )
                        .await?;
                        loaded_count += 1;
                    }
                    KeyType::Aes256Gcm => {
                        let key_bytes = Zeroizing::new(fs::read(&path).await.map_err(|e| {
                            BearDogError::security(format!("Failed to read symmetric key: {e}"))
                        })?);

                        let mut inner = self.inner.write().await;
                        inner.symmetric_keys.insert(key_id.to_string(), key_bytes);
                        inner.metadata.insert(key_id.to_string(), metadata);
                        drop(inner);

                        loaded_count += 1;
                    }
                    _ => {} // Skip unsupported key types
                }
            }
        }

        Ok(loaded_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_generate_signing_key() {
        let key_store = KeyStore::new(None);

        let public_key = key_store
            .generate_signing_key(
                "test-key-1".to_string(),
                vec![KeyUsage::Sign, KeyUsage::Verify],
                KeyStorage::Ephemeral,
                Some("Test signing key".to_string()),
            )
            .await
            .expect("Key generation should succeed");

        assert_eq!(public_key.len(), PUBLIC_KEY_LENGTH);

        let metadata = key_store
            .get_metadata("test-key-1")
            .await
            .expect("Metadata should exist");
        assert_eq!(metadata.key_type, KeyType::Ed25519);
        assert_eq!(metadata.description, Some("Test signing key".to_string()));
    }

    #[tokio::test]
    async fn test_generate_symmetric_key() {
        let key_store = KeyStore::new(None);

        key_store
            .generate_symmetric_key(
                "test-sym-1".to_string(),
                vec![KeyUsage::Encrypt, KeyUsage::Decrypt],
                KeyStorage::Ephemeral,
                Some("Test symmetric key".to_string()),
            )
            .await
            .expect("Key generation should succeed");

        let key = key_store
            .get_symmetric_key("test-sym-1")
            .await
            .expect("Key should exist");
        assert_eq!(key.len(), 32);
    }

    #[tokio::test]
    async fn test_key_persistence() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let key_store = KeyStore::new(Some(temp_dir.path().to_path_buf()));

        // Generate and persist a key
        key_store
            .generate_signing_key(
                "persistent-key".to_string(),
                vec![KeyUsage::Sign],
                KeyStorage::File(temp_dir.path().to_path_buf()),
                None,
            )
            .await
            .expect("Key generation should succeed");

        // Create new key store and load from storage
        let key_store2 = KeyStore::new(Some(temp_dir.path().to_path_buf()));
        let loaded_count = key_store2
            .load_from_storage()
            .await
            .expect("Load should succeed");

        assert_eq!(loaded_count, 1);

        let metadata = key_store2
            .get_metadata("persistent-key")
            .await
            .expect("Metadata should exist after load");
        assert_eq!(metadata.key_type, KeyType::Ed25519);
    }

    #[tokio::test]
    async fn test_delete_key() {
        let key_store = KeyStore::new(None);

        key_store
            .generate_signing_key(
                "delete-me".to_string(),
                vec![KeyUsage::Sign],
                KeyStorage::Ephemeral,
                None,
            )
            .await
            .expect("Key generation should succeed");

        key_store
            .delete_key("delete-me")
            .await
            .expect("Delete should succeed");

        let result = key_store.get_metadata("delete-me").await;
        assert!(result.is_err(), "Key should not exist after deletion");
    }

    #[tokio::test]
    async fn test_list_keys() {
        let key_store = KeyStore::new(None);

        key_store
            .generate_signing_key(
                "key1".to_string(),
                vec![KeyUsage::Sign],
                KeyStorage::Ephemeral,
                None,
            )
            .await
            .expect("Key generation should succeed");

        key_store
            .generate_signing_key(
                "key2".to_string(),
                vec![KeyUsage::Sign],
                KeyStorage::Ephemeral,
                None,
            )
            .await
            .expect("Key generation should succeed");

        let keys = key_store.list_keys().await.expect("List should succeed");
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&"key1".to_string()));
        assert!(keys.contains(&"key2".to_string()));
    }
}
