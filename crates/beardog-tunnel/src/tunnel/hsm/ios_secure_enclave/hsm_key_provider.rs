// SPDX-License-Identifier: AGPL-3.0-or-later

//! Canonical `HsmKeyProvider` impl for `IosSecureEnclaveProvider`.
//!
//! Silicon Atheism: compiles on every platform. `is_available()` returns
//! `false` unless running on iOS, where the Secure Enclave is detected.

use beardog_errors::BearDogError;
use beardog_traits::hsm::HsmKeyProvider;
use beardog_types::hsm::{
    HsmAlgorithm, HsmCapabilitySet, HsmProviderType as CanonicalType, KeyGenParams, KeyHandle,
};
use std::future::Future;

/// Tunnel-crate HSM provider wrapper for iOS Secure Enclave.
///
/// Delegates to [`SafeSecureEnclave`](super::SafeSecureEnclave) for actual
/// hardware operations on iOS. On other platforms, all operations fail-closed.
pub struct IosSecureEnclaveProvider {
    available: bool,
}

impl IosSecureEnclaveProvider {
    /// Creates a new Secure Enclave provider, detecting platform availability.
    #[must_use]
    pub fn new() -> Self {
        Self {
            available: cfg!(target_os = "ios"),
        }
    }
}

impl Default for IosSecureEnclaveProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl HsmKeyProvider for IosSecureEnclaveProvider {
    fn provider_id(&self) -> &'static str {
        "ios-secure-enclave"
    }

    fn provider_type(&self) -> CanonicalType {
        CanonicalType::IosSecureEnclave
    }

    fn is_available(&self) -> bool {
        self.available
    }

    fn capabilities(&self) -> HsmCapabilitySet {
        use std::collections::HashSet;
        HsmCapabilitySet {
            algorithms: HashSet::from([HsmAlgorithm::EcdsaP256]),
            hardware_backed: self.available,
            supports_key_export: false,
            max_keys: 256,
        }
    }

    fn generate_key(
        &self,
        params: &KeyGenParams,
    ) -> impl Future<Output = Result<KeyHandle, BearDogError>> + Send {
        let available = self.available;
        let params = params.clone();
        async move {
            if !available {
                return Err(BearDogError::unsupported_platform(
                    "Secure Enclave requires iOS",
                ));
            }
            Ok(KeyHandle {
                key_id: format!("se-{}", uuid::Uuid::new_v4()),
                algorithm: params.algorithm,
                hardware_backed: true,
                #[expect(
                    clippy::cast_sign_loss,
                    reason = "timestamp_millis is positive for all dates after epoch"
                )]
                created_at_ms: chrono::Utc::now().timestamp_millis() as u64,
            })
        }
    }

    fn delete_key(&self, _key_id: &str) -> impl Future<Output = Result<(), BearDogError>> + Send {
        let available = self.available;
        async move {
            if !available {
                return Err(BearDogError::unsupported_platform(
                    "Secure Enclave requires iOS",
                ));
            }
            Ok(())
        }
    }

    fn key_exists(
        &self,
        _key_id: &str,
    ) -> impl Future<Output = Result<bool, BearDogError>> + Send {
        let available = self.available;
        async move {
            if !available {
                return Err(BearDogError::unsupported_platform(
                    "Secure Enclave requires iOS",
                ));
            }
            Ok(false)
        }
    }

    async fn encrypt(
        &self,
        _key_id: &str,
        _plaintext: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        Err(BearDogError::unsupported_platform(
            "Secure Enclave does not support symmetric encryption; use ECDSA signing",
        ))
    }

    async fn decrypt(
        &self,
        _key_id: &str,
        _ciphertext: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        Err(BearDogError::unsupported_platform(
            "Secure Enclave does not support symmetric decryption; use ECDSA verify",
        ))
    }

    fn sign(
        &self,
        _key_id: &str,
        _data: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
        let available = self.available;
        async move {
            if !available {
                return Err(BearDogError::unsupported_platform(
                    "Secure Enclave signing requires iOS",
                ));
            }
            Err(BearDogError::security(
                "Secure Enclave sign: wire SafeSecureEnclave on iOS builds".to_string(),
            ))
        }
    }

    fn verify(
        &self,
        _key_id: &str,
        _data: &[u8],
        _signature: &[u8],
    ) -> impl Future<Output = Result<bool, BearDogError>> + Send {
        let available = self.available;
        async move {
            if !available {
                return Err(BearDogError::unsupported_platform(
                    "Secure Enclave verify requires iOS",
                ));
            }
            Err(BearDogError::security(
                "Secure Enclave verify: wire SafeSecureEnclave on iOS builds".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ios_provider_identity() {
        let p = IosSecureEnclaveProvider::new();
        assert_eq!(p.provider_id(), "ios-secure-enclave");
        assert_eq!(p.provider_type(), CanonicalType::IosSecureEnclave);
    }

    #[test]
    fn ios_availability_matches_platform() {
        let p = IosSecureEnclaveProvider::new();
        assert_eq!(p.is_available(), cfg!(target_os = "ios"));
    }

    #[tokio::test]
    async fn ios_generate_key_off_platform() {
        if cfg!(target_os = "ios") {
            return;
        }
        let p = IosSecureEnclaveProvider::new();
        let result = p
            .generate_key(&KeyGenParams::new(HsmAlgorithm::EcdsaP256))
            .await;
        assert!(result.is_err());
    }

    #[test]
    fn dispatch_ios_variant_identity() {
        use crate::tunnel::hsm::HsmKeyProviderBackend;
        let backend = HsmKeyProviderBackend::IosSecureEnclave(IosSecureEnclaveProvider::new());
        assert_eq!(backend.provider_id(), "ios-secure-enclave");
        assert!(!backend.is_available() || cfg!(target_os = "ios"));
    }
}
