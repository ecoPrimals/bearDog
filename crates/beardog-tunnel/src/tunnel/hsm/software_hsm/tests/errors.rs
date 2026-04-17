// SPDX-License-Identifier: AGPL-3.0-or-later
//! Error paths, invalid inputs, and recovery.

#![cfg(test)]

use super::common::*;

/// Test error handling - encrypt with non-existent key
#[tokio::test]
async fn test_encrypt_with_nonexistent_key_fails() {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await.unwrap();

    let plaintext = b"test data";
    let result = hsm.encrypt("nonexistent-key", plaintext).await;

    assert!(result.is_err());
}

/// Test error handling - sign with non-existent key
#[tokio::test]
async fn test_sign_with_nonexistent_key_fails() {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await.unwrap();

    let message = b"test message";
    let result = hsm.sign("nonexistent-key", message).await;

    assert!(result.is_err());
}

/// Test error handling - delete non-existent key
#[tokio::test]
async fn test_delete_nonexistent_key_fails() {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await.unwrap();

    let result = hsm.delete_key("nonexistent-key").await;
    assert!(result.is_err());
}

/// Test error recovery from failed operations
#[tokio::test]
async fn test_error_recovery_from_failed_operations() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Try to use non-existent key (should fail)
    let result1 = hsm.encrypt("nonexistent-key", b"test").await;
    assert!(result1.is_err());

    // HSM should still be functional after error
    let health = hsm.health_check().await?;
    assert!(health.is_healthy);

    // Should be able to generate new key after error
    let request = GenerateKeyRequest {
        key_type: KeyType::Aes,
        key_id: "recovery-test-key".to_string(),
    };
    hsm.generate_key(request).await?;

    // Should be able to use new key
    let ciphertext = hsm.encrypt("recovery-test-key", b"test data").await?;
    let decrypted = hsm.decrypt("recovery-test-key", &ciphertext).await?;
    assert_eq!(decrypted, b"test data");

    Ok(())
}

/// Test invalid ciphertext handling
#[tokio::test]
async fn test_invalid_ciphertext_handling() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Generate key
    let request = GenerateKeyRequest {
        key_type: KeyType::Aes,
        key_id: "invalid-ct-key".to_string(),
    };
    hsm.generate_key(request).await?;

    // Try to decrypt invalid ciphertext
    let invalid_ciphertext = b"this is not valid ciphertext";
    let result = hsm.decrypt("invalid-ct-key", invalid_ciphertext).await;

    // Should fail with authentication/decryption error
    assert!(result.is_err());

    // HSM should still be healthy
    let health = hsm.health_check().await?;
    assert!(health.is_healthy);

    Ok(())
}
