// SPDX-License-Identifier: AGPL-3.0-or-later

//! Default AES-GCM software encryption key and the backend enum.

use beardog_errors::BearDogError;
use std::future::Future;
use std::sync::Arc;

use super::config::SoftwareHsmConfig;

/// Default encryption key implementation using AES-256-GCM
#[derive(Clone, Copy)]
pub struct DefaultEncryptionKey {
    /// Root key for encryption
    root_key: [u8; 32],
}

impl DefaultEncryptionKey {
    /// # Errors
    ///
    /// Returns an error if decryption fails.
    /// Create new default encryption key
    pub fn create(_config: &SoftwareHsmConfig) -> Result<Arc<EncryptionKeyBackend>, BearDogError> {
        let root_key = *b"BearDog_RootKey_256bit_Secure!!!"; // 32 bytes
        Ok(Arc::new(EncryptionKeyBackend::Default(Self { root_key })))
    }

    /// Encrypt data (AES-256-GCM).
    ///
    /// # Errors
    ///
    /// Returns an error if encryption or random nonce generation fails.
    pub fn encrypt(
        &self,
        plaintext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send + '_ {
        let plaintext = plaintext.to_vec();
        let root_key = self.root_key;
        async move {
            use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce, aead::Aead};
            use rand::RngCore;

            let key = Key::<Aes256Gcm>::from_slice(&root_key);
            let cipher = Aes256Gcm::new(key);

            let mut nonce_bytes = [0u8; 12];
            rand::rng().fill_bytes(&mut nonce_bytes);
            let nonce = Nonce::from_slice(&nonce_bytes);

            let ciphertext = cipher.encrypt(nonce, plaintext.as_slice()).map_err(|e| {
                BearDogError::crypto_error(format!("AES-256-GCM encryption failed: {e}"))
            })?;

            let mut result = nonce_bytes.to_vec();
            result.extend_from_slice(&ciphertext);
            Ok(result)
        }
    }

    /// Decrypt data (AES-256-GCM).
    ///
    /// # Errors
    ///
    /// Returns an error if the ciphertext is malformed or authentication fails.
    pub fn decrypt(
        &self,
        ciphertext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send + '_ {
        let ciphertext = ciphertext.to_vec();
        let root_key = self.root_key;
        async move {
            use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce, aead::Aead};

            if ciphertext.len() < 12 {
                return Err(BearDogError::crypto_error(
                    "Ciphertext too short to contain nonce",
                ));
            }

            let (nonce_bytes, encrypted_data) = ciphertext.split_at(12);
            let nonce = Nonce::from_slice(nonce_bytes);

            let key = Key::<Aes256Gcm>::from_slice(&root_key);
            let cipher = Aes256Gcm::new(key);

            cipher.decrypt(nonce, encrypted_data).map_err(|e| {
                BearDogError::crypto_error(format!("AES-256-GCM decryption failed: {e}"))
            })
        }
    }
}

impl Default for DefaultEncryptionKey {
    fn default() -> Self {
        Self {
            root_key: *b"BearDog_RootKey_256bit_Secure!!!", // 32 bytes
        }
    }
}

/// Encryption key backend (replaces `Arc<dyn EncryptionKeyTrait>`).
pub enum EncryptionKeyBackend {
    /// Default AES-256-GCM software key
    Default(DefaultEncryptionKey),
}

impl EncryptionKeyBackend {
    /// Encrypt data
    ///
    /// # Errors
    ///
    /// Returns an error if encryption fails.
    pub fn encrypt(
        &self,
        data: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send + '_ {
        let slf = self;
        let data = data.to_vec();
        async move {
            match slf {
                Self::Default(k) => k.encrypt(&data).await,
            }
        }
    }

    /// Decrypt data
    ///
    /// # Errors
    ///
    /// Returns an error if decryption fails.
    pub fn decrypt(
        &self,
        ciphertext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send + '_ {
        let slf = self;
        let ciphertext = ciphertext.to_vec();
        async move {
            match slf {
                Self::Default(k) => k.decrypt(&ciphertext).await,
            }
        }
    }
}
