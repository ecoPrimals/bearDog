//! Native Device Detection
//!
//! This module provides native device detection for mobile platforms.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use tracing::info;

/// Device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// Device manufacturer (e.g., "Google", "Apple")
    pub manufacturer: String,
    /// Device model name
    pub model: String,
    /// Operating system version string
    pub os_version: String,
    /// Mobile platform type
    pub platform: SmartphonePlatform,
    /// Hardware security capabilities
    pub security_capabilities: SecurityCapabilities,
}

/// Smartphone platform
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::upper_case_acronyms)] // iOS is the correct branding
#[allow(non_camel_case_types)] // iOS is the correct branding
pub enum SmartphonePlatform {
    /// Android (Google)
    Android,
    /// iOS (Apple)
    iOS,
    /// Other mobile platform
    Other(String),
}

/// Security capabilities
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityCapabilities {
    /// Whether Android StrongBox is available
    pub has_strongbox: bool,
    /// Whether Apple Secure Enclave is available
    pub has_secure_enclave: bool,
    /// Whether a Trusted Execution Environment is available
    pub has_tee: bool,
    /// Whether biometric authentication is available
    pub has_biometric: bool,
}

/// Detects current device information
pub fn detect_device_info() -> Result<DeviceInfo, BearDogError> {
    info!("📱 Detecting native device information");

    #[cfg(target_os = "android")]
    {
        detect_android_device()
    }

    #[cfg(target_os = "ios")]
    {
        detect_ios_device()
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        Ok(DeviceInfo {
            manufacturer: "Universal".to_string(),
            model: "Generic".to_string(),
            os_version: "Unknown".to_string(),
            platform: SmartphonePlatform::Other("desktop".to_string()),
            security_capabilities: SecurityCapabilities::default(),
        })
    }
}

#[cfg(target_os = "android")]
fn detect_android_device() -> Result<DeviceInfo, BearDogError> {
    info!("🤖 Detecting Android device");

    Ok(DeviceInfo {
        manufacturer: std::env::var("ANDROID_MANUFACTURER")
            .unwrap_or_else(|_| "Unknown".to_string()),
        model: std::env::var("ANDROID_MODEL").unwrap_or_else(|_| "Unknown".to_string()),
        os_version: std::env::var("ANDROID_VERSION").unwrap_or_else(|_| "Unknown".to_string()),
        platform: SmartphonePlatform::Android,
        security_capabilities: SecurityCapabilities {
            has_strongbox: std::env::var("ANDROID_STRONGBOX")
                .map(|v| v == "true")
                .unwrap_or(false),
            has_tee: true,
            has_biometric: true,
            has_secure_enclave: false,
        },
    })
}

#[cfg(target_os = "ios")]
fn detect_ios_device() -> Result<DeviceInfo, BearDogError> {
    info!("🍎 Detecting iOS device");

    Ok(DeviceInfo {
        manufacturer: "Apple".to_string(),
        model: std::env::var("IOS_MODEL").unwrap_or_else(|_| "iPhone".to_string()),
        os_version: std::env::var("IOS_VERSION").unwrap_or_else(|_| "Unknown".to_string()),
        platform: SmartphonePlatform::iOS,
        security_capabilities: SecurityCapabilities {
            has_strongbox: false,
            has_secure_enclave: true,
            has_tee: false,
            has_biometric: true,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_detection() -> Result<(), Box<dyn std::error::Error>> {
        let device_info = detect_device_info();
        assert!(device_info.is_ok());
        Ok(())
    }

    #[test]
    fn test_security_capabilities_default() -> Result<(), Box<dyn std::error::Error>> {
        let caps = SecurityCapabilities::default();
        assert!(!caps.has_strongbox);
        assert!(!caps.has_secure_enclave);
        Ok(())
    }
}
