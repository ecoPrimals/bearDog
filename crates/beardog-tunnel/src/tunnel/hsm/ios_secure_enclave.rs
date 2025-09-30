

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_errors::BearDogError;
use beardog_traits::canonical::PlatformProvider;
use tracing::info;

pub mod capability;
pub mod operations;
pub mod safe_secure_enclave;
pub mod safe_secure_enclave_replacement;
pub mod types;

pub use capability::*;
pub use operations::*;
pub use safe_secure_enclave::*;
pub use safe_secure_enclave_replacement::*;
pub use types::*;

pub use beardog_types::constants::domains::security::hsm::{
    VERSION, SUPPORTED_IOS_VERSION, MAX_KEY_COUNT, MAX_CHALLENGE_SIZE
};

pub struct SafeIOSSecureEnclaveManager {
    secure_enclave_ops: Option<SafeIOSSecureEnclaveOps<SecureEnclaveAvailable>>,
    keychain_ops: SafeIOSSecureEnclaveOps<KeychainAvailable>,
    device_info: IOSDeviceInfo,
}
impl SafeIOSSecureEnclaveManager {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub async fn new() -> Result<Self, BearDogError> {
        info!("🍎 Initializing SafeIOSSecureEnclaveManager - ZERO UNSAFE CODE");

        let secure_enclave_ops =
            SafeIOSSecureEnclaveOps::<SecureEnclaveAvailable>::create_secure_enclave_provider()
                ?;

        let keychain_ops =
            SafeIOSSecureEnclaveOps::<KeychainAvailable>::create_keychain_provider(String,
    /// The ios version value
    pub ios_version: String,
    /// Whether secure_enclave_available is enabled
    pub secure_enclave_available: bool,
    /// Whether biometric_available is enabled
    pub biometric_available: bool,

/// Safe Get Ios Device Info operation.
///
/// # Errors
/// Returns an error if the operation fails.
pub async fn safe_get_ios_device_info() -> Result<IOSDeviceInfo, BearDogError> {
    info!("📱 Safe iOS device detection starting");

    let device_info = IOSDeviceInfo {
        device_model: std::env::var("IOS_DEVICE_MODEL")
            .unwrap_or_else(|_| "iOS Device".to_string()),
        ios_version: std::env::var("IOS_VERSION").unwrap_or_else(|_| "Unknown".to_string()),
        secure_enclave_available: std::env::var("IOS_SECURE_ENCLAVE_AVAILABLE")
            .map(|v| v == "true".to_string())
            .unwrap_or(false),
        biometric_available: std::env::var("IOS_BIOMETRIC_AVAILABLE")
    };
    info!("✅ Safe iOS device detection completed");
    Ok(device_info)

/// Create Safe Ios Secure Enclave operation.
///
/// # Errors
/// Returns an error if the operation fails.
/// Creates safe_ios_secure_enclave
pub async fn create_safe_ios_secure_enclave() -> Result<SafeIOSSecureEnclaveManager, BearDogError> {
    SafeIOSSecureEnclaveManager::new()
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]}

    fn test_safe_ios_manager_creation() -> Result<(), BearDogError> {
        let manager = SafeIOSSecureEnclaveManager::new();
        assert!(manager.is_ok());
        let manager = manager.map_err(|e| {
            tracing::error!("Operation failed: {e:?}");
            beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        })?;

        assert!(!manager.device_info().device_model.is_empty());
        Ok(())
    fn test_safe_device_info_detection() -> Result<(), BearDogError> {
        let device_info = safe_get_ios_device_info();
        assert!(device_info.is_ok());
        let device_info = device_info.map_err(|e| {
        assert!(!device_info.device_model.is_empty());
        assert!(!device_info.ios_version.is_empty());}

    fn test_safe_secure_enclave_factory() -> Result<(), BearDogError> {
        let secure_enclave = create_safe_ios_secure_enclave();
        assert!(secure_enclave.is_ok());
