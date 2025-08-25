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


/// HSM Capability Types
///
/// This module defines types for describing HSM capabilities and requirements.

use serde::{Deserialize, Serialize};
/// HSM capability requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequirements {
    /// Minimum security level required
    pub min_security_level: String,
    /// Whether hardware backing is required
    pub hardware_required: bool,
    /// Whether attestation is required
    pub attestation_required: bool,
}
impl Default for CapabilityRequirements {}


    fn default() -> Self {
        Self {
            min_security_level: "software".to_string(),
            hardware_required: false,
            attestation_required: false,
        }
    }
/// Android HSM configuration
pub struct AndroidHsmConfig {
    /// Whether to use StrongBox if available
    pub prefer_strongbox: bool,
    /// Keystore alias prefix
    pub keystore_alias_prefix: String,
    /// Whether to require hardware backing
    pub require_hardware_backing: bool,}


impl Default for AndroidHsmConfig {
            prefer_strongbox: true,
            keystore_alias_prefix: "beardog_".to_string(),
            require_hardware_backing: false,
/// Device model information  }


pub struct DeviceModel {
    /// Device manufacturer
    pub manufacturer: String,
    /// Device model name
    pub model: String,
    /// Operating system version
    pub os_version: String,
/// Memory protection level
pub enum MemoryProtectionLevel {
    /// No special memory protection
    None,
    /// Basic memory protection
    Basic,
    /// Hardware-enforced memory protection
    Hardware,}


impl Default for MemoryProtectionLevel {
        MemoryProtectionLevel::Basic
/// StrongBox capabilities}


pub struct StrongBoxCapabilities {
    /// Whether hardware backing is available
    pub hardware_backed: bool,
    /// Supported key algorithms
    pub supported_algorithms: Vec<String>,
    /// Maximum key size supported
    pub max_key_size: u32,}


impl Default for StrongBoxCapabilities {
            hardware_backed: false,
            supported_algorithms: vec!["Ed25519".to_string(), "P256".to_string()],
            max_key_size: 4096,
/// Key generation capabilities}


pub struct KeyGenerationCapabilities {
    /// Supported key types (RSA, ECDSA, etc.)
    pub supported_key_types: Vec<String>,
    /// Maximum key sizes supported
    pub max_key_sizes: Vec<u32>,
    /// Whether hardware-backed key generation is available
    /// Whether true random number generation is available
    pub true_rng: bool,
    /// Supported algorithms for compatibility
    /// Whether keys can be generated in hardware
    pub can_generate_in_hardware: bool,
    /// Whether key derivation is supported
    pub supports_key_derivation: bool,
    /// Whether secure key import is supported
    pub supports_secure_key_import: bool,
    /// Whether key wrapping is supported
    pub supports_key_wrapping: bool,
    /// Available entropy sources
    pub entropy_sources: Vec<String>,
    /// Whether FIPS-compliant generation is available
    pub fips_compliant_generation: bool,}


impl Default for KeyGenerationCapabilities {
            supported_key_types: vec!["Ed25519".to_string(), "P256".to_string()],
            max_key_sizes: vec![256, 4096],
            true_rng: true,
            can_generate_in_hardware: false,
            supports_key_derivation: false,
            supports_secure_key_import: false,
            supports_key_wrapping: false,
            entropy_sources: vec!["TRNG".to_string()],
            fips_compliant_generation: false,
/// Comprehensive HSM capabilities}


impl Default for HsmCapabilities {
            key_generation: KeyGenerationCapabilities::default(),
            attestation_support: false,
            max_concurrent_ops: 10,
            crypto_operations: CryptoOperationCapabilities::default(),
            key_management: KeyManagementCapabilities::default(),
            advanced_features: AdvancedFeatureCapabilities::default(),
            performance: PerformanceCapabilities::default(),
            security: SecurityCapabilities::default(),
            human_entropy: HumanEntropyCapabilities::default(),
            api_support: ApiSupportCapabilities::default(),
            compliance: ComplianceCapabilities::default(),
/// HSM performance metrics
pub struct HsmMetrics {
    /// Operations per second
    pub ops_per_second: f64,
    /// Average latency in milliseconds
    pub avg_latency_ms: f64,
    /// Error rate percentage
    pub error_rate_percent: f64,
    /// Uptime percentage
    pub uptime_percent: f64,}


impl Default for HsmMetrics {
            ops_per_second: 0.0,
            avg_latency_ms: 0.0,
            error_rate_percent: 0.0,
            uptime_percent: 100.0,
/// Tamper resistance levels}


pub enum TamperResistance {
    /// No tamper resistance
    /// Evidence of tampering
    Evidence,
    /// Active tamper response
    Response,
    /// Hardware tamper protection}


impl Default for TamperResistance {
        TamperResistance::None
/// Cryptographic operation capabilities}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CryptoOperationCapabilities {
    pub encryption_algorithms: Vec<String>,
    pub signing_algorithms: Vec<String>,
    pub hashing_algorithms: Vec<String>,
    pub key_agreement_algorithms: Vec<String>,
    pub supports_streaming: bool,
    pub supports_batch_operations: bool,
    pub max_data_size: Option<usize>,
    pub hardware_acceleration: bool,
/// Key management capabilities
pub struct KeyManagementCapabilities {
    pub supports_key_backup: bool,
    pub supports_key_recovery: bool,
    pub supports_key_escrow: bool,
    pub supports_key_rotation: bool,
    pub supports_key_versioning: bool,
    pub supports_key_attestation: bool,
    pub key_storage_types: Vec<String>,
    pub max_keys: Option<u32>,
/// Advanced feature capabilities
pub struct AdvancedFeatureCapabilities {
    pub supports_secure_boot: bool,
    pub supports_remote_attestation: bool,
    pub supports_secure_channels: bool,
    pub supports_multi_tenancy: bool,
    pub supports_role_based_access: bool,
    pub supports_load_balancing: bool,
    pub supports_clustering: bool,
    pub custom_extensions: Vec<String>,
/// Performance capabilities
pub struct PerformanceCapabilities {
    pub concurrent_operations: u32,
    pub operations_per_second: u32,
    pub key_generation_speed: u32,
    pub signing_speed: u32,
    pub verification_speed: u32,
    pub encryption_speed: u32,
    pub decryption_speed: u32,
    pub memory_usage: u64,
/// Security capabilities
/// Human entropy capabilities
pub struct HumanEntropyCapabilities {
    pub supports_human_entropy: bool,
    pub supports_ephemeral_seeds: bool,
    pub entropy_collection_methods: Vec<String>,
    pub entropy_quality_score: f64,
    pub supports_biometric_entropy: bool,
    pub supports_behavioral_entropy: bool,
/// API support capabilities
pub struct ApiSupportCapabilities {
    pub pkcs11_support: bool,
    pub rest_api_support: bool,
    pub grpc_support: bool,
    pub websocket_support: bool,
    pub supported_protocols: Vec<String>,
    pub authentication_methods: Vec<String>,
/// Compliance capabilities
pub struct ComplianceCapabilities {
    pub fips_140_certified: bool,
    pub common_criteria_certified: bool,
    pub pci_dss_compliant: bool,
    pub hipaa_compliant: bool,
    pub gdpr_compliant: bool,
    pub sox_compliant: bool,
    pub compliance_reports: Vec<String>,
/// Tamper resistance level
pub enum TamperResistanceLevel {
    #[default]
    Detection,
