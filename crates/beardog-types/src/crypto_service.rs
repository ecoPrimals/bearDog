// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Crypto Service Types
//!
//! Protocol-agnostic types for crypto service operations.
//! These types are designed to work across JSON-RPC, HTTP, and any future protocols.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::SystemTime;

/// Supported encryption algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CryptoAlgorithm {
    /// AES-256-GCM (recommended)
    Aes256Gcm,
    /// ChaCha20-Poly1305 (lightweight, mobile-friendly)
    ChaCha20Poly1305,
    /// AES-128-GCM (faster, lower security margin)
    Aes128Gcm,
}

impl CryptoAlgorithm {
    /// Convert to canonical string name
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Aes256Gcm => "aes-256-gcm",
            Self::ChaCha20Poly1305 => "chacha20-poly1305",
            Self::Aes128Gcm => "aes-128-gcm",
        }
    }
}

/// Encrypted data with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    /// Ciphertext (encrypted plaintext) - base64 encoded in JSON
    pub ciphertext: Vec<u8>,

    /// Algorithm used for encryption
    pub algorithm: CryptoAlgorithm,

    /// Encryption metadata (nonce, tag, etc.)
    pub metadata: EncryptionMetadata,
}

/// Metadata for encrypted data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionMetadata {
    /// When the encryption was performed
    #[serde(with = "time_serde")]
    pub timestamp: SystemTime,

    /// Key ID that was used (not the key itself!)
    pub key_id: Option<String>,

    /// Nonce/IV used for encryption - base64 encoded in JSON
    pub nonce: Vec<u8>,

    /// Authentication tag (for AEAD ciphers) - base64 encoded in JSON
    pub tag: Option<Vec<u8>>,
}

/// Options for encryption
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EncryptOptions {
    /// Key identifier (not the key itself)
    pub key_id: String,

    /// Optional additional authenticated data (AAD) - base64 encoded in JSON
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_data: Option<Vec<u8>>,
}

/// Options for decryption
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DecryptOptions {
    /// Key identifier
    pub key_id: String,

    /// Optional additional authenticated data (AAD) - must match encryption - base64 encoded in JSON
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_data: Option<Vec<u8>>,
}

/// Supported signature algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignatureAlgorithm {
    /// Ed25519 (recommended for most use cases)
    Ed25519,
    /// ECDSA with P-256 curve
    EcdsaP256,
    /// RSA-PSS with 4096-bit key
    RsaPss,
}

impl SignatureAlgorithm {
    /// Convert to canonical string name
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Ed25519 => "ed25519",
            Self::EcdsaP256 => "ecdsa-p256",
            Self::RsaPss => "rsa-pss",
        }
    }
}

/// Digital signature with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    /// Signature bytes - base64 encoded in JSON
    pub signature: Vec<u8>,

    /// Algorithm used for signing
    pub algorithm: SignatureAlgorithm,

    /// Signature metadata
    pub metadata: SignatureMetadata,
}

/// Metadata for signatures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureMetadata {
    /// When the signature was created
    #[serde(with = "time_serde")]
    pub timestamp: SystemTime,

    /// Key ID used for signing
    pub key_id: Option<String>,

    /// Optional context string
    pub context: Option<String>,
}

/// Options for signing
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SignOptions {
    /// Signing key identifier
    pub key_id: String,

    /// Optional context string (domain separation)
    pub context: Option<String>,
}

/// Options for signature verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyOptions {
    /// Public key for verification - base64 encoded in JSON
    pub public_key: Vec<u8>,

    /// Optional context string (must match signing context)
    pub context: Option<String>,
}

/// Supported key algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyAlgorithm {
    /// AES-256 symmetric key
    Aes256,
    /// ChaCha20-Poly1305 symmetric key
    ChaCha20Poly1305,
    /// Ed25519 signing key
    Ed25519,
    /// ECDSA P-256 key
    EcdsaP256,
    /// RSA 4096-bit key
    Rsa4096,
}

impl KeyAlgorithm {
    /// Convert to canonical string name
    ///
    /// Returns the full algorithm identifier including cipher mode where applicable.
    /// Modern Rust: Prefer complete, unambiguous identifiers over abbreviations.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Aes256 => "aes-256-gcm",
            Self::ChaCha20Poly1305 => "chacha20-poly1305",
            Self::Ed25519 => "ed25519",
            Self::EcdsaP256 => "ecdsa-p256",
            Self::Rsa4096 => "rsa-4096",
        }
    }
}

/// Key information (metadata only - never includes actual key material)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyInfo {
    /// Unique key identifier
    pub key_id: String,

    /// Key algorithm
    pub algorithm: KeyAlgorithm,

    /// When the key was created
    #[serde(with = "time_serde")]
    pub created_at: SystemTime,

    /// Key metadata
    pub metadata: KeyMetadata,
}

/// Metadata for keys
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadata {
    /// Key size in bits
    pub size_bits: u32,

    /// Whether key is backed by HSM
    pub hsm_backed: bool,

    /// Whether key was generated with genetic mixing
    pub genetic_mixed: bool,

    /// Intended purpose
    pub purpose: Option<String>,
}

/// Options for key generation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyGenOptions {
    /// Optional manual key ID (if not provided, one will be auto-generated)
    /// Modern Rust idiom: explicit Option for optional values
    pub key_id: Option<String>,

    /// Use HSM for key generation
    #[serde(default)]
    pub use_hsm: bool,

    /// Use genetic key mixing
    #[serde(default)]
    pub use_genetic: bool,

    /// Intended purpose
    pub purpose: Option<String>,

    /// Additional metadata
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
}

/// Service capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCapabilities {
    /// Service name
    pub service_name: String,

    /// Service version
    pub version: String,

    /// Supported algorithms (canonical names)
    pub supported_algorithms: Vec<String>,

    /// Available features (hsm, genetic, audit, etc.)
    pub features: Vec<String>,

    /// Maximum data size for single operation (bytes)
    pub max_data_size: usize,
}

/// Service health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Overall health status
    pub healthy: bool,

    /// Service uptime in seconds
    pub uptime_seconds: u64,

    /// Number of operations completed
    pub operations_completed: u64,

    /// HSM connection status
    pub hsm_connected: bool,
}

/// Custom serde module for `SystemTime`
mod time_serde {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = time
            .duration_since(UNIX_EPOCH)
            .map_err(serde::ser::Error::custom)?;
        serializer.serialize_u64(duration.as_secs())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(UNIX_EPOCH + std::time::Duration::from_secs(secs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_serialization() {
        let algo = CryptoAlgorithm::Aes256Gcm;
        let json = serde_json::to_string(&algo).expect("Failed to serialize");
        // Serde uses PascalCase -> snake_case by default
        assert_eq!(json, "\"aes256_gcm\"");

        let deserialized: CryptoAlgorithm =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, algo);
    }

    #[test]
    fn test_encrypted_data_roundtrip() {
        let encrypted = EncryptedData {
            ciphertext: vec![1, 2, 3, 4, 5],
            algorithm: CryptoAlgorithm::Aes256Gcm,
            metadata: EncryptionMetadata {
                timestamp: SystemTime::now(),
                key_id: Some("test-key".to_string()),
                nonce: vec![0; 12],
                tag: Some(vec![0; 16]),
            },
        };

        let json = serde_json::to_string(&encrypted).expect("Failed to serialize");
        let deserialized: EncryptedData =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(deserialized.ciphertext, encrypted.ciphertext);
        assert_eq!(deserialized.algorithm, encrypted.algorithm);
    }

    #[test]
    fn test_signature_algorithm_names() {
        assert_eq!(SignatureAlgorithm::Ed25519.as_str(), "ed25519");
        assert_eq!(SignatureAlgorithm::EcdsaP256.as_str(), "ecdsa-p256");
        assert_eq!(SignatureAlgorithm::RsaPss.as_str(), "rsa-pss");
    }

    #[test]
    fn test_key_algorithm_names() {
        // Modern Rust: Use complete, unambiguous algorithm identifiers
        assert_eq!(KeyAlgorithm::Aes256.as_str(), "aes-256-gcm");
        assert_eq!(KeyAlgorithm::ChaCha20Poly1305.as_str(), "chacha20-poly1305");
        assert_eq!(KeyAlgorithm::Ed25519.as_str(), "ed25519");
        assert_eq!(KeyAlgorithm::EcdsaP256.as_str(), "ecdsa-p256");
        assert_eq!(KeyAlgorithm::Rsa4096.as_str(), "rsa-4096");
    }

    #[test]
    fn test_capabilities_serialization() {
        let caps = ServiceCapabilities {
            service_name: "beardog".to_string(),
            version: "1.0.0".to_string(),
            supported_algorithms: vec!["aes-256-gcm".to_string(), "ed25519".to_string()],
            features: vec!["hsm".to_string(), "genetic".to_string()],
            max_data_size: 10 * 1024 * 1024,
        };

        let json = serde_json::to_string(&caps).expect("Failed to serialize");
        let deserialized: ServiceCapabilities =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(deserialized.service_name, caps.service_name);
        assert_eq!(deserialized.supported_algorithms, caps.supported_algorithms);
    }

    // Include additional validation tests
    mod crypto_service_validation_tests {
        include!("crypto_service/tests/crypto_service_validation_tests.rs");
    }
}
