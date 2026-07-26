// SPDX-License-Identifier: AGPL-3.0-or-later

//! [`HsmKeyProvider`] implementation for the Windows DPAPI backend.

use super::WindowsDpapiHsm;
use beardog_errors::BearDogError;
use beardog_traits::hsm::HsmKeyProvider;
use beardog_types::hsm::{
    HsmAlgorithm, HsmCapabilitySet, HsmProviderType, KeyGenParams, KeyHandle,
};
use std::collections::HashSet;
use std::future::Future;

impl HsmKeyProvider for WindowsDpapiHsm {
    fn provider_id(&self) -> &'static str {
        "windows-dpapi"
    }

    fn provider_type(&self) -> HsmProviderType {
        HsmProviderType::WindowsDpapi
    }

    fn is_available(&self) -> bool {
        cfg!(windows)
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
            ]),
            hardware_backed: true,
            supports_key_export: false,
            max_keys: 0,
        }
    }

    #[expect(
        clippy::manual_async_fn,
        reason = "trait method returns impl Future; async fn would refine the signature"
    )]
    fn generate_key(
        &self,
        #[cfg_attr(
            not(windows),
            expect(unused_variables, reason = "key generation params only used on Windows DPAPI")
        )]
        params: &KeyGenParams,
    ) -> impl Future<Output = Result<KeyHandle, BearDogError>> + Send {
        async move {
            if !self.is_available() {
                return Err(BearDogError::not_yet_available(
                    "Windows DPAPI HSM is not available on this platform",
                ));
            }

            #[cfg(windows)]
            {
                let params = params.clone();
                let key_id = params
                    .label
                    .clone()
                    .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

                let key_material = generate_raw_key_material(params.algorithm)?;

                let protected = Self::dpapi_protect(&key_material)?;
                self.store_blob(&key_id, &protected).await?;

                let now_ms = u64::try_from(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis(),
                )
                .unwrap_or(0);

                let entry = super::DpapiKeyEntry {
                    key_id: key_id.clone(),
                    algorithm: params.algorithm,
                    created_at_ms: now_ms,
                };
                self.keys.write().await.insert(key_id.clone(), entry);

                Ok(KeyHandle {
                    key_id,
                    algorithm: params.algorithm,
                    hardware_backed: true,
                    created_at_ms: now_ms,
                })
            }
            #[cfg(not(windows))]
            {
                Err(BearDogError::not_yet_available(
                    "DPAPI not available on this platform",
                ))
            }
        }
    }

    fn delete_key(&self, key_id: &str) -> impl Future<Output = Result<(), BearDogError>> + Send {
        let key_id = key_id.to_string();
        async move {
            self.keys.write().await.remove(&key_id);
            let path = self.blob_path(&key_id);
            if path.exists() {
                tokio::fs::remove_file(&path).await.map_err(|e| {
                    BearDogError::internal(format!("Failed to delete DPAPI blob: {e}"))
                })?;
            }
            Ok(())
        }
    }

    fn key_exists(&self, key_id: &str) -> impl Future<Output = Result<bool, BearDogError>> + Send {
        let key_id = key_id.to_string();
        async move { Ok(self.keys.read().await.contains_key(&key_id)) }
    }

    fn encrypt(
        &self,
        key_id: &str,
        plaintext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        let key_id = key_id.to_string();
        let plaintext = plaintext.to_vec();
        async move {
            let key_material = self.unwrap_key(&key_id).await?;
            aes_256_gcm_encrypt(&key_material, &plaintext)
        }
    }

    fn decrypt(
        &self,
        key_id: &str,
        ciphertext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        let key_id = key_id.to_string();
        let ciphertext = ciphertext.to_vec();
        async move {
            let key_material = self.unwrap_key(&key_id).await?;
            aes_256_gcm_decrypt(&key_material, &ciphertext)
        }
    }

    fn sign(
        &self,
        key_id: &str,
        data: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        let key_id = key_id.to_string();
        let data = data.to_vec();
        async move {
            let key_material = self.unwrap_key(&key_id).await?;
            hmac_sha256_sign(&key_material, &data)
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
        async move {
            let key_material = self.unwrap_key(&key_id).await?;
            let expected = hmac_sha256_sign(&key_material, &data)?;
            Ok(expected == signature)
        }
    }
}

impl WindowsDpapiHsm {
    /// Unwrap (DPAPI-decrypt) the key material for `key_id`.
    async fn unwrap_key(&self, key_id: &str) -> Result<Vec<u8>, BearDogError> {
        if !self.is_available() {
            return Err(BearDogError::not_yet_available(
                "Windows DPAPI HSM is not available on this platform",
            ));
        }

        #[cfg(windows)]
        {
            let blob = self.load_blob(key_id).await?;
            Self::dpapi_unprotect(&blob)
        }
        #[cfg(not(windows))]
        {
            let _ = key_id;
            Err(BearDogError::not_yet_available(
                "DPAPI not available on this platform",
            ))
        }
    }
}

#[cfg(windows)]
fn generate_raw_key_material(algorithm: HsmAlgorithm) -> Result<Vec<u8>, BearDogError> {
    use rand_core::RngCore;

    let size = match algorithm {
        HsmAlgorithm::Aes256Gcm | HsmAlgorithm::ChaCha20Poly1305 | HsmAlgorithm::HmacSha256 => 32,
        HsmAlgorithm::Ed25519 | HsmAlgorithm::X25519 => 32,
        HsmAlgorithm::EcdsaP256 | HsmAlgorithm::EcdsaP384 => 32,
        HsmAlgorithm::Rsa2048 | HsmAlgorithm::Rsa4096 => {
            return Err(BearDogError::internal(
                "RSA key generation not supported in DPAPI backend (use software HSM)".to_string(),
            ));
        }
    };

    let mut buf = vec![0u8; size];
    rand::rng().fill_bytes(&mut buf);
    Ok(buf)
}

fn aes_256_gcm_encrypt(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, BearDogError> {
    use aes_gcm::{Aes256Gcm, KeyInit, aead::Aead};
    use rand_core::RngCore;

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| BearDogError::internal(format!("AES key init: {e}")))?;
    let mut nonce_bytes = [0u8; 12];
    rand::rng().fill_bytes(&mut nonce_bytes);
    let nonce = aes_gcm::Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| BearDogError::internal(format!("AES encrypt: {e}")))?;
    let mut out = nonce_bytes.to_vec();
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

fn aes_256_gcm_decrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>, BearDogError> {
    use aes_gcm::{Aes256Gcm, KeyInit, aead::Aead};

    if data.len() < 12 {
        return Err(BearDogError::internal(
            "Ciphertext too short (missing nonce)".to_string(),
        ));
    }
    let (nonce_bytes, ciphertext) = data.split_at(12);
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| BearDogError::internal(format!("AES key init: {e}")))?;
    let nonce = aes_gcm::Nonce::from_slice(nonce_bytes);
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| BearDogError::internal(format!("AES decrypt: {e}")))
}

fn hmac_sha256_sign(key: &[u8], data: &[u8]) -> Result<Vec<u8>, BearDogError> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    let mut mac = <Hmac<Sha256>>::new_from_slice(key)
        .map_err(|e| BearDogError::internal(format!("HMAC init: {e}")))?;
    mac.update(data);
    Ok(mac.finalize().into_bytes().to_vec())
}
