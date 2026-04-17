// SPDX-License-Identifier: AGPL-3.0-or-later
//! Key deletion, batch operations, and rotation scenarios.

#![cfg(test)]

use super::common::*;

/// Test key deletion
#[tokio::test]
async fn test_delete_key() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Generate key
    let request = GenerateKeyRequest {
        key_type: KeyType::Aes,
        key_id: "key-to-delete".to_string(),
    };
    hsm.generate_key(request).await?;

    // Verify key exists
    let key_info = hsm.get_key_info("key-to-delete").await?;
    assert_eq!(key_info.key_id, "key-to-delete");

    // Delete key
    hsm.delete_key("key-to-delete").await?;

    // Verify key is gone
    let result = hsm.get_key_info("key-to-delete").await;
    assert!(result.is_err());

    Ok(())
}

/// Test multiple keys management
#[tokio::test]
async fn test_multiple_keys_management() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Generate multiple keys
    for i in 0..10 {
        let request = GenerateKeyRequest {
            key_type: KeyType::Aes,
            key_id: format!("test-key-{i}"),
        };
        hsm.generate_key(request).await?;
    }

    // Verify all keys exist
    for i in 0..10 {
        let key_info = hsm.get_key_info(&format!("test-key-{i}")).await?;
        assert_eq!(key_info.key_id, format!("test-key-{i}"));
    }

    // Delete all keys
    for i in 0..10 {
        hsm.delete_key(&format!("test-key-{i}")).await?;
    }

    // Verify all keys are gone
    for i in 0..10 {
        let result = hsm.get_key_info(&format!("test-key-{i}")).await;
        assert!(result.is_err());
    }

    Ok(())
}

/// Test key rotation during active operation
#[tokio::test]
async fn test_key_rotation_during_operation() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Generate initial key
    let request = GenerateKeyRequest {
        key_type: KeyType::Aes,
        key_id: "rotation-test-key".to_string(),
    };
    hsm.generate_key(request).await?;

    // Start encryption operation
    let plaintext = b"test data during rotation";
    let ciphertext = hsm.encrypt("rotation-test-key", plaintext).await?;

    // Simulate key rotation by generating a new key with same ID
    // (Note: Real rotation would involve versioning)
    let request2 = GenerateKeyRequest {
        key_type: KeyType::Aes,
        key_id: "rotation-test-key-v2".to_string(),
    };
    hsm.generate_key(request2).await?;

    // Original key should still work for decryption
    let decrypted = hsm.decrypt("rotation-test-key", &ciphertext).await?;
    assert_eq!(decrypted, plaintext);

    Ok(())
}

/// Test key deletion with pending operations
#[tokio::test]
async fn test_key_deletion_immediate() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Generate key
    let request = GenerateKeyRequest {
        key_type: KeyType::Aes,
        key_id: "delete-test-key".to_string(),
    };
    hsm.generate_key(request).await?;

    // Encrypt data
    let plaintext = b"data before deletion";
    let _ciphertext = hsm.encrypt("delete-test-key", plaintext).await?;

    // Delete key immediately
    hsm.delete_key("delete-test-key").await?;

    // Verify key is gone
    let result = hsm.get_key_info("delete-test-key").await;
    assert!(result.is_err());

    Ok(())
}

/// Test batch key operations
#[tokio::test]
async fn test_batch_key_operations() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Generate 20 keys rapidly
    for i in 0..20 {
        let request = GenerateKeyRequest {
            key_type: KeyType::Aes,
            key_id: format!("batch-key-{i}"),
        };
        hsm.generate_key(request).await?;
    }

    // Verify all keys exist
    for i in 0..20 {
        let key_info = hsm.get_key_info(&format!("batch-key-{i}")).await?;
        assert_eq!(key_info.key_id, format!("batch-key-{i}"));
    }

    // Delete all keys in batch
    for i in 0..20 {
        hsm.delete_key(&format!("batch-key-{i}")).await?;
    }

    // Verify all keys deleted
    for i in 0..20 {
        let result = hsm.get_key_info(&format!("batch-key-{i}")).await;
        assert!(result.is_err());
    }

    Ok(())
}
