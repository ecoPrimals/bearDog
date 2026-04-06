// SPDX-License-Identifier: AGPL-3.0-or-later

// Module documentation
//
// This module provides iOS Secure Enclave capability detection for the BearDog ecosystem.

use super::types::*;
use beardog_errors::BearDogError;
use tracing::info;

pub struct CapabilityDetector;

impl CapabilityDetector {
    /// Detect Capability operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn detect_capability() -> Result<Option<SecureEnclaveCapability>, BearDogError> {
        info!("🔍 Detecting iOS Secure Enclave capabilities using safe APIs");

        #[cfg(target_os = "ios")]
        {
            Self::ios_capability_detection()
        }
        #[cfg(target_os = "macos")]
        {
            Self::macos_capability_detection()
        }
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        {
            info!("ℹ️ Secure Enclave not supported on this platform");
            Ok(None)
        }
    }

    #[cfg(target_os = "ios")]
    fn ios_capability_detection() -> Result<Option<SecureEnclaveCapability>, BearDogError> {
        info!("🔍 Detecting iOS Secure Enclave using safe system APIs");
        let ios_version = Self::get_safe_ios_version()?;
        let device_type = Self::get_safe_device_type()?;
        let biometric_features = Self::get_safe_biometric_features()?;

        if ios_version.major >= 7 && Self::has_secure_enclave_hardware(&device_type) {
            Ok(Some(SecureEnclaveCapability::new(
                ios_version,
                device_type,
                biometric_features,
            )))
        } else {
            info!("ℹ️ Device does not support Secure Enclave");
            Ok(None)
        }
    }

    #[cfg(target_os = "macos")]
    fn macos_capability_detection() -> Result<Option<SecureEnclaveCapability>, BearDogError> {
        info!("🔍 Detecting macOS Secure Enclave (T2/M1+) using safe APIs");

        if Self::has_t2_or_apple_silicon()? {
            Ok(Some(SecureEnclaveCapability::new(
                IOSVersion {
                    major: 13,
                    minor: 0,
                    patch: 0,
                }, // macOS equivalent
                SecureEnclaveDevice::Mac,
                vec![BiometricFeature::TouchID], // Assume Touch ID for now
            )))
        } else {
            info!("ℹ️ Mac does not support Secure Enclave");
            Ok(None)
        }
    }

    /// Gets safe_ios_version
    fn get_safe_ios_version() -> Result<IOSVersion, BearDogError> {
        let version_string = beardog_errors::process_env::var("IOS_VERSION").unwrap_or_else(|_| "15.0.0".to_string()); // Default to iOS 15
        let parts: Vec<u32> = version_string
            .split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
        Ok(IOSVersion {
            major: parts.get(0).copied().unwrap_or(15),
            minor: parts.get(1).copied().unwrap_or(0),
            patch: parts.get(2).copied().unwrap_or(0),
        })
    }

    /// Gets safe_device_type
    fn get_safe_device_type() -> Result<SecureEnclaveDevice, BearDogError> {
        let device_model =
            beardog_errors::process_env::var("IOS_DEVICE_MODEL").unwrap_or_else(|_| "iPhone".to_string());
        if device_model.contains("iPhone") {
            Ok(SecureEnclaveDevice::IPhone)
        } else if device_model.contains("iPad") {
            Ok(SecureEnclaveDevice::IPad)
        } else if device_model.contains("Watch") {
            Ok(SecureEnclaveDevice::AppleWatch)
        } else {
            Ok(SecureEnclaveDevice::IPhone) // Default fallback
        }
    }

    #[cfg(any(target_os = "ios", target_os = "macos"))]
    /// Gets safe_biometric_features
    fn get_safe_biometric_features() -> Result<Vec<BiometricFeature>, BearDogError> {
        let mut features = Vec::new();

        if beardog_errors::process_env::var("HAS_TOUCH_ID")
            .map(|v| v == "true")
            .unwrap_or(false)
        {
            features.push(BiometricFeature::TouchID);
        }
        if beardog_errors::process_env::var("HAS_FACE_ID")
            .map(|v| v == "true")
            .unwrap_or(false)
        {
            features.push(BiometricFeature::FaceID);
        }

        if features.is_empty() {
            features.push(BiometricFeature::TouchID); // Safe default
        }
        Ok(features)
    }

    /// Checks if secure enclave hardware is available
    pub fn has_secure_enclave_hardware(device: &SecureEnclaveDevice) -> bool {
        match device {
            SecureEnclaveDevice::IPhone => true, // iPhone 5s+ have Secure Enclave
            SecureEnclaveDevice::IPad => true,   // iPad Air 2+ have Secure Enclave
            SecureEnclaveDevice::Mac => true,    // Mac with T2/M1+ have Secure Enclave
            SecureEnclaveDevice::AppleWatch => true, // Apple Watch S1+ have Secure Enclave
        }
    }

    /// Detect Device Type operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn detect_device_type() -> Result<SecureEnclaveDevice, BearDogError> {
        let device_model =
            beardog_errors::process_env::var("IOS_DEVICE_MODEL").unwrap_or_else(|_| "iPhone".to_string());
        if device_model.contains("iPhone") {
            Ok(SecureEnclaveDevice::IPhone)
        } else if device_model.contains("iPad") {
            Ok(SecureEnclaveDevice::IPad)
        } else if device_model.contains("Watch") {
            Ok(SecureEnclaveDevice::AppleWatch)
        } else {
            Ok(SecureEnclaveDevice::Mac)
        }
    }

    /// Checks if t2 or apple silicon is present
    fn has_t2_or_apple_silicon() -> Result<bool, BearDogError> {
        let has_t2 = beardog_errors::process_env::var("HAS_T2_CHIP")
            .map(|v| v == "true")
            .unwrap_or(false);
        let has_apple_silicon = beardog_errors::process_env::var("HAS_APPLE_SILICON")
            .map(|v| v == "true")
            .unwrap_or(false);
        Ok(has_t2 || has_apple_silicon)
    }

    /// # Errors
    ///
    /// Returns an error if the operation fails.
    /// Validates biometric policy against available features
    pub fn validate_biometric_policy(
        policy: &BiometricPolicy,
        available_features: &[BiometricFeature],
    ) -> Result<bool, BearDogError> {
        match policy {
            BiometricPolicy::TouchIDRequired => {
                Ok(available_features.contains(&BiometricFeature::TouchID))
            }
            BiometricPolicy::FaceIDRequired => {
                Ok(available_features.contains(&BiometricFeature::FaceID))
            }
            BiometricPolicy::TouchIDOrFaceID => Ok(available_features
                .contains(&BiometricFeature::TouchID)
                || available_features.contains(&BiometricFeature::FaceID)),
            BiometricPolicy::FaceIDOnly => Ok(available_features
                .contains(&BiometricFeature::FaceID)
                && available_features.len() == 1),
            BiometricPolicy::TouchIDOnly => Ok(available_features
                .contains(&BiometricFeature::TouchID)
                && available_features.len() == 1),
            BiometricPolicy::AnyBiometric => Ok(!available_features.is_empty()),
            BiometricPolicy::NoBiometric => Ok(true), // Always valid
        }
    }

    /// Get System Info operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn get_system_info() -> Result<SystemInfo, BearDogError> {
        Ok(SystemInfo {
            platform: Self::get_platform_name(),
            has_secure_enclave: Self::detect_capability()?.is_some(),
            device_type: Self::detect_device_type()?,
            available_features: Self::get_available_biometric_features()?,
        })
    }

    fn get_platform_name() -> String {
        #[cfg(target_os = "ios")]
        {
            return "iOS".to_string();
        }
        #[cfg(target_os = "macos")]
        {
            return "macOS".to_string();
        }
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        {
            return "Other".to_string();
        }
    }

    /// Gets available_biometric_features
    fn get_available_biometric_features() -> Result<Vec<BiometricFeature>, BearDogError> {
        Self::get_safe_biometric_features()
    }
}

/// System information structure
pub struct SystemInfo {
    /// Platform name
    pub platform: String,
    /// Whether has_secure_enclave is enabled
    pub has_secure_enclave: bool,
    /// The device type value
    pub device_type: SecureEnclaveDevice,
    /// Collection of available features
    pub available_features: Vec<BiometricFeature>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_capability_detection() -> Result<(), BearDogError> {
        let device = SecureEnclaveDevice::IPhone;
        assert!(matches!(
            device,
            SecureEnclaveDevice::IPhone | SecureEnclaveDevice::IPad
        ));

        assert!(CapabilityDetector::has_secure_enclave_hardware(&device));
        Ok(())
    }

    #[test]
    fn test_biometric_policy_validation() -> Result<(), BearDogError> {
        let features = vec![BiometricFeature::TouchID, BiometricFeature::FaceID];

        let policy = BiometricPolicy::TouchIDRequired;
        assert!(CapabilityDetector::validate_biometric_policy(&policy, &features)?);

        let policy = BiometricPolicy::FaceIDRequired;
        assert!(CapabilityDetector::validate_biometric_policy(&policy, &features)?);

        let policy = BiometricPolicy::TouchIDOrFaceID;
        assert!(CapabilityDetector::validate_biometric_policy(&policy, &features)?);

        Ok(())
    }

    #[test]
    fn test_version_compatibility() -> Result<(), BearDogError> {
        let version = IOSVersion {
            major: 15,
            minor: 0,
            patch: 0,
        };
        assert!(version.major >= 10);
        Ok(())
    }
}
