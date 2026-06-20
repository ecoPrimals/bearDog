// SPDX-License-Identifier: AGPL-3.0-or-later

//! HSM device selection logic.

use super::discovery::HsmSource;
use super::orchestrator::HsmEntropyOrchestrator;
use super::types::EntropyGenerationRequest;
use beardog_errors::BearDogError;
use tracing::warn;

#[cfg(any(
    feature = "fido2",
    all(feature = "mobile", target_os = "android"),
    all(feature = "mobile", target_os = "ios")
))]
use tracing::debug;

impl HsmEntropyOrchestrator {
    /// Select best available HSM based on request and config
    pub(super) async fn select_best_hsm(
        &self,
        request: &EntropyGenerationRequest,
    ) -> Result<HsmSource, BearDogError> {
        // If preferred device specified, try to use it
        if let Some(device_id) = &request.preferred_device {
            if let Some(source) = self.find_device_by_id(device_id) {
                return Ok(source);
            }
            warn!(
                "⚠️  Preferred device '{}' not found, auto-selecting",
                device_id
            );
        }

        // Priority order: StrongBox/Secure Enclave > FIDO2

        #[cfg(all(feature = "mobile", target_os = "android"))]
        if self.android_provider.is_some() {
            debug!("🎯 Selected Android StrongBox");
            return Ok(HsmSource::Android);
        }

        #[cfg(all(feature = "mobile", target_os = "ios"))]
        if self.ios_provider.is_some() {
            debug!("🎯 Selected iOS Secure Enclave");
            return Ok(HsmSource::IOS);
        }

        #[cfg(feature = "fido2")]
        if !self.fido2_providers.is_empty() {
            // Select first FIDO2 device
            // PHASE-2(Orchestrator): Implement smarter selection based on capabilities
            debug!("🎯 Selected FIDO2 device");
            return Ok(HsmSource::Fido2(0));
        }

        Err(BearDogError::system("No HSM devices available".to_string()))
    }

    /// Find device by ID
    pub(super) fn find_device_by_id(&self, device_id: &str) -> Option<HsmSource> {
        #[cfg(feature = "fido2")]
        {
            if device_id.starts_with("fido2_")
                && let Ok(idx) = device_id.strip_prefix("fido2_")?.parse::<usize>()
                && idx < self.fido2_providers.len()
            {
                return Some(HsmSource::Fido2(idx));
            }
        }

        #[cfg(all(feature = "mobile", target_os = "android"))]
        {
            if device_id == "android_strongbox" && self.android_provider.is_some() {
                return Some(HsmSource::Android);
            }
        }

        #[cfg(not(any(feature = "fido2", target_os = "android", target_os = "ios")))]
        {
            let _ = device_id; // Suppress unused variable warning
        }

        #[cfg(all(feature = "mobile", target_os = "ios"))]
        if device_id == "ios_secure_enclave" && self.ios_provider.is_some() {
            return Some(HsmSource::IOS);
        }

        None
    }
}
