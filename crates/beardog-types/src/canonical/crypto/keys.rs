// SPDX-License-Identifier: AGPL-3.0-only

use super::algorithms::{CryptoAlgorithm, KeyDerivationFunction};
use super::constants::ROTATION_30_DAYS_SECS;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
            rotation_interval_seconds: Some(ROTATION_30_DAYS_SECS), // 30 days
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
