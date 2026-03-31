// SPDX-License-Identifier: AGPL-3.0-only

use crate::constants::domains::security::crypto as security_crypto;
use serde::{Deserialize, Serialize};

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
            memory: security_crypto::ARGON2_MEMORY_COST,
            time: security_crypto::ARGON2_TIME_COST,
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
