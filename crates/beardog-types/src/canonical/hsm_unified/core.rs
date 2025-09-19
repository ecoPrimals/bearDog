// Core HSM Types and Configuration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core HSM configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HsmCoreConfig {
    /// HSM enabled
    /// Whether feature is enabled
    pub enabled: bool,

    /// HSM type
    /// The hsm type value
    pub hsm_type: HsmType,

    /// HSM name/identifier
    /// Name of the item
    pub name: String,

    /// HSM version
    /// The version value
    pub version: String,

    /// Supported algorithms
    /// Collection of supported algorithms
    pub supported_algorithms: Vec<HsmAlgorithm>,

    /// HSM capabilities
    /// Collection of capabilities
    pub capabilities: Vec<HsmCapability>,

    /// Configuration metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

/// HSM types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
/// Types of hsm
pub enum HsmType {
    /// Software-based HSM implementation (default)
    #[default]
    /// Represents software variant
    Software,
    /// Hardware variant
    Hardware,
    /// Network variant
    Network,
    /// Cloud variant
    Cloud,
    /// `UsbToken` variant
    UsbToken,
    /// `SmartCard` variant
    SmartCard,
    /// Mobile variant
    Mobile,
    /// PKCS#11 standard interface HSM
    Pkcs11,
}

/// HSM algorithms
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HsmAlgorithm {
    /// RSA 2048-bit key
    Rsa2048,
    /// RSA 4096-bit key
    Rsa4096,
    /// ECDSA P-256 curve
    EcdsaP256,
    /// ECDSA P-384 curve
    EcdsaP384,
    /// ECDSA P-521 curve
    EcdsaP521,
    /// AES 128-bit symmetric key
    Aes128,
    /// AES 256-bit symmetric key
    Aes256,
    /// `ChaCha20` stream cipher key
    ChaCha20,
    /// Ed25519 signature key
    Ed25519,
    /// X25519 key exchange key
    X25519,
    /// Post-quantum cryptographic algorithm
    PostQuantum(String),
}

/// HSM capabilities
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HsmCapability {
    /// `KeyGeneration` variant
    KeyGeneration,
    /// `KeyStorage` variant
    KeyStorage,
    /// Signing variant
    Signing,
    /// Encryption variant
    Encryption,
    /// Decryption variant
    Decryption,
    /// `KeyDerivation` variant
    KeyDerivation,
    /// `TamperDetection` variant
    TamperDetection,
    /// Attestation variant
    Attestation,
    /// Backup variant
    Backup,
    /// `HighAvailability` variant
    HighAvailability,
}
