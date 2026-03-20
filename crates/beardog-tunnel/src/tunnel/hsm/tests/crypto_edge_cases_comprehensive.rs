// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive Crypto Edge Case Tests
//!
//! Deep testing of edge cases and error paths for cryptographic operations.
//! Targets coverage gaps in beardog-tunnel (70% → 90%).
//!
//! TEST_CATEGORY: edge_case
//! TEST_DOMAIN: crypto
//! TEST_PRIORITY: high

use crate::tunnel::hsm::GenerateKeyRequest;
use crate::tunnel::hsm::manager::HsmProvider;
use crate::tunnel::hsm::software_hsm::core::RustSoftwareHsm;
use crate::tunnel::hsm::types::KeyType;
use crate::tunnel::hsm::types::config::SoftwareHsmConfig;
use beardog_errors::BearDogError;
use std::sync::Arc;

// ============================================================================
// Edge Case: Empty and Boundary Inputs
// ============================================================================

#[tokio::test]
async fn test_encrypt_decrypt_empty_data() -> Result<(), BearDogError> {
    let hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default()).await?;

    // Generate key
    let request = GenerateKeyRequest {
        key_id: "empty-data-key".to_string(),
        key_type: KeyType::Aes,
    };
    hsm.generate_key(request).await?;

    // Encrypt empty data
    let empty_data = vec![];
    let encrypted = hsm.encrypt("empty-data-key", &empty_data).await;

    // Should either succeed (some algorithms allow) or fail gracefully
    match encrypted {
        Ok(ciphertext) => {
            // If it succeeds, decrypt should return empty data
            let decrypted = hsm.decrypt("empty-data-key", &ciphertext).await?;
            assert_eq!(
                decrypted, empty_data,
                "Empty data should round-trip correctly"
            );
        }
        Err(e) => {
            // Graceful failure is acceptable
            assert!(
                e.to_string().contains("empty") || e.to_string().contains("invalid"),
                "Error should indicate empty data issue"
            );
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_sign_verify_empty_message() -> Result<(), BearDogError> {
    let hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default()).await?;

    // Generate signing key
    let request = GenerateKeyRequest {
        key_id: "empty-sign-key".to_string(),
        key_type: KeyType::Ed25519,
    };
    hsm.generate_key(request).await?;

    // Sign empty message
    let empty_message = vec![];
    let signature = hsm.sign("empty-sign-key", &empty_message).await;

    match signature {
        Ok(sig) => {
            // If signing succeeds, verify should also work
            let verified = hsm.verify("empty-sign-key", &empty_message, &sig).await?;
            assert!(verified, "Empty message signature should verify");
        }
        Err(e) => {
            // Graceful failure acceptable
            println!("Empty message signing failed (acceptable): {}", e);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_very_long_key_id() -> Result<(), BearDogError> {
    let hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default()).await?;

    // Test with very long key ID (1000 characters)
    let long_id = "a".repeat(1000);

    let request = GenerateKeyRequest {
        key_id: long_id.clone(),
        key_type: KeyType::Aes,
    };

    let result = hsm.generate_key(request).await;

    // Should either accept or reject gracefully
    if let Err(e) = result {
        assert!(
            e.to_string().contains("length")
                || e.to_string().contains("long")
                || e.to_string().contains("invalid"),
            "Error should indicate ID length issue"
        );
    }

    Ok(())
}

// ============================================================================
// Edge Case: Concurrent Operations on Same Key
// ============================================================================

#[tokio::test]
async fn test_concurrent_encrypt_same_key() -> Result<(), BearDogError> {
    let hsm = Arc::new(RustSoftwareHsm::new(SoftwareHsmConfig::default()).await?);

    // Generate shared key
    let request = GenerateKeyRequest {
        key_id: "concurrent-key".to_string(),
        key_type: KeyType::Aes,
    };
    hsm.generate_key(request).await?;

    let mut handles = vec![];

    // 50 concurrent encryptions using the same key
    for i in 0..50 {
        let hsm_clone = Arc::clone(&hsm);
        let handle = tokio::spawn(async move {
            let plaintext = format!("message_{}", i);
            hsm_clone
                .encrypt("concurrent-key", plaintext.as_bytes())
                .await
        });
        handles.push(handle);
    }

    // All should succeed
    let mut success_count = 0;
    for handle in handles {
        if handle.await.unwrap().is_ok() {
            success_count += 1;
        }
    }

    assert!(
        success_count >= 48,
        "At least 96% of concurrent operations should succeed (got {}/50)",
        success_count
    );

    Ok(())
}

#[tokio::test]
async fn test_interleaved_encrypt_decrypt() -> Result<(), BearDogError> {
    let hsm = Arc::new(RustSoftwareHsm::new(SoftwareHsmConfig::default()).await?);

    // Generate key
    let request = GenerateKeyRequest {
        key_id: "interleaved-key".to_string(),
        key_type: KeyType::Aes,
    };
    hsm.generate_key(request).await?;

    let mut handles = vec![];

    // Interleave encrypt and decrypt operations
    for i in 0..20 {
        let hsm_clone = Arc::clone(&hsm);

        if i % 2 == 0 {
            // Encrypt
            let handle = tokio::spawn(async move {
                let plaintext = format!("test_{}", i);
                hsm_clone
                    .encrypt("interleaved-key", plaintext.as_bytes())
                    .await
            });
            handles.push(handle);
        } else {
            // Decrypt (using pre-encrypted data)
            let handle = tokio::spawn(async move {
                let plaintext = b"test_data";
                let encrypted = hsm_clone.encrypt("interleaved-key", plaintext).await?;
                hsm_clone.decrypt("interleaved-key", &encrypted).await
            });
            handles.push(handle);
        }
    }

    // Most should succeed
    let mut success_count = 0;
    for handle in handles {
        if handle.await.unwrap().is_ok() {
            success_count += 1;
        }
    }

    assert!(success_count >= 18, "90%+ should succeed");

    Ok(())
}

// ============================================================================
// Edge Case: Operations on Nonexistent Keys
// ============================================================================

#[tokio::test]
async fn test_encrypt_nonexistent_key_error() -> Result<(), BearDogError> {
    let hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default()).await?;

    let result = hsm.encrypt("nonexistent-key", b"test data").await;

    assert!(result.is_err(), "Should fail with nonexistent key");

    let err = result.unwrap_err();
    let err_msg = err.to_string();
    assert!(
        err_msg.contains("not found") || err_msg.contains("does not exist"),
        "Error should indicate key not found: {}",
        err_msg
    );

    Ok(())
}

#[tokio::test]
async fn test_decrypt_nonexistent_key_error() -> Result<(), BearDogError> {
    let hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default()).await?;

    let result = hsm.decrypt("nonexistent-key", b"fake ciphertext").await;

    assert!(result.is_err(), "Should fail with nonexistent key");

    Ok(())
}

#[tokio::test]
async fn test_sign_nonexistent_key_error() -> Result<(), BearDogError> {
    let hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default()).await?;

    let result = hsm.sign("nonexistent-key", b"data to sign").await;

    assert!(result.is_err(), "Should fail with nonexistent key");

    Ok(())
}

#[tokio::test]
async fn test_delete_nonexistent_key() -> Result<(), BearDogError> {
    let hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default()).await?;

    let result = hsm.delete_key("nonexistent-key").await;

    // Should either succeed (idempotent) or return clear error
    if let Err(e) = result {
        assert!(
            e.to_string().contains("not found") || e.to_string().contains("does not exist"),
            "Error should indicate key not found"
        );
    }

    Ok(())
}

// ============================================================================
// Edge Case: Large Data Handling
// ============================================================================

#[tokio::test]
async fn test_encrypt_1mb_data() -> Result<(), BearDogError> {
    let hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default()).await?;

    // Generate key
    let request = GenerateKeyRequest {
        key_id: "large-data-key".to_string(),
        key_type: KeyType::Aes,
    };
    hsm.generate_key(request).await?;

    // Test with 1MB of data
    let large_data = vec![0xAB; 1_048_576];

    let encrypted = hsm.encrypt("large-data-key", &large_data).await?;
    let decrypted = hsm.decrypt("large-data-key", &encrypted).await?;

    assert_eq!(decrypted, large_data, "1MB data should decrypt correctly");

    Ok(())
}

#[tokio::test]
async fn test_sign_large_data() -> Result<(), BearDogError> {
    let hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default()).await?;

    // Generate signing key
    let request = GenerateKeyRequest {
        key_id: "large-sign-key".to_string(),
        key_type: KeyType::Ed25519,
    };
    hsm.generate_key(request).await?;

    // Sign 100KB of data
    let large_data = vec![0xCD; 102_400];

    let signature = hsm.sign("large-sign-key", &large_data).await?;
    let verified = hsm
        .verify("large-sign-key", &large_data, &signature)
        .await?;

    assert!(verified, "Large data signature should verify");

    Ok(())
}

// ============================================================================
// Edge Case: Rapid Key Lifecycle
// ============================================================================

#[tokio::test]
async fn test_rapid_key_creation_deletion() -> Result<(), BearDogError> {
    let hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default()).await?;

    for i in 0..10 {
        let key_id = format!("rapid-key-{}", i);

        // Create
        let request = GenerateKeyRequest {
            key_id: key_id.clone(),
            key_type: KeyType::Aes,
        };
        hsm.generate_key(request).await?;

        // Use immediately
        let encrypted = hsm.encrypt(&key_id, b"test").await?;
        let _decrypted = hsm.decrypt(&key_id, &encrypted).await?;

        // Delete immediately
        let _ = hsm.delete_key(&key_id).await;
    }

    Ok(())
}

#[tokio::test]
async fn test_use_after_delete() -> Result<(), BearDogError> {
    let hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default()).await?;

    // Create key
    let request = GenerateKeyRequest {
        key_id: "delete-test-key".to_string(),
        key_type: KeyType::Aes,
    };
    hsm.generate_key(request).await?;

    // Delete key
    let _ = hsm.delete_key("delete-test-key").await;

    // Try to use deleted key
    let result = hsm.encrypt("delete-test-key", b"test").await;

    assert!(result.is_err(), "Should fail to use deleted key");

    Ok(())
}

// ============================================================================
// Edge Case: Health Check Under Stress
// ============================================================================

#[tokio::test]
async fn test_health_check_during_operations() -> Result<(), BearDogError> {
    let hsm = Arc::new(RustSoftwareHsm::new(SoftwareHsmConfig::default()).await?);

    // Generate key
    let request = GenerateKeyRequest {
        key_id: "stress-key".to_string(),
        key_type: KeyType::Aes,
    };
    hsm.generate_key(request).await?;

    // Start background operations
    let mut bg_handles = vec![];
    for i in 0..20 {
        let hsm_clone = Arc::clone(&hsm);
        let handle = tokio::spawn(async move {
            let data = format!("data_{}", i);
            hsm_clone.encrypt("stress-key", data.as_bytes()).await
        });
        bg_handles.push(handle);
    }

    // Perform health checks while operations are running
    for _ in 0..5 {
        let health = hsm.health_check().await?;
        assert!(
            health.is_healthy,
            "HSM should remain healthy during operations"
        );
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }

    // Wait for background to complete
    for handle in bg_handles {
        let _ = handle.await;
    }

    // Final health check
    let health = hsm.health_check().await?;
    assert!(health.is_healthy, "HSM should be healthy after operations");

    Ok(())
}

// ============================================================================
// Edge Case: Multiple Key Types
// ============================================================================

#[tokio::test]
async fn test_multiple_key_types_simultaneously() -> Result<(), BearDogError> {
    let hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default()).await?;

    // Generate multiple key types
    let key_types = vec![
        ("aes-key", KeyType::Aes),
        ("ed25519-key", KeyType::Ed25519),
        ("chacha-key", KeyType::ChaCha20),
    ];

    for (key_id, key_type) in &key_types {
        let request = GenerateKeyRequest {
            key_id: key_id.to_string(),
            key_type: key_type.clone(),
        };
        hsm.generate_key(request).await?;
    }

    // Use all keys
    for (key_id, key_type) in &key_types {
        match key_type {
            KeyType::Aes | KeyType::ChaCha20 => {
                let encrypted = hsm.encrypt(key_id, b"test data").await?;
                let _decrypted = hsm.decrypt(key_id, &encrypted).await?;
            }
            KeyType::Ed25519 => {
                let signature = hsm.sign(key_id, b"test data").await?;
                let verified = hsm.verify(key_id, b"test data", &signature).await?;
                assert!(verified, "Signature should verify for {}", key_id);
            }
            _ => {}
        }
    }

    Ok(())
}

// ============================================================================
// Edge Case: Error Recovery
// ============================================================================

#[tokio::test]
async fn test_recovery_after_multiple_errors() -> Result<(), BearDogError> {
    let hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default()).await?;

    // Generate valid key
    let request = GenerateKeyRequest {
        key_id: "recovery-key".to_string(),
        key_type: KeyType::Aes,
    };
    hsm.generate_key(request).await?;

    // Cause 10 errors
    for _ in 0..10 {
        let _ = hsm.encrypt("nonexistent", b"data").await;
        let _ = hsm.decrypt("nonexistent", b"data").await;
    }

    // HSM should still work correctly
    let encrypted = hsm.encrypt("recovery-key", b"test data").await?;
    let decrypted = hsm.decrypt("recovery-key", &encrypted).await?;

    assert_eq!(decrypted, b"test data", "HSM should work after errors");

    Ok(())
}
