

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

#[derive(Debug)]
pub enum StorageBackend {

    InMemory,

    File { path: String },

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
    async fn initialize(&self) -> Result<(), BearDogError>;
    async fn store(&self, key_id: &str, key_data: &[u8]) -> Result<(), BearDogError>;
    async fn retrieve(&self, key_id: &str) -> Result<Vec<u8>, BearDogError>>;
    async fn delete(&self, key_id: &str) -> Result<(), BearDogError>;
    async fn list_keys(&self) -> Result<Vec<String>, BearDogError>>;
    async fn backup(&self) -> Result<Vec<u8>, BearDogError>>;
    async fn restore(&self, backup_data: &[u8]) -> Result<(), BearDogError>;

pub trait EncryptionKeyTrait: Send + Sync {}

    async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, BearDogError>>;
    async fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, BearDogError>>;

pub struct MemoryProtector {

    pub secure_allocation: bool,

    pub clear_on_dealloc: bool,

    pub lock_memory: bool,}

impl Clone for MemoryProtector {
        Self {
            secure_allocation: self.secure_allocation,
            clear_on_dealloc: self.clear_on_dealloc,
            lock_memory: self.lock_memory,

pub struct EncryptionKey {

    pub derivation_method: String,

    pub key_size: u32,

    pub hardware_entropy: bool,}

impl Clone for EncryptionKey {
            derivation_method: self.derivation_method.clone(),
            key_size: self.key_size,
            hardware_entropy: self.hardware_entropy,

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareHsmConfig {

    pub storage: StorageBackend,

    pub memory_protection: MemoryProtectionLevel,

    pub key_storage: KeyStorageType,

    pub audit_logging: bool,

    pub max_keys: Option<usize>,

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SoftwareKey {

    pub id: String,

    pub key_type: KeyType,

    pub key_material: ProtectedMemory,

    pub created_at: chrono::DateTime<Utc>,

    pub metadata: KeyMetadata,

pub struct ProtectedMemory {

    pub data: Vec<u8>,

    pub protected: bool,

pub use beardog_traits::canonical::CryptoProvider;

pub trait MemoryProtector: Send + Sync {

    async fn initialize(&self) -> Result<(), BearDogError>;

    async fn protect_key_material(&self, key_material: &[u8]) -> Result<ProtectedMemory, BearDogError>;

    async fn unprotect_key_material(&self, protected: &ProtectedMemory) -> Result<Vec<u8>, BearDogError>>;

    async fn zeroize_key_material(&self, key_material: &[u8]) -> Result<(), BearDogError>;

pub trait StorageBackend: Send + Sync {

    async fn store(&self, key_id: &str, encrypted_key: &[u8]) -> Result<(), BearDogError>;

    async fn load(&self, key_id: &str) -> Result<Vec<u8>, BearDogError>>;

pub trait EncryptionKey: Send + Sync {

    async fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, BearDogError>>;

    async fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError>>;

pub trait AuditLogger: Send + Sync {

    async fn log_operation(&self, operation: &AuditLogEntry) -> Result<(), BearDogError>;

    async fn get_audit_log(&self, filter: &AuditLogFilter) -> Result<Vec<AuditLogEntry>, BearDogError>>;

pub struct SoftwareKeyStore {

    pub storage_backend: impl StorageBackendTrait,

    pub encryption_key: impl EncryptionKeyTrait,

    pub key_cache: Arc<RwLock<std::collections::HashMap<String, SoftwareKey>>>, // placeholder for lru::LruCache<String, SoftwareKey>

pub struct SoftwareHealthMonitor {

    pub health_status: Arc<RwLock<HsmHealthStatus>>,

    pub metrics: Arc<RwLock<super::super::types::canonical::PerformanceMetrics>>,

pub struct DefaultMemoryProtector {

    pub config: crate::tunnel::hsm::software_hsm::MemoryConfig,

pub struct DefaultAuditLogger {

    pub storage: Arc<super::audit::PersistentAuditStorage>,

pub struct FileStorageBackend;}

impl FileStorageBackend {
    pub async fn new(_config: &KeyStoreConfig) -> Result<Self, BearDogError> {
        Ok(FileStorageBackend)}

impl StorageBackendTrait for FileStorageBackend {
    async fn initialize(&self) -> Result<(), BearDogError> {
        Ok(())}

    async fn store(&self, _key_id: &str, _encrypted_key: &[u8]) -> Result<(), BearDogError> {
        Err(BearDogError::unsupported_operation("FileStorageBackend not implemented".to_string(),
        ))
    async fn retrieve(&self, _key_id: &str) -> Result<Vec<u8>, BearDogError>> {}

    async fn delete(&self, _key_id: &str) -> Result<(), BearDogError> {
    async fn list_keys(&self) -> Result<Vec<String>, BearDogError>> {}

    async fn backup(&self) -> Result<Vec<u8>, BearDogError>> {
    async fn restore(&self, _backup_data: &[u8]) -> Result<(), BearDogError> {

pub struct DatabaseStorageBackend;
impl DatabaseStorageBackend {
        Ok(DatabaseStorageBackend)}

impl StorageBackendTrait for DatabaseStorageBackend {
        Err(BearDogError::unsupported_operation("DatabaseStorageBackend not implemented".to_string(),

pub struct MemoryStorageBackend {

    pub storage: Arc<RwLock<HashMap<String, Vec<u8>>>>,}

impl MemoryStorageBackend {
    pub async fn new() -> Result<Self, BearDogError> {
        Ok(MemoryStorageBackend {
            storage: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        })
impl StorageBackendTrait for MemoryStorageBackend {
    async fn store(&self, key_id: &str, encrypted_key: &[u8]) -> Result<(), BearDogError> {
        let mut storage = self.storage.write().await;
        storage.insert(key_id.to_string(), encrypted_key.to_vec());}

    async fn retrieve(&self, key_id: &str) -> Result<Vec<u8>, BearDogError>> {
        let storage = self.storage.read().await;
        storage
            .get(key_id)
            .cloned()
            .ok_or_else(|| BearDogError::not_found(format!("Key not found: {key_id}"))},
            })
    async fn delete(&self, key_id: &str) -> Result<(), BearDogError> {
        storage.remove(key_id);
        Ok(storage.keys().cloned().collect())
        bincode::serialize(&*storage).map_err(|e| BearDogError::Serialization {
            message: e.to_string(),}

    async fn restore(&self, backup_data: &[u8]) -> Result<(), BearDogError> {
        let restored: HashMap<String, Vec<u8>> =
            bincode::deserialize(backup_data).map_err(|e| BearDogError::internal(e.to_string(),
            ))?;
        *storage = restored;

pub struct DefaultEncryptionKey;
impl DefaultEncryptionKey {
    pub async fn create(_config: &SoftwareHsmConfig) -> Result<Box<dyn EncryptionKeyTrait, BearDogError>> {
        Ok(DefaultEncryptionKey)}

impl EncryptionKeyTrait for DefaultEncryptionKey {
    async fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, BearDogError>> {

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
    async fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError>> {

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

        Ok(RustCryptoProvider)

pub struct OpenSslCryptoProvider;

pub struct RustSoftwareHsm {

    pub config: SoftwareHsmConfig,

    pub key_store: Arc<RwLock<SoftwareKeyStore>>,

    pub crypto_provider: impl CryptoProvider + Send + Sync + 'static,

    pub memory_protector: Arc<MemoryProtector>,

    pub audit_logger: impl AuditLogger,

    pub health_monitor: Arc<SoftwareHealthMonitor>,}

impl ProtectedMemory {

    pub fn new(data: Vec<u8>, protected: bool) -> Self {
        Self { data, protected }

    pub fn data(&self) -> &[u8] {
        &self.data

    pub fn is_protected(&self) -> bool {
        self.protected
impl SoftwareKey {

    pub fn new(
        id: &str,
        key_type: KeyType,
        key_material: ProtectedMemory,
        metadata: KeyMetadata,
    ) -> Self {
            id,
            key_type,
            key_material,
            created_at: Utc::now(),
            metadata,

    pub fn id(&self) -> &str {
        &self.id

    pub fn key_type(&self) -> &KeyType {
        &self.key_type

    pub fn key_material(&self) -> &ProtectedMemory {
        &self.key_material

    pub fn metadata(&self) -> &KeyMetadata {
        &self.metadata

    pub fn created_at(&self) -> chrono::DateTime<Utc> {
        self.created_at

impl AuditLogFilter {

    pub fn new() -> Self {
            operation: None,
            key_id: None,
            actor: None,
            result: None,
            from_time: None,
            to_time: None,
            limit: Some(100),

    pub fn with_time_range(
        mut self,
        start: chrono::DateTime<Utc>,
        end: chrono::DateTime<Utc>,
        self.from_time = Some(start);
        self.to_time = Some(end);
        self

    pub fn with_operation(mut self, operation: &str) -> Self {
        self.operation = Some(operation);

    pub fn with_key_id(mut self, key_id: &str) -> Self {
        self.key_id = Some(key_id);

    pub fn with_actor(mut self, actor: &str) -> Self {
        self.actor = Some(actor);
impl Default for AuditLogFilter {}

    fn default() -> Self {
        Self::new()
