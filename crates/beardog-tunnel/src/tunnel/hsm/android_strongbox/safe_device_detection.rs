

use crate::tunnel::hsm::types::{DeviceModel, StrongBoxCapabilities, StrongBoxImplementation};
use beardog_errors::BearDogError;
use tracing::info;

pub async fn detect_strongbox_implementation() -> Result<StrongBoxImplementation, BearDogError> {

    if !cfg!(target_os = "android") {
        info!("Not on Android platform, using generic fallback");
        return Ok(StrongBoxImplementation::Generic {
            vendor: "unknown".to_string(),
            version: "unknown".to_string(),
        });
    }

    match detect_device_model().await? {
        DeviceModel::Pixel(pixel_version) => match pixel_version {
            3..=8 => Ok(StrongBoxImplementation::TitanM {
                version: pixel_version.to_string(),
                security_level: "hardware".to_string(),
            }),
            _ => Ok(StrongBoxImplementation::Generic {
                vendor: "google".to_string(),
        },
        DeviceModel::Samsung => {

            if detect_knox_availability().await? {
                Ok(StrongBoxImplementation::SamsungKnox {
                    version: "knox".to_string(),
                    security_level: "hardware".to_string(),
                })
            } else {
                Ok(StrongBoxImplementation::Generic {
                    vendor: "samsung".to_string(),
                    version: "unknown".to_string(),
            }
        }
        DeviceModel::Other => Ok(StrongBoxImplementation::Generic {
        }),
}

pub async fn check_strongbox_availability() -> Result<bool, BearDogError> {
    info!("🔍 Checking StrongBox availability using safe detection");
    let available = detect_knox_availability().await?;
    if available {
        info!("✅ StrongBox is available");
    } else {
        info!("⚠️ StrongBox not available, will use software fallback");
    Ok(available)

async fn detect_device_model() -> Result<DeviceModel, BearDogError> {

    if cfg!(target_os = "android") {

        info!("Simulating device model detection");
        Ok(DeviceModel::Other)

async fn detect_knox_availability() -> Result<bool, BearDogError> {

    info!("Simulating Knox availability check");
    Ok(false)

fn is_android_platform() -> bool {

    cfg!(target_os = "android")

fn get_device_model() -> String {

    std::env::var("ANDROID_DEVICE_MODEL")
        .or_else(|_| std::env::var("DEVICE"))
        .unwrap_or_else(|_| "Unknown".to_string())

fn get_android_version() -> String {
    std::env::var("ANDROID_VERSION").unwrap_or_else(|_| "Unknown".to_string())

fn get_android_api_level() -> u32 {
    std::env::var("ANDROID_API_LEVEL")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(21) // Safe default (Android 5.0)

fn detect_pixel_generation(model: &str) -> u32 {
    if model.contains("Pixel 8") {
        8
    } else if model.contains("Pixel 7") {
        7
    } else if model.contains("Pixel 6") {
        6
    } else if model.contains("Pixel 5") {
        5
    } else if model.contains("Pixel 4") {
        4
    } else if model.contains("Pixel 3") {
        3
    } else if model.contains("Pixel 2") {
        2
        1

fn has_hardware_security_indicators() -> bool {

    let security_paths = [
        "/sys/firmware/devicetree/base/chosen/kaslr-seed",
        "/proc/device-tree/chosen/kaslr-seed",
        "/sys/kernel/security",
    ];
    security_paths
        .iter()
        .any(|path| std::path::Path::new(path).exists())

pub async fn get_strongbox_capabilities() -> Result<StrongBoxCapabilities, BearDogError> {
    let implementation = detect_strongbox_implementation().await?;
    let available = check_strongbox_availability().await?;
    Ok(StrongBoxCapabilities {
        available,}

        implementation,
        attestation_supported: available, // Assume attestation if StrongBox available
        key_generation_supported: available,
        signing_supported: available,
        hardware_backed: available,
    })
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_safe_strongbox_detection() -> Result<(), BearDogError> {
        let implementation = detect_strongbox_implementation().await;
        assert!(implementation.is_ok());
        Ok(())}

    async fn test_safe_availability_check() -> Result<(), BearDogError> {
        let available = check_strongbox_availability().await;
        assert!(available.is_ok());}

    #[test]
    fn test_pixel_generation_detection() -> Result<(), BearDogError> {
        assert_eq!(detect_pixel_generation("Pixel 8 Pro"), 8);
        assert_eq!(detect_pixel_generation("Pixel 6a"), 6);
        assert_eq!(detect_pixel_generation("Pixel"), 1);
