// SPDX-License-Identifier: AGPL-3.0-only

//! Canonical, object-safe HSM provider trait.
//!
//! `HsmKeyProvider` is the single authoritative abstraction for all
//! key-management and crypto operations backed by either hardware
//! (Android StrongBox, iOS Secure Enclave, TPM, PKCS#11) or software
//! (RustCrypto in-process key store).
//!
//! It is intentionally lighter than the full `BearDogProvider` hierarchy so
//! it can be used as `Arc<dyn HsmKeyProvider>` for dynamic dispatch in the
//! `HsmProviderRegistry` and JSON-RPC handlers.

use async_trait::async_trait;
use beardog_errors::BearDogError;
use beardog_types::hsm::{HsmCapabilitySet, HsmProviderType, KeyGenParams, KeyHandle};

/// Object-safe HSM provider contract.
///
/// Every HSM backend (software, StrongBox, Secure Enclave, PKCS#11, TPM)
/// implements this trait. The registry holds `Arc<dyn HsmKeyProvider>` and
/// selects the best available backend at runtime.
#[async_trait]
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
    async fn generate_key(&self, params: &KeyGenParams) -> Result<KeyHandle, BearDogError>;

    /// Permanently delete the key identified by `key_id`.
    async fn delete_key(&self, key_id: &str) -> Result<(), BearDogError>;

    /// Returns `true` if a key with the given id exists in this provider.
    async fn key_exists(&self, key_id: &str) -> Result<bool, BearDogError>;

    // ── crypto operations ───────────────────────────────────────────

    /// Encrypt `plaintext` with the key identified by `key_id`.
    async fn encrypt(&self, key_id: &str, plaintext: &[u8]) -> Result<Vec<u8>, BearDogError>;

    /// Decrypt `ciphertext` previously produced by [`Self::encrypt`].
    async fn decrypt(&self, key_id: &str, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError>;

    /// Produce a signature over `data` using `key_id`.
    async fn sign(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError>;

    /// Verify `signature` over `data` against `key_id`.
    async fn verify(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::hsm::{HsmAlgorithm, HsmCapabilitySet, HsmProviderType};
    use std::collections::HashSet;

    struct StubProvider;

    #[async_trait]
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

        async fn generate_key(&self, params: &KeyGenParams) -> Result<KeyHandle, BearDogError> {
            Ok(KeyHandle {
                key_id: "stub-key".into(),
                algorithm: params.algorithm,
                hardware_backed: false,
                created_at_ms: 0,
            })
        }

        async fn delete_key(&self, _key_id: &str) -> Result<(), BearDogError> {
            Ok(())
        }

        async fn key_exists(&self, _key_id: &str) -> Result<bool, BearDogError> {
            Ok(true)
        }

        async fn encrypt(&self, _key_id: &str, plaintext: &[u8]) -> Result<Vec<u8>, BearDogError> {
            Ok(plaintext.to_vec())
        }

        async fn decrypt(&self, _key_id: &str, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError> {
            Ok(ciphertext.to_vec())
        }

        async fn sign(&self, _key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
            Ok(data.to_vec())
        }

        async fn verify(
            &self,
            _key_id: &str,
            _data: &[u8],
            _signature: &[u8],
        ) -> Result<bool, BearDogError> {
            Ok(true)
        }
    }

    #[tokio::test]
    async fn stub_provider_round_trip() {
        let p: Box<dyn HsmKeyProvider> = Box::new(StubProvider);
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

    #[test]
    fn trait_is_object_safe() {
        fn _assert_object_safe(_: &dyn HsmKeyProvider) {}
        fn _assert_arc(p: std::sync::Arc<dyn HsmKeyProvider>) {
            let _ = p.provider_id();
        }
    }
}
