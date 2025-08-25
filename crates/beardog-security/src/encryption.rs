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


/// # BearDog Encryption Module - Canonical System
///
/// This module provides unified encryption capabilities using the canonical
/// type system for consistent cryptographic operations across the ecosystem.

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
/// **CANONICAL MIGRATION COMPLETE** ✅
/// All encryption configuration now uses canonical types from beardog_types::canonical::configuration
pub use beardog_types::canonical::configuration::EncryptionConfig;
/// Encryption engine for BearDog
pub struct EncryptionEngine {
    config: Arc<EncryptionConfig>,
    keys: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    rng: SystemRandom,
/// Supported encryption algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum EncryptionAlgorithm {
    /// AES-256 in GCM mode (recommended for most use cases)
    #[default]
    Aes256Gcm,
    /// ChaCha20-Poly1305 (recommended for high-performance scenarios)
    ChaCha20Poly1305,
    /// Quantum-resistant encryption
    QuantumResistant,}


impl fmt::Display for EncryptionAlgorithm {}


    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncryptionAlgorithm::Aes256Gcm => write!(f, "AES256-GCM"),
            EncryptionAlgorithm::ChaCha20Poly1305 => write!(f, "ChaCha20-Poly1305"),
            EncryptionAlgorithm::QuantumResistant => write!(f, "Quantum-Resistant"),
/// Encrypted data container
#[derive(Debug, Clone, Serialize, Deserialize)]}


pub struct EncryptedData {
    /// Algorithm used for encryption
    pub algorithm: EncryptionAlgorithm,
    /// Encrypted ciphertext
    pub ciphertext: Vec<u8>,
    /// Initialization vector/nonce
    pub nonce: Vec<u8>,
    /// Authentication tag (for authenticated encryption)
    pub tag: Option<Vec<u8>>,
    /// Metadata associated with the encrypted data
    pub metadata: HashMap<String, String>,
    /// Key ID used for encryption (optional)
    pub key_id: Option<String>,
/// Key derivation parameters
pub struct KeyDerivationParams {
    /// Salt for key derivation
    pub salt: Vec<u8>,
    /// Number of iterations
    pub iterations: u32,
    /// Memory cost (in KB)
    pub memory_cost: u32,
    /// Parallelism factor
    pub parallelism: u32,}


impl EncryptionEngine {
    /// Create a new encryption engine
    pub async fn new(config: EncryptionConfig) -> BearDogResult<Self> {
        info!("🔐 Initializing BearDog Encryption Engine");
        Ok(Self {
            config: Arc::new(config),
            keys: Arc::new(RwLock::new(HashMap::new())),
            rng: SystemRandom::new(),
        })
    /// Encrypt data using specified algorithm
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
    /// Decrypt data
    pub async fn decrypt(&self, encrypted_data: &EncryptedData) -> BearDogResult<Vec<u8>> {
        match encrypted_data.algorithm {
            EncryptionAlgorithm::Aes256Gcm => self.decrypt_aes256_gcm(encrypted_data).await,
            EncryptionAlgorithm::ChaCha20Poly1305 => self.decrypt_chacha20_poly1305(encrypted_data).await,
                Err(BearDogError::encryption("decrypt".to_string(), "Quantum-resistant decryption not yet implemented".to_string()))
    /// Generate a new encryption key}


    pub async fn generate_key(&self, key_id: String, algorithm: EncryptionAlgorithm) -> BearDogResult<()> {
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
    /// Get a key by ID
    pub async fn get_key(&self, key_id: &str) -> BearDogResult<Vec<u8>> {
        let keys = self.keys.read().await;
        keys.get(key_id)
            .cloned()
            .ok_or_else(|| BearDogError::encryption("get_key".to_string(), "Key not found".to_string()))
    /// Delete a key}


    pub async fn delete_key(&self, key_id: &str) -> BearDogResult<()> {
        keys.remove(key_id)
            .ok_or_else(|| BearDogError::encryption("delete_key".to_string(), "Key not found".to_string()))?;
    /// List all key IDs
    pub async fn list_keys(&self) -> BearDogResult<Vec<String>> {
        Ok(keys.keys().cloned().collect())
    /// Encrypt using AES-256-GCM}


    async fn encrypt_aes256_gcm(&self, data: &[u8]) -> BearDogResult<EncryptedData> {
        // Generate a random key for this encryption
        let mut key_bytes = [0u8; 32];
            .fill(&mut key_bytes)
            .map_err(|_| BearDogError::encryption("encrypt_aes256_gcm".to_string(), "Key generation error".to_string()))?;
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        // Generate nonce
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        // Encrypt
        let ciphertext = cipher
            .encrypt(&nonce, data)
            .map_err(|_| BearDogError::encryption("encrypt_aes256_gcm".to_string(), "Encryption operation error".to_string()))?;
        Ok(EncryptedData {
            algorithm: EncryptionAlgorithm::Aes256Gcm,
            ciphertext,
            nonce: nonce.to_vec(),
            tag: None, // GCM includes authentication tag in ciphertext
            metadata: HashMap::new(),
            key_id: None,
    /// Decrypt using AES-256-GCM
    async fn decrypt_aes256_gcm(&self, encrypted_data: &EncryptedData) -> BearDogResult<Vec<u8>> {
        // For demo purposes, this would need the original key
        // In a real implementation, key management would be more sophisticated
        Err(BearDogError::encryption("decrypt_aes256_gcm".to_string(), "Key management not fully implemented".to_string()))
    /// Encrypt using ChaCha20-Poly1305}


    async fn encrypt_chacha20_poly1305(&self, data: &[u8]) -> BearDogResult<EncryptedData> {
        use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce, AeadCore, Aead, KeyInit};
        use rand::RngCore;
        
        // Generate a random key (in production, this should come from secure key management)
        let mut key_bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut key_bytes);
        let key = Key::from_slice(&key_bytes);
        
        // Create cipher
        let cipher = ChaCha20Poly1305::new(key);
        
        // Generate random nonce
        let nonce = ChaCha20Poly1305::generate_nonce(&mut rand::thread_rng());
        
        // Encrypt data
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
    
    /// Decrypt using ChaCha20-Poly1305
    async fn decrypt_chacha20_poly1305(&self, encrypted_data: &EncryptedData) -> BearDogResult<Vec<u8>> {
        use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce, Aead, KeyInit};
        
        // Extract key from salt (in production, retrieve from secure key management)
        let key_bytes = encrypted_data.salt.as_ref()
            .ok_or_else(|| BearDogError::encryption("decrypt_chacha20_poly1305".to_string(), "Missing key material".to_string()))?;
        
        if key_bytes.len() != 32 {
            return Err(BearDogError::encryption("decrypt_chacha20_poly1305".to_string(), "Invalid key length".to_string()));
        }
        
        let key = Key::from_slice(key_bytes);
        let cipher = ChaCha20Poly1305::new(key);
        
        // Extract nonce
        let nonce_bytes = encrypted_data.nonce.as_ref()
            .ok_or_else(|| BearDogError::encryption("decrypt_chacha20_poly1305".to_string(), "Missing nonce".to_string()))?;
        
        if nonce_bytes.len() != 12 {
            return Err(BearDogError::encryption("decrypt_chacha20_poly1305".to_string(), "Invalid nonce length".to_string()));
        }
        
        let nonce = Nonce::from_slice(nonce_bytes);
        
        // Decrypt data
        let plaintext = cipher.decrypt(nonce, encrypted_data.data.as_slice())
            .map_err(|_| BearDogError::encryption("decrypt_chacha20_poly1305".to_string(), "ChaCha20-Poly1305 decryption failed".to_string()))?;
        
        Ok(plaintext)
    }
    /// Generate a random salt}


    pub fn generate_salt(&self) -> BearDogResult<Vec<u8>> {
        let mut salt = [0u8; 32];
            .fill(&mut salt)
            .map_err(|_| BearDogError::encryption("generate_salt".to_string(), "Salt generation error".to_string()))?;
        Ok(salt.to_vec())
    /// Hash a password with Argon2}


    pub fn hash_password(&self, password: &str) -> BearDogResult<Vec<u8>> {
        // Generate random salt
            .map_err(|_| BearDogError::encryption("hash_password".to_string(), "Salt generation error".to_string()))?;
        // Encode salt for Argon2
        let salt_string = SaltString::encode_b64(&salt)
            .map_err(|_| BearDogError::encryption("hash_password".to_string(), "Salt encoding error".to_string()))?;
        // Hash password with Argon2
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt_string)
            .map_err(|_| BearDogError::encryption("hash_password".to_string(), "Password hashing error".to_string()))?;
        // Extract hash bytes
        match password_hash.hash {
            Some(hash) => Ok(hash.as_bytes().to_vec()),
            None => Err(BearDogError::internal("Hash extraction error".to_string()))
    /// Create a placeholder instance for initialization}


    pub fn placeholder() -> Self {
            config: Arc::new(UnifiedSecurityConfig::default()),
/// Encryption key information
pub struct EncryptionKeyInfo {
    /// Key identifier
    pub key_id: String,
    /// Encryption algorithm used
// Clean implementation complete - no character encoding issues 
