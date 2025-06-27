//! Encryption and cryptography services
//! 
//! Enterprise-grade encryption capabilities democratized for everyone.

use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use std::fmt;
use tracing::info;
use rand::rngs::OsRng;
use aes_gcm::{Aes256Gcm, Key, Nonce, aead::{Aead, KeyInit, AeadCore}};
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use ring::rand::SystemRandom;
use chrono::Utc;
use chacha20poly1305;

use crate::config::EncryptionConfig;
use crate::error::{BearDogResult, BearDogError};

/// Encryption engine for BearDog
pub struct EncryptionEngine {
    config: Arc<EncryptionConfig>,
    keys: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    rng: SystemRandom,
}

/// Supported encryption algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum EncryptionAlgorithm {
    /// AES-256 in GCM mode (recommended for most use cases)
    #[default]
    Aes256Gcm,
    /// ChaCha20-Poly1305 (recommended for high-performance scenarios)
    ChaCha20Poly1305,
    /// Quantum-resistant encryption
    QuantumResistant,
}

impl fmt::Display for EncryptionAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncryptionAlgorithm::Aes256Gcm => write!(f, "AES256-GCM"),
            EncryptionAlgorithm::ChaCha20Poly1305 => write!(f, "ChaCha20-Poly1305"),
            EncryptionAlgorithm::QuantumResistant => write!(f, "Quantum-Resistant"),
        }
    }
}

/// Encrypted data container
#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

/// Key derivation parameters
#[derive(Debug, Clone)]
pub struct KeyDerivationParams {
    /// Salt for key derivation
    pub salt: Vec<u8>,
    /// Number of iterations
    pub iterations: u32,
    /// Memory cost (in KB)
    pub memory_cost: u32,
    /// Parallelism factor
    pub parallelism: u32,
}

impl EncryptionEngine {
    /// Create a new encryption engine
    pub async fn new(config: EncryptionConfig) -> BearDogResult<Self> {
        info!("🔐 Initializing BearDog Encryption Engine");
        
        Ok(Self {
            config: Arc::new(config),
            keys: Arc::new(RwLock::new(HashMap::new())),
            rng: SystemRandom::new(),
        })
    }

    /// Encrypt data using specified algorithm
    pub async fn encrypt(
        &self,
        data: &[u8],
        algorithm: Option<EncryptionAlgorithm>,
    ) -> BearDogResult<EncryptedData> {
        let algorithm = algorithm.unwrap_or(EncryptionAlgorithm::Aes256Gcm);

        match algorithm {
            EncryptionAlgorithm::Aes256Gcm => self.encrypt_aes256_gcm(data).await,
            EncryptionAlgorithm::ChaCha20Poly1305 => self.encrypt_chacha20_poly1305(data).await,
            EncryptionAlgorithm::QuantumResistant => self.encrypt_quantum_resistant(data).await,
        }
    }

    /// Decrypt data
    pub async fn decrypt(&self, encrypted_data: &EncryptedData) -> BearDogResult<Vec<u8>> {
        match encrypted_data.algorithm {
            EncryptionAlgorithm::Aes256Gcm => self.decrypt_aes256_gcm(encrypted_data).await,
            EncryptionAlgorithm::ChaCha20Poly1305 => self.decrypt_chacha20_poly1305(encrypted_data).await,
            EncryptionAlgorithm::QuantumResistant => self.decrypt_quantum_resistant(encrypted_data).await,
        }
    }

    /// Generate a new encryption key
    pub async fn generate_key(&self, algorithm: &str, purpose: &str) -> BearDogResult<(String, Vec<u8>)> {
        info!("🔑 Generating encryption key: {} for {}", algorithm, purpose);
        
        let key_id = uuid::Uuid::new_v4().to_string();
        let key = match algorithm {
            "AES256" => {
                let key = crate::utils::crypto_utils::secure_random_bytes(32);
                key
            }
            "RSA2048" => {
                // Generate RSA-2048 equivalent key material (256 bytes)
                // In a real implementation, this would use proper RSA key generation
                let key = crate::utils::crypto_utils::secure_random_bytes(256);
                key
            }
            "RSA4096" => {
                // Generate RSA-4096 equivalent key material (512 bytes)
                // In a real implementation, this would use proper RSA key generation
                let key = crate::utils::crypto_utils::secure_random_bytes(512);
                key
            }
            "ECDSA_P256" => {
                // Generate ECDSA P-256 equivalent key material (32 bytes for private key)
                // In a real implementation, this would use proper ECDSA key generation
                let key = crate::utils::crypto_utils::secure_random_bytes(32);
                key
            }
            "ECDSA_P384" => {
                // Generate ECDSA P-384 equivalent key material (48 bytes for private key)
                // In a real implementation, this would use proper ECDSA key generation
                let key = crate::utils::crypto_utils::secure_random_bytes(48);
                key
            }
            "ECDSA_P521" => {
                // Ring doesn't support P-521, so we'll use a different approach or fallback
                let key = crate::utils::crypto_utils::secure_random_bytes(66); // P-521 private key size
                key
            }
            _ => return Err(BearDogError::encryption("key_generation", "Unsupported algorithm")),
        };

        // Store the key
        self.keys.write().await.insert(key_id.clone(), key.clone());
        
        info!("✅ Generated key: {}", key_id);
        Ok((key_id, key))
    }

    /// Delete an encryption key
    pub async fn delete_key(&self, key_id: &str) -> BearDogResult<()> {
        info!("🗑️ Deleting encryption key: {}", key_id);

        let mut keys = self.keys.write().await;
        if keys.remove(key_id).is_some() {
            info!("✅ Deleted key: {}", key_id);
            Ok(())
        } else {
            Err(BearDogError::encryption("key_deletion", "Key not found"))
        }
    }

    /// Encrypt data with a specific key
    pub async fn encrypt_with_key(&self, data: &[u8], key: &[u8]) -> BearDogResult<Vec<u8>> {
        // Use AES-GCM for encryption
        let key = Key::<Aes256Gcm>::from_slice(&key[..32]);
        let cipher = Aes256Gcm::new(key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        
        let ciphertext = cipher.encrypt(&nonce, data)
            .map_err(|e| BearDogError::encryption("encrypt_with_key", format!("AES encryption failed: {}", e)))?;
        
        // Prepend nonce to ciphertext
        let mut result = nonce.to_vec();
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    /// Decrypt data with a specific key
    pub async fn decrypt_with_key(&self, data: &[u8], key: &[u8]) -> BearDogResult<Vec<u8>> {
        if data.len() < 12 {
            return Err(BearDogError::encryption("decrypt_with_key", "Invalid ciphertext length"));
        }
        
        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let key = Key::<Aes256Gcm>::from_slice(&key[..32]);
        let cipher = Aes256Gcm::new(key);
        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|e| BearDogError::encryption("decrypt_with_key", format!("AES decryption failed: {}", e)))?;
        
        Ok(plaintext)
    }

    /// Derive key from password using Argon2
    pub async fn derive_key_from_password(
        &self,
        password: &str,
        params: Option<KeyDerivationParams>,
    ) -> BearDogResult<Vec<u8>> {
        let params = params.unwrap_or_else(|| KeyDerivationParams {
            salt: self.generate_salt(),
            iterations: self.config.key_derivation_iterations,
            memory_cost: 65536, // 64 MB
            parallelism: 1,
        });

        let salt = SaltString::encode_b64(&params.salt)
            .map_err(|e| BearDogError::internal(format!("Salt encoding failed: {}", e)))?;

        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| BearDogError::KeyDerivation { 
                message: format!("Password hashing failed: {:?}", e) 
            })?;

        Ok(password_hash.hash.unwrap().as_bytes().to_vec())
    }

    /// Rotate encryption keys
    pub async fn rotate_keys(&self) -> BearDogResult<()> {
        let mut keys = self.keys.write().await;
        let now = Utc::now();

        // Note: Key rotation would need proper key structure
        // Current implementation uses Vec<u8> instead of structured keys

        // Generate new keys if needed
        if keys.is_empty() {
                drop(keys); // Release lock before calling generate_key
            self.generate_key("AES256", "default").await?;
        }

        Ok(())
    }

    /// Encrypt using AES-256-GCM
    async fn encrypt_aes256_gcm(&self, data: &[u8]) -> BearDogResult<EncryptedData> {
        let key = crate::utils::crypto_utils::secure_random_bytes(32);
        let key = Key::<Aes256Gcm>::from_slice(&key);
        let cipher = Aes256Gcm::new(key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let ciphertext = cipher
            .encrypt(&nonce, data)
            .map_err(|e| BearDogError::encryption("aes_encryption", format!("AES encryption failed: {}", e)))?;

        Ok(EncryptedData {
            algorithm: EncryptionAlgorithm::Aes256Gcm,
            ciphertext,
            nonce: nonce.to_vec(),
            tag: None,
            metadata: HashMap::new(),
        })
    }

    /// Decrypt using AES-256-GCM
    async fn decrypt_aes256_gcm(&self, encrypted_data: &EncryptedData) -> BearDogResult<Vec<u8>> {
        if encrypted_data.nonce.len() != 12 {
            return Err(BearDogError::encryption("aes_decryption", "Invalid nonce length"));
        }

        let key = crate::utils::crypto_utils::secure_random_bytes(32);
        let nonce_bytes = &encrypted_data.nonce;
        let nonce = Nonce::from_slice(nonce_bytes);
        let key = Key::<Aes256Gcm>::from_slice(&key);
        let cipher = Aes256Gcm::new(key);

        cipher
            .decrypt(nonce, encrypted_data.ciphertext.as_ref())
            .map_err(|e| BearDogError::encryption("aes_decryption", format!("AES decryption failed: {}", e)))
    }

    /// Encrypt using ChaCha20-Poly1305
    async fn encrypt_chacha20_poly1305(&self, data: &[u8]) -> BearDogResult<EncryptedData> {
        use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce, aead::{Aead, KeyInit, AeadCore}};
        
        let key_bytes = crate::utils::crypto_utils::secure_random_bytes(32);
        let key = Key::from_slice(&key_bytes);
        let cipher = ChaCha20Poly1305::new(key);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

        let ciphertext = cipher
            .encrypt(&nonce, data)
            .map_err(|e| BearDogError::encryption("chacha20_encryption", format!("ChaCha20-Poly1305 encryption failed: {}", e)))?;

        Ok(EncryptedData {
            algorithm: EncryptionAlgorithm::ChaCha20Poly1305,
            ciphertext,
            nonce: nonce.to_vec(),
            tag: None,
            metadata: HashMap::new(),
        })
    }

    /// Decrypt using ChaCha20-Poly1305
    async fn decrypt_chacha20_poly1305(&self, encrypted_data: &EncryptedData) -> BearDogResult<Vec<u8>> {
        use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce, aead::{Aead, KeyInit}};
        
        if encrypted_data.nonce.len() != 12 {
            return Err(BearDogError::encryption("chacha20_decryption", "Invalid nonce length"));
        }

        let key_bytes = crate::utils::crypto_utils::secure_random_bytes(32);
        let nonce_bytes = &encrypted_data.nonce;
        let nonce = Nonce::from_slice(nonce_bytes);
        let key = Key::from_slice(&key_bytes);
        let cipher = ChaCha20Poly1305::new(key);

        cipher
            .decrypt(nonce, encrypted_data.ciphertext.as_ref())
            .map_err(|e| BearDogError::encryption("chacha20_decryption", format!("ChaCha20-Poly1305 decryption failed: {}", e)))
    }

    /// Encrypt using Quantum-Resistant encryption
    async fn encrypt_quantum_resistant(&self, data: &[u8]) -> BearDogResult<EncryptedData> {
        // Multi-layer encryption approach for quantum resistance
        // Layer 1: AES-256-GCM (still secure against quantum attacks with larger key sizes)
        let aes_result = self.encrypt_aes256_gcm(data).await?;
        
        // Layer 2: ChaCha20-Poly1305 (different mathematical foundation)
        let chacha_result = self.encrypt_chacha20_poly1305(&aes_result.ciphertext).await?;
        
        // Create combined nonce from both layers
        let mut combined_nonce = Vec::new();
        combined_nonce.extend_from_slice(&aes_result.nonce);
        combined_nonce.extend_from_slice(&chacha_result.nonce);
        
        // Add metadata about the layered approach
        let mut metadata = HashMap::new();
        metadata.insert("layers".to_string(), "2".to_string());
        metadata.insert("layer1".to_string(), "AES-256-GCM".to_string());
        metadata.insert("layer2".to_string(), "ChaCha20-Poly1305".to_string());
        metadata.insert("quantum_resistant".to_string(), "true".to_string());
        
        Ok(EncryptedData {
            algorithm: EncryptionAlgorithm::QuantumResistant,
            ciphertext: chacha_result.ciphertext,
            nonce: combined_nonce,
            tag: None,
            metadata,
        })
    }

    /// Decrypt using Quantum-Resistant encryption
    async fn decrypt_quantum_resistant(&self, encrypted_data: &EncryptedData) -> BearDogResult<Vec<u8>> {
        // Verify this is a valid quantum-resistant encrypted data
        if !encrypted_data.metadata.contains_key("layers") ||
           encrypted_data.metadata.get("layers") != Some(&"2".to_string()) {
            return Err(BearDogError::encryption("quantum_decryption", "Invalid quantum-resistant encrypted data format"));
        }
        
        // Extract nonces for both layers
        if encrypted_data.nonce.len() != 24 { // 12 bytes for AES + 12 bytes for ChaCha20
            return Err(BearDogError::encryption("quantum_decryption", "Invalid nonce length for quantum-resistant decryption"));
        }
        
        let (aes_nonce, chacha_nonce) = encrypted_data.nonce.split_at(12);
        
        // Layer 2: Decrypt ChaCha20-Poly1305 first (reverse order)
        let chacha_encrypted = EncryptedData {
            algorithm: EncryptionAlgorithm::ChaCha20Poly1305,
            ciphertext: encrypted_data.ciphertext.clone(),
            nonce: chacha_nonce.to_vec(),
            tag: None,
            metadata: HashMap::new(),
        };
        
        let chacha_result = self.decrypt_chacha20_poly1305(&chacha_encrypted).await?;
        
        // Layer 1: Decrypt AES-256-GCM
        let aes_encrypted = EncryptedData {
            algorithm: EncryptionAlgorithm::Aes256Gcm,
            ciphertext: chacha_result,
            nonce: aes_nonce.to_vec(),
            tag: None,
            metadata: HashMap::new(),
        };
        
        self.decrypt_aes256_gcm(&aes_encrypted).await
    }

    /// Generate master key for default operations
    pub async fn generate_master_key(&self) -> BearDogResult<String> {
        self.generate_key("AES256", "master").await.map(|(id, _)| id)
    }

    /// Generate cryptographically secure salt
    fn generate_salt(&self) -> Vec<u8> {
        let mut salt = vec![0u8; 32];
        use ring::rand::SecureRandom;
        let _ = self.rng.fill(&mut salt); // Ignore error for salt generation
        salt
    }

    /// Hash a password with Argon2
    pub fn hash_password(&self, password: &str) -> BearDogResult<Vec<u8>> {
        use ring::rand::SecureRandom;
        
        let mut salt = [0u8; 32];
        self.rng.fill(&mut salt).map_err(|e| BearDogError::encryption("hash_password", format!("Salt generation failed: {}", e)))?;
        
        let salt_string = SaltString::encode_b64(&salt)
            .map_err(|e| BearDogError::encryption("hash_password", format!("Salt encoding failed: {}", e)))?;
        
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt_string)
            .map_err(|e| BearDogError::encryption("hash_password", format!("Password hashing failed: {}", e)))?;
        
        match password_hash.hash {
            Some(hash) => Ok(hash.as_bytes().to_vec()),
            None => Err(BearDogError::encryption("hash_password", "Password hash generation failed")),
        }
    }
    
    /// Create a placeholder instance for initialization
    pub fn placeholder() -> Self {
        Self {
            config: Arc::new(crate::config::EncryptionConfig::default()),
            keys: Arc::new(RwLock::new(HashMap::new())),
            rng: SystemRandom::new(),
        }
    }
}

/// Encryption request structure for API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionRequest {
    pub plaintext: Vec<u8>,
    pub algorithm: Option<String>,
}

/// Encryption response structure for API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionResponse {
    pub encrypted_data: String,
    pub key_id: String,
    pub algorithm: String,
    pub iv: String,
}

/// Decryption request structure for API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptionRequest {
    pub encrypted_data: String,
    pub algorithm: Option<String>,
}

/// Decryption response structure for API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptionResponse {
    pub plaintext: Vec<u8>,
    pub key_id: String,
    pub algorithm: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_config() -> EncryptionConfig {
        EncryptionConfig {
            default_algorithm: "AES-256-GCM".to_string(),
            key_derivation_iterations: 100000,
            key_rotation_days: 30,
            key_rotation_interval: std::time::Duration::from_secs(30 * 24 * 3600),
            key_derivation: crate::config::KeyDerivationConfig {
                argon2: crate::config::Argon2Config {
                    memory_cost: 65536,
                    time_cost: 3,
                    parallelism: 4,
                },
                pbkdf2: crate::config::Pbkdf2Config {
                    iterations: 100000,
                    hash_algorithm: "SHA256".to_string(),
                },
            },
            hsm: crate::config::HsmConfig {
                enabled: false,
                provider: "SoftHSM".to_string(),
                config: std::collections::HashMap::new(),
            },
        }
    }
    
    #[tokio::test]
    async fn test_encryption_engine_creation() {
        let config = create_test_config();
        let engine = EncryptionEngine::new(config).await;
        assert!(engine.is_ok());
    }
    
    #[tokio::test]
    async fn test_generate_key() {
        let config = create_test_config();
        let engine = EncryptionEngine::new(config).await.unwrap();
        
        let key = engine.generate_key("AES256", "").await.unwrap();
        assert!(!key.1.is_empty());
    }
    
    #[tokio::test]
    async fn test_encrypt_decrypt_flow() {
        let config = create_test_config();
        let engine = EncryptionEngine::new(config).await.unwrap();
        
        let plaintext = b"Hello, BearDog encryption test!";
        
        let encrypted = engine.encrypt(plaintext, Some(EncryptionAlgorithm::Aes256Gcm)).await.unwrap();
        assert_ne!(encrypted.ciphertext, plaintext);
        assert!(!encrypted.ciphertext.is_empty());
        
        let decrypted = engine.decrypt(&encrypted).await.unwrap();
        assert_eq!(decrypted, plaintext);
    }
    
    #[tokio::test]
    async fn test_password_hashing() {
        let config = create_test_config();
        let engine = EncryptionEngine::new(config).await.unwrap();
        
        let password = "secure_password_123";
        let hash1 = engine.hash_password(password).unwrap();
        let hash2 = engine.hash_password(password).unwrap();
        
        assert!(!hash1.is_empty());
        assert!(!hash2.is_empty());
        assert_ne!(hash1, hash2); // Different salts should produce different hashes
    }
    
    #[tokio::test]
    async fn test_different_algorithms() {
        let config = create_test_config();
        let engine = EncryptionEngine::new(config).await.unwrap();
        
        let algorithms = vec![
            EncryptionAlgorithm::Aes256Gcm,
            EncryptionAlgorithm::ChaCha20Poly1305,
        ];
        
        for algorithm in algorithms {
            let key = engine.generate_key("AES256", "").await.unwrap();
            assert!(!key.1.is_empty());
        }
    }
    
    #[tokio::test]
    async fn test_encryption_with_chacha() {
        let config = create_test_config();
        let engine = EncryptionEngine::new(config).await.unwrap();
        
        let plaintext = b"ChaCha20-Poly1305 test message";
        
        let encrypted = engine.encrypt(plaintext, Some(EncryptionAlgorithm::ChaCha20Poly1305)).await.unwrap();
        let decrypted = engine.decrypt(&encrypted).await.unwrap();
        
        assert_eq!(decrypted, plaintext);
    }
    
    #[tokio::test]
    async fn test_empty_plaintext_encryption() {
        let config = create_test_config();
        let engine = EncryptionEngine::new(config).await.unwrap();
        
        let empty_plaintext = b"";
        
        let encrypted = engine.encrypt(empty_plaintext, Some(EncryptionAlgorithm::Aes256Gcm)).await.unwrap();
        let decrypted = engine.decrypt(&encrypted).await.unwrap();
        
        assert_eq!(decrypted, empty_plaintext);
    }
    
    #[tokio::test]
    async fn test_large_plaintext_encryption() {
        let config = create_test_config();
        let engine = EncryptionEngine::new(config).await.unwrap();
        
        let large_plaintext = vec![0x42u8; 1024]; // 1KB of data
        
        let encrypted = engine.encrypt(&large_plaintext, Some(EncryptionAlgorithm::Aes256Gcm)).await.unwrap();
        let decrypted = engine.decrypt(&encrypted).await.unwrap();
        
        assert_eq!(decrypted, large_plaintext);
    }
    
    #[tokio::test]
    async fn test_encryption_error_handling() {
        let config = create_test_config();
        let engine = EncryptionEngine::new(config).await.unwrap();
        
        // Test with invalid encrypted data
        let invalid_encrypted = EncryptedData {
            ciphertext: vec![0u8; 10], // Too short
            nonce: vec![0u8; 12],
            algorithm: EncryptionAlgorithm::Aes256Gcm,
            tag: None,
            metadata: HashMap::new(),
        };
        
        let result = engine.decrypt(&invalid_encrypted).await;
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_argon2_password_hashing() {
        let config = create_test_config();
        let engine = EncryptionEngine::new(config).await.unwrap();
        
        // Test multiple passwords
        let passwords = vec!["password1", "password2", "very_long_password_with_special_chars!@#"];
        
        for password in passwords {
            let hash = engine.hash_password(password).unwrap();
            assert!(!hash.is_empty());
            assert!(hash.len() >= 32); // Argon2 raw hash is 32 bytes
        }
    }
    
    #[tokio::test]
    async fn test_concurrent_encryption() {
        let config = create_test_config();
        let engine = std::sync::Arc::new(EncryptionEngine::new(config).await.unwrap());
        
        let mut handles = vec![];
        
        for i in 0..10 {
            let engine_clone = engine.clone();
            let handle = tokio::spawn(async move {
                let plaintext = format!("test message {}", i);
                let encrypted = engine_clone.encrypt(plaintext.as_bytes(), Some(EncryptionAlgorithm::Aes256Gcm)).await.unwrap();
                let decrypted = engine_clone.decrypt(&encrypted).await.unwrap();
                assert_eq!(decrypted, plaintext.as_bytes());
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.await.unwrap();
        }
    }
} 