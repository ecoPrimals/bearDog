//! Crypto Requirements
//!
//! Defines requirements for selecting a crypto provider, similar to HSM requirements.

use super::algorithms::*;
use super::capabilities::Platform;
use crate::tunnel::hsm::types::key::KeyMetadata;
use serde::{Deserialize, Serialize};

/// Requirements for crypto operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoRequirements {
    /// What operation needs to be performed
    pub operation: CryptoOperation,

    /// Specific algorithm (if known)
    pub algorithm: Option<CryptoAlgorithm>,

    /// Security requirements
    pub min_security_level: u8, // Security level (0-10)
    pub require_constant_time: bool,
    pub side_channel_protection: bool,

    /// Performance requirements
    pub max_latency_us: Option<u64>,
    pub min_throughput_mbps: Option<f64>,

    /// Platform requirements
    pub require_hardware_accel: bool,
    pub platform_specific: Option<Platform>,

    /// Preference ordering
    pub prefer_performance: bool, // vs prefer_security
}

impl Default for CryptoRequirements {
    fn default() -> Self {
        Self {
            operation: CryptoOperation::SymmetricEncryption,
            algorithm: None,
            min_security_level: 5, // Default to medium security level
            require_constant_time: true,
            side_channel_protection: true,
            max_latency_us: None,
            min_throughput_mbps: None,
            require_hardware_accel: false,
            platform_specific: None,
            prefer_performance: false,
        }
    }
}

impl CryptoRequirements {
    /// Create requirements from key metadata
    pub fn from_key_metadata(metadata: &KeyMetadata) -> Self {
        let (operation, algorithm) = Self::algorithm_from_key_type(&metadata.key_type);

        Self {
            operation,
            algorithm: Some(algorithm),
            min_security_level: 7, // Higher security for key-derived operations
            require_constant_time: true,
            side_channel_protection: true,
            max_latency_us: None,
            min_throughput_mbps: None,
            require_hardware_accel: false,
            platform_specific: None,
            prefer_performance: false,
        }
    }

    /// Determine algorithm from key type
    fn algorithm_from_key_type(
        key_type: &crate::tunnel::hsm::types::key::KeyType,
    ) -> (CryptoOperation, CryptoAlgorithm) {
        use crate::tunnel::hsm::types::key::KeyType;

        match key_type {
            KeyType::Aes { key_size } => (
                CryptoOperation::SymmetricEncryption,
                CryptoAlgorithm::Symmetric(SymmetricAlgorithm::Aes {
                    mode: AesMode::Gcm,
                    key_size: *key_size,
                }),
            ),
            KeyType::Ed25519 => (
                CryptoOperation::Signing,
                CryptoAlgorithm::Signature(SignatureAlgorithm::Ed25519),
            ),
            KeyType::EccP256 => (
                CryptoOperation::Signing,
                CryptoAlgorithm::Signature(SignatureAlgorithm::EcdsaP256 {
                    hash: HashAlgorithm::Sha256,
                }),
            ),
            KeyType::EccP384 => (
                CryptoOperation::Signing,
                CryptoAlgorithm::Signature(SignatureAlgorithm::EcdsaP384 {
                    hash: HashAlgorithm::Sha384,
                }),
            ),
            KeyType::Rsa { key_size } => (
                CryptoOperation::AsymmetricEncryption,
                CryptoAlgorithm::Asymmetric(AsymmetricAlgorithm::RsaOaep {
                    key_size: *key_size,
                    hash: HashAlgorithm::Sha256,
                }),
            ),
            // Generic or other types default to AES-256-GCM
            _ => (
                CryptoOperation::SymmetricEncryption,
                CryptoAlgorithm::Symmetric(SymmetricAlgorithm::Aes {
                    mode: AesMode::Gcm,
                    key_size: 256,
                }),
            ),
        }
    }

    /// Create requirements for encryption
    pub fn for_encryption(algorithm: SymmetricAlgorithm) -> Self {
        Self {
            operation: CryptoOperation::SymmetricEncryption,
            algorithm: Some(CryptoAlgorithm::Symmetric(algorithm)),
            ..Default::default()
        }
    }

    /// Create requirements for signing
    pub fn for_signing(algorithm: SignatureAlgorithm) -> Self {
        Self {
            operation: CryptoOperation::Signing,
            algorithm: Some(CryptoAlgorithm::Signature(algorithm)),
            ..Default::default()
        }
    }
}
