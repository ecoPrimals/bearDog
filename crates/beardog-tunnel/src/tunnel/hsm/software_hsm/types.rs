// SPDX-License-Identifier: AGPL-3.0-or-later

//! Software HSM Types
//!
//! Core type definitions for the Software HSM implementation.

// ✅ MIGRATED: Using real crypto providers from software_hsm/crypto_providers and canonical trait
pub use beardog_types::hsm::CryptoProvider; // Canonical trait
pub use beardog_types::hsm::InMemoryStorageBackend;

// Use proper KeyStoreConfig from beardog-types
use crate::tunnel::hsm::software_hsm::KeyMetadata;
use crate::tunnel::hsm::types::{
    HsmHealthStatus, KeyStorageType, KeyType, MemoryProtectionLevel, PerformanceMetrics,
};
use beardog_errors::BearDogError;
use beardog_types::hsm::KeyStoreConfig;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::debug;

pub use super::audit::types::{AuditLogEntry, AuditLogFilter, OperationResult};

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
                    std::fs::create_dir_all(&b.path)
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
                    std::fs::write(&file_path, key_data)
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
                    std::fs::read(&file_path)
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
                    std::fs::remove_file(&file_path)
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
                    let entries = std::fs::read_dir(&b.path)
                        .map_err(|e| BearDogError::io_error(&e.to_string()))?;
                    let mut keys = Vec::new();
                    for entry in entries.flatten() {
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
                    let entries = std::fs::read_dir(&path)
                        .map_err(|e| BearDogError::io_error(&e.to_string()))?;
                    let mut keys = Vec::new();
                    for entry in entries.flatten() {
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
                        let data = std::fs::read(&file_path)
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
                        std::fs::write(&file_path, data)
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

/// Memory protection configuration
#[derive(Debug, Clone)]
pub struct MemoryProtector {
    /// Use secure memory allocation
    pub secure_allocation: bool,
    /// Clear memory on deallocation
    pub clear_on_dealloc: bool,
    /// Lock memory to prevent swapping
    pub lock_memory: bool,
}

/// Encryption key configuration
#[derive(Debug, Clone)]
pub struct EncryptionKey {
    /// Key derivation method
    pub derivation_method: String,
    /// Key size in bits
    pub key_size: u32,
    /// Use hardware entropy
    pub hardware_entropy: bool,
}

/// Software HSM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareHsmConfig {
    /// Storage backend configuration
    pub storage: SoftwareHsmStorageKind,
    /// Memory protection level
    pub memory_protection: MemoryProtectionLevel,
    /// Key storage type
    pub key_storage: KeyStorageType,
    /// Enable audit logging
    pub audit_logging: bool,
    /// Maximum number of keys (None = unlimited)
    pub max_keys: Option<usize>,
}

/// Software key representation
#[derive(Debug, Clone)]
pub struct SoftwareKey {
    /// Unique key identifier
    pub id: String,
    /// Key type
    pub key_type: KeyType,
    /// Protected key material
    pub key_material: ProtectedMemory,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Key metadata
    pub metadata: KeyMetadata,
}

impl SoftwareKey {
    /// Create a new software key
    pub fn new(
        id: String,
        key_type: KeyType,
        key_material: ProtectedMemory,
        metadata: KeyMetadata,
    ) -> Self {
        Self {
            id,
            key_type,
            key_material,
            created_at: Utc::now(),
            metadata,
        }
    }

    /// Get key ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get key type
    pub const fn key_type(&self) -> &KeyType {
        &self.key_type
    }

    /// Get key material
    pub const fn key_material(&self) -> &ProtectedMemory {
        &self.key_material
    }

    /// Get metadata
    pub const fn metadata(&self) -> &KeyMetadata {
        &self.metadata
    }

    /// Get creation timestamp
    pub const fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

/// Protected memory wrapper
#[derive(Debug, Clone)]
pub struct ProtectedMemory {
    /// Encrypted data (shared via [`bytes::Bytes`] for cheap clones on hot paths)
    pub data: bytes::Bytes,
    /// Whether memory is protected
    pub protected: bool,
}

impl ProtectedMemory {
    /// Create new protected memory
    pub fn new(data: Vec<u8>, protected: bool) -> Self {
        Self {
            data: bytes::Bytes::from(data),
            protected,
        }
    }

    /// Wrap existing shared bytes (e.g. after [`bytes::Bytes::clone`] from the same buffer).
    pub fn from_bytes(data: bytes::Bytes, protected: bool) -> Self {
        Self { data, protected }
    }

    /// Get data reference
    pub fn data(&self) -> &[u8] {
        &self.data[..]
    }

    /// Check if memory is protected
    pub const fn is_protected(&self) -> bool {
        self.protected
    }
}

/// Trait for memory protection operations
pub trait MemoryProtectorTrait: Send + Sync {
    /// Initialize memory protection
    fn initialize(&self) -> impl Future<Output = Result<(), BearDogError>> + Send;

    /// Protect key material in memory
    fn protect_key_material(
        &self,
        key_material: &[u8],
    ) -> impl Future<Output = Result<ProtectedMemory, BearDogError>> + Send;

    /// Unprotect key material
    fn unprotect_key_material(
        &self,
        protected: &ProtectedMemory,
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    /// Securely zeroize key material
    fn zeroize_key_material(
        &self,
        key_material: &mut [u8],
    ) -> impl Future<Output = Result<(), BearDogError>> + Send;
}

/// Trait for audit logging
pub trait AuditLogger: Send + Sync {
    /// Log an operation
    fn log_operation(
        &self,
        operation: &AuditLogEntry,
    ) -> impl Future<Output = Result<(), BearDogError>> + Send;

    /// Get audit log entries with filter
    fn get_audit_log(
        &self,
        filter: &AuditLogFilter,
    ) -> impl Future<Output = Result<Vec<AuditLogEntry>, BearDogError>> + Send;
}

/// Audit logger dispatch (replaces `Arc<dyn AuditLogger>`).
pub enum AuditLoggerBackend {
    /// Default persistent audit logger
    Default(super::audit::logger::DefaultAuditLogger),
}

impl AuditLogger for AuditLoggerBackend {
    fn log_operation(
        &self,
        operation: &AuditLogEntry,
    ) -> impl Future<Output = Result<(), BearDogError>> + Send {
        let op = operation.clone();
        let slf = self;
        async move {
            match slf {
                Self::Default(l) => AuditLogger::log_operation(l, &op).await,
            }
        }
    }

    fn get_audit_log(
        &self,
        filter: &AuditLogFilter,
    ) -> impl Future<Output = Result<Vec<AuditLogEntry>, BearDogError>> + Send {
        let filter = filter.clone();
        let slf = self;
        async move {
            match slf {
                Self::Default(l) => AuditLogger::get_audit_log(l, &filter).await,
            }
        }
    }
}

/// Encryption key backend (replaces `Arc<dyn EncryptionKeyTrait>`).
pub enum EncryptionKeyBackend {
    /// Default AES-256-GCM software key
    Default(DefaultEncryptionKey),
}

/// Software key store
pub struct SoftwareKeyStore {
    /// Storage backend
    pub storage_backend: Arc<StorageBackend>,
    /// Encryption key
    pub encryption_key: Arc<EncryptionKeyBackend>,
    /// Key cache (LRU cache)
    pub key_cache: Arc<RwLock<HashMap<String, SoftwareKey>>>,
}

impl SoftwareKeyStore {
    /// Create a new software key store
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub async fn new(
        _config: &crate::tunnel::hsm::software_hsm::KeyStoreConfig,
    ) -> Result<Self, BearDogError> {
        Ok(Self {
            storage_backend: Arc::new(StorageBackend::InMemory(InMemoryStorageBackend)),
            encryption_key: Arc::new(EncryptionKeyBackend::Default(
                DefaultEncryptionKey::default(),
            )),
            key_cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Get key from store
    ///
    /// # Errors
    /// Returns an error if key not found or retrieval fails
    pub async fn get_key(&self, key_id: &str) -> Result<SoftwareKey, BearDogError> {
        let cache = self.key_cache.read().await;
        cache
            .get(key_id)
            .cloned()
            .ok_or_else(|| BearDogError::not_found(format!("Key not found: {key_id}")))
    }

    /// Store key in cache
    ///
    /// # Errors
    /// Returns an error if storage fails
    pub async fn store_key(&self, key: SoftwareKey) -> Result<(), BearDogError> {
        let mut cache = self.key_cache.write().await;
        cache.insert(key.id.clone(), key);
        Ok(())
    }

    /// Initialize the key store
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub async fn initialize(&self) -> Result<(), BearDogError> {
        debug!("Initializing key store");
        Ok(())
    }

    /// Delete a key from the store
    ///
    /// # Errors
    /// Returns an error if deletion fails or key not found
    pub async fn delete_key(&mut self, key_id: &str) -> Result<(), BearDogError> {
        let mut cache = self.key_cache.write().await;
        if cache.remove(key_id).is_some() {
            debug!("Deleted key: {}", key_id);
            Ok(())
        } else {
            Err(BearDogError::not_found(format!("Key not found: {key_id}")))
        }
    }
}

/// Software health monitor
pub struct SoftwareHealthMonitor {
    /// Current health status
    pub health_status: Arc<RwLock<HsmHealthStatus>>,
    /// Performance metrics
    pub metrics: Arc<RwLock<PerformanceMetrics>>,
}

impl SoftwareHealthMonitor {
    /// Create a new software health monitor
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub async fn new() -> Result<Self, BearDogError> {
        use chrono::Utc;
        Ok(Self {
            health_status: Arc::new(RwLock::new(HsmHealthStatus {
                is_healthy: true,
                last_check: Utc::now(),
                error_message: None,
                performance_metrics: PerformanceMetrics {
                    operations_per_second: 0.0,
                    average_latency_ms: 0.0,
                    success_rate: 100.0,
                    memory_usage_mb: 0.0,
                    cpu_usage_percent: 0.0,
                    network_throughput_bps: 0.0,
                    latency_ms: 0.0,
                    throughput_mbps: 0.0,
                    uptime_seconds: 0,
                },
            })),
            metrics: Arc::new(RwLock::new(PerformanceMetrics {
                operations_per_second: 0.0,
                average_latency_ms: 0.0,
                success_rate: 100.0,
                memory_usage_mb: 0.0,
                cpu_usage_percent: 0.0,
                network_throughput_bps: 0.0,
                latency_ms: 0.0,
                throughput_mbps: 0.0,
                uptime_seconds: 0,
            })),
        })
    }

    /// Check health status
    ///
    /// # Errors
    /// Returns an error if health check fails
    pub async fn check_health(&self) -> Result<HsmHealthStatus, BearDogError> {
        let status = self.health_status.read().await;
        Ok(status.clone())
    }
}

/// Default encryption key implementation using AES-256-GCM
#[derive(Clone, Copy)]
pub struct DefaultEncryptionKey {
    /// Root key for encryption
    root_key: [u8; 32],
}

impl DefaultEncryptionKey {
    /// # Errors
    ///
    /// Returns an error if decryption fails.
    /// Create new default encryption key
    pub async fn create(
        _config: &SoftwareHsmConfig,
    ) -> Result<Arc<EncryptionKeyBackend>, BearDogError> {
        let root_key = *b"BearDog_RootKey_256bit_Secure!!!"; // 32 bytes
        Ok(Arc::new(EncryptionKeyBackend::Default(Self { root_key })))
    }

    /// Encrypt data (AES-256-GCM).
    ///
    /// # Errors
    ///
    /// Returns an error if encryption or random nonce generation fails.
    pub fn encrypt(
        &self,
        plaintext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send + '_ {
        let plaintext = plaintext.to_vec();
        let root_key = self.root_key;
        async move {
            use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce, aead::Aead};
            use rand::RngCore;

            let key = Key::<Aes256Gcm>::from_slice(&root_key);
            let cipher = Aes256Gcm::new(key);

            let mut nonce_bytes = [0u8; 12];
            rand::rng().fill_bytes(&mut nonce_bytes);
            let nonce = Nonce::from_slice(&nonce_bytes);

            let ciphertext = cipher.encrypt(nonce, plaintext.as_slice()).map_err(|e| {
                BearDogError::crypto_error(format!("AES-256-GCM encryption failed: {e}"))
            })?;

            let mut result = nonce_bytes.to_vec();
            result.extend_from_slice(&ciphertext);
            Ok(result)
        }
    }

    /// Decrypt data (AES-256-GCM).
    ///
    /// # Errors
    ///
    /// Returns an error if the ciphertext is malformed or authentication fails.
    pub fn decrypt(
        &self,
        ciphertext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send + '_ {
        let ciphertext = ciphertext.to_vec();
        let root_key = self.root_key;
        async move {
            use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce, aead::Aead};

            if ciphertext.len() < 12 {
                return Err(BearDogError::crypto_error(
                    "Ciphertext too short to contain nonce",
                ));
            }

            let (nonce_bytes, encrypted_data) = ciphertext.split_at(12);
            let nonce = Nonce::from_slice(nonce_bytes);

            let key = Key::<Aes256Gcm>::from_slice(&root_key);
            let cipher = Aes256Gcm::new(key);

            cipher.decrypt(nonce, encrypted_data).map_err(|e| {
                BearDogError::crypto_error(format!("AES-256-GCM decryption failed: {e}"))
            })
        }
    }
}

impl Default for DefaultEncryptionKey {
    fn default() -> Self {
        Self {
            root_key: *b"BearDog_RootKey_256bit_Secure!!!", // 32 bytes
        }
    }
}

impl EncryptionKeyBackend {
    /// Encrypt data
    ///
    /// # Errors
    ///
    /// Returns an error if encryption fails.
    pub fn encrypt(
        &self,
        data: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send + '_ {
        let slf = self;
        let data = data.to_vec();
        async move {
            match slf {
                Self::Default(k) => k.encrypt(&data).await,
            }
        }
    }

    /// Decrypt data
    ///
    /// # Errors
    ///
    /// Returns an error if decryption fails.
    pub fn decrypt(
        &self,
        ciphertext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send + '_ {
        let slf = self;
        let ciphertext = ciphertext.to_vec();
        async move {
            match slf {
                Self::Default(k) => k.decrypt(&ciphertext).await,
            }
        }
    }
}

// NOTE: Removed duplicate Default implementation
// AuditLogFilter already has #[derive(Default)] in audit/types.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protected_memory_creation() -> Result<(), Box<dyn std::error::Error>> {
        let data = vec![1, 2, 3, 4, 5];
        let protected = ProtectedMemory::new(data.clone(), true);

        assert!(protected.is_protected());
        assert_eq!(protected.data(), &data);
        Ok(())
    }

    #[test]
    fn test_software_key_creation() -> Result<(), Box<dyn std::error::Error>> {
        let key_material = ProtectedMemory::new(vec![0u8; 32], true);
        let metadata = KeyMetadata::new("test-key".to_string(), KeyType::Ed25519);

        let key = SoftwareKey::new("test-key".to_string(), KeyType::Aes, key_material, metadata);

        assert_eq!(key.id(), "test-key");
        assert_eq!(key.key_type(), &KeyType::Aes);
        Ok(())
    }

    #[tokio::test]
    async fn test_memory_storage_backend() -> Result<(), Box<dyn std::error::Error>> {
        let backend = StorageBackend::Memory(MemoryStorageBackend::new().await?);

        backend.initialize().await?;

        let key_id = "test-key";
        let key_data = vec![1, 2, 3, 4, 5];

        backend.store(key_id, &key_data).await?;

        let retrieved = backend.retrieve(key_id).await?;
        assert_eq!(retrieved, key_data);

        let keys = backend.list_keys().await?;
        assert_eq!(keys.len(), 1);
        assert!(keys.contains(&key_id.to_string()));

        backend.delete(key_id).await?;

        let result = backend.retrieve(key_id).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_memory_storage_backup_restore() -> Result<(), Box<dyn std::error::Error>> {
        let backend = StorageBackend::Memory(MemoryStorageBackend::new().await?);

        backend.store("key1", &[1, 2, 3]).await?;
        backend.store("key2", &[4, 5, 6]).await?;

        let backup = backend.backup().await?;

        let backend2 = StorageBackend::Memory(MemoryStorageBackend::new().await?);
        backend2.restore(&backup).await?;

        let key1 = backend2.retrieve("key1").await?;
        let key2 = backend2.retrieve("key2").await?;

        assert_eq!(key1, vec![1, 2, 3]);
        assert_eq!(key2, vec![4, 5, 6]);
        Ok(())
    }

    #[tokio::test]
    async fn test_default_encryption_key() -> Result<(), Box<dyn std::error::Error>> {
        let config = SoftwareHsmConfig {
            storage: SoftwareHsmStorageKind::InMemory,
            memory_protection: MemoryProtectionLevel::Medium,
            key_storage: KeyStorageType::Encrypted,
            audit_logging: true,
            max_keys: Some(100),
        };

        let enc_key = DefaultEncryptionKey::create(&config).await?;

        let plaintext = b"Hello, BearDog!";
        let ciphertext = enc_key.encrypt(plaintext).await?;

        assert_ne!(ciphertext, plaintext);
        assert!(ciphertext.len() > plaintext.len()); // includes nonce

        let decrypted = enc_key.decrypt(&ciphertext).await?;
        assert_eq!(decrypted, plaintext);
        Ok(())
    }

    #[tokio::test]
    async fn test_default_encryption_key_invalid_ciphertext()
    -> Result<(), Box<dyn std::error::Error>> {
        let config = SoftwareHsmConfig {
            storage: SoftwareHsmStorageKind::InMemory,
            memory_protection: MemoryProtectionLevel::Medium,
            key_storage: KeyStorageType::Encrypted,
            audit_logging: true,
            max_keys: Some(100),
        };

        let enc_key = DefaultEncryptionKey::create(&config).await?;

        // Too short
        let result = enc_key.decrypt(&[1, 2, 3]).await;
        assert!(result.is_err());

        // Invalid ciphertext
        let result = enc_key.decrypt(&[0u8; 50]).await;
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_audit_log_filter_default() -> Result<(), Box<dyn std::error::Error>> {
        let filter = AuditLogFilter::default();
        // AuditLogFilter::default() creates a default filter
        assert!(filter.from_time.is_none());
        Ok(())
    }

    #[test]
    fn test_storage_backend_clone() -> Result<(), Box<dyn std::error::Error>> {
        let backend1 = SoftwareHsmStorageKind::InMemory;
        let backend2 = backend1.clone();

        match backend2 {
            SoftwareHsmStorageKind::InMemory => {}
            _ => panic!("Expected InMemory variant"),
        }

        let backend3 = SoftwareHsmStorageKind::File {
            path: "/tmp/keys".to_string(),
        };
        let backend4 = backend3.clone();

        match backend4 {
            SoftwareHsmStorageKind::File { path } => assert_eq!(path, "/tmp/keys"),
            _ => panic!("Expected File variant"),
        }
        Ok(())
    }
}
