

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryProtectionLevel {

    None,

    Low,

    Medium,

    High,

    Maximum,
}
impl Default for MemoryProtectionLevel {}

    fn default() -> Self {
        Self::Medium
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {

    pub operations_per_second: f64,

    pub average_latency_ms: f64,

    pub success_rate: f64,

    pub memory_usage_mb: f64,

    pub cpu_usage_percent: f64,

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

    Software,

    Hardware,

    VerifiedBoot,

    StrongBox,

pub enum HsmType {

    Network,

    Usb,

    Pcie,

    Cloud,

    Mobile,

    Tpm,

pub enum SmartphoneType {

    Android,

    Ios,

    Other(String),

pub enum SecureEnclaveType {

    AppleSecureEnclave,

    AndroidStrongBox,

    SamsungKnox,

    QualcommSpu,

    TrustedExecutionEnvironment,

pub enum SoftwareHsmType {

    SoftHsm,

    OpenSsl,

    BearDogNative,

    Custom(String),

pub enum EntropyQualityRating {

    Insufficient,

    Basic,

    Good,

    Excellent,

    Premium,}

impl Default for EntropyQualityRating {
        Self::Basic

pub enum EntropyCollectionMethod {

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

pub enum HsmInterfaceType {

    Pkcs11 { library_path: String },

    NetworkHsm { endpoint: String, protocol: String },

    UsbHsm { device_path: String },

    SmartCard { reader_name: String },

    Tpm { version: String },

    AwsKms { region: String },

    AzureKeyVault { vault_url: String },

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

    pub max_retries: u32,

    pub pool_size: u32,

    pub parameters: HashMap<String, String>,}

impl Default for HsmConnectionInfo {
            timeout: 30,
            max_retries: 3,
            pool_size: 10,
            parameters: HashMap::with_capacity(16),

pub struct DiscoveredHsm {

    pub name: String,

    pub hsm_type: HsmType,

    pub endpoint: String,

    pub supports_human_entropy: bool,

    pub discovered_at: DateTime<Utc>,

    pub vendor: Option<String>,

    pub model: Option<String>,

    pub version: Option<String>,

    pub interface_type: Option<HsmInterfaceType>,

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
            key_id: String::with_capacity(64),
            key_name: None,
            key_type: None,
            algorithm: None,
            key_size: None,
            created_at: Some(Utc::now()),
            creation_time: Some(Utc::now()),
            expires_at: None,
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
