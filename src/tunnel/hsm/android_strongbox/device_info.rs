//! # Android Device Information
//!
//! This module provides device detection and information gathering
//! functionality for Android devices with StrongBox capabilities.

use super::types::*;
use crate::error::{BearDogError, BearDogResult};
use crate::tunnel::hsm::types::*;
use tracing::{debug, info, warn};

impl AndroidDeviceInfo {
    /// Create a new AndroidDeviceInfo instance
    pub fn new(
        manufacturer: String,
        model: String,
        android_version: String,
        strongbox_version: Option<String>,
        titan_m_version: Option<String>,
        security_patch_level: String,
        verified_boot_state: VerifiedBootState,
    ) -> Self {
        Self {
            manufacturer,
            model,
            android_version,
            strongbox_version,
            titan_m_version,
            security_patch_level,
            verified_boot_state,
        }
    }

    /// Detect the current Android device configuration
    pub async fn detect() -> BearDogResult<Self> {
        info!("Detecting Android device configuration");
        
        // Mock implementation - in a real implementation this would use Android APIs
        let device_info = AndroidDeviceInfo::new(
            "Google".to_string(),
            "Pixel 8a".to_string(),
            "14".to_string(),
            Some("1.0".to_string()),
            Some("2.0".to_string()),
            "2024-01-01".to_string(),
            VerifiedBootState::Green,
        );
        
        Ok(device_info)
    }

    /// Check if StrongBox is available on this device
    pub fn is_strongbox_available(&self) -> bool {
        self.strongbox_version.is_some()
    }

    /// Check if the device is in optimal security configuration
    pub fn is_optimal_security_config(&self) -> bool {
        self.strongbox_version.is_some() && 
        self.titan_m_version.is_some() && 
        self.verified_boot_state == VerifiedBootState::Green
    }

    /// Get device capabilities
    pub fn get_capabilities(&self) -> DeviceCapabilities {
        DeviceCapabilities {
            strongbox_available: self.strongbox_version.is_some(),
            titan_m_available: self.titan_m_version.is_some(),
            verified_boot_green: self.verified_boot_state == VerifiedBootState::Green,
            biometric_support: true, // Mock value
        }
    }

    /// Check if this is a hardware-backed implementation
    pub fn is_hardware_backed(&self) -> bool {
        self.strongbox_version.is_some() && self.titan_m_version.is_some()
    }

    /// Check if key attestation is supported
    pub fn is_key_attestation_supported(&self) -> bool {
        self.strongbox_version.is_some()
    }

    /// Get the strongbox implementation type
    pub fn get_strongbox_implementation(&self) -> StrongBoxImplementation {
        if self.titan_m_version.is_some() {
            StrongBoxImplementation::TitanM {
                version: self.titan_m_version.as_ref().unwrap_or(&"unknown".to_string()).clone(),
                security_level: "Hardware".to_string(),
            }
        } else {
            StrongBoxImplementation::Generic {
                vendor: self.manufacturer.clone(),
                implementation: "StrongBox".to_string(),
                version: self.strongbox_version.as_ref().unwrap_or(&"unknown".to_string()).clone(),
            }
        }
    }

    /// Get hardware backing status
    pub fn get_hardware_backed(&self) -> bool {
        self.is_hardware_backed()
    }

    /// Get key attestation support status
    pub fn get_key_attestation_supported(&self) -> bool {
        self.is_key_attestation_supported()
    }

    /// Get a reference to the strongbox implementation
    pub fn strongbox_implementation(&self) -> StrongBoxImplementation {
        self.get_strongbox_implementation()
    }

    /// Get hardware backing status
    pub fn hardware_backed(&self) -> bool {
        self.is_hardware_backed()
    }

    /// Get key attestation support status
    pub fn key_attestation_supported(&self) -> bool {
        self.is_key_attestation_supported()
    }
}

/// Device capabilities structure
#[derive(Debug, Clone)]
pub struct DeviceCapabilities {
    pub strongbox_available: bool,
    pub titan_m_available: bool,
    pub verified_boot_green: bool,
    pub biometric_support: bool,
} 

impl DeviceCapabilities {
    /// Create new device capabilities
    pub fn new(
        strongbox_available: bool,
        titan_m_available: bool,
        verified_boot_green: bool,
        biometric_support: bool,
    ) -> Self {
        Self {
            strongbox_available,
            titan_m_available,
            verified_boot_green,
            biometric_support,
        }
    }

    /// Check if the device is ready for production use
    pub fn is_production_ready(&self) -> bool {
        self.strongbox_available && 
        self.titan_m_available && 
        self.verified_boot_green
    }
} 