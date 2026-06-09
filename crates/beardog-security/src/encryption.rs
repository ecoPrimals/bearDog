// SPDX-License-Identifier: AGPL-3.0-or-later

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
    /// * `key` - The encryption key (must match configured `key_size`)
    ///
    /// # Returns
    /// Encrypted data with embedded nonce (first 12 bytes)
    ///
    /// # Errors
    ///
    /// Returns an error if the key size is wrong or AES-GCM encryption fails.
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
    ///
    /// # Errors
    ///
    /// Returns an error if the key size is wrong, ciphertext is too short, or decryption fails.
    pub fn decrypt(&self, encrypted_data: &[u8], key: &[u8]) -> Result<Vec<u8>, BearDogError> {
        self.decrypt_data(encrypted_data, key)
    }

    /// Encrypt Data operation.
    ///
    /// # Errors
    ///
    /// Returns an error if the key size is wrong or encryption fails.
    pub fn encrypt_data(&self, input_bytes: &[u8], key: &[u8]) -> Result<Vec<u8>, BearDogError> {
        debug!("Encrypting {} bytes of data", input_bytes.len());

        if key.len() != self.config.key_size {
            return Err(BearDogError::internal(format!(
                "Invalid key size: expected {}, got {}",
                self.config.key_size,
                key.len()
            )));
        }

        match self.config.algorithm {
            EncryptionAlgorithm::Aes256Gcm => {
                let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(key, input_bytes, None)?;
                let mut result = Vec::with_capacity(nonce.len() + ciphertext.len());
                result.extend_from_slice(&nonce);
                result.extend_from_slice(&ciphertext);
                Ok(result)
            }
            EncryptionAlgorithm::ChaCha20Poly1305 => {
                encrypt_chacha20_poly1305(key, input_bytes)
            }
        }
    }

    /// Decrypt Data operation.
    ///
    /// # Errors
    ///
    /// Returns an error if the key size is wrong, ciphertext is too short, or decryption fails.
    pub fn decrypt_data(&self, encrypted_data: &[u8], key: &[u8]) -> Result<Vec<u8>, BearDogError> {
        debug!("Decrypting {} bytes of data", encrypted_data.len());

        if key.len() != self.config.key_size {
            return Err(BearDogError::internal(format!(
                "Invalid key size: expected {}, got {}",
                self.config.key_size,
                key.len()
            )));
        }

        let nonce_len = match self.config.algorithm {
            EncryptionAlgorithm::Aes256Gcm => AES_GCM_NONCE_LEN,
            EncryptionAlgorithm::ChaCha20Poly1305 => CHACHA20_NONCE_LEN,
        };

        if encrypted_data.len() < nonce_len + AES_GCM_TAG_LEN {
            return Err(BearDogError::security(
                "Encrypted data too short".to_string(),
            ));
        }

        let (nonce, ciphertext) = encrypted_data.split_at(nonce_len);

        match self.config.algorithm {
            EncryptionAlgorithm::Aes256Gcm => {
                BearDogCrypto::decrypt_aes_gcm(key, ciphertext, nonce)
            }
            EncryptionAlgorithm::ChaCha20Poly1305 => {
                decrypt_chacha20_poly1305(key, ciphertext, nonce)
            }
        }
    }
}

pub(crate) const CHACHA20_NONCE_LEN: usize = 12;

fn encrypt_chacha20_poly1305(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, BearDogError> {
    use chacha20poly1305::aead::Aead;
    use chacha20poly1305::{ChaCha20Poly1305, KeyInit, Nonce};

    let cipher = ChaCha20Poly1305::new_from_slice(key)
        .map_err(|e| BearDogError::crypto_error(format!("Invalid ChaCha20 key: {e}")))?;

    let mut nonce_bytes = [0u8; CHACHA20_NONCE_LEN];
    rand::fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| BearDogError::crypto_error(format!("ChaCha20-Poly1305 encryption failed: {e}")))?;

    let mut result = Vec::with_capacity(CHACHA20_NONCE_LEN + ciphertext.len());
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);
    Ok(result)
}

fn decrypt_chacha20_poly1305(
    key: &[u8],
    ciphertext: &[u8],
    nonce: &[u8],
) -> Result<Vec<u8>, BearDogError> {
    use chacha20poly1305::aead::Aead;
    use chacha20poly1305::{ChaCha20Poly1305, KeyInit, Nonce};

    let cipher = ChaCha20Poly1305::new_from_slice(key)
        .map_err(|e| BearDogError::crypto_error(format!("Invalid ChaCha20 key: {e}")))?;

    let nonce = Nonce::from_slice(nonce);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| BearDogError::crypto_error(format!("ChaCha20-Poly1305 decryption failed: {e}")))
}
