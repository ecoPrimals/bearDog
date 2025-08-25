// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// HSM tier enumeration - defines the different types of HSMs available  
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
/// Smartphone device types
pub enum SmartphoneType {
    /// iPhone device with iOS and Secure Enclave
    IPhone {
        /// iPhone model (e.g., "iPhone 14 Pro")}


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
        /// Android version (e.g., "13", "14")
        android_version: String,
        /// Optional StrongBox version identifier
        strongbox_version: Option<String>,
}
/// Secure enclave types for smartphone HSMs
pub enum SecureEnclaveType {
    /// iOS Secure Enclave
    IosSecureEnclave {
        /// Chip type (A-series, M-series)
        chip_type: String, // A-series, M-series
        /// Whether biometric authentication is supported
        biometric_support: bool,
        /// Whether key attestation is supported
        key_attestation: bool,
    /// Android StrongBox
    AndroidStrongBox {
        /// StrongBox implementation type}


        implementation: StrongBoxImplementation,
        /// Whether the implementation is hardware-backed
        hardware_backed: bool,
    /// Generic trusted execution environment
    TrustedExecutionEnvironment {
        /// TEE vendor name
        vendor: String,
        /// Type of trusted execution environment
        tee_type: String,
        /// Optional security certification
        certification: Option<String>,
/// StrongBox implementation types}


pub enum StrongBoxImplementation {
    /// Google Titan M chip
    TitanM {
        /// Titan M chip version
        version: String,
        /// Security level provided by the chip
        security_level: String,
    /// Qualcomm Secure Processing Unit
    QualcommSpu {
        /// SPU version
        /// Type of SPU implementation
        spu_type: String,
    /// Samsung Knox
    SamsungKnox {
        /// Knox version
        /// Knox security level
    /// Generic StrongBox implementation
    Generic {
        /// Vendor name
        /// Implementation version}


impl Default for StrongBoxImplementation {
    fn default() -> Self {
        Self::Generic {
            vendor: "Generic".to_string(),
            version: "1.0".to_string(),
        }
    }
/// Software HSM implementation types
pub enum SoftwareHsmType {
    /// Rust-based implementation (default)
    RustSoftwareHsm,
    /// OpenSSL-based implementation
    OpenSslSoftwareHsm,
    /// Custom implementation
    Custom(String),
/// Key storage backend types}


pub enum KeyStorageType {
    /// In-memory storage (not persistent)
    InMemory,
    /// Encrypted file storage
    EncryptedFile,
    /// Database storage
    Database,
    /// Custom storage backend
/// Memory protection level for key material}


pub enum MemoryProtectionLevel {
    /// No special memory protection
    None,
    /// Basic memory protection
    Basic,
    /// High memory protection with secure allocation
    High,
    /// Enhanced memory protection with encryption
    Enhanced,
    /// Maximum security with hardware-backed protection
    Maximum,
/// HSM vendor enumeration
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
/// Security certification levels}


pub enum CertificationLevel {
    /// No formal certification
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
/// Tamper resistance levels
pub enum TamperResistanceLevel {
    /// No tamper resistance
    /// Tamper evident (detects tampering)
    TamperEvident,
    /// Tamper resistant (resists tampering)
    TamperResistant,
    /// Tamper responsive (responds to tampering)
    TamperResponsive,
    /// Hardware tamper resistance
    Hardware,
    /// Software tamper resistance
    Software,
    /// Hardware destruction on tampering
    HardwareDestruction,
/// Key hierarchy management strategies}


pub enum KeyHierarchy {
    /// Flat key hierarchy (all keys at same level)
    Flat,
    /// Hierarchical key derivation
    Hierarchical,
    /// Tree-based key hierarchy
    Tree,
    /// Custom hierarchy strategy
/// Fallback strategies for HSM failures
pub enum FallbackStrategy {
    /// Fail immediately on HSM failure
    FailFast,
    /// Fallback to next available HSM tier
    FallbackToNext,
    /// Fallback to software HSM
    FallbackToSoftware,
    /// Custom fallback strategy
/// Attestation levels for device security}


pub enum AttestationLevel {
    /// No attestation available
    /// Basic attestation (software-based)
    /// Hardware-backed attestation
    /// Strong attestation with remote verification
    Strong,
    /// Certified hardware attestation
    CertifiedHardware,
// Add the missing AndroidKeyAlgorithm type
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
// Duplicate HsmTier enum removed - using the detailed version above
// The simple Hardware/Software/Hybrid/Cloud variants are available
// through the detailed enum's pattern matching
