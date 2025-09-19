

// MODERNIZATION NOTE: This file contains vendor-specific references that should be migrated
// to universal adapter patterns. See migration guide: docs/guides/UNIVERSAL_ADAPTER_USAGE_GUIDE.md
// Target: Replace with capability-based discovery for vendor/primal agnosticism
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone)]
        ios_version: String,

        secure_enclave_version: String,
    },

    Android {

        manufacturer: String,

        android_version: String,

        strongbox_version: Option<String>,
}
/// Types of secure enclave
pub enum SecureEnclaveType {

    /// Represents ios secure enclave variant
    IosSecureEnclave {

        chip_type: String, // A-series, M-series

        biometric_support: bool,

        key_attestation: bool,

    /// Represents android strong box variant
    AndroidStrongBox {

        implementation: StrongBoxImplementation,

        hardware_backed: bool,

    /// Represents trusted execution environment variant
    TrustedExecutionEnvironment {

        vendor: String,

        tee_type: String,

        certification: Option<String>,

pub enum StrongBoxImplementation {

    /// Represents titan m variant
    TitanM {

        version: String,

        security_level: String,

    /// Represents qualcomm spu variant
    QualcommSpu {

        spu_type: String,

    /// Represents samsung knox variant
    SamsungKnox {

    /// Represents generic variant
    Generic {

impl Default for StrongBoxImplementation {
    fn default() -> Self {
        /// Represents self:: generic variant
        Self::Generic {
            vendor: "Generic".to_string(),
            version: "1.0".to_string(),
        }
        }
        }
    }
/// Types of software hsm
pub enum SoftwareHsmType {


    /// Represents rust software hsm variant
    RustSoftwareHsm,


    /// Represents open ssl software hsm variant
    OpenSslSoftwareHsm,

    /// Represents custom variant
    Custom(String),
/// Types of key storage
pub enum KeyStorageType {


    /// Represents in memory variant
    InMemory,


    /// Represents encrypted file variant
    EncryptedFile,


    /// Represents database variant
    Database,

pub enum MemoryProtectionLevel {


    /// No none specified
    None,


    /// Represents basic variant
    Basic,


    /// Represents high variant
    High,


    /// State indicating enhanced
    Enhanced,


    /// Represents maximum variant
    Maximum,

pub enum HsmVendor {


    /// Represents thales variant
    Thales,


    /// Represents gemalto variant
    Gemalto,


    /// Represents utimaco variant
    Utimaco,


    /// Represents aws variant
    Aws,


    /// Represents entrust variant
    Entrust,

pub enum CertificationLevel {


    /// Represents fips140 level1 variant
    Fips140Level1,


    /// Represents fips140 level2 variant
    Fips140Level2,


    /// Represents fips140 level3 variant
    Fips140Level3,


    /// Represents fips140 level4 variant
    Fips140Level4,


    /// Represents common criteria eal4 plus variant
    CommonCriteriaEal4Plus,

pub enum TamperResistanceLevel {


    /// Represents tamper evident variant
    TamperEvident,


    /// Represents tamper resistant variant
    TamperResistant,


    /// Represents tamper responsive variant
    TamperResponsive,


    /// Represents hardware variant
    Hardware,


    /// Represents software variant
    Software,


    /// Represents hardware destruction variant
    HardwareDestruction,

pub enum KeyHierarchy {


    /// Represents flat variant
    Flat,


    /// Represents hierarchical variant
    Hierarchical,


    /// Represents tree variant
    Tree,

pub enum FallbackStrategy {


    /// Represents fail fast variant
    FailFast,


    /// Represents fallback to next variant
    FallbackToNext,


    /// Represents fallback to software variant
    FallbackToSoftware,

pub enum AttestationLevel {


    /// Represents strong variant
    Strong,


    /// Represents certified hardware variant
    CertifiedHardware,

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AndroidKeyAlgorithm {
    /// Represents rsa2048 variant
    Rsa2048,
    /// Represents rsa4096 variant
    Rsa4096,
    /// Represents ec p256 variant
    EcP256,
    /// Represents ec p384 variant
    EcP384,
    /// Represents ec p521 variant
    EcP521,
    /// Represents ed25519 variant
    Ed25519,
    /// Represents aes128 variant
    Aes128,
    /// Represents aes256 variant
    Aes256,
    Hmac,}
    Hmac,}
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

