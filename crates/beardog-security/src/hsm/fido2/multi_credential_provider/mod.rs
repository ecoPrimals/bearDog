// SPDX-License-Identifier: AGPL-3.0-or-later

//! FIDO2 Multi-Credential HSM Provider Implementation
//!
//! This module implements the `MultiCredentialHsmProvider` trait for FIDO2-compliant
//! security keys (`SoloKeys`, `YubiKey` FIDO2, Nitrokey FIDO2, etc.)

mod config;
mod trait_bear_dog_provider;
mod trait_multi_credential;

#[cfg(test)]
mod tests;

pub use config::Fido2ProviderConfig;

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use beardog_errors::BearDogError;
use beardog_traits::unified::hsm_multi_credential::{CredentialInfo, CredentialRequest};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

use super::provider::Fido2HsmProvider;
use super::types::Fido2DeviceInfo;

/// FIDO2 Multi-Credential HSM Provider
///
/// Implements multi-credential operations for FIDO2-compliant security keys.
/// This works with any FIDO2 device: `SoloKeys`, `YubiKey` 5, Nitrokey FIDO2, etc.
pub struct Fido2MultiCredentialProvider {
    /// Device information
    device_info: Fido2DeviceInfo,

    /// In-memory credential storage (backed by device)
    /// In production, this would query the device directly
    credentials: Arc<RwLock<HashMap<String, CredentialInfo>>>,

    /// FIDO2-specific configuration
    /// Note: Used for future protocol operations (makeCredential, getAssertion)
    #[expect(dead_code, reason = "Config used when CTAP2 credential ops are wired")]
    config: Fido2ProviderConfig,
}

impl Fido2MultiCredentialProvider {
    /// Create a new FIDO2 multi-credential provider
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when the device does not support resident keys.
    pub async fn new(
        device_info: Fido2DeviceInfo,
        config: Option<Fido2ProviderConfig>,
    ) -> Result<Self, BearDogError> {
        info!(
            "🔐 Initializing FIDO2 Multi-Credential Provider: {} ({})",
            device_info.product, device_info.manufacturer
        );

        // Verify device supports resident keys (required for multi-credential)
        if !device_info.capabilities.resident_keys {
            return Err(BearDogError::system(format!(
                "Device '{}' does not support resident keys (required for multi-credential operations)",
                device_info.product
            )));
        }

        Ok(Self {
            device_info,
            credentials: Arc::new(RwLock::new(HashMap::new())),
            config: config.unwrap_or_default(),
        })
    }

    /// Get device information
    pub const fn device_info(&self) -> &Fido2DeviceInfo {
        &self.device_info
    }

    /// Create a [`Fido2HsmProvider`] for this device's entropy/HSM operations.
    ///
    /// # Errors
    ///
    /// Returns `BearDogError` if the provider cannot be created (e.g., FIDO2 feature not enabled).
    pub async fn hsm_provider(&self) -> Result<Fido2HsmProvider, BearDogError> {
        Fido2HsmProvider::new(self.device_info.clone()).await
    }

    /// Convert FIDO2 credential ID (bytes) to universal string ID
    fn credential_id_to_string(id: &[u8]) -> String {
        // Use base64url encoding for credential IDs
        URL_SAFE_NO_PAD.encode(id)
    }

    /// Convert universal string ID back to FIDO2 credential ID (bytes)
    fn string_to_credential_id(id: &str) -> Result<Vec<u8>, BearDogError> {
        URL_SAFE_NO_PAD
            .decode(id)
            .map_err(|e| BearDogError::system(format!("Invalid credential ID format: {e}")))
    }

    /// Send CTAP2 `MakeCredential` command
    async fn ctap2_make_credential(
        &self,
        request: &CredentialRequest,
    ) -> Result<(Vec<u8>, Vec<u8>), BearDogError> {
        debug!("Sending CTAP2 MakeCredential for role: {}", request.role);

        // PHASE-2(CTAP2): Implement CTAP2 MakeCredential command
        // Universal provider architecture is ready for CTAP2 protocol implementation
        // Implementation plan:
        // 1. Build CBOR-encoded MakeCredential request
        // 2. Send via CTAPHID transport
        // 3. Parse CBOR response
        // 4. Return (credential_id, public_key)

        Err(BearDogError::system(format!(
            "CTAP2 MakeCredential planned for Phase 2 - protocol architecture ready. \
             Role: '{}', Algorithm: '{}'",
            request.role,
            request.algorithm.as_deref().unwrap_or("ES256")
        )))
    }

    /// Send CTAP2 `GetAssertion` command
    async fn ctap2_get_assertion(
        &self,
        credential_id: &[u8],
        data: &[u8],
        require_user_presence: bool,
    ) -> Result<Vec<u8>, BearDogError> {
        debug!(
            "Sending CTAP2 GetAssertion for credential ID (len={})",
            credential_id.len()
        );

        // PHASE-2(CTAP2): Implement CTAP2 GetAssertion command
        // Universal signing interface is ready for CTAP2 protocol
        // Implementation plan: CBOR request → CTAPHID transport → Parse signature

        Err(BearDogError::system(format!(
            "CTAP2 GetAssertion planned for Phase 2 - signing architecture ready. \
             Data size: {} bytes, User presence: {}",
            data.len(),
            require_user_presence
        )))
    }

    /// Enumerate credentials using CTAP2 `CredentialManagement`
    async fn ctap2_enumerate_credentials(&self) -> Result<Vec<CredentialInfo>, BearDogError> {
        debug!("Enumerating credentials via CTAP2 CredentialManagement");

        // PHASE-2(CTAP2): Implement CTAP2 credentialManagement enumerate
        // Currently using in-memory cache - Phase 2 will query device directly
        // Universal credential management architecture supports vendor-agnostic enumeration

        // Return in-memory cache (Phase 2 will query device)
        let creds = self.credentials.read().await;
        Ok(creds.values().cloned().collect())
    }

    /// Delete credential using CTAP2 `CredentialManagement`
    async fn ctap2_delete_credential(&self, credential_id: &[u8]) -> Result<(), BearDogError> {
        debug!(
            "Deleting credential via CTAP2 (len={})",
            credential_id.len()
        );

        // PHASE-2(CTAP2): Implement CTAP2 credentialManagement delete
        // Universal credential lifecycle management architecture ready
        // Implementation plan: CBOR delete request → Device acknowledgment

        Err(BearDogError::system(
            "CTAP2 credential deletion planned for Phase 2 - lifecycle management ready"
                .to_string(),
        ))
    }

    /// Generate hardware entropy using hmac-secret extension.
    ///
    /// Requires a credential that was created with `hmac-secret` and a device PIN.
    /// The entropy is derived from the authenticator's internal HMAC secret.
    ///
    /// # Errors
    ///
    /// Returns error if device doesn't support hmac-secret or no credential is available.
    async fn ctap2_hmac_secret_entropy(&self, size: usize) -> Result<Vec<u8>, BearDogError> {
        debug!("Generating {} bytes of entropy via hmac-secret", size);

        if !self.device_info.capabilities.hmac_secret {
            return Err(BearDogError::system(
                "Device does not support hmac-secret extension".to_string(),
            ));
        }

        Err(BearDogError::requires_capability(
            "hmac-secret-credential",
            "hmac-secret entropy requires a pre-existing credential created with hmac-secret extension; \
             create one via beardog.fido2.register with extensions: [\"hmac-secret\"], then call \
             beardog.fido2.entropy with the credential_id",
        ))
    }
}
