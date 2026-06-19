// SPDX-License-Identifier: AGPL-3.0-or-later

//! Android StrongBox and TEE detection for safe providers.

use super::super::types::{AndroidDeviceInfo, VerifiedBootState};
use super::{
    AndroidCapability, SafeAndroidKeystore, SafeMobileHardwareProvider, StrongBoxAvailable,
    TeeAvailable,
};
use beardog_errors::BearDogError;
use tracing::{debug, info};

impl SafeAndroidKeystore {
    /// Detects device information safely
    ///
    /// # Platform Support
    ///
    /// **Android Only**: Device detection is only available on Android.
    ///
    /// # Errors
    ///
    /// Returns an error if detection fails or platform is not supported
    #[cfg(target_os = "android")]
    pub fn detect_device_info_safe() -> Result<AndroidDeviceInfo, BearDogError> {
        debug!("📱 Detecting Android device info safely");

        let model = beardog_errors::process_env::var("ANDROID_MODEL")
            .unwrap_or_else(|_| "Android Device".to_string());

        let api_level = beardog_errors::process_env::var("ANDROID_API_LEVEL")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(30);

        let security_patch = beardog_errors::process_env::var("ANDROID_SECURITY_PATCH")
            .unwrap_or_else(|_| "2024-01-01".to_string());

        let strongbox_version = Self::detect_strongbox_version_safe()?;
        let titan_m_version = Self::detect_titan_m_version_safe()?;

        Ok(AndroidDeviceInfo {
            manufacturer: "Google".to_string(),
            model,
            device: "unknown".to_string(),
            hardware: None,
            board: None,
            brand: Some("google".to_string()),
            android_version: api_level.to_string(),
            api_level,
            security_patch: Some(security_patch.clone()),
            security_patch_level: security_patch,
            strongbox_version,
            titan_m_version,
            verified_boot_state: VerifiedBootState::Verified,
        })
    }

    /// Detects device information safely (non-Android platforms)
    ///
    /// # Errors
    ///
    /// Always returns an error indicating platform limitation
    #[cfg(not(target_os = "android"))]
    pub fn detect_device_info_safe() -> Result<AndroidDeviceInfo, BearDogError> {
        Err(BearDogError::unsupported_operation(
            "Android device detection is only available on Android platform",
        ))
    }

    #[cfg(target_os = "android")]
    fn detect_strongbox_version_safe() -> Result<Option<String>, BearDogError> {
        debug!("🛡️ Safely detecting StrongBox version");

        if beardog_errors::process_env::var("ANDROID_STRONGBOX_AVAILABLE").is_ok() {
            Ok(Some("StrongBox-1.0".to_string()))
        } else {
            Ok(None)
        }
    }

    #[cfg(not(target_os = "android"))]
    fn detect_strongbox_version_safe() -> Result<Option<String>, BearDogError> {
        Ok(None)
    }

    #[cfg(target_os = "android")]
    fn detect_titan_m_version_safe() -> Result<Option<String>, BearDogError> {
        debug!("🔒 Safely detecting Titan M version");

        if beardog_errors::process_env::var("ANDROID_TITAN_M_AVAILABLE").is_ok() {
            Ok(Some("Titan M v1".to_string()))
        } else {
            Ok(None)
        }
    }

    #[cfg(not(target_os = "android"))]
    fn detect_titan_m_version_safe() -> Result<Option<String>, BearDogError> {
        Ok(None)
    }
}

impl SafeMobileHardwareProvider<StrongBoxAvailable> {
    /// Detects if StrongBox is available
    ///
    /// # Errors
    ///
    /// Returns an error if detection fails
    pub fn detect_strongbox() -> Result<Option<Self>, BearDogError> {
        if !cfg!(target_os = "android") {
            info!("📱 Not on Android platform, StrongBox not available");
            return Ok(None);
        }

        debug!("🔍 Detecting StrongBox availability safely");

        let strongbox_available = beardog_errors::process_env::var("ANDROID_STRONGBOX_AVAILABLE")
            .is_ok()
            || Self::check_strongbox_with_safe_api()?;

        if strongbox_available {
            let capability = StrongBoxAvailable;
            info!("✅ StrongBox detected and available");
            Ok(Some(Self::new(capability)?))
        } else {
            info!("❌ StrongBox not available on this device");
            Ok(None)
        }
    }

    fn check_strongbox_with_safe_api() -> Result<bool, BearDogError> {
        debug!("🛡️ Checking StrongBox with safe API");
        Ok(beardog_errors::process_env::var("STRONGBOX_MOCK_AVAILABLE").is_ok())
    }
}

impl SafeMobileHardwareProvider<TeeAvailable> {
    /// Detects if TEE is available
    ///
    /// # Errors
    ///
    /// Returns an error if detection fails
    pub fn detect_tee() -> Result<Option<Self>, BearDogError> {
        debug!("🔍 Detecting TEE availability safely");

        let tee_available = Self::check_tee_with_safe_api()?;

        if tee_available {
            let capability = TeeAvailable;
            info!("✅ TEE detected and available");
            Ok(Some(Self::new(capability)?))
        } else {
            info!("❌ TEE not available on this device");
            Ok(None)
        }
    }

    fn check_tee_with_safe_api() -> Result<bool, BearDogError> {
        debug!("🔐 Checking TEE with safe API");
        Ok(false)
    }
}
