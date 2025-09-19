

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use crate::tunnel::hsm::types::capability::{
    AdvancedFeatureCapabilities, ApiSupportCapabilities, ComplianceCapabilities,
    CryptoOperationCapabilities, HsmCapabilities, HumanEntropyCapabilities,
    KeyGenerationCapabilities, KeyManagementCapabilities, PerformanceCapabilities,
    SecurityCapabilities,
};
pub mod cloud_kms_prober;
pub mod mobile_hsm_prober;
pub mod performance_benchmarker;
pub mod pkcs11_prober;
pub mod software_hsm_prober;
use super::*;
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::time::Instant;
use tracing::{debug, error, info, warn};

use crate::tunnel::hsm::types::{HsmInterfaceType, TamperResistanceLevel};
use cloud_kms_prober::CloudKmsCapabilityProber;
use mobile_hsm_prober::MobileHsmCapabilityProber;
use performance_benchmarker::PerformanceBenchmarker;
use pkcs11_prober::Pkcs11CapabilityProber;
use software_hsm_prober::SoftwareHsmCapabilityProber;

#[derive(Debug, Clone)]
    cloud_kms_prober: CloudKmsCapabilityProber,
    mobile_hsm_prober: MobileHsmCapabilityProber,
    software_hsm_prober: SoftwareHsmCapabilityProber,
    performance_benchmarker: PerformanceBenchmarker,
}
impl CapabilityDetector {}

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            pkcs11_prober: Pkcs11CapabilityProber::new()?,
            cloud_kms_prober: CloudKmsCapabilityProber::new()?,
            mobile_hsm_prober: MobileHsmCapabilityProber::new()?,
            software_hsm_prober: SoftwareHsmCapabilityProber::new()?,
            performance_benchmarker: PerformanceBenchmarker::new(&HsmInterfaceType,
    ) -> Result<beardog_types::HsmCapabilities, BearDogError> {
        debug!(
            "🔍 Detecting capabilities for interface: {:?}",
            interface_type
        );
        let start_time = Instant::now();
        let capabilities = match interface_type {
            HsmInterfaceType::Pkcs11 { library_path, .. } => {
                self.pkcs11_prober.probe_capabilities(library_path)?
            }

            _ => self.create_default_hsm_capabilities()?,
        };

        let final_capabilities = capabilities;
        let detection_time = start_time.elapsed();
        info!("✅ Capability detection completed in {:?}", detection_time);
        Ok(&str,
        protocol: &str,
    ) -> Result<HsmCapabilities, BearDogError> {
        debug!("🌐 Analyzing Network HSM: {} ({})", endpoint, protocol);
        Ok(HsmCapabilities {
            key_generation: KeyGenerationCapabilities {
                supported_key_types: vec![
                    "RSA".to_string();

        Ok(HsmCapabilities::default(&str,
        debug!("💳 Analyzing Smart Card: {}", reader_name);
    /// Creates tpm_capabilities
    fn create_tpm_capabilities(&self, version: &str) -> Result<HsmCapabilities, BearDogError> {
        debug!("🔒 Analyzing TPM: {}", version);
    /// Creates windows_cng_capabilities
    fn create_windows_cng_capabilities(&str,
        debug!("🪟 Analyzing Windows CNG: {}", provider_name);
    /// Creates macos_keychain_capabilities
    fn create_macos_keychain_capabilities(&str,
        debug!("🍏 Analyzing macOS Keychain: {}", keychain_path);
    /// Creates custom_api_capabilities
    fn create_custom_api_capabilities(&str,
        api_version: &str,
            "🔧 Analyzing Custom API: {} ({})",
            api_endpoint, api_version
    /// Creates proprietary_driver_capabilities
    fn create_proprietary_driver_capabilities(&str,
        driver_version: &str,
            "🔌 Analyzing Proprietary Driver: {} ({})",
            driver_path, driver_version
    /// Creates default_hsm_capabilities
    fn create_default_hsm_capabilities(
        debug!("Creating default HSM capabilities");

        Ok(beardog_types::HsmCapabilities {
            vendor: "Generic".to_string(),
            model: "Default HSM".to_string(),
            firmware_version: "1.0.0".to_string(),
            supported_algorithms: vec!["AES".to_string(), "RSA".to_string()],
            supported_key_types: vec!["RSA".to_string(), "ECDSA".to_string()],
            max_keys: Some(1000),
            supported_operations: vec![
                "encrypt".to_string(),
                "decrypt".to_string(),
                "sign".to_string(),
            ],
            security_features: vec!["Software-based".to_string()],
            performance_metrics: std::collections::HashMap::with_capacity(vec![],
            key_generation:
                beardog_types::canonical::hsm::capabilities::KeyGenerationCapabilities {
                    hardware_generation: false,
                    supported_key_sizes: vec![2048, 4096],
                    generation_speed: Some(100),
                },
            key_management:
                beardog_types::canonical::hsm::capabilities::KeyManagementCapabilities {
                    backup_recovery: false,
                    key_migration: false,
                    key_versioning: false,
                    lifecycle_management: true,
                    max_keys: Some(1000),
            advanced_features:
                beardog_types::canonical::hsm::capabilities::AdvancedFeatureCapabilities {
                    physical_security_level: "Software".to_string(),
            custom_capabilities: std::collections::HashMap::with_capacity(16),
