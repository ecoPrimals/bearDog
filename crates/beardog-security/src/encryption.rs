// SPDX-License-Identifier: AGPL-3.0-only

//! Encryption Service Implementation
//!
//! Provides secure encryption and decryption capabilities using modern cryptographic algorithms.
//! Supports multiple algorithms including AES-256-GCM and ChaCha20-Poly1305.

#[cfg(test)]
#[path = "encryption_comprehensive_tests.rs"]
mod encryption_comprehensive_tests;

use crate::crypto_utils::BearDogCrypto;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

pub(crate) const AES_GCM_NONCE_LEN: usize = 12;
pub(crate) const AES_GCM_TAG_LEN: usize = 16;
pub(crate) const AES_256_KEY_LEN: usize = 32;

/// Supported encryption algorithms
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum EncryptionAlgorithm {
    #[serde(rename = "AES-256-GCM")]
    /// Represents aes256 gcm variant
    #[default]
    Aes256Gcm,
    #[serde(rename = "ChaCha20-Poly1305")]
    /// Represents cha cha20 poly1305 variant
    ChaCha20Poly1305,
}

/// Encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// The algorithm value
    pub algorithm: EncryptionAlgorithm,
    /// Number of `key_size`
    pub key_size: usize,
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            algorithm: EncryptionAlgorithm::default(),
            key_size: AES_256_KEY_LEN,
        }
    }
}

/// High-level encryption service for secure data protection
///
/// Provides authenticated encryption using modern algorithms like
/// AES-256-GCM with automatic nonce handling and key management.
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
    #[must_use]
    pub const fn is_initialized(&self) -> bool {
        true // Always initialized after construction
    }

    /// Encrypt data using the configured algorithm
    ///
    /// # Arguments
    /// * `input_bytes` - The plaintext data to encrypt
    /// * `key` - The encryption key (must match configured key_size)
    ///
    /// # Returns
    /// Encrypted data with embedded nonce (first 12 bytes)
    pub fn encrypt(&self, input_bytes: &[u8], key: &[u8]) -> Result<Vec<u8>, BearDogError> {
        self.encrypt_data(input_bytes, key)
    }

    /// Decrypt data using the configured algorithm
    ///
    /// # Arguments
    /// * `encrypted_data` - The ciphertext with embedded nonce
    /// * `key` - The decryption key
    ///
    /// # Returns
    /// The original plaintext data
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

        // Use AES-256-GCM for authenticated encryption
        let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(key, input_bytes, None)?;

        // Format: nonce (12 bytes) + ciphertext (includes 16-byte auth tag)
        let mut result = Vec::with_capacity(nonce.len() + ciphertext.len());
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);

        debug!(
            "✅ Successfully encrypted {} bytes to {} bytes",
            input_bytes.len(),
            result.len()
        );
        Ok(result)
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

        // Minimum size check: nonce + auth tag
        if encrypted_data.len() < AES_GCM_NONCE_LEN + AES_GCM_TAG_LEN {
            return Err(BearDogError::security(
                "Encrypted data too short (minimum 28 bytes required)".to_string(),
            ));
        }

        // Extract nonce and ciphertext (remaining bytes)
        let (nonce, ciphertext) = encrypted_data.split_at(AES_GCM_NONCE_LEN);

        // Decrypt using AES-256-GCM with authentication
        let plaintext = BearDogCrypto::decrypt_aes_gcm(key, ciphertext, nonce)?;

        debug!(
            "✅ Successfully decrypted {} bytes to {} bytes",
            encrypted_data.len(),
            plaintext.len()
        );
        Ok(plaintext)
    }
}
