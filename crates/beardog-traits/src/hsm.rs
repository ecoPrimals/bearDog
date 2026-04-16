// SPDX-License-Identifier: AGPL-3.0-or-later

//! HSM provider trait for key management and crypto operations.
//!
//! Backends include hardware (Android `StrongBox`, iOS Secure Enclave, TPM, PKCS#11)
//! and software (`RustCrypto` in-process key store). The tunnel crate uses enum
//! dispatch instead of trait objects for runtime polymorphism.

use beardog_errors::BearDogError;
use beardog_types::hsm::{HsmCapabilitySet, HsmProviderType, KeyGenParams, KeyHandle};
use std::future::Future;

/// HSM provider contract.
///
/// Every HSM backend (software, `StrongBox`, Secure Enclave, PKCS#11, TPM)
/// implements this trait. The tunnel registry selects the best available backend at runtime
/// via an enum wrapper rather than `dyn` dispatch.
pub trait HsmKeyProvider: Send + Sync {
    // ── identity ────────────────────────────────────────────────────

    /// Stable, unique identifier for this provider instance (e.g. `"software-rustcrypto"`).
    fn provider_id(&self) -> &'static str;

    /// The kind of backend (software, hardware, etc.).
    fn provider_type(&self) -> HsmProviderType;

    /// `true` when the provider is operational on this platform/device.
    fn is_available(&self) -> bool;

    /// Advertised algorithm support and constraints.
    fn capabilities(&self) -> HsmCapabilitySet;

    // ── key lifecycle ───────────────────────────────────────────────

    /// Generate a new key according to `params` and return its handle.
    fn generate_key(
        &self,
        params: &KeyGenParams,
    ) -> impl Future<Output = Result<KeyHandle, BearDogError>> + Send;

    /// Permanently delete the key identified by `key_id`.
    fn delete_key(&self, key_id: &str) -> impl Future<Output = Result<(), BearDogError>> + Send;

    /// Returns `true` if a key with the given id exists in this provider.
    fn key_exists(&self, key_id: &str) -> impl Future<Output = Result<bool, BearDogError>> + Send;

    // ── crypto operations ───────────────────────────────────────────

    /// Encrypt `plaintext` with the key identified by `key_id`.
    fn encrypt(
        &self,
        key_id: &str,
        plaintext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    /// Decrypt `ciphertext` previously produced by [`Self::encrypt`].
    fn decrypt(
        &self,
        key_id: &str,
        ciphertext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    /// Produce a signature over `data` using `key_id`.
    fn sign(
        &self,
        key_id: &str,
        data: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    /// Verify `signature` over `data` against `key_id`.
    fn verify(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> impl Future<Output = Result<bool, BearDogError>> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::hsm::{HsmAlgorithm, HsmCapabilitySet, HsmProviderType};
    use std::collections::HashSet;

    struct StubProvider;

    impl HsmKeyProvider for StubProvider {
        fn provider_id(&self) -> &'static str {
            "stub"
        }
        fn provider_type(&self) -> HsmProviderType {
            HsmProviderType::Software
        }
        fn is_available(&self) -> bool {
            true
        }
        fn capabilities(&self) -> HsmCapabilitySet {
            HsmCapabilitySet {
                algorithms: HashSet::from([HsmAlgorithm::Aes256Gcm]),
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
            async move {
                Ok(KeyHandle {
                    key_id: "stub-key".into(),
                    algorithm: params.algorithm,
                    hardware_backed: false,
                    created_at_ms: 0,
                })
            }
        }

        fn delete_key(
            &self,
            _key_id: &str,
        ) -> impl Future<Output = Result<(), BearDogError>> + Send {
            async move { Ok(()) }
        }

        fn key_exists(
            &self,
            _key_id: &str,
        ) -> impl Future<Output = Result<bool, BearDogError>> + Send {
            async move { Ok(true) }
        }

        fn encrypt(
            &self,
            _key_id: &str,
            plaintext: &[u8],
        ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
            let plaintext = plaintext.to_vec();
            async move { Ok(plaintext) }
        }

        fn decrypt(
            &self,
            _key_id: &str,
            ciphertext: &[u8],
        ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
            let ciphertext = ciphertext.to_vec();
            async move { Ok(ciphertext) }
        }

        fn sign(
            &self,
            _key_id: &str,
            data: &[u8],
        ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
            let data = data.to_vec();
            async move { Ok(data) }
        }

        fn verify(
            &self,
            _key_id: &str,
            _data: &[u8],
            _signature: &[u8],
        ) -> impl Future<Output = Result<bool, BearDogError>> + Send {
            async move { Ok(true) }
        }
    }

    #[tokio::test]
    async fn stub_provider_round_trip() {
        let p = StubProvider;
        assert_eq!(p.provider_id(), "stub");
        assert!(p.is_available());
        assert_eq!(p.provider_type(), HsmProviderType::Software);

        let handle = p
            .generate_key(&KeyGenParams::new(HsmAlgorithm::Aes256Gcm))
            .await
            .unwrap();
        assert_eq!(handle.key_id, "stub-key");
        assert!(!handle.hardware_backed);

        let ct = p.encrypt("stub-key", b"hello").await.unwrap();
        let pt = p.decrypt("stub-key", &ct).await.unwrap();
        assert_eq!(pt, b"hello");

        let sig = p.sign("stub-key", b"msg").await.unwrap();
        assert!(p.verify("stub-key", b"msg", &sig).await.unwrap());

        assert!(p.key_exists("stub-key").await.unwrap());
        assert!(p.delete_key("stub-key").await.is_ok());
    }
}
