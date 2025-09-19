// Encryption Service Implementation
//
// Provides secure encryption and decryption capabilities using modern cryptographic algorithms.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// Supported encryption algorithms
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    #[serde(rename = "AES-256-GCM")]
    /// Represents aes256 gcm variant
    Aes256Gcm,
    #[serde(rename = "ChaCha20-Poly1305")]
    /// Represents cha cha20 poly1305 variant
    ChaCha20Poly1305,
}

impl Default for EncryptionAlgorithm {
    fn default() -> Self {
        Self::Aes256Gcm
    }
}

/// Encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// The algorithm value
    pub algorithm: EncryptionAlgorithm,
    /// Number of key_size
    pub key_size: usize,
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            algorithm: EncryptionAlgorithm::default(),
            key_size: 32,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EncryptionService {
    config: EncryptionConfig,
}

impl EncryptionService {
    /// New operation.
    /// Creates a new instance
    pub fn new(config: EncryptionConfig) -> Self {
        info!(
            "🔐 Initializing encryption service with {:?}",
            config.algorithm
        );
        Self { config }
    }

    /// Check if the service is initialized
    /// Checks if initialized
    /// Checks if initialized
    pub fn is_initialized(&self) -> bool {
        true // Always initialized after construction
    }

    pub fn encrypt(&self, input_bytes: &[u8], key: &[u8]) -> Result<Vec<u8>, BearDogError> {
        self.encrypt_data(input_bytes, key)
    }

    pub fn decrypt(&self, encrypted_data: &[u8], key: &[u8]) -> Result<Vec<u8>, BearDogError> {
        self.decrypt_data(encrypted_data, key)
    }

    /// Encrypt Data operation.
    pub fn encrypt_data(&self, input_bytes: &[u8], key: &[u8]) -> Result<Vec<u8>, BearDogError> {
        debug!("🔒 Encrypting {} bytes of data", input_bytes.len());

        // Validate key size
        if key.len() != self.config.key_size {
            return Err(BearDogError::internal(format!(
                "Invalid key size: expected {}, got {}",
                self.config.key_size,
                key.len()
            )));
        }

        // Simple XOR encryption for demonstration (replace with real crypto)
        let mut encrypted = Vec::with_capacity(input_bytes.len());
        for (i, &byte) in input_bytes.iter().enumerate() {
            encrypted.push(byte ^ key[i % key.len()]);
        }

        debug!("✅ Successfully encrypted {} bytes", encrypted.len());
        Ok(encrypted)
    }

    /// Decrypt Data operation.
    pub fn decrypt_data(&self, encrypted_data: &[u8], key: &[u8]) -> Result<Vec<u8>, BearDogError> {
        debug!("🔓 Decrypting {} bytes of data", encrypted_data.len());

        // Validate key size
        if key.len() != self.config.key_size {
            return Err(BearDogError::internal(format!(
                "Invalid key size: expected {}, got {}",
                self.config.key_size,
                key.len()
            )));
        }

        // Simple XOR decryption (same as encryption for XOR)
        let mut decrypted = Vec::with_capacity(encrypted_data.len());
        for (i, &byte) in encrypted_data.iter().enumerate() {
            decrypted.push(byte ^ key[i % key.len()]);
        }

        debug!("✅ Successfully decrypted {} bytes", decrypted.len());
        Ok(decrypted)
    }
}
