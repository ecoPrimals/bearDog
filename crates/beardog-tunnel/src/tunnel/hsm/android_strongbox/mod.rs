// SPDX-License-Identifier: AGPL-3.0-or-later

//! Safe Android StrongBox HSM Implementation
//!
//! This module provides a memory-safe interface to Android StrongBox hardware security.
//! All operations are designed to avoid unchecked memory patterns while maintaining security guarantees.

use beardog_errors::BearDogError;
use tracing::info;
use types::VerifiedBootState;

/// Maximum challenge size for StrongBox attestation (bytes)
pub const MAX_CHALLENGE_SIZE: usize = 64;

/// Maximum number of keys supported in StrongBox
pub const MAX_KEY_COUNT: usize = 256;

/// Minimum Android version for StrongBox support (Android 11+)
pub const SUPPORTED_ANDROID_VERSION: u32 = 11;

/// Module version
pub const VERSION: &str = "1.0.0";

pub mod safe_device_detection;
pub mod types;
// NOTE: safe_keystore_replacement.rs was removed (Jan 2026)
// Functionality provided by safe_native_wrapper instead
pub mod core;
pub mod safe_android_provider;
pub mod safe_native_wrapper;

// Explicit imports to avoid ambiguity
pub use core::AndroidStrongBoxHsm;
// Backward compatibility alias
pub use core::AndroidStrongBoxHsm as AndroidStrongBox;
pub use safe_device_detection::*;
pub use safe_native_wrapper::SafeAndroidKeystore;
pub use types::AndroidDeviceInfo;

/// Safe Android StrongBox Manager
///
/// Provides high-level interface to Android StrongBox hardware security module
/// without using unchecked memory patterns.
pub struct SafeAndroidStrongBoxManager {
    keystore: SafeAndroidKeystore,
    device_info: AndroidDeviceInfo,
}

impl SafeAndroidStrongBoxManager {
    /// Creates a new SafeAndroidStrongBoxManager instance
    ///
    /// # Errors
    /// Returns an error if device detection or keystore initialization fails.
    pub async fn new() -> Result<Self, BearDogError> {
        info!("🤖 Initializing SafeAndroidStrongBoxManager - safe Rust only");

        let device_info = safe_get_android_device_info().await?;
        let keystore = SafeAndroidKeystore::new(); // Returns SafeAndroidStrongBoxWrapper directly, not Result

        Ok(Self {
            keystore,
            device_info,
        })
    }

    /// Returns device information
    pub fn device_info(&self) -> &AndroidDeviceInfo {
        &self.device_info
    }

    /// Returns keystore operations interface
    pub fn keystore(&self) -> &SafeAndroidKeystore {
        &self.keystore
    }
}

/// Safe Android device information retrieval
///
/// # Errors
/// Returns an error if device information cannot be retrieved.
pub async fn safe_get_android_device_info() -> Result<AndroidDeviceInfo, BearDogError> {
    info!("📱 Safe Android device detection starting");

    let device_info = AndroidDeviceInfo {
        manufacturer: beardog_errors::process_env::var("ANDROID_MANUFACTURER")
            .unwrap_or_else(|_| "Unknown".to_string()),
        model: beardog_errors::process_env::var("ANDROID_MODEL")
            .unwrap_or_else(|_| "Android Device".to_string()),
        device: beardog_errors::process_env::var("ANDROID_DEVICE")
            .unwrap_or_else(|_| "unknown".to_string()),
        hardware: beardog_errors::process_env::var("ANDROID_HARDWARE").ok(),
        board: beardog_errors::process_env::var("ANDROID_BOARD").ok(),
        brand: beardog_errors::process_env::var("ANDROID_BRAND").ok(),
        android_version: beardog_errors::process_env::var("ANDROID_VERSION")
            .unwrap_or_else(|_| "Unknown".to_string()),
        api_level: beardog_errors::process_env::var("ANDROID_API_LEVEL")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(29),
        security_patch: beardog_errors::process_env::var("ANDROID_SECURITY_PATCH").ok(),
        security_patch_level: beardog_errors::process_env::var("ANDROID_SECURITY_PATCH")
            .unwrap_or_else(|_| "unknown".to_string()),
        strongbox_version: beardog_errors::process_env::var("ANDROID_STRONGBOX_VERSION").ok(),
        titan_m_version: beardog_errors::process_env::var("ANDROID_TITAN_M_VERSION").ok(),
        verified_boot_state: VerifiedBootState::Unverified, // Runtime discovery
    };

    info!("✅ Safe Android device detection completed");
    Ok(device_info)
}

/// Factory function to create SafeAndroidStrongBoxManager
///
/// # Errors
/// Returns an error if manager creation fails.
pub async fn create_safe_android_strongbox() -> Result<SafeAndroidStrongBoxManager, BearDogError> {
    SafeAndroidStrongBoxManager::new().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_safe_android_manager_creation() -> Result<(), BearDogError> {
        let manager = SafeAndroidStrongBoxManager::new().await?;
        assert!(!manager.device_info().device_model.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_safe_device_info_detection() -> Result<(), BearDogError> {
        let device_info = safe_get_android_device_info().await?;
        assert!(!device_info.device_model().is_empty()); // Method, not field
        assert!(!device_info.android_version.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_safe_strongbox_factory() -> Result<(), BearDogError> {
        let strongbox = create_safe_android_strongbox().await?;
        assert!(!strongbox.device_info().device_model.is_empty());
        Ok(())
    }
}
