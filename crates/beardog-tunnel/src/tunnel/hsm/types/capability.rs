

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequirements {

    pub min_security_level: String,

    pub hardware_required: bool,

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

pub struct AndroidHsmConfig {

    pub prefer_strongbox: bool,

    pub keystore_alias_prefix: String,

    pub require_hardware_backing: bool,}

impl Default for AndroidHsmConfig {
            prefer_strongbox: true,
            keystore_alias_prefix: "beardog_".to_string(),
            require_hardware_backing: false,

pub struct DeviceModel {

    pub manufacturer: String,

    pub model: String,

    pub os_version: String,

pub enum MemoryProtectionLevel {

    None,

    Basic,

    Hardware,}

impl Default for MemoryProtectionLevel {
        MemoryProtectionLevel::Basic

pub struct StrongBoxCapabilities {

    pub hardware_backed: bool,

    pub supported_algorithms: Vec<String>,

    pub max_key_size: u32,}

impl Default for StrongBoxCapabilities {
            hardware_backed: false,
            supported_algorithms: vec!["Ed25519".to_string(), "P256".to_string()],
            max_key_size: 4096,

pub struct KeyGenerationCapabilities {

    pub supported_key_types: Vec<String>,

    pub max_key_sizes: Vec<u32>,

    pub true_rng: bool,

    pub can_generate_in_hardware: bool,

    pub supports_key_derivation: bool,

    pub supports_secure_key_import: bool,

    pub supports_key_wrapping: bool,

    pub entropy_sources: Vec<String>,

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

pub struct HsmMetrics {

    pub ops_per_second: f64,

    pub avg_latency_ms: f64,

    pub error_rate_percent: f64,

    pub uptime_percent: f64,}

impl Default for HsmMetrics {
            ops_per_second: 0.0,
            avg_latency_ms: 0.0,
            error_rate_percent: 0.0,
            uptime_percent: 100.0,

pub enum TamperResistance {

    Evidence,

    Response,

impl Default for TamperResistance {
        TamperResistance::None

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

pub struct KeyManagementCapabilities {
    pub supports_key_backup: bool,
    pub supports_key_recovery: bool,
    pub supports_key_escrow: bool,
    pub supports_key_rotation: bool,
    pub supports_key_versioning: bool,
    pub supports_key_attestation: bool,
    pub key_storage_types: Vec<String>,
    pub max_keys: Option<u32>,

pub struct AdvancedFeatureCapabilities {
    pub supports_secure_boot: bool,
    pub supports_remote_attestation: bool,
    pub supports_secure_channels: bool,
    pub supports_multi_tenancy: bool,
    pub supports_role_based_access: bool,
    pub supports_load_balancing: bool,
    pub supports_clustering: bool,
    pub custom_extensions: Vec<String>,

pub struct PerformanceCapabilities {
    pub concurrent_operations: u32,
    pub operations_per_second: u32,
    pub key_generation_speed: u32,
    pub signing_speed: u32,
    pub verification_speed: u32,
    pub encryption_speed: u32,
    pub decryption_speed: u32,
    pub memory_usage: u64,

pub struct HumanEntropyCapabilities {
    pub supports_human_entropy: bool,
    pub supports_ephemeral_seeds: bool,
    pub entropy_collection_methods: Vec<String>,
    pub entropy_quality_score: f64,
    pub supports_biometric_entropy: bool,
    pub supports_behavioral_entropy: bool,

pub struct ApiSupportCapabilities {
    pub pkcs11_support: bool,
    pub rest_api_support: bool,
    pub grpc_support: bool,
    pub websocket_support: bool,
    pub supported_protocols: Vec<String>,
    pub authentication_methods: Vec<String>,

pub struct ComplianceCapabilities {
    pub fips_140_certified: bool,
    pub common_criteria_certified: bool,
    pub pci_dss_compliant: bool,
    pub hipaa_compliant: bool,
    pub gdpr_compliant: bool,
    pub sox_compliant: bool,
    pub compliance_reports: Vec<String>,

pub enum TamperResistanceLevel {
    #[default]
    Detection,
