// SPDX-License-Identifier: AGPL-3.0-only

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
    /// Minimum security level (0-10)
    pub min_security_level: u8,
    /// Whether constant-time operations are required
    pub require_constant_time: bool,
    /// Whether side-channel attack protection is required
    pub side_channel_protection: bool,

    /// Maximum acceptable latency in microseconds
    pub max_latency_us: Option<u64>,
    /// Minimum required throughput in MB/s
    pub min_throughput_mbps: Option<f64>,

    /// Whether hardware acceleration is required
    pub require_hardware_accel: bool,
    /// Platform-specific requirement (if any)
    pub platform_specific: Option<Platform>,

    /// Whether to prefer performance over security
    pub prefer_performance: bool,
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
            KeyType::Aes => (
                CryptoOperation::SymmetricEncryption,
                CryptoAlgorithm::Symmetric(SymmetricAlgorithm::Aes {
                    mode: AesMode::Gcm,
                    key_size: 256, // Default to AES-256 for vendor-agnostic
                }),
            ),
            KeyType::Ed25519 => (
                CryptoOperation::Signing,
                CryptoAlgorithm::Signature(SignatureAlgorithm::Ed25519),
            ),
            KeyType::EllipticCurve => (
                CryptoOperation::Signing,
                CryptoAlgorithm::Signature(SignatureAlgorithm::EcdsaP256 {
                    hash: HashAlgorithm::Sha256,
                }),
            ),
            KeyType::Rsa => (
                CryptoOperation::AsymmetricEncryption,
                CryptoAlgorithm::Asymmetric(AsymmetricAlgorithm::RsaOaep {
                    key_size: 2048, // Default to RSA-2048 for vendor-agnostic
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tunnel::hsm::types::key::KeyType;
    use chrono::Utc;
    use std::collections::HashMap;

    fn create_test_key_metadata(key_type: KeyType) -> KeyMetadata {
        KeyMetadata {
            key_id: "test-key-123".to_string(),
            key_type,
            alias: Some("test-key".to_string()),
            created_at: Utc::now(),
            expires_at: None,
            tags: HashMap::new(),
        }
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: crypto
    // TEST_PRIORITY: normal
    #[test]
    fn test_crypto_requirements_default() {
        let reqs = CryptoRequirements::default();
        assert_eq!(reqs.operation, CryptoOperation::SymmetricEncryption);
        assert!(reqs.algorithm.is_none());
        assert_eq!(reqs.min_security_level, 5);
        assert!(reqs.require_constant_time);
        assert!(reqs.side_channel_protection);
        assert!(reqs.max_latency_us.is_none());
        assert!(reqs.min_throughput_mbps.is_none());
        assert!(!reqs.require_hardware_accel);
        assert!(reqs.platform_specific.is_none());
        assert!(!reqs.prefer_performance);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: crypto
    // TEST_PRIORITY: normal
    #[test]
    fn test_from_key_metadata_aes() {
        let metadata = create_test_key_metadata(KeyType::Aes);
        let reqs = CryptoRequirements::from_key_metadata(&metadata);

        assert_eq!(reqs.operation, CryptoOperation::SymmetricEncryption);
        assert!(reqs.algorithm.is_some());
        if let Some(CryptoAlgorithm::Symmetric(SymmetricAlgorithm::Aes { mode, key_size })) =
            reqs.algorithm
        {
            assert_eq!(mode, AesMode::Gcm);
            assert_eq!(key_size, 256);
        } else {
            panic!("Expected AES algorithm");
        }
        assert_eq!(reqs.min_security_level, 7);
        assert!(reqs.require_constant_time);
        assert!(reqs.side_channel_protection);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: crypto
    // TEST_PRIORITY: normal
    #[test]
    fn test_from_key_metadata_ed25519() {
        let metadata = create_test_key_metadata(KeyType::Ed25519);
        let reqs = CryptoRequirements::from_key_metadata(&metadata);

        assert_eq!(reqs.operation, CryptoOperation::Signing);
        assert!(reqs.algorithm.is_some());
        if let Some(CryptoAlgorithm::Signature(SignatureAlgorithm::Ed25519)) = reqs.algorithm {
            // Expected
        } else {
            panic!("Expected Ed25519 signature algorithm");
        }
        assert_eq!(reqs.min_security_level, 7);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: crypto
    // TEST_PRIORITY: normal
    #[test]
    fn test_from_key_metadata_elliptic_curve() {
        let metadata = create_test_key_metadata(KeyType::EllipticCurve);
        let reqs = CryptoRequirements::from_key_metadata(&metadata);

        assert_eq!(reqs.operation, CryptoOperation::Signing);
        assert!(reqs.algorithm.is_some());
        if let Some(CryptoAlgorithm::Signature(SignatureAlgorithm::EcdsaP256 { hash })) =
            reqs.algorithm
        {
            assert_eq!(hash, HashAlgorithm::Sha256);
        } else {
            panic!("Expected ECDSA P256 signature algorithm");
        }
        assert_eq!(reqs.min_security_level, 7);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: crypto
    // TEST_PRIORITY: normal
    #[test]
    fn test_from_key_metadata_rsa() {
        let metadata = create_test_key_metadata(KeyType::Rsa);
        let reqs = CryptoRequirements::from_key_metadata(&metadata);

        assert_eq!(reqs.operation, CryptoOperation::AsymmetricEncryption);
        assert!(reqs.algorithm.is_some());
        if let Some(CryptoAlgorithm::Asymmetric(AsymmetricAlgorithm::RsaOaep { key_size, hash })) =
            reqs.algorithm
        {
            assert_eq!(key_size, 2048);
            assert_eq!(hash, HashAlgorithm::Sha256);
        } else {
            panic!("Expected RSA-OAEP algorithm");
        }
        assert_eq!(reqs.min_security_level, 7);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: crypto
    // TEST_PRIORITY: normal
    #[test]
    fn test_from_key_metadata_chacha20() {
        let metadata = create_test_key_metadata(KeyType::ChaCha20);
        let reqs = CryptoRequirements::from_key_metadata(&metadata);

        // ChaCha20 falls through to default AES-256-GCM
        assert_eq!(reqs.operation, CryptoOperation::SymmetricEncryption);
        assert!(reqs.algorithm.is_some());
        if let Some(CryptoAlgorithm::Symmetric(SymmetricAlgorithm::Aes { mode, key_size })) =
            reqs.algorithm
        {
            assert_eq!(mode, AesMode::Gcm);
            assert_eq!(key_size, 256);
        } else {
            panic!("Expected AES algorithm for ChaCha20 fallback");
        }
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: crypto
    // TEST_PRIORITY: normal
    #[test]
    fn test_from_key_metadata_x25519() {
        let metadata = create_test_key_metadata(KeyType::X25519);
        let reqs = CryptoRequirements::from_key_metadata(&metadata);

        // X25519 falls through to default AES-256-GCM
        assert_eq!(reqs.operation, CryptoOperation::SymmetricEncryption);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: crypto
    // TEST_PRIORITY: normal
    #[test]
    fn test_from_key_metadata_generic() {
        let metadata = create_test_key_metadata(KeyType::Generic);
        let reqs = CryptoRequirements::from_key_metadata(&metadata);

        // Generic falls through to default AES-256-GCM
        assert_eq!(reqs.operation, CryptoOperation::SymmetricEncryption);
        assert!(reqs.algorithm.is_some());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: crypto
    // TEST_PRIORITY: normal
    #[test]
    fn test_from_key_metadata_custom() {
        let metadata = create_test_key_metadata(KeyType::Custom("MyCustomKey".to_string()));
        let reqs = CryptoRequirements::from_key_metadata(&metadata);

        // Custom falls through to default AES-256-GCM
        assert_eq!(reqs.operation, CryptoOperation::SymmetricEncryption);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: crypto
    // TEST_PRIORITY: normal
    #[test]
    fn test_for_encryption_aes256gcm() {
        let algorithm = SymmetricAlgorithm::Aes256Gcm;
        let reqs = CryptoRequirements::for_encryption(algorithm);

        assert_eq!(reqs.operation, CryptoOperation::SymmetricEncryption);
        assert!(reqs.algorithm.is_some());
        if let Some(CryptoAlgorithm::Symmetric(algo)) = reqs.algorithm {
            assert_eq!(algo, SymmetricAlgorithm::Aes256Gcm);
        } else {
            panic!("Expected symmetric algorithm");
        }
        assert_eq!(reqs.min_security_level, 5);
        assert!(reqs.require_constant_time);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: crypto
    // TEST_PRIORITY: normal
    #[test]
    fn test_for_encryption_aes128gcm() {
        let algorithm = SymmetricAlgorithm::Aes128Gcm;
        let reqs = CryptoRequirements::for_encryption(algorithm);

        assert_eq!(reqs.operation, CryptoOperation::SymmetricEncryption);
        if let Some(CryptoAlgorithm::Symmetric(algo)) = reqs.algorithm {
            assert_eq!(algo, SymmetricAlgorithm::Aes128Gcm);
        } else {
            panic!("Expected symmetric algorithm");
        }
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: crypto
    // TEST_PRIORITY: normal
    #[test]
    fn test_for_encryption_chacha20poly1305() {
        let algorithm = SymmetricAlgorithm::ChaCha20Poly1305;
        let reqs = CryptoRequirements::for_encryption(algorithm);

        assert_eq!(reqs.operation, CryptoOperation::SymmetricEncryption);
        if let Some(CryptoAlgorithm::Symmetric(algo)) = reqs.algorithm {
            assert_eq!(algo, SymmetricAlgorithm::ChaCha20Poly1305);
        } else {
            panic!("Expected symmetric algorithm");
        }
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: crypto
    // TEST_PRIORITY: normal
    #[test]
    fn test_for_signing_ed25519() {
        let algorithm = SignatureAlgorithm::Ed25519;
        let reqs = CryptoRequirements::for_signing(algorithm);

        assert_eq!(reqs.operation, CryptoOperation::Signing);
        assert!(reqs.algorithm.is_some());
        if let Some(CryptoAlgorithm::Signature(algo)) = reqs.algorithm {
            assert_eq!(algo, SignatureAlgorithm::Ed25519);
        } else {
            panic!("Expected signature algorithm");
        }
        assert_eq!(reqs.min_security_level, 5);
        assert!(reqs.require_constant_time);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: crypto
    // TEST_PRIORITY: normal
    #[test]
    fn test_for_signing_ecdsa_p256() {
        let algorithm = SignatureAlgorithm::EcdsaP256 {
            hash: HashAlgorithm::Sha256,
        };
        let reqs = CryptoRequirements::for_signing(algorithm);

        assert_eq!(reqs.operation, CryptoOperation::Signing);
        if let Some(CryptoAlgorithm::Signature(SignatureAlgorithm::EcdsaP256 { hash })) =
            reqs.algorithm
        {
            assert_eq!(hash, HashAlgorithm::Sha256);
        } else {
            panic!("Expected ECDSA P256 signature algorithm");
        }
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: crypto
    // TEST_PRIORITY: normal
    #[test]
    fn test_crypto_requirements_clone() {
        let reqs = CryptoRequirements::default();
        let cloned = reqs.clone();
        assert_eq!(cloned.min_security_level, reqs.min_security_level);
        assert_eq!(cloned.require_constant_time, reqs.require_constant_time);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: crypto
    // TEST_PRIORITY: normal
    #[test]
    fn test_crypto_requirements_debug() {
        let reqs = CryptoRequirements::default();
        let debug_str = format!("{:?}", reqs);
        assert!(debug_str.contains("CryptoRequirements"));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: crypto
    // TEST_PRIORITY: normal
    #[test]
    fn test_crypto_requirements_with_custom_settings() {
        let reqs = CryptoRequirements {
            operation: CryptoOperation::Hashing,
            algorithm: Some(CryptoAlgorithm::Hash(HashAlgorithm::Sha256)),
            min_security_level: 8,
            require_constant_time: true,
            side_channel_protection: true,
            max_latency_us: Some(1000),
            min_throughput_mbps: Some(100.0),
            require_hardware_accel: true,
            platform_specific: Some(Platform::Linux),
            prefer_performance: true,
        };

        assert_eq!(reqs.operation, CryptoOperation::Hashing);
        assert_eq!(reqs.min_security_level, 8);
        assert_eq!(reqs.max_latency_us, Some(1000));
        assert_eq!(reqs.min_throughput_mbps, Some(100.0));
        assert!(reqs.require_hardware_accel);
        assert!(reqs.prefer_performance);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: crypto
    // TEST_PRIORITY: normal
    #[test]
    fn test_for_encryption_inherits_default_security() {
        let algorithm = SymmetricAlgorithm::Aes256Gcm;
        let reqs = CryptoRequirements::for_encryption(algorithm);

        // Should inherit default security settings
        assert_eq!(reqs.min_security_level, 5);
        assert!(reqs.require_constant_time);
        assert!(reqs.side_channel_protection);
        assert!(!reqs.prefer_performance);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: crypto
    // TEST_PRIORITY: normal
    #[test]
    fn test_for_signing_inherits_default_security() {
        let algorithm = SignatureAlgorithm::Ed25519;
        let reqs = CryptoRequirements::for_signing(algorithm);

        // Should inherit default security settings
        assert_eq!(reqs.min_security_level, 5);
        assert!(reqs.require_constant_time);
        assert!(reqs.side_channel_protection);
        assert!(!reqs.prefer_performance);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: crypto
    // TEST_PRIORITY: normal
    #[test]
    fn test_algorithm_from_key_type_coverage() {
        // Test all KeyType variants through from_key_metadata
        let key_types = vec![
            KeyType::Aes,
            KeyType::Ed25519,
            KeyType::EllipticCurve,
            KeyType::Rsa,
            KeyType::ChaCha20,
            KeyType::X25519,
            KeyType::Generic,
            KeyType::Custom("test".to_string()),
        ];

        for key_type in key_types {
            let metadata = create_test_key_metadata(key_type);
            let reqs = CryptoRequirements::from_key_metadata(&metadata);
            // Just ensure it doesn't panic and produces valid requirements
            assert!(reqs.min_security_level >= 5);
            assert!(reqs.algorithm.is_some());
        }
    }
}
