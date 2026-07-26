// SPDX-License-Identifier: AGPL-3.0-or-later

//! Enum dispatch for [`beardog_traits::hsm::HsmKeyProvider`].

use beardog_errors::BearDogError;
use beardog_traits::hsm::HsmKeyProvider;
use beardog_types::hsm::{HsmCapabilitySet, HsmProviderType, KeyGenParams, KeyHandle};
use std::future::Future;

use crate::tunnel::hsm::android_strongbox::AndroidStrongBoxHsm;
use crate::tunnel::hsm::linux_secret_service::LinuxSecretServiceHsm;
use crate::tunnel::hsm::software_hsm::RustSoftwareHsm;
use crate::tunnel::hsm::windows_dpapi::WindowsDpapiHsm;

/// Canonical `HsmKeyProvider` dispatch enum for the tunnel crate.
pub enum HsmKeyProviderBackend {
    /// In-process Rust/software HSM.
    Software(RustSoftwareHsm),
    /// Android StrongBox (hardware).
    AndroidStrongBox(AndroidStrongBoxHsm),
    /// Windows DPAPI (keys bound to user/machine credentials).
    WindowsDpapi(WindowsDpapiHsm),
    /// Linux Secret Service (GNOME Keyring / `KWallet` via D-Bus).
    LinuxSecretService(LinuxSecretServiceHsm),
    /// Test double: software-like stub.
    #[cfg(test)]
    StubSoftware(tests::FakeSwProvider),
    /// Test double: hardware-like stub.
    #[cfg(test)]
    StubHardware(tests::FakeHwProvider),
}

impl HsmKeyProvider for HsmKeyProviderBackend {
    fn provider_id(&self) -> &'static str {
        match self {
            Self::Software(p) => p.provider_id(),
            Self::AndroidStrongBox(p) => p.provider_id(),
            Self::WindowsDpapi(p) => p.provider_id(),
            Self::LinuxSecretService(p) => p.provider_id(),
            #[cfg(test)]
            Self::StubSoftware(p) => p.provider_id(),
            #[cfg(test)]
            Self::StubHardware(p) => p.provider_id(),
        }
    }

    fn provider_type(&self) -> HsmProviderType {
        match self {
            Self::Software(p) => p.provider_type(),
            Self::AndroidStrongBox(p) => p.provider_type(),
            Self::WindowsDpapi(p) => p.provider_type(),
            Self::LinuxSecretService(p) => p.provider_type(),
            #[cfg(test)]
            Self::StubSoftware(p) => p.provider_type(),
            #[cfg(test)]
            Self::StubHardware(p) => p.provider_type(),
        }
    }

    fn is_available(&self) -> bool {
        match self {
            Self::Software(p) => p.is_available(),
            Self::AndroidStrongBox(p) => p.is_available(),
            Self::WindowsDpapi(p) => p.is_available(),
            Self::LinuxSecretService(p) => p.is_available(),
            #[cfg(test)]
            Self::StubSoftware(p) => p.is_available(),
            #[cfg(test)]
            Self::StubHardware(p) => p.is_available(),
        }
    }

    fn capabilities(&self) -> HsmCapabilitySet {
        match self {
            Self::Software(p) => p.capabilities(),
            Self::AndroidStrongBox(p) => p.capabilities(),
            Self::WindowsDpapi(p) => p.capabilities(),
            Self::LinuxSecretService(p) => p.capabilities(),
            #[cfg(test)]
            Self::StubSoftware(p) => p.capabilities(),
            #[cfg(test)]
            Self::StubHardware(p) => p.capabilities(),
        }
    }

    fn generate_key(
        &self,
        params: &KeyGenParams,
    ) -> impl Future<Output = Result<KeyHandle, BearDogError>> + Send {
        let params = params.clone();
        async move {
            match self {
                Self::Software(p) => p.generate_key(&params).await,
                Self::AndroidStrongBox(p) => p.generate_key(&params).await,
                Self::WindowsDpapi(p) => p.generate_key(&params).await,
                Self::LinuxSecretService(p) => p.generate_key(&params).await,
                #[cfg(test)]
                Self::StubSoftware(p) => p.generate_key(&params).await,
                #[cfg(test)]
                Self::StubHardware(p) => p.generate_key(&params).await,
            }
        }
    }

    fn delete_key(&self, key_id: &str) -> impl Future<Output = Result<(), BearDogError>> + Send {
        let key_id = key_id.to_string();
        async move {
            match self {
                Self::Software(p) => p.delete_key(&key_id).await,
                Self::AndroidStrongBox(p) => p.delete_key(&key_id).await,
                Self::WindowsDpapi(p) => p.delete_key(&key_id).await,
                Self::LinuxSecretService(p) => p.delete_key(&key_id).await,
                #[cfg(test)]
                Self::StubSoftware(p) => p.delete_key(&key_id).await,
                #[cfg(test)]
                Self::StubHardware(p) => p.delete_key(&key_id).await,
            }
        }
    }

    fn key_exists(&self, key_id: &str) -> impl Future<Output = Result<bool, BearDogError>> + Send {
        let key_id = key_id.to_string();
        async move {
            match self {
                Self::Software(p) => p.key_exists(&key_id).await,
                Self::AndroidStrongBox(p) => p.key_exists(&key_id).await,
                Self::WindowsDpapi(p) => p.key_exists(&key_id).await,
                Self::LinuxSecretService(p) => p.key_exists(&key_id).await,
                #[cfg(test)]
                Self::StubSoftware(p) => p.key_exists(&key_id).await,
                #[cfg(test)]
                Self::StubHardware(p) => p.key_exists(&key_id).await,
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
        async move {
            match self {
                Self::Software(p) => p.encrypt(&key_id, &plaintext).await,
                Self::AndroidStrongBox(p) => p.encrypt(&key_id, &plaintext).await,
                Self::WindowsDpapi(p) => p.encrypt(&key_id, &plaintext).await,
                Self::LinuxSecretService(p) => p.encrypt(&key_id, &plaintext).await,
                #[cfg(test)]
                Self::StubSoftware(p) => p.encrypt(&key_id, &plaintext).await,
                #[cfg(test)]
                Self::StubHardware(p) => p.encrypt(&key_id, &plaintext).await,
            }
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
            match self {
                Self::Software(p) => p.decrypt(&key_id, &ciphertext).await,
                Self::AndroidStrongBox(p) => p.decrypt(&key_id, &ciphertext).await,
                Self::WindowsDpapi(p) => p.decrypt(&key_id, &ciphertext).await,
                Self::LinuxSecretService(p) => p.decrypt(&key_id, &ciphertext).await,
                #[cfg(test)]
                Self::StubSoftware(p) => p.decrypt(&key_id, &ciphertext).await,
                #[cfg(test)]
                Self::StubHardware(p) => p.decrypt(&key_id, &ciphertext).await,
            }
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
            match self {
                Self::Software(p) => p.sign(&key_id, &data).await,
                Self::AndroidStrongBox(p) => p.sign(&key_id, &data).await,
                Self::WindowsDpapi(p) => p.sign(&key_id, &data).await,
                Self::LinuxSecretService(p) => p.sign(&key_id, &data).await,
                #[cfg(test)]
                Self::StubSoftware(p) => p.sign(&key_id, &data).await,
                #[cfg(test)]
                Self::StubHardware(p) => p.sign(&key_id, &data).await,
            }
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
            match self {
                Self::Software(p) => p.verify(&key_id, &data, &signature).await,
                Self::AndroidStrongBox(p) => p.verify(&key_id, &data, &signature).await,
                Self::WindowsDpapi(p) => p.verify(&key_id, &data, &signature).await,
                Self::LinuxSecretService(p) => p.verify(&key_id, &data, &signature).await,
                #[cfg(test)]
                Self::StubSoftware(p) => p.verify(&key_id, &data, &signature).await,
                #[cfg(test)]
                Self::StubHardware(p) => p.verify(&key_id, &data, &signature).await,
            }
        }
    }
}

#[cfg(test)]
#[expect(
    clippy::manual_async_fn,
    reason = "trait method returns BoxFuture, cannot use async fn"
)]
pub mod tests {
    use super::*;
    use beardog_types::hsm::{
        HsmCapabilitySet, HsmProviderType as CanonicalType, KeyGenParams, KeyHandle,
    };

    pub struct FakeHwProvider;

    impl HsmKeyProvider for FakeHwProvider {
        fn provider_id(&self) -> &'static str {
            "fake-hw"
        }
        fn provider_type(&self) -> CanonicalType {
            CanonicalType::AndroidStrongBox
        }
        fn is_available(&self) -> bool {
            true
        }
        fn capabilities(&self) -> HsmCapabilitySet {
            HsmCapabilitySet::default()
        }
        fn generate_key(
            &self,
            p: &KeyGenParams,
        ) -> impl Future<Output = Result<KeyHandle, BearDogError>> + Send {
            let p = p.clone();
            async move {
                Ok(KeyHandle {
                    key_id: "hw-key".into(),
                    algorithm: p.algorithm,
                    hardware_backed: true,
                    created_at_ms: 0,
                })
            }
        }
        fn delete_key(&self, _: &str) -> impl Future<Output = Result<(), BearDogError>> + Send {
            async move { Ok(()) }
        }
        fn key_exists(&self, _: &str) -> impl Future<Output = Result<bool, BearDogError>> + Send {
            async move { Ok(true) }
        }
        fn encrypt(
            &self,
            _: &str,
            d: &[u8],
        ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
            let d = d.to_vec();
            async move { Ok(d) }
        }
        fn decrypt(
            &self,
            _: &str,
            d: &[u8],
        ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
            let d = d.to_vec();
            async move { Ok(d) }
        }
        fn sign(
            &self,
            _: &str,
            d: &[u8],
        ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
            let d = d.to_vec();
            async move { Ok(d) }
        }
        fn verify(
            &self,
            _: &str,
            _: &[u8],
            _: &[u8],
        ) -> impl Future<Output = Result<bool, BearDogError>> + Send {
            async move { Ok(true) }
        }
    }

    pub struct FakeSwProvider;

    impl HsmKeyProvider for FakeSwProvider {
        fn provider_id(&self) -> &'static str {
            "fake-sw"
        }
        fn provider_type(&self) -> CanonicalType {
            CanonicalType::Software
        }
        fn is_available(&self) -> bool {
            true
        }
        fn capabilities(&self) -> HsmCapabilitySet {
            HsmCapabilitySet::default()
        }
        fn generate_key(
            &self,
            p: &KeyGenParams,
        ) -> impl Future<Output = Result<KeyHandle, BearDogError>> + Send {
            let p = p.clone();
            async move {
                Ok(KeyHandle {
                    key_id: "sw-key".into(),
                    algorithm: p.algorithm,
                    hardware_backed: false,
                    created_at_ms: 0,
                })
            }
        }
        fn delete_key(&self, _: &str) -> impl Future<Output = Result<(), BearDogError>> + Send {
            async move { Ok(()) }
        }
        fn key_exists(&self, _: &str) -> impl Future<Output = Result<bool, BearDogError>> + Send {
            async move { Ok(true) }
        }
        fn encrypt(
            &self,
            _: &str,
            d: &[u8],
        ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
            let d = d.to_vec();
            async move { Ok(d) }
        }
        fn decrypt(
            &self,
            _: &str,
            d: &[u8],
        ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
            let d = d.to_vec();
            async move { Ok(d) }
        }
        fn sign(
            &self,
            _: &str,
            d: &[u8],
        ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send {
            let d = d.to_vec();
            async move { Ok(d) }
        }
        fn verify(
            &self,
            _: &str,
            _: &[u8],
            _: &[u8],
        ) -> impl Future<Output = Result<bool, BearDogError>> + Send {
            async move { Ok(true) }
        }
    }

    #[test]
    fn dispatch_software_variant_identity() {
        let sw = HsmKeyProviderBackend::StubSoftware(FakeSwProvider);
        assert_eq!(sw.provider_id(), "fake-sw");
        assert_eq!(sw.provider_type(), CanonicalType::Software);
        assert!(sw.is_available());
    }

    #[test]
    fn dispatch_hardware_variant_identity() {
        let hw = HsmKeyProviderBackend::StubHardware(FakeHwProvider);
        assert_eq!(hw.provider_id(), "fake-hw");
        assert_eq!(hw.provider_type(), CanonicalType::AndroidStrongBox);
        assert!(hw.is_available());
    }

    #[test]
    fn dispatch_linux_secret_service_variant_construction() {
        let ss = crate::tunnel::hsm::linux_secret_service::LinuxSecretServiceHsm::new()
            .expect("construction");
        let backend = HsmKeyProviderBackend::LinuxSecretService(ss);
        assert_eq!(backend.provider_id(), "linux-secret-service");
        assert_eq!(backend.provider_type(), CanonicalType::LinuxSecretService);
        if cfg!(target_os = "linux") {
            assert!(backend.is_available());
        } else {
            assert!(!backend.is_available());
        }
    }

    #[tokio::test]
    async fn dispatch_generate_key_through_stub() {
        use beardog_types::hsm::{HsmAlgorithm, KeyGenParams};

        let backend = HsmKeyProviderBackend::StubSoftware(FakeSwProvider);
        let handle = backend
            .generate_key(&KeyGenParams::new(HsmAlgorithm::Aes256Gcm))
            .await
            .expect("generate_key");
        assert_eq!(handle.key_id, "sw-key");
        assert!(!handle.hardware_backed);
    }
}
