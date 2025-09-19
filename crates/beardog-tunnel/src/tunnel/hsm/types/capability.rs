

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
    /// Whether hardware_required is enabled
    pub hardware_required: bool,

    /// Whether attestation_required is enabled
    pub attestation_required: bool,
}
impl Default for CapabilityRequirements {}

    fn default() -> Self {
        Self {
            min_security_level: "software".to_string(),
            key_management: KeyManagementCapabilities::default(),
            advanced_features: AdvancedFeatureCapabilities::default(),
            performance: PerformanceCapabilities::default(),
            security: SecurityCapabilities::default(),
            human_entropy: HumanEntropyCapabilities::default(),
            api_support: ApiSupportCapabilities::default(),
            compliance: ComplianceCapabilities::default(f64,

    /// The avg latency ms value
    pub avg_latency_ms: f64,

    /// The error rate percent value
    pub error_rate_percent: f64,


    pub uptime_percent: f64,}

impl Default for HsmMetrics {
            ops_per_second: 0.0,
            avg_latency_ms: 0.0,
            error_rate_percent: 0.0,
            uptime_percent: 100.0,

pub enum TamperResistance {


    /// Represents evidence variant
    Evidence,


    /// Represents response variant
    Response,

impl Default for TamperResistance {
        /// Represents tamper resistance:: none variant
        TamperResistance::None

#[derive(Debug, Clone)]
    /// Collection of signing algorithms
    pub signing_algorithms: Vec<String>,
    /// Collection of hashing algorithms
    pub hashing_algorithms: Vec<String>,
    /// Collection of key agreement algorithms
    pub key_agreement_algorithms: Vec<String>,
    /// Whether supports_streaming is enabled
    pub supports_streaming: bool,
    /// Whether supports_batch_operations is enabled
    pub supports_batch_operations: bool,
    /// Optional max data size
    pub max_data_size: Option<usize>,
    /// Whether hardware_acceleration is enabled
    pub hardware_acceleration: bool,

pub struct KeyManagementCapabilities {
    /// Whether supports_key_backup is enabled
    pub supports_key_backup: bool,
    /// Whether supports_key_recovery is enabled
    pub supports_key_recovery: bool,
    /// Whether supports_key_escrow is enabled
    pub supports_key_escrow: bool,
    /// Whether supports_key_rotation is enabled
    pub supports_key_rotation: bool,
    /// Whether supports_key_versioning is enabled
    pub supports_key_versioning: bool,
    /// Whether supports_key_attestation is enabled
    pub supports_key_attestation: bool,
    /// Collection of key storage types
    pub key_storage_types: Vec<String>,
    /// Optional max keys
    pub max_keys: Option<u32>,

pub struct AdvancedFeatureCapabilities {
    /// Whether supports_secure_boot is enabled
    pub supports_secure_boot: bool,
    /// Whether supports_remote_attestation is enabled
    pub supports_remote_attestation: bool,
    /// Whether supports_secure_channels is enabled
    pub supports_secure_channels: bool,
    /// Whether supports_multi_tenancy is enabled
    pub supports_multi_tenancy: bool,
    /// Whether supports_role_based_access is enabled
    pub supports_role_based_access: bool,
    /// Whether supports_load_balancing is enabled
    pub supports_load_balancing: bool,
    /// Whether supports_clustering is enabled
    pub supports_clustering: bool,
    /// Collection of custom extensions
    pub custom_extensions: Vec<String>,

pub struct PerformanceCapabilities {
    /// Number of concurrent_operations
    pub concurrent_operations: u32,
    /// Number of operations_per_second
    pub operations_per_second: u32,
    /// Number of key_generation_speed
    pub key_generation_speed: u32,
    /// Number of signing_speed
    pub signing_speed: u32,
    /// Number of verification_speed
    pub verification_speed: u32,
    /// Number of encryption_speed
    pub encryption_speed: u32,
    /// Number of decryption_speed
    pub decryption_speed: u32,
    /// Number of memory_usage
    pub memory_usage: u64,

pub struct HumanEntropyCapabilities {
    /// Whether supports_human_entropy is enabled
    pub supports_human_entropy: bool,
    /// Whether supports_ephemeral_seeds is enabled
    pub supports_ephemeral_seeds: bool,
    /// Collection of entropy collection methods
    pub entropy_collection_methods: Vec<String>,
    /// The entropy quality score value
    pub entropy_quality_score: f64,
    /// Whether supports_biometric_entropy is enabled
    pub supports_biometric_entropy: bool,
    /// Whether supports_behavioral_entropy is enabled
    pub supports_behavioral_entropy: bool,

pub struct ApiSupportCapabilities {
    /// Whether pkcs11_support is enabled
    pub pkcs11_support: bool,
    /// Whether rest_api_support is enabled
    pub rest_api_support: bool,
    /// Whether grpc_support is enabled
    pub grpc_support: bool,
    /// Whether websocket_support is enabled
    pub websocket_support: bool,
    /// Collection of supported protocols
    pub supported_protocols: Vec<String>,
    /// Collection of authentication methods
    pub authentication_methods: Vec<String>,

pub struct ComplianceCapabilities {
    /// Whether fips_140_certified is enabled
    pub fips_140_certified: bool,
    /// Whether common_criteria_certified is enabled
    pub common_criteria_certified: bool,
    /// Whether pci_dss_compliant is enabled
    pub pci_dss_compliant: bool,
    /// Whether hipaa_compliant is enabled
    pub hipaa_compliant: bool,
    /// Whether gdpr_compliant is enabled
    pub gdpr_compliant: bool,
    /// Whether sox_compliant is enabled
    pub sox_compliant: bool,
    /// Collection of compliance reports
    pub compliance_reports: Vec<String>,

pub enum TamperResistanceLevel {
    #[default]
    /// Represents detection variant
    Detection,
