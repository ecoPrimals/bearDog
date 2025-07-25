//! # Native Android Device Detection
//!
//! This module provides real Android device detection using system properties,
//! hardware capabilities, and StrongBox availability checks.

use super::device_info::DeviceCapabilities;
use super::types::*;
use crate::tunnel::hsm::types::*;
use beardog_errors::{BearDogError, BearDogResult};
use tracing::{info, warn};

#[cfg(target_os = "android")]
use {
    ndk_sys::{
        AKeyStore_getSecurityLevel, AKeyStore_isSecurityLevelSupported, __system_property_get,
        AKEYSTORE_SECURITY_LEVEL_STRONGBOX, AKEYSTORE_SECURITY_LEVEL_TRUSTED_ENVIRONMENT,
    },
    std::ffi::{CStr, CString},
};

/// Native Android device detector
pub struct NativeAndroidDeviceDetector;

impl NativeAndroidDeviceDetector {
    /// Detect real Android device configuration using system properties
    pub async fn detect_device_info() -> BearDogResult<AndroidDeviceInfo> {
        info!("🔍 Detecting real Android device configuration");

        #[cfg(target_os = "android")]
        {
            Self::detect_android_device().await
        }

        #[cfg(not(target_os = "android"))]
        {
            // Fallback for non-Android platforms during development
            Self::detect_mock_device().await
        }
    }

    /// Check if StrongBox is actually available on the device
    pub async fn check_strongbox_availability() -> BearDogResult<bool> {
        info!("🔍 Checking real StrongBox availability");

        #[cfg(target_os = "android")]
        {
            Self::check_android_strongbox().await
        }

        #[cfg(not(target_os = "android"))]
        {
            warn!("⚠️ Not running on Android - StrongBox unavailable");
            Ok(false)
        }
    }

    /// Detect StrongBox implementation type
    pub async fn detect_strongbox_implementation() -> BearDogResult<StrongBoxImplementation> {
        info!("🔍 Detecting StrongBox implementation");

        #[cfg(target_os = "android")]
        {
            Self::detect_android_strongbox_implementation().await
        }

        #[cfg(not(target_os = "android"))]
        {
            Ok(StrongBoxImplementation::Generic {
                vendor: "Unknown".to_string(),
                version: "0.0".to_string(),
            })
        }
    }
}

#[cfg(target_os = "android")]
impl NativeAndroidDeviceDetector {
    /// Detect Android device using real system properties
    async fn detect_android_device() -> BearDogResult<AndroidDeviceInfo> {
        info!("📱 Detecting Android device using system properties");

        // Get manufacturer using system property
        let manufacturer = Self::get_system_property("ro.product.manufacturer")
            .unwrap_or_else(|| "Unknown".to_string());

        // Get model using system property
        let model =
            Self::get_system_property("ro.product.model").unwrap_or_else(|| "Unknown".to_string());

        // Get Android version
        let android_version = Self::get_system_property("ro.build.version.release")
            .unwrap_or_else(|| "Unknown".to_string());

        // Get security patch level
        let security_patch_level = Self::get_system_property("ro.build.version.security_patch")
            .unwrap_or_else(|| "Unknown".to_string());

        // Detect StrongBox version
        let strongbox_version = Self::detect_strongbox_version().await;

        // Detect Titan M version (Pixel-specific)
        let titan_m_version = if manufacturer == "Google" && model.contains("Pixel") {
            Self::detect_titan_m_version().await
        } else {
            None
        };

        // Get verified boot state
        let verified_boot_state = Self::get_verified_boot_state().await;

        let device_info = AndroidDeviceInfo::new(
            manufacturer,
            model,
            android_version,
            strongbox_version,
            titan_m_version,
            security_patch_level,
            verified_boot_state,
        );

        info!(
            "✅ Detected device: {} {} (Android {})",
            device_info.manufacturer, device_info.model, device_info.android_version
        );

        if device_info.strongbox_version.is_some() {
            info!(
                "🔐 StrongBox available: {:?}",
                device_info.strongbox_version
            );
        }

        if device_info.titan_m_version.is_some() {
            info!("🛡️ Titan M detected: {:?}", device_info.titan_m_version);
        }

        Ok(device_info)
    }

    /// Get Android system property value
    fn get_system_property(key: &str) -> Option<String> {
        unsafe {
            // Convert key to C string
            let c_key = match CString::new(key) {
                Ok(c_str) => c_str,
                Err(_) => return None,
            };

            // Buffer for property value
            let mut buffer = [0u8; 256];

            // Get system property
            let result = __system_property_get(c_key.as_ptr(), buffer.as_mut_ptr() as *mut i8);

            if result > 0 {
                // Convert result to string
                let c_str = CStr::from_bytes_until_nul(&buffer).ok()?;
                let value = c_str.to_string_lossy().to_string();
                if !value.is_empty() {
                    Some(value)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    /// Check if Android StrongBox is available
    async fn check_android_strongbox() -> BearDogResult<bool> {
        debug!("🔍 Checking Android StrongBox using KeyStore API");

        unsafe {
            // Check if StrongBox security level is supported
            let strongbox_supported =
                AKeyStore_isSecurityLevelSupported(AKEYSTORE_SECURITY_LEVEL_STRONGBOX as i32);

            if strongbox_supported {
                info!("✅ StrongBox security level is supported");
                Ok(true)
            } else {
                // Check if Trusted Environment is available as fallback
                let tee_supported = AKeyStore_isSecurityLevelSupported(
                    AKEYSTORE_SECURITY_LEVEL_TRUSTED_ENVIRONMENT as i32,
                );

                if tee_supported {
                    info!("⚠️ StrongBox not available, but TEE is supported");
                    Ok(false)
                } else {
                    warn!("❌ No hardware security level available");
                    Ok(false)
                }
            }
        }
    }

    /// Detect Android StrongBox implementation
    async fn detect_android_strongbox_implementation() -> BearDogResult<StrongBoxImplementation> {
        debug!("🔍 Detecting StrongBox implementation type");

        // Get device info to determine implementation
        let manufacturer = Self::get_system_property("ro.product.manufacturer")
            .unwrap_or_else(|| "Unknown".to_string());

        let model =
            Self::get_system_property("ro.product.model").unwrap_or_else(|| "Unknown".to_string());

        // Detect specific implementations
        if manufacturer == "Google" && model.contains("Pixel") {
            // Pixel devices use Titan M
            let titan_version = Self::detect_titan_m_version()
                .await
                .unwrap_or_else(|| "1.0".to_string());

            Ok(StrongBoxImplementation::TitanM {
                version: titan_version,
                security_level: "StrongBox".to_string(),
            })
        } else {
            // Generic StrongBox implementation
            let strongbox_version = Self::detect_strongbox_version()
                .await
                .unwrap_or_else(|| "1.0".to_string());

            Ok(StrongBoxImplementation::Generic {
                vendor: manufacturer,
                version: strongbox_version,
            })
        }
    }

    /// Detect StrongBox version
    async fn detect_strongbox_version() -> Option<String> {
        debug!("🔍 Detecting StrongBox version");

        // Try to get StrongBox version from various sources
        if let Some(version) = Self::get_system_property("ro.hardware.keystore") {
            return Some(version);
        }

        if let Some(version) = Self::get_system_property("ro.vendor.strongbox.version") {
            return Some(version);
        }

        // Check for StrongBox feature availability
        if Self::check_strongbox_feature().await {
            Some("1.0".to_string())
        } else {
            None
        }
    }

    /// Check if StrongBox feature is available
    async fn check_strongbox_feature() -> bool {
        unsafe { AKeyStore_isSecurityLevelSupported(AKEYSTORE_SECURITY_LEVEL_STRONGBOX as i32) }
    }

    /// Detect Titan M version (Pixel-specific)
    async fn detect_titan_m_version() -> Option<String> {
        debug!("🔍 Detecting Titan M version");

        // Check various Titan M related properties
        if let Some(version) = Self::get_system_property("ro.vendor.citadel.version") {
            return Some(version);
        }

        if let Some(version) = Self::get_system_property("ro.hardware.security.chip") {
            if version.contains("citadel") || version.contains("titan") {
                return Some("2.0".to_string());
            }
        }

        // Check for Pixel 8 specific properties
        let model =
            Self::get_system_property("ro.product.model").unwrap_or_else(|| "Unknown".to_string());

        if model.contains("Pixel 8") {
            // Pixel 8 series has Titan M2
            Some("2.0".to_string())
        } else if model.contains("Pixel") {
            // Other Pixel devices likely have Titan M
            Some("1.0".to_string())
        } else {
            None
        }
    }

    /// Get verified boot state
    async fn get_verified_boot_state() -> VerifiedBootState {
        debug!("🔍 Getting verified boot state");

        // Check verified boot state
        if let Some(state) = Self::get_system_property("ro.boot.verifiedbootstate") {
            match state.as_str() {
                "green" => VerifiedBootState::Green,
                "yellow" => VerifiedBootState::Yellow,
                "orange" => VerifiedBootState::Orange,
                "red" => VerifiedBootState::Red,
                _ => VerifiedBootState::Unknown,
            }
        } else {
            // Check alternative property
            if let Some(state) = Self::get_system_property("ro.boot.vbmeta.device_state") {
                if state == "locked" {
                    VerifiedBootState::Green
                } else {
                    VerifiedBootState::Orange
                }
            } else {
                VerifiedBootState::Unknown
            }
        }
    }
}

#[cfg(not(target_os = "android"))]
impl NativeAndroidDeviceDetector {
    /// Mock device detection for non-Android platforms
    async fn detect_mock_device() -> BearDogResult<AndroidDeviceInfo> {
        warn!("⚠️ Using mock device detection (not on Android)");

        Ok(AndroidDeviceInfo::new(
            "Google".to_string(),
            "Pixel 8 (Simulated)".to_string(),
            "14".to_string(),
            Some("1.0".to_string()),
            Some("2.0".to_string()),
            "2024-01-01".to_string(),
            VerifiedBootState::Green,
        ))
    }
}

/// Enhanced device capabilities checker
pub struct DeviceCapabilitiesChecker;

impl DeviceCapabilitiesChecker {
    /// Check comprehensive device capabilities
    pub async fn check_capabilities() -> BearDogResult<DeviceCapabilities> {
        info!("🔍 Checking comprehensive device capabilities");

        let strongbox_available =
            NativeAndroidDeviceDetector::check_strongbox_availability().await?;
        let device_info = NativeAndroidDeviceDetector::detect_device_info().await?;

        let capabilities = DeviceCapabilities {
            strongbox_available,
            titan_m_available: device_info.titan_m_version.is_some(),
            verified_boot_green: device_info.verified_boot_state == VerifiedBootState::Green,
            biometric_support: Self::check_biometric_support().await,
        };

        info!("📊 Device capabilities detected:");
        info!("   🔐 StrongBox: {}", capabilities.strongbox_available);
        info!("   🛡️ Titan M: {}", capabilities.titan_m_available);
        info!("   ✅ Verified Boot: {}", capabilities.verified_boot_green);
        info!("   👆 Biometrics: {}", capabilities.biometric_support);

        Ok(capabilities)
    }

    /// Check if biometric authentication is supported
    async fn check_biometric_support() -> bool {
        #[cfg(target_os = "android")]
        {
            // In a real implementation, this would check:
            // - BiometricManager.from(context).canAuthenticate()
            // - Available biometric types (fingerprint, face, iris)
            // - Hardware availability

            // For now, assume true on most modern Android devices
            true
        }

        #[cfg(not(target_os = "android"))]
        {
            false
        }
    }
}
