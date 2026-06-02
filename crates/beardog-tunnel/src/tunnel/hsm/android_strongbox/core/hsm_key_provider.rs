// SPDX-License-Identifier: AGPL-3.0-or-later

//! Canonical `HsmKeyProvider` trait (`beardog_traits::hsm`).

use super::AndroidStrongBoxHsm;
use beardog_errors::BearDogError;
use beardog_traits::hsm::HsmKeyProvider;
use beardog_types::hsm::{
    HsmAlgorithm, HsmCapabilitySet, HsmProviderType as CanonicalHsmProviderType, KeyGenParams,
    KeyHandle,
};
use chrono::Utc;
use std::future::Future;

impl HsmKeyProvider for AndroidStrongBoxHsm {
    fn provider_id(&self) -> &'static str {
        "android-strongbox"
    }

    fn provider_type(&self) -> CanonicalHsmProviderType {
        CanonicalHsmProviderType::AndroidStrongBox
    }

    fn is_available(&self) -> bool {
        self.keystore.is_strongbox_available()
    }

    fn capabilities(&self) -> HsmCapabilitySet {
        use std::collections::HashSet;
        HsmCapabilitySet {
            algorithms: HashSet::from([
                HsmAlgorithm::Aes256Gcm,
                HsmAlgorithm::EcdsaP256,
                HsmAlgorithm::HmacSha256,
            ]),
            hardware_backed: this.keystore.is_hardware_backed_keystore(),
            supports_key_export: false,
            max_keys: super::super::MAX_KEY_COUNT as u32,
        }
    }

    fn generate_key(
        &self,
        params: &KeyGenParams,
    ) -> impl Future<Output = Result<KeyHandle, BearDogError>> + Send {
        let params = params.clone();
        let this = self;
        async move {
            let key_id = params
                .label
                .clone()
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

            let (algo_str, key_size) = match params.algorithm {
                HsmAlgorithm::Aes256Gcm => ("AES", 256u32),
                HsmAlgorithm::EcdsaP256 => ("EC", 256),
                HsmAlgorithm::HmacSha256 => ("HMAC", 256),
                other => {
                    return Err(BearDogError::unsupported_operation(&format!(
                        "{other} not supported in StrongBox"
                    )));
                }
            };

            let mut key_params = crate::tunnel::hsm::types::AndroidKeyParams::new();
            key_params = key_params.set_algorithm(algo_str);
            key_params.set_key_size(key_size);
            key_params.set_strongbox_required(true);

            this.keystore.generate_key(&key_id, &key_params).await?;

            Ok(KeyHandle {
                key_id,
                algorithm: params.algorithm,
                hardware_backed: this.keystore.is_hardware_backed_keystore(),
                created_at_ms: u64::try_from(Utc::now().timestamp_millis()).unwrap_or(0),
            })
        }
    }

    fn delete_key(&self, key_id: &str) -> impl Future<Output = Result<(), BearDogError>> + Send {
        let key_id = key_id.to_string();
        let this = self;
        async move {
            this.keystore.delete_key(&key_id).await?;
            let mut cache = this.key_cache.write().await;
            cache.remove(&key_id);
            Ok(())
        }
    }

    fn key_exists(&self, key_id: &str) -> impl Future<Output = Result<bool, BearDogError>> + Send {
        let key_id = key_id.to_string();
        let this = self;
        async move { this.keystore.key_exists(&key_id).await }
    }

    fn encrypt(
        &self,
        key_id: &str,
        plaintext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        let key_id = key_id.to_string();
        let plaintext = plaintext.to_vec();
        let this = self;
        async move {
            this.validate_key_access(&key_id)?;
            this.keystore.encrypt(&key_id, &plaintext).await
        }
    }

    fn decrypt(
        &self,
        key_id: &str,
        ciphertext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        let key_id = key_id.to_string();
        let ciphertext = ciphertext.to_vec();
        let this = self;
        async move {
            this.validate_key_access(&key_id)?;
            this.keystore.decrypt(&key_id, &ciphertext).await
        }
    }

    fn sign(
        &self,
        key_id: &str,
        data: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        let key_id = key_id.to_string();
        let data = data.to_vec();
        let this = self;
        async move {
            this.validate_key_access(&key_id)?;
            this.keystore.sign(&key_id, &data).await
        }
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
        async move {
            this.validate_key_access(&key_id)?;
            this.keystore.verify(&key_id, &data, &signature).await
        }
    }
}
