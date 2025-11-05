//! Algorithm Abstractions
//!
//! Vendor-agnostic algorithm definitions that can be implemented by any crypto library.

use serde::{Deserialize, Serialize};

/// Universal algorithm type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CryptoAlgorithm {
    Symmetric(SymmetricAlgorithm),
    Asymmetric(AsymmetricAlgorithm),
    Signature(SignatureAlgorithm),
    Hash(HashAlgorithm),
    Kdf(KdfAlgorithm),
}

impl CryptoAlgorithm {
    pub fn as_symmetric(&self) -> Result<SymmetricAlgorithm, String> {
        match self {
            CryptoAlgorithm::Symmetric(alg) => Ok(alg.clone()),
            _ => Err(format!("Expected symmetric algorithm, got: {:?}", self)),
        }
    }

    pub fn as_signature(&self) -> Result<SignatureAlgorithm, String> {
        match self {
            CryptoAlgorithm::Signature(alg) => Ok(alg.clone()),
            _ => Err(format!("Expected signature algorithm, got: {:?}", self)),
        }
    }
}

impl std::fmt::Display for CryptoAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoAlgorithm::Symmetric(alg) => write!(f, "{}", alg),
            CryptoAlgorithm::Asymmetric(alg) => write!(f, "{}", alg),
            CryptoAlgorithm::Signature(alg) => write!(f, "{}", alg),
            CryptoAlgorithm::Hash(alg) => write!(f, "{}", alg),
            CryptoAlgorithm::Kdf(alg) => write!(f, "{}", alg),
        }
    }
}

/// Symmetric encryption algorithms
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SymmetricAlgorithm {
    Aes { mode: AesMode, key_size: u32 },
    ChaCha20Poly1305,
    ChaCha20 { key_size: u32 },
    Aes256Gcm,
    Aes128Gcm,
    Custom { name: String, spec: AlgorithmSpec },
}

impl std::fmt::Display for SymmetricAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Aes { mode, key_size } => write!(f, "AES-{}-{}", key_size, mode),
            Self::ChaCha20Poly1305 => write!(f, "ChaCha20-Poly1305"),
            Self::ChaCha20 { key_size } => write!(f, "ChaCha20-{}", key_size),
            Self::Aes256Gcm => write!(f, "AES-256-GCM"),
            Self::Aes128Gcm => write!(f, "AES-128-GCM"),
            Self::Custom { name, .. } => write!(f, "Custom({})", name),
        }
    }
}

/// AES modes
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AesMode {
    Gcm,
    Ctr,
    Cbc,
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AsymmetricAlgorithm {
    RsaOaep { key_size: u32, hash: HashAlgorithm },
    RsaPkcs1v15 { key_size: u32 },
    EciesP256,
    EciesP384,
    Custom { name: String, spec: AlgorithmSpec },
}

impl std::fmt::Display for AsymmetricAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RsaOaep { key_size, hash } => write!(f, "RSA-OAEP-{}-{}", key_size, hash),
            Self::RsaPkcs1v15 { key_size } => write!(f, "RSA-PKCS1v15-{}", key_size),
            Self::EciesP256 => write!(f, "ECIES-P256"),
            Self::EciesP384 => write!(f, "ECIES-P384"),
            Self::Custom { name, .. } => write!(f, "Custom({})", name),
        }
    }
}

/// Signature algorithms
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SignatureAlgorithm {
    Ed25519,
    EcdsaP256 { hash: HashAlgorithm },
    EcdsaP384 { hash: HashAlgorithm },
    RsaPss { key_size: u32, hash: HashAlgorithm },
    RsaPkcs1v15 { key_size: u32, hash: HashAlgorithm },
    Custom { name: String, spec: AlgorithmSpec },
}

impl std::fmt::Display for SignatureAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ed25519 => write!(f, "Ed25519"),
            Self::EcdsaP256 { hash } => write!(f, "ECDSA-P256-{}", hash),
            Self::EcdsaP384 { hash } => write!(f, "ECDSA-P384-{}", hash),
            Self::RsaPss { key_size, hash } => write!(f, "RSA-PSS-{}-{}", key_size, hash),
            Self::RsaPkcs1v15 { key_size, hash } => write!(f, "RSA-PKCS1v15-{}-{}", key_size, hash),
            Self::Custom { name, .. } => write!(f, "Custom({})", name),
        }
    }
}

/// Hash algorithms
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HashAlgorithm {
    Sha256,
    Sha384,
    Sha512,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    Blake2b { output_size: usize },
    Blake2s { output_size: usize },
    Blake3,
    Custom { name: String, spec: AlgorithmSpec },
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
            Self::Blake2b { output_size } => write!(f, "BLAKE2b-{}", output_size),
            Self::Blake2s { output_size } => write!(f, "BLAKE2s-{}", output_size),
            Self::Blake3 => write!(f, "BLAKE3"),
            Self::Custom { name, .. } => write!(f, "Custom({})", name),
        }
    }
}

/// Key derivation algorithms
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KdfAlgorithm {
    HkdfSha256,
    HkdfSha384,
    HkdfSha512,
    Pbkdf2 {
        hash: HashAlgorithm,
        iterations: u32,
    },
    Scrypt {
        n: u64,
        r: u32,
        p: u32,
    },
    Argon2 {
        variant: Argon2Variant,
    },
    Custom {
        name: String,
        spec: AlgorithmSpec,
    },
}

impl std::fmt::Display for KdfAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HkdfSha256 => write!(f, "HKDF-SHA256"),
            Self::HkdfSha384 => write!(f, "HKDF-SHA384"),
            Self::HkdfSha512 => write!(f, "HKDF-SHA512"),
            Self::Pbkdf2 { hash, iterations } => write!(f, "PBKDF2-{}-{}", hash, iterations),
            Self::Scrypt { n, r, p } => write!(f, "Scrypt-{}-{}-{}", n, r, p),
            Self::Argon2 { variant } => write!(f, "Argon2-{:?}", variant),
            Self::Custom { name, .. } => write!(f, "Custom({})", name),
        }
    }
}

/// Argon2 variants
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Argon2Variant {
    Argon2d,
    Argon2i,
    Argon2id,
}

/// Custom algorithm specification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AlgorithmSpec {
    pub name: String,
    pub category: AlgorithmCategory,
    pub key_sizes: Vec<u32>,
    pub block_sizes: Vec<u32>,
    pub parameters: Vec<(String, String)>, // Changed from HashMap to Vec for Hash impl
}

/// Algorithm categories
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlgorithmCategory {
    SymmetricEncryption,
    AsymmetricEncryption,
    DigitalSignature,
    Hash,
    KeyDerivation,
}

/// Crypto operation types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CryptoOperation {
    SymmetricEncryption,
    SymmetricDecryption,
    AsymmetricEncryption,
    AsymmetricDecryption,
    Signing,
    Verification,
    Hashing,
    KeyDerivation,
}

/// Encrypted data container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    pub algorithm: String,
    pub ciphertext: Vec<u8>,
    pub nonce: Option<Vec<u8>>,
    pub tag: Option<Vec<u8>>,
}

/// Digital signature container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub algorithm: String,
    pub signature: Vec<u8>,
}

/// Encryption options
#[derive(Debug, Clone, Default)]
pub struct EncryptionOptions {
    pub nonce: Option<Vec<u8>>,
    pub associated_data: Option<Vec<u8>>,
}

/// Decryption options
#[derive(Debug, Clone, Default)]
pub struct DecryptionOptions {
    pub associated_data: Option<Vec<u8>>,
}

/// Signing options
#[derive(Debug, Clone, Default)]
pub struct SigningOptions {
    pub deterministic: bool,
}

/// Verification options
#[derive(Debug, Clone, Default)]
pub struct VerificationOptions {
    // Future extension point
}
