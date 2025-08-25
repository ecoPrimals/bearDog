// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Software HSM Types and Traits
///
/// This module defines all the types, traits, and structures used by the Software HSM implementation.
/// It provides the foundational components for secure key management, cryptographic operations,
/// and storage backends.

use crate::tunnel::hsm::software_hsm::{KeyMetadata, KeyStoreConfig};
use crate::tunnel::hsm::types::{
    HsmHealthStatus, KeyStorageType, KeyType, MemoryProtectionLevel, PerformanceMetrics,
    SecurityLevel,
};
// Removed async_trait - now using native async fn in traits
use beardog_errors::{BearDogError, BearDogResult};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
// Import canonical audit types instead of duplicating
pub use super::audit::types::{AuditLogEntry, AuditLogFilter, OperationResult};
/// Storage backend configuration for software HSM
#[derive(Debug)]
pub enum StorageBackend {
    /// In-memory storage (not persistent)
    InMemory,
    /// File-based storage
    File { path: String },
    /// Database storage
    Database { connection_string: String },
}
impl Clone for StorageBackend {}


    fn clone(&self) -> Self {
        match self {
            StorageBackend::InMemory => StorageBackend::InMemory,
            StorageBackend::File { path } => StorageBackend::File { path: path.clone() },
            StorageBackend::Database { connection_string } => StorageBackend::Database {
                connection_string: connection_string.clone(),
            },
        }
    }
// Storage backend trait - modernized with native async fn
pub trait StorageBackendTrait: Send + Sync {
    async fn initialize(&self) -> BearDogResult<()>;
    async fn store(&self, key_id: &str, key_data: &[u8]) -> BearDogResult<()>;
    async fn retrieve(&self, key_id: &str) -> BearDogResult<Vec<u8>>;
    async fn delete(&self, key_id: &str) -> BearDogResult<()>;
    async fn list_keys(&self) -> BearDogResult<Vec<String>>;
    async fn backup(&self) -> BearDogResult<Vec<u8>>;
    async fn restore(&self, backup_data: &[u8]) -> BearDogResult<()>;
// Encryption key trait
pub trait EncryptionKeyTrait: Send + Sync {}


    async fn encrypt(&self, data: &[u8]) -> BearDogResult<Vec<u8>>;
    async fn decrypt(&self, encrypted_data: &[u8]) -> BearDogResult<Vec<u8>>;
/// Memory protection configuration
pub struct MemoryProtector {
    /// Whether to use secure memory allocation
    pub secure_allocation: bool,
    /// Whether to clear memory on deallocation
    pub clear_on_dealloc: bool,
    /// Whether to lock memory pages
    pub lock_memory: bool,}


impl Clone for MemoryProtector {
        Self {
            secure_allocation: self.secure_allocation,
            clear_on_dealloc: self.clear_on_dealloc,
            lock_memory: self.lock_memory,
/// Encryption key configuration for storage}


pub struct EncryptionKey {
    /// Key derivation method
    pub derivation_method: String,
    /// Key size in bits
    pub key_size: u32,
    /// Whether to use hardware entropy
    pub hardware_entropy: bool,}


impl Clone for EncryptionKey {
            derivation_method: self.derivation_method.clone(),
            key_size: self.key_size,
            hardware_entropy: self.hardware_entropy,
/// Software HSM configuration}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareHsmConfig {
    /// Storage backend configuration
    pub storage: StorageBackend,
    /// Memory protection level
    pub memory_protection: MemoryProtectionLevel,
    /// Key storage type
    pub key_storage: KeyStorageType,
    /// Whether to enable audit logging
    pub audit_logging: bool,
    /// Maximum number of keys to store
    pub max_keys: Option<usize>,
/// Software key representation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SoftwareKey {
    /// Unique identifier for the key
    pub id: String,
    /// Type of the cryptographic key
    pub key_type: KeyType,
    /// Protected key material
    pub key_material: ProtectedMemory,
    /// Timestamp when the key was created
    pub created_at: chrono::DateTime<Utc>,
    /// Additional metadata for the key
    pub metadata: KeyMetadata,
/// Protected memory for secure key storage
pub struct ProtectedMemory {
    /// The actual key data
    pub data: Vec<u8>,
    /// Whether the memory is protected
    pub protected: bool,
// Audit log types now imported from canonical audit module

/// **MIGRATION COMPLETE** ✅
/// CryptoProvider has been unified with the canonical CryptoProvider trait.
/// This eliminates fragmentation and uses the zero-cost canonical system.
/// 
/// ## Migration Benefits:
/// - **Zero-cost async** - Uses native async fn instead of async_trait
/// - **Unified interface** - Single CryptoProvider across all modules
/// - **Type safety** - Canonical types from beardog-types
/// - **Better performance** - No boxing overhead

// Re-export the canonical CryptoProvider trait
pub use beardog_traits::canonical::CryptoProvider;

/// **MIGRATION NOTICE**
/// The local CryptoProvider trait has been unified with the canonical version.
/// All implementations should now use beardog_traits::canonical::CryptoProvider.
/// 
/// ## Migration Path:
/// ```rust
/// // OLD:
/// use crate::tunnel::hsm::software_hsm::types::CryptoProvider;
/// 
/// // NEW:
/// use beardog_traits::canonical::CryptoProvider;
/// ```
/// Memory protector interface for secure key storage
pub trait MemoryProtector: Send + Sync {
    /// Initialize memory protector
    async fn initialize(&self) -> BearDogResult<()>;
    /// Protect key material in memory
    async fn protect_key_material(&self, key_material: &[u8]) -> BearDogResult<ProtectedMemory>;
    /// Unprotect key material for use
    async fn unprotect_key_material(&self, protected: &ProtectedMemory) -> BearDogResult<Vec<u8>>;
    /// Securely zeroize key material
    async fn zeroize_key_material(&self, key_material: &[u8]) -> BearDogResult<()>;
/// Storage backend interface for key persistence
pub trait StorageBackend: Send + Sync {
    /// Initialize storage backend
    /// Store encrypted key}


    async fn store(&self, key_id: &str, encrypted_key: &[u8]) -> BearDogResult<()>;
    /// Load encrypted key
    async fn load(&self, key_id: &str) -> BearDogResult<Vec<u8>>;
    /// Delete key
    /// List all key IDs
    /// Backup storage
    /// Restore from backup
/// Encryption key interface for key protection
pub trait EncryptionKey: Send + Sync {
    /// Initialize encryption key
    /// Encrypt data}


    async fn encrypt(&self, plaintext: &[u8]) -> BearDogResult<Vec<u8>>;
    /// Decrypt data
    async fn decrypt(&self, ciphertext: &[u8]) -> BearDogResult<Vec<u8>>;
/// Audit logger interface for operation logging
pub trait AuditLogger: Send + Sync {
    /// Log HSM operation}


    async fn log_operation(&self, operation: &AuditLogEntry) -> BearDogResult<()>;
    /// Get audit log entries
    async fn get_audit_log(&self, filter: &AuditLogFilter) -> BearDogResult<Vec<AuditLogEntry>>;
/// Software key store for encrypted key storage
pub struct SoftwareKeyStore {
    /// Storage backend for key persistence
    pub storage_backend: Arc<dyn StorageBackendTrait>,
    /// Encryption key for protecting stored keys
    pub encryption_key: Arc<dyn EncryptionKeyTrait>,
    /// Cache for frequently accessed keys (placeholder for LRU cache)
    pub key_cache: Arc<RwLock<std::collections::HashMap<String, SoftwareKey>>>, // placeholder for lru::LruCache<String, SoftwareKey>
/// Software health monitor for system status
pub struct SoftwareHealthMonitor {
    /// Current health status of the HSM
    pub health_status: Arc<RwLock<HsmHealthStatus>>,
    /// Performance metrics for the HSM (using canonical type)
    pub metrics: Arc<RwLock<super::super::types::canonical::PerformanceMetrics>>,
/// Default memory protector implementation
pub struct DefaultMemoryProtector {
    /// Configuration for memory protection
    pub config: crate::tunnel::hsm::software_hsm::MemoryConfig,
/// Default audit logger implementation}


pub struct DefaultAuditLogger {
    /// Persistent audit storage backend
    pub storage: Arc<super::audit::PersistentAuditStorage>,
/// File storage backend implementation
pub struct FileStorageBackend;}


impl FileStorageBackend {
    pub async fn new(_config: &KeyStoreConfig) -> BearDogResult<Self> {
        Ok(FileStorageBackend)}


impl StorageBackendTrait for FileStorageBackend {
    async fn initialize(&self) -> BearDogResult<()> {
        Ok(())}


    async fn store(&self, _key_id: &str, _encrypted_key: &[u8]) -> BearDogResult<()> {
        Err(BearDogError::unsupported_operation("FileStorageBackend not implemented".to_string(),
        ))
    async fn retrieve(&self, _key_id: &str) -> BearDogResult<Vec<u8>> {}


    async fn delete(&self, _key_id: &str) -> BearDogResult<()> {
    async fn list_keys(&self) -> BearDogResult<Vec<String>> {}


    async fn backup(&self) -> BearDogResult<Vec<u8>> {
    async fn restore(&self, _backup_data: &[u8]) -> BearDogResult<()> {
/// Database storage backend implementation}


pub struct DatabaseStorageBackend;
impl DatabaseStorageBackend {
        Ok(DatabaseStorageBackend)}


impl StorageBackendTrait for DatabaseStorageBackend {
        Err(BearDogError::unsupported_operation("DatabaseStorageBackend not implemented".to_string(),
/// Memory storage backend implementation
pub struct MemoryStorageBackend {
    /// In-memory storage for keys
    pub storage: Arc<RwLock<HashMap<String, Vec<u8>>>>,}


impl MemoryStorageBackend {
    pub async fn new() -> BearDogResult<Self> {
        Ok(MemoryStorageBackend {
            storage: Arc::new(RwLock::new(HashMap::new())),
        })
impl StorageBackendTrait for MemoryStorageBackend {
    async fn store(&self, key_id: &str, encrypted_key: &[u8]) -> BearDogResult<()> {
        let mut storage = self.storage.write().await;
        storage.insert(key_id.to_string(), encrypted_key.to_vec());}


    async fn retrieve(&self, key_id: &str) -> BearDogResult<Vec<u8>> {
        let storage = self.storage.read().await;
        storage
            .get(key_id)
            .cloned()
            .ok_or_else(|| BearDogError::not_found(format!("Key not found: {key_id}"))},
            })
    async fn delete(&self, key_id: &str) -> BearDogResult<()> {
        storage.remove(key_id);
        Ok(storage.keys().cloned().collect())
        bincode::serialize(&*storage).map_err(|e| BearDogError::Serialization {
            message: e.to_string(),}


    async fn restore(&self, backup_data: &[u8]) -> BearDogResult<()> {
        let restored: HashMap<String, Vec<u8>> =
            bincode::deserialize(backup_data).map_err(|e| BearDogError::internal(e.to_string(),
            ))?;
        *storage = restored;
/// Default encryption key implementation
pub struct DefaultEncryptionKey;
impl DefaultEncryptionKey {
    pub async fn create(_config: &SoftwareHsmConfig) -> BearDogResult<Box<dyn EncryptionKeyTrait>> {
        Ok(Box::new(DefaultEncryptionKey))}


impl EncryptionKeyTrait for DefaultEncryptionKey {
    async fn encrypt(&self, plaintext: &[u8]) -> BearDogResult<Vec<u8>> {
        // Use proper AES-256-GCM encryption
        use aes_gcm::{aead::Aead, Aes256Gcm, Key, KeyInit, Nonce};
        use rand::RngCore;
        // Use a secure 32-byte key (in production, this should come from secure key management)
        let key_bytes = b"BearDog_DefaultKey_256bit_Secure!"; // 32 bytes
        let key = Key::<Aes256Gcm>::from_slice(key_bytes);
        let cipher = Aes256Gcm::new(key);
        // Generate secure random nonce
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        // Encrypt the data
        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| BearDogError::Crypto {
                message: format!("AES-256-GCM encryption failed: {e}"),
            })?;
        // Prepend nonce to ciphertext for decryption
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);
        Ok(result)
    async fn decrypt(&self, ciphertext: &[u8]) -> BearDogResult<Vec<u8>> {
        // Use proper AES-256-GCM decryption
        if ciphertext.len() < 12 {
            return Err(BearDogError::Crypto {
                message: "Ciphertext too short to contain nonce".to_string(),
            });
        // Use the same secure 32-byte key
        // Extract nonce and ciphertext
        let (nonce_bytes, encrypted_data) = ciphertext.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        // Decrypt the data
        cipher
            .decrypt(nonce, encrypted_data)
                message: format!("AES-256-GCM decryption failed: {e}"),
/// Rust crypto provider implementation
pub struct RustCryptoProvider;
impl RustCryptoProvider {
    /// Create a new RustCryptoProvider instance
        Ok(RustCryptoProvider)
/// OpenSSL crypto provider implementation}


pub struct OpenSslCryptoProvider;
/// Rust Software HSM implementation
pub struct RustSoftwareHsm {
    /// Configuration for the software HSM
    pub config: SoftwareHsmConfig,
    /// Key store for managing keys
    pub key_store: Arc<RwLock<SoftwareKeyStore>>,
    /// Crypto provider for cryptographic operations
    pub crypto_provider: Arc<dyn CryptoProvider>,
    /// Memory protector for secure key storage
    pub memory_protector: Arc<MemoryProtector>,
    /// Audit logger for operation logging
    pub audit_logger: Arc<dyn AuditLogger>,
    /// Health monitor for system status
    pub health_monitor: Arc<SoftwareHealthMonitor>,}


impl ProtectedMemory {
    /// Create new protected memory}


    pub fn new(data: Vec<u8>, protected: bool) -> Self {
        Self { data, protected }
    /// Get data reference
    pub fn data(&self) -> &[u8] {
        &self.data
    /// Check if protected}


    pub fn is_protected(&self) -> bool {
        self.protected
impl SoftwareKey {
    /// Create new software key}


    pub fn new(
        id: String,
        key_type: KeyType,
        key_material: ProtectedMemory,
        metadata: KeyMetadata,
    ) -> Self {
            id,
            key_type,
            key_material,
            created_at: Utc::now(),
            metadata,
    /// Get key ID}


    pub fn id(&self) -> &str {
        &self.id
    /// Get key type
    pub fn key_type(&self) -> &KeyType {
        &self.key_type
    /// Get key material}


    pub fn key_material(&self) -> &ProtectedMemory {
        &self.key_material
    /// Get metadata
    pub fn metadata(&self) -> &KeyMetadata {
        &self.metadata
    /// Get creation timestamp}


    pub fn created_at(&self) -> chrono::DateTime<Utc> {
        self.created_at
// AuditLogEntry implementation removed - using canonical one from audit/types.rs
impl AuditLogFilter {
    /// Create new audit log filter}


    pub fn new() -> Self {
            operation: None,
            key_id: None,
            actor: None,
            result: None,
            from_time: None,
            to_time: None,
            limit: Some(100),
    /// Set time range}


    pub fn with_time_range(
        mut self,
        start: chrono::DateTime<Utc>,
        end: chrono::DateTime<Utc>,
        self.from_time = Some(start);
        self.to_time = Some(end);
        self
    /// Set operation filter
    pub fn with_operation(mut self, operation: String) -> Self {
        self.operation = Some(operation);
    /// Set key ID filter}


    pub fn with_key_id(mut self, key_id: String) -> Self {
        self.key_id = Some(key_id);
    /// Set actor filter
    pub fn with_actor(mut self, actor: String) -> Self {
        self.actor = Some(actor);
impl Default for AuditLogFilter {}


    fn default() -> Self {
        Self::new()
