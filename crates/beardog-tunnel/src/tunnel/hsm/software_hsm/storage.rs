

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use beardog_errors::BearDogError;

pub struct MemoryHsmStorage {

    storage: Arc<RwLock<HashMap<String, StoredKey>>>,

    master_key: Arc<Key<Aes256Gcm>>,

    stats: Arc<RwLock<StorageStats>>,
}

#[derive(Debug, Clone)]
    /// Collection of encrypted data
    pub encrypted_data: Vec<u8>,

    /// Collection of nonce
    pub nonce: Vec<u8>,

    /// The metadata value
    pub metadata: KeyMetadata,

    /// The created at value
    pub created_at: DateTime<Utc>,

    /// The last accessed value
    pub last_accessed: DateTime<Utc>,

#[derive(Debug, Clone)]
    /// Number of total_size
    pub total_size: usize,

    /// Number of operations
    pub operations_count: u64,

    /// Optional last operation
    pub last_operation: Option<DateTime<Utc>>,

pub trait EncryptionKey: Send + Sync {

    /// Initializes componentialize
    fn initialize(&self) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;


    fn encrypt(&[u8],
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>>;


    fn decrypt(&[u8],

pub struct DefaultEncryptionKey {
    cipher: Aes256Gcm,}

impl MemoryHsmStorage {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new() -> Result<Self, BearDogError> {

        let master_key_bytes = rand::random::<[u8; 32]>();
        let master_key = Key::<Aes256Gcm>::from_slice(&master_key_bytes);
        info!("Initialized memory HSM storage with secure master key");
        Ok(Self {
            storage: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            master_key: Arc::new(*master_key),
            stats: Arc::new(RwLock::new(StorageStats::default(&str,
        key_data: &[u8],
        metadata: KeyMetadata,
    ) -> Result<(), BearDogError> {
        debug!("Storing key in memory HSM: {}", key_id);

        let cipher = Aes256Gcm::new(&self.master_key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let encrypted_data =
            cipher
                .encrypt(&nonce, key_data)
                .map_err(|e| BearDogError::Cryptographic {
                    operation: format!("Failed to encrypt key data: {e}"),
                })?;
        let encrypted_data_len = encrypted_data.len();
        let stored_key = StoredKey {
            key_id: key_id.clone(),
            encrypted_data,
            nonce: nonce.to_vec(),
            metadata,
            created_at: Utc::now(),
            last_accessed: Utc::now(),
        };

        {
            let mut storage = self.storage.write();
            storage.insert(key_id.clone(), stored_key);
        }

            let mut stats = self.stats.write();
            stats.total_keys += 1;
            stats.total_size += encrypted_data_len;
            stats.operations_count += 1;
            stats.last_operation = Some(Utc::now({}", key_id);
        Ok(())

/// Retrieve Key operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn retrieve_key(&self, key_id: &str) -> Result<Vec<u8>, BearDogError>> {
        debug!("Retrieving key from memory HSM: {}", key_id);
        let stored_key = {
            let stored_key = storage
                .get_mut(key_id)
                .ok_or_else(|| BearDogError::not_found(format!("Key not found: {key_id}"))},

            stored_key.last_accessed = Utc::now();
            stored_key.clone()

        let nonce = Nonce::from_slice(&stored_key.nonce);
        let decrypted_data = cipher
            .decrypt(nonce, stored_key.encrypted_data.as_ref())
            .map_err(|e| BearDogError::Cryptographic {
                operation: format!("Failed to decrypt key data: {e}"),
            })?;
        debug!("Successfully retrieved key: {}", key_id);
        Ok(decrypted_data)

/// Delete Key operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Removes key
    /// Removes key
    pub fn delete_key(&self, key_id: &str) -> Result<(), BearDogError> {
        debug!("Deleting key from memory HSM: {}", key_id);
        let removed_key = {
            storage
                .remove({}", key_id);

/// List Keys operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn list_keys(&self) -> Result<Vec<String>, BearDogError>> {
        let storage = self.storage.read();
        let keys: Vec<String> = storage.keys().cloned().collect();
        debug!("Listed {} keys from memory HSM", keys.len());
        Ok(keys)

/// Get Key Metadata operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets key_metadata
    /// Gets key_metadata
    pub fn get_key_metadata(&self, key_id: &str) -> Result<KeyMetadata, BearDogError> {
        let stored_key = storage.get(key_id).ok_or_else(|| BearDogError::not_found(format!("Key not found: {key_id}"))},
        })?;
        Ok(stored_key.metadata)

/// Get Statistics operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets statistics
    /// Gets statistics
    pub fn get_statistics(&self) -> Result<StorageStats, BearDogError> {
        let stats = self.stats.read();
        Ok(stats.clone())

/// Clear Storage operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn clear_storage(&self) -> Result<(), BearDogError> {
        warn!("Clearing all keys from memory HSM storage");
            storage.clear();

            stats.total_keys = 0;
            stats.total_size = 0;
        info!("Memory HSM storage cleared");
impl DefaultEncryptionKey {

        let key_bytes = rand::random::<[u8; 32]>();
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        Ok(Self { cipher })
impl EncryptionKey for DefaultEncryptionKey {

    /// Initializes componentialize
    fn initialize(&self) -> Result<(), BearDogError> {
        info!("Initializing default encryption key");


    fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        debug!("Encrypting {} bytes with AES-256-GCM", plaintext.len());
        let ciphertext =
            self.cipher
                .encrypt(&nonce, plaintext)
                    operation: format!("Failed to encrypt data: {e}"),

        let mut encrypted_data = nonce.to_vec({} bytes",
            encrypted_data.len()
        );
        Ok(encrypted_data)


    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        debug!("Decrypting {} bytes with AES-256-GCM", ciphertext.len());
        if ciphertext.len() < 12 {
            return Err(BearDogError::Cryptographic {
                operation: "Ciphertext too short to contain nonce".to_string(),
            });

        let (nonce_bytes, encrypted_data) = ciphertext.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        let plaintext = self.cipher.decrypt(nonce, encrypted_data).map_err(|e| {
            BearDogError::Cryptographic {
                operation: format!("Failed to decrypt data: {e}"),
            }
            "Successfully decrypted data, output size: {} bytes",
            plaintext.len()
        Ok(plaintext)
impl Default for MemoryHsmStorage {}

    fn default() -> Self {

        let master_key = Arc::new(*Key::<Aes256Gcm>::from_slice(Arc::new(RwLock::new(StorageStats {
                total_keys: 0,
                total_size: 0,
                operations_count: 0,
                last_operation: None,
            })),
impl Default for DefaultEncryptionKey {

        Self { cipher }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MemoryStorageStatistics {

    /// Number of key
    pub key_count: usize,

    /// Number of average_key_size
    pub average_key_size: usize,

/// Create Storage Backend operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates storage_backend
    /// Creates storage_backend
    pub fn create_storage_backend() -> Result<MemoryHsmStorage, BearDogError> {
    MemoryHsmStorage::new()
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]}


    fn test_memory_storage_operations() -> Result<(), BearDogError> {
        let storage = MemoryHsmStorage::new().map_err(|e| {
            tracing::error!("Operation failed: {e:?}");
            beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        let key_data = b"test_key_data_123";
        let metadata = KeyMetadata {
            key_type: "aes256".to_string(),
            usage: "encryption ".to_string(),
            exportable: true,
            attributes: HashMap::with_capacity(16),

        storage
            .store_key("test_key".to_string(), key_data, metadata)
            .map_err(|e| {
                tracing::error!("Operation failed: {e:?}");
                beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))

        let retrieved_data = storage.retrieve_key("test_key").map_err(|e| {
        assert_eq!(retrieved_data, key_data);

        let keys = storage.list_keys().map_err(|e| {
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0], "test_key");

        storage.delete_key("test_key").map_err(|e| {

        let result = storage.retrieve_key("test_key");
        assert!(result.is_err());
    fn test_encryption_key() -> Result<(), BearDogError> {
        let key = DefaultEncryptionKey::new().map_err(|e| {
        key.initialize().map_err(|e| {
        let plaintext = b"Hello, World!";
        let ciphertext = key.encrypt(plaintext).map_err(|e| {
        let decrypted = key.decrypt(&ciphertext).map_err(|e| {
        assert_eq!(decrypted, plaintext);}


    fn test_storage_statistics() -> Result<(), BearDogError> {
        let stats = storage.get_statistics().map_err(|e| {
        assert_eq!(stats.total_keys, 0);
        assert_eq!(stats.total_size, 0);
        let key_data = b"test_key_data";
        assert_eq!(stats.total_keys, 1);
        assert!(stats.total_size > 0);
        assert!(stats.operations_count > 0);
