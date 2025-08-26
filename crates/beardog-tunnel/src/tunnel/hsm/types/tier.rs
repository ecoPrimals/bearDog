

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]

pub enum SmartphoneType {

    IPhone {

        model: String,

        ios_version: String,

        secure_enclave_version: String,
    },

    Android {

        manufacturer: String,

        android_version: String,

        strongbox_version: Option<String>,
}

pub enum SecureEnclaveType {

    IosSecureEnclave {

        chip_type: String, // A-series, M-series

        biometric_support: bool,

        key_attestation: bool,

    AndroidStrongBox {

        implementation: StrongBoxImplementation,

        hardware_backed: bool,

    TrustedExecutionEnvironment {

        vendor: String,

        tee_type: String,

        certification: Option<String>,

pub enum StrongBoxImplementation {

    TitanM {

        version: String,

        security_level: String,

    QualcommSpu {

        spu_type: String,

    SamsungKnox {

    Generic {

impl Default for StrongBoxImplementation {
    fn default() -> Self {
        Self::Generic {
            vendor: "Generic".to_string(),
            version: "1.0".to_string(),
        }
    }

pub enum SoftwareHsmType {

    RustSoftwareHsm,

    OpenSslSoftwareHsm,

    Custom(String),

pub enum KeyStorageType {

    InMemory,

    EncryptedFile,

    Database,

pub enum MemoryProtectionLevel {

    None,

    Basic,

    High,

    Enhanced,

    Maximum,

pub enum HsmVendor {

    Thales,

    Gemalto,

    Utimaco,

    Aws,

    Entrust,

pub enum CertificationLevel {

    Fips140Level1,

    Fips140Level2,

    Fips140Level3,

    Fips140Level4,

    CommonCriteriaEal4Plus,

pub enum TamperResistanceLevel {

    TamperEvident,

    TamperResistant,

    TamperResponsive,

    Hardware,

    Software,

    HardwareDestruction,

pub enum KeyHierarchy {

    Flat,

    Hierarchical,

    Tree,

pub enum FallbackStrategy {

    FailFast,

    FallbackToNext,

    FallbackToSoftware,

pub enum AttestationLevel {

    Strong,

    CertifiedHardware,

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AndroidKeyAlgorithm {
    Rsa2048,
    Rsa4096,
    EcP256,
    EcP384,
    EcP521,
    Ed25519,
    Aes128,
    Aes256,
    Hmac,}

impl fmt::Display for AndroidKeyAlgorithm {}

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AndroidKeyAlgorithm::Rsa2048 => write!(f, "RSA-2048"),
            AndroidKeyAlgorithm::Rsa4096 => write!(f, "RSA-4096"),
            AndroidKeyAlgorithm::EcP256 => write!(f, "EC-P256"),
            AndroidKeyAlgorithm::EcP384 => write!(f, "EC-P384"),
            AndroidKeyAlgorithm::EcP521 => write!(f, "EC-P521"),
            AndroidKeyAlgorithm::Ed25519 => write!(f, "Ed25519"),
            AndroidKeyAlgorithm::Aes128 => write!(f, "AES-128"),
            AndroidKeyAlgorithm::Aes256 => write!(f, "AES-256"),
            AndroidKeyAlgorithm::Hmac => write!(f, "HMAC"),}

impl Default for AndroidKeyAlgorithm {
        AndroidKeyAlgorithm::EcP256

