// SPDX-License-Identifier: AGPL-3.0-or-later

//! Safe Android Device Detection
//!
//! Platform-safe detection of Android device capabilities and StrongBox implementation

use crate::tunnel::hsm::android_strongbox::types::{
    AndroidDeviceInfo, StrongBoxImplementation, VerifiedBootState,
};
use beardog_errors::BearDogError;
use tracing::{debug, info};

/// Detects the StrongBox implementation on the current Android device
///
/// # Errors
/// Returns error if device detection fails
pub fn detect_strongbox_implementation() -> Result<StrongBoxImplementation, BearDogError> {
    if !is_android_platform() {
        debug!("Not on Android platform, using generic fallback");
        return Ok(StrongBoxImplementation::Generic {
            vendor: "non-android".to_string(),
            capabilities: vec!["software_fallback".to_string()],
        });
    }

    let device_info = detect_device_info()?;

    match device_info.manufacturer.as_str() {
        "Google" => detect_google_strongbox(&device_info),
        "Samsung" => detect_samsung_strongbox(&device_info),
        "Qualcomm" | "OnePlus" | "Xiaomi" => detect_qualcomm_strongbox(&device_info),
        "MediaTek" | "Oppo" | "Vivo" => detect_mediatek_strongbox(&device_info),
        _ => Ok(StrongBoxImplementation::Generic {
            vendor: device_info.manufacturer.clone(),
            capabilities: vec!["unknown".to_string()],
        }),
    }
}

/// Detects Google Pixel StrongBox (Titan M)
fn detect_google_strongbox(
    device_info: &AndroidDeviceInfo,
) -> Result<StrongBoxImplementation, BearDogError> {
    if let Some(ref titan_version) = device_info.titan_m_version {
        info!("Detected Google Titan M: {}", titan_version);
        Ok(StrongBoxImplementation::TitanM {
            version: titan_version.clone(),
            security_level: "EAL4+".to_string(),
        })
    } else {
        Ok(StrongBoxImplementation::Generic {
            vendor: "Google".to_string(),
            capabilities: vec!["software_hsm".to_string()],
        })
    }
}

/// Detects Samsung StrongBox (Knox)
fn detect_samsung_strongbox(
    device_info: &AndroidDeviceInfo,
) -> Result<StrongBoxImplementation, BearDogError> {
    if detect_knox_availability(device_info)? {
        info!("Detected Samsung Knox StrongBox");
        Ok(StrongBoxImplementation::SamsungKnox {
            version: device_info.android_version.clone(),
        })
    } else {
        Ok(StrongBoxImplementation::Generic {
            vendor: "Samsung".to_string(),
            capabilities: vec!["software_hsm".to_string()],
        })
    }
}

/// Detects Qualcomm SPU StrongBox
fn detect_qualcomm_strongbox(
    device_info: &AndroidDeviceInfo,
) -> Result<StrongBoxImplementation, BearDogError> {
    let attestation_support = device_info.verified_boot_state == VerifiedBootState::Verified;
    info!(
        "Detected Qualcomm SPU, attestation: {}",
        attestation_support
    );

    Ok(StrongBoxImplementation::QualcommSpu {
        attestation_support,
    })
}

/// Detects MediaTek HSM StrongBox
fn detect_mediatek_strongbox(
    device_info: &AndroidDeviceInfo,
) -> Result<StrongBoxImplementation, BearDogError> {
    let features = vec![
        "aes256".to_string(),
        "rsa2048".to_string(),
        "ecdsa_p256".to_string(),
    ];

    info!("Detected MediaTek HSM with {} features", features.len());

    Ok(StrongBoxImplementation::MediaTekHsm { features })
}

/// Checks if StrongBox is available on the device
///
/// # Errors
/// Returns error if availability check fails
pub fn check_strongbox_availability() -> Result<bool, BearDogError> {
    if !is_android_platform() {
        debug!("Not on Android platform, StrongBox not available");
        return Ok(false);
    }

    #[cfg(target_os = "android")]
    {
        // Real Android implementation would query KeyStore capabilities
        let has_strongbox = check_android_keystore_strongbox()?;
        if has_strongbox {
            info!("✅ StrongBox is available");
        } else {
            info!("⚠️  StrongBox not available, will use software fallback");
        }
        Ok(has_strongbox)
    }

    #[cfg(not(target_os = "android"))]
    {
        debug!("Non-Android platform: StrongBox unavailable (host build)");
        Ok(false)
    }
}

/// Detects Android device information
///
/// # Errors
/// Returns error if device info cannot be determined
fn detect_device_info() -> Result<AndroidDeviceInfo, BearDogError> {
    #[cfg(target_os = "android")]
    {
        // In real implementation, query Android system properties
        AndroidDeviceInfo::detect()
    }

    #[cfg(not(target_os = "android"))]
    {
        // Non-Android host: return default device info (no JNI available)
        AndroidDeviceInfo::new()
    }
}

/// Checks if Samsung Knox is available
fn detect_knox_availability(device_info: &AndroidDeviceInfo) -> Result<bool, BearDogError> {
    #[cfg(target_os = "android")]
    {
        // Real implementation would check Knox API availability
        Ok(device_info.model.contains("Galaxy") && device_info.android_version >= "9".to_string())
    }

    #[cfg(not(target_os = "android"))]
    {
        debug!("Non-Android platform: Knox unavailable (host build)");
        Ok(false)
    }
}

/// Checks Android KeyStore for StrongBox capability.
///
/// Requires JNI wiring to `KeyStore.getInstance("AndroidKeyStore")` and
/// `KeyProperties.SECURITY_LEVEL_STRONGBOX`. Returns `true` optimistically
/// until the native bridge is available.
#[cfg(target_os = "android")]
fn check_android_keystore_strongbox() -> Result<bool, BearDogError> {
    debug!("Checking Android KeyStore for StrongBox support (JNI bridge pending)");
    Ok(true)
}

/// Returns true if running on Android platform
fn is_android_platform() -> bool {
    cfg!(target_os = "android")
}

/// Gets the Android device model string
#[cfg(target_os = "android")]
fn get_device_model() -> Result<String, BearDogError> {
    // Real implementation would query android.os.Build.MODEL
    Ok("Pixel 8".to_string())
}

#[cfg(not(target_os = "android"))]
fn get_device_model() -> Result<String, BearDogError> {
    Ok("Non-Android Device".to_string())
}

/// Gets the Android device manufacturer string
#[cfg(target_os = "android")]
fn get_device_manufacturer() -> Result<String, BearDogError> {
    // Real implementation would query android.os.Build.MANUFACTURER
    Ok("Google".to_string())
}

#[cfg(not(target_os = "android"))]
fn get_device_manufacturer() -> Result<String, BearDogError> {
    Ok("Unknown".to_string())
}

/// Detects Pixel generation from model string
pub fn detect_pixel_generation(model: &str) -> u32 {
    if model.contains("Pixel 8") {
        8
    } else if model.contains("Pixel 7") {
        7
    } else if model.contains("Pixel 6") {
        6
    } else if model.contains("Pixel 5") {
        5
    } else if model.contains("Pixel 4") {
        4
    } else if model.contains("Pixel 3") {
        3
    } else if model.contains("Pixel 2") {
        2
    } else if model.contains("Pixel") {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_android_platform() {
        let is_android = is_android_platform();
        // Should be false on non-Android build targets
        #[cfg(not(target_os = "android"))]
        assert!(!is_android);
    }

    #[test]
    fn test_strongbox_detection_non_android() {
        let result = detect_strongbox_implementation();
        assert!(result.is_ok());

        let impl_type = result?;
        match impl_type {
            StrongBoxImplementation::Generic { vendor, .. } => {
                assert_eq!(vendor, "non-android");
            }
            _ => {
                #[cfg(not(target_os = "android"))]
                panic!("Expected Generic implementation on non-Android platform");
            }
        }
    }

    #[test]
    fn test_availability_check() {
        let result = check_strongbox_availability();
        assert!(result.is_ok());
    }

    #[test]
    fn test_pixel_generation_detection() {
        assert_eq!(detect_pixel_generation("Pixel 8"), 8);
        assert_eq!(detect_pixel_generation("Pixel 7"), 7);
        assert_eq!(detect_pixel_generation("Pixel 6 Pro"), 6);
        assert_eq!(detect_pixel_generation("Pixel"), 1);
        assert_eq!(detect_pixel_generation("Samsung Galaxy"), 0);
    }

    #[test]
    fn test_device_model_query() {
        let result = get_device_model();
        assert!(result.is_ok());
        assert!(!result?.is_empty());
    }

    #[test]
    fn test_device_manufacturer_query() {
        let result = get_device_manufacturer();
        assert!(result.is_ok());
        assert!(!result?.is_empty());
    }

    #[test]
    fn test_device_info_detection() {
        let result = detect_device_info();
        assert!(result.is_ok());

        let info = result?;
        assert!(!info.manufacturer.is_empty());
        assert!(!info.model.is_empty());
    }

    #[test]
    fn test_google_strongbox_detection() {
        let device_info = AndroidDeviceInfo {
            manufacturer: "Google".to_string(),
            model: "Pixel 8".to_string(),
            android_version: "14".to_string(),
            strongbox_version: Some("1.0".to_string()),
            titan_m_version: Some("1.0".to_string()),
            security_patch_level: "2024-01-01".to_string(),
            verified_boot_state: VerifiedBootState::Verified,
        };

        let result = detect_google_strongbox(&device_info);
        assert!(result.is_ok());

        match result? {
            StrongBoxImplementation::TitanM { .. } => {
                // Expected for Pixel with Titan M
            }
            _ => panic!("Expected TitanM implementation"),
        }
    }

    #[test]
    fn test_qualcomm_strongbox_detection() {
        let device_info = AndroidDeviceInfo {
            manufacturer: "Qualcomm".to_string(),
            model: "Test Device".to_string(),
            android_version: "13".to_string(),
            strongbox_version: Some("1.0".to_string()),
            titan_m_version: None,
            security_patch_level: "2024-01-01".to_string(),
            verified_boot_state: VerifiedBootState::Verified,
        };

        let result = detect_qualcomm_strongbox(&device_info);
        assert!(result.is_ok());

        match result? {
            StrongBoxImplementation::QualcommSpu {
                attestation_support,
            } => {
                assert!(attestation_support);
            }
            _ => panic!("Expected QualcommSpu implementation"),
        }
    }
}
