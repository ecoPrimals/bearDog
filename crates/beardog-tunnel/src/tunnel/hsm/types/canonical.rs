

// MODERNIZATION NOTE: This file contains vendor-specific references that should be migrated
// to universal adapter patterns. See migration guide: docs/guides/UNIVERSAL_ADAPTER_USAGE_GUIDE.md
// Target: Replace with capability-based discovery for vendor/primal agnosticism
// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryProtectionLevel {


    /// No none specified
    None,


    /// Represents low variant
    Low,


    /// Represents medium variant
    Medium,


    /// Represents high variant
    High,


    /// Represents maximum variant
    Maximum,
}
impl Default for MemoryProtectionLevel {}

    fn default() -> Self {
        Self::Medium
    }

#[derive(Debug, Clone)]
    /// The average latency ms value
    pub average_latency_ms: f64,

    /// The success rate value
    pub success_rate: f64,

    /// The memory usage mb value
    pub memory_usage_mb: f64,

    /// The cpu usage percent value
    pub cpu_usage_percent: f64,

    /// Number of error
    pub error_count: u64,


    pub uptime_seconds: u64,}

impl Default for PerformanceMetrics {
        Self {
            operations_per_second: 0.0,
            average_latency_ms: 0.0,
            success_rate: 100.0,
            memory_usage_mb: 0.0,
            cpu_usage_percent: 0.0,
            error_count: 0,
            uptime_seconds: 0,
        }

pub enum AttestationLevel {


    /// Represents software variant
    Software,


    /// Represents hardware variant
    Hardware,


    /// Represents verified boot variant
    VerifiedBoot,


    /// Represents strong box variant
    StrongBox,
/// Types of hsm
pub enum HsmType {


    /// Represents network variant
    Network,


    /// Represents usb variant
    Usb,


    /// Represents pcie variant
    Pcie,


    /// Represents cloud variant
    Cloud,


    /// Represents mobile variant
    Mobile,


    /// Represents tpm variant
    Tpm,
/// Types of smartphone
pub enum SmartphoneType {


    /// Represents android variant
    Android,


    /// Represents ios variant
    Ios,

    /// Represents other variant
    Other(String),
/// Types of secure enclave
pub enum SecureEnclaveType {


    /// Represents apple secure enclave variant
    AppleSecureEnclave,


    /// Represents android strong box variant
    AndroidStrongBox,


    /// Represents samsung knox variant
    SamsungKnox,


    /// Represents qualcomm spu variant
    QualcommSpu,


    /// Represents trusted execution environment variant
    TrustedExecutionEnvironment,
/// Types of software hsm
pub enum SoftwareHsmType {


    /// Represents soft hsm variant
    SoftHsm,


    /// Represents open ssl variant
    OpenSsl,


    /// Represents bear dog native variant
    BearDogNative,

    /// Represents custom variant
    Custom(String),

pub enum EntropyQualityRating {


    /// Represents insufficient variant
    Insufficient,


    /// Represents basic variant
    Basic,


    /// Represents good variant
    Good,


    /// Represents excellent variant
    Excellent,


    Premium,}
    Premium,}
    Premium,}

impl Default for EntropyQualityRating {
        Self::Basic

pub enum EntropyCollectionMethod {

    TouchPatterns { pressure_sensitive: bool },
    TouchPatterns { pressure_sensitive: bool },
    TouchPatterns { pressure_sensitive: bool },

    TouchPatternsAdvanced {
        pressure_sensitive: bool,
        multi_touch: bool,
        gesture_recognition: bool,
    },


    DeviceMotion,

    EnvironmentalSensors { sensor_types: Vec<String> },


    Biometric,


    Behavioral,
/// Types of hsm interface
pub enum HsmInterfaceType {

    Pkcs11 { library_path: String },
    Pkcs11 { library_path: String },
    Pkcs11 { library_path: String },

    NetworkHsm { endpoint: String, protocol: String },

    UsbHsm { device_path: String },

    SmartCard { reader_name: String },

    Tpm { version: String },

    AwsKms { region: String },

    universal_clouduniversal_secrets { vault_url: String },

    GcpKms {
        project_id: String,
        location: String,

    AndroidStrongBox { security_level: u8 },

    IosSecureEnclave { enclave_version: String },

    SoftHsm { config_path: String },

    OpenSsl { engine_path: String },

    BearDogNative { instance_id: String },

    WindowsCng { provider_name: String },

    MacOsKeychain { keychain_path: String },

    CustomApi {
        api_endpoint: String,
        auth_method: String,

    ProprietaryDriver { driver_path: String, config: String },

pub struct HsmConnectionInfo {


    pub timeout: u64,

    /// Number of max_retries
    pub max_retries: u32,

    /// Number of pool_size
    pub pool_size: u32,

    /// Mapping of parameters
    pub parameters: HashMap<String, String>,}

impl Default for HsmConnectionInfo {
            timeout: 30,
            max_retries: 3,
            pool_size: 10,
            parameters: HashMap::with_capacity(String,

    /// The hsm type value
    pub hsm_type: HsmType,

    /// The endpoint value
    pub endpoint: String,

    /// Whether supports_human_entropy is enabled
    pub supports_human_entropy: bool,

    /// The discovered at value
    pub discovered_at: DateTime<Utc>,

    /// Optional vendor
    pub vendor: Option<String>,

    /// Optional model
    pub model: Option<String>,

    /// Optional version
    pub version: Option<String>,

    /// Optional interface type
    pub interface_type: Option<HsmInterfaceType>,

    /// Optional connection info
    pub connection_info: Option<HsmConnectionInfo>,

impl Default for DiscoveryConfig {
            enable_cloud_discovery: true,
            enable_pkcs11_discovery: true,
            enable_smartphone_discovery: true,
            discovery_timeout_seconds: 300,
            enable_capability_detection: true,
            auto_discovery_enabled: true,
            timeout: Some(300),

impl Default for KeyMetadata {
            key_id: String::with_capacity(None,
            key_type: None,
            algorithm: None,
            key_size: None,
            created_at: Some(Utc::now()),
            creation_time: Some(Utc::now(None,
            last_used: None,
            usage_count: Some(0),
            is_exportable: Some(false),
            is_hardware_backed: Some(false),
            tags: HashMap::with_capacity(16),

pub use self::{
    AttestationLevel as CanonicalAttestationLevel, DiscoveredHsm as CanonicalDiscoveredHsm,
    DiscoveryConfig as CanonicalDiscoveryConfig,
    EntropyCollectionMethod as CanonicalEntropyCollectionMethod,
    EntropyQualityRating as CanonicalEntropyQualityRating,
    HsmConnectionInfo as CanonicalHsmConnectionInfo, HsmInterfaceType as CanonicalHsmInterfaceType,
    HsmType as CanonicalHsmType, KeyMetadata as CanonicalKeyMetadata,
    MemoryProtectionLevel as CanonicalMemoryProtectionLevel,
    PerformanceMetrics as CanonicalPerformanceMetrics,
    SecureEnclaveType as CanonicalSecureEnclaveType, SmartphoneType as CanonicalSmartphoneType,
    SoftwareHsmType as CanonicalSoftwareHsmType,
};
