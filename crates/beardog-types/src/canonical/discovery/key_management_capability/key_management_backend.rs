// SPDX-License-Identifier: AGPL-3.0-or-later

//! Concrete key-management backends (enum dispatch).

use super::software_hsm_provider::SoftwareHsmProvider;
use super::types::{
    KeyManagementCapability, KeyMetadata, KeySpec, KmsCapabilities, KmsError, KmsHealthStatus,
};
use crate::canonical::discovery::software_hsm_impl::SecureSoftwareHsm;
use crate::canonical::types::ids::KeyId;

/// Resolved key-management backend (no `dyn` dispatch).
#[derive(Debug)]
pub enum KeyManagementBackend {
    /// Software HSM provider wrapping [`SecureSoftwareHsm`].
    SoftwareHsm(SoftwareHsmProvider),
    /// Direct secure software HSM implementation.
    SecureSoftware(SecureSoftwareHsm),
}

impl KeyManagementCapability for KeyManagementBackend {
    async fn encrypt(&self, plaintext: &[u8], key_id: &KeyId) -> Result<Vec<u8>, KmsError> {
        match self {
            Self::SoftwareHsm(p) => p.encrypt(plaintext, key_id).await,
            Self::SecureSoftware(s) => s.encrypt(plaintext, key_id).await,
        }
    }

    async fn decrypt(&self, ciphertext: &[u8], key_id: &KeyId) -> Result<Vec<u8>, KmsError> {
        match self {
            Self::SoftwareHsm(p) => p.decrypt(ciphertext, key_id).await,
            Self::SecureSoftware(s) => s.decrypt(ciphertext, key_id).await,
        }
    }

    async fn generate_key(&self, spec: KeySpec) -> Result<KeyId, KmsError> {
        match self {
            Self::SoftwareHsm(p) => p.generate_key(spec).await,
            Self::SecureSoftware(s) => s.generate_key(spec).await,
        }
    }

    async fn sign(&self, data: &[u8], key_id: &KeyId) -> Result<Vec<u8>, KmsError> {
        match self {
            Self::SoftwareHsm(p) => p.sign(data, key_id).await,
            Self::SecureSoftware(s) => s.sign(data, key_id).await,
        }
    }

    async fn verify(
        &self,
        data: &[u8],
        signature: &[u8],
        key_id: &KeyId,
    ) -> Result<bool, KmsError> {
        match self {
            Self::SoftwareHsm(p) => p.verify(data, signature, key_id).await,
            Self::SecureSoftware(s) => s.verify(data, signature, key_id).await,
        }
    }

    async fn generate_random(&self, num_bytes: usize) -> Result<Vec<u8>, KmsError> {
        match self {
            Self::SoftwareHsm(p) => p.generate_random(num_bytes).await,
            Self::SecureSoftware(s) => s.generate_random(num_bytes).await,
        }
    }

    async fn get_public_key(&self, key_id: &KeyId) -> Result<Vec<u8>, KmsError> {
        match self {
            Self::SoftwareHsm(p) => p.get_public_key(key_id).await,
            Self::SecureSoftware(s) => s.get_public_key(key_id).await,
        }
    }

    async fn delete_key(&self, key_id: &KeyId) -> Result<(), KmsError> {
        match self {
            Self::SoftwareHsm(p) => p.delete_key(key_id).await,
            Self::SecureSoftware(s) => s.delete_key(key_id).await,
        }
    }

    async fn list_keys(&self) -> Result<Vec<KeyMetadata>, KmsError> {
        match self {
            Self::SoftwareHsm(p) => p.list_keys().await,
            Self::SecureSoftware(s) => s.list_keys().await,
        }
    }

    async fn rotate_key(&self, key_id: &KeyId) -> Result<KeyId, KmsError> {
        match self {
            Self::SoftwareHsm(p) => p.rotate_key(key_id).await,
            Self::SecureSoftware(s) => s.rotate_key(key_id).await,
        }
    }

    async fn health_check(&self) -> Result<KmsHealthStatus, KmsError> {
        match self {
            Self::SoftwareHsm(p) => p.health_check().await,
            Self::SecureSoftware(s) => s.health_check().await,
        }
    }

    fn provider_name(&self) -> &str {
        match self {
            Self::SoftwareHsm(p) => p.provider_name(),
            Self::SecureSoftware(s) => s.provider_name(),
        }
    }

    fn capabilities(&self) -> KmsCapabilities {
        match self {
            Self::SoftwareHsm(p) => p.capabilities(),
            Self::SecureSoftware(s) => s.capabilities(),
        }
    }
}
