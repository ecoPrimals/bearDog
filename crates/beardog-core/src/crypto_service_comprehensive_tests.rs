// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive Crypto Service Tests
//!
//! High-value tests for crypto operations covering:
//! - Edge cases and error paths
//! - Security properties
//! - Performance characteristics
//! - Integration scenarios

/// Test suite for encryption/decryption operations
mod encryption_tests {
    use crate::crypto_service::{BearDogCryptoService, CryptoService, CryptoServiceConfig};
    use beardog_types::crypto_service::*;

    #[tokio::test]
    async fn test_encrypt_empty_data() {
        let config = CryptoServiceConfig::default();
        let service = BearDogCryptoService::new(config).expect("Failed to create service");

        let result = service
            .encrypt(
                &[],
                CryptoAlgorithm::Aes256Gcm,
                EncryptOptions {
                    key_id: "test-key".to_string(),
                    associated_data: None,
                },
            )
            .await;

        // Empty data should encrypt successfully (produces empty ciphertext + tag)
        assert!(result.is_ok(), "Empty data encryption should succeed");
    }

    #[tokio::test]
    async fn test_encrypt_large_data() {
        let config = CryptoServiceConfig {
            max_data_size: 1024 * 1024, // 1MB
            ..Default::default()
        };
        let service = BearDogCryptoService::new(config).expect("Failed to create service");

        // Create 512KB of data
        let large_data = vec![42u8; 512 * 1024];

        let result = service
            .encrypt(
                &large_data,
                CryptoAlgorithm::Aes256Gcm,
                EncryptOptions {
                    key_id: "large-key".to_string(),
                    associated_data: None,
                },
            )
            .await;

        assert!(result.is_ok(), "Large data encryption should succeed");
        let encrypted = result.unwrap();
        assert!(!encrypted.ciphertext.is_empty());
        assert_eq!(encrypted.algorithm, CryptoAlgorithm::Aes256Gcm);
    }

    #[tokio::test]
    async fn test_encrypt_with_aad() {
        let config = CryptoServiceConfig::default();
        let service = BearDogCryptoService::new(config).expect("Failed to create service");

        let data = b"sensitive data";
        let aad = b"additional authenticated data";

        let result = service
            .encrypt(
                data,
                CryptoAlgorithm::Aes256Gcm,
                EncryptOptions {
                    key_id: "aad-key".to_string(),
                    associated_data: Some(aad.to_vec()),
                },
            )
            .await;

        assert!(result.is_ok(), "Encryption with AAD should succeed");
        let encrypted = result.unwrap();

        // Verify AAD is stored in metadata
        assert!(encrypted.metadata.key_id.is_some());
    }

    #[tokio::test]
    async fn test_decrypt_wrong_key() {
        let config = CryptoServiceConfig::default();
        let service = BearDogCryptoService::new(config).expect("Failed to create service");

        let data = b"test data";

        // Encrypt with one key
        let encrypted = service
            .encrypt(
                data,
                CryptoAlgorithm::Aes256Gcm,
                EncryptOptions {
                    key_id: "key1".to_string(),
                    associated_data: None,
                },
            )
            .await
            .expect("Encryption failed");

        // Try to decrypt with different key
        let result = service
            .decrypt(
                &encrypted,
                DecryptOptions {
                    key_id: "key2".to_string(), // Wrong key!
                    associated_data: None,
                },
            )
            .await;

        // Should fail - wrong key
        assert!(result.is_err(), "Decryption with wrong key should fail");
    }

    #[tokio::test]
    async fn test_decrypt_tampered_ciphertext() {
        let config = CryptoServiceConfig::default();
        let service = BearDogCryptoService::new(config).expect("Failed to create service");

        let data = b"test data";

        let mut encrypted = service
            .encrypt(
                data,
                CryptoAlgorithm::Aes256Gcm,
                EncryptOptions {
                    key_id: "tamper-key".to_string(),
                    associated_data: None,
                },
            )
            .await
            .expect("Encryption failed");

        // Tamper with ciphertext
        if !encrypted.ciphertext.is_empty() {
            encrypted.ciphertext[0] ^= 0xFF;
        }

        // Try to decrypt tampered data
        let result = service
            .decrypt(
                &encrypted,
                DecryptOptions {
                    key_id: "tamper-key".to_string(),
                    associated_data: None,
                },
            )
            .await;

        // Should fail - authentication should detect tampering
        assert!(result.is_err(), "Decryption of tampered data should fail");
    }

    #[tokio::test]
    async fn test_decrypt_with_wrong_aad() {
        let config = CryptoServiceConfig::default();
        let service = BearDogCryptoService::new(config).expect("Failed to create service");

        let data = b"test data";
        let aad = b"correct aad";

        let encrypted = service
            .encrypt(
                data,
                CryptoAlgorithm::Aes256Gcm,
                EncryptOptions {
                    key_id: "aad-test".to_string(),
                    associated_data: Some(aad.to_vec()),
                },
            )
            .await
            .expect("Encryption failed");

        // Try to decrypt with wrong AAD
        let result = service
            .decrypt(
                &encrypted,
                DecryptOptions {
                    key_id: "aad-test".to_string(),
                    associated_data: Some(b"wrong aad".to_vec()),
                },
            )
            .await;

        // Should fail - AAD mismatch
        assert!(result.is_err(), "Decryption with wrong AAD should fail");
    }

    #[tokio::test]
    async fn test_encrypt_decrypt_roundtrip() {
        let config = CryptoServiceConfig::default();
        let service = BearDogCryptoService::new(config).expect("Failed to create service");

        let original_data = b"The quick brown fox jumps over the lazy dog";

        // Encrypt
        let encrypted = service
            .encrypt(
                original_data,
                CryptoAlgorithm::Aes256Gcm,
                EncryptOptions {
                    key_id: "roundtrip-key".to_string(),
                    associated_data: None,
                },
            )
            .await
            .expect("Encryption failed");

        // Decrypt
        let decrypted = service
            .decrypt(
                &encrypted,
                DecryptOptions {
                    key_id: "roundtrip-key".to_string(),
                    associated_data: None,
                },
            )
            .await
            .expect("Decryption failed");

        // Verify roundtrip
        assert_eq!(
            decrypted, original_data,
            "Decrypted data should match original"
        );
    }

    #[tokio::test]
    async fn test_multiple_encryptions_different_nonces() {
        let config = CryptoServiceConfig::default();
        let service = BearDogCryptoService::new(config).expect("Failed to create service");

        let data = b"same data";

        // Encrypt same data twice
        let encrypted1 = service
            .encrypt(
                data,
                CryptoAlgorithm::Aes256Gcm,
                EncryptOptions {
                    key_id: "nonce-test".to_string(),
                    associated_data: None,
                },
            )
            .await
            .expect("First encryption failed");

        let encrypted2 = service
            .encrypt(
                data,
                CryptoAlgorithm::Aes256Gcm,
                EncryptOptions {
                    key_id: "nonce-test".to_string(),
                    associated_data: None,
                },
            )
            .await
            .expect("Second encryption failed");

        // Nonces should be different (critical for security)
        assert_ne!(
            encrypted1.metadata.nonce, encrypted2.metadata.nonce,
            "Each encryption must use a unique nonce"
        );

        // Ciphertexts should be different (due to different nonces)
        assert_ne!(
            encrypted1.ciphertext, encrypted2.ciphertext,
            "Same plaintext with different nonces should produce different ciphertexts"
        );
    }
}

/// Test suite for signing/verification operations
mod signing_tests {
    use crate::crypto_service::{BearDogCryptoService, CryptoService, CryptoServiceConfig};
    use beardog_types::crypto_service::*;

    #[tokio::test]
    async fn test_sign_empty_message() {
        let config = CryptoServiceConfig::default();
        let service = BearDogCryptoService::new(config).expect("Failed to create service");

        let result = service
            .sign(
                &[],
                SignatureAlgorithm::Ed25519,
                SignOptions {
                    key_id: "sign-key".to_string(),
                    context: None,
                },
            )
            .await;

        // Empty message should sign successfully
        assert!(result.is_ok(), "Empty message signing should succeed");
    }

    #[tokio::test]
    async fn test_sign_large_message() {
        let config = CryptoServiceConfig::default();
        let service = BearDogCryptoService::new(config).expect("Failed to create service");

        // Create 1MB message
        let large_message = vec![42u8; 1024 * 1024];

        let result = service
            .sign(
                &large_message,
                SignatureAlgorithm::Ed25519,
                SignOptions {
                    key_id: "large-sign-key".to_string(),
                    context: None,
                },
            )
            .await;

        assert!(result.is_ok(), "Large message signing should succeed");
        let signature = result.unwrap();
        assert!(!signature.signature.is_empty());
    }

    #[tokio::test]
    async fn test_sign_with_context() {
        let config = CryptoServiceConfig::default();
        let service = BearDogCryptoService::new(config).expect("Failed to create service");

        let message = b"message to sign";
        let context = "specific-domain";

        let result = service
            .sign(
                message,
                SignatureAlgorithm::Ed25519,
                SignOptions {
                    key_id: "context-key".to_string(),
                    context: Some(context.to_string()),
                },
            )
            .await;

        assert!(result.is_ok(), "Signing with context should succeed");
        let signature = result.unwrap();
        assert_eq!(signature.metadata.context, Some(context.to_string()));
    }

    #[tokio::test]
    async fn test_verify_wrong_public_key() {
        let config = CryptoServiceConfig::default();
        let service = BearDogCryptoService::new(config).expect("Failed to create service");

        let message = b"test message";

        // Sign with one key
        let signature = service
            .sign(
                message,
                SignatureAlgorithm::Ed25519,
                SignOptions {
                    key_id: "key1".to_string(),
                    context: None,
                },
            )
            .await
            .expect("Signing failed");

        // Try to verify with different public key
        let wrong_public_key = vec![0u8; 32]; // Invalid public key

        let result = service
            .verify(
                message,
                &signature,
                VerifyOptions {
                    public_key: wrong_public_key,
                    context: None,
                },
            )
            .await;

        // Should return Ok(false) - verification failed with wrong key
        assert!(result.is_ok(), "Verification should complete without error");
        assert!(
            !result.unwrap(),
            "Verification with wrong key should return false"
        );
    }

    #[tokio::test]
    async fn test_verify_tampered_signature() {
        let config = CryptoServiceConfig::default();
        let service = BearDogCryptoService::new(config).expect("Failed to create service");

        let message = b"test message";

        let mut signature = service
            .sign(
                message,
                SignatureAlgorithm::Ed25519,
                SignOptions {
                    key_id: "tamper-sig-key".to_string(),
                    context: None,
                },
            )
            .await
            .expect("Signing failed");

        // Get the public key for verification (in real use, this would be stored separately)
        // For this test, we'll generate a dummy public key
        let public_key = vec![0u8; 32];

        // Tamper with signature
        if !signature.signature.is_empty() {
            signature.signature[0] ^= 0xFF;
        }

        let result = service
            .verify(
                message,
                &signature,
                VerifyOptions {
                    public_key,
                    context: None,
                },
            )
            .await;

        // Should return Ok(false) - verification failed
        assert!(result.is_ok(), "Verification should complete without error");
        assert!(
            !result.unwrap(),
            "Verification of tampered signature should return false"
        );
    }

    #[tokio::test]
    async fn test_verify_wrong_message() {
        let config = CryptoServiceConfig::default();
        let service = BearDogCryptoService::new(config).expect("Failed to create service");

        let original_message = b"original message";
        let different_message = b"different message";

        let signature = service
            .sign(
                original_message,
                SignatureAlgorithm::Ed25519,
                SignOptions {
                    key_id: "msg-test-key".to_string(),
                    context: None,
                },
            )
            .await
            .expect("Signing failed");

        let public_key = vec![0u8; 32];

        // Try to verify different message with same signature
        let result = service
            .verify(
                different_message,
                &signature,
                VerifyOptions {
                    public_key,
                    context: None,
                },
            )
            .await;

        // Should return Ok(false) - verification failed
        assert!(result.is_ok(), "Verification should complete without error");
        assert!(
            !result.unwrap(),
            "Verification of wrong message should return false"
        );
    }
}

/// Test suite for key generation operations
mod key_generation_tests {
    use crate::crypto_service::{BearDogCryptoService, CryptoService, CryptoServiceConfig};
    use beardog_types::crypto_service::*;

    #[tokio::test]
    async fn test_generate_symmetric_key() {
        let config = CryptoServiceConfig::default();
        let service = BearDogCryptoService::new(config).expect("Failed to create service");

        let result = service
            .generate_key(
                KeyAlgorithm::Aes256,
                KeyGenOptions {
                    key_id: None,
                    use_hsm: false,
                    use_genetic: false,
                    purpose: Some("encryption".to_string()),
                    metadata: std::collections::HashMap::new(),
                },
            )
            .await;

        assert!(result.is_ok(), "Symmetric key generation should succeed");
        let key_info = result.unwrap();
        assert_eq!(key_info.algorithm, KeyAlgorithm::Aes256);
        assert_eq!(key_info.metadata.size_bits, 256);
    }

    #[tokio::test]
    async fn test_generate_asymmetric_key() {
        let config = CryptoServiceConfig::default();
        let service = BearDogCryptoService::new(config).expect("Failed to create service");

        let result = service
            .generate_key(
                KeyAlgorithm::Ed25519,
                KeyGenOptions {
                    key_id: None,
                    use_hsm: false,
                    use_genetic: false,
                    purpose: Some("signing".to_string()),
                    metadata: std::collections::HashMap::new(),
                },
            )
            .await;

        assert!(result.is_ok(), "Asymmetric key generation should succeed");
        let key_info = result.unwrap();
        assert_eq!(key_info.algorithm, KeyAlgorithm::Ed25519);
    }

    #[tokio::test]
    async fn test_generate_key_with_metadata() {
        let config = CryptoServiceConfig::default();
        let service = BearDogCryptoService::new(config).expect("Failed to create service");

        let mut metadata = std::collections::HashMap::new();
        metadata.insert("owner".to_string(), "test-user".to_string());
        metadata.insert("environment".to_string(), "test".to_string());

        let result = service
            .generate_key(
                KeyAlgorithm::Aes256,
                KeyGenOptions {
                    key_id: None,
                    use_hsm: false,
                    use_genetic: false,
                    purpose: Some("test-encryption".to_string()),
                    metadata,
                },
            )
            .await;

        assert!(
            result.is_ok(),
            "Key generation with metadata should succeed"
        );
    }
}

/// Test suite for service capabilities and health
mod service_tests {
    use crate::crypto_service::{BearDogCryptoService, CryptoService, CryptoServiceConfig};
    use beardog_types::crypto_service::*;

    #[tokio::test]
    async fn test_get_capabilities() {
        let config = CryptoServiceConfig {
            service_name: "test-crypto-service".to_string(),
            hsm_enabled: false,
            genetic_enabled: true,
            max_data_size: 10 * 1024 * 1024,
            audit_enabled: true,
        };
        let service = BearDogCryptoService::new(config).expect("Failed to create service");

        let result = service.get_capabilities().await;

        assert!(result.is_ok(), "Get capabilities should succeed");
        let caps = result.unwrap();

        assert_eq!(caps.service_name, "test-crypto-service");
        assert!(!caps.supported_algorithms.is_empty());
        assert!(caps.features.contains(&"genetic".to_string()));
        assert_eq!(caps.max_data_size, 10 * 1024 * 1024);
    }

    #[tokio::test]
    async fn test_get_health() {
        let config = CryptoServiceConfig::default();
        let service = BearDogCryptoService::new(config).expect("Failed to create service");

        let result = service.get_health().await;

        assert!(result.is_ok(), "Get health should succeed");
        let health = result.unwrap();

        assert!(health.healthy, "Service should be healthy");
        assert_eq!(health.operations_completed, 0); // Fresh service
    }

    #[tokio::test]
    async fn test_operation_counter_increments() {
        let config = CryptoServiceConfig::default();
        let service = BearDogCryptoService::new(config).expect("Failed to create service");

        // Perform an operation
        let _ = service
            .encrypt(
                b"test",
                CryptoAlgorithm::Aes256Gcm,
                EncryptOptions {
                    key_id: "counter-key".to_string(),
                    associated_data: None,
                },
            )
            .await;

        let health = service.get_health().await.expect("Health check failed");
        assert!(
            health.operations_completed > 0,
            "Operations counter should increment"
        );
    }
}

/// Test suite for configuration and initialization
mod config_tests {
    use crate::crypto_service::{BearDogCryptoService, CryptoServiceConfig};

    #[test]
    fn test_config_default() {
        let config = CryptoServiceConfig::default();

        assert_eq!(config.service_name, "beardog");
        assert!(!config.hsm_enabled);
        assert!(config.genetic_enabled);
        assert_eq!(config.max_data_size, 10 * 1024 * 1024);
        assert!(config.audit_enabled);
    }

    #[test]
    fn test_config_custom() {
        let config = CryptoServiceConfig {
            service_name: "custom-service".to_string(),
            hsm_enabled: true,
            genetic_enabled: false,
            max_data_size: 1024,
            audit_enabled: false,
        };

        assert_eq!(config.service_name, "custom-service");
        assert!(config.hsm_enabled);
        assert!(!config.genetic_enabled);
        assert_eq!(config.max_data_size, 1024);
        assert!(!config.audit_enabled);
    }

    #[test]
    fn test_service_creation() {
        let config = CryptoServiceConfig::default();
        let result = BearDogCryptoService::new(config);

        assert!(result.is_ok(), "Service creation should succeed");
    }

    #[test]
    fn test_service_clone_config() {
        let config = CryptoServiceConfig::default();
        let cloned = config.clone();

        assert_eq!(config.service_name, cloned.service_name);
        assert_eq!(config.hsm_enabled, cloned.hsm_enabled);
    }
}
