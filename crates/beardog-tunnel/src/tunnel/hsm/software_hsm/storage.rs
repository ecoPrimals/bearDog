

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use beardog_errors::{BearDogError, BearDogResult};

#[derive(Clone)]
pub struct MemoryHsmStorage {

    storage: Arc<RwLock<HashMap<String, StoredKey>>>,

    master_key: Arc<Key<Aes256Gcm>>,

    stats: Arc<RwLock<StorageStats>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoredKey {

    pub key_id: String,

    pub encrypted_data: Vec<u8>,

    pub nonce: Vec<u8>,

    pub metadata: KeyMetadata,

    pub created_at: DateTime<Utc>,

    pub last_accessed: DateTime<Utc>,

#[derive(Clone, Debug, Default)]
pub struct StorageStats {

    pub total_keys: usize,

    pub total_size: usize,

    pub operations_count: u64,

    pub last_operation: Option<DateTime<Utc>>,

pub trait EncryptionKey: Send + Sync {

    fn initialize(&self) -> impl std::future::Future<Output = BearDogResult<()>> + Send;

    fn encrypt(
        &self,
        plaintext: &[u8],
    ) -> impl std::future::Future<Output = BearDogResult<Vec<u8>>> + Send;

    fn decrypt(
        ciphertext: &[u8],

pub struct DefaultEncryptionKey {
    cipher: Aes256Gcm,}

impl MemoryHsmStorage {

    pub fn new() -> BearDogResult<Self> {

        let master_key_bytes = rand::random::<[u8; 32]>();
        let master_key = Key::<Aes256Gcm>::from_slice(&master_key_bytes);
        info!("Initialized memory HSM storage with secure master key");
        Ok(Self {
            storage: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            master_key: Arc::new(*master_key),
            stats: Arc::new(RwLock::new(StorageStats::default())),
        })
    }

    pub async fn store_key(
        key_id: &str,
        key_data: &[u8],
        metadata: KeyMetadata,
    ) -> BearDogResult<()> {
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
            let mut storage = self.storage.write().await;
            storage.insert(key_id.clone(), stored_key);
        }

            let mut stats = self.stats.write().await;
            stats.total_keys += 1;
            stats.total_size += encrypted_data_len;
            stats.operations_count += 1;
            stats.last_operation = Some(Utc::now());
        info!("Successfully stored key: {}", key_id);
        Ok(())

    pub async fn retrieve_key(&self, key_id: &str) -> BearDogResult<Vec<u8>> {
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

    pub async fn delete_key(&self, key_id: &str) -> BearDogResult<()> {
        debug!("Deleting key from memory HSM: {}", key_id);
        let removed_key = {
            storage
                .remove(key_id)
                })?
            stats.total_keys -= 1;
            stats.total_size -= removed_key.encrypted_data.len();
        info!("Successfully deleted key: {}", key_id);

    pub async fn list_keys(&self) -> BearDogResult<Vec<String>> {
        let storage = self.storage.read().await;
        let keys: Vec<String> = storage.keys().cloned().collect();
        debug!("Listed {} keys from memory HSM", keys.len());
        Ok(keys)

    pub async fn get_key_metadata(&self, key_id: &str) -> BearDogResult<KeyMetadata> {
        let stored_key = storage.get(key_id).ok_or_else(|| BearDogError::not_found(format!("Key not found: {key_id}"))},
        })?;
        Ok(stored_key.metadata.clone())

    pub async fn get_statistics(&self) -> BearDogResult<StorageStats> {
        let stats = self.stats.read().await;
        Ok(stats.clone())

    pub async fn clear_storage(&self) -> BearDogResult<()> {
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

    async fn initialize(&self) -> BearDogResult<()> {
        info!("Initializing default encryption key");

    async fn encrypt(&self, plaintext: &[u8]) -> BearDogResult<Vec<u8>> {
        debug!("Encrypting {} bytes with AES-256-GCM", plaintext.len());
        let ciphertext =
            self.cipher
                .encrypt(&nonce, plaintext)
                    operation: format!("Failed to encrypt data: {e}"),

        let mut encrypted_data = nonce.to_vec();
        encrypted_data.extend_from_slice(&ciphertext);
        debug!(
            "Successfully encrypted data, output size: {} bytes",
            encrypted_data.len()
        );
        Ok(encrypted_data)

    async fn decrypt(&self, ciphertext: &[u8]) -> BearDogResult<Vec<u8>> {
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

        let master_key = Arc::new(*Key::<Aes256Gcm>::from_slice(&master_key_bytes));
        Self {
            master_key,
            stats: Arc::new(RwLock::new(StorageStats {
                total_keys: 0,
                total_size: 0,
                operations_count: 0,
                last_operation: None,
            })),
impl Default for DefaultEncryptionKey {

        Self { cipher }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MemoryStorageStatistics {

    pub key_count: usize,

    pub average_key_size: usize,

pub fn create_storage_backend() -> BearDogResult<MemoryHsmStorage> {
    MemoryHsmStorage::new()
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]}

    async fn test_memory_storage_operations() -> beardog_errors::BearDogResult<()> {
        let storage = MemoryHsmStorage::new().map_err(|e| {
            tracing::error!("Operation failed: {e:?}");
            beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        let key_data = b"test_key_data_123";
        let metadata = KeyMetadata {
            key_type: "aes256".to_string(),
            usage: "encryption".to_string(),
            key_size: 256,
            exportable: true,
            attributes: HashMap::with_capacity(16),

        storage
            .store_key("test_key".to_string(), key_data, metadata)
            .await
            .map_err(|e| {
                tracing::error!("Operation failed: {e:?}");
                beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))

        let retrieved_data = storage.retrieve_key("test_key").await.map_err(|e| {
        assert_eq!(retrieved_data, key_data);

        let keys = storage.list_keys().await.map_err(|e| {
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0], "test_key");

        storage.delete_key("test_key").await.map_err(|e| {

        let result = storage.retrieve_key("test_key").await;
        assert!(result.is_err());
    async fn test_encryption_key() -> beardog_errors::BearDogResult<()> {
        let key = DefaultEncryptionKey::new().map_err(|e| {
        key.initialize().await.map_err(|e| {
        let plaintext = b"Hello, World!";
        let ciphertext = key.encrypt(plaintext).await.map_err(|e| {
        let decrypted = key.decrypt(&ciphertext).await.map_err(|e| {
        assert_eq!(decrypted, plaintext);}

    async fn test_storage_statistics() -> beardog_errors::BearDogResult<()> {
        let stats = storage.get_statistics().await.map_err(|e| {
        assert_eq!(stats.total_keys, 0);
        assert_eq!(stats.total_size, 0);
        let key_data = b"test_key_data";
        assert_eq!(stats.total_keys, 1);
        assert!(stats.total_size > 0);
        assert!(stats.operations_count > 0);
