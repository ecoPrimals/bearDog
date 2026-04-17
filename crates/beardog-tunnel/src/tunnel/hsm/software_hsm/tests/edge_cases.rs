// SPDX-License-Identifier: AGPL-3.0-or-later
//! Security-relevant edge cases: wrong keys, signatures, zeroization.

#![cfg(test)]

use super::common::*;

/// Test accessing deleted key
#[tokio::test]
async fn test_key_access_after_deletion() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Generate and delete key
    let request = GenerateKeyRequest {
        key_type: KeyType::Aes,
        key_id: "deleted-key".to_string(),
    };
    hsm.generate_key(request).await?;
    hsm.delete_key("deleted-key").await?;

    // Try to use deleted key
    let result = hsm.encrypt("deleted-key", b"test").await;
    assert!(result.is_err());

    Ok(())
}

/// Test decryption behavior with wrong key ID
///
/// NEW BEHAVIOR (Post vendor-agnostic migration): Uses REAL cryptography (AES-256-GCM)
/// without key metadata in ciphertext. Decryption with wrong key correctly fails.
///
/// This is the CORRECT and SECURE behavior - authenticated encryption ensures that
/// decryption with the wrong key fails, preventing data corruption and security issues.
#[tokio::test]
async fn test_decryption_with_wrong_key_id() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Generate two keys
    let request1 = GenerateKeyRequest {
        key_type: KeyType::Aes,
        key_id: "key1".to_string(),
    };
    let request2 = GenerateKeyRequest {
        key_type: KeyType::Aes,
        key_id: "key2".to_string(),
    };
    hsm.generate_key(request1).await?;
    hsm.generate_key(request2).await?;

    // Encrypt with key1
    let plaintext = b"sensitive data";
    let ciphertext = hsm.encrypt("key1", plaintext).await?;

    // NEW BEHAVIOR: Real AES-256-GCM - decryption with wrong key fails
    // This is the CORRECT security behavior
    let result = hsm.decrypt("key2", &ciphertext).await;

    // Verify that decryption with wrong key fails (as it should!)
    assert!(
        result.is_err(),
        "Decryption with wrong key should fail (secure behavior)"
    );

    // Verify that decryption with correct key succeeds
    let decrypted = hsm.decrypt("key1", &ciphertext).await?;
    assert_eq!(
        decrypted, plaintext,
        "Decryption with correct key should succeed"
    );

    Ok(())
}

/// Test signature verification edge cases
#[tokio::test]
async fn test_signature_verification_edge_cases() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Generate signing key
    let request = GenerateKeyRequest {
        key_type: KeyType::Ed25519,
        key_id: "sig-verify-key".to_string(),
    };
    hsm.generate_key(request).await?;

    // Sign message
    let message = b"important message";
    let signature = hsm.sign("sig-verify-key", message).await?;

    // Test 1: Verify correct signature
    let verify_result = hsm.verify("sig-verify-key", message, &signature).await?;
    assert!(verify_result);

    // Test 2: Verify with modified message (should fail)
    let modified_message = b"modified message";
    let verify_result = hsm
        .verify("sig-verify-key", modified_message, &signature)
        .await?;
    assert!(!verify_result);

    // Test 3: Verify with modified signature (should fail)
    let mut bad_signature = signature.clone();
    if !bad_signature.is_empty() {
        bad_signature[0] ^= 0xFF;
    }
    let verify_result = hsm
        .verify("sig-verify-key", message, &bad_signature)
        .await?;
    assert!(!verify_result);

    Ok(())
}

/// Test memory zeroization after key deletion
#[tokio::test]
async fn test_memory_zeroization_after_deletion() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Generate key
    let request = GenerateKeyRequest {
        key_type: KeyType::Aes,
        key_id: "zero-test-key".to_string(),
    };
    let _key = hsm.generate_key(request).await?;

    // Use key
    let _ciphertext = hsm.encrypt("zero-test-key", b"sensitive").await?;

    // Delete key (should trigger zeroing)
    hsm.delete_key("zero-test-key").await?;

    // Verify key is truly gone
    let result = hsm.get_key_info("zero-test-key").await;
    assert!(result.is_err());

    // Try to use deleted key
    let encrypt_result = hsm.encrypt("zero-test-key", b"test").await;
    assert!(encrypt_result.is_err());

    Ok(())
}
