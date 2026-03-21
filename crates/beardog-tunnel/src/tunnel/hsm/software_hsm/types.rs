// SPDX-License-Identifier: AGPL-3.0-only

//! Software HSM Types
//!
//! Core type definitions for the Software HSM implementation.

// ✅ MIGRATED: Using real crypto providers from software_hsm/crypto_providers and canonical trait
pub use beardog_types::hsm::CryptoProvider; // Canonical trait
pub use beardog_types::hsm::InMemoryStorageBackend;

// Use proper KeyStoreConfig from beardog-types
use crate::tunnel::hsm::software_hsm::{KeyMetadata, MemoryConfig};
use crate::tunnel::hsm::types::{
    HsmHealthStatus, KeyStorageType, KeyType, MemoryProtectionLevel, PerformanceMetrics,
};
use beardog_errors::BearDogError;
use beardog_types::hsm::KeyStoreConfig;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::debug;

pub use super::audit::types::{AuditLogEntry, AuditLogFilter, OperationResult};

/// Storage backend variants for the Software HSM
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum StorageBackend {
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

/// Trait for storage backend implementations
#[async_trait::async_trait]
pub trait StorageBackendTrait: Send + Sync {
    /// Initialize the storage backend
    async fn initialize(&self) -> Result<(), BearDogError>;

    /// Store encrypted key data
    async fn store(&self, key_id: &str, key_data: &[u8]) -> Result<(), BearDogError>;

    /// Retrieve encrypted key data
    async fn retrieve(&self, key_id: &str) -> Result<Vec<u8>, BearDogError>;

    /// Delete a key
    async fn delete(&self, key_id: &str) -> Result<(), BearDogError>;

    /// List all key IDs
    async fn list_keys(&self) -> Result<Vec<String>, BearDogError>;

    /// Create a backup of all keys
    async fn backup(&self) -> Result<Vec<u8>, BearDogError>;

    /// Restore from backup
    async fn restore(&self, backup_data: &[u8]) -> Result<(), BearDogError>;
}

/// Trait for encryption key operations
#[async_trait::async_trait]
pub trait EncryptionKeyTrait: Send + Sync {
    /// Encrypt data
    async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError>;

    /// Decrypt data
    async fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, BearDogError>;
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
    pub storage: StorageBackend,
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
    /// Encrypted data
    pub data: Vec<u8>,
    /// Whether memory is protected
    pub protected: bool,
}

impl ProtectedMemory {
    /// Create new protected memory
    pub const fn new(data: Vec<u8>, protected: bool) -> Self {
        Self { data, protected }
    }

    /// Get data reference
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Check if memory is protected
    pub const fn is_protected(&self) -> bool {
        self.protected
    }
}

/// Trait for memory protection operations
#[async_trait::async_trait]
pub trait MemoryProtectorTrait: Send + Sync {
    /// Initialize memory protection
    async fn initialize(&self) -> Result<(), BearDogError>;

    /// Protect key material in memory
    async fn protect_key_material(
        &self,
        key_material: &[u8],
    ) -> Result<ProtectedMemory, BearDogError>;

    /// Unprotect key material
    async fn unprotect_key_material(
        &self,
        protected: &ProtectedMemory,
    ) -> Result<Vec<u8>, BearDogError>;

    /// Securely zeroize key material
    async fn zeroize_key_material(&self, key_material: &mut [u8]) -> Result<(), BearDogError>;
}

/// Trait for audit logging
#[async_trait::async_trait]
pub trait AuditLogger: Send + Sync {
    /// Log an operation
    async fn log_operation(&self, operation: &AuditLogEntry) -> Result<(), BearDogError>;

    /// Get audit log entries with filter
    async fn get_audit_log(
        &self,
        filter: &AuditLogFilter,
    ) -> Result<Vec<AuditLogEntry>, BearDogError>;
}

/// Software key store
pub struct SoftwareKeyStore {
    /// Storage backend
    pub storage_backend: Arc<dyn StorageBackendTrait>,
    /// Encryption key
    pub encryption_key: Arc<dyn EncryptionKeyTrait>,
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
            storage_backend: Arc::new(InMemoryStorageBackend),
            encryption_key: Arc::new(DefaultEncryptionKey::default()),
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

/// Default memory protector implementation
pub struct DefaultMemoryProtector {
    /// Memory configuration
    pub config: MemoryConfig,
}

/// Default audit logger implementation
pub struct DefaultAuditLogger {
    /// Persistent audit storage
    pub storage: Arc<super::audit::PersistentAuditStorage>,
}

/// File-based storage backend
pub struct FileStorageBackend {
    /// Storage path
    path: String,
}

impl FileStorageBackend {
    /// Create new file storage backend
    pub async fn new(config: &KeyStoreConfig) -> Result<Self, BearDogError> {
        // Use the path from the new canonical KeyStoreConfig
        let path = config.path.to_string_lossy().to_string();
        Ok(Self { path })
    }
}

#[async_trait::async_trait]
impl StorageBackendTrait for FileStorageBackend {
    async fn initialize(&self) -> Result<(), BearDogError> {
        // Create directory if it doesn't exist
        std::fs::create_dir_all(&self.path).map_err(|e| BearDogError::io_error(&e.to_string()))?;
        Ok(())
    }

    async fn store(&self, key_id: &str, encrypted_key: &[u8]) -> Result<(), BearDogError> {
        let file_path = format!("{}/{}.key", self.path, key_id);
        std::fs::write(&file_path, encrypted_key)
            .map_err(|e| BearDogError::io_error(&e.to_string()))?;
        Ok(())
    }

    async fn retrieve(&self, key_id: &str) -> Result<Vec<u8>, BearDogError> {
        let file_path = format!("{}/{}.key", self.path, key_id);
        std::fs::read(&file_path)
            .map_err(|e| BearDogError::not_found(format!("Key not found: {e}")))
    }

    async fn delete(&self, key_id: &str) -> Result<(), BearDogError> {
        let file_path = format!("{}/{}.key", self.path, key_id);
        std::fs::remove_file(&file_path).map_err(|e| BearDogError::io_error(&e.to_string()))?;
        Ok(())
    }

    async fn list_keys(&self) -> Result<Vec<String>, BearDogError> {
        let entries =
            std::fs::read_dir(&self.path).map_err(|e| BearDogError::io_error(&e.to_string()))?;

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

    async fn backup(&self) -> Result<Vec<u8>, BearDogError> {
        let keys = self.list_keys().await?;
        let mut backup_data = HashMap::new();

        for key_id in keys {
            let data = self.retrieve(&key_id).await?;
            backup_data.insert(key_id, data);
        }

        postcard::to_allocvec(&backup_data).map_err(|e| BearDogError::internal(e.to_string()))
    }

    async fn restore(&self, backup_data: &[u8]) -> Result<(), BearDogError> {
        let backup: HashMap<String, Vec<u8>> =
            postcard::from_bytes(backup_data).map_err(|e| BearDogError::internal(e.to_string()))?;

        for (key_id, data) in backup {
            self.store(&key_id, &data).await?;
        }
        Ok(())
    }
}

/// Database storage backend (placeholder)
pub struct DatabaseStorageBackend;

impl DatabaseStorageBackend {
    /// Create new database storage backend
    pub async fn new(_config: &KeyStoreConfig) -> Result<Self, BearDogError> {
        Ok(Self)
    }
}

#[async_trait::async_trait]
impl StorageBackendTrait for DatabaseStorageBackend {
    async fn initialize(&self) -> Result<(), BearDogError> {
        Err(BearDogError::unsupported_operation(
            "DatabaseStorageBackend not yet implemented".to_string(),
        ))
    }

    async fn store(&self, _key_id: &str, _encrypted_key: &[u8]) -> Result<(), BearDogError> {
        Err(BearDogError::unsupported_operation(
            "DatabaseStorageBackend not yet implemented".to_string(),
        ))
    }

    async fn retrieve(&self, _key_id: &str) -> Result<Vec<u8>, BearDogError> {
        Err(BearDogError::unsupported_operation(
            "DatabaseStorageBackend not yet implemented".to_string(),
        ))
    }

    async fn delete(&self, _key_id: &str) -> Result<(), BearDogError> {
        Err(BearDogError::unsupported_operation(
            "DatabaseStorageBackend not yet implemented".to_string(),
        ))
    }

    async fn list_keys(&self) -> Result<Vec<String>, BearDogError> {
        Err(BearDogError::unsupported_operation(
            "DatabaseStorageBackend not yet implemented".to_string(),
        ))
    }

    async fn backup(&self) -> Result<Vec<u8>, BearDogError> {
        Err(BearDogError::unsupported_operation(
            "DatabaseStorageBackend not yet implemented".to_string(),
        ))
    }

    async fn restore(&self, _backup_data: &[u8]) -> Result<(), BearDogError> {
        Err(BearDogError::unsupported_operation(
            "DatabaseStorageBackend not yet implemented".to_string(),
        ))
    }
}

/// Implement StorageBackendTrait for the canonical InMemoryStorageBackend
#[async_trait::async_trait]
impl StorageBackendTrait for beardog_types::hsm::InMemoryStorageBackend {
    async fn initialize(&self) -> Result<(), BearDogError> {
        Ok(())
    }

    async fn store(&self, _key_id: &str, _encrypted_key: &[u8]) -> Result<(), BearDogError> {
        // Stub implementation - keys are not actually stored in memory
        Ok(())
    }

    async fn retrieve(&self, _key_id: &str) -> Result<Vec<u8>, BearDogError> {
        // Stub implementation - always returns not found
        Err(BearDogError::not_found(
            "Key not found in ephemeral storage".to_string(),
        ))
    }

    async fn delete(&self, _key_id: &str) -> Result<(), BearDogError> {
        Ok(())
    }

    async fn list_keys(&self) -> Result<Vec<String>, BearDogError> {
        Ok(vec![])
    }

    async fn backup(&self) -> Result<Vec<u8>, BearDogError> {
        Ok(vec![])
    }

    async fn restore(&self, _data: &[u8]) -> Result<(), BearDogError> {
        Ok(())
    }
}

/// In-memory storage backend
pub struct MemoryStorageBackend {
    /// In-memory key storage
    pub storage: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl MemoryStorageBackend {
    /// Create new memory storage backend
    pub async fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            storage: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        })
    }
}

#[async_trait::async_trait]
impl StorageBackendTrait for MemoryStorageBackend {
    async fn initialize(&self) -> Result<(), BearDogError> {
        Ok(())
    }

    async fn store(&self, key_id: &str, encrypted_key: &[u8]) -> Result<(), BearDogError> {
        let mut storage = self.storage.write().await;
        storage.insert(key_id.to_string(), encrypted_key.to_vec());
        Ok(())
    }

    async fn retrieve(&self, key_id: &str) -> Result<Vec<u8>, BearDogError> {
        let storage = self.storage.read().await;
        storage
            .get(key_id)
            .cloned()
            .ok_or_else(|| BearDogError::not_found(format!("Key not found: {key_id}")))
    }

    async fn delete(&self, key_id: &str) -> Result<(), BearDogError> {
        let mut storage = self.storage.write().await;
        storage.remove(key_id);
        Ok(())
    }

    async fn list_keys(&self) -> Result<Vec<String>, BearDogError> {
        let storage = self.storage.read().await;
        Ok(storage.keys().cloned().collect())
    }

    async fn backup(&self) -> Result<Vec<u8>, BearDogError> {
        let storage = self.storage.read().await;
        postcard::to_allocvec(&*storage).map_err(|e| BearDogError::internal(e.to_string()))
    }

    async fn restore(&self, backup_data: &[u8]) -> Result<(), BearDogError> {
        let restored: HashMap<String, Vec<u8>> =
            postcard::from_bytes(backup_data).map_err(|e| BearDogError::internal(e.to_string()))?;
        let mut storage = self.storage.write().await;
        *storage = restored;
        Ok(())
    }
}

/// Default encryption key implementation using AES-256-GCM
pub struct DefaultEncryptionKey {
    /// Root key for encryption
    root_key: [u8; 32],
}

impl DefaultEncryptionKey {
    /// Create new default encryption key
    pub async fn create(
        _config: &SoftwareHsmConfig,
    ) -> Result<Arc<dyn EncryptionKeyTrait>, BearDogError> {
        // In production, derive from HSM root key or secure key derivation
        let root_key = *b"BearDog_RootKey_256bit_Secure!!!"; // 32 bytes

        Ok(Arc::new(Self { root_key }))
    }
}

impl Default for DefaultEncryptionKey {
    fn default() -> Self {
        Self {
            root_key: *b"BearDog_RootKey_256bit_Secure!!!", // 32 bytes
        }
    }
}

#[async_trait::async_trait]
impl EncryptionKeyTrait for DefaultEncryptionKey {
    async fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce, aead::Aead};
        use rand::RngCore;

        let key = Key::<Aes256Gcm>::from_slice(&self.root_key);
        let cipher = Aes256Gcm::new(key);

        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher.encrypt(nonce, plaintext).map_err(|e| {
            BearDogError::crypto_error(format!("AES-256-GCM encryption failed: {e}"))
        })?;

        // Prepend nonce to ciphertext
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    async fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce, aead::Aead};

        if ciphertext.len() < 12 {
            return Err(BearDogError::crypto_error(
                "Ciphertext too short to contain nonce",
            ));
        }

        let (nonce_bytes, encrypted_data) = ciphertext.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let key = Key::<Aes256Gcm>::from_slice(&self.root_key);
        let cipher = Aes256Gcm::new(key);

        cipher
            .decrypt(nonce, encrypted_data)
            .map_err(|e| BearDogError::crypto_error(format!("AES-256-GCM decryption failed: {e}")))
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
        let backend = MemoryStorageBackend::new().await?;

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
        let backend = MemoryStorageBackend::new().await?;

        backend.store("key1", &[1, 2, 3]).await?;
        backend.store("key2", &[4, 5, 6]).await?;

        let backup = backend.backup().await?;

        let backend2 = MemoryStorageBackend::new().await?;
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
            storage: StorageBackend::InMemory,
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
            storage: StorageBackend::InMemory,
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
        let backend1 = StorageBackend::InMemory;
        let backend2 = backend1.clone();

        match backend2 {
            StorageBackend::InMemory => {}
            _ => panic!("Expected InMemory variant"),
        }

        let backend3 = StorageBackend::File {
            path: "/tmp/keys".to_string(),
        };
        let backend4 = backend3.clone();

        match backend4 {
            StorageBackend::File { path } => assert_eq!(path, "/tmp/keys"),
            _ => panic!("Expected File variant"),
        }
        Ok(())
    }
}
