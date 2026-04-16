// SPDX-License-Identifier: AGPL-3.0-or-later

//! [`beardog_traits::hsm::HsmKeyProvider`] implementation for canonical key handles.

use super::RustSoftwareHsm;
use crate::tunnel::hsm::GenerateKeyRequest;
use crate::tunnel::hsm::manager::HsmProvider;
use crate::tunnel::hsm::types::KeyType;
use beardog_errors::BearDogError;
use beardog_traits::hsm::HsmKeyProvider;
use beardog_types::hsm::{
    HsmAlgorithm, HsmCapabilitySet, HsmProviderType, KeyGenParams, KeyHandle,
};
use chrono::Utc;
use std::collections::HashSet;
use std::future::Future;

impl HsmKeyProvider for RustSoftwareHsm {
    fn provider_id(&self) -> &'static str {
        "software-rustcrypto"
    }

    fn provider_type(&self) -> HsmProviderType {
        HsmProviderType::Software
    }

    fn is_available(&self) -> bool {
        true
    }

    fn capabilities(&self) -> HsmCapabilitySet {
        HsmCapabilitySet {
            algorithms: HashSet::from([
                HsmAlgorithm::Aes256Gcm,
                HsmAlgorithm::ChaCha20Poly1305,
                HsmAlgorithm::Ed25519,
                HsmAlgorithm::EcdsaP256,
                HsmAlgorithm::EcdsaP384,
                HsmAlgorithm::X25519,
                HsmAlgorithm::HmacSha256,
                HsmAlgorithm::Rsa2048,
                HsmAlgorithm::Rsa4096,
            ]),
            hardware_backed: false,
            supports_key_export: true,
            max_keys: 0,
        }
    }

    fn generate_key(
        &self,
        params: &KeyGenParams,
    ) -> impl Future<Output = Result<KeyHandle, BearDogError>> + Send {
        let params = params.clone();
        let this = self;
        async move {
            let key_type = match params.algorithm {
                HsmAlgorithm::Aes256Gcm | HsmAlgorithm::HmacSha256 => KeyType::Aes,
                HsmAlgorithm::ChaCha20Poly1305 => KeyType::ChaCha20,
                HsmAlgorithm::Ed25519 | HsmAlgorithm::X25519 => KeyType::Ed25519,
                HsmAlgorithm::EcdsaP256 | HsmAlgorithm::EcdsaP384 => KeyType::EllipticCurve,
                HsmAlgorithm::Rsa2048 | HsmAlgorithm::Rsa4096 => KeyType::Rsa,
            };
            let request = GenerateKeyRequest {
                key_id: params
                    .label
                    .clone()
                    .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
                key_type,
            };
            let hsm_key = this.generate_software_key(&request).await?;

            Ok(KeyHandle {
                key_id: hsm_key.id,
                algorithm: params.algorithm,
                hardware_backed: false,
                created_at_ms: u64::try_from(Utc::now().timestamp_millis()).unwrap_or(0),
            })
        }
    }

    fn delete_key(&self, key_id: &str) -> impl Future<Output = Result<(), BearDogError>> + Send {
        let key_id = key_id.to_string();
        let this = self;
        async move { HsmProvider::delete_key(this, &key_id).await }
    }

    fn key_exists(&self, key_id: &str) -> impl Future<Output = Result<bool, BearDogError>> + Send {
        let key_id = key_id.to_string();
        let this = self;
        async move {
            let store = this.key_store.read().await;
            match store.get_key(&key_id).await {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        }
    }

    fn encrypt(
        &self,
        key_id: &str,
        plaintext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        let key_id = key_id.to_string();
        let plaintext = plaintext.to_vec();
        let this = self;
        async move { HsmProvider::encrypt(this, &key_id, &plaintext).await }
    }

    fn decrypt(
        &self,
        key_id: &str,
        ciphertext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        let key_id = key_id.to_string();
        let ciphertext = ciphertext.to_vec();
        let this = self;
        async move { HsmProvider::decrypt(this, &key_id, &ciphertext).await }
    }

    fn sign(
        &self,
        key_id: &str,
        data: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        let key_id = key_id.to_string();
        let data = data.to_vec();
        let this = self;
        async move { HsmProvider::sign(this, &key_id, &data).await }
    }

    fn verify(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> impl Future<Output = Result<bool, BearDogError>> + Send {
        let key_id = key_id.to_string();
        let data = data.to_vec();
        let signature = signature.to_vec();
        let this = self;
        async move { HsmProvider::verify(this, &key_id, &data, &signature).await }
    }
}
