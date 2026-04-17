// SPDX-License-Identifier: AGPL-3.0-or-later
//! Key generation for supported algorithms.

#![cfg(test)]

use super::common::*;

/// Test key generation for AES-256
#[tokio::test]
async fn test_generate_aes256_key() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    let request = GenerateKeyRequest {
        key_type: KeyType::Aes,
        key_id: "test-aes256-key".to_string(),
    };

    let key = hsm.generate_key(request).await?;
    assert_eq!(key.metadata.key_id, "test-aes256-key");
    assert_eq!(key.metadata.key_type, KeyType::Aes);

    Ok(())
}

/// Test key generation for Ed25519 (signing)
#[tokio::test]
async fn test_generate_ed25519_key() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    let request = GenerateKeyRequest {
        key_type: KeyType::Ed25519,
        key_id: "test-ed25519-key".to_string(),
    };

    let key = hsm.generate_key(request).await?;
    assert_eq!(key.metadata.key_id, "test-ed25519-key");
    assert_eq!(key.metadata.key_type, KeyType::Ed25519);

    Ok(())
}

/// Test key generation for P256 (ECDSA)
#[tokio::test]
async fn test_generate_p256_key() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    let request = GenerateKeyRequest {
        key_type: KeyType::EllipticCurve, // Vendor-agnostic
        key_id: "test-p256-key".to_string(),
    };

    let key = hsm.generate_key(request).await?;
    assert_eq!(key.metadata.key_id, "test-p256-key");
    assert_eq!(key.metadata.key_type, KeyType::EllipticCurve);

    Ok(())
}

/// Test key generation with all supported algorithms
#[tokio::test]
async fn test_all_key_algorithm_types() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // AES-128
    let aes128_request = GenerateKeyRequest {
        key_type: KeyType::Aes,
        key_id: "aes128-key".to_string(),
    };
    hsm.generate_key(aes128_request).await?;

    // AES-256
    let aes256_request = GenerateKeyRequest {
        key_type: KeyType::Aes,
        key_id: "aes256-key".to_string(),
    };
    hsm.generate_key(aes256_request).await?;

    // Ed25519
    let ed25519_request = GenerateKeyRequest {
        key_type: KeyType::Ed25519,
        key_id: "ed25519-key".to_string(),
    };
    hsm.generate_key(ed25519_request).await?;

    // ECC P-256
    let ecc_request = GenerateKeyRequest {
        key_type: KeyType::EllipticCurve, // Vendor-agnostic
        key_id: "ecc-key".to_string(),
    };
    hsm.generate_key(ecc_request).await?;

    // Verify all keys exist
    assert!(hsm.get_key_info("aes128-key").await.is_ok());
    assert!(hsm.get_key_info("aes256-key").await.is_ok());
    assert!(hsm.get_key_info("ed25519-key").await.is_ok());
    assert!(hsm.get_key_info("ecc-key").await.is_ok());

    Ok(())
}
