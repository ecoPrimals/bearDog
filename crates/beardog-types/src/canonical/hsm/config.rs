use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmConfig {
    pub provider_type: HsmProviderType,

    pub connection: ConnectionConfig,

    pub security: SecurityConfig,

    pub performance: PerformanceConfig,

    pub provider_settings: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum HsmProviderType {
    Software,

    Hardware,

    Pkcs11,

    Cloud,

    AndroidStrongBox,

    IosSecureEnclave,

    NetworkHsm,

    UsbToken,

    SmartCard,

    Tpm,

    Custom(String),
}

impl fmt::Display for HsmProviderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Software => write!(f, "Software"),
            Self::Hardware => write!(f, "Hardware"),
            Self::Pkcs11 => write!(f, "PKCS#11"),
            Self::Cloud => write!(f, "Cloud"),
            Self::AndroidStrongBox => write!(f, "Android StrongBox"),
            Self::IosSecureEnclave => write!(f, "iOS Secure Enclave"),
            Self::NetworkHsm => write!(f, "Network HSM"),
            Self::UsbToken => write!(f, "USB Token"),
            Self::SmartCard => write!(f, "Smart Card"),
            Self::Tpm => write!(f, "TPM"),
            Self::Custom(name) => write!(f, "Custom({name})"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub timeout_ms: u32,

    pub max_retries: u32,

    pub pool_size: u32,

    pub keep_alive_ms: u32,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            timeout_ms: 30_000,
            max_retries: 3,
            pool_size: 10,
            keep_alive_ms: 60_000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub auth_required: bool,

    pub encryption_enabled: bool,

    pub key_validation: String,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            auth_required: true,
            encryption_enabled: true,
            key_validation: "strict".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub operation_timeout: Duration,

    pub batch_size: u32,

    pub cache_size: Option<u32>,

    pub max_parallel_ops: u32,
}

pub enum AuthMethod {
    None,

    Password(String),

    Certificate { cert_path: String, key_path: String },

    Token(String),

    SmartCard { slot_id: u32, pin: Option<String> },
}

pub struct SoftwareHsmConfig {
    pub base: HsmConfig,

    pub storage_path: String,

    pub storage_encryption: bool,

    pub memory_protection: bool,
}

pub struct HsmTierConfig {
    pub tier: HsmSecurityTier,

    pub tamper_resistance: TamperResistanceLevel,

    pub attestation: AttestationLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum HsmSecurityTier {
    CommonCriteria,

    Fips140Level2,

    Fips140Level3,

    Fips140Level4,
}

pub enum TamperResistanceLevel {
    Basic,

    Evident,

    Resistant,

    Proof,
}

pub enum AttestationLevel {
    Enhanced,

    Remote,
}

impl Default for HsmConfig {
    fn default() -> Self {
        Self {
            provider_type: HsmProviderType::Software,
            connection: ConnectionConfig::default(),
            security: SecurityConfig::default(),
            performance: PerformanceConfig::default(),
            provider_settings: HashMap::with_capacity(16),
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            operation_timeout: Duration::from_secs(10),
            batch_size: 100,
            cache_size: Some(1000),
            max_parallel_ops: 4,
        }
    }
}

impl Default for SoftwareHsmConfig {
    fn default() -> Self {
        Self {
            base: HsmConfig::default(),
            storage_path: "/tmp/beardog_hsm".to_string(),
            storage_encryption: true,
            memory_protection: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationConfig {
    pub require_hardware_attestation: bool,

    pub accepted_attestation_levels: Vec<String>,

    pub attestation_timeout: std::time::Duration,

    pub enabled: bool,

    pub challenge_timeout_secs: u64,

    pub cache_results: bool,

    pub cache_ttl_secs: u64,
}
