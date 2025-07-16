use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// HSM tier enumeration - defines the different types of HSMs available  
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum HsmTier {
    /// Tier 1: Smartphone HSM (Most Accessible)
    /// - iOS Secure Enclave, Android StrongBox
    /// - Always available, user-controlled
    /// - Good for personal keys, authentication
    SmartphoneHsm {
        /// Type of smartphone device (iPhone or Android)
        device_type: SmartphoneType,
        /// Secure enclave implementation type
        secure_enclave: SecureEnclaveType,
        /// Level of attestation provided by the device
        attestation_level: AttestationLevel,
        /// Whether user presence is required for operations
        user_presence_required: bool,
    },

    /// Tier 2: Software HSM (Scalable)
    /// - Rust-based implementation
    /// - Encrypted key storage
    /// - Good for development, testing, light production
    SoftwareHsm {
        /// Software HSM implementation type
        implementation: SoftwareHsmType,
        /// Key storage backend configuration
        key_storage: KeyStorageType,
        /// Whether encryption at rest is enabled
        encryption_at_rest: bool,
        /// Memory protection level for key material
        memory_protection: MemoryProtectionLevel,
    },

    /// Tier 3: Hardware HSM (Highest Security)
    /// - FIPS 140-2 Level 3+ certified
    /// - Tamper-resistant hardware
    /// - Good for root keys, high-value operations
    HardwareHsm {
        /// Hardware HSM vendor
        vendor: HsmVendor,
        /// HSM model identifier
        model: String,
        /// Security certification level
        certification: CertificationLevel,
        /// Tamper resistance capabilities
        tamper_resistance: TamperResistanceLevel,
    },

    /// Tier 4: Hybrid HSM (Best of All Worlds)
    /// - Combines multiple HSM types
    /// - Hierarchical key management
    /// - Optimal security and performance
    HybridHsm {
        /// List of HSM tiers in the hybrid configuration
        tiers: Vec<Box<HsmTier>>,
        /// Key hierarchy management strategy
        key_hierarchy: KeyHierarchy,
        /// Fallback strategy for HSM failures
        fallback_strategy: Box<FallbackStrategy>,
    },
}

/// Smartphone device types
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SmartphoneType {
    /// iPhone device with iOS and Secure Enclave
    IPhone {
        /// iPhone model (e.g., "iPhone 14 Pro")
        model: String,
        /// iOS version (e.g., "16.0")
        ios_version: String,
        /// Secure Enclave version identifier
        secure_enclave_version: String,
    },
    /// Android device with optional StrongBox support
    Android {
        /// Device manufacturer (e.g., "Google", "Samsung")
        manufacturer: String,
        /// Device model (e.g., "Pixel 7", "Galaxy S23")
        model: String,
        /// Android version (e.g., "13", "14")
        android_version: String,
        /// Optional StrongBox version identifier
        strongbox_version: Option<String>,
    },
}

/// Secure enclave types for smartphone HSMs
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SecureEnclaveType {
    /// iOS Secure Enclave
    IosSecureEnclave {
        /// Chip type (A-series, M-series)
        chip_type: String, // A-series, M-series
        /// Whether biometric authentication is supported
        biometric_support: bool,
        /// Whether key attestation is supported
        key_attestation: bool,
    },

    /// Android StrongBox
    AndroidStrongBox {
        /// StrongBox implementation type
        implementation: StrongBoxImplementation,
        /// Whether the implementation is hardware-backed
        hardware_backed: bool,
        /// Whether key attestation is supported
        key_attestation: bool,
    },

    /// Generic trusted execution environment
    TrustedExecutionEnvironment {
        /// TEE vendor name
        vendor: String,
        /// Type of trusted execution environment
        tee_type: String,
        /// Optional security certification
        certification: Option<String>,
    },
}

/// StrongBox implementation types
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum StrongBoxImplementation {
    /// Google Titan M chip
    TitanM {
        /// Titan M chip version
        version: String,
        /// Security level provided by the chip
        security_level: String,
    },
    /// Qualcomm Secure Processing Unit
    QualcommSpu { 
        /// SPU version
        version: String, 
        /// Type of SPU implementation
        spu_type: String 
    },
    /// Samsung Knox
    SamsungKnox { 
        /// Knox version
        version: String, 
        /// Knox security level
        security_level: String 
    },
    /// Generic StrongBox implementation
    Generic { 
        /// Vendor name
        vendor: String, 
        /// Implementation version
        version: String 
    },
}

/// Software HSM implementation types
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SoftwareHsmType {
    /// Rust-based implementation (default)
    RustSoftwareHsm,
    /// OpenSSL-based implementation
    OpenSslSoftwareHsm,
    /// Custom implementation
    Custom(String),
}

/// Key storage backend types
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum KeyStorageType {
    /// Encrypted file storage
    EncryptedFile,
    /// In-memory storage (temporary)
    InMemory,
    /// Database storage
    Database,
}

/// Memory protection levels for key material
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MemoryProtectionLevel {
    /// No special memory protection
    None,
    /// Basic memory protection (encrypted)
    Basic,
    /// Advanced memory protection (encrypted + locked)
    Advanced,
    /// Hardware-backed memory protection
    Hardware,
}

/// HSM vendor enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum HsmVendor {
    /// Thales (formerly SafeNet)
    Thales,
    /// Gemalto (now part of Thales)
    Gemalto,
    /// Utimaco
    Utimaco,
    /// AWS CloudHSM
    Aws,
    /// Entrust
    Entrust,
    /// Custom vendor
    Custom(String),
}

/// Security certification levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CertificationLevel {
    /// No formal certification
    None,
    /// FIPS 140-2 Level 1
    Fips140Level1,
    /// FIPS 140-2 Level 2
    Fips140Level2,
    /// FIPS 140-2 Level 3
    Fips140Level3,
    /// FIPS 140-2 Level 4
    Fips140Level4,
    /// Common Criteria EAL4+
    CommonCriteriaEal4Plus,
    /// Custom certification
    Custom(String),
}

/// Tamper resistance levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TamperResistanceLevel {
    /// No tamper resistance
    None,
    /// Tamper evident (detects tampering)
    TamperEvident,
    /// Tamper resistant (resists tampering)
    TamperResistant,
    /// Tamper responsive (responds to tampering)
    TamperResponsive,
}

/// Key hierarchy management strategies
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum KeyHierarchy {
    /// Flat key hierarchy (all keys at same level)
    Flat,
    /// Hierarchical key derivation
    Hierarchical,
    /// Tree-based key hierarchy
    Tree,
    /// Custom hierarchy strategy
    Custom(String),
}

/// Fallback strategies for HSM failures
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FallbackStrategy {
    /// Fail immediately on HSM failure
    FailFast,
    /// Fallback to next available HSM tier
    FallbackToNext,
    /// Fallback to software HSM
    FallbackToSoftware,
    /// Custom fallback strategy
    Custom(String),
}

/// Attestation levels for device security
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AttestationLevel {
    /// No attestation available
    None,
    /// Basic attestation (software-based)
    Basic,
    /// Hardware-backed attestation
    Hardware,
    /// Strong attestation with remote verification
    Strong,
} 