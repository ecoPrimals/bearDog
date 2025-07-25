//! Safe Android Device Detection
//!
//! This module provides safe alternatives to unsafe Android NDK calls for device detection.
//! Instead of using unsafe FFI, we use safe runtime detection methods.

use beardog_errors::BearDogResult;
use crate::tunnel::hsm::types::{StrongBoxImplementation, DeviceModel};
use tracing::info;

/// Safe Android StrongBox Device Detection
/// 
/// This module provides safe detection of Android StrongBox capabilities without
/// requiring unsafe code or direct JNI operations.

/// Safely detect StrongBox implementation on device
pub async fn detect_strongbox_implementation() -> BearDogResult<StrongBoxImplementation> {
    // Check if we're on Android platform
    if !cfg!(target_os = "android") {
        info!("Not on Android platform, using generic fallback");
        return Ok(StrongBoxImplementation::Generic { 
            vendor: "unknown".to_string(), 
            version: "unknown".to_string() 
        });
    }

    // Safe device detection logic
    match detect_device_model().await? {
        DeviceModel::Pixel(pixel_version) => {
            match pixel_version {
                3..=8 => Ok(StrongBoxImplementation::TitanM { 
                    version: pixel_version.to_string(), 
                    security_level: "hardware".to_string() 
                }),
                _ => Ok(StrongBoxImplementation::Generic { 
                    vendor: "google".to_string(), 
                    version: pixel_version.to_string() 
                }),
            }
        }
        DeviceModel::Samsung => {
            // Samsung Knox detection
            if detect_knox_availability().await? {
                Ok(StrongBoxImplementation::SamsungKnox { 
                    version: "knox".to_string(), 
                    security_level: "hardware".to_string() 
                })
            } else {
                Ok(StrongBoxImplementation::Generic { 
                    vendor: "samsung".to_string(), 
                    version: "unknown".to_string() 
                })
            }
        }
        DeviceModel::Other => Ok(StrongBoxImplementation::Generic { 
            vendor: "unknown".to_string(), 
            version: "unknown".to_string() 
        }),
    }
}

/// Check StrongBox availability using safe methods
pub async fn check_strongbox_availability() -> BearDogResult<bool> {
    info!("🔍 Checking StrongBox availability using safe detection");

    let available = detect_knox_availability().await?;
    
    if available {
        info!("✅ StrongBox is available");
    } else {
        info!("⚠️ StrongBox not available, will use software fallback");
    }
    
    Ok(available)
}

/// Detect the device model safely
async fn detect_device_model() -> BearDogResult<DeviceModel> {
    // Safe device model detection (placeholder implementation)
    if cfg!(target_os = "android") {
        // In a real implementation, this would safely query Android system properties
        info!("Simulating device model detection");
        Ok(DeviceModel::Other)
    } else {
        Ok(DeviceModel::Other)
    }
}

/// Detect Knox availability safely  
async fn detect_knox_availability() -> BearDogResult<bool> {
    // Safe Knox detection (placeholder implementation)
    info!("Simulating Knox availability check");
    Ok(false)
}

/// Safe Android platform detection
fn is_android_platform() -> bool {
    // Safe compile-time check
    cfg!(target_os = "android")
}

/// Safe device model detection
fn get_device_model() -> String {
    // Use safe environment variable detection
    std::env::var("ANDROID_DEVICE_MODEL")
        .or_else(|_| std::env::var("DEVICE"))
        .unwrap_or_else(|_| "Unknown".to_string())
}

/// Safe Android version detection
fn get_android_version() -> String {
    std::env::var("ANDROID_VERSION")
        .unwrap_or_else(|_| "Unknown".to_string())
}

/// Safe Android API level detection
fn get_android_api_level() -> u32 {
    std::env::var("ANDROID_API_LEVEL")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(21) // Safe default (Android 5.0)
}

/// Detect Pixel generation from model string
fn detect_pixel_generation(model: &str) -> u32 {
    if model.contains("Pixel 8") { 8 }
    else if model.contains("Pixel 7") { 7 }
    else if model.contains("Pixel 6") { 6 }
    else if model.contains("Pixel 5") { 5 }
    else if model.contains("Pixel 4") { 4 }
    else if model.contains("Pixel 3") { 3 }
    else if model.contains("Pixel 2") { 2 }
    else { 1 }
}

/// Check for hardware security indicators using safe methods
fn has_hardware_security_indicators() -> bool {
    // Check for existence of security-related files/directories
    // This is a safe way to detect hardware security modules
    let security_paths = [
        "/sys/firmware/devicetree/base/chosen/kaslr-seed",
        "/proc/device-tree/chosen/kaslr-seed",
        "/sys/kernel/security",
    ];

    security_paths.iter().any(|path| std::path::Path::new(path).exists())
}

/// Get StrongBox capabilities using safe detection
pub async fn get_strongbox_capabilities() -> BearDogResult<StrongBoxCapabilities> {
    let implementation = detect_strongbox_implementation().await?;
    let available = check_strongbox_availability().await?;

    Ok(StrongBoxCapabilities {
        available,
        implementation,
        attestation_supported: available, // Assume attestation if StrongBox available
        key_generation_supported: available,
        signing_supported: available,
        hardware_backed: available,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_safe_strongbox_detection() {
        let implementation = detect_strongbox_implementation().await;
        assert!(implementation.is_ok());
    }

    #[tokio::test]
    async fn test_safe_availability_check() {
        let available = check_strongbox_availability().await;
        assert!(available.is_ok());
    }

    #[test]
    fn test_pixel_generation_detection() {
        assert_eq!(detect_pixel_generation("Pixel 8 Pro"), 8);
        assert_eq!(detect_pixel_generation("Pixel 6a"), 6);
        assert_eq!(detect_pixel_generation("Pixel"), 1);
    }
} 