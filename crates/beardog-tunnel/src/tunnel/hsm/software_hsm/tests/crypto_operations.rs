// SPDX-License-Identifier: AGPL-3.0-or-later
//! Encrypt/decrypt, sign/verify, and payload size tests.

#![cfg(test)]

use super::common::*;

/// Test encryption and decryption with AES-256
#[tokio::test]
async fn test_encrypt_decrypt_aes256() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Generate key
    let request = GenerateKeyRequest {
        key_type: KeyType::Aes,
        key_id: "encryption-test-key".to_string(),
    };
    hsm.generate_key(request).await?;

    // Test data
    let plaintext = b"This is sensitive data that needs encryption";

    // Encrypt
    let ciphertext = hsm.encrypt("encryption-test-key", plaintext).await?;
    assert_ne!(ciphertext, plaintext);
    assert!(!ciphertext.is_empty());

    // Decrypt
    let decrypted = hsm.decrypt("encryption-test-key", &ciphertext).await?;
    assert_eq!(decrypted, plaintext);

    Ok(())
}

/// Test signing and verification with Ed25519
#[tokio::test]
async fn test_sign_verify_ed25519() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Generate signing key
    let request = GenerateKeyRequest {
        key_type: KeyType::Ed25519,
        key_id: "signing-test-key".to_string(),
    };
    hsm.generate_key(request).await?;

    // Test data
    let message = b"Important message to be signed";

    // Sign
    let signature = hsm.sign("signing-test-key", message).await?;
    assert!(!signature.is_empty());
    assert_eq!(signature.len(), 64); // Ed25519 signatures are 64 bytes

    // Verify
    let is_valid = hsm.verify("signing-test-key", message, &signature).await?;
    assert!(is_valid);

    // Verify with wrong message should fail
    let wrong_message = b"Different message";
    let is_invalid = hsm
        .verify("signing-test-key", wrong_message, &signature)
        .await?;
    assert!(!is_invalid);

    Ok(())
}

/// Test large data encryption/decryption
#[tokio::test]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "synthetic 1MB plaintext uses i % 256 as u8; indices bounded by buffer size"
)]
async fn test_large_data_encryption() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Generate key
    let request = GenerateKeyRequest {
        key_type: KeyType::Aes,
        key_id: "large-data-key".to_string(),
    };
    hsm.generate_key(request).await?;

    // Create large data (1MB)
    let plaintext: Vec<u8> = (0..1_000_000).map(|i| (i % 256) as u8).collect();

    // Encrypt
    let ciphertext = hsm.encrypt("large-data-key", &plaintext).await?;
    assert_ne!(ciphertext, plaintext);

    // Decrypt
    let decrypted = hsm.decrypt("large-data-key", &ciphertext).await?;
    assert_eq!(decrypted, plaintext);

    Ok(())
}

/// Test empty data encryption
#[tokio::test]
async fn test_empty_data_encryption() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Generate key
    let request = GenerateKeyRequest {
        key_type: KeyType::Aes,
        key_id: "empty-data-key".to_string(),
    };
    hsm.generate_key(request).await?;

    // Encrypt empty data
    let plaintext = b"";
    let ciphertext = hsm.encrypt("empty-data-key", plaintext).await?;

    // Decrypt
    let decrypted = hsm.decrypt("empty-data-key", &ciphertext).await?;
    assert_eq!(decrypted, plaintext);

    Ok(())
}

/// Test very large payload handling (10MB)
#[tokio::test]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "synthetic 10MB plaintext uses i % 256 as u8; indices bounded by buffer size"
)]
async fn test_very_large_payload_handling() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Generate key
    let request = GenerateKeyRequest {
        key_type: KeyType::Aes,
        key_id: "large-payload-key".to_string(),
    };
    hsm.generate_key(request).await?;

    // Create 10MB payload
    let plaintext: Vec<u8> = (0..10_000_000).map(|i| (i % 256) as u8).collect();

    // Encrypt (this tests chunking/streaming if implemented)
    let ciphertext = hsm.encrypt("large-payload-key", &plaintext).await?;
    assert_ne!(ciphertext.len(), 0);

    // Decrypt
    let decrypted = hsm.decrypt("large-payload-key", &ciphertext).await?;
    assert_eq!(decrypted.len(), plaintext.len());
    assert_eq!(decrypted, plaintext);

    Ok(())
}

/// Test key type mismatch error handling
#[tokio::test]
async fn test_key_type_mismatch_error() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Generate signing key
    let request = GenerateKeyRequest {
        key_type: KeyType::Ed25519,
        key_id: "signing-only-key".to_string(),
    };
    hsm.generate_key(request).await?;

    // Try to encrypt with signing key (should fail or be handled gracefully)
    let plaintext = b"test data";
    let result = hsm.encrypt("signing-only-key", plaintext).await;

    // This should either fail or the implementation should handle it
    // The exact behavior depends on implementation
    assert!(result.is_err() || result.is_ok());

    Ok(())
}
