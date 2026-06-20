// SPDX-License-Identifier: AGPL-3.0-or-later

//! Storage backends and persistence operations for the software HSM.

use beardog_errors::BearDogError;
use beardog_types::hsm::InMemoryStorageBackend;
use beardog_types::hsm::KeyStoreConfig;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Storage backend variants for the Software HSM (configuration / serde)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SoftwareHsmStorageKind {
    /// In-memory storage (ephemeral)
    InMemory,
    /// File-based storage
    File {
        /// Path to the storage file
        path: String,
    },
    /// Database storage
    Database {
        /// Database connection string
        connection_string: String,
    },
}

/// File-based storage backend
pub struct FileStorageBackend {
    /// Storage path (directory containing `*.key` files)
    pub path: String,
}

impl FileStorageBackend {
    /// # Errors
    ///
    /// Returns an error if filesystem access fails.
    /// Create new file storage backend
    pub async fn new(config: &KeyStoreConfig) -> Result<Self, BearDogError> {
        let path = config.path.to_string_lossy().to_string();
        Ok(Self { path })
    }
}

/// In-memory storage backend
pub struct MemoryStorageBackend {
    /// In-memory key storage
    pub storage: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl MemoryStorageBackend {
    /// # Errors
    ///
    /// Returns an error if encryption fails.
    /// Create new memory storage backend
    pub async fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            storage: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        })
    }
}

/// Active storage backend implementation (replaces `Arc<dyn StorageBackendTrait>`).
pub enum StorageBackend {
    /// File-backed keys on disk
    File(FileStorageBackend),
    /// Ephemeral marker backend from `beardog-types`
    InMemory(InMemoryStorageBackend),
    /// In-process map backend
    Memory(MemoryStorageBackend),
}

impl StorageBackend {
    /// Initialize the storage backend
    ///
    /// # Errors
    ///
    /// Returns an error if the storage location cannot be created or accessed.
    pub fn initialize(&self) -> impl Future<Output = Result<(), BearDogError>> + Send + '_ {
        let slf = self;
        async move {
            match slf {
                Self::File(b) => {
                    tokio::fs::create_dir_all(&b.path)
                        .await
                        .map_err(|e| BearDogError::io_error(&e.to_string()))?;
                    Ok(())
                }
                Self::InMemory(_b) => Ok(()),
                Self::Memory(_b) => Ok(()),
            }
        }
    }

    /// Store encrypted key data
    ///
    /// # Errors
    ///
    /// Returns an error if the key cannot be written to storage.
    pub fn store(
        &self,
        key_id: &str,
        key_data: &[u8],
    ) -> impl Future<Output = Result<(), BearDogError>> + Send + '_ {
        let key_id = key_id.to_string();
        let key_data = key_data.to_vec();
        let slf = self;
        async move {
            match slf {
                Self::File(b) => {
                    let file_path = format!("{}/{}.key", b.path, key_id);
                    tokio::fs::write(&file_path, key_data)
                        .await
                        .map_err(|e| BearDogError::io_error(&e.to_string()))?;
                    Ok(())
                }
                Self::InMemory(_b) => Ok(()),
                Self::Memory(b) => {
                    let mut storage = b.storage.write().await;
                    storage.insert(key_id, key_data);
                    Ok(())
                }
            }
        }
    }

    /// Retrieve encrypted key data
    ///
    /// # Errors
    ///
    /// Returns an error if the key is missing or cannot be read.
    pub fn retrieve(
        &self,
        key_id: &str,
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send + '_ {
        let key_id = key_id.to_string();
        let slf = self;
        async move {
            match slf {
                Self::File(b) => {
                    let file_path = format!("{}/{}.key", b.path, key_id);
                    tokio::fs::read(&file_path)
                        .await
                        .map_err(|e| BearDogError::not_found(format!("Key not found: {e}")))
                }
                Self::InMemory(_b) => Err(BearDogError::not_found(
                    "Key not found in ephemeral storage".to_string(),
                )),
                Self::Memory(b) => {
                    let storage = b.storage.read().await;
                    storage
                        .get(&key_id)
                        .cloned()
                        .ok_or_else(|| BearDogError::not_found(format!("Key not found: {key_id}")))
                }
            }
        }
    }

    /// Delete a key
    ///
    /// # Errors
    ///
    /// Returns an error if the key file cannot be removed.
    pub fn delete(
        &self,
        key_id: &str,
    ) -> impl Future<Output = Result<(), BearDogError>> + Send + '_ {
        let key_id = key_id.to_string();
        let slf = self;
        async move {
            match slf {
                Self::File(b) => {
                    let file_path = format!("{}/{}.key", b.path, key_id);
                    tokio::fs::remove_file(&file_path)
                        .await
                        .map_err(|e| BearDogError::io_error(&e.to_string()))?;
                    Ok(())
                }
                Self::InMemory(_b) => Ok(()),
                Self::Memory(b) => {
                    let mut storage = b.storage.write().await;
                    storage.remove(&key_id);
                    Ok(())
                }
            }
        }
    }

    /// List all key IDs
    ///
    /// # Errors
    ///
    /// Returns an error if the storage directory cannot be read.
    pub fn list_keys(&self) -> impl Future<Output = Result<Vec<String>, BearDogError>> + Send + '_ {
        let slf = self;
        async move {
            match slf {
                Self::File(b) => {
                    let mut dir = tokio::fs::read_dir(&b.path)
                        .await
                        .map_err(|e| BearDogError::io_error(&e.to_string()))?;
                    let mut keys = Vec::new();
                    while let Some(entry) = dir
                        .next_entry()
                        .await
                        .map_err(|e| BearDogError::io_error(&e.to_string()))?
                    {
                        if let Some(name) = entry.file_name().to_str() {
                            let path = std::path::Path::new(name);
                            if path
                                .extension()
                                .is_some_and(|ext| ext.eq_ignore_ascii_case("key"))
                                && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
                            {
                                keys.push(stem.to_string());
                            }
                        }
                    }
                    Ok(keys)
                }
                Self::InMemory(_b) => Ok(vec![]),
                Self::Memory(b) => {
                    let storage = b.storage.read().await;
                    Ok(storage.keys().cloned().collect())
                }
            }
        }
    }

    /// Create a backup of all keys
    ///
    /// # Errors
    ///
    /// Returns an error if keys cannot be read or serialized.
    pub fn backup(&self) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send + '_ {
        let slf = self;
        async move {
            match slf {
                Self::File(b) => {
                    let path = b.path.clone();
                    let mut dir = tokio::fs::read_dir(&path)
                        .await
                        .map_err(|e| BearDogError::io_error(&e.to_string()))?;
                    let mut keys = Vec::new();
                    while let Some(entry) = dir
                        .next_entry()
                        .await
                        .map_err(|e| BearDogError::io_error(&e.to_string()))?
                    {
                        if let Some(name) = entry.file_name().to_str() {
                            let p = std::path::Path::new(name);
                            if p.extension()
                                .is_some_and(|ext| ext.eq_ignore_ascii_case("key"))
                                && let Some(stem) = p.file_stem().and_then(|s| s.to_str())
                            {
                                keys.push(stem.to_string());
                            }
                        }
                    }
                    let mut backup_data = HashMap::new();
                    for key_id in keys {
                        let file_path = format!("{path}/{key_id}.key");
                        let data = tokio::fs::read(&file_path)
                            .await
                            .map_err(|e| BearDogError::not_found(format!("Key not found: {e}")))?;
                        backup_data.insert(key_id, data);
                    }
                    postcard::to_allocvec(&backup_data)
                        .map_err(|e| BearDogError::internal(e.to_string()))
                }
                Self::InMemory(_b) => Ok(vec![]),
                Self::Memory(b) => {
                    let storage = b.storage.read().await;
                    postcard::to_allocvec(&*storage)
                        .map_err(|e| BearDogError::internal(e.to_string()))
                }
            }
        }
    }

    /// Restore from backup
    ///
    /// # Errors
    ///
    /// Returns an error if the backup payload is invalid or cannot be written.
    pub fn restore(
        &self,
        backup_data: &[u8],
    ) -> impl Future<Output = Result<(), BearDogError>> + Send + '_ {
        let backup_data = backup_data.to_vec();
        let slf = self;
        async move {
            match slf {
                Self::File(b) => {
                    let backup: HashMap<String, Vec<u8>> = postcard::from_bytes(&backup_data)
                        .map_err(|e| BearDogError::internal(e.to_string()))?;
                    for (key_id, data) in backup {
                        let file_path = format!("{}/{}.key", b.path, key_id);
                        tokio::fs::write(&file_path, data)
                            .await
                            .map_err(|e| BearDogError::io_error(&e.to_string()))?;
                    }
                    Ok(())
                }
                Self::InMemory(_b) => Ok(()),
                Self::Memory(b) => {
                    let restored: HashMap<String, Vec<u8>> = postcard::from_bytes(&backup_data)
                        .map_err(|e| BearDogError::internal(e.to_string()))?;
                    let mut storage = b.storage.write().await;
                    *storage = restored;
                    Ok(())
                }
            }
        }
    }
}
