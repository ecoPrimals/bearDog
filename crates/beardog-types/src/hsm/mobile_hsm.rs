// SPDX-License-Identifier: AGPL-3.0-only

//! Mobile HSM Type Definitions
//!
//! Canonical types for mobile Hardware Security Modules including
//! Android StrongBox and iOS Secure Enclave implementations.

use serde::{Deserialize, Serialize};

/// Android StrongBox HSM
///
/// Represents an Android hardware-backed security module (official Android API: KeyMaster).
/// Available on devices with dedicated security hardware (e.g., Google Pixel with Titan M,
/// Samsung devices with Knox, Qualcomm devices with SPU).
///
/// # Platform Support
/// - **Pixel 6+**: Titan M2 security chip
/// - **Pixel 4-5**: Titan M security chip  
/// - **Samsung S21+**: Knox security platform
/// - **Qualcomm 888+**: Secure Processing Unit (SPU)
///
/// # Security Level
/// - **EAL4+** certified hardware isolation
/// - Hardware-bound key generation
/// - Attestation support for key provenance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidStrongBoxHsm {
    /// Unique identifier for this HSM instance
    pub id: String,

    /// Device information including manufacturer, model, and security features
    pub device_info: super::AndroidDeviceInfo,
}

impl AndroidStrongBoxHsm {
    /// Creates a new Android StrongBox HSM with the given ID and device info
    #[must_use]
    pub fn new(id: impl Into<String>, device_info: super::AndroidDeviceInfo) -> Self {
        Self {
            id: id.into(),
            device_info,
        }
    }

    /// Creates a StrongBox HSM with default ID and device info
    ///
    /// This constructs the **canonical type** from defaults; it does not open StrongBox hardware.
    /// On Android builds, use the platform HSM in `beardog-tunnel` for real keystore operations.
    ///
    /// # Errors
    /// Currently always returns `Ok` for this DTO constructor.
    pub fn with_defaults() -> Result<Self, beardog_errors::BearDogError> {
        Ok(Self {
            id: "android-strongbox".to_string(),
            device_info: super::AndroidDeviceInfo::default(),
        })
    }
}

/// Default is only implemented for non-Android when compiling this crate's unit tests.
/// Production code on other targets should use [`Self::new`], [`Self::with_defaults`], or the
/// platform HSM in `beardog-tunnel` (Android).
#[cfg(all(not(target_os = "android"), test))]
impl Default for AndroidStrongBoxHsm {
    fn default() -> Self {
        Self {
            id: "android-strongbox-test-default".to_string(),
            device_info: super::AndroidDeviceInfo::default(),
        }
    }
}

#[cfg(target_os = "android")]
impl Default for AndroidStrongBoxHsm {
    fn default() -> Self {
        // On Android: attempt real StrongBox; fallback to mock if unavailable
        Self::with_defaults().unwrap_or_else(|_| Self {
            id: "android-strongbox".to_string(),
            device_info: super::AndroidDeviceInfo::default(),
        })
    }
}

/// iOS Secure Enclave HSM
///
/// Represents Apple's Secure Enclave Processor (SEP) hardware security module.
/// Available on devices with A7+ chips (iPhone 5s and later) and all Apple Silicon Macs.
///
/// # Platform Support
/// - **iPhone 5s+**: All devices with A7+ chips
/// - **iPad Air+**: All devices with A7+ chips
/// - **Apple Watch**: All series
/// - **M1/M2/M3 Macs**: All Apple Silicon devices
///
/// # Security Level
/// - Hardware-isolated coprocessor
/// - Secure boot chain
/// - Hardware key derivation
/// - Biometric authentication integration (Touch ID, Face ID)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IosSecureEnclaveHsm {
    /// Unique identifier for this HSM instance
    pub id: String,

    /// iOS/iPadOS/macOS version
    pub os_version: Option<String>,

    /// Chip generation (e.g., "A17 Pro", "M3")
    pub chip_generation: Option<String>,
}

impl IosSecureEnclaveHsm {
    /// Creates a new iOS Secure Enclave HSM with the given ID
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            os_version: None,
            chip_generation: None,
        }
    }

    /// Creates a Secure Enclave HSM with OS and chip information
    #[must_use]
    pub fn with_info(
        id: impl Into<String>,
        os_version: impl Into<String>,
        chip_generation: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            os_version: Some(os_version.into()),
            chip_generation: Some(chip_generation.into()),
        }
    }

    /// Creates a Secure Enclave HSM with default ID
    #[must_use]
    pub fn with_default_id() -> Self {
        Self::new("ios-secure-enclave")
    }
}

impl Default for IosSecureEnclaveHsm {
    fn default() -> Self {
        Self::with_default_id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_android_strongbox_creation() {
        let device_info = super::super::AndroidDeviceInfo::default();
        let hsm = AndroidStrongBoxHsm::new("test-hsm", device_info.clone());
        assert_eq!(hsm.id, "test-hsm");
        assert_eq!(hsm.device_info.device_model, device_info.device_model);
    }

    #[test]
    fn test_android_strongbox_default() {
        let hsm = AndroidStrongBoxHsm::default();
        // On Android: "android-strongbox"; on other platforms: mock id
        #[cfg(target_os = "android")]
        assert_eq!(hsm.id, "android-strongbox");
        #[cfg(not(target_os = "android"))]
        assert_eq!(hsm.id, "android-strongbox-test-default");
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_android_strongbox_with_defaults() -> Result<(), beardog_errors::BearDogError> {
        let hsm = AndroidStrongBoxHsm::with_defaults()?;
        assert_eq!(hsm.id, "android-strongbox");
        Ok(())
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_ios_secure_enclave_creation() {
        let hsm = IosSecureEnclaveHsm::new("test-hsm");
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(hsm.id, "test-hsm");
        assert!(hsm.os_version.is_none());
        assert!(hsm.chip_generation.is_none());
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_ios_secure_enclave_with_info() {
        let hsm = IosSecureEnclaveHsm::with_info("test-hsm", "iOS 17.0", "A17 Pro");
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(hsm.id, "test-hsm");
        assert_eq!(hsm.os_version, Some("iOS 17.0".to_string()));
        assert_eq!(hsm.chip_generation, Some("A17 Pro".to_string()));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_ios_secure_enclave_default() {
        let hsm = IosSecureEnclaveHsm::default();
        assert_eq!(hsm.id, "ios-secure-enclave");
    }
}
