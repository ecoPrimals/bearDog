// SPDX-License-Identifier: AGPL-3.0-only

//! Core HSM Types and Configuration
//!
//! Hardware Security Module integration types supporting multiple HSM backends
//! including software HSMs, hardware devices, cloud services, and PKCS#11 interfaces.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core HSM Configuration
///
/// Central configuration for Hardware Security Module integration.
/// Supports multiple HSM types (hardware, software, cloud) with
/// comprehensive algorithm and capability detection.
///
/// ## Example
///
/// ```rust
/// use beardog_types::canonical::hsm_unified::core::{HsmCoreConfig, HsmType};
///
/// let config = HsmCoreConfig {
///     enabled: true,
///     hsm_type: HsmType::Hardware,
///     name: "production-hsm".to_string(),
///     version: "2.0".to_string(),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HsmCoreConfig {
    /// Whether HSM integration is enabled
    ///
    /// When disabled, operations fall back to software implementations.
    pub enabled: bool,

    /// Type of HSM backend being used
    ///
    /// Determines which HSM provider to use (hardware, software, cloud, etc.).
    pub hsm_type: HsmType,

    /// Human-readable HSM name or identifier
    ///
    /// Used for logging, monitoring, and multi-HSM configurations.
    pub name: String,

    /// HSM version string
    ///
    /// Version of the HSM hardware, firmware, or software implementation.
    pub version: String,

    /// Cryptographic algorithms supported by this HSM
    ///
    /// List of algorithms that this HSM can perform (RSA, ECDSA, AES, etc.).
    pub supported_algorithms: Vec<HsmAlgorithm>,

    /// HSM capabilities and features
    ///
    /// Functional capabilities like key generation, signing, encryption, etc.
    pub capabilities: Vec<HsmCapability>,

    /// Additional HSM-specific metadata
    ///
    /// Custom key-value pairs for HSM-specific configuration options.
    pub metadata: HashMap<String, String>,
}

/// HSM Type - Hardware Security Module Backend Types
///
/// Defines the various types of HSM backends supported by BearDog.
/// Each type has different characteristics, performance profiles, and use cases.
///
/// ## HSM Types
///
/// - **Software** - Software-based HSM (default, no hardware required)
/// - **Hardware** - Dedicated HSM hardware (highest security)
/// - **Network** - Network-attached HSM (shared across systems)
/// - **Cloud** - Cloud-based HSM service (AWS CloudHSM, Azure Key Vault)
/// - **UsbToken** - USB security token (YubiKey, etc.)
/// - **SmartCard** - Smart card HSM
/// - **Mobile** - Mobile device secure enclave
/// - **Pkcs11** - PKCS#11 standard interface HSM
///
/// ## Example
///
/// ```rust
/// use beardog_types::canonical::hsm_unified::core::HsmType;
///
/// // Production: Use hardware HSM for maximum security
/// let prod_hsm = HsmType::Hardware;
///
/// // Development: Use software HSM for convenience
/// let dev_hsm = HsmType::Software;
///
/// // Cloud deployment: Use cloud HSM service
/// let cloud_hsm = HsmType::Cloud;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
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
