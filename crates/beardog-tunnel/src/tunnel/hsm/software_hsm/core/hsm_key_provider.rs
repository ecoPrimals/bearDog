// SPDX-License-Identifier: AGPL-3.0-only

//! [`beardog_traits::hsm::HsmKeyProvider`] implementation for canonical key handles.

use super::RustSoftwareHsm;
use crate::tunnel::hsm::GenerateKeyRequest;
use crate::tunnel::hsm::manager::HsmProvider;
use beardog_errors::BearDogError;
use beardog_traits::hsm::HsmKeyProvider;
use beardog_types::hsm::{
    HsmAlgorithm, HsmCapabilitySet, HsmProviderType, KeyGenParams, KeyHandle,
};
use chrono::Utc;
use std::collections::HashSet;

#[async_trait::async_trait]
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

    async fn generate_key(&self, params: &KeyGenParams) -> Result<KeyHandle, BearDogError> {
        let key_type = Self::algorithm_to_key_type(params.algorithm);
        let request = GenerateKeyRequest {
            key_id: params
                .label
                .clone()
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
            key_type,
        };
        let hsm_key = self.generate_software_key(&request).await?;

        Ok(KeyHandle {
            key_id: hsm_key.id,
            algorithm: params.algorithm,
            hardware_backed: false,
            created_at_ms: u64::try_from(Utc::now().timestamp_millis()).unwrap_or(0),
        })
    }

    async fn delete_key(&self, key_id: &str) -> Result<(), BearDogError> {
        HsmProvider::delete_key(self, key_id).await
    }

    async fn key_exists(&self, key_id: &str) -> Result<bool, BearDogError> {
        let store = self.key_store.read().await;
        match store.get_key(key_id).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    async fn encrypt(&self, key_id: &str, plaintext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        HsmProvider::encrypt(self, key_id, plaintext).await
    }

    async fn decrypt(&self, key_id: &str, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        HsmProvider::decrypt(self, key_id, ciphertext).await
    }

    async fn sign(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        HsmProvider::sign(self, key_id, data).await
    }

    async fn verify(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
        HsmProvider::verify(self, key_id, data, signature).await
    }
}
