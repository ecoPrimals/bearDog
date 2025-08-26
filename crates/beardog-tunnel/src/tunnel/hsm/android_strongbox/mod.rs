

use beardog_errors::BearDogResult;
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

pub mod core; // Make core module available

pub const VERSION: &str = "2.0.0"; // Updated to reflect safe implementation

pub const SUPPORTED_ANDROID_VERSION: u32 = 9; // Minimum Android version for StrongBox

pub const MAX_KEY_COUNT: usize = 1000;

pub const MAX_CHALLENGE_SIZE: usize = 1024;

pub struct SafeAndroidStrongBoxManager {
    keystore_ops: SafeAndroidKeystoreOps,
    device_info: AndroidDeviceInfo,
}
impl SafeAndroidStrongBoxManager {

    pub async fn new() -> BearDogResult<Self> {
        info!("🤖 Initializing SafeAndroidStrongBoxManager - ZERO UNSAFE CODE");
        let keystore_ops = SafeAndroidKeystoreOps::new().await?;
        let device_info = safe_get_android_device_info().await?;
        info!("✅ SafeAndroidStrongBoxManager initialized successfully");
        Ok(Self {
            keystore_ops,
            device_info,
        })
    }

    pub fn keystore_ops(&self) -> &SafeAndroidKeystoreOps {
        &self.keystore_ops

    pub fn device_info(&self) -> &AndroidDeviceInfo {
        &self.device_info

    pub fn is_strongbox_available(&self) -> bool {
        self.device_info.strongbox_available

    pub fn is_tee_available(&self) -> bool {
        self.device_info.tee_available

#[derive(Debug, Clone)]
pub struct AndroidDeviceInfo {
    pub device_model: String,
    pub android_version: String,
    pub strongbox_available: bool,
    pub tee_available: bool,
    pub hardware_attestation_supported: bool,

pub async fn safe_get_android_device_info() -> BearDogResult<AndroidDeviceInfo> {
    info!("📱 Safe Android device detection starting");

    let device_info = AndroidDeviceInfo {
        device_model: std::env::var("ANDROID_DEVICE_MODEL")
            .unwrap_or_else(|_| "Android Device".to_string()),
        android_version: std::env::var("ANDROID_VERSION").unwrap_or_else(|_| "Unknown".to_string()),
        strongbox_available: std::env::var("ANDROID_STRONGBOX_AVAILABLE")
            .map(|v| v == "true")
            .unwrap_or(false),
        tee_available: std::env::var("ANDROID_TEE_AVAILABLE")
            .unwrap_or(true), // TEE is generally available on modern Android
        hardware_attestation_supported: std::env::var("ANDROID_HARDWARE_ATTESTATION")
    };
    info!("✅ Safe Android device detection completed");
    Ok(device_info)

pub async fn create_safe_android_strongbox() -> BearDogResult<SafeAndroidStrongBoxManager> {
    SafeAndroidStrongBoxManager::new().await
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]}

    async fn test_safe_android_manager_creation() -> beardog_errors::BearDogResult<()> {
        let manager = SafeAndroidStrongBoxManager::new().await;
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
    async fn test_safe_device_info_detection() -> beardog_errors::BearDogResult<()> {
        let device_info = safe_get_android_device_info().await;
        assert!(device_info.is_ok());
        let device_info = device_info.map_err(|e| {
                "Failed to get Android device info",
                "Failed to get Android device info", e
        assert!(!device_info.device_model.is_empty());
        assert!(!device_info.android_version.is_empty());}

    async fn test_safe_strongbox_factory() -> beardog_errors::BearDogResult<()> {
        let strongbox = create_safe_android_strongbox().await;
        assert!(strongbox.is_ok());
