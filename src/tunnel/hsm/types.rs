//! # HSM Types Module
//!
//! This module contains all the types and structures used across the HSM system.
//! It includes HSM tiers, key types, configurations, and other supporting types.

use crate::error::{BearDogError, BearDogResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// HSM tier enumeration - defines the different types of HSMs available  
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum HsmTier {
    /// Tier 1: Smartphone HSM (Most Accessible)
    /// - iOS Secure Enclave, Android StrongBox
    /// - Always available, user-controlled
    /// - Good for personal keys, authentication
    SmartphoneHsm {
        device_type: SmartphoneType,
        secure_enclave: SecureEnclaveType,
        attestation_level: AttestationLevel,
        user_presence_required: bool,
    },

    /// Tier 2: Software HSM (Scalable)
    /// - Rust-based implementation
    /// - Encrypted key storage
    /// - Good for development, testing, light production
    SoftwareHsm {
        implementation: SoftwareHsmType,
        key_storage: KeyStorageType,
        encryption_at_rest: bool,
        memory_protection: MemoryProtectionLevel,
    },

    /// Tier 3: Hardware HSM (Highest Security)
    /// - FIPS 140-2 Level 3+ certified
    /// - Tamper-resistant hardware
    /// - Good for root keys, high-value operations
    HardwareHsm {
        vendor: HsmVendor,
        model: String,
        certification: CertificationLevel,
        tamper_resistance: TamperResistanceLevel,
    },

    /// Tier 4: Hybrid HSM (Best of All Worlds)
    /// - Combines multiple HSM types
    /// - Hierarchical key management
    /// - Optimal security and performance
    HybridHsm {
        tiers: Vec<Box<HsmTier>>,
        key_hierarchy: KeyHierarchy,
        fallback_strategy: Box<FallbackStrategy>,
    },
}

/// Smartphone device types
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SmartphoneType {
    IPhone {
        model: String,
        ios_version: String,
        secure_enclave_version: String,
    },
    Android {
        manufacturer: String,
        model: String,
        android_version: String,
        strongbox_version: Option<String>,
    },
}

/// Secure enclave types for smartphone HSMs
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SecureEnclaveType {
    /// iOS Secure Enclave
    IosSecureEnclave {
        chip_type: String, // A-series, M-series
        biometric_support: bool,
        key_attestation: bool,
    },

    /// Android StrongBox
    AndroidStrongBox {
        implementation: StrongBoxImplementation,
        hardware_backed: bool,
        key_attestation: bool,
    },

    /// Generic trusted execution environment
    TrustedExecutionEnvironment {
        vendor: String,
        tee_type: String,
        certification: Option<String>,
    },
}

/// StrongBox implementation types
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum StrongBoxImplementation {
    /// Google Titan M chip
    TitanM {
        version: String,
        security_level: String,
    },
    /// Qualcomm Secure Processing Unit
    QualcommSpu { version: String, spu_type: String },
    /// Samsung Knox
    SamsungKnox { version: String, knox_level: String },
    /// Generic StrongBox implementation
    Generic {
        vendor: String,
        implementation: String,
        version: String,
    },
}

/// Software HSM implementation types
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SoftwareHsmType {
    /// BearDog's Rust-based software HSM
    RustSoftwareHsm,
    /// PKCS#11 software token
    Pkcs11SoftToken,
    /// OpenSSL-based software HSM
    OpenSslSoftHsm,
    /// Custom implementation
    Custom(String),
}

/// Key storage types for software HSMs
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum KeyStorageType {
    /// Encrypted file storage
    EncryptedFile,
    /// Database storage
    Database,
    /// Memory-only storage
    Memory,
    /// Custom storage backend
    Custom(String),
}

/// Memory protection levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MemoryProtectionLevel {
    /// No special memory protection
    None,
    /// Basic memory protection (mlock)
    Basic,
    /// High memory protection (mlock + zeroization)
    High,
    /// Maximum memory protection (mlock + zeroization + guard pages)
    Maximum,
}

/// Hardware HSM vendors
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum HsmVendor {
    /// AWS CloudHSM
    Aws,
    /// SafeNet Luna
    SafeNet,
    /// Thales nShield
    Thales,
    /// Utimaco
    Utimaco,
    /// Gemalto
    Gemalto,
    /// Custom vendor
    Custom(String),
}

/// Certification levels for hardware HSMs
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CertificationLevel {
    Fips140Level1,
    Fips140Level2,
    Fips140Level3,
    Fips140Level4,
    CommonCriteria,
    Custom(String),
}

/// Tamper resistance levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TamperResistanceLevel {
    /// No tamper resistance
    None,
    /// Software-based tamper detection
    Software,
    /// Hardware-based tamper detection
    Hardware,
    /// Hardware-based tamper destruction
    HardwareDestruction,
}

/// Key hierarchy management
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum KeyHierarchy {
    /// Flat key structure
    Flat,
    /// Hierarchical key structure
    Hierarchical,
    /// Tiered key structure
    Tiered,
    /// Custom hierarchy
    Custom(String),
}

/// Failover strategies for hybrid HSMs
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FallbackStrategy {
    /// Use next available HSM tier
    NextTier,
    /// Use specific HSM tier
    SpecificTier(Box<HsmTier>),
    /// Use software HSM as fallback
    SoftwareFallback,
    /// No fallback - fail operation
    NoFallback,
}

/// Attestation levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AttestationLevel {
    /// No attestation
    None,
    /// Software attestation
    Software,
    /// Hardware attestation
    Hardware,
    /// Certified hardware attestation
    CertifiedHardware,
}

/// Key types supported by HSMs
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyType {
    /// RSA keys
    Rsa {
        key_size: u32,
    },
    /// Elliptic curve keys
    EccP256,
    EccP384,
    EccP521,
    /// AES symmetric keys
    Aes128,
    Aes192,
    Aes256,
    /// ChaCha20 keys
    ChaCha20,
    /// Ed25519 keys
    Ed25519,
    /// X25519 keys
    X25519,
    /// Custom key type
    Custom(String),
}

/// Key metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadata {
    pub key_id: String,
    pub key_type: KeyType,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub usage_policy: KeyUsagePolicy,
    pub attributes: HashMap<String, String>,
}

/// Key usage policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyUsagePolicy {
    pub can_encrypt: bool,
    pub can_decrypt: bool,
    pub can_sign: bool,
    pub can_verify: bool,
    pub can_wrap: bool,
    pub can_unwrap: bool,
    pub can_derive: bool,
    pub exportable: bool,
    pub user_presence_required: bool,
    pub max_usage_count: Option<u64>,
}

/// HSM key representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmKey {
    pub id: String,
    pub hsm_type: HsmTier,
    pub key_type: KeyType,
    pub attestation: Option<KeyAttestation>,
    pub created_at: DateTime<Utc>,
    pub metadata: KeyMetadata,
}

/// HSM key information (without sensitive data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmKeyInfo {
    /// Unique identifier for the key
    pub key_id: String,
    /// Type of cryptographic key (RSA, ECC, AES, etc.)
    pub key_type: KeyType,
    /// HSM tier that stores this key
    pub hsm_type: HsmTier,
    /// Timestamp when the key was created
    pub created_at: DateTime<Utc>,
    /// Usage policy defining allowed operations
    pub usage_policy: KeyUsagePolicy,
    /// Current health status of the key
    pub health_status: KeyHealthStatus,
}

/// Key attestation data providing proof of hardware backing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyAttestation {
    /// Level of attestation provided (software, hardware, certified)
    pub attestation_type: AttestationLevel,
    /// X.509 certificate chain proving key authenticity
    pub certificate_chain: Vec<Vec<u8>>,
    /// Raw attestation data from the HSM
    pub attestation_data: Vec<u8>,
    /// Digital signature of the attestation data
    pub attestation_signature: Vec<u8>,
    /// Whether the attestation has been verified
    pub verified: bool,
}

/// Health status of a cryptographic key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyHealthStatus {
    /// Key is healthy and operational
    Healthy,
    /// Key has warnings but is still functional
    Warning(String),
    /// Key is unhealthy and should not be used
    Unhealthy(String),
    /// Key health status is unknown
    Unknown,
}

/// Comprehensive HSM information and capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmInfo {
    /// HSM tier and configuration
    pub hsm_type: HsmTier,
    /// HSM vendor name
    pub vendor: String,
    /// HSM model identifier
    pub model: String,
    /// Software/firmware version
    pub version: String,
    /// List of supported HSM capabilities
    pub capabilities: Vec<HsmCapability>,
    /// Cryptographic algorithms supported by this HSM
    pub supported_algorithms: Vec<Algorithm>,
    /// Maximum key size supported (in bits)
    pub max_key_size: Option<u32>,
    /// Security certification level (FIPS 140-2, etc.)
    pub certification: Option<String>,
    /// Level of tamper resistance provided
    pub tamper_resistance: TamperResistanceLevel,
}

/// HSM operational capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum HsmCapability {
    /// Generate new cryptographic keys
    KeyGeneration,
    /// Import external keys into the HSM
    KeyImport,
    /// Export keys from the HSM (if allowed)
    KeyExport,
    /// Encrypt data using stored keys
    Encryption,
    /// Decrypt data using stored keys
    Decryption,
    /// Generate digital signatures
    Signing,
    /// Verify digital signatures
    Verification,
    /// Wrap keys with other keys
    KeyWrapping,
    /// Unwrap previously wrapped keys
    KeyUnwrapping,
    /// Derive new keys from existing keys
    KeyDerivation,
    /// Generate key attestation certificates
    KeyAttestation,
    /// Support biometric authentication
    BiometricAuthentication,
    /// Validate user presence for operations
    UserPresenceValidation,
    /// Detect physical tampering attempts
    TamperDetection,
    /// Securely backup HSM state
    SecureBackup,
    /// Restore HSM from secure backup
    SecureRestore,
    /// High availability clustering support
    HighAvailability,
    /// Load balancing across multiple HSMs
    LoadBalancing,
    /// HSM clustering for scalability
    Clustering,
}

/// Cryptographic algorithms supported by HSMs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Algorithm {
    // Symmetric encryption algorithms
    /// AES-128-GCM authenticated encryption
    Aes128Gcm,
    /// AES-192-GCM authenticated encryption
    Aes192Gcm,
    /// AES-256-GCM authenticated encryption
    Aes256Gcm,
    /// ChaCha20-Poly1305 authenticated encryption
    ChaCha20Poly1305,

    // Asymmetric algorithms
    /// RSA PKCS#1 v1.5 padding
    RsaPkcs1V15,
    /// RSA PSS (Probabilistic Signature Scheme)
    RsaPss,
    /// Elliptic Curve P-256
    EccP256,
    /// Elliptic Curve P-384
    EccP384,
    /// Elliptic Curve P-521
    EccP521,
    /// Ed25519 signature algorithm
    Ed25519,
    /// X25519 key agreement algorithm
    X25519,

    // Digital signature algorithms
    /// ECDSA with SHA-256
    EcdsaSha256,
    /// ECDSA with SHA-384
    EcdsaSha384,
    /// ECDSA with SHA-512
    EcdsaSha512,
    /// RSA with SHA-256
    RsaSha256,
    /// RSA with SHA-384
    RsaSha384,
    /// RSA with SHA-512
    RsaSha512,

    // Key agreement algorithms
    /// ECDH with P-256 curve
    EcdhP256,
    /// ECDH with P-384 curve
    EcdhP384,
    /// ECDH with P-521 curve
    EcdhP521,
    /// X25519 key agreement
    X25519KeyAgreement,

    // AEAD algorithms
    /// AES-GCM authenticated encryption
    AesGcm,
    /// ChaCha20-Poly1305 AEAD
    ChaCha20Poly1305Aead,

    // Key derivation algorithms
    /// HKDF with SHA-256
    HkdfSha256,
    /// HKDF with SHA-384
    HkdfSha384,
    /// HKDF with SHA-512
    HkdfSha512,
    /// PBKDF2 key derivation
    Pbkdf2,

    // Hash algorithms
    /// SHA-256 hash function
    Sha256,
    /// SHA-384 hash function
    Sha384,
    /// SHA-512 hash function
    Sha512,
    /// BLAKE2b hash function
    Blake2b,
    /// BLAKE2s hash function
    Blake2s,

    /// Custom algorithm implementation
    Custom(String),
}

/// HSM health monitoring information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmHealthStatus {
    /// Whether the HSM is currently healthy and operational
    pub healthy: bool,
    /// Timestamp of the last health check
    pub last_check: DateTime<Utc>,
    /// Error message if health check failed
    pub error_message: Option<String>,
    /// Performance metrics for this HSM
    pub performance_metrics: PerformanceMetrics,
}

/// Performance metrics for HSM operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Operations performed per second
    pub operations_per_second: f64,
    /// Average latency in milliseconds
    pub average_latency_ms: f64,
    /// Error rate as a percentage (0.0 to 1.0)
    pub error_rate: f64,
    /// Availability percentage (0.0 to 100.0)
    pub availability_percentage: f64,
}

/// Operation context for HSM operations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HsmOperation {
    /// Key generation operation
    KeyGeneration,
    /// Digital signature operation
    DigitalSignature,
    /// Data encryption operation
    DataEncryption,
    /// Data decryption operation
    DataDecryption,
    /// Key derivation operation
    KeyDerivation,
    /// Random number generation
    RandomGeneration,
    /// Cryptographic hash operation
    CryptographicHash,
    /// Certificate operations
    CertificateOperation,
    /// Attestation operations
    AttestationOperation,
    /// Entropy collection
    EntropyCollection,
    /// Biometric template operations
    BiometricTemplate,
    /// Secure storage operations
    SecureStorage,
    /// Key agreement operations
    KeyAgreement,
    /// Zero-knowledge proof operations
    ZeroKnowledgeProof,
    /// Audit and compliance operations
    AuditCompliance,
    /// Genetic evolution operations
    GeneticEvolution,
    /// Lineage proof operations
    LineageProof,
    /// Genetic witness operations
    GeneticWitness,
}

/// HSM configuration for different HSM types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmConfig {
    /// Type of HSM being configured
    pub hsm_type: HsmType,
    /// Key-value configuration data
    pub config_data: HashMap<String, String>,
    /// iOS-specific configuration
    pub ios_config: Option<IosHsmConfig>,
    /// Android-specific configuration
    pub android_config: Option<AndroidHsmConfig>,
    /// Software HSM configuration
    pub software_config: Option<SoftwareHsmConfig>,
    /// AWS CloudHSM configuration
    pub aws_config: Option<AwsHsmConfig>,
    /// SafeNet Luna HSM configuration
    pub luna_config: Option<LunaHsmConfig>,
}

/// Enumeration of HSM types for configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HsmType {
    /// iOS Secure Enclave HSM
    SmartphoneIos,
    /// Android StrongBox HSM
    SmartphoneAndroid,
    /// Rust-based software HSM
    SoftwareRust,
    /// AWS CloudHSM
    HardwareAws,
    /// SafeNet Luna HSM
    HardwareLuna,
    /// Thales nShield HSM
    HardwareThales,
    /// Utimaco HSM
    HardwareUtimaco,
    /// Custom HSM implementation
    Custom(String),
}

/// iOS Secure Enclave HSM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IosHsmConfig {
    /// iOS device model (iPhone 12, iPad Pro, etc.)
    pub device_model: String,
    /// iOS version running on the device
    pub ios_version: String,
    /// Secure Enclave version
    pub secure_enclave_version: String,
    /// Chip type (A14, A15, M1, etc.)
    pub chip_type: String,
    /// Whether biometric authentication is supported
    pub biometric_support: bool,
    /// Keychain access group for shared keys
    pub keychain_access_group: Option<String>,
    /// Key attestation configuration
    pub attestation_config: AttestationConfig,
}

/// Android StrongBox HSM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidHsmConfig {
    /// Device manufacturer (Google, Samsung, etc.)
    pub manufacturer: String,
    /// Device model (Pixel 8a, Galaxy S23, etc.)
    pub model: String,
    /// Android version
    pub android_version: String,
    /// StrongBox implementation version
    pub strongbox_version: Option<String>,
    /// Specific StrongBox implementation
    pub strongbox_implementation: StrongBoxImplementation,
    /// Android Keystore configuration
    pub keystore_config: KeystoreConfig,
    /// Key attestation configuration
    pub attestation_config: AttestationConfig,
}

/// Software HSM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareHsmConfig {
    /// Software HSM implementation type
    pub implementation: SoftwareHsmType,
    /// Key storage configuration
    pub key_store_config: KeyStoreConfig,
    /// Memory protection configuration
    pub memory_config: MemoryConfig,
    /// Cryptographic backend to use
    pub crypto_backend: CryptoBackend,
}

/// AWS CloudHSM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsHsmConfig {
    /// AWS region where HSM cluster is located
    pub region: String,
    /// CloudHSM cluster identifier
    pub cluster_id: String,
    /// HSM username for authentication
    pub username: String,
    /// AWS Secrets Manager ARN for HSM password
    pub password_secret_arn: String,
    /// HSM cluster CA certificate
    pub ca_certificate: String,
    /// Client certificate for authentication
    pub client_certificate: String,
    /// Client private key for authentication
    pub client_private_key: String,
}

/// SafeNet Luna HSM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LunaHsmConfig {
    /// Path to Luna client library
    pub library_path: String,
    /// HSM slot number
    pub slot_number: u32,
    /// Token label for identification
    pub token_label: String,
    /// Security Officer PIN
    pub so_pin: String,
    /// User PIN for authentication
    pub user_pin: String,
    /// High availability group name
    pub ha_group: Option<String>,
}

/// Key attestation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationConfig {
    /// Whether attestation is enabled
    pub enabled: bool,
    /// Require hardware-backed attestation
    pub require_hardware_backed: bool,
    /// Trusted root certificates for validation
    pub trusted_certificates: Vec<Vec<u8>>,
    /// Length of attestation challenge in bytes
    pub challenge_length: usize,
}

/// Android Keystore configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeystoreConfig {
    /// Prefix for key aliases
    pub alias_prefix: String,
    /// Whether user authentication is required
    pub require_user_authentication: bool,
    /// User authentication validity duration in seconds
    pub user_authentication_validity_duration: Option<u32>,
    /// Whether to require StrongBox backing
    pub require_strongbox: bool,
}

/// Key storage configuration for software HSMs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyStoreConfig {
    /// Type of key storage backend
    pub storage_type: KeyStorageType,
    /// Source of encryption keys for storage
    pub encryption_key_source: KeySource,
    /// Whether to enable automatic backups
    pub backup_enabled: bool,
    /// Size of in-memory key cache
    pub cache_size: usize,
    /// File storage specific configuration
    pub file_config: Option<FileStorageConfig>,
    /// Database storage specific configuration
    pub db_config: Option<DatabaseConfig>,
}

/// Source of encryption keys for key storage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeySource {
    /// Derive keys from master key
    Derived,
    /// Use externally provided keys
    External,
    /// Use hardware-derived keys
    Hardware,
}

/// File storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStorageConfig {
    /// Directory path for key storage
    pub storage_path: String,
    /// File permissions (Unix octal format)
    pub file_permissions: u32,
    /// Optional backup directory path
    pub backup_path: Option<String>,
}

/// Database storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database connection string
    pub connection_string: String,
    /// Table name for key storage
    pub table_name: String,
    /// Whether to encrypt data at rest
    pub encryption_enabled: bool,
}

/// Memory protection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Level of memory protection to apply
    pub protection_level: MemoryProtectionLevel,
    /// Whether to use secure memory allocator
    pub use_secure_allocator: bool,
    /// Whether to zero memory on free
    pub zero_on_free: bool,
    /// Whether to use guard pages
    pub use_guard_pages: bool,
}

/// Cryptographic backend implementations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CryptoBackend {
    /// RustCrypto ecosystem crates
    RustCrypto,
    /// OpenSSL library
    OpenSsl,
    /// Ring cryptography library
    Ring,
    /// Custom crypto implementation
    Custom(String),
}

/// Request structure for key generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateKeyRequest {
    /// Unique identifier for the new key
    pub key_id: String,
    /// Type of key to generate
    pub key_type: KeyType,
    /// Usage policy for the key
    pub usage_policy: KeyUsagePolicy,
    /// Metadata for the key
    pub metadata: KeyMetadata,
    /// Whether user presence is required
    pub require_user_presence: bool,
    /// Optional attestation challenge
    pub attestation_challenge: Option<Vec<u8>>,
}

// Default implementations
impl Default for KeyUsagePolicy {
    fn default() -> Self {
        Self {
            can_encrypt: true,
            can_decrypt: true,
            can_sign: true,
            can_verify: true,
            can_wrap: false,
            can_unwrap: false,
            can_derive: false,
            exportable: false,
            user_presence_required: false,
            max_usage_count: None,
        }
    }
}

impl Default for AttestationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            require_hardware_backed: false,
            trusted_certificates: vec![],
            challenge_length: 32,
        }
    }
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            protection_level: MemoryProtectionLevel::High,
            use_secure_allocator: true,
            zero_on_free: true,
            use_guard_pages: false,
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            operations_per_second: 0.0,
            average_latency_ms: 0.0,
            error_rate: 0.0,
            availability_percentage: 100.0,
        }
    }
}

impl Default for SoftwareHsmConfig {
    fn default() -> Self {
        Self {
            implementation: SoftwareHsmType::RustSoftwareHsm,
            key_store_config: KeyStoreConfig::default(),
            memory_config: MemoryConfig::default(),
            crypto_backend: CryptoBackend::RustCrypto,
        }
    }
}

impl Default for AndroidHsmConfig {
    fn default() -> Self {
        Self {
            manufacturer: "Google".to_string(),
            model: "Pixel".to_string(),
            android_version: "13".to_string(),
            strongbox_version: Some("1.0".to_string()),
            strongbox_implementation: StrongBoxImplementation::TitanM {
                version: "1.0".to_string(),
                security_level: "StrongBox".to_string(),
            },
            keystore_config: KeystoreConfig::default(),
            attestation_config: AttestationConfig::default(),
        }
    }
}

impl Default for KeyStoreConfig {
    fn default() -> Self {
        Self {
            storage_type: KeyStorageType::EncryptedFile,
            encryption_key_source: KeySource::Derived,
            backup_enabled: false,
            cache_size: 1000,
            file_config: Some(FileStorageConfig::default()),
            db_config: None,
        }
    }
}

impl Default for FileStorageConfig {
    fn default() -> Self {
        Self {
            storage_path: "/tmp/hsm_keys".to_string(),
            file_permissions: 0o600,
            backup_path: None,
        }
    }
}

impl Default for KeystoreConfig {
    fn default() -> Self {
        Self {
            alias_prefix: "beardog_".to_string(),
            require_user_authentication: false,
            user_authentication_validity_duration: None,
            require_strongbox: false,
        }
    }
}
