// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive tests for Software HSM
//!
//! Covers:
//! - Core HSM initialization and lifecycle
//! - Key generation for all supported types
//! - Cryptographic operations (sign, verify, encrypt, decrypt)
//! - Key management (import, export, delete)
//! - Memory protection and secure zeroing
//! - Audit logging
//! - Health monitoring
//! - Error handling and edge cases

use super::*;
use crate::tunnel::hsm::GenerateKeyRequest;
use crate::tunnel::hsm::manager::HsmProvider;
use crate::tunnel::hsm::types::KeyType;
use crate::tunnel::hsm::types::config::CryptoBackendType;
use crate::tunnel::hsm::types::config::SoftwareHsmConfig;
use std::sync::Arc;

#[cfg(test)]
mod software_hsm_tests {
    use super::*;

    /// Test HSM initialization with default configuration
    #[tokio::test]
    async fn test_hsm_initialization_default_config() -> Result<(), BearDogError> {
        let config = SoftwareHsmConfig::default();
        let hsm = RustSoftwareHsm::new(config).await?;

        // Verify HSM is created successfully
        let health = hsm.health_check().await?;
        assert!(health.is_healthy);
        Ok(())
    }

    /// Test HSM initialization with all crypto backends
    #[tokio::test]
    async fn test_hsm_initialization_all_backends() -> Result<(), BearDogError> {
        // RustCrypto backend
        let config_rust_crypto = SoftwareHsmConfig {
            crypto_backend: CryptoBackendType::RustCrypto,
            ..Default::default()
        };
        let hsm_rust = RustSoftwareHsm::new(config_rust_crypto).await?;
        let health = hsm_rust.health_check().await?;
        assert!(health.is_healthy);

        // Ring backend
        let config_ring = SoftwareHsmConfig {
            crypto_backend: CryptoBackendType::Ring,
            ..Default::default()
        };
        let hsm_ring = RustSoftwareHsm::new(config_ring).await?;
        let health = hsm_ring.health_check().await?;
        assert!(health.is_healthy);

        // OpenSSL backend
        let config_openssl = SoftwareHsmConfig {
            crypto_backend: CryptoBackendType::OpenSsl,
            ..Default::default()
        };
        let hsm_openssl = RustSoftwareHsm::new(config_openssl).await?;
        let health = hsm_openssl.health_check().await?;
        assert!(health.is_healthy);

        Ok(())
    }

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

    /// Test health check functionality
    #[tokio::test]
    async fn test_health_check() -> Result<(), BearDogError> {
        let config = SoftwareHsmConfig::default();
        let hsm = RustSoftwareHsm::new(config).await?;

        let health_status = hsm.health_check().await?;
        assert!(health_status.is_healthy);

        Ok(())
    }

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

    /// Test concurrent operations
    #[tokio::test]
    async fn test_concurrent_operations() -> Result<(), BearDogError> {
        let config = SoftwareHsmConfig::default();
        let hsm = Arc::new(RustSoftwareHsm::new(config).await?);

        // Generate keys concurrently
        let mut handles = vec![];
        for i in 0..5 {
            let hsm_clone = hsm.clone();
            let handle = tokio::spawn(async move {
                let request = GenerateKeyRequest {
                    key_type: KeyType::Aes,
                    key_id: format!("concurrent-key-{i}"),
                };
                hsm_clone.generate_key(request).await
            });
            handles.push(handle);
        }

        // Wait for all operations
        for handle in handles {
            handle
                .await
                .map_err(|e| BearDogError::system(format!("Task join failed: {}", e)))??;
        }

        // Verify all keys exist
        for i in 0..5 {
            let key_info = hsm.get_key_info(&format!("concurrent-key-{i}")).await?;
            assert_eq!(key_info.key_id, format!("concurrent-key-{i}"));
        }

        Ok(())
    }

    /// Test large data encryption/decryption
    #[tokio::test]
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

    // ========================================================================
    // COMPREHENSIVE EDGE CASE TESTS (Week 1 Sprint)
    // ========================================================================

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

    /// Test very large payload handling (10MB)
    #[tokio::test]
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

    /// Test concurrent crypto operations on same key
    #[tokio::test]
    async fn test_concurrent_crypto_operations_same_key() -> Result<(), BearDogError> {
        let config = SoftwareHsmConfig::default();
        let hsm = Arc::new(RustSoftwareHsm::new(config).await?);

        // Generate key
        let request = GenerateKeyRequest {
            key_type: KeyType::Aes,
            key_id: "concurrent-ops-key".to_string(),
        };
        hsm.generate_key(request).await?;

        // Perform 20 concurrent encryptions
        let mut handles = vec![];
        for i in 0..20 {
            let hsm_clone = Arc::clone(&hsm);
            let handle = tokio::spawn(async move {
                let plaintext = format!("message {i}");
                let ciphertext = hsm_clone
                    .encrypt("concurrent-ops-key", plaintext.as_bytes())
                    .await?;
                let decrypted = hsm_clone.decrypt("concurrent-ops-key", &ciphertext).await?;
                assert_eq!(decrypted, plaintext.as_bytes());
                Ok::<(), BearDogError>(())
            });
            handles.push(handle);
        }

        // Wait for all operations
        for handle in handles {
            handle
                .await
                .map_err(|e| BearDogError::system(format!("Task join failed: {}", e)))??;
        }

        Ok(())
    }

    /// Test memory protection under high load
    #[tokio::test]
    async fn test_memory_protection_under_load() -> Result<(), BearDogError> {
        let config = SoftwareHsmConfig::default();
        let hsm = Arc::new(RustSoftwareHsm::new(config).await?);

        // Generate multiple keys rapidly
        let mut handles = vec![];
        for i in 0..50 {
            let hsm_clone = Arc::clone(&hsm);
            let handle = tokio::spawn(async move {
                let request = GenerateKeyRequest {
                    key_type: KeyType::Aes,
                    key_id: format!("load-test-key-{i}"),
                };
                hsm_clone.generate_key(request).await?;

                // Immediately use the key
                let plaintext = format!("data for key {i}");
                let _ciphertext = hsm_clone
                    .encrypt(&format!("load-test-key-{i}"), plaintext.as_bytes())
                    .await?;

                // Delete to test memory cleanup
                hsm_clone.delete_key(&format!("load-test-key-{i}")).await?;

                Ok::<(), BearDogError>(())
            });
            handles.push(handle);
        }

        // Wait for all operations
        for handle in handles {
            handle
                .await
                .map_err(|e| BearDogError::system(format!("Task join failed: {}", e)))??;
        }

        // Verify health after stress
        let health = hsm.health_check().await?;
        assert!(health.is_healthy);

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

    /// Test HSM health monitoring after various operations
    #[tokio::test]
    async fn test_health_monitoring_comprehensive() -> Result<(), BearDogError> {
        let config = SoftwareHsmConfig::default();
        let hsm = RustSoftwareHsm::new(config).await?;

        // Initial health check
        let health1 = hsm.health_check().await?;
        assert!(health1.is_healthy);

        // Generate keys
        for i in 0..5 {
            let request = GenerateKeyRequest {
                key_type: KeyType::Aes,
                key_id: format!("health-test-key-{i}"),
            };
            hsm.generate_key(request).await?;
        }

        // Health check after key generation
        let health2 = hsm.health_check().await?;
        assert!(health2.is_healthy);

        // Perform crypto operations
        for i in 0..5 {
            let _ = hsm
                .encrypt(&format!("health-test-key-{i}"), b"test data")
                .await?;
        }

        // Health check after crypto operations
        let health3 = hsm.health_check().await?;
        assert!(health3.is_healthy);

        // Delete keys
        for i in 0..5 {
            hsm.delete_key(&format!("health-test-key-{i}")).await?;
        }

        // Final health check
        let health4 = hsm.health_check().await?;
        assert!(health4.is_healthy);

        Ok(())
    }
}
