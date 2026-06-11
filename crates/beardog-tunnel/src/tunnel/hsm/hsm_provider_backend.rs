// SPDX-License-Identifier: AGPL-3.0-or-later

//! Enum dispatch for [`crate::tunnel::hsm::manager::HsmProvider`].

use crate::tunnel::hsm::GenerateKeyRequest;
use crate::tunnel::hsm::manager::HsmProvider;
use crate::tunnel::hsm::manager::implementation::{HealthStatus, KeyInfo, ProviderInfo};
use crate::tunnel::hsm::software_hsm::RustSoftwareHsm;
use crate::tunnel::hsm::types::HsmKey;
use beardog_errors::BearDogError;

#[cfg(test)]
use crate::tunnel::hsm::hsm_provider_mocks::{
    DefaultManagerMockProvider, MockCloudHsm, MockHardwareHsm, MockHsmProvider, MockSoftwareHsm,
};

/// Legacy `HsmProvider` dispatch enum for the tunnel crate.
pub enum HsmProviderBackend {
    /// Production software HSM.
    RustSoftware(RustSoftwareHsm),
    #[cfg(test)]
    /// Manager unit-test mock (`manager/tests.rs`).
    Mock(MockHsmProvider),
    #[cfg(test)]
    MockHardware(MockHardwareHsm),
    #[cfg(test)]
    MockSoftware(MockSoftwareHsm),
    #[cfg(test)]
    MockCloud(MockCloudHsm),
    #[cfg(test)]
    /// `DefaultHsmManager` unit tests (`implementation.rs`).
    DefaultManagerMock(DefaultManagerMockProvider),
}

impl std::fmt::Debug for HsmProviderBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RustSoftware(_) => f.debug_tuple("RustSoftware").finish(),
            #[cfg(test)]
            Self::Mock(_) => f.debug_tuple("Mock").finish(),
            #[cfg(test)]
            Self::MockHardware(_) => f.debug_tuple("MockHardware").finish(),
            #[cfg(test)]
            Self::MockSoftware(_) => f.debug_tuple("MockSoftware").finish(),
            #[cfg(test)]
            Self::MockCloud(_) => f.debug_tuple("MockCloud").finish(),
            #[cfg(test)]
            Self::DefaultManagerMock(_) => f.debug_tuple("DefaultManagerMock").finish(),
        }
    }
}

impl HsmProvider for HsmProviderBackend {
    async fn get_info(&self) -> Result<ProviderInfo, BearDogError> {
        match self {
            Self::RustSoftware(p) => p.get_info().await,
            #[cfg(test)]
            Self::Mock(m) => m.get_info().await,
            #[cfg(test)]
            Self::MockHardware(m) => m.get_info().await,
            #[cfg(test)]
            Self::MockSoftware(m) => m.get_info().await,
            #[cfg(test)]
            Self::MockCloud(m) => m.get_info().await,
            #[cfg(test)]
            Self::DefaultManagerMock(m) => m.get_info().await,
        }
    }

    async fn generate_key(&self, request: GenerateKeyRequest) -> Result<HsmKey, BearDogError> {
        match self {
            Self::RustSoftware(p) => p.generate_key(request).await,
            #[cfg(test)]
            Self::Mock(m) => m.generate_key(request).await,
            #[cfg(test)]
            Self::MockHardware(m) => m.generate_key(request).await,
            #[cfg(test)]
            Self::MockSoftware(m) => m.generate_key(request).await,
            #[cfg(test)]
            Self::MockCloud(m) => m.generate_key(request).await,
            #[cfg(test)]
            Self::DefaultManagerMock(m) => m.generate_key(request).await,
        }
    }

    async fn sign(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        let key_id = key_id.to_string();
        let data = data.to_vec();
        match self {
            Self::RustSoftware(p) => p.sign(&key_id, &data).await,
            #[cfg(test)]
            Self::Mock(m) => m.sign(&key_id, &data).await,
            #[cfg(test)]
            Self::MockHardware(m) => m.sign(&key_id, &data).await,
            #[cfg(test)]
            Self::MockSoftware(m) => m.sign(&key_id, &data).await,
            #[cfg(test)]
            Self::MockCloud(m) => m.sign(&key_id, &data).await,
            #[cfg(test)]
            Self::DefaultManagerMock(m) => m.sign(&key_id, &data).await,
        }
    }

    async fn verify(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
        let key_id = key_id.to_string();
        let data = data.to_vec();
        let signature = signature.to_vec();
        match self {
            Self::RustSoftware(p) => p.verify(&key_id, &data, &signature).await,
            #[cfg(test)]
            Self::Mock(m) => m.verify(&key_id, &data, &signature).await,
            #[cfg(test)]
            Self::MockHardware(m) => m.verify(&key_id, &data, &signature).await,
            #[cfg(test)]
            Self::MockSoftware(m) => m.verify(&key_id, &data, &signature).await,
            #[cfg(test)]
            Self::MockCloud(m) => m.verify(&key_id, &data, &signature).await,
            #[cfg(test)]
            Self::DefaultManagerMock(m) => m.verify(&key_id, &data, &signature).await,
        }
    }

    async fn encrypt(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        let key_id = key_id.to_string();
        let data = data.to_vec();
        match self {
            Self::RustSoftware(p) => p.encrypt(&key_id, &data).await,
            #[cfg(test)]
            Self::Mock(m) => m.encrypt(&key_id, &data).await,
            #[cfg(test)]
            Self::MockHardware(m) => m.encrypt(&key_id, &data).await,
            #[cfg(test)]
            Self::MockSoftware(m) => m.encrypt(&key_id, &data).await,
            #[cfg(test)]
            Self::MockCloud(m) => m.encrypt(&key_id, &data).await,
            #[cfg(test)]
            Self::DefaultManagerMock(m) => m.encrypt(&key_id, &data).await,
        }
    }

    async fn decrypt(&self, key_id: &str, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        let key_id = key_id.to_string();
        let ciphertext = ciphertext.to_vec();
        match self {
            Self::RustSoftware(p) => p.decrypt(&key_id, &ciphertext).await,
            #[cfg(test)]
            Self::Mock(m) => m.decrypt(&key_id, &ciphertext).await,
            #[cfg(test)]
            Self::MockHardware(m) => m.decrypt(&key_id, &ciphertext).await,
            #[cfg(test)]
            Self::MockSoftware(m) => m.decrypt(&key_id, &ciphertext).await,
            #[cfg(test)]
            Self::MockCloud(m) => m.decrypt(&key_id, &ciphertext).await,
            #[cfg(test)]
            Self::DefaultManagerMock(m) => m.decrypt(&key_id, &ciphertext).await,
        }
    }

    async fn import_key(&self, key_data: &[u8], key_id: &str) -> Result<HsmKey, BearDogError> {
        let key_data = key_data.to_vec();
        let key_id = key_id.to_string();
        match self {
            Self::RustSoftware(p) => p.import_key(&key_data, &key_id).await,
            #[cfg(test)]
            Self::Mock(m) => m.import_key(&key_data, &key_id).await,
            #[cfg(test)]
            Self::MockHardware(m) => m.import_key(&key_data, &key_id).await,
            #[cfg(test)]
            Self::MockSoftware(m) => m.import_key(&key_data, &key_id).await,
            #[cfg(test)]
            Self::MockCloud(m) => m.import_key(&key_data, &key_id).await,
            #[cfg(test)]
            Self::DefaultManagerMock(m) => m.import_key(&key_data, &key_id).await,
        }
    }

    async fn delete_key(&self, key_id: &str) -> Result<(), BearDogError> {
        let key_id = key_id.to_string();
        match self {
            Self::RustSoftware(p) => p.delete_key(&key_id).await,
            #[cfg(test)]
            Self::Mock(m) => m.delete_key(&key_id).await,
            #[cfg(test)]
            Self::MockHardware(m) => m.delete_key(&key_id).await,
            #[cfg(test)]
            Self::MockSoftware(m) => m.delete_key(&key_id).await,
            #[cfg(test)]
            Self::MockCloud(m) => m.delete_key(&key_id).await,
            #[cfg(test)]
            Self::DefaultManagerMock(m) => m.delete_key(&key_id).await,
        }
    }

    async fn get_key_info(&self, key_id: &str) -> Result<KeyInfo, BearDogError> {
        let key_id = key_id.to_string();
        match self {
            Self::RustSoftware(p) => p.get_key_info(&key_id).await,
            #[cfg(test)]
            Self::Mock(m) => m.get_key_info(&key_id).await,
            #[cfg(test)]
            Self::MockHardware(m) => m.get_key_info(&key_id).await,
            #[cfg(test)]
            Self::MockSoftware(m) => m.get_key_info(&key_id).await,
            #[cfg(test)]
            Self::MockCloud(m) => m.get_key_info(&key_id).await,
            #[cfg(test)]
            Self::DefaultManagerMock(m) => m.get_key_info(&key_id).await,
        }
    }

    async fn health_check(&self) -> Result<HealthStatus, BearDogError> {
        match self {
            Self::RustSoftware(p) => p.health_check().await,
            #[cfg(test)]
            Self::Mock(m) => m.health_check().await,
            #[cfg(test)]
            Self::MockHardware(m) => m.health_check().await,
            #[cfg(test)]
            Self::MockSoftware(m) => m.health_check().await,
            #[cfg(test)]
            Self::MockCloud(m) => m.health_check().await,
            #[cfg(test)]
            Self::DefaultManagerMock(m) => m.health_check().await,
        }
    }

    fn is_available(&self) -> bool {
        match self {
            Self::RustSoftware(p) => p.is_available(),
            #[cfg(test)]
            Self::Mock(m) => m.is_available(),
            #[cfg(test)]
            Self::MockHardware(m) => m.is_available(),
            #[cfg(test)]
            Self::MockSoftware(m) => m.is_available(),
            #[cfg(test)]
            Self::MockCloud(m) => m.is_available(),
            #[cfg(test)]
            Self::DefaultManagerMock(m) => m.is_available(),
        }
    }
}
