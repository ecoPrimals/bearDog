// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// HSM Capability Detection
///
/// This module analyzes HSM capabilities by probing their interfaces and testing
/// their features, providing detailed capability reports for various HSM types.

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
use beardog_errors::BearDogResult;
use std::collections::HashMap;
use std::time::Instant;
use tracing::{debug, error, info, warn};
// Import required types
use crate::tunnel::hsm::types::{HsmInterfaceType, TamperResistanceLevel};
use cloud_kms_prober::CloudKmsCapabilityProber;
use mobile_hsm_prober::MobileHsmCapabilityProber;
use performance_benchmarker::PerformanceBenchmarker;
use pkcs11_prober::Pkcs11CapabilityProber;
use software_hsm_prober::SoftwareHsmCapabilityProber;
/// Main capability detection coordinator
#[derive(Debug)]
pub struct CapabilityDetector {
    pkcs11_prober: Pkcs11CapabilityProber,
    cloud_kms_prober: CloudKmsCapabilityProber,
    mobile_hsm_prober: MobileHsmCapabilityProber,
    software_hsm_prober: SoftwareHsmCapabilityProber,
    performance_benchmarker: PerformanceBenchmarker,
}
impl CapabilityDetector {}


    pub fn new() -> BearDogResult<Self> {
        Ok(Self {
            pkcs11_prober: Pkcs11CapabilityProber::new()?,
            cloud_kms_prober: CloudKmsCapabilityProber::new()?,
            mobile_hsm_prober: MobileHsmCapabilityProber::new()?,
            software_hsm_prober: SoftwareHsmCapabilityProber::new()?,
            performance_benchmarker: PerformanceBenchmarker::new()?,
        })
    }
    /// Detect comprehensive capabilities for an HSM interface
    pub async fn detect_capabilities(
        &self,
        interface_type: &HsmInterfaceType,
    ) -> BearDogResult<beardog_types::HsmCapabilities> {
        debug!(
            "🔍 Detecting capabilities for interface: {:?}",
            interface_type
        );
        let start_time = Instant::now();
        let capabilities = match interface_type {
            HsmInterfaceType::Pkcs11 { library_path, .. } => {
                self.pkcs11_prober.probe_capabilities(library_path).await?
            }
            // Handle all other HSM interface types with a default capability set
            _ => self.create_default_hsm_capabilities().await?,
        };
        // Return capabilities directly - performance benchmarking handled separately
        let final_capabilities = capabilities;
        let detection_time = start_time.elapsed();
        info!("✅ Capability detection completed in {:?}", detection_time);
        Ok(final_capabilities)
    // Network HSM capabilities
    async fn create_network_hsm_capabilities(
        endpoint: &str,
        protocol: &str,
    ) -> BearDogResult<HsmCapabilities> {
        debug!("🌐 Analyzing Network HSM: {} ({})", endpoint, protocol);
        Ok(HsmCapabilities {
            key_generation: KeyGenerationCapabilities {
                supported_key_types: vec![
                    "RSA".to_string(),
                    "ECDSA".to_string(),
                    "Ed25519".to_string(),
                ],
                max_key_sizes: vec![2048, 3072, 4096, 256, 384, 521],
                hardware_backed: true,
                true_rng: true,
                supported_algorithms: vec![
                can_generate_in_hardware: true,
                supports_key_derivation: true,
                supports_secure_key_import: true,
                supports_key_wrapping: true,
                entropy_sources: vec!["TRNG".to_string(), "Hardware RNG".to_string()],
                fips_compliant_generation: true,
            },
            hardware_backed: true,
            attestation_support: true,
            max_concurrent_ops: 100,
            crypto_operations: CryptoOperationCapabilities {
                encryption_algorithms: vec![
                    "AES-GCM".to_string(),
                    "AES-CBC".to_string(),
                    "ChaCha20-Poly1305".to_string(),
                signing_algorithms: vec![
                    "RSA-PSS".to_string(),
                hashing_algorithms: vec![
                    "SHA-256".to_string(),
                    "SHA-384".to_string(),
                    "SHA-512".to_string(),
                    "BLAKE3".to_string(),
                key_agreement_algorithms: vec!["ECDH".to_string(), "X25519".to_string()],
                supports_streaming: true,
                supports_batch_operations: true,
                max_data_size: Some(1024 * 1024 * 1024), // 1GB
                hardware_acceleration: true,
            key_management: KeyManagementCapabilities {
                supports_key_backup: true,
                supports_key_recovery: true,
                supports_key_escrow: true,
                supports_key_rotation: true,
                supports_key_versioning: true,
                supports_key_attestation: true,
                key_storage_types: vec!["Hardware".to_string(), "Encrypted".to_string()],
                max_keys: Some(10000),
            advanced_features: AdvancedFeatureCapabilities {
                supports_secure_boot: true,
                supports_remote_attestation: true,
                supports_secure_channels: true,
                supports_multi_tenancy: true,
                supports_role_based_access: true,
                supports_load_balancing: true,
                supports_clustering: true,
                custom_extensions: vec!["Network HSM SDK".to_string()],
            performance: PerformanceCapabilities {
                concurrent_operations: 100,
                operations_per_second: 10000,
                key_generation_speed: 1000,
                signing_speed: 5000,
                verification_speed: 8000,
                encryption_speed: 50000,
                decryption_speed: 45000,
                memory_usage: 512 * 1024 * 1024, // 512MB
            security: SecurityCapabilities {
                fips_140_level: 3,
                common_criteria_level: Some("EAL4+".to_string()),
                tamper_resistance: TamperResistanceLevel::Evidence,
                secure_key_storage: true,
                side_channel_resistance: true,
                security_certifications: vec![
                    "FIPS 140-2 Level 3".to_string(),
                    "Common Criteria EAL4+".to_string(),
            human_entropy: HumanEntropyCapabilities {
                supports_human_entropy: false,
                supports_ephemeral_seeds: false,
                entropy_collection_methods: vec![],
                entropy_quality_score: 0.0,
                supports_biometric_entropy: false,
                supports_behavioral_entropy: false,
            api_support: ApiSupportCapabilities {
                pkcs11_support: true,
                rest_api_support: true,
                grpc_support: true,
                websocket_support: false,
                supported_protocols: vec![
                    "PKCS#11".to_string(),
                    "REST".to_string(),
                    "gRPC".to_string(),
                authentication_methods: vec!["Certificate".to_string(), "Token".to_string()],
            compliance: ComplianceCapabilities {
                fips_140_certified: true,
                common_criteria_certified: true,
                pci_dss_compliant: true,
                hipaa_compliant: true,
                gdpr_compliant: true,
                sox_compliant: true,
                compliance_reports: vec![
                    "PCI DSS Level 1".to_string(),
    // USB HSM capabilities (simplified implementation)
    async fn create_usb_hsm_capabilities(
        device_path: &str,
        debug!("🔌 Analyzing USB HSM: {}", device_path);
        // This would contain a comprehensive USB HSM capability analysis
        // For brevity, using basic capabilities
        Ok(HsmCapabilities::default())
    // Other capability creation methods (simplified)
    async fn create_smart_card_capabilities(
        reader_name: &str,
        debug!("💳 Analyzing Smart Card: {}", reader_name);
    async fn create_tpm_capabilities(&self, version: &str) -> BearDogResult<HsmCapabilities> {
        debug!("🔒 Analyzing TPM: {}", version);
    async fn create_windows_cng_capabilities(
        provider_name: &str,
        debug!("🪟 Analyzing Windows CNG: {}", provider_name);
    async fn create_macos_keychain_capabilities(
        keychain_path: &str,
        debug!("🍏 Analyzing macOS Keychain: {}", keychain_path);
    async fn create_custom_api_capabilities(
        api_endpoint: &str,
        api_version: &str,
            "🔧 Analyzing Custom API: {} ({})",
            api_endpoint, api_version
    async fn create_proprietary_driver_capabilities(
        driver_path: &str,
        driver_version: &str,
            "🔌 Analyzing Proprietary Driver: {} ({})",
            driver_path, driver_version
    async fn create_default_hsm_capabilities(
        debug!("Creating default HSM capabilities");
        // Return canonical beardog_types::HsmCapabilities
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
            performance_metrics: std::collections::HashMap::new(),
            certifications: vec![],
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
                    tamper_resistance: false,
                    secure_boot: false,
                    attestation: false,
                    hardware_rng: false,
                    side_channel_resistance: false,
            api_support: beardog_types::canonical::hsm::capabilities::ApiSupportCapabilities {
                pkcs11: false,
                crypto_api: false,
                jca: false,
                openssl: false,
                rest_api: true,
                grpc_api: false,
                websocket: false,
            security: beardog_types::canonical::hsm::capabilities::SecurityCapabilities {
                authentication_methods: vec!["None".to_string()],
                rbac: false,
                audit_logging: false,
                secure_protocols: vec!["HTTP".to_string()],
                compliance_certifications: vec![],
                security_level: "Software".to_string(),
            human_entropy: beardog_types::canonical::capabilities::HumanEntropyCapabilities {
                available_methods: vec![],
                touch_capabilities: beardog_types::canonical::capabilities::TouchCapabilities {
                    available: false,
                    pressure_sensitive: false,
                    multi_touch: false,
                    max_touch_points: 0,
                    resolution: None,
                motion_capabilities: beardog_types::canonical::capabilities::MotionCapabilities {
                    accelerometer: false,
                    gyroscope: false,
                    magnetometer: false,
                    precision_level: 0,
                biometric_capabilities:
                    beardog_types::canonical::capabilities::BiometricCapabilities {
                        fingerprint: false,
                        face_recognition: false,
                        voice_recognition: false,
                        iris_scanning: false,
                        security_level: 0,
                    },
                environmental_capabilities:
                    beardog_types::canonical::capabilities::EnvironmentalCapabilities {
                        ambient_light: false,
                        proximity: false,
                        temperature: false,
                        humidity: false,
                        pressure: false,
            performance: beardog_types::canonical::capabilities::PerformanceCapabilities::default(),
            compliance: beardog_types::canonical::hsm::capabilities::ComplianceCapabilities {
                certifications: vec![],
                data_residency_control: false,
                encryption_at_rest: false,
                encryption_in_transit: false,
                access_control: false,
            vendor_capabilities: std::collections::HashMap::new(),
            custom_capabilities: std::collections::HashMap::new(),
