// SPDX-License-Identifier: AGPL-3.0-or-later

use super::types::{
    KeyManagementCapability, KeyMetadata, KeySpec, KmsCapabilities, KmsError, KmsHealthStatus,
};
use crate::canonical::types::ids::KeyId;
use async_trait::async_trait;
use std::sync::Arc;

/// Software HSM provider using `SecureSoftwareHsm`
///
/// This implementation provides a pure-Rust HSM for environments without
/// hardware security modules. All keys are encrypted at rest and operations
/// use modern cryptography from the `RustCrypto` ecosystem.
pub struct SoftwareHsmProvider {
    /// Inner secure software HSM implementation
    inner: Arc<super::super::software_hsm_impl::SecureSoftwareHsm>,
}

impl std::fmt::Debug for SoftwareHsmProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SoftwareHsmProvider")
            .field("inner", &"<SecureSoftwareHsm>")
            .finish()
    }
}

impl SoftwareHsmProvider {
    /// Create new Software HSM provider
    ///
    /// # Errors
    /// Returns an error if HSM initialization fails (extremely rare - only if OS entropy unavailable)
    pub fn new() -> Result<Self, KmsError> {
        Ok(Self {
            inner: Arc::new(super::super::software_hsm_impl::SecureSoftwareHsm::new()?),
        })
    }

    /// Create with configuration
    ///
    /// # Errors
    /// Returns an error if HSM initialization fails
    #[allow(
        dead_code,
        reason = "Public API hook for tunable software HSM; not referenced internally yet"
    )]
    pub async fn with_config(
        #[allow(
            unused_variables,
            reason = "Parameter reserved for future SecureSoftwareHsm options"
        )]
        config: std::collections::HashMap<String, String>,
    ) -> Result<Self, KmsError> {
        // For now, configuration is not used - SecureSoftwareHsm always uses secure defaults
        // Future: Could add options for key derivation params, memory limits, etc.
        Self::new()
    }
}

// Production implementation delegating to SecureSoftwareHsm
#[async_trait]
impl KeyManagementCapability for SoftwareHsmProvider {
    async fn encrypt(&self, plaintext: &[u8], key_id: &KeyId) -> Result<Vec<u8>, KmsError> {
        self.inner.encrypt(plaintext, key_id).await
    }

    async fn decrypt(&self, ciphertext: &[u8], key_id: &KeyId) -> Result<Vec<u8>, KmsError> {
        self.inner.decrypt(ciphertext, key_id).await
    }

    async fn generate_key(&self, spec: KeySpec) -> Result<KeyId, KmsError> {
        self.inner.generate_key(spec).await
    }

    async fn sign(&self, data: &[u8], key_id: &KeyId) -> Result<Vec<u8>, KmsError> {
        self.inner.sign(data, key_id).await
    }

    async fn verify(
        &self,
        data: &[u8],
        signature: &[u8],
        key_id: &KeyId,
    ) -> Result<bool, KmsError> {
        self.inner.verify(data, signature, key_id).await
    }

    async fn generate_random(&self, num_bytes: usize) -> Result<Vec<u8>, KmsError> {
        self.inner.generate_random(num_bytes).await
    }

    async fn get_public_key(&self, key_id: &KeyId) -> Result<Vec<u8>, KmsError> {
        self.inner.get_public_key(key_id).await
    }

    async fn delete_key(&self, key_id: &KeyId) -> Result<(), KmsError> {
        self.inner.delete_key(key_id).await
    }

    async fn list_keys(&self) -> Result<Vec<KeyMetadata>, KmsError> {
        self.inner.list_keys().await
    }

    async fn rotate_key(&self, key_id: &KeyId) -> Result<KeyId, KmsError> {
        self.inner.rotate_key(key_id).await
    }

    async fn health_check(&self) -> Result<KmsHealthStatus, KmsError> {
        self.inner.health_check().await
    }

    fn provider_name(&self) -> &str {
        self.inner.provider_name()
    }

    fn capabilities(&self) -> KmsCapabilities {
        self.inner.capabilities()
    }
}
