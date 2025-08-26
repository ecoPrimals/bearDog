

use beardog_errors::BearDogResult;
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

pub const VERSION: &str = "2.0.0"; // Updated to reflect safe implementation

pub const SUPPORTED_IOS_VERSION: &str = "13.0";

pub const MAX_KEY_COUNT: usize = 1000;

pub const MAX_CHALLENGE_SIZE: usize = 1024;

pub struct SafeIOSSecureEnclaveManager {
    secure_enclave_ops: Option<SafeIOSSecureEnclaveOps<SecureEnclaveAvailable>>,
    keychain_ops: SafeIOSSecureEnclaveOps<KeychainAvailable>,
    device_info: IOSDeviceInfo,
}
impl SafeIOSSecureEnclaveManager {

    pub async fn new() -> BearDogResult<Self> {
        info!("🍎 Initializing SafeIOSSecureEnclaveManager - ZERO UNSAFE CODE");

        let secure_enclave_ops =
            SafeIOSSecureEnclaveOps::<SecureEnclaveAvailable>::create_secure_enclave_provider()
                .await?;

        let keychain_ops =
            SafeIOSSecureEnclaveOps::<KeychainAvailable>::create_keychain_provider().await?;
        let device_info = safe_get_ios_device_info().await?;
        info!("✅ SafeIOSSecureEnclaveManager initialized successfully");
        Ok(Self {
            secure_enclave_ops,
            keychain_ops,
            device_info,
        })
    }

    pub fn is_secure_enclave_available(&self) -> bool {
        self.secure_enclave_ops.is_some()

    pub fn get_best_provider(&self) -> &dyn SafeIOSProvider {
        if let Some(ref secure_enclave) = self.secure_enclave_ops {
            secure_enclave as &dyn SafeIOSProvider
        } else {
            &self.keychain_ops as &dyn SafeIOSProvider
        }

    pub fn device_info(&self) -> &IOSDeviceInfo {
        &self.device_info

#[derive(Debug, Clone)]
pub struct IOSDeviceInfo {
    pub device_model: String,
    pub ios_version: String,
    pub secure_enclave_available: bool,
    pub biometric_available: bool,

pub async fn safe_get_ios_device_info() -> BearDogResult<IOSDeviceInfo> {
    info!("📱 Safe iOS device detection starting");

    let device_info = IOSDeviceInfo {
        device_model: std::env::var("IOS_DEVICE_MODEL")
            .unwrap_or_else(|_| "iOS Device".to_string()),
        ios_version: std::env::var("IOS_VERSION").unwrap_or_else(|_| "Unknown".to_string()),
        secure_enclave_available: std::env::var("IOS_SECURE_ENCLAVE_AVAILABLE")
            .map(|v| v == "true")
            .unwrap_or(false),
        biometric_available: std::env::var("IOS_BIOMETRIC_AVAILABLE")
    };
    info!("✅ Safe iOS device detection completed");
    Ok(device_info)

#[deprecated(since = "3.1.0", note = "Use PlatformProvider instead")]
#[deprecated(since = "3.1.0", note = "Use PlatformProvider instead")]
pub trait SafeIOSProvider: Send + Sync {
    fn security_level(&self) -> SecurityLevel;
    fn supports_biometric(&self) -> bool;

pub async fn create_safe_ios_secure_enclave() -> BearDogResult<SafeIOSSecureEnclaveManager> {
    SafeIOSSecureEnclaveManager::new().await
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]}

    async fn test_safe_ios_manager_creation() -> beardog_errors::BearDogResult<()> {
        let manager = SafeIOSSecureEnclaveManager::new().await;
        assert!(manager.is_ok());
        let manager = manager.map_err(|e| {
            tracing::error!("Operation failed: {e:?}");
            beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        })?;

        assert!(!manager.device_info().device_model.is_empty());
        Ok(())
    async fn test_safe_device_info_detection() -> beardog_errors::BearDogResult<()> {
        let device_info = safe_get_ios_device_info().await;
        assert!(device_info.is_ok());
        let device_info = device_info.map_err(|e| {
        assert!(!device_info.device_model.is_empty());
        assert!(!device_info.ios_version.is_empty());}

    async fn test_safe_secure_enclave_factory() -> beardog_errors::BearDogResult<()> {
        let secure_enclave = create_safe_ios_secure_enclave().await;
        assert!(secure_enclave.is_ok());
