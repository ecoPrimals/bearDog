// SPDX-License-Identifier: AGPL-3.0-or-later

//! Universal HSM Entropy Orchestrator Implementation
//!
//! # Entropy source honesty
//!
//! HSM device selection is implemented, but hardware RNG integration is not yet
//! wired for FIDO2, Android `StrongBox`, or iOS Secure Enclave. Until Phase 2 lands,
//! [`HsmEntropyOrchestrator::generate_from_hsm`] fills bytes from the OS CSPRNG
//! and reports `source: "os_rng"`, `device_used: "os_rng_fallback"`, and
//! `hardware_backed: false` rather than mislabeling software entropy as hardware.

use super::config::OrchestratorConfig;
use super::discovery::discover_providers;
use beardog_errors::BearDogError;

#[cfg(feature = "fido2")]
use crate::hsm::fido2::multi_credential_provider::Fido2MultiCredentialProvider;
#[cfg(all(feature = "mobile", target_os = "ios"))]
use std::sync::Arc;

/// Universal HSM entropy orchestrator
///
/// Connects all available hardware security modules (FIDO2, Android `StrongBox`,
/// iOS Secure Enclave) to `BearDog`'s entropy hierarchy system.
///
/// **Note:** Until hardware RNG providers are integrated, entropy bytes come from
/// the OS CSPRNG fallback path and results are labeled accordingly.
pub struct HsmEntropyOrchestrator {
    /// Available FIDO2 providers
    #[cfg(feature = "fido2")]
    pub(super) fido2_providers: Vec<Fido2MultiCredentialProvider>,

    /// Android StrongBox provider (if available)
    #[cfg(all(feature = "mobile", target_os = "android"))]
    pub(super) android_provider:
        Option<crate::hsm::android_strongbox::StrongBoxMultiCredentialProvider>,

    /// iOS Secure Enclave provider (if available)
    #[cfg(all(feature = "mobile", target_os = "ios"))]
    pub(super) ios_provider: Option<Arc<parking_lot::RwLock<()>>>,

    /// Configuration - used for Phase 2 orchestration logic
    pub(super) _config: OrchestratorConfig,
}

impl HsmEntropyOrchestrator {
    /// Initialize orchestrator and discover all available HSMs
    ///
    /// This method will:
    /// 1. Discover FIDO2 devices (if feature enabled)
    /// 2. Check for Android `StrongBox` (if on Android)
    /// 3. Check for iOS Secure Enclave (if on iOS)
    ///
    /// # Errors
    ///
    /// Returns an error if orchestrator initialization fails.
    pub async fn new() -> Result<Self, BearDogError> {
        Self::new_with_config(OrchestratorConfig::default()).await
    }

    /// Initialize with custom configuration
    ///
    /// # Errors
    ///
    /// Returns an error if orchestrator initialization fails.
    pub async fn new_with_config(config: OrchestratorConfig) -> Result<Self, BearDogError> {
        #[cfg_attr(
            not(any(
                feature = "fido2",
                all(feature = "mobile", target_os = "android"),
                all(feature = "mobile", target_os = "ios")
            )),
            allow(unused_variables)
        )]
        let providers = discover_providers().await?;

        Ok(Self {
            #[cfg(feature = "fido2")]
            fido2_providers: providers.fido2_providers,
            #[cfg(all(feature = "mobile", target_os = "android"))]
            android_provider: providers.android_provider,
            #[cfg(all(feature = "mobile", target_os = "ios"))]
            ios_provider: providers.ios_provider,
            _config: config,
        })
    }
}
