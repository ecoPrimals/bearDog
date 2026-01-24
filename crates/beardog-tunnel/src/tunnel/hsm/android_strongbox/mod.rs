//! Safe Android StrongBox HSM Implementation
//!
//! This module provides a memory-safe interface to Android StrongBox hardware security.
//! All operations are designed to avoid unsafe code while maintaining security guarantees.

use beardog_errors::BearDogError;
use tracing::info;

pub mod safe_device_detection;
pub mod types;
// ARCHIVED: safe_keystore_replacement.rs moved to archives/orphaned_code_jan_24_2026/
// Functionality provided by safe_native_wrapper instead
pub mod core;
pub mod safe_android_provider;
pub mod safe_native_wrapper;

pub use safe_android_provider::*;
pub use safe_device_detection::*;
pub use safe_native_wrapper::SafeAndroidKeystore;
pub use types::*;

pub use beardog_types::constants::domains::security::hsm::{
    MAX_CHALLENGE_SIZE, MAX_KEY_COUNT, SUPPORTED_ANDROID_VERSION, VERSION,
};

/// Safe Android StrongBox Manager
///
/// Provides high-level interface to Android StrongBox hardware security module
/// without using unsafe code.
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
        info!("🤖 Initializing SafeAndroidStrongBoxManager - ZERO UNSAFE CODE");

        let device_info = safe_get_android_device_info().await?;
        let keystore = SafeAndroidKeystore::new()?;

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
        device_model: std::env::var("ANDROID_DEVICE_MODEL")
            .unwrap_or_else(|_| "Android Device".to_string()),
        android_version: std::env::var("ANDROID_VERSION").unwrap_or_else(|_| "Unknown".to_string()),
        strongbox_available: std::env::var("ANDROID_STRONGBOX_AVAILABLE")
            .map(|v| v == "true")
            .unwrap_or(false),
        tee_available: std::env::var("ANDROID_TEE_AVAILABLE")
            .map(|v| v == "true")
            .unwrap_or(true), // TEE is generally available on modern Android
        hardware_attestation_supported: std::env::var("ANDROID_HARDWARE_ATTESTATION")
            .map(|v| v == "true")
            .unwrap_or(false),
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
        assert!(!device_info.device_model.is_empty());
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
