

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use crate::tunnel::hsm::software_hsm::{KeyMetadata, KeyStoreConfig};
use crate::tunnel::hsm::types::{
    HsmHealthStatus, KeyStorageType, KeyType, MemoryProtectionLevel, PerformanceMetrics,
    SecurityLevel,
};

use beardog_errors::BearDogError;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub use super::audit::types::{AuditLogEntry, AuditLogFilter, OperationResult};

#[derive(Debug, Clone)]
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

pub trait StorageBackendTrait: Send + Sync {
    /// Initializes componentialize
    fn initialize(&str, key_data: &[u8]) -> Result<(), BearDogError>;
    fn retrieve(&self, key_id: &str) -> Result<Vec<u8>, BearDogError>>;
    /// Removes 
    fn delete(&self, key_id: &str) -> Result<(), BearDogError>;
    fn list_keys(&self) -> Result<Vec<String>, BearDogError>>;
    fn backup(&self) -> Result<Vec<u8>, BearDogError>>;
    fn restore(&self, backup_data: &[u8]) -> Result<(), BearDogError>;

pub trait EncryptionKeyTrait: Send + Sync {}


    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError>>;
    fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, BearDogError>>;

pub struct MemoryProtector {

    /// Whether secure_allocation is enabled
    pub secure_allocation: bool,

    /// Whether clear_on_dealloc is enabled
    pub clear_on_dealloc: bool,

    /// Whether lock_memory is enabled
    pub lock_memory: bool,}

impl Clone for MemoryProtector {
        Self {
            secure_allocation: self.secure_allocation,
            clear_on_dealloc: self.clear_on_dealloc,
            lock_memory: self.lock_memory,

pub struct EncryptionKey {

    /// The derivation method value
    pub derivation_method: String,

    /// Number of key_size
    pub key_size: u32,

    /// Whether hardware_entropy is enabled
    pub hardware_entropy: bool,}

impl Clone for EncryptionKey {
            derivation_method: &self.derivation_method,
            key_size: self.key_size,
            hardware_entropy: self.hardware_entropy,

#[derive(Debug, Clone)]
    /// The memory protection value
    pub memory_protection: MemoryProtectionLevel,

    /// The key storage value
    pub key_storage: KeyStorageType,

    /// Whether audit_logging is enabled
    pub audit_logging: bool,

    /// Optional max keys
    pub max_keys: Option<usize>,

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SoftwareKey {


    pub id: String,

    /// The key type value
    pub key_type: KeyType,

    /// The key material value
    pub key_material: ProtectedMemory,

    /// The created at value
    pub created_at: chrono::DateTime<Utc>,

    /// The metadata value
    pub metadata: KeyMetadata,

pub struct ProtectedMemory {

    /// Collection of data
    pub data: Vec<u8>,

    /// Whether protected is enabled
    pub protected: bool,

pub use beardog_traits::canonical::CryptoProvider;

pub trait MemoryProtector: Send + Sync {

    /// Initializes componentialize
    fn initialize(&self) -> Result<(), BearDogError>;


    fn protect_key_material(&self, key_material: &[u8]) -> Result<ProtectedMemory, BearDogError>;


    fn unprotect_key_material(&self, protected: &ProtectedMemory) -> Result<Vec<u8>, BearDogError>>;


    fn zeroize_key_material(&self, key_material: &[u8]) -> Result<(), BearDogError>;

pub trait StorageBackend: Send + Sync {


    fn store(&str, encrypted_key: &[u8]) -> Result<(), BearDogError>;

    /// Loads data
    fn load(&self, key_id: &str) -> Result<Vec<u8>, BearDogError>>;

pub trait EncryptionKey: Send + Sync {


    fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, BearDogError>>;


    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError>>;

pub trait AuditLogger: Send + Sync {


    fn log_operation(&self, operation: &AuditLogEntry) -> Result<(), BearDogError>;

    /// Gets audit_log
    fn get_audit_log(&self, filter: &AuditLogFilter) -> Result<Vec<AuditLogEntry>, BearDogError>>;

pub struct SoftwareKeyStore {

    /// Number of storage_backend
    pub storage_backend: impl StorageBackendTrait,

    /// Number of encryption_key
    pub encryption_key: impl EncryptionKeyTrait,

    /// The key cache value
    pub key_cache: Arc<RwLock<std::collections::HashMap<String, SoftwareKey>>>, // placeholder for lru::LruCache<String, SoftwareKey>

pub struct SoftwareHealthMonitor {

    /// Current status of the health
    pub health_status: Arc<RwLock<HsmHealthStatus>>,

    /// The metrics value
    pub metrics: Arc<RwLock<super::super::types::canonical::PerformanceMetrics>>,

pub struct DefaultMemoryProtector {


    pub config: crate::tunnel::hsm::software_hsm::MemoryConfig,

pub struct DefaultAuditLogger {

    /// The storage value
    pub storage: Arc<super::audit::PersistentAuditStorage>,

pub struct FileStorageBackend;}

impl FileStorageBackend {
/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub async fn new(_config: &KeyStoreConfig) -> Result<Self, BearDogError> {
        Ok(&str, _encrypted_key: &[u8]) -> Result<(), BearDogError> {
        Err(BearDogError::unsupported_operation("FileStorageBackend not implemented"))
    fn retrieve(&self, _key_id: &str) -> Result<Vec<u8>, BearDogError>> {}

    /// Removes 
    fn delete(&self, _key_id: &str) -> Result<(), BearDogError> {
    fn list_keys(&self) -> Result<Vec<String>, BearDogError>> {}


    fn backup(&self) -> Result<Vec<u8>, BearDogError>> {
    fn restore(&self, _backup_data: &[u8]) -> Result<(), BearDogError> {

pub struct DatabaseStorageBackend;
impl DatabaseStorageBackend {
        Ok(DatabaseStorageBackend)}

impl StorageBackendTrait for DatabaseStorageBackend {
        Err(BearDogError::unsupported_operation(Arc<RwLock<HashMap<String, Vec<u8>>>>,}

impl MemoryStorageBackend {
/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub async fn new() -> Result<Self, BearDogError> {
        Ok(MemoryStorageBackend {
            storage: Arc::new(RwLock::new(HashMap::with_capacity(&str, encrypted_key: &[u8]) -> Result<(), BearDogError> {
        let mut storage = self.storage.write();
        storage.insert(key_id.to_string(), encrypted_key.to_vec());}


    fn retrieve(&self, key_id: &str) -> Result<Vec<u8>, BearDogError>> {
        let storage = self.storage.read();
        storage
            .get(key_id)
            .cloned()
            .ok_or_else(|| BearDogError::not_found(format!("Key not found: {key_id}"))},
            })
    /// Removes 
    fn delete(&self, key_id: &str) -> Result<(), BearDogError> {
        storage.remove(key_id);
        Ok(storage.keys().cloned().collect())
        bincode::serialize(&*storage).map_err(|e| BearDogError::Serialization {
            message: e.to_string(),}


    fn restore(&self, backup_data: &[u8]) -> Result<(), BearDogError> {
        let restored: HashMap<String, Vec<u8>> =
            bincode::deserialize(backup_data).map_err(|e| BearDogError::internal(e))?;
        *storage = restored;

pub struct DefaultEncryptionKey;
impl DefaultEncryptionKey {
/// Create operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates item
    /// Creates item
    pub fn create(_config: &SoftwareHsmConfig) -> Result<Box<dyn EncryptionKeyTrait, BearDogError>> {
        Ok(DefaultEncryptionKey)}

impl EncryptionKeyTrait for DefaultEncryptionKey {
    fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, BearDogError>> {

        use aes_gcm::{aead::Aead, Aes256Gcm, Key, KeyInit, Nonce};
        use rand::RngCore;

        let key_bytes = b"BearDog_DefaultKey_256bit_Secure!"; // 32 bytes
        let key = Key::<Aes256Gcm>::from_slice(key_bytes);
        let cipher = Aes256Gcm::new(key);

        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| BearDogError::Crypto {
                message: format!("AES-256-GCM encryption failed: {e}"),
            })?;

        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);
        Ok(result)
    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError>> {

        if ciphertext.len() < 12 {
            return Err(BearDogError::Crypto {
                message: "Ciphertext too short to contain nonce".to_string(),
            });

        let (nonce_bytes, encrypted_data) = ciphertext.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        cipher
            .decrypt(nonce, encrypted_data)
                message: format!("AES-256-GCM decryption failed: {e}"),

pub struct RustCryptoProvider;
impl RustCryptoProvider {

        Ok(SoftwareHsmConfig,

    /// The key store value
    pub key_store: Arc<RwLock<SoftwareKeyStore>>,


    pub crypto_provider: impl CryptoProvider + Send + Sync + 'static,

    /// The memory protector value
    pub memory_protector: Arc<MemoryProtector>,

    /// Number of audit_logger
    pub audit_logger: impl AuditLogger,

    /// The health monitor value
    pub health_monitor: Arc<SoftwareHealthMonitor>,}

impl ProtectedMemory {

/// New operation.
    /// Creates a new instance
    pub fn new(Vec<u8>, protected: bool) -> Self {
        Self { data, protected }

/// Data operation.
    pub fn data(&str,
        key_type: KeyType,
        key_material: ProtectedMemory,
        metadata: KeyMetadata,
    ) -> Self {
            id: id.to_string(),
            key_type,
            key_material,
            created_at: Utc::now(),
            metadata,

/// Id operation.
    pub fn id(&self) -> &str {
        &self.id

/// Key Type operation.
    pub fn key_type(&self) -> &KeyType {
        &self.key_type

/// Key Material operation.
    pub fn key_material(&self) -> &ProtectedMemory {
        &self.key_material

/// Metadata operation.
    pub fn metadata(&self) -> &KeyMetadata {
        &self.metadata

/// Created At operation.
    /// Creates itemd_at
    /// Creates itemd_at
    pub fn created_at(&self) -> chrono::DateTime<Utc> {
        self.created_at

impl AuditLogFilter {

/// New operation.
    /// Creates a new instance
    pub fn new(None,
            key_id: None,
            actor: None,
            result: None,
            from_time: None,
            to_time: None,
            limit: Some(chrono::DateTime<Utc>,
        end: chrono::DateTime<Utc>,
        self.from_time = Some(start);
        self.to_time = Some(end);
        self

/// With Operation operation.
    /// Creates instance with operation
    pub fn with_operation(mut self, operation: &str) -> Self {
        self.operation = Some(operation);

/// With Key Id operation.
    /// Creates instance with key id
    pub fn with_key_id(mut self, key_id: &str) -> Self {
        self.key_id = Some(key_id);

/// With Actor operation.
    /// Creates instance with actor
    pub fn with_actor(mut self, actor: &str) -> Self {
        self.actor = Some(actor);
impl Default for AuditLogFilter {}

    fn default() -> Self {
        Self::new()
