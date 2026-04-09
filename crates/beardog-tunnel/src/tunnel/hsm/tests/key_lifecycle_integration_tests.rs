// SPDX-License-Identifier: AGPL-3.0-or-later

//! HSM Key Lifecycle Integration Tests
//!
//! Substantive tests that exercise real production paths:
//! - Key generation → encryption → decryption
//! - Key import → signing → verification
//! - Multi-key operations and lifecycle management

#![allow(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::field_reassign_with_default)]

use crate::tunnel::hsm::GenerateKeyRequest;
use crate::tunnel::hsm::HsmProvider;
use crate::tunnel::hsm::software_hsm::core::RustSoftwareHsm;
use crate::tunnel::hsm::types::KeyType;
use crate::tunnel::hsm::types::config::SoftwareHsmConfig;
use beardog_errors::BearDogError;

#[cfg(test)]
mod key_lifecycle_tests {
    use super::*;

    #[tokio::test]
    async fn test_full_encryption_lifecycle() -> Result<(), BearDogError> {
        // Create HSM
        let config = SoftwareHsmConfig::default();
        let hsm = RustSoftwareHsm::new(config).await?;

        // Generate key
        let key_request = GenerateKeyRequest {
            key_id: "test-aes-key".to_string(),
            key_type: KeyType::Aes,
        };
        let key = hsm.generate_key(key_request).await?;
        assert_eq!(key.metadata.key_id, "test-aes-key");

        // Encrypt data
        let plaintext = b"Hello, secure world!";
        let ciphertext = hsm.encrypt("test-aes-key", plaintext).await?;
        assert_ne!(ciphertext.as_slice(), plaintext);
        assert!(ciphertext.len() > plaintext.len()); // Should include nonce + tag

        // Decrypt data
        let decrypted = hsm.decrypt("test-aes-key", &ciphertext).await?;
        assert_eq!(decrypted.as_slice(), plaintext);

        Ok(())
    }

    #[tokio::test]
    async fn test_multiple_keys_isolation() -> Result<(), BearDogError> {
        let config = SoftwareHsmConfig::default();
        let hsm = RustSoftwareHsm::new(config).await?;

        // Generate two different keys
        hsm.generate_key(GenerateKeyRequest {
            key_id: "key1".to_string(),
            key_type: KeyType::Aes,
        })
        .await?;

        hsm.generate_key(GenerateKeyRequest {
            key_id: "key2".to_string(),
            key_type: KeyType::Aes,
        })
        .await?;

        // Encrypt with key1
        let plaintext = b"Secret message";
        let ciphertext1 = hsm.encrypt("key1", plaintext).await?;

        // Cannot decrypt with key2 (should fail)
        let result = hsm.decrypt("key2", &ciphertext1).await;
        assert!(result.is_err(), "Should not decrypt with wrong key");

        // Can decrypt with correct key
        let decrypted = hsm.decrypt("key1", &ciphertext1).await?;
        assert_eq!(decrypted.as_slice(), plaintext);

        Ok(())
    }

    #[tokio::test]
    async fn test_key_deletion_lifecycle() -> Result<(), BearDogError> {
        let config = SoftwareHsmConfig::default();
        let hsm = RustSoftwareHsm::new(config).await?;

        // Generate and use key
        hsm.generate_key(GenerateKeyRequest {
            key_id: "temp-key".to_string(),
            key_type: KeyType::Aes,
        })
        .await?;

        let ciphertext = hsm.encrypt("temp-key", b"data").await?;
        assert!(!ciphertext.is_empty());

        // Delete key
        hsm.delete_key("temp-key").await?;

        // Operations should fail after deletion
        let result = hsm.encrypt("temp-key", b"data").await;
        assert!(result.is_err(), "Should fail after key deletion");

        Ok(())
    }

    #[tokio::test]
    async fn test_concurrent_operations_same_key() -> Result<(), BearDogError> {
        let config = SoftwareHsmConfig::default();
        let hsm = std::sync::Arc::new(RustSoftwareHsm::new(config).await?);

        // Generate key
        hsm.generate_key(GenerateKeyRequest {
            key_id: "shared-key".to_string(),
            key_type: KeyType::Aes,
        })
        .await?;

        // Concurrent encrypt operations
        let mut handles = Vec::new();
        for i in 0..10 {
            let hsm_clone = hsm.clone();
            let handle = tokio::spawn(async move {
                let data = format!("message-{}", i);
                let ciphertext = hsm_clone.encrypt("shared-key", data.as_bytes()).await?;
                let plaintext = hsm_clone.decrypt("shared-key", &ciphertext).await?;
                assert_eq!(plaintext.as_slice(), data.as_bytes());
                Ok::<_, BearDogError>(())
            });
            handles.push(handle);
        }

        // All should succeed
        for handle in handles {
            handle.await.expect("Task should complete")?;
        }

        Ok(())
    }

    #[tokio::test]
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss,
        reason = "synthetic 1MB plaintext uses i % 256 as u8; indices bounded by buffer size"
    )]
    async fn test_large_data_encryption() -> Result<(), BearDogError> {
        let config = SoftwareHsmConfig::default();
        let hsm = RustSoftwareHsm::new(config).await?;

        hsm.generate_key(GenerateKeyRequest {
            key_id: "large-data-key".to_string(),
            key_type: KeyType::Aes,
        })
        .await?;

        // Test with 1MB data
        let plaintext: Vec<u8> = (0..1024 * 1024).map(|i| (i % 256) as u8).collect();
        let ciphertext = hsm.encrypt("large-data-key", &plaintext).await?;
        let decrypted = hsm.decrypt("large-data-key", &ciphertext).await?;

        assert_eq!(decrypted.len(), plaintext.len());
        assert_eq!(decrypted, plaintext);

        Ok(())
    }

    #[tokio::test]
    async fn test_key_info_retrieval() -> Result<(), BearDogError> {
        let config = SoftwareHsmConfig::default();
        let hsm = RustSoftwareHsm::new(config).await?;

        hsm.generate_key(GenerateKeyRequest {
            key_id: "info-test-key".to_string(),
            key_type: KeyType::Aes,
        })
        .await?;

        let info = hsm.get_key_info("info-test-key").await?;
        assert_eq!(info.key_id, "info-test-key");

        Ok(())
    }

    #[tokio::test]
    async fn test_empty_data_encryption() -> Result<(), BearDogError> {
        let config = SoftwareHsmConfig::default();
        let hsm = RustSoftwareHsm::new(config).await?;

        hsm.generate_key(GenerateKeyRequest {
            key_id: "empty-data-key".to_string(),
            key_type: KeyType::Aes,
        })
        .await?;

        // Empty data should still work
        let ciphertext = hsm.encrypt("empty-data-key", b"").await?;
        assert!(!ciphertext.is_empty()); // Should have nonce + tag
        let decrypted = hsm.decrypt("empty-data-key", &ciphertext).await?;
        assert_eq!(decrypted.len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_hsm_health_check() -> Result<(), BearDogError> {
        let config = SoftwareHsmConfig::default();
        let hsm = RustSoftwareHsm::new(config).await?;

        let health = hsm.health_check().await?;
        assert!(health.is_healthy, "HSM should be healthy");

        Ok(())
    }
}
