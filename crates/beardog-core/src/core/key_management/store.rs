// SPDX-License-Identifier: AGPL-3.0-or-later

//! In-memory [`KeyStore`] and thread-safe key lifecycle API.

use super::persistence;
use super::types::{KeyMetadata, KeyStorage, KeyType, KeyUsage};
use beardog_errors::BearDogError;
use ed25519_dalek::{PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH, SigningKey, VerifyingKey};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use zeroize::Zeroizing;

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

    pub(crate) const fn storage_dir(&self) -> Option<&PathBuf> {
        self.storage_dir.as_ref()
    }

    /// Used when loading symmetric keys from disk (no import API for symmetric material).
    pub(crate) async fn insert_symmetric_key_loaded(
        &self,
        key_id: String,
        key_bytes: Zeroizing<Vec<u8>>,
        metadata: KeyMetadata,
    ) {
        let mut inner = self.inner.write().await;
        inner.symmetric_keys.insert(key_id.clone(), key_bytes);
        inner.metadata.insert(key_id, metadata); // key_id consumed here
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
        use rand::RngCore;
        let mut secret = [0u8; SECRET_KEY_LENGTH];
        rand::rng().fill_bytes(&mut secret);
        let signing_key = SigningKey::from_bytes(&secret);
        let verifying_key = signing_key.verifying_key();
        let public_key_bytes = verifying_key.to_bytes().to_vec();

        let needs_persist = matches!(&storage, KeyStorage::File(_));
        let metadata = KeyMetadata {
            key_id: key_id.clone(),
            key_type: KeyType::Ed25519,
            usage,
            storage,
            created_at: chrono::Utc::now(),
            expires_at: None,
            description,
        };

        let mut inner = self.inner.write().await;
        inner.signing_keys.insert(key_id.clone(), signing_key);
        inner.verifying_keys.insert(key_id.clone(), verifying_key);
        inner.metadata.insert(key_id.clone(), metadata);
        drop(inner);

        if needs_persist {
            self.persist_signing_key_inner(&key_id).await?;
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
        rand::rng().fill_bytes(&mut key_bytes);

        let metadata = KeyMetadata {
            key_id: key_id.clone(),
            key_type: KeyType::Aes256Gcm,
            usage,
            storage,
            created_at: chrono::Utc::now(),
            expires_at: None,
            description,
        };

        let needs_persist = matches!(&metadata.storage, KeyStorage::File(_));
        let mut inner = self.inner.write().await;
        inner.symmetric_keys.insert(key_id.clone(), key_bytes);
        inner.metadata.insert(key_id.clone(), metadata);
        drop(inner);

        if needs_persist {
            self.persist_symmetric_key_inner(&key_id).await?;
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

        let needs_persist = matches!(&storage, KeyStorage::File(_));
        let metadata = KeyMetadata {
            key_id: key_id.clone(),
            key_type: KeyType::Ed25519,
            usage,
            storage,
            created_at: chrono::Utc::now(),
            expires_at: None,
            description,
        };

        let mut inner = self.inner.write().await;
        inner.signing_keys.insert(key_id.clone(), signing_key);
        inner.verifying_keys.insert(key_id.clone(), verifying_key);
        inner.metadata.insert(key_id.clone(), metadata);
        drop(inner);

        if needs_persist {
            self.persist_signing_key_inner(&key_id).await?;
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
                inner.metadata.insert(key_id, metadata);
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

        if let Some(KeyMetadata {
            storage: KeyStorage::File(_),
            ..
        }) = metadata
        {
            persistence::delete_persisted_key(self.storage_dir.as_deref(), key_id).await?;
        }

        Ok(())
    }

    async fn persist_signing_key_inner(&self, key_id: &str) -> Result<(), BearDogError> {
        let storage_dir = self.storage_dir.as_ref().ok_or_else(|| {
            BearDogError::security("Key storage directory not configured".to_string())
        })?;

        let (key_bytes, metadata_json) = {
            let inner = self.inner.read().await;
            let signing_key = inner.signing_keys.get(key_id).ok_or_else(|| {
                BearDogError::security(format!("Signing key not found: {key_id}"))
            })?;
            let metadata = inner.metadata.get(key_id).ok_or_else(|| {
                BearDogError::security(format!("Key metadata not found: {key_id}"))
            })?;

            let key_bytes = signing_key.to_bytes();
            let metadata_json = serde_json::to_string_pretty(metadata).map_err(|e| {
                BearDogError::security(format!("Failed to serialize key metadata: {e}"))
            })?;
            (key_bytes, metadata_json)
        };

        persistence::persist_signing_key(storage_dir, key_id, key_bytes.as_slice(), &metadata_json)
            .await
    }

    async fn persist_symmetric_key_inner(&self, key_id: &str) -> Result<(), BearDogError> {
        let storage_dir = self.storage_dir.as_ref().ok_or_else(|| {
            BearDogError::security("Key storage directory not configured".to_string())
        })?;

        let (key_bytes, metadata_json) = {
            let inner = self.inner.read().await;
            let symmetric_key = inner.symmetric_keys.get(key_id).ok_or_else(|| {
                BearDogError::security(format!("Symmetric key not found: {key_id}"))
            })?;
            let metadata = inner.metadata.get(key_id).ok_or_else(|| {
                BearDogError::security(format!("Key metadata not found: {key_id}"))
            })?;

            let key_bytes: Vec<u8> = (**symmetric_key).clone();
            let metadata_json = serde_json::to_string_pretty(metadata).map_err(|e| {
                BearDogError::security(format!("Failed to serialize key metadata: {e}"))
            })?;
            (key_bytes, metadata_json)
        };

        persistence::persist_symmetric_key(storage_dir, key_id, &key_bytes, &metadata_json).await
    }

    /// Load keys from persistent storage
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails.
    pub async fn load_from_storage(&self) -> Result<usize, BearDogError> {
        persistence::load_from_storage(self).await
    }
}
