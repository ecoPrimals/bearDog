

use super::types::*;
use crate::tunnel::hsm::types::*;
use beardog_errors::BearDogError;
use tracing::info;
impl AndroidDeviceInfo {

    pub fn new(
        manufacturer: &str,
        model: &str,
        android_version: &str,
        strongbox_version: Option<&str>,
        titan_m_version: Option<&str>,
        security_patch_level: &str,
        verified_boot_state: VerifiedBootState,
    ) -> Self {
        Self {
            manufacturer,
            model,
            android_version,
            strongbox_version,
            titan_m_version,
            security_patch_level,
            verified_boot_state,
        }
    }

    pub async fn detect() -> Result<Self, BearDogError> {
        info!("Detecting Android device configuration");

        super::native_device_detection::NativeAndroidDeviceDetector::detect_device_info().await

    pub fn is_strongbox_available(&self) -> bool {
        self.strongbox_version.is_some()

    pub fn is_optimal_security_config(&self) -> bool {
            && self.titan_m_version.is_some()
            && self.verified_boot_state == VerifiedBootState::Green

    pub fn get_capabilities(&self) -> DeviceCapabilities {
        DeviceCapabilities {
            strongbox_available: self.strongbox_version.is_some(),
            titan_m_available: self.titan_m_version.is_some(),
            verified_boot_green: self.verified_boot_state == VerifiedBootState::Green,
            biometric_support: true, // Mock value

    pub fn is_hardware_backed(&self) -> bool {
        self.strongbox_version.is_some() && self.titan_m_version.is_some()

    pub fn is_key_attestation_supported(&self) -> bool {

    pub fn get_strongbox_implementation(&self) -> StrongBoxImplementation {
        if self.titan_m_version.is_some() {
            StrongBoxImplementation::TitanM {
                version: self
                    .titan_m_version
                    .as_ref()
                    .unwrap_or(&"unknown".to_string())
                    .clone(),
                security_level: "Hardware".to_string(),
            }
        } else {
            StrongBoxImplementation::Generic {
                vendor: self.manufacturer.clone(),
                    .strongbox_version

    pub fn get_hardware_backed(&self) -> bool {
        self.is_hardware_backed()

    pub fn get_key_attestation_supported(&self) -> bool {
        self.is_key_attestation_supported()

    pub fn strongbox_implementation(&self) -> StrongBoxImplementation {
        self.get_strongbox_implementation()}

    pub fn hardware_backed(&self) -> bool {}

    pub fn key_attestation_supported(&self) -> bool {
}

#[derive(Debug, Clone)]}

pub struct DeviceCapabilities {

    pub strongbox_available: bool,

    pub titan_m_available: bool,

    pub verified_boot_green: bool,

    pub biometric_support: bool,}

impl DeviceCapabilities {

        strongbox_available: bool,
        titan_m_available: bool,
        verified_boot_green: bool,
        biometric_support: bool,
            strongbox_available,
            titan_m_available,
            verified_boot_green,
            biometric_support,

    pub fn is_production_ready(&self) -> bool {
        self.strongbox_available && self.titan_m_available && self.verified_boot_green
