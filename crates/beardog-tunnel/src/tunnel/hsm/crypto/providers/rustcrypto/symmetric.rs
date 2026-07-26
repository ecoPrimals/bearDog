// SPDX-License-Identifier: AGPL-3.0-or-later

//! Symmetric encryption operations (AES-GCM, ChaCha20-Poly1305).

use super::RustCryptoProvider;
use crate::tunnel::hsm::crypto::algorithms::{DecryptionOptions, EncryptedData, EncryptionOptions};
use crate::tunnel::hsm::crypto::provider::NonceGenerator;
use beardog_errors::BearDogError;

impl RustCryptoProvider {
    pub(super) fn encrypt_aes_256_gcm(
        &self,
        key: &[u8],
        plaintext: &[u8],
        options: &EncryptionOptions,
    ) -> Result<EncryptedData, BearDogError> {
        self.encrypt_aes_gcm::<aes_gcm::Aes256Gcm>(key, plaintext, options, "AES-256-GCM")
    }

    pub(super) fn decrypt_aes_256_gcm(
        &self,
        key: &[u8],
        encrypted: &EncryptedData,
        options: &DecryptionOptions,
    ) -> Result<Vec<u8>, BearDogError> {
        self.decrypt_aes_gcm::<aes_gcm::Aes256Gcm>(key, encrypted, options, "AES-256-GCM")
    }

    pub(super) fn encrypt_aes_128_gcm(
        &self,
        key: &[u8],
        plaintext: &[u8],
        options: &EncryptionOptions,
    ) -> Result<EncryptedData, BearDogError> {
        self.encrypt_aes_gcm::<aes_gcm::Aes128Gcm>(key, plaintext, options, "AES-128-GCM")
    }

    pub(super) fn decrypt_aes_128_gcm(
        &self,
        key: &[u8],
        encrypted: &EncryptedData,
        options: &DecryptionOptions,
    ) -> Result<Vec<u8>, BearDogError> {
        self.decrypt_aes_gcm::<aes_gcm::Aes128Gcm>(key, encrypted, options, "AES-128-GCM")
    }

    pub(super) fn encrypt_chacha20_poly1305(
        &self,
        key: &[u8],
        plaintext: &[u8],
        options: &EncryptionOptions,
    ) -> Result<EncryptedData, BearDogError> {
        use chacha20poly1305::aead::Aead;
        use chacha20poly1305::{ChaCha20Poly1305, KeyInit, Nonce};

        let cipher = ChaCha20Poly1305::new_from_slice(key)
            .map_err(|e| BearDogError::crypto_error(format!("Invalid ChaCha20 key: {e}")))?;

        let nonce_vec = options
            .nonce
            .clone()
            .unwrap_or_else(|| self.generate_nonce(12));
        let nonce = Nonce::from_slice(&nonce_vec);

        let ciphertext = cipher.encrypt(nonce, plaintext).map_err(|e| {
            BearDogError::crypto_error(format!("ChaCha20-Poly1305 encryption failed: {e}"))
        })?;

        Ok(EncryptedData {
            algorithm: "ChaCha20-Poly1305".to_string(),
            ciphertext,
            nonce: Some(nonce_vec),
            tag: None,
        })
    }

    pub(super) fn decrypt_chacha20_poly1305(
        &self,
        key: &[u8],
        encrypted: &EncryptedData,
        _options: &DecryptionOptions,
    ) -> Result<Vec<u8>, BearDogError> {
        use chacha20poly1305::aead::Aead;
        use chacha20poly1305::{ChaCha20Poly1305, KeyInit, Nonce};

        let cipher = ChaCha20Poly1305::new_from_slice(key)
            .map_err(|e| BearDogError::crypto_error(format!("Invalid ChaCha20 key: {e}")))?;

        let nonce_vec = encrypted
            .nonce
            .as_ref()
            .ok_or_else(|| BearDogError::crypto_error("Missing nonce for ChaCha20-Poly1305"))?;
        let nonce = Nonce::from_slice(nonce_vec);

        cipher
            .decrypt(nonce, encrypted.ciphertext.as_ref())
            .map_err(|e| {
                BearDogError::crypto_error(format!("ChaCha20-Poly1305 decryption failed: {e}"))
            })
    }

    /// Generic AES-GCM encrypt — deduplicates AES-128/256.
    fn encrypt_aes_gcm<C>(
        &self,
        key: &[u8],
        plaintext: &[u8],
        options: &EncryptionOptions,
        algorithm_name: &str,
    ) -> Result<EncryptedData, BearDogError>
    where
        C: aes_gcm::KeyInit + aes_gcm::aead::Aead,
    {
        use aes_gcm::Nonce;

        let cipher = C::new_from_slice(key).map_err(|e| {
            BearDogError::crypto_error(format!("Invalid {algorithm_name} key: {e}"))
        })?;

        let nonce_vec = options
            .nonce
            .clone()
            .unwrap_or_else(|| self.generate_nonce(12));
        let nonce = Nonce::from_slice(&nonce_vec);

        let ciphertext = cipher.encrypt(nonce, plaintext).map_err(|e| {
            BearDogError::crypto_error(format!("{algorithm_name} encryption failed: {e}"))
        })?;

        Ok(EncryptedData {
            algorithm: algorithm_name.to_string(),
            ciphertext,
            nonce: Some(nonce_vec),
            tag: None,
        })
    }

    /// Generic AES-GCM decrypt — deduplicates AES-128/256.
    fn decrypt_aes_gcm<C>(
        &self,
        key: &[u8],
        encrypted: &EncryptedData,
        _options: &DecryptionOptions,
        algorithm_name: &str,
    ) -> Result<Vec<u8>, BearDogError>
    where
        C: aes_gcm::KeyInit + aes_gcm::aead::Aead,
    {
        use aes_gcm::Nonce;

        let cipher = C::new_from_slice(key).map_err(|e| {
            BearDogError::crypto_error(format!("Invalid {algorithm_name} key: {e}"))
        })?;

        let nonce_vec = encrypted.nonce.as_ref().ok_or_else(|| {
            BearDogError::crypto_error(format!("Missing nonce for {algorithm_name}"))
        })?;
        let nonce = Nonce::from_slice(nonce_vec);

        cipher
            .decrypt(nonce, encrypted.ciphertext.as_ref())
            .map_err(|e| {
                BearDogError::crypto_error(format!("{algorithm_name} decryption failed: {e}"))
            })
    }
}
