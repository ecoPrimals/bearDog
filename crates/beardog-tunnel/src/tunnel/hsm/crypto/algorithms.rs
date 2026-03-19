// SPDX-License-Identifier: AGPL-3.0-only

//! Algorithm Abstractions
//!
//! Vendor-agnostic algorithm definitions that can be implemented by any crypto library.

use serde::{Deserialize, Serialize};

/// Universal cryptographic algorithm type
///
/// Represents all supported cryptographic algorithms in a unified enum.
/// Each variant wraps a specific algorithm category.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CryptoAlgorithm {
    /// Symmetric encryption algorithm (AES, ChaCha20, etc.)
    Symmetric(SymmetricAlgorithm),
    /// Asymmetric encryption algorithm (RSA-OAEP, ECIES, etc.)
    Asymmetric(AsymmetricAlgorithm),
    /// Digital signature algorithm (Ed25519, ECDSA, RSA-PSS, etc.)
    Signature(SignatureAlgorithm),
    /// Cryptographic hash algorithm (SHA-2, SHA-3, BLAKE, etc.)
    Hash(HashAlgorithm),
    /// Key derivation function (HKDF, PBKDF2, Argon2, etc.)
    Kdf(KdfAlgorithm),
}

impl CryptoAlgorithm {
    /// Extracts the symmetric algorithm if this is a `Symmetric` variant
    ///
    /// # Errors
    /// Returns an error if this is not a symmetric algorithm
    pub fn as_symmetric(&self) -> Result<SymmetricAlgorithm, String> {
        match self {
            CryptoAlgorithm::Symmetric(alg) => Ok(alg.clone()),
            _ => Err(format!("Expected symmetric algorithm, got: {self:?}")),
        }
    }

    /// Extracts the signature algorithm if this is a `Signature` variant
    ///
    /// # Errors
    /// Returns an error if this is not a signature algorithm
    pub fn as_signature(&self) -> Result<SignatureAlgorithm, String> {
        match self {
            CryptoAlgorithm::Signature(alg) => Ok(alg.clone()),
            _ => Err(format!("Expected signature algorithm, got: {self:?}")),
        }
    }
}

impl std::fmt::Display for CryptoAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoAlgorithm::Symmetric(alg) => write!(f, "{alg}"),
            CryptoAlgorithm::Asymmetric(alg) => write!(f, "{alg}"),
            CryptoAlgorithm::Signature(alg) => write!(f, "{alg}"),
            CryptoAlgorithm::Hash(alg) => write!(f, "{alg}"),
            CryptoAlgorithm::Kdf(alg) => write!(f, "{alg}"),
        }
    }
}

/// Symmetric encryption algorithms
///
/// Supports both authenticated (AEAD) and non-authenticated ciphers.
/// Prefer authenticated modes (GCM, Poly1305) for new implementations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SymmetricAlgorithm {
    /// AES with configurable mode and key size
    Aes {
        /// AES block cipher mode (GCM, CTR, CBC, CFB)
        mode: AesMode,
        /// Key size in bits (128, 192, or 256)
        key_size: u32,
    },
    /// ChaCha20-Poly1305 AEAD cipher (256-bit key)
    ChaCha20Poly1305,
    /// ChaCha20 stream cipher (non-authenticated)
    ChaCha20 {
        /// Key size in bits (typically 256)
        key_size: u32,
    },
    /// AES-256-GCM authenticated encryption
    Aes256Gcm,
    /// AES-128-GCM authenticated encryption
    Aes128Gcm,
    /// Custom symmetric algorithm for extensibility
    Custom {
        /// Algorithm name identifier
        name: String,
        /// Algorithm specification details
        spec: AlgorithmSpec,
    },
}

impl std::fmt::Display for SymmetricAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Aes { mode, key_size } => write!(f, "AES-{key_size}-{mode}"),
            Self::ChaCha20Poly1305 => write!(f, "ChaCha20-Poly1305"),
            Self::ChaCha20 { key_size } => write!(f, "ChaCha20-{key_size}"),
            Self::Aes256Gcm => write!(f, "AES-256-GCM"),
            Self::Aes128Gcm => write!(f, "AES-128-GCM"),
            Self::Custom { name, .. } => write!(f, "Custom({name})"),
        }
    }
}

/// AES block cipher modes of operation
///
/// GCM is recommended for most use cases as it provides both
/// confidentiality and integrity (authenticated encryption).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AesMode {
    /// Galois/Counter Mode - authenticated encryption (RECOMMENDED)
    Gcm,
    /// Counter Mode - stream cipher, requires separate MAC
    Ctr,
    /// Cipher Block Chaining - requires padding, use with HMAC
    Cbc,
    /// Cipher Feedback Mode - stream-like, requires separate MAC
    Cfb,
}

impl std::fmt::Display for AesMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gcm => write!(f, "GCM"),
            Self::Ctr => write!(f, "CTR"),
            Self::Cbc => write!(f, "CBC"),
            Self::Cfb => write!(f, "CFB"),
        }
    }
}

/// Asymmetric encryption algorithms
///
/// Used for key encapsulation and small data encryption.
/// RSA-OAEP or ECIES are recommended for new implementations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AsymmetricAlgorithm {
    /// RSA-OAEP (Optimal Asymmetric Encryption Padding)
    RsaOaep {
        /// RSA key size in bits (2048, 3072, or 4096)
        key_size: u32,
        /// Hash algorithm for OAEP padding
        hash: HashAlgorithm,
    },
    /// RSA PKCS#1 v1.5 padding (legacy, not recommended)
    RsaPkcs1v15 {
        /// RSA key size in bits
        key_size: u32,
    },
    /// Elliptic Curve Integrated Encryption Scheme on P-256
    EciesP256,
    /// Elliptic Curve Integrated Encryption Scheme on P-384
    EciesP384,
    /// Custom asymmetric algorithm for extensibility
    Custom {
        /// Algorithm name identifier
        name: String,
        /// Algorithm specification details
        spec: AlgorithmSpec,
    },
}

impl std::fmt::Display for AsymmetricAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RsaOaep { key_size, hash } => write!(f, "RSA-OAEP-{key_size}-{hash}"),
            Self::RsaPkcs1v15 { key_size } => write!(f, "RSA-PKCS1v15-{key_size}"),
            Self::EciesP256 => write!(f, "ECIES-P256"),
            Self::EciesP384 => write!(f, "ECIES-P384"),
            Self::Custom { name, .. } => write!(f, "Custom({name})"),
        }
    }
}

/// Digital signature algorithms
///
/// Ed25519 is recommended for most use cases due to its security,
/// performance, and resistance to implementation errors.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SignatureAlgorithm {
    /// Ed25519 Edwards-curve signature (RECOMMENDED)
    Ed25519,
    /// ECDSA on NIST P-256 curve
    EcdsaP256 {
        /// Hash algorithm for message digest
        hash: HashAlgorithm,
    },
    /// ECDSA on NIST P-384 curve
    EcdsaP384 {
        /// Hash algorithm for message digest
        hash: HashAlgorithm,
    },
    /// RSA-PSS (Probabilistic Signature Scheme)
    RsaPss {
        /// RSA key size in bits (2048, 3072, or 4096)
        key_size: u32,
        /// Hash algorithm for message digest
        hash: HashAlgorithm,
    },
    /// RSA PKCS#1 v1.5 signature (legacy, not recommended)
    RsaPkcs1v15 {
        /// RSA key size in bits
        key_size: u32,
        /// Hash algorithm for message digest
        hash: HashAlgorithm,
    },
    /// Custom signature algorithm for extensibility
    Custom {
        /// Algorithm name identifier
        name: String,
        /// Algorithm specification details
        spec: AlgorithmSpec,
    },
}

impl std::fmt::Display for SignatureAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ed25519 => write!(f, "Ed25519"),
            Self::EcdsaP256 { hash } => write!(f, "ECDSA-P256-{hash}"),
            Self::EcdsaP384 { hash } => write!(f, "ECDSA-P384-{hash}"),
            Self::RsaPss { key_size, hash } => write!(f, "RSA-PSS-{key_size}-{hash}"),
            Self::RsaPkcs1v15 { key_size, hash } => write!(f, "RSA-PKCS1v15-{key_size}-{hash}"),
            Self::Custom { name, .. } => write!(f, "Custom({name})"),
        }
    }
}

/// Cryptographic hash algorithms
///
/// SHA-256 and BLAKE3 are recommended for most use cases.
/// BLAKE3 is faster and provides 256-bit security.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HashAlgorithm {
    /// SHA-256 (256-bit output)
    Sha256,
    /// SHA-384 (384-bit output)
    Sha384,
    /// SHA-512 (512-bit output)
    Sha512,
    /// SHA3-256 (Keccak-based, 256-bit output)
    Sha3_256,
    /// SHA3-384 (Keccak-based, 384-bit output)
    Sha3_384,
    /// SHA3-512 (Keccak-based, 512-bit output)
    Sha3_512,
    /// BLAKE2b (variable output up to 512 bits)
    Blake2b {
        /// Output size in bytes (1-64)
        output_size: usize,
    },
    /// BLAKE2s (variable output up to 256 bits)
    Blake2s {
        /// Output size in bytes (1-32)
        output_size: usize,
    },
    /// BLAKE3 (256-bit output, fastest modern hash)
    Blake3,
    /// Custom hash algorithm for extensibility
    Custom {
        /// Algorithm name identifier
        name: String,
        /// Algorithm specification details
        spec: AlgorithmSpec,
    },
}

impl std::fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sha256 => write!(f, "SHA-256"),
            Self::Sha384 => write!(f, "SHA-384"),
            Self::Sha512 => write!(f, "SHA-512"),
            Self::Sha3_256 => write!(f, "SHA3-256"),
            Self::Sha3_384 => write!(f, "SHA3-384"),
            Self::Sha3_512 => write!(f, "SHA3-512"),
            Self::Blake2b { output_size } => write!(f, "BLAKE2b-{output_size}"),
            Self::Blake2s { output_size } => write!(f, "BLAKE2s-{output_size}"),
            Self::Blake3 => write!(f, "BLAKE3"),
            Self::Custom { name, .. } => write!(f, "Custom({name})"),
        }
    }
}

/// Key derivation function algorithms
///
/// Use HKDF for key derivation from high-entropy input.
/// Use Argon2id for password-based key derivation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KdfAlgorithm {
    /// HKDF with SHA-256 (for high-entropy input)
    HkdfSha256,
    /// HKDF with SHA-384 (for high-entropy input)
    HkdfSha384,
    /// HKDF with SHA-512 (for high-entropy input)
    HkdfSha512,
    /// PBKDF2 (password-based, legacy)
    Pbkdf2 {
        /// Hash algorithm for HMAC
        hash: HashAlgorithm,
        /// Number of iterations (minimum 100,000 recommended)
        iterations: u32,
    },
    /// scrypt (memory-hard password KDF)
    Scrypt {
        /// CPU/memory cost parameter (power of 2)
        n: u64,
        /// Block size parameter
        r: u32,
        /// Parallelization parameter
        p: u32,
    },
    /// Argon2 (RECOMMENDED for passwords)
    Argon2 {
        /// Argon2 variant (d, i, or id)
        variant: Argon2Variant,
    },
    /// Custom KDF for extensibility
    Custom {
        /// Algorithm name identifier
        name: String,
        /// Algorithm specification details
        spec: AlgorithmSpec,
    },
}

impl std::fmt::Display for KdfAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HkdfSha256 => write!(f, "HKDF-SHA256"),
            Self::HkdfSha384 => write!(f, "HKDF-SHA384"),
            Self::HkdfSha512 => write!(f, "HKDF-SHA512"),
            Self::Pbkdf2 { hash, iterations } => write!(f, "PBKDF2-{hash}-{iterations}"),
            Self::Scrypt { n, r, p } => write!(f, "Scrypt-{n}-{r}-{p}"),
            Self::Argon2 { variant } => write!(f, "Argon2-{variant:?}"),
            Self::Custom { name, .. } => write!(f, "Custom({name})"),
        }
    }
}

/// Argon2 algorithm variants
///
/// Argon2id is recommended for most applications as it provides
/// both side-channel resistance (from Argon2i) and GPU resistance (from Argon2d).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Argon2Variant {
    /// Data-dependent (vulnerable to side-channels, GPU-resistant)
    Argon2d,
    /// Data-independent (side-channel resistant, less GPU-resistant)
    Argon2i,
    /// Hybrid (RECOMMENDED) - combines both approaches
    Argon2id,
}

/// Custom algorithm specification
///
/// Allows defining custom or vendor-specific algorithms while maintaining
/// compatibility with the algorithm abstraction layer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AlgorithmSpec {
    /// Algorithm name identifier
    pub name: String,
    /// Algorithm category (encryption, signature, hash, etc.)
    pub category: AlgorithmCategory,
    /// Supported key sizes in bits
    pub key_sizes: Vec<u32>,
    /// Supported block sizes in bytes
    pub block_sizes: Vec<u32>,
    /// Additional algorithm parameters (key-value pairs)
    pub parameters: Vec<(String, String)>,
}

/// Algorithm categories
///
/// Used to classify custom algorithms for proper routing and validation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlgorithmCategory {
    /// Symmetric (secret-key) encryption algorithms
    SymmetricEncryption,
    /// Asymmetric (public-key) encryption algorithms
    AsymmetricEncryption,
    /// Digital signature algorithms
    DigitalSignature,
    /// Cryptographic hash functions
    Hash,
    /// Key derivation functions
    KeyDerivation,
}

/// Cryptographic operation types
///
/// Used for operation routing, logging, and access control.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CryptoOperation {
    /// Encrypt data with symmetric key
    SymmetricEncryption,
    /// Decrypt data with symmetric key
    SymmetricDecryption,
    /// Encrypt data with public key
    AsymmetricEncryption,
    /// Decrypt data with private key
    AsymmetricDecryption,
    /// Create digital signature with private key
    Signing,
    /// Verify digital signature with public key
    Verification,
    /// Compute cryptographic hash
    Hashing,
    /// Derive key material
    KeyDerivation,
}

/// Encrypted data container
///
/// Holds ciphertext along with algorithm metadata for decryption.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    /// Algorithm used for encryption (for decryption routing)
    pub algorithm: String,
    /// The encrypted data
    pub ciphertext: Vec<u8>,
    /// Initialization vector or nonce (for modes that require it)
    pub nonce: Option<Vec<u8>>,
    /// Authentication tag (for AEAD modes)
    pub tag: Option<Vec<u8>>,
}

/// Digital signature container
///
/// Holds signature bytes along with algorithm metadata for verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    /// Algorithm used for signing (for verification routing)
    pub algorithm: String,
    /// The signature bytes
    pub signature: Vec<u8>,
}

/// Encryption operation options
///
/// Optional parameters for encryption operations.
#[derive(Debug, Clone, Default)]
pub struct EncryptionOptions {
    /// Optional nonce/IV (if not provided, a random one is generated)
    pub nonce: Option<Vec<u8>>,
    /// Associated authenticated data (for AEAD modes)
    pub associated_data: Option<Vec<u8>>,
}

/// Decryption operation options
///
/// Optional parameters for decryption operations.
#[derive(Debug, Clone, Default)]
pub struct DecryptionOptions {
    /// Associated authenticated data (must match encryption AAD)
    pub associated_data: Option<Vec<u8>>,
}

/// Signing operation options
///
/// Optional parameters for signature generation.
#[derive(Debug, Clone, Default)]
pub struct SigningOptions {
    /// Use deterministic signing (RFC 6979) if supported
    pub deterministic: bool,
}

/// Verification operation options
///
/// Optional parameters for signature verification.
/// Reserved for future extension.
#[derive(Debug, Clone, Default)]
pub struct VerificationOptions {
    // Future extension point
}
