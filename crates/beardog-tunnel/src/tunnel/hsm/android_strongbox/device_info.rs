// SPDX-License-Identifier: AGPL-3.0-only



use super::types::*;
use crate::tunnel::hsm::types::*;
use beardog_errors::BearDogError;
use tracing::info;
impl AndroidDeviceInfo {

/// New operation.
    /// Creates a new instance
    pub fn new(&str,
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

/// Detect operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn detect() -> Result<Self, BearDogError> {
        info!("Detecting Android device configuration");

        super::native_device_detection::NativeAndroidDeviceDetector::detect_device_info()

/// Is Strongbox Available operation.
    /// Checks if strongbox available
    /// Checks if strongbox available
    pub fn is_strongbox_available(&self) -> bool {
        self.strongbox_version.is_some()

/// Is Optimal Security Config operation.
    /// Checks if optimal security config
    /// Checks if optimal security config
    pub fn is_optimal_security_config(&self) -> bool {
            && self.titan_m_version.is_some()
            && self.verified_boot_state == VerifiedBootState::Green

/// Get Capabilities operation.
    /// Gets capabilities
    /// Gets capabilities
    pub fn get_capabilities(&self) -> DeviceCapabilities {
        DeviceCapabilities {
            strongbox_available: self.strongbox_version.is_some(),
            titan_m_available: self.titan_m_version.is_some(self.verified_boot_state == VerifiedBootState::Green,
            biometric_support: true, // Mock value

/// Is Hardware Backed operation.
    /// Checks if hardware backed
    /// Checks if hardware backed
    pub fn is_hardware_backed(&self) -> bool {
        self.strongbox_version.is_some() && self.titan_m_version.is_some()

/// Is Key Attestation Supported operation.
    /// Checks if key attestation supported
    /// Checks if key attestation supported
    pub fn is_key_attestation_supported(&self) -> bool {

/// Get Strongbox Implementation operation.
    /// Gets strongbox_implementation
    /// Gets strongbox_implementation
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
                vendor: &self.manufacturer,
                    .strongbox_version

/// Get Hardware Backed operation.
    /// Gets hardware_backed
    /// Gets hardware_backed
    pub fn get_hardware_backed(bool,

    /// Whether titan_m_available is enabled
    pub titan_m_available: bool,

    /// Whether verified_boot_green is enabled
    pub verified_boot_green: bool,

    /// Whether biometric_support is enabled
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

/// Is Production Ready operation.
    /// Checks if production ready
    /// Checks if production ready
    pub fn is_production_ready(&self) -> bool {
        self.strongbox_available && self.titan_m_available && self.verified_boot_green
