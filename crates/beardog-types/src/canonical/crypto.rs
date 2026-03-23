// SPDX-License-Identifier: AGPL-3.0-only

// Cryptographic types and configurations for BearDog
// Provides structured definitions for cryptographic algorithms, keys, and security settings

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Cryptographic algorithm enumeration
/// `CryptoAlgorithm`
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CryptoAlgorithm {
    /// AES encryption with specified key size
    Aes {
        /// AES key size in bits (128, 192, or 256)
        key_size: u32,
    },
    /// `ChaCha20` stream cipher
    /// `ChaCha20`
    ChaCha20,
    /// RSA asymmetric encryption
    Rsa {
        /// RSA key size in bits (2048, 3072, or 4096)
        key_size: u32,
    },
    /// Elliptic Curve Cryptography
    Ecc {
        /// The curve name (e.g., "secp256k1", "P-256")
        curve: String,
    },
    /// Ed25519 signature algorithm
    /// Ed25519
    Ed25519,
    /// HMAC message authentication
    Hmac {
        /// The underlying hash algorithm (e.g., "SHA-256")
        hash_algorithm: String,
    },
    /// SHA-3 hash function
    Sha3 {
        /// The SHA-3 variant (224, 256, 384, 512)
        variant: u32,
    },
}

impl Default for CryptoAlgorithm {
    fn default() -> Self {
        Self::Aes { key_size: 256 }
    }
}

/// Key derivation function types
/// `KeyDerivationFunction`
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyDerivationFunction {
    /// PBKDF2 with specified iterations
    Pbkdf2 {
        /// Iteration count; must be tuned to your threat model and backend latency budget.
        iterations: u32,
    },
    /// Argon2 memory-hard function
    Argon2 {
        /// Argon2 variant (Argon2d, Argon2i, or Argon2id)
        variant: String,
        /// Memory usage in KiB
        memory: u32,
        /// Time cost (number of iterations)
        time: u32,
    },
    /// scrypt key derivation
    Scrypt {
        /// CPU/memory cost parameter
        n: u32,
        /// Block size parameter
        r: u32,
        /// Parallelization parameter
        p: u32,
    },
    /// HKDF extract-and-expand
    Hkdf {
        /// Hash function to use (SHA-256, SHA-384, etc.)
        hash: String,
    },
}

impl Default for KeyDerivationFunction {
    fn default() -> Self {
        Self::Argon2 {
            variant: "Argon2id".to_string(),
            memory: 65536,
            time: 3,
        }
    }
}

/// Cryptographic key configuration
/// `KeyConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyConfig {
    /// Key algorithm
    /// The algorithm value
    pub algorithm: CryptoAlgorithm,
    /// Key size in bits
    /// Number of `key_size`
    pub key_size: u32,
    /// Key derivation function
    /// The kdf value
    pub kdf: KeyDerivationFunction,
    /// Key usage restrictions
    /// The usage value
    pub usage: KeyUsage,
    /// Key rotation interval in seconds
    /// Optional rotation interval seconds
    pub rotation_interval_seconds: Option<u32>,
    /// Key metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            algorithm: CryptoAlgorithm::default(),
            key_size: 256,
            kdf: KeyDerivationFunction::default(),
            usage: KeyUsage::default(),
            rotation_interval_seconds: Some(86400 * 30), // 30 days
            metadata: HashMap::new(),
        }
    }
}

/// Key usage permissions and restrictions
/// `KeyUsage`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyUsage {
    /// Allow encryption operations
    /// Whether encrypt is enabled
    pub encrypt: bool,
    /// Allow decryption operations
    /// Whether decrypt is enabled
    pub decrypt: bool,
    /// Allow signing operations
    /// Whether sign is enabled
    pub sign: bool,
    /// Allow signature verification
    /// Whether verify is enabled
    pub verify: bool,
    /// Allow key derivation
    /// Whether derive is enabled
    pub derive: bool,
    /// Allow key wrapping
    /// Whether wrap is enabled
    pub wrap: bool,
    /// Allow key unwrapping
    /// Whether unwrap is enabled
    pub unwrap: bool,
    /// Maximum number of operations (None = unlimited)
    /// Optional max operations
    pub max_operations: Option<u64>,
}

impl Default for KeyUsage {
    fn default() -> Self {
        Self {
            encrypt: true,
            decrypt: true,
            sign: false,
            verify: false,
            derive: false,
            wrap: false,
            unwrap: false,
            max_operations: None,
        }
    }
}

/// Encryption mode configuration
/// `EncryptionMode`
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum EncryptionMode {
    /// Galois/Counter Mode (authenticated encryption)
    /// Gcm
    #[default]
    Gcm,
    /// Counter Mode
    /// Ctr
    Ctr,
    /// Cipher Block Chaining
    /// Cbc
    Cbc,
    /// Electronic Codebook (not recommended)
    /// Ecb
    Ecb,
    /// ChaCha20-Poly1305 (authenticated encryption)
    /// `ChaCha20Poly1305`
    ChaCha20Poly1305,
}

/// `PaddingScheme`
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PaddingScheme {
    /// PKCS#7 padding
    /// Pkcs7
    #[default]
    Pkcs7,
    /// ANSI X9.23 padding
    /// `AnsiX923`
    AnsiX923,
    /// ISO/IEC 7816-4 padding
    /// Iso7816
    Iso7816,
    /// No padding (stream ciphers)
    /// None
    None,
}

/// Hash algorithm configuration
/// `HashAlgorithm`
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HashAlgorithm {
    /// SHA-2 family
    Sha2 {
        /// SHA-2 variant (224, 256, 384, 512)
        variant: u32,
    },
    /// SHA-3 family
    Sha3 {
        /// SHA-3 variant (224, 256, 384, 512)
        variant: u32,
    },
    /// BLAKE2 hash function
    Blake2 {
        /// BLAKE2 variant (blake2b or blake2s)
        variant: String,
    },
    /// BLAKE3 hash function
    /// Blake3
    Blake3,
}

impl Default for HashAlgorithm {
    fn default() -> Self {
        Self::Sha3 { variant: 256 }
    }
}

/// Digital signature configuration
/// `SignatureConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureConfig {
    /// Signature algorithm
    /// The algorithm value
    pub algorithm: CryptoAlgorithm,
    /// The hash algorithm value
    pub hash_algorithm: HashAlgorithm,
    /// Key configuration
    pub key_config: KeyConfig,
    /// Serialized signature container format (e.g. `DER`, `P1363`) understood by verifiers.
    pub format: String,
}

impl Default for SignatureConfig {
    fn default() -> Self {
        Self {
            algorithm: CryptoAlgorithm::Ed25519,
            hash_algorithm: HashAlgorithm::default(),
            key_config: KeyConfig::default(),
            format: "DER".to_string(),
        }
    }
}

/// Comprehensive cryptographic configuration
/// `CryptoConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CryptoConfig {
    /// Default encryption configuration
    /// The encryption value
    pub encryption: EncryptionConfig,
    /// Default signature configuration
    /// The signature value
    pub signature: SignatureConfig,
    /// Key management configuration
    /// The key management value
    pub key_management: KeyManagementConfig,
    /// Random number generation configuration
    /// The rng value
    pub rng: RngConfig,
}

/// Encryption configuration
/// `EncryptionConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// Encryption algorithm
    /// The algorithm value
    pub algorithm: CryptoAlgorithm,
    /// Encryption mode
    /// The mode value
    pub mode: EncryptionMode,
    /// Padding scheme
    /// The padding value
    pub padding: PaddingScheme,
    /// Key configuration
    pub key_config: KeyConfig,
    /// Optional auth tag size
    pub auth_tag_size: Option<u32>,
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            algorithm: CryptoAlgorithm::Aes { key_size: 256 },
            mode: EncryptionMode::Gcm,
            padding: PaddingScheme::None, // GCM doesn't need padding
            key_config: KeyConfig::default(),
            auth_tag_size: Some(128), // 128-bit auth tag for GCM
        }
    }
}

/// Key management configuration
/// `KeyManagementConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagementConfig {
    /// Key storage backend
    /// The storage backend value
    pub storage_backend: String,
    /// Key backup enabled
    /// Whether backup is enabled
    pub backup_enabled: bool,
    /// Key escrow configuration
    pub escrow_config: Option<EscrowConfig>,
    /// Key rotation policy
    /// The rotation policy value
    pub rotation_policy: RotationPolicy,
    /// Hardware security module configuration
    pub hsm_config: Option<HsmConfig>,
}

impl Default for KeyManagementConfig {
    fn default() -> Self {
        Self {
            storage_backend: "secure_enclave".to_string(),
            backup_enabled: true,
            escrow_config: None,
            rotation_policy: RotationPolicy::default(),
            hsm_config: None,
        }
    }
}

/// Key rotation policy
/// `RotationPolicy`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationPolicy {
    /// Automatic rotation enabled
    /// Whether automatic is enabled
    pub automatic: bool,
    /// Rotation interval in seconds
    /// Number of `interval_seconds`
    pub interval_seconds: u32,
    /// Maximum key age in seconds
    /// Number of `max_age_seconds`
    pub max_age_seconds: u32,
    /// Rotation on compromise
    /// Whether `rotate_on_compromise` is enabled
    pub rotate_on_compromise: bool,
}

impl Default for RotationPolicy {
    fn default() -> Self {
        Self {
            automatic: true,
            interval_seconds: 86400 * 30, // 30 days
            max_age_seconds: 86400 * 90,  // 90 days
            rotate_on_compromise: true,
        }
    }
}

/// Key escrow configuration
/// `EscrowConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscrowConfig {
    /// Escrow enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Escrow agents required
    /// Number of `agents_required`
    pub agents_required: u32,
    /// Number of threshold
    pub threshold: u32,
    /// Escrow metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

/// Hardware Security Module configuration
/// `HsmConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmConfig {
    /// HSM provider
    pub provider: String,
    /// HSM connection configuration
    /// Mapping of connection
    pub connection: HashMap<String, String>,
    /// HSM authentication configuration
    /// Mapping of authentication
    pub authentication: HashMap<String, String>,
    /// HSM feature flags
    /// Mapping of features
    pub features: HashMap<String, bool>,
}

/// Random Number Generator configuration
/// `RngConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RngConfig {
    /// RNG algorithm
    /// The algorithm value
    pub algorithm: String,
    /// Entropy source
    /// The entropy source value
    pub entropy_source: String,
    /// Seed size in bytes
    /// Number of `seed_size`
    pub seed_size: u32,
    /// Reseed interval in seconds
    /// Number of `reseed_interval_seconds`
    pub reseed_interval_seconds: u32,
}

impl Default for RngConfig {
    fn default() -> Self {
        Self {
            algorithm: "ChaCha20".to_string(),
            entropy_source: "hardware".to_string(),
            seed_size: 32,
            reseed_interval_seconds: 3600, // 1 hour
        }
    }
}

/// `KeyPairAlgorithm`
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum KeyPairAlgorithm {
    /// RSA key pair with specified key size
    Rsa {
        /// RSA key size in bits (e.g., 2048, 4096)
        bits: u16,
    },
    /// Elliptic Curve key pair
    Ec {
        /// Elliptic curve name (e.g., "P-256", "P-384", "secp256k1")
        curve: String,
    },
    /// Ed25519 signature key pair
    /// Ed25519
    #[default]
    Ed25519,
}

/// Cryptographic key pair structure
/// `CryptoKeyPair`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoKeyPair {
    /// Public key bytes
    /// Collection of public key
    pub public_key: Vec<u8>,
    /// Private key bytes (should be handled securely)
    /// Collection of private key
    pub private_key: Vec<u8>,
    /// The algorithm value
    pub algorithm: KeyPairAlgorithm,
}

impl CryptoKeyPair {
    /// Create a new key pair
    #[must_use]
    /// Creates a new instance
    pub const fn new(
        public_key: Vec<u8>,
        private_key: Vec<u8>,
        algorithm: KeyPairAlgorithm,
    ) -> Self {
        Self {
            public_key,
            private_key,
            algorithm,
        }
    }

    /// Get the public key
    #[must_use]
    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    /// Get the algorithm
    #[must_use]
    pub const fn algorithm(&self) -> &KeyPairAlgorithm {
        &self.algorithm
    }

    /// Clear the private key from memory (security measure)
    pub fn clear_private_key(&mut self) {
        self.private_key.clear();
        self.private_key.shrink_to_fit();
    }
}

impl CryptoConfig {
    /// Create new crypto configuration with secure defaults
    #[must_use]
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate the cryptographic configuration
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), String> {
        // Validate key sizes
        match &self.encryption.algorithm {
            CryptoAlgorithm::Aes { key_size } => {
                if ![128, 192, 256].contains(key_size) {
                    return Err("Invalid AES key size".to_string());
                }
            }
            CryptoAlgorithm::Rsa { key_size } => {
                if *key_size < 2048 {
                    return Err("RSA key size must be at least 2048 bits".to_string());
                }
            }
            _ => {}
        }

        // Validate encryption mode compatibility
        match (&self.encryption.algorithm, &self.encryption.mode) {
            (CryptoAlgorithm::ChaCha20, EncryptionMode::ChaCha20Poly1305)
            | (
                CryptoAlgorithm::Aes { .. },
                EncryptionMode::Gcm | EncryptionMode::Ctr | EncryptionMode::Cbc,
            ) => {}
            _ => return Err("Incompatible algorithm and mode combination".to_string()),
        }

        Ok(())
    }

    /// Heuristic: returns true when the active algorithms are labeled or classified as post-quantum ready.
    #[must_use]
    pub const fn is_post_quantum(&self) -> bool {
        // This would be extended with actual post-quantum algorithms
        matches!(self.encryption.algorithm, CryptoAlgorithm::Ed25519)
    }

    /// Best-effort symmetric-equivalent key size in bits for policy comparisons.
    #[must_use]
    pub const fn effective_key_size(&self) -> u32 {
        match &self.encryption.algorithm {
            CryptoAlgorithm::Aes { key_size } | CryptoAlgorithm::Rsa { key_size } => *key_size,
            CryptoAlgorithm::ChaCha20 | CryptoAlgorithm::Ed25519 | CryptoAlgorithm::Ecc { .. } => {
                256
            } // Assuming P-256
            _ => self.encryption.key_config.key_size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // CryptoAlgorithm tests
    #[test]
    fn test_crypto_algorithm_default() {
        let algo = CryptoAlgorithm::default();
        assert!(matches!(algo, CryptoAlgorithm::Aes { key_size: 256 }));
    }

    #[test]
    fn test_crypto_algorithm_aes_variants() {
        let aes128 = CryptoAlgorithm::Aes { key_size: 128 };
        let aes256 = CryptoAlgorithm::Aes { key_size: 256 };

        assert_ne!(aes128, aes256);
    }

    #[test]
    fn test_crypto_algorithm_rsa() {
        let rsa = CryptoAlgorithm::Rsa { key_size: 2048 };
        match rsa {
            CryptoAlgorithm::Rsa { key_size } => {
                assert_eq!(key_size, 2048);
            }
            _ => panic!("Expected RSA variant"),
        }
    }

    #[test]
    fn test_crypto_algorithm_ecc() {
        let ecc = CryptoAlgorithm::Ecc {
            curve: "P-256".to_string(),
        };
        match ecc {
            CryptoAlgorithm::Ecc { curve } => {
                assert_eq!(curve, "P-256");
            }
            _ => panic!("Expected ECC variant"),
        }
    }

    // KeyDerivationFunction tests
    #[test]
    fn test_kdf_default() {
        let kdf = KeyDerivationFunction::default();
        match kdf {
            KeyDerivationFunction::Argon2 {
                variant,
                memory,
                time,
            } => {
                assert_eq!(variant, "Argon2id");
                assert_eq!(memory, 65536);
                assert_eq!(time, 3);
            }
            _ => panic!("Expected Argon2 variant"),
        }
    }

    #[test]
    fn test_kdf_pbkdf2() {
        let kdf = KeyDerivationFunction::Pbkdf2 {
            iterations: 100_000,
        };
        match kdf {
            KeyDerivationFunction::Pbkdf2 { iterations } => {
                assert_eq!(iterations, 100_000);
            }
            _ => panic!("Expected PBKDF2 variant"),
        }
    }

    #[test]
    fn test_kdf_scrypt() {
        let kdf = KeyDerivationFunction::Scrypt {
            n: 16384,
            r: 8,
            p: 1,
        };
        match kdf {
            KeyDerivationFunction::Scrypt { n, r, p } => {
                assert_eq!(n, 16384);
                assert_eq!(r, 8);
                assert_eq!(p, 1);
            }
            _ => panic!("Expected Scrypt variant"),
        }
    }

    // KeyConfig tests
    #[test]
    fn test_key_config_default() {
        let config = KeyConfig::default();

        assert_eq!(config.key_size, 256);
        assert!(config.rotation_interval_seconds.is_some());
        assert_eq!(
            config
                .rotation_interval_seconds
                .expect("default rotation interval"),
            86400 * 30
        );
        assert!(config.metadata.is_empty());
    }

    #[test]
    fn test_key_config_custom() {
        let mut metadata = HashMap::new();
        metadata.insert("purpose".to_string(), "test".to_string());

        let config = KeyConfig {
            algorithm: CryptoAlgorithm::Aes { key_size: 128 },
            key_size: 128,
            kdf: KeyDerivationFunction::default(),
            usage: KeyUsage::default(),
            rotation_interval_seconds: Some(3600),
            metadata,
        };

        assert_eq!(config.key_size, 128);
        assert_eq!(
            config
                .rotation_interval_seconds
                .expect("custom rotation interval"),
            3600
        );
        assert_eq!(config.metadata.len(), 1);
    }

    // KeyUsage tests
    #[test]
    fn test_key_usage_default() {
        let usage = KeyUsage::default();

        assert!(usage.encrypt);
        assert!(usage.decrypt);
        assert!(!usage.sign);
        assert!(!usage.verify);
        assert!(!usage.derive);
        assert!(usage.max_operations.is_none());
    }

    #[test]
    fn test_key_usage_signing() {
        let usage = KeyUsage {
            encrypt: false,
            decrypt: false,
            sign: true,
            verify: true,
            derive: false,
            wrap: false,
            unwrap: false,
            max_operations: Some(1000),
        };

        assert!(!usage.encrypt);
        assert!(usage.sign);
        assert!(usage.verify);
        assert_eq!(usage.max_operations.expect("max operations"), 1000);
    }

    // EncryptionMode tests
    #[test]
    fn test_encryption_mode_default() {
        let mode = EncryptionMode::default();
        assert_eq!(mode, EncryptionMode::Gcm);
    }

    #[test]
    fn test_encryption_mode_variants() {
        let modes = [
            EncryptionMode::Gcm,
            EncryptionMode::Ctr,
            EncryptionMode::Cbc,
            EncryptionMode::ChaCha20Poly1305,
        ];

        assert_eq!(modes.len(), 4);
        assert!(modes.contains(&EncryptionMode::Gcm));
    }

    // HashAlgorithm tests
    #[test]
    fn test_hash_algorithm_sha2() {
        let sha256 = HashAlgorithm::Sha2 { variant: 256 };
        let sha512 = HashAlgorithm::Sha2 { variant: 512 };

        assert_ne!(sha256, sha512);
    }

    #[test]
    fn test_hash_algorithm_sha3() {
        let sha3_256 = HashAlgorithm::Sha3 { variant: 256 };
        match sha3_256 {
            HashAlgorithm::Sha3 { variant } => {
                assert_eq!(variant, 256);
            }
            _ => panic!("Expected Sha3 variant"),
        }
    }

    // CryptoConfig tests
    #[test]
    fn test_crypto_config_default() {
        let _config = CryptoConfig::default();
        // Just verify it can be created without panicking
    }

    // KeyPairAlgorithm tests
    #[test]
    fn test_keypair_algorithm_ed25519() {
        let ed25519 = KeyPairAlgorithm::Ed25519;
        assert!(matches!(ed25519, KeyPairAlgorithm::Ed25519));
    }

    #[test]
    fn test_keypair_algorithm_default() {
        let default = KeyPairAlgorithm::default();
        assert!(matches!(default, KeyPairAlgorithm::Ed25519));
    }

    #[test]
    fn test_keypair_algorithm_ec() {
        let ec = KeyPairAlgorithm::Ec {
            curve: "P-256".to_string(),
        };
        match ec {
            KeyPairAlgorithm::Ec { curve } => {
                assert_eq!(curve, "P-256");
            }
            _ => panic!("Expected Ec variant"),
        }
    }

    #[test]
    fn test_keypair_algorithm_rsa() {
        let rsa = KeyPairAlgorithm::Rsa { bits: 2048 };
        match rsa {
            KeyPairAlgorithm::Rsa { bits } => {
                assert_eq!(bits, 2048);
            }
            _ => panic!("Expected RSA variant"),
        }
    }

    // Serialization tests
    #[test]
    fn test_crypto_algorithm_serialization() {
        let algo = CryptoAlgorithm::default();
        let json = serde_json::to_string(&algo);
        assert!(json.is_ok(), "Should be able to serialize CryptoAlgorithm");
    }

    #[test]
    fn test_key_config_serialization() {
        let config = KeyConfig::default();
        let json = serde_json::to_string(&config);
        assert!(json.is_ok(), "Should be able to serialize KeyConfig");
    }

    #[test]
    fn test_encryption_mode_serialization() {
        let mode = EncryptionMode::Gcm;
        let json = serde_json::to_string(&mode);
        assert!(json.is_ok(), "Should be able to serialize EncryptionMode");
    }
}
