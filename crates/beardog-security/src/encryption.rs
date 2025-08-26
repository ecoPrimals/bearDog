

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use base64;
use chacha20poly1305;
use chrono::Utc;
use rand::rngs::OsRng;
use ring::rand::SystemRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;
use beardog_errors::{BearDogError, BearDogResult};

pub use beardog_types::canonical::configuration::EncryptionConfig;

pub struct EncryptionEngine {
    config: Arc<EncryptionConfig>,
    keys: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    rng: SystemRandom,

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum EncryptionAlgorithm {

    #[default]
    Aes256Gcm,

    ChaCha20Poly1305,

    QuantumResistant,}

impl fmt::Display for EncryptionAlgorithm {}

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncryptionAlgorithm::Aes256Gcm => write!(f, "AES256-GCM"),
            EncryptionAlgorithm::ChaCha20Poly1305 => write!(f, "ChaCha20-Poly1305"),
            EncryptionAlgorithm::QuantumResistant => write!(f, "Quantum-Resistant"),

#[derive(Debug, Clone, Serialize, Deserialize)]}

pub struct EncryptedData {

    pub algorithm: EncryptionAlgorithm,

    pub ciphertext: Vec<u8>,

    pub nonce: Vec<u8>,

    pub tag: Option<Vec<u8>>,

    pub metadata: HashMap<String, String>,

    pub key_id: Option<String>,

pub struct KeyDerivationParams {

    pub salt: Vec<u8>,

    pub iterations: u32,

    pub memory_cost: u32,

    pub parallelism: u32,}

impl EncryptionEngine {

    pub async fn new(config: EncryptionConfig) -> BearDogResult<Self> {
        info!("🔐 Initializing BearDog Encryption Engine");
        Ok(Self {
            config: Arc::new(config),
            keys: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            rng: SystemRandom::new(),
        })

    pub async fn encrypt(
        &self,
        data: &[u8],
        algorithm: EncryptionAlgorithm,
    ) -> BearDogResult<EncryptedData> {
        match algorithm {
            EncryptionAlgorithm::Aes256Gcm => self.encrypt_aes256_gcm(data).await,
            EncryptionAlgorithm::ChaCha20Poly1305 => self.encrypt_chacha20_poly1305(data).await,
            EncryptionAlgorithm::QuantumResistant => {
                Err(BearDogError::encryption("encrypt".to_string(), "Quantum-resistant encryption not yet implemented".to_string()))
            }

    pub async fn decrypt(&self, encrypted_data: &EncryptedData) -> BearDogResult<Vec<u8>> {
        match encrypted_data.algorithm {
            EncryptionAlgorithm::Aes256Gcm => self.decrypt_aes256_gcm(encrypted_data).await,
            EncryptionAlgorithm::ChaCha20Poly1305 => self.decrypt_chacha20_poly1305(encrypted_data).await,
                Err(BearDogError::encryption("decrypt".to_string(), "Quantum-resistant decryption not yet implemented".to_string()))

    pub async fn generate_key(&self, key_id: &str, algorithm: EncryptionAlgorithm) -> BearDogResult<()> {
        use ring::rand::SecureRandom;
        let key_length = match algorithm {
            EncryptionAlgorithm::Aes256Gcm => 32, // 256 bits
            EncryptionAlgorithm::ChaCha20Poly1305 => 32, // 256 bits
            EncryptionAlgorithm::QuantumResistant => 64, // 512 bits for future quantum resistance
        };
        let mut key = vec![0u8; key_length];
        self.rng
            .fill(&mut key)
            .map_err(|_| BearDogError::encryption("generate_key".to_string(), "Key generation error".to_string()))?;
        let mut keys = self.keys.write().await;
        keys.insert(key_id, key);
        Ok(())

    pub async fn get_key(&self, key_id: &str) -> BearDogResult<Vec<u8>> {
        let keys = self.keys.read().await;
        keys.get(key_id)
            .cloned()
            .ok_or_else(|| BearDogError::encryption("get_key".to_string(), "Key not found".to_string()))

    pub async fn delete_key(&self, key_id: &str) -> BearDogResult<()> {
        keys.remove(key_id)
            .ok_or_else(|| BearDogError::encryption("delete_key".to_string(), "Key not found".to_string()))?;

    pub async fn list_keys(&self) -> BearDogResult<Vec<String>> {
        Ok(keys.keys().cloned().collect())

    async fn encrypt_aes256_gcm(&self, data: &[u8]) -> BearDogResult<EncryptedData> {

        let mut key_bytes = [0u8; 32];
            .fill(&mut key_bytes)
            .map_err(|_| BearDogError::encryption("encrypt_aes256_gcm".to_string(), "Key generation error".to_string()))?;
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);

        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let ciphertext = cipher
            .encrypt(&nonce, data)
            .map_err(|_| BearDogError::encryption("encrypt_aes256_gcm".to_string(), "Encryption operation error".to_string()))?;
        Ok(EncryptedData {
            algorithm: EncryptionAlgorithm::Aes256Gcm,
            ciphertext,
            nonce: nonce.to_vec(),
            tag: None, // GCM includes authentication tag in ciphertext
            metadata: HashMap::with_capacity(16),
            key_id: None,

    async fn decrypt_aes256_gcm(&self, encrypted_data: &EncryptedData) -> BearDogResult<Vec<u8>> {

        Err(BearDogError::encryption("decrypt_aes256_gcm".to_string(), "Key management not fully implemented".to_string()))

    async fn encrypt_chacha20_poly1305(&self, data: &[u8]) -> BearDogResult<EncryptedData> {
        use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce, AeadCore, Aead, KeyInit};
        use rand::RngCore;

        let mut key_bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut key_bytes);
        let key = Key::from_slice(&key_bytes);

        let cipher = ChaCha20Poly1305::new(key);

        let nonce = ChaCha20Poly1305::generate_nonce(&mut rand::thread_rng());

        let ciphertext = cipher.encrypt(&nonce, data)
            .map_err(|_| BearDogError::encryption("encrypt_chacha20_poly1305".to_string(), "ChaCha20-Poly1305 encryption failed".to_string()))?;
        
        Ok(EncryptedData {
            data: ciphertext,
            key_id: "chacha20_key".to_string(),
            algorithm: "ChaCha20-Poly1305".to_string(),
            nonce: Some(nonce.to_vec()),
            salt: Some(key_bytes.to_vec()), // In production, store key securely
        })
    }

    async fn decrypt_chacha20_poly1305(&self, encrypted_data: &EncryptedData) -> BearDogResult<Vec<u8>> {
        use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce, Aead, KeyInit};

        let key_bytes = encrypted_data.salt.as_ref()
            .ok_or_else(|| BearDogError::encryption("decrypt_chacha20_poly1305".to_string(), "Missing key material".to_string()))?;
        
        if key_bytes.len() != 32 {
            return Err(BearDogError::encryption("decrypt_chacha20_poly1305".to_string(), "Invalid key length".to_string()));
        }
        
        let key = Key::from_slice(key_bytes);
        let cipher = ChaCha20Poly1305::new(key);

        let nonce_bytes = encrypted_data.nonce.as_ref()
            .ok_or_else(|| BearDogError::encryption("decrypt_chacha20_poly1305".to_string(), "Missing nonce".to_string()))?;
        
        if nonce_bytes.len() != 12 {
            return Err(BearDogError::encryption("decrypt_chacha20_poly1305".to_string(), "Invalid nonce length".to_string()));
        }
        
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = cipher.decrypt(nonce, encrypted_data.data.as_slice())
            .map_err(|_| BearDogError::encryption("decrypt_chacha20_poly1305".to_string(), "ChaCha20-Poly1305 decryption failed".to_string()))?;
        
        Ok(plaintext)
    }

    pub fn generate_salt(&self) -> BearDogResult<Vec<u8>> {
        let mut salt = [0u8; 32];
            .fill(&mut salt)
            .map_err(|_| BearDogError::encryption("generate_salt".to_string(), "Salt generation error".to_string()))?;
        Ok(salt.to_vec())

    pub fn hash_password(&self, password: &str) -> BearDogResult<Vec<u8>> {

            .map_err(|_| BearDogError::encryption("hash_password".to_string(), "Salt generation error".to_string()))?;

        let salt_string = SaltString::encode_b64(&salt)
            .map_err(|_| BearDogError::encryption("hash_password".to_string(), "Salt encoding error".to_string()))?;

        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt_string)
            .map_err(|_| BearDogError::encryption("hash_password".to_string(), "Password hashing error".to_string()))?;

        match password_hash.hash {
            Some(hash) => Ok(hash.as_bytes().to_vec()),
            None => Err(BearDogError::internal("Hash extraction error".to_string()))

    pub fn placeholder() -> Self {
            config: Arc::new(UnifiedSecurityConfig::default()),

pub struct EncryptionKeyInfo {

    pub key_id: String,

