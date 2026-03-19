// SPDX-License-Identifier: AGPL-3.0-only

//! HSM Mobile Platform Types
//!
//! Provides types for mobile platform HSM integration (Android, iOS).

use serde::{Deserialize, Serialize};

/// Android device information
///
/// Contains details about Android device security capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidDeviceInfo {
    /// Device model identifier
    pub device_model: String,

    /// Android OS version
    pub android_version: String,

    /// Whether StrongBox HSM is available (Android KeyMaster API)
    pub strongbox_available: bool,

    /// Whether Trusted Execution Environment (TEE) is available
    pub tee_available: bool,

    /// Whether hardware-backed key attestation is supported
    pub hardware_attestation_supported: bool,
}

impl AndroidDeviceInfo {
    /// Creates new Android device information
    #[must_use]
    pub fn new(device_model: impl Into<String>, android_version: impl Into<String>) -> Self {
        Self {
            device_model: device_model.into(),
            android_version: android_version.into(),
            strongbox_available: false,
            tee_available: true, // TEE is standard on most devices
            hardware_attestation_supported: false,
        }
    }

    /// Sets StrongBox availability
    #[must_use]
    pub fn with_strongbox(mut self, available: bool) -> Self {
        self.strongbox_available = available;
        self
    }

    /// Sets TEE availability
    #[must_use]
    pub fn with_tee(mut self, available: bool) -> Self {
        self.tee_available = available;
        self
    }

    /// Sets hardware attestation support
    #[must_use]
    pub fn with_hardware_attestation(mut self, supported: bool) -> Self {
        self.hardware_attestation_supported = supported;
        self
    }

    /// Checks if device has any hardware security
    #[must_use]
    pub fn has_hardware_security(&self) -> bool {
        self.strongbox_available || self.tee_available
    }

    /// Gets the security level of the device
    #[must_use]
    pub fn security_level(&self) -> SecurityLevel {
        if self.strongbox_available {
            SecurityLevel::StrongBox
        } else if self.tee_available {
            SecurityLevel::TrustedExecutionEnvironment
        } else {
            SecurityLevel::Software
        }
    }
}

impl Default for AndroidDeviceInfo {
    fn default() -> Self {
        Self {
            device_model: "Unknown".to_string(),
            android_version: "Unknown".to_string(),
            strongbox_available: false,
            tee_available: true,
            hardware_attestation_supported: false,
        }
    }
}

/// Security level for mobile devices
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityLevel {
    /// Software-only security (no hardware backing)
    Software,

    /// Trusted Execution Environment (TEE)
    TrustedExecutionEnvironment,

    /// Hardware-backed (StrongBox on Android, Secure Enclave on iOS)
    StrongBox,
}

impl SecurityLevel {
    /// Returns a human-readable name
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Self::Software => "Software",
            Self::TrustedExecutionEnvironment => "TEE",
            Self::StrongBox => "StrongBox",
        }
    }

    /// Returns whether this level has hardware backing
    #[must_use]
    pub fn is_hardware_backed(&self) -> bool {
        matches!(self, Self::TrustedExecutionEnvironment | Self::StrongBox)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_android_device_info_creation() {
        let info = AndroidDeviceInfo::new("Pixel 8 Pro", "14");
        assert_eq!(info.device_model, "Pixel 8 Pro");
        assert_eq!(info.android_version, "14");
        assert!(!info.strongbox_available);
        assert!(info.tee_available);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_android_device_info_with_strongbox() {
        let info = AndroidDeviceInfo::new("Pixel 8 Pro", "14")
            .with_strongbox(true)
            .with_hardware_attestation(true);

        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(info.strongbox_available);
        assert!(info.hardware_attestation_supported);
        assert!(info.has_hardware_security());
    }

    #[test]
    fn test_android_device_info_security_level() {
        let info_software = AndroidDeviceInfo::new("Emulator", "13").with_tee(false);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(info_software.security_level(), SecurityLevel::Software);

        let info_tee = AndroidDeviceInfo::new("Generic", "13");
        assert_eq!(
            info_tee.security_level(),
            SecurityLevel::TrustedExecutionEnvironment
        );

        let info_strongbox = AndroidDeviceInfo::new("Pixel 8", "14").with_strongbox(true);
        assert_eq!(info_strongbox.security_level(), SecurityLevel::StrongBox);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_security_level_properties() {
        assert_eq!(SecurityLevel::Software.name(), "Software");
        assert!(!SecurityLevel::Software.is_hardware_backed());

        assert_eq!(SecurityLevel::TrustedExecutionEnvironment.name(), "TEE");
        assert!(SecurityLevel::TrustedExecutionEnvironment.is_hardware_backed());

        assert_eq!(SecurityLevel::StrongBox.name(), "StrongBox");
        assert!(SecurityLevel::StrongBox.is_hardware_backed());
    }
}
