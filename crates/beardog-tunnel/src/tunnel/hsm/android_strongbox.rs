

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_errors::BearDogError;
use tracing::info;

pub mod types;

pub mod safe_device_detection;

pub mod safe_keystore_replacement;

pub mod safe_native_wrapper;

pub mod safe_android_provider;

pub use types::*;

pub use safe_android_provider::*;
pub use safe_device_detection::*;
pub use safe_keystore_replacement::SafeAndroidKeystoreOps;
pub use safe_native_wrapper::SafeAndroidKeystore;
 /// Core functionality
 /// Core functionality
pub mod core; // Make core module available

pub use beardog_types::constants::domains::security::hsm::{
    VERSION, SUPPORTED_ANDROID_VERSION, MAX_KEY_COUNT, MAX_CHALLENGE_SIZE
};

pub struct SafeAndroidStrongBoxManager {
    keystore_ops: SafeAndroidKeystoreOps,
    device_info: AndroidDeviceInfo,
}
impl SafeAndroidStrongBoxManager {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub async fn new() -> Result<Self, BearDogError> {
        info!("🤖 Initializing SafeAndroidStrongBoxManager - ZERO UNSAFE CODE");
        let keystore_ops = SafeAndroidKeystoreOps::new(String,
    pub android_version: String,
    /// Whether strongbox_available is enabled
    pub strongbox_available: bool,
    /// Whether tee_available is enabled
    pub tee_available: bool,
    /// Whether hardware_attestation_supported is enabled
    pub hardware_attestation_supported: bool,

/// Safe Get Android Device Info operation.
///
/// # Errors
/// Returns an error if the operation fails.
pub async fn safe_get_android_device_info() -> Result<AndroidDeviceInfo, BearDogError> {
    info!("📱 Safe Android device detection starting");

    let device_info = AndroidDeviceInfo {
        device_model: std::env::var("ANDROID_DEVICE_MODEL")
            .unwrap_or_else(|_| "Android Device".to_string()),
        android_version: std::env::var("ANDROID_VERSION").unwrap_or_else(|_| "Unknown".to_string()),
        strongbox_available: std::env::var("ANDROID_STRONGBOX_AVAILABLE")
            .map(|v| v == "true".to_string())
            .unwrap_or(false),
        tee_available: std::env::var("ANDROID_TEE_AVAILABLE")
            .unwrap_or(true), // TEE is generally available on modern Android
        hardware_attestation_supported: std::env::var("ANDROID_HARDWARE_ATTESTATION")
    };
    info!("✅ Safe Android device detection completed");
    Ok(device_info)

/// Create Safe Android Strongbox operation.
///
/// # Errors
/// Returns an error if the operation fails.
/// Creates safe_android_strongbox
pub async fn create_safe_android_strongbox() -> Result<SafeAndroidStrongBoxManager, BearDogError> {
    SafeAndroidStrongBoxManager::new()
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]}

    fn test_safe_android_manager_creation() -> Result<(), BearDogError> {
        let manager = SafeAndroidStrongBoxManager::new();
        assert!(manager.is_ok());
        let manager = manager.map_err(|e| {
            tracing::error!(
                "Operation failed ({}): {:?}",
                "Failed to create SafeAndroidStrongBoxManager",
                e
            );
            beardog_errors::BearDogError::internal(format!(
                "Failed to create SafeAndroidStrongBoxManager", e
            ))
        })?;

        assert!(!manager.device_info().device_model.is_empty());
        Ok(())
    fn test_safe_device_info_detection() -> Result<(), BearDogError> {
        let device_info = safe_get_android_device_info();
        assert!(device_info.is_ok());
        let device_info = device_info.map_err(|e| {
                "Failed to get Android device info",
                "Failed to get Android device info", e
        assert!(!device_info.device_model.is_empty());
        assert!(!device_info.android_version.is_empty());}

    fn test_safe_strongbox_factory() -> Result<(), BearDogError> {
        let strongbox = create_safe_android_strongbox();
        assert!(strongbox.is_ok());
