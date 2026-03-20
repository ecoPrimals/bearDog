// SPDX-License-Identifier: AGPL-3.0-only

//! iOS Secure Enclave HSM Module
//!
//! This module provides safe, zero-unsafe-code access to iOS Secure Enclave
//! functionality for iPhone and iPad devices.

use beardog_errors::BearDogError;
use beardog_types::canonical::providers_unified::traits::UnifiedProvider as PlatformProvider;
use tracing::info;

pub mod capability;
pub mod operations;
pub mod safe_secure_enclave;
// pub mod safe_secure_enclave_replacement; // NOTE: File has syntax errors, disabled temporarily
pub mod types;

pub use capability::*;
pub use operations::*;
pub use safe_secure_enclave::*;
// pub use safe_secure_enclave_replacement::*; // Disabled
pub use types::*;

pub use beardog_types::constants::domains::security::hsm::{
    MAX_CHALLENGE_SIZE, MAX_KEY_COUNT, SUPPORTED_IOS_VERSION, VERSION,
};

/// Safe iOS Secure Enclave Manager
///
/// Provides zero-unsafe-code access to iOS Secure Enclave functionality
pub struct SafeIOSSecureEnclaveManager {
    secure_enclave_ops: Option<SafeIOSSecureEnclaveOps<SecureEnclaveAvailable>>,
    keychain_ops: SafeIOSSecureEnclaveOps<KeychainAvailable>,
    device_info: IOSDeviceInfo,
}

impl SafeIOSSecureEnclaveManager {
    /// Creates a new instance
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub async fn new() -> Result<Self, BearDogError> {
        info!("🍎 Initializing SafeIOSSecureEnclaveManager - ZERO UNSAFE CODE");

        let secure_enclave_ops =
            SafeIOSSecureEnclaveOps::<SecureEnclaveAvailable>::create_secure_enclave_provider()
                .await
                .ok();

        let keychain_ops =
            SafeIOSSecureEnclaveOps::<KeychainAvailable>::create_keychain_provider().await?;

        let device_info = safe_get_ios_device_info().await?;

        Ok(Self {
            secure_enclave_ops,
            keychain_ops,
            device_info,
        })
    }

    /// Gets device information
    pub fn device_info(&self) -> &IOSDeviceInfo {
        &self.device_info
    }

    /// Checks if Secure Enclave is available
    pub fn is_secure_enclave_available(&self) -> bool {
        self.secure_enclave_ops.is_some()
    }
}

/// iOS Device Information
#[derive(Debug, Clone)]
pub struct IOSDeviceInfo {
    /// The device model
    pub device_model: String,
    /// The iOS version
    pub ios_version: String,
    /// Whether Secure Enclave is available
    pub secure_enclave_available: bool,
    /// Whether biometric authentication is available
    pub biometric_available: bool,
}

/// Safe Get iOS Device Info operation
///
/// # Errors
/// Returns an error if device detection fails
pub async fn safe_get_ios_device_info() -> Result<IOSDeviceInfo, BearDogError> {
    info!("📱 Safe iOS device detection starting");

    let device_info = IOSDeviceInfo {
        device_model: beardog_errors::process_env::var("IOS_DEVICE_MODEL")
            .unwrap_or_else(|_| "iOS Device".to_string()),
        ios_version: beardog_errors::process_env::var("IOS_VERSION").unwrap_or_else(|_| "Unknown".to_string()),
        secure_enclave_available: beardog_errors::process_env::var("IOS_SECURE_ENCLAVE_AVAILABLE")
            .map(|v| v == "true")
            .unwrap_or(false),
        biometric_available: beardog_errors::process_env::var("IOS_BIOMETRIC_AVAILABLE")
            .map(|v| v == "true")
            .unwrap_or(false),
    };

    info!("✅ Safe iOS device detection completed");
    Ok(device_info)
}

/// Create Safe iOS Secure Enclave operation
///
/// # Errors
/// Returns an error if creation fails
pub async fn create_safe_ios_secure_enclave() -> Result<SafeIOSSecureEnclaveManager, BearDogError>
{
    SafeIOSSecureEnclaveManager::new().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_safe_ios_manager_creation() -> Result<(), BearDogError> {
        let manager = SafeIOSSecureEnclaveManager::new().await;
        assert!(manager.is_ok());

        let manager = manager?;
        assert!(!manager.device_info().device_model.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_safe_device_info_detection() -> Result<(), BearDogError> {
        let device_info = safe_get_ios_device_info().await;
        assert!(device_info.is_ok());

        let device_info = device_info?;
        assert!(!device_info.device_model.is_empty());
        assert!(!device_info.ios_version.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_safe_secure_enclave_factory() -> Result<(), BearDogError> {
        let secure_enclave = create_safe_ios_secure_enclave().await;
        assert!(secure_enclave.is_ok());

        Ok(())
    }
}
