//! Cryptographic Operations Configuration for SIMD
//!
//! This module handles configuration for various cryptographic operations using SIMD.

use serde::{Deserialize, Serialize};

/// Cryptographic operations configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoOperationsConfig {
    /// Hash operations configuration
    pub hash: HashOperationsConfig,
    /// Encryption operations configuration
    pub encryption: EncryptionOperationsConfig,
    /// Key derivation configuration
    pub key_derivation: KeyDerivationConfig,
    /// Digital signature configuration
    pub signature: DigitalSignatureConfig,
    /// Random number generation configuration
    pub random_generation: RandomGenerationConfig,
}

/// Hash operations configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashOperationsConfig {
    /// Enable hash operations
    pub enabled: bool,
    /// Supported hash algorithms
    pub algorithms: Vec<HashAlgorithm>,
    /// Optimization strategy
    pub optimization_strategy: HashOptimizationStrategy,
}

/// Hash algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HashAlgorithm {
    /// SHA-256 hash algorithm
    SHA256,
    /// SHA-384 hash algorithm
    SHA384,
    /// SHA-512 hash algorithm
    SHA512,
    /// BLAKE2b hash algorithm
    BLAKE2b,
    /// BLAKE2s hash algorithm
    BLAKE2s,
    /// SHA-3 hash algorithm
    SHA3,
    /// Keccak hash algorithm
    Keccak,
}

/// Hash optimization strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HashOptimizationStrategy {
    /// Speed-optimized hashing
    Speed,
    /// Memory-optimized hashing
    Memory,
    /// Balanced optimization
    Balanced,
}

/// Encryption operations configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionOperationsConfig {
    /// Enable encryption operations
    pub enabled: bool,
    /// Supported encryption algorithms
    pub algorithms: Vec<EncryptionAlgorithm>,
    /// Block cipher modes
    pub cipher_modes: Vec<BlockCipherMode>,
    /// Optimization strategy
    pub optimization_strategy: EncryptionOptimizationStrategy,
}

/// Encryption algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    /// AES-128 encryption
    AES128,
    /// AES-256 encryption
    AES256,
    /// ChaCha20 encryption
    ChaCha20,
    /// XChaCha20 encryption
    XChaCha20,
}

/// Block cipher mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockCipherMode {
    /// Galois/Counter Mode
    GCM,
    /// Counter mode
    CTR,
    /// Cipher Block Chaining
    CBC,
    /// Electronic Codebook
    ECB,
    /// XEX-based tweaked-codebook mode
    XTS,
}

/// Encryption optimization strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionOptimizationStrategy {
    /// Throughput optimization
    Throughput,
    /// Latency optimization
    Latency,
    /// Power efficiency optimization
    PowerEfficiency,
}

/// Key derivation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyDerivationConfig {
    /// Enable key derivation
    pub enabled: bool,
    /// Supported key derivation functions
    pub functions: Vec<KeyDerivationFunction>,
    /// Optimization strategy
    pub optimization_strategy: KeyDerivationOptimizationStrategy,
}

/// Key derivation function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyDerivationFunction {
    /// PBKDF2 key derivation
    PBKDF2,
    /// Scrypt key derivation
    Scrypt,
    /// Argon2 key derivation
    Argon2,
    /// HKDF key derivation
    HKDF,
}

/// Key derivation optimization strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyDerivationOptimizationStrategy {
    /// Security-first optimization
    Security,
    /// Speed optimization
    Speed,
    /// Memory optimization
    Memory,
}

/// Digital signature configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalSignatureConfig {
    /// Enable digital signatures
    pub enabled: bool,
    /// Supported signature algorithms
    pub algorithms: Vec<SignatureAlgorithm>,
    /// Optimization strategy
    pub optimization_strategy: SignatureOptimizationStrategy,
}

/// Signature algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignatureAlgorithm {
    /// Ed25519 signature algorithm
    Ed25519,
    /// ECDSA P-256 signature algorithm
    ECDSAP256,
    /// ECDSA P-384 signature algorithm
    ECDSAP384,
    /// RSA PSS signature algorithm
    RSAPSS,
}

/// Signature optimization strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignatureOptimizationStrategy {
    /// Verification speed optimization
    VerificationSpeed,
    /// Signing speed optimization
    SigningSpeed,
    /// Balanced optimization
    Balanced,
}

/// Random number generation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomGenerationConfig {
    /// Enable random generation
    pub enabled: bool,
    /// Random number generators
    pub generators: Vec<RandomGenerator>,
    /// Entropy sources
    pub entropy_sources: Vec<EntropySource>,
    /// Optimization strategy
    pub optimization_strategy: RandomOptimizationStrategy,
}

/// Random number generator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RandomGenerator {
    /// ChaCha20 RNG
    ChaCha20,
    /// AES-CTR RNG
    AESCTR,
    /// Hardware RNG
    Hardware,
}

/// Entropy source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntropySource {
    /// Hardware entropy
    Hardware,
    /// Operating system entropy
    OS,
    /// RDRAND instruction
    RDRAND,
    /// RDSEED instruction
    RDSEED,
}

/// Random optimization strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RandomOptimizationStrategy {
    /// Quality optimization
    Quality,
    /// Speed optimization
    Speed,
    /// Balanced optimization
    Balanced,
}

impl Default for CryptoOperationsConfig {
    fn default() -> Self {
        Self {
            hash: HashOperationsConfig::default(),
            encryption: EncryptionOperationsConfig::default(),
            key_derivation: KeyDerivationConfig::default(),
            signature: DigitalSignatureConfig::default(),
            random_generation: RandomGenerationConfig::default(),
        }
    }
}

impl Default for HashOperationsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            algorithms: vec![HashAlgorithm::SHA256, HashAlgorithm::BLAKE2b],
            optimization_strategy: HashOptimizationStrategy::Balanced,
        }
    }
}

impl Default for EncryptionOperationsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            algorithms: vec![EncryptionAlgorithm::AES256, EncryptionAlgorithm::ChaCha20],
            cipher_modes: vec![BlockCipherMode::GCM, BlockCipherMode::CTR],
            optimization_strategy: EncryptionOptimizationStrategy::Throughput,
        }
    }
}

impl Default for KeyDerivationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            functions: vec![KeyDerivationFunction::Argon2, KeyDerivationFunction::HKDF],
            optimization_strategy: KeyDerivationOptimizationStrategy::Security,
        }
    }
}

impl Default for DigitalSignatureConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            algorithms: vec![SignatureAlgorithm::Ed25519, SignatureAlgorithm::ECDSAP256],
            optimization_strategy: SignatureOptimizationStrategy::Balanced,
        }
    }
}

impl Default for RandomGenerationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            generators: vec![RandomGenerator::ChaCha20, RandomGenerator::Hardware],
            entropy_sources: vec![EntropySource::Hardware, EntropySource::OS],
            optimization_strategy: RandomOptimizationStrategy::Quality,
        }
    }
}

impl CryptoOperationsConfig {
    /// Create production crypto operations configuration
    pub fn production() -> Self {
        Self {
            hash: HashOperationsConfig::production(),
            encryption: EncryptionOperationsConfig::production(),
            key_derivation: KeyDerivationConfig::production(),
            signature: DigitalSignatureConfig::production(),
            random_generation: RandomGenerationConfig::production(),
        }
    }

    /// Create development crypto operations configuration
    pub fn development() -> Self {
        Self {
            hash: HashOperationsConfig::development(),
            encryption: EncryptionOperationsConfig::development(),
            key_derivation: KeyDerivationConfig::development(),
            signature: DigitalSignatureConfig::development(),
            random_generation: RandomGenerationConfig::development(),
        }
    }
}

impl HashOperationsConfig {
    /// Create production hash operations configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            algorithms: vec![
                HashAlgorithm::SHA256,
                HashAlgorithm::SHA384,
                HashAlgorithm::SHA512,
                HashAlgorithm::BLAKE2b,
                HashAlgorithm::BLAKE2s,
                HashAlgorithm::SHA3,
            ],
            optimization_strategy: HashOptimizationStrategy::Speed,
        }
    }

    /// Create development hash operations configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            algorithms: vec![HashAlgorithm::SHA256],
            optimization_strategy: HashOptimizationStrategy::Memory,
        }
    }
}

impl EncryptionOperationsConfig {
    /// Create production encryption operations configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            algorithms: vec![
                EncryptionAlgorithm::AES128,
                EncryptionAlgorithm::AES256,
                EncryptionAlgorithm::ChaCha20,
                EncryptionAlgorithm::XChaCha20,
            ],
            cipher_modes: vec![
                BlockCipherMode::GCM,
                BlockCipherMode::CTR,
                BlockCipherMode::XTS,
            ],
            optimization_strategy: EncryptionOptimizationStrategy::Throughput,
        }
    }

    /// Create development encryption operations configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            algorithms: vec![EncryptionAlgorithm::AES256],
            cipher_modes: vec![BlockCipherMode::GCM],
            optimization_strategy: EncryptionOptimizationStrategy::PowerEfficiency,
        }
    }
}

impl KeyDerivationConfig {
    /// Create production key derivation configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            functions: vec![
                KeyDerivationFunction::PBKDF2,
                KeyDerivationFunction::Scrypt,
                KeyDerivationFunction::Argon2,
                KeyDerivationFunction::HKDF,
            ],
            optimization_strategy: KeyDerivationOptimizationStrategy::Security,
        }
    }

    /// Create development key derivation configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            functions: vec![KeyDerivationFunction::PBKDF2],
            optimization_strategy: KeyDerivationOptimizationStrategy::Speed,
        }
    }
}

impl DigitalSignatureConfig {
    /// Create production digital signature configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            algorithms: vec![
                SignatureAlgorithm::Ed25519,
                SignatureAlgorithm::ECDSAP256,
                SignatureAlgorithm::ECDSAP384,
                SignatureAlgorithm::RSAPSS,
            ],
            optimization_strategy: SignatureOptimizationStrategy::VerificationSpeed,
        }
    }

    /// Create development digital signature configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            algorithms: vec![SignatureAlgorithm::Ed25519],
            optimization_strategy: SignatureOptimizationStrategy::Balanced,
        }
    }
}

impl RandomGenerationConfig {
    /// Create production random generation configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            generators: vec![
                RandomGenerator::ChaCha20,
                RandomGenerator::AESCTR,
                RandomGenerator::Hardware,
            ],
            entropy_sources: vec![
                EntropySource::Hardware,
                EntropySource::OS,
                EntropySource::RDRAND,
                EntropySource::RDSEED,
            ],
            optimization_strategy: RandomOptimizationStrategy::Quality,
        }
    }

    /// Create development random generation configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            generators: vec![RandomGenerator::ChaCha20],
            entropy_sources: vec![EntropySource::OS],
            optimization_strategy: RandomOptimizationStrategy::Speed,
        }
    }
}
