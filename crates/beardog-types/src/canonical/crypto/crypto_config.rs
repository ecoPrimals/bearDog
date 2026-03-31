// SPDX-License-Identifier: AGPL-3.0-only

use super::algorithms::{CryptoAlgorithm, EncryptionMode, PaddingScheme};
use super::key_management::{KeyManagementConfig, RngConfig};
use super::keys::KeyConfig;
use super::signature::SignatureConfig;
use serde::{Deserialize, Serialize};

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

impl CryptoConfig {
    /// Create new crypto configuration with secure defaults
    #[must_use]
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate the cryptographic configuration
    /// Validates input
    ///
    /// # Errors
    ///
    /// Returns an error if key sizes, algorithm/mode pairing, or other crypto settings are invalid.
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
