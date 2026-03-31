// SPDX-License-Identifier: AGPL-3.0-only

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use async_trait::async_trait;
use beardog_errors::BearDogError;
use rand_core::{OsRng, RngCore};
use std::sync::Arc;

const AES_GCM_NONCE_LEN: usize = 12;
const AES_GCM_TAG_LEN: usize = 16;
const AES_256_KEY_LEN: usize = 32;

#[async_trait]
pub trait EncryptionKey: Send + Sync {
    async fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, BearDogError>;

    async fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError>;
}

pub struct AesGcmEncryptionKey {
    cipher: Arc<Aes256Gcm>,
}

pub type DefaultEncryptionKey = AesGcmEncryptionKey;

impl AesGcmEncryptionKey {
    /// # Errors
    ///
    /// Returns an error if encryption fails.
    pub fn new() -> Result<Self, BearDogError> {
        let mut key_bytes = [0u8; AES_256_KEY_LEN];
        rand::rng().fill_bytes(&mut key_bytes);
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        Ok(Self {
            cipher: Arc::new(cipher),
        })
    }

    /// # Errors
    ///
    /// Returns an error if encryption fails.
    pub fn from_key_material(key_material: &[u8]) -> Result<Self, BearDogError> {
        if key_material.len() != AES_256_KEY_LEN {
            return Err(BearDogError::encryption(
                "key_initialization".to_string(),
                "Key material must be exactly 32 bytes for AES-256".to_string(),
            ));
        }
        let key = Key::<Aes256Gcm>::from_slice(key_material);
        let cipher = Aes256Gcm::new(key);
        Ok(Self {
            cipher: Arc::new(cipher),
        })
    }

    #[must_use]
    pub fn fallback() -> Self {
        let zero_key = [0u8; AES_256_KEY_LEN];
        let key = Key::<Aes256Gcm>::from_slice(&zero_key);
        Self {
            cipher: Arc::new(Aes256Gcm::new(key)),
        }
    }
}

impl Default for AesGcmEncryptionKey {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            tracing::error!(
                "Operation failed ({}): {:?}",
                "Failed to create default encryption key",
                e
            );
            Self::fallback()
        })
    }
}

#[async_trait]
impl EncryptionKey for AesGcmEncryptionKey {
    async fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        let mut nonce_bytes = [0u8; AES_GCM_NONCE_LEN];
        rand::rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| {
                BearDogError::encryption(
                    "aes_gcm_encrypt".to_string(),
                    format!("AES-GCM encryption failed: {e}"),
                )
            })?;

        let mut result = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    async fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        if ciphertext.len() < AES_GCM_NONCE_LEN + AES_GCM_TAG_LEN {
            return Err(BearDogError::encryption(
                "aes_gcm_decrypt".to_string(),
                "Ciphertext too short - missing nonce or tag".to_string(),
            ));
        }

        let (nonce_bytes, encrypted_data) = ciphertext.split_at(AES_GCM_NONCE_LEN);
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = self
            .cipher
            .decrypt(nonce, encrypted_data)
            .map_err(|e| {
                BearDogError::encryption(
                    "aes_gcm_decrypt".to_string(),
                    format!("AES-GCM decryption failed: {e}"),
                )
            })?;
        Ok(plaintext)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_encryption_roundtrip() -> Result<(), BearDogError> {
        let key = AesGcmEncryptionKey::new()?;
        let plaintext = b"Hello, `BearDog` secure encryption!";
        let ciphertext = key.encrypt(plaintext).await?;
        let decrypted = key.decrypt(&ciphertext).await?;
        assert_eq!(plaintext, decrypted.as_slice());
        Ok(())
    }

    #[tokio::test]
    async fn test_different_nonces() -> Result<(), BearDogError> {
        let key = AesGcmEncryptionKey::new()?;
        let plaintext = b"Same message, different nonces";
        let ciphertext1 = key.encrypt(plaintext).await?;
        let ciphertext2 = key.encrypt(plaintext).await?;

        assert_ne!(ciphertext1, ciphertext2);

        assert_eq!(key.decrypt(&ciphertext1).await?, plaintext);
        assert_eq!(key.decrypt(&ciphertext2).await?, plaintext);
        Ok(())
    }
}
