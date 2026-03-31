// SPDX-License-Identifier: AGPL-3.0-only

use super::*;
use std::collections::HashMap;

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
