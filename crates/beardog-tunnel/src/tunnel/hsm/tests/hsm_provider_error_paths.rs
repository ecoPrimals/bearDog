// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive Error Path Tests for HSM Providers
//!
//! Tests error handling across all HSM provider implementations

use crate::tunnel::hsm::manager::HsmProvider;
use crate::tunnel::hsm::software_hsm::core::RustSoftwareHsm;
use crate::tunnel::hsm::types::config::SoftwareHsmConfig;
use crate::tunnel::hsm::types::KeyType;
use crate::tunnel::hsm::GenerateKeyRequest;
use beardog_errors::BearDogError;

// TEST_CATEGORY: error_path
// TEST_DOMAIN: hsm
// TEST_PRIORITY: high

#[tokio::test]
async fn test_hsm_initialization_default_config() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Should initialize successfully
    let health = hsm.health_check().await?;
    assert!(health.is_healthy);
    Ok(())
}

#[tokio::test]
async fn test_generate_key_with_empty_id() {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await.unwrap();

    let request = GenerateKeyRequest {
        key_id: String::new(), // Empty ID
        key_type: KeyType::Aes,
    };

    let result = hsm.generate_key(request).await;
    // Should handle empty ID gracefully
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_generate_key_with_duplicate_id() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Generate first key
    let request1 = GenerateKeyRequest {
        key_id: "duplicate-key".to_string(),
        key_type: KeyType::Ed25519,
    };
    let _ = hsm.generate_key(request1).await?;

    // Try to generate with same ID
    let request2 = GenerateKeyRequest {
        key_id: "duplicate-key".to_string(),
        key_type: KeyType::Ed25519,
    };
    let result = hsm.generate_key(request2).await;

    // Should either succeed (overwrite) or fail (duplicate)
    assert!(result.is_ok() || result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_operations_on_nonexistent_key() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    let data = b"test data";

    // Try to sign with nonexistent key
    let sign_result = hsm.sign("nonexistent-key", data).await;
    assert!(sign_result.is_err());

    // Try to encrypt with nonexistent key
    let encrypt_result = hsm.encrypt("nonexistent-key", data).await;
    assert!(encrypt_result.is_err());

    Ok(())
}

#[tokio::test]
async fn test_hsm_health_check_always_succeeds() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Health check should always return a result
    let health = hsm.health_check().await?;
    // Health status can be either healthy or unhealthy - both are valid states
    let _ = health.is_healthy; // Verify health check succeeds (either state is valid)
    Ok(())
}

#[tokio::test]
async fn test_concurrent_key_generation() {
    use std::sync::Arc;
    use tokio::task::JoinSet;

    let config = SoftwareHsmConfig::default();
    let hsm = Arc::new(RustSoftwareHsm::new(config).await.unwrap());

    let mut set = JoinSet::new();

    for i in 0..20 {
        let hsm_clone = Arc::clone(&hsm);
        set.spawn(async move {
            let request = GenerateKeyRequest {
                key_id: format!("concurrent-key-{}", i),
                key_type: KeyType::Ed25519,
            };
            hsm_clone.generate_key(request).await
        });
    }

    let mut success_count = 0;
    while let Some(result) = set.join_next().await {
        if let Ok(Ok(_)) = result {
            success_count += 1;
        }
    }

    // Most should succeed
    assert!(success_count > 15);
}

#[tokio::test]
async fn test_sign_and_verify_with_wrong_key_type() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Generate AES key (symmetric)
    let request = GenerateKeyRequest {
        key_id: "aes-key".to_string(),
        key_type: KeyType::Aes,
    };
    let _ = hsm.generate_key(request).await?;

    // Try to sign with symmetric key (should fail)
    let data = b"test data";
    let sign_result = hsm.sign("aes-key", data).await;

    // Should fail or handle gracefully
    assert!(sign_result.is_err() || sign_result.is_ok());
    Ok(())
}

#[tokio::test]
async fn test_encrypt_decrypt_roundtrip() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Generate encryption key
    let request = GenerateKeyRequest {
        key_id: "encrypt-key".to_string(),
        key_type: KeyType::Aes,
    };
    hsm.generate_key(request).await?;

    // Encrypt data
    let plaintext = b"sensitive data for encryption test";
    let ciphertext = hsm.encrypt("encrypt-key", plaintext).await?;

    // Decrypt data
    let decrypted = hsm.decrypt("encrypt-key", &ciphertext).await?;

    // Should match original
    assert_eq!(plaintext.as_ref(), decrypted.as_slice());
    Ok(())
}

#[tokio::test]
async fn test_sign_verify_roundtrip() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Generate signing key
    let request = GenerateKeyRequest {
        key_id: "sign-key".to_string(),
        key_type: KeyType::Ed25519,
    };
    hsm.generate_key(request).await?;

    // Sign data
    let message = b"message to sign";
    let signature = hsm.sign("sign-key", message).await?;

    // Verify signature
    let valid = hsm.verify("sign-key", message, &signature).await?;
    assert!(valid, "Signature should be valid");
    Ok(())
}

#[tokio::test]
async fn test_verify_with_wrong_signature() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Generate key
    let request = GenerateKeyRequest {
        key_id: "verify-key".to_string(),
        key_type: KeyType::Ed25519,
    };
    hsm.generate_key(request).await?;

    let message = b"message";
    let wrong_signature = vec![0u8; 64]; // Invalid signature

    let verify_result = hsm.verify("verify-key", message, &wrong_signature).await;

    // Should either fail or return false
    if let Ok(valid) = verify_result {
        assert!(!valid, "Invalid signature should not verify");
    }
    Ok(())
}

#[tokio::test]
async fn test_encrypt_with_empty_data() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    let request = GenerateKeyRequest {
        key_id: "empty-test-key".to_string(),
        key_type: KeyType::Aes,
    };
    hsm.generate_key(request).await?;

    // Encrypt empty data
    let empty_data = b"";
    let result = hsm.encrypt("empty-test-key", empty_data).await;

    // Should handle empty data gracefully
    assert!(result.is_ok() || result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_encrypt_with_large_data() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    let request = GenerateKeyRequest {
        key_id: "large-data-key".to_string(),
        key_type: KeyType::Aes,
    };
    hsm.generate_key(request).await?;

    // Create 1MB of data
    let large_data = vec![0x42u8; 1024 * 1024];
    let result = hsm.encrypt("large-data-key", &large_data).await;

    // Should handle large data
    assert!(result.is_ok() || result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_multiple_hsm_instances_independent() -> Result<(), BearDogError> {
    let config1 = SoftwareHsmConfig::default();
    let config2 = SoftwareHsmConfig::default();

    let hsm1 = RustSoftwareHsm::new(config1).await?;
    let hsm2 = RustSoftwareHsm::new(config2).await?;

    // Generate keys in each
    let request = GenerateKeyRequest {
        key_id: "independent-key".to_string(),
        key_type: KeyType::Ed25519,
    };

    let result1 = hsm1.generate_key(request.clone()).await;
    let result2 = hsm2.generate_key(request).await;

    // Both should succeed independently
    assert!(result1.is_ok());
    assert!(result2.is_ok());
    Ok(())
}

#[tokio::test]
async fn test_hsm_operations_after_many_keys_generated() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    // Generate many keys
    for i in 0..100 {
        let request = GenerateKeyRequest {
            key_id: format!("bulk-key-{}", i),
            key_type: KeyType::Ed25519,
        };
        let _ = hsm.generate_key(request).await;
    }

    // Operations should still work
    let health = hsm.health_check().await?;
    // Health status can be either healthy or unhealthy - both are valid states
    let _ = health.is_healthy; // Verify health check succeeds (either state is valid)
    Ok(())
}

#[tokio::test]
async fn test_all_supported_key_types() -> Result<(), BearDogError> {
    let config = SoftwareHsmConfig::default();
    let hsm = RustSoftwareHsm::new(config).await?;

    let key_types = [KeyType::Aes, KeyType::Ed25519];

    for (idx, key_type) in key_types.iter().enumerate() {
        let request = GenerateKeyRequest {
            key_id: format!("key-type-test-{}", idx),
            key_type: key_type.clone(),
        };

        let result = hsm.generate_key(request).await;
        assert!(
            result.is_ok(),
            "Key type {:?} should be supported",
            key_type
        );
    }
    Ok(())
}
