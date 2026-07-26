// SPDX-License-Identifier: AGPL-3.0-or-later

//! HSM device discovery and listing.

use super::orchestrator::HsmEntropyOrchestrator;
use super::types::HsmDeviceInfo;

#[cfg(any(
    feature = "fido2",
    all(feature = "mobile", target_os = "android"),
    all(feature = "mobile", target_os = "ios")
))]
use super::types::{HsmDeviceType, SecurityLevel};
use beardog_errors::BearDogError;
use tracing::{debug, info};

#[cfg(any(
    feature = "fido2",
    all(feature = "mobile", target_os = "android"),
    all(feature = "mobile", target_os = "ios")
))]
use tracing::warn;

#[cfg(feature = "fido2")]
use crate::hsm::fido2::{
    discover_fido2_devices, multi_credential_provider::Fido2MultiCredentialProvider,
};

/// Internal enum for HSM source selection
#[derive(Debug)]
pub(super) enum HsmSource {
    /// FIDO2 device
    #[cfg(feature = "fido2")]
    Fido2(usize), // Index in fido2_providers vec

    /// Android StrongBox
    #[cfg(all(feature = "mobile", target_os = "android"))]
    Android,

    /// iOS Secure Enclave
    #[cfg(all(feature = "mobile", target_os = "ios"))]
    IOS,
}

/// Providers discovered during orchestrator initialization.
pub(super) struct DiscoveredProviders {
    /// Available FIDO2 providers
    #[cfg(feature = "fido2")]
    pub(super) fido2_providers: Vec<Fido2MultiCredentialProvider>,

    /// Android StrongBox provider (if available)
    #[cfg(all(feature = "mobile", target_os = "android"))]
    pub(super) android_provider:
        Option<crate::hsm::android_strongbox::StrongBoxMultiCredentialProvider>,

    /// iOS Secure Enclave provider (if available)
    #[cfg(all(feature = "mobile", target_os = "ios"))]
    pub(super) ios_provider: Option<std::sync::Arc<std::sync::RwLock<()>>>, // PHASE-2(iOS): Replace with actual iOS provider once types.rs fixed
}

/// Discover and initialize all available HSM providers.
pub(super) async fn discover_providers() -> Result<DiscoveredProviders, BearDogError> {
    info!("🌐 Initializing Universal HSM Entropy Orchestrator");

    // Discover FIDO2 devices
    #[cfg(feature = "fido2")]
    let fido2_providers = {
        info!("🔍 Discovering FIDO2 devices...");
        match discover_fido2_devices().await {
            Ok(devices) => {
                info!("✅ Found {} FIDO2 device(s)", devices.len());
                let mut providers = Vec::new();
                for device_info in devices {
                    match Fido2MultiCredentialProvider::new(device_info.clone(), None).await {
                        Ok(provider) => {
                            info!("Initialized FIDO2 provider: {} {}", device_info.manufacturer, device_info.product);
                            providers.push(provider);
                        }
                        Err(e) => {
                            warn!("Failed to init FIDO2 provider for {}: {}", device_info.product, e);
                        }
                    }
                }
                providers
            }
            Err(e) => {
                warn!("FIDO2 discovery failed: {}", e);
                Vec::new()
            }
        }
    };

    // Check for Android StrongBox
    #[cfg(all(feature = "mobile", target_os = "android"))]
    let android_provider = {
        info!("🔍 Checking for Android StrongBox...");
        // PHASE-2(Android-JNI): Implement Android StrongBox provider initialization
        // This requires ndk-context which is only available in Android app context
        warn!("ℹ️  Android StrongBox integration pending (requires app context)");
        None
    };

    // Check for iOS Secure Enclave
    #[cfg(all(feature = "mobile", target_os = "ios"))]
    let ios_provider = {
        info!("🔍 Checking for iOS Secure Enclave...");
        // PHASE-2(iOS): Implement iOS Secure Enclave provider detection
        warn!("⚠️  iOS Secure Enclave provider not yet fully integrated");
        None
    };

    #[cfg(feature = "fido2")]
    let fido2_device_count = fido2_providers.len();
    #[cfg(not(feature = "fido2"))]
    let fido2_device_count = 0usize;

    #[cfg(all(feature = "mobile", target_os = "android"))]
    let android_device_count = usize::from(android_provider.is_some());
    #[cfg(not(all(feature = "mobile", target_os = "android")))]
    let android_device_count = 0usize;

    #[cfg(all(feature = "mobile", target_os = "ios"))]
    let ios_device_count = usize::from(ios_provider.is_some());
    #[cfg(not(all(feature = "mobile", target_os = "ios")))]
    let ios_device_count = 0usize;

    let total_devices = fido2_device_count + android_device_count + ios_device_count;

    info!(
        "🎯 Orchestrator initialized with {} HSM device(s)",
        total_devices
    );

    Ok(DiscoveredProviders {
        #[cfg(feature = "fido2")]
        fido2_providers,
        #[cfg(all(feature = "mobile", target_os = "android"))]
        android_provider,
        #[cfg(all(feature = "mobile", target_os = "ios"))]
        ios_provider,
    })
}

impl HsmEntropyOrchestrator {
    /// Get list of available HSM devices
    ///
    /// Returns device information for user selection or display.
    pub fn list_available_devices(&self) -> Vec<HsmDeviceInfo> {
        #[cfg_attr(
            not(any(feature = "fido2", target_os = "android", target_os = "ios")),
            allow(unused_mut)
        )]
        let mut devices = Vec::new();

        // Add FIDO2 devices
        #[cfg(feature = "fido2")]
        for (idx, _provider) in self.fido2_providers.iter().enumerate() {
            devices.push(HsmDeviceInfo {
                device_type: HsmDeviceType::Fido2,
                device_id: format!("fido2_{idx}"),
                name: format!("FIDO2 Device #{}", idx + 1),
                security_level: SecurityLevel::Hardware,
                biometric_capable: true, // Assume most FIDO2 devices support user verification
            });
        }

        // Add Android StrongBox
        #[cfg(all(feature = "mobile", target_os = "android"))]
        if self.android_provider.is_some() {
            devices.push(HsmDeviceInfo {
                device_type: HsmDeviceType::AndroidStrongBox,
                device_id: "android_strongbox".to_string(),
                name: "Android StrongBox".to_string(),
                security_level: SecurityLevel::StrongBox,
                biometric_capable: true,
            });
        }

        // Add iOS Secure Enclave
        #[cfg(all(feature = "mobile", target_os = "ios"))]
        if self.ios_provider.is_some() {
            devices.push(HsmDeviceInfo {
                device_type: HsmDeviceType::IOSSecureEnclave,
                device_id: "ios_secure_enclave".to_string(),
                name: "iOS Secure Enclave".to_string(),
                security_level: SecurityLevel::StrongBox,
                biometric_capable: true,
            });
        }

        debug!("📋 Listed {} available HSM device(s)", devices.len());
        devices
    }
}
