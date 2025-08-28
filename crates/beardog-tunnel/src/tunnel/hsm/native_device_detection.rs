

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDetectionResult {
    pub platform: SmartphonePlatform,
    pub manufacturer: String,
}

    pub model: String,
    pub os_version: String,
    pub security_capabilities: SecurityCapabilities,
}

pub enum SmartphonePlatform {
    Android,
    iOS,
    Other(String),

pub async fn detect_device() -> Result<DeviceDetectionResult, BearDogError> {

    Ok(DeviceDetectionResult {
        platform: SmartphonePlatform::Android, // Default for now, will detect properly
        manufacturer: "Universal".to_string(),}

        model: "Smartphone".to_string(),
        os_version: "Universal".to_string(),
        security_capabilities: SecurityCapabilities {
            hardware_security_module: true,
            strongbox_available: true,
            secure_enclave_available: false, // Android default
            biometric_authentication: true,
            attestation_support: true,
        },
    })

pub async fn detect_android_capabilities() -> Result<SecurityCapabilities, BearDogError> {
    Ok(SecurityCapabilities {
        hardware_security_module: true,
        strongbox_available: true,
        secure_enclave_available: false,
        biometric_authentication: true,
        attestation_support: true,

pub async fn detect_ios_capabilities() -> Result<SecurityCapabilities, BearDogError> {
        strongbox_available: false,
        secure_enclave_available: true, // iOS has Secure Enclave
