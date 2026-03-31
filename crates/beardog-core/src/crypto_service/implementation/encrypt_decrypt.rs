// SPDX-License-Identifier: AGPL-3.0-only

//! Symmetric encryption and decryption (`CryptoAlgorithm` AEAD paths).

use super::BearDogCryptoService;
use crate::crypto_service::Result;
use crate::crypto_service::algorithms::symmetric;
use beardog_errors::BearDogError;
use beardog_types::crypto_service::{
    CryptoAlgorithm, DecryptOptions, EncryptOptions, EncryptedData, EncryptionMetadata,
};
use std::time::SystemTime;

impl BearDogCryptoService {
    pub(crate) async fn encrypt_impl(
        &self,
        data: &[u8],
        algorithm: CryptoAlgorithm,
        options: EncryptOptions,
    ) -> Result<EncryptedData> {
        let _op_id = self.next_operation_id();
        let start_time = SystemTime::now();

        // Validate input
        self.validate_data_size(data)?;

        let key_id = &options.key_id;

        // Check algorithm support
        if !self.algorithms.supports_crypto(&algorithm) {
            return Err(BearDogError::business(format!(
                "Algorithm {algorithm:?} not supported"
            )));
        }

        // Delegate to appropriate algorithm module
        let (ciphertext, nonce, tag) = match algorithm {
            CryptoAlgorithm::Aes256Gcm => {
                let key = self.derive_key_256(key_id)?;
                symmetric::encrypt_aes_256_gcm(data, &key, options.associated_data.as_deref())?
            }
            CryptoAlgorithm::Aes128Gcm => {
                let key = self.derive_key_128(key_id)?;
                symmetric::encrypt_aes_128_gcm(data, &key, options.associated_data.as_deref())?
            }
            CryptoAlgorithm::ChaCha20Poly1305 => {
                let key = self.derive_key_256(key_id)?;
                symmetric::encrypt_chacha20_poly1305(
                    data,
                    &key,
                    options.associated_data.as_deref(),
                )?
            }
        };

        self.audit_log("encrypt", Some(key_id), true);

        Ok(EncryptedData {
            ciphertext,
            algorithm,
            metadata: EncryptionMetadata {
                timestamp: start_time,
                key_id: Some(key_id.clone()),
                nonce,
                tag: Some(tag),
            },
        })
    }

    pub(crate) async fn decrypt_impl(
        &self,
        encrypted: &EncryptedData,
        options: DecryptOptions,
    ) -> Result<Vec<u8>> {
        let _op_id = self.next_operation_id();

        let key_id = if options.key_id.is_empty() {
            encrypted
                .metadata
                .key_id
                .as_ref()
                .ok_or_else(|| BearDogError::validation("key_id is required for decryption"))?
        } else {
            &options.key_id
        };

        let nonce = &encrypted.metadata.nonce;
        let tag = encrypted
            .metadata
            .tag
            .as_deref()
            .ok_or_else(|| BearDogError::validation("Authentication tag is required"))?;

        // Delegate to appropriate algorithm module
        let plaintext = match encrypted.algorithm {
            CryptoAlgorithm::Aes256Gcm => {
                let key = self.derive_key_256(key_id)?;
                symmetric::decrypt_aes_256_gcm(
                    &encrypted.ciphertext,
                    nonce,
                    tag,
                    &key,
                    options.associated_data.as_deref(),
                )?
            }
            CryptoAlgorithm::Aes128Gcm => {
                let key = self.derive_key_128(key_id)?;
                symmetric::decrypt_aes_128_gcm(
                    &encrypted.ciphertext,
                    nonce,
                    tag,
                    &key,
                    options.associated_data.as_deref(),
                )?
            }
            CryptoAlgorithm::ChaCha20Poly1305 => {
                let key = self.derive_key_256(key_id)?;
                symmetric::decrypt_chacha20_poly1305(
                    &encrypted.ciphertext,
                    nonce,
                    tag,
                    &key,
                    options.associated_data.as_deref(),
                )?
            }
        };

        self.audit_log("decrypt", Some(key_id), true);

        Ok(plaintext)
    }
}
