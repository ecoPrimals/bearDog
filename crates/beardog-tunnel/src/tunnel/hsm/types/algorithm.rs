// SPDX-License-Identifier: AGPL-3.0-or-later

//! Cryptographic Algorithm Types
//!
//! Type definitions for cryptographic algorithms supported by the HSM.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Algorithm parameters configuration
#[derive(Debug, Clone)]
pub struct AlgorithmParameters {
    /// Cryptographic algorithm
    pub algorithm: CryptographicAlgorithm,
    /// Optional key size in bits
    pub key_size: Option<u32>,
    /// Algorithm-specific parameters
    pub parameters: AlgorithmSpecificParameters,
}

impl Default for AlgorithmParameters {
    fn default() -> Self {
        Self {
            algorithm: CryptographicAlgorithm::Aes256Gcm,
            key_size: Some(256),
            parameters: AlgorithmSpecificParameters::Aes {
                mode: AesMode::Gcm,
                iv: None,
                aad: None,
            },
        }
    }
}

/// Algorithm-specific parameters
#[derive(Debug, Clone)]
pub enum AlgorithmSpecificParameters {
    /// AES encryption parameters
    Aes {
        /// Cipher mode
        mode: AesMode,
        /// Initialization vector
        iv: Option<Vec<u8>>,
        /// Additional authenticated data
        aad: Option<Vec<u8>>,
    },
    /// RSA encryption parameters
    Rsa {
        /// Public exponent (typically 65537)
        public_exponent: Option<u32>,
        /// Padding scheme
        padding: RsaPadding,
        /// Hash algorithm for OAEP/PSS
        hash_algorithm: Option<HashAlgorithm>,
        /// Optional label for OAEP
        label: Option<Vec<u8>>,
    },
    /// Elliptic curve parameters
    EllipticCurve {
        /// Elliptic curve type
        curve: EllipticCurveType,
        /// Use point compression
        point_compression: bool,
    },
    /// HKDF parameters
    Hkdf {
        /// Hash algorithm
        hash_algorithm: HashAlgorithm,
        /// Salt value
        salt: Option<Vec<u8>>,
        /// Info/context string
        info: Option<Vec<u8>>,
        /// Output length in bytes
        output_length: u32,
    },
    /// PBKDF2 parameters
    Pbkdf2 {
        /// Salt value
        salt: Vec<u8>,
        /// Number of iterations
        iterations: u32,
        /// Hash algorithm
        hash_algorithm: HashAlgorithm,
    },
    /// Generic parameters for custom algorithms
    Generic {
        /// Key-value parameters
        parameters: HashMap<String, String>,
    },
}

/// Hash algorithm types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HashAlgorithm {
    /// SHA-1 (deprecated, use for legacy only)
    Sha1,
    /// SHA-256
    Sha256,
    /// SHA-384
    Sha384,
    /// SHA-512
    Sha512,
    /// SHA-512/224
    Sha512_224,
    /// SHA-512/256
    Sha512_256,
    /// SHA3-256
    Sha3_256,
    /// SHA3-384
    Sha3_384,
    /// SHA3-512
    Sha3_512,
    /// `BLAKE2b`
    Blake2b,
    /// BLAKE2s
    Blake2s,
}

/// AES cipher modes
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AesMode {
    /// Electronic Codebook (deprecated)
    Ecb,
    /// Cipher Block Chaining
    Cbc,
    /// Counter mode
    Ctr,
    /// Galois/Counter Mode (AEAD)
    Gcm,
    /// Cipher Feedback
    Cfb,
    /// Output Feedback
    Ofb,
}

/// RSA padding schemes
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RsaPadding {
    /// No padding (raw RSA)
    None,
    /// PKCS#1 v1.5 padding
    Pkcs1v15,
    /// OAEP padding
    Oaep,
    /// PSS padding (for signatures)
    Pss,
}

/// Elliptic curve types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EllipticCurveType {
    /// NIST P-256 (secp256r1)
    P256,
    /// NIST P-384 (secp384r1)
    P384,
    /// NIST P-521 (secp521r1)
    P521,
    /// secp256k1 (Bitcoin curve)
    Secp256k1,
    /// Curve25519 (for Ed25519)
    Curve25519,
    /// Curve448 (for Ed448)
    Curve448,
}

/// Cryptographic algorithms
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CryptographicAlgorithm {
    /// AES-128-GCM
    Aes128Gcm,
    /// AES-256-GCM
    Aes256Gcm,
    /// AES-128-CBC
    Aes128Cbc,
    /// AES-256-CBC
    Aes256Cbc,
    /// RSA-2048
    Rsa2048,
    /// RSA-3072
    Rsa3072,
    /// RSA-4096
    Rsa4096,
    /// ECDSA with P-256
    EcdsaP256,
    /// ECDSA with P-384
    EcdsaP384,
    /// Ed25519 (`EdDSA`)
    Ed25519,
    /// HMAC-SHA256
    HmacSha256,
    /// HMAC-SHA384
    HmacSha384,
    /// HMAC-SHA512
    HmacSha512,
    /// HKDF-SHA256
    HkdfSha256,
    /// PBKDF2-HMAC-SHA256
    Pbkdf2HmacSha256,
    /// ChaCha20-Poly1305
    ChaCha20Poly1305,
}

impl CryptographicAlgorithm {
    /// Get the recommended key size for this algorithm
    pub const fn recommended_key_size(&self) -> Option<u32> {
        match self {
            Self::Aes128Gcm | Self::Aes128Cbc => Some(128),
            Self::Aes256Gcm | Self::Aes256Cbc => Some(256),
            Self::Rsa2048 => Some(2048),
            Self::Rsa3072 => Some(3072),
            Self::Rsa4096 => Some(4096),
            Self::EcdsaP256 => Some(256),
            Self::EcdsaP384 => Some(384),
            Self::Ed25519 => Some(256),
            Self::HmacSha256 | Self::HkdfSha256 | Self::Pbkdf2HmacSha256 => Some(256),
            Self::HmacSha384 => Some(384),
            Self::HmacSha512 => Some(512),
            Self::ChaCha20Poly1305 => Some(256),
        }
    }

    /// Check if this algorithm supports authenticated encryption
    pub const fn is_authenticated_encryption(&self) -> bool {
        matches!(
            self,
            Self::Aes128Gcm | Self::Aes256Gcm | Self::ChaCha20Poly1305
        )
    }

    /// Check if this algorithm is for signing/verification
    pub const fn is_signature_algorithm(&self) -> bool {
        matches!(
            self,
            Self::EcdsaP256
                | Self::EcdsaP384
                | Self::Ed25519
                | Self::Rsa2048
                | Self::Rsa3072
                | Self::Rsa4096
        )
    }
}

/// Performance characteristics for an algorithm
#[derive(Debug, Clone)]
pub struct PerformanceCharacteristics {
    /// Operations per second
    pub ops_per_second: f64,
    /// Average latency in microseconds
    pub avg_latency_us: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
}

impl Default for PerformanceCharacteristics {
    fn default() -> Self {
        Self {
            ops_per_second: 1000.0,
            avg_latency_us: 100.0,
            memory_usage_bytes: 1024,
            cpu_usage_percent: 5.0,
        }
    }
}

/// Algorithm capability information
#[derive(Debug, Clone)]
pub struct AlgorithmCapability {
    /// Algorithm type
    pub algorithm: CryptographicAlgorithm,
    /// Whether the algorithm is supported
    pub supported: bool,
    /// Whether hardware acceleration is available
    pub hardware_accelerated: bool,
    /// Performance characteristics
    pub performance: PerformanceCharacteristics,
}

/// Canonical cryptographic algorithm enumeration
///
/// Unified algorithm definitions for all HSM operations across `BearDog`.
/// This enum consolidates algorithm specifications from multiple sources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Algorithm {
    /// AES-256 in GCM mode (symmetric encryption)
    Aes256Gcm,
    /// ChaCha20-Poly1305 authenticated encryption
    ChaCha20Poly1305,

    /// Elliptic Curve Cryptography with P-256 curve
    EccP256,
    /// Elliptic Curve Cryptography with P-384 curve
    EccP384,

    /// ECDSA with P-256 curve
    EcdsaP256,
    /// ECDSA with P-384 curve
    EcdsaP384,
    /// ECDSA with SHA-256 hash
    EcdsaSha256,

    /// RSA with SHA-256 hash
    RsaSha256,
    /// RSA-PSS with 2048-bit key
    RsaPss2048,
    /// RSA-PSS with 3072-bit key
    RsaPss3072,
    /// RSA-PSS with 4096-bit key
    RsaPss4096,

    /// HKDF with SHA-256 for key derivation
    HkdfSha256,

    /// Ed25519 digital signatures
    Ed25519,
    /// X25519 key exchange
    X25519,
}

impl Algorithm {
    /// Returns the security strength in bits
    pub const fn security_bits(&self) -> usize {
        match self {
            Self::Aes256Gcm | Self::EccP256 | Self::EcdsaP256 | Self::Ed25519 | Self::X25519 => 256,
            Self::EccP384 | Self::EcdsaP384 => 384,
            Self::RsaPss2048 | Self::RsaSha256 => 112, // Effective security
            Self::RsaPss3072 => 128,
            Self::RsaPss4096 => 152,
            Self::ChaCha20Poly1305 => 256,
            Self::HkdfSha256 | Self::EcdsaSha256 => 256,
        }
    }

    /// Returns whether this is a signature algorithm
    pub const fn is_signature_algorithm(&self) -> bool {
        matches!(
            self,
            Self::EcdsaP256
                | Self::EcdsaP384
                | Self::EcdsaSha256
                | Self::RsaSha256
                | Self::RsaPss2048
                | Self::RsaPss3072
                | Self::RsaPss4096
                | Self::Ed25519
        )
    }

    /// Returns whether this is an encryption algorithm
    pub const fn is_encryption_algorithm(&self) -> bool {
        matches!(self, Self::Aes256Gcm | Self::ChaCha20Poly1305)
    }
}

impl From<Algorithm> for super::key::KeyType {
    fn from(algo: Algorithm) -> Self {
        match algo {
            Algorithm::EcdsaP256 | Algorithm::EccP256 | Algorithm::EcdsaSha256 => {
                Self::EllipticCurve
            }
            Algorithm::EcdsaP384 | Algorithm::EccP384 => Self::EllipticCurve,
            Algorithm::RsaPss2048
            | Algorithm::RsaPss3072
            | Algorithm::RsaPss4096
            | Algorithm::RsaSha256 => Self::Rsa,
            Algorithm::Aes256Gcm => Self::Aes,
            Algorithm::ChaCha20Poly1305 => Self::ChaCha20,
            Algorithm::Ed25519 => Self::Ed25519,
            Algorithm::X25519 => Self::X25519,
            Algorithm::HkdfSha256 => Self::Generic, // KDF doesn't map directly to key type
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_parameters_default() {
        let params = AlgorithmParameters::default();
        assert_eq!(params.algorithm, CryptographicAlgorithm::Aes256Gcm);
        assert_eq!(params.key_size, Some(256));
    }

    #[test]
    fn test_recommended_key_sizes() {
        assert_eq!(
            CryptographicAlgorithm::Aes128Gcm.recommended_key_size(),
            Some(128)
        );
        assert_eq!(
            CryptographicAlgorithm::Aes256Gcm.recommended_key_size(),
            Some(256)
        );
        assert_eq!(
            CryptographicAlgorithm::Rsa2048.recommended_key_size(),
            Some(2048)
        );
        assert_eq!(
            CryptographicAlgorithm::Ed25519.recommended_key_size(),
            Some(256)
        );
    }

    #[test]
    fn test_authenticated_encryption() {
        assert!(CryptographicAlgorithm::Aes256Gcm.is_authenticated_encryption());
        assert!(CryptographicAlgorithm::ChaCha20Poly1305.is_authenticated_encryption());
        assert!(!CryptographicAlgorithm::Aes256Cbc.is_authenticated_encryption());
        assert!(!CryptographicAlgorithm::Ed25519.is_authenticated_encryption());
    }

    #[test]
    fn test_signature_algorithms() {
        assert!(CryptographicAlgorithm::Ed25519.is_signature_algorithm());
        assert!(CryptographicAlgorithm::EcdsaP256.is_signature_algorithm());
        assert!(CryptographicAlgorithm::Rsa2048.is_signature_algorithm());
        assert!(!CryptographicAlgorithm::Aes256Gcm.is_signature_algorithm());
        assert!(!CryptographicAlgorithm::HmacSha256.is_signature_algorithm());
    }

    #[test]
    fn test_performance_characteristics_default() {
        let perf = PerformanceCharacteristics::default();
        assert_eq!(perf.ops_per_second, 1000.0);
        assert_eq!(perf.avg_latency_us, 100.0);
        assert_eq!(perf.memory_usage_bytes, 1024);
        assert_eq!(perf.cpu_usage_percent, 5.0);
    }

    #[test]
    fn test_hash_algorithm_serialization() {
        let alg = HashAlgorithm::Sha256;
        let serialized = serde_json::to_string(&alg).unwrap();
        let deserialized: HashAlgorithm = serde_json::from_str(&serialized).unwrap();
        assert_eq!(alg, deserialized);
    }

    #[test]
    fn test_aes_mode_variants() {
        let modes = [
            AesMode::Ecb,
            AesMode::Cbc,
            AesMode::Ctr,
            AesMode::Gcm,
            AesMode::Cfb,
            AesMode::Ofb,
        ];
        assert_eq!(modes.len(), 6);
    }

    #[test]
    fn test_elliptic_curve_types() {
        let curves = [
            EllipticCurveType::P256,
            EllipticCurveType::P384,
            EllipticCurveType::P521,
            EllipticCurveType::Secp256k1,
            EllipticCurveType::Curve25519,
            EllipticCurveType::Curve448,
        ];
        assert_eq!(curves.len(), 6);
    }
}
