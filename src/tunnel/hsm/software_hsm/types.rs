//! # Software HSM Types and Traits
//!
//! This module defines all the types, traits, and structures used by the Software HSM implementation.
//! It provides the foundational components for secure key management, cryptographic operations,
//! and storage backends.

use crate::error::{BearDogError, BearDogResult};
use crate::tunnel::hsm::types::*;
use async_trait::async_trait;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Software key representation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SoftwareKey {
    pub id: String,
    pub key_type: KeyType,
    pub key_material: ProtectedMemory,
    pub created_at: chrono::DateTime<Utc>,
    pub metadata: KeyMetadata,
}

/// Protected memory for secure key storage
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProtectedMemory {
    pub data: Vec<u8>,
    pub protected: bool,
}

/// Audit log entry for HSM operations
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuditLogEntry {
    pub timestamp: chrono::DateTime<Utc>,
    pub operation: String,
    pub key_id: Option<String>,
    pub user_id: Option<String>,
    pub result: String,
    pub details: HashMap<String, String>,
}

/// Filter for audit log queries
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuditLogFilter {
    pub start_time: Option<chrono::DateTime<Utc>>,
    pub end_time: Option<chrono::DateTime<Utc>>,
    pub operation: Option<String>,
    pub key_id: Option<String>,
    pub user_id: Option<String>,
}

/// Crypto provider interface for cryptographic operations
#[async_trait]
pub trait CryptoProvider: Send + Sync {
    /// Initialize the crypto provider
    async fn initialize(&self) -> BearDogResult<()>;

    /// Generate secure key material
    async fn generate_key_material(&self, key_type: &KeyType) -> BearDogResult<Vec<u8>>;

    /// Encrypt data with key
    async fn encrypt(&self, key_material: &[u8], plaintext: &[u8]) -> BearDogResult<Vec<u8>>;

    /// Decrypt data with key
    async fn decrypt(&self, key_material: &[u8], ciphertext: &[u8]) -> BearDogResult<Vec<u8>>;

    /// Sign data with key
    async fn sign(&self, key_material: &[u8], data: &[u8]) -> BearDogResult<Vec<u8>>;

    /// Verify signature with key
    async fn verify(
        &self,
        key_material: &[u8],
        data: &[u8],
        signature: &[u8],
    ) -> BearDogResult<bool>;

    /// Derive key from master key
    async fn derive_key(&self, master_key: &[u8], derivation_data: &[u8])
        -> BearDogResult<Vec<u8>>;
}

/// Memory protector interface for secure key storage
#[async_trait]
pub trait MemoryProtector: Send + Sync {
    /// Initialize memory protector
    async fn initialize(&self) -> BearDogResult<()>;

    /// Protect key material in memory
    async fn protect_key_material(&self, key_material: &[u8]) -> BearDogResult<ProtectedMemory>;

    /// Unprotect key material for use
    async fn unprotect_key_material(&self, protected: &ProtectedMemory) -> BearDogResult<Vec<u8>>;

    /// Securely zeroize key material
    async fn zeroize_key_material(&self, key_material: &[u8]) -> BearDogResult<()>;
}

/// Storage backend interface for key persistence
#[async_trait]
pub trait StorageBackend: Send + Sync {
    /// Initialize storage backend
    async fn initialize(&self) -> BearDogResult<()>;

    /// Store encrypted key
    async fn store(&self, key_id: &str, encrypted_key: &[u8]) -> BearDogResult<()>;

    /// Load encrypted key
    async fn load(&self, key_id: &str) -> BearDogResult<Vec<u8>>;

    /// Delete key
    async fn delete(&self, key_id: &str) -> BearDogResult<()>;

    /// List all key IDs
    async fn list_keys(&self) -> BearDogResult<Vec<String>>;

    /// Backup storage
    async fn backup(&self) -> BearDogResult<Vec<u8>>;

    /// Restore from backup
    async fn restore(&self, backup_data: &[u8]) -> BearDogResult<()>;
}

/// Encryption key interface for key protection
#[async_trait]
pub trait EncryptionKey: Send + Sync {
    /// Initialize encryption key
    async fn initialize(&self) -> BearDogResult<()>;

    /// Encrypt data
    async fn encrypt(&self, plaintext: &[u8]) -> BearDogResult<Vec<u8>>;

    /// Decrypt data
    async fn decrypt(&self, ciphertext: &[u8]) -> BearDogResult<Vec<u8>>;
}

/// Audit logger interface for operation logging
#[async_trait]
pub trait AuditLogger: Send + Sync {
    /// Log HSM operation
    async fn log_operation(&self, operation: &AuditLogEntry) -> BearDogResult<()>;

    /// Get audit log entries
    async fn get_audit_log(&self, filter: &AuditLogFilter) -> BearDogResult<Vec<AuditLogEntry>>;
}

/// Software key store for encrypted key storage
pub struct SoftwareKeyStore {
    pub storage_backend: Arc<dyn StorageBackend>,
    pub encryption_key: Arc<dyn EncryptionKey>,
    pub key_cache: Arc<RwLock<lru::LruCache<String, SoftwareKey>>>,
}

/// Software health monitor for system status
pub struct SoftwareHealthMonitor {
    pub health_status: Arc<RwLock<HsmHealthStatus>>,
    pub metrics: Arc<RwLock<PerformanceMetrics>>,
}

/// Default memory protector implementation
pub struct DefaultMemoryProtector {
    pub config: MemoryConfig,
}

/// Default audit logger implementation
pub struct DefaultAuditLogger;

/// File storage backend implementation
pub struct FileStorageBackend;

/// Database storage backend implementation
pub struct DatabaseStorageBackend;

/// Memory storage backend implementation
pub struct MemoryStorageBackend {
    pub storage: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

/// Default encryption key implementation
pub struct DefaultEncryptionKey;

/// Rust crypto provider implementation
pub struct RustCryptoProvider;

/// Ring crypto provider implementation
pub struct RingCryptoProvider;

/// OpenSSL crypto provider implementation
pub struct OpenSslCryptoProvider;

/// Rust Software HSM implementation
pub struct RustSoftwareHsm {
    pub config: SoftwareHsmConfig,
    pub key_store: Arc<RwLock<SoftwareKeyStore>>,
    pub crypto_provider: Arc<dyn CryptoProvider>,
    pub memory_protector: Arc<dyn MemoryProtector>,
    pub audit_logger: Arc<dyn AuditLogger>,
    pub health_monitor: Arc<SoftwareHealthMonitor>,
}

impl ProtectedMemory {
    /// Create new protected memory
    pub fn new(data: Vec<u8>, protected: bool) -> Self {
        Self { data, protected }
    }

    /// Get data reference
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Check if protected
    pub fn is_protected(&self) -> bool {
        self.protected
    }
}

impl SoftwareKey {
    /// Create new software key
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
    pub fn key_type(&self) -> &KeyType {
        &self.key_type
    }

    /// Get key material
    pub fn key_material(&self) -> &ProtectedMemory {
        &self.key_material
    }

    /// Get metadata
    pub fn metadata(&self) -> &KeyMetadata {
        &self.metadata
    }

    /// Get creation timestamp
    pub fn created_at(&self) -> chrono::DateTime<Utc> {
        self.created_at
    }
}

impl AuditLogEntry {
    /// Create new audit log entry
    pub fn new(
        operation: String,
        key_id: Option<String>,
        user_id: Option<String>,
        result: String,
        details: HashMap<String, String>,
    ) -> Self {
        Self {
            timestamp: Utc::now(),
            operation,
            key_id,
            user_id,
            result,
            details,
        }
    }

    /// Create success entry
    pub fn success(
        operation: String,
        key_id: Option<String>,
        user_id: Option<String>,
    ) -> Self {
        Self::new(operation, key_id, user_id, "success".to_string(), HashMap::new())
    }

    /// Create failure entry
    pub fn failure(
        operation: String,
        key_id: Option<String>,
        user_id: Option<String>,
        error: String,
    ) -> Self {
        let mut details = HashMap::new();
        details.insert("error".to_string(), error);
        Self::new(operation, key_id, user_id, "failure".to_string(), details)
    }
}

impl AuditLogFilter {
    /// Create new audit log filter
    pub fn new() -> Self {
        Self {
            start_time: None,
            end_time: None,
            operation: None,
            key_id: None,
            user_id: None,
        }
    }

    /// Set time range
    pub fn with_time_range(
        mut self,
        start: chrono::DateTime<Utc>,
        end: chrono::DateTime<Utc>,
    ) -> Self {
        self.start_time = Some(start);
        self.end_time = Some(end);
        self
    }

    /// Set operation filter
    pub fn with_operation(mut self, operation: String) -> Self {
        self.operation = Some(operation);
        self
    }

    /// Set key ID filter
    pub fn with_key_id(mut self, key_id: String) -> Self {
        self.key_id = Some(key_id);
        self
    }

    /// Set user ID filter
    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }
}

impl Default for AuditLogFilter {
    fn default() -> Self {
        Self::new()
    }
} 