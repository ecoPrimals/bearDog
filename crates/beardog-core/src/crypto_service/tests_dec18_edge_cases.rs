// SPDX-License-Identifier: AGPL-3.0-only

//! Crypto Service Edge Cases Tests - December 18, 2025
//!
//! Deep testing of edge cases, boundary conditions, and error paths
//! for `BearDogCryptoService` to increase coverage to 90%.

use super::implementation::BearDogCryptoService;
use super::r#trait::CryptoService;
use super::types::CryptoServiceConfig;
use beardog_types::crypto_service::{
    CryptoAlgorithm, DecryptOptions, EncryptOptions, KeyAlgorithm, KeyGenOptions,
};

/// Helper to create test crypto service
fn create_test_service() -> BearDogCryptoService {
    let config = CryptoServiceConfig {
        service_name: "test-crypto".to_string(),
        audit_enabled: false,
        max_data_size: 10 * 1024 * 1024, // 10 MB
        genetic_enabled: false,
        hsm_enabled: false,
    };
    BearDogCryptoService::new(config).expect("Failed to create test service")
}

// =============================================================================
// Encryption Edge Cases
// =============================================================================

#[tokio::test]
async fn test_encrypt_empty_data() {
    let service = create_test_service();
    let empty_data = b"";

    let options = EncryptOptions {
        key_id: "test-key".to_string(),
        associated_data: None,
    };

    // Empty data should be handled gracefully
    let result = service
        .encrypt(empty_data, CryptoAlgorithm::Aes256Gcm, options)
        .await;

    assert!(
        result.is_ok(),
        "Empty data encryption should succeed or fail gracefully"
    );
}

#[tokio::test]
async fn test_encrypt_single_byte() {
    let service = create_test_service();
    let data = b"A";

    let options = EncryptOptions {
        key_id: "single-byte-key".to_string(),
        associated_data: None,
    };

    let result = service
        .encrypt(data, CryptoAlgorithm::Aes256Gcm, options)
        .await;

    assert!(result.is_ok(), "Single byte encryption should work");
    if let Ok(encrypted) = result {
        assert!(!encrypted.ciphertext.is_empty());
        assert!(!encrypted.metadata.nonce.is_empty());
    }
}

#[tokio::test]
async fn test_encrypt_large_data() {
    let service = create_test_service();
    // 1 MB of data
    let large_data = vec![0u8; 1024 * 1024];

    let options = EncryptOptions {
        key_id: "large-data-key".to_string(),
        associated_data: None,
    };

    let result = service
        .encrypt(&large_data, CryptoAlgorithm::Aes256Gcm, options)
        .await;

    assert!(result.is_ok(), "Large data encryption should succeed");
}

#[tokio::test]
async fn test_encrypt_exceeds_max_size() {
    let config = CryptoServiceConfig {
        service_name: "test-crypto".to_string(),
        audit_enabled: false,
        max_data_size: 1024, // Only 1 KB max
        genetic_enabled: false,
        hsm_enabled: false,
    };
    let service = BearDogCryptoService::new(config).unwrap();

    // Try to encrypt 2 KB (exceeds limit)
    let too_large = vec![0u8; 2048];

    let options = EncryptOptions {
        key_id: "test-key".to_string(),
        associated_data: None,
    };

    let result = service
        .encrypt(&too_large, CryptoAlgorithm::Aes256Gcm, options)
        .await;

    assert!(result.is_err(), "Should reject data exceeding max size");
    if let Err(e) = result {
        assert!(e.to_string().contains("exceeds maximum"));
    }
}

#[tokio::test]
async fn test_encrypt_with_associated_data() {
    let service = create_test_service();
    let data = b"secret message";
    let aad = b"context information";

    let options = EncryptOptions {
        key_id: "aad-test-key".to_string(),
        associated_data: Some(aad.to_vec()),
    };

    let result = service
        .encrypt(data, CryptoAlgorithm::Aes256Gcm, options)
        .await;

    assert!(result.is_ok(), "Encryption with AAD should work");
}

#[tokio::test]
async fn test_encrypt_decrypt_roundtrip_various_sizes() {
    let service = create_test_service();

    let data_1kb = vec![0u8; 1024];
    let data_64kb = vec![0u8; 64 * 1024];
    let test_cases = vec![
        (b"a" as &[u8], "single"),
        (b"hello world", "short"),
        (&data_1kb[..], "1kb"),
        (&data_64kb[..], "64kb"),
    ];

    for (data, label) in test_cases {
        let options = EncryptOptions {
            key_id: format!("roundtrip-{label}"),
            associated_data: None,
        };

        let encrypted = service
            .encrypt(data, CryptoAlgorithm::Aes256Gcm, options)
            .await
            .unwrap_or_else(|_| panic!("Encryption failed for {label}"));

        let decrypt_opts = DecryptOptions {
            key_id: encrypted.metadata.key_id.clone().unwrap_or_default(),
            associated_data: None,
        };

        let decrypted = service
            .decrypt(&encrypted, decrypt_opts)
            .await
            .unwrap_or_else(|_| panic!("Decryption failed for {label}"));

        assert_eq!(decrypted, data, "Roundtrip failed for {label}");
    }
}

// =============================================================================
// Decryption Edge Cases
// =============================================================================

#[tokio::test]
async fn test_decrypt_with_wrong_key() {
    let service = create_test_service();
    let data = b"secret";

    // Encrypt with one key
    let encrypt_opts = EncryptOptions {
        key_id: "key-1".to_string(),
        associated_data: None,
    };

    let encrypted = service
        .encrypt(data, CryptoAlgorithm::Aes256Gcm, encrypt_opts)
        .await
        .unwrap();

    // Try to decrypt with different key
    let decrypt_opts = DecryptOptions {
        key_id: "key-2".to_string(),
        associated_data: None,
    };

    let result = service.decrypt(&encrypted, decrypt_opts).await;

    assert!(result.is_err(), "Decryption with wrong key should fail");
}

#[tokio::test]
async fn test_decrypt_with_corrupted_ciphertext() {
    let service = create_test_service();
    let data = b"secret";

    let encrypt_opts = EncryptOptions {
        key_id: "corruption-test".to_string(),
        associated_data: None,
    };

    let mut encrypted = service
        .encrypt(data, CryptoAlgorithm::Aes256Gcm, encrypt_opts.clone())
        .await
        .unwrap();

    // Corrupt the ciphertext
    if !encrypted.ciphertext.is_empty() {
        encrypted.ciphertext[0] ^= 0xFF;
    }

    let decrypt_opts = DecryptOptions {
        key_id: encrypted.metadata.key_id.clone().unwrap_or_default(),
        associated_data: None,
    };

    let result = service.decrypt(&encrypted, decrypt_opts).await;

    assert!(result.is_err(), "Decryption of corrupted data should fail");
}

#[tokio::test]
async fn test_decrypt_with_wrong_aad() {
    let service = create_test_service();
    let data = b"secret";
    let aad = b"correct context";

    let encrypt_opts = EncryptOptions {
        key_id: "aad-mismatch".to_string(),
        associated_data: Some(aad.to_vec()),
    };

    let encrypted = service
        .encrypt(data, CryptoAlgorithm::Aes256Gcm, encrypt_opts.clone())
        .await
        .unwrap();

    // Try to decrypt with wrong AAD
    let decrypt_opts = DecryptOptions {
        key_id: encrypted.metadata.key_id.clone().unwrap_or_default(),
        associated_data: Some(b"wrong context".to_vec()),
    };

    let result = service.decrypt(&encrypted, decrypt_opts).await;

    assert!(result.is_err(), "Decryption with wrong AAD should fail");
}

// =============================================================================
// Key Generation Edge Cases
// =============================================================================

#[tokio::test]
async fn test_generate_key_various_algorithms() {
    let service = create_test_service();

    let algorithms = vec![
        KeyAlgorithm::Aes256,
        KeyAlgorithm::Ed25519,
        KeyAlgorithm::EcdsaP256,
    ];

    for algo in algorithms {
        let options = KeyGenOptions {
            key_id: None, // Auto-generate key IDs for tests
            use_hsm: false,
            use_genetic: false,
            purpose: None,
            metadata: std::collections::HashMap::new(),
        };

        let result = service.generate_key(algo, options).await;
        assert!(result.is_ok(), "Key generation for {algo:?} should succeed");
    }
}

// =============================================================================
// Concurrent Operations
// =============================================================================

#[tokio::test]
async fn test_concurrent_encryptions() {
    use std::sync::Arc;

    let service = Arc::new(create_test_service());
    let mut handles = vec![];

    for i in 0..10 {
        let svc = Arc::clone(&service);
        let handle = tokio::spawn(async move {
            let data = format!("concurrent message {i}");
            let options = EncryptOptions {
                key_id: format!("concurrent-key-{i}"),
                associated_data: None,
            };

            svc.encrypt(data.as_bytes(), CryptoAlgorithm::Aes256Gcm, options)
                .await
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.await;
        assert!(result.is_ok(), "Concurrent operation should succeed");
        assert!(result.unwrap().is_ok(), "Encryption should succeed");
    }
}

#[tokio::test]
async fn test_concurrent_encrypt_decrypt() {
    use std::sync::Arc;

    let service = Arc::new(create_test_service());
    let mut handles = vec![];

    for i in 0..5 {
        let svc = Arc::clone(&service);
        let handle = tokio::spawn(async move {
            let data = format!("message {i}");
            let key_id = format!("concurrent-ed-{i}");

            let encrypt_opts = EncryptOptions {
                key_id: key_id.clone(),
                associated_data: None,
            };

            let encrypted = svc
                .encrypt(data.as_bytes(), CryptoAlgorithm::Aes256Gcm, encrypt_opts)
                .await?;

            let decrypt_opts = DecryptOptions {
                key_id: encrypted.metadata.key_id.clone().unwrap_or_default(),
                associated_data: None,
            };

            let decrypted = svc.decrypt(&encrypted, decrypt_opts).await?;

            Ok::<_, beardog_errors::BearDogError>(decrypted == data.as_bytes())
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.await;
        assert!(result.is_ok(), "Task should complete");
        assert!(
            matches!(result.unwrap(), Ok(true)),
            "Encrypt/decrypt should match"
        );
    }
}

// =============================================================================
// Service Health and Capabilities
// =============================================================================

#[tokio::test]
async fn test_get_capabilities() {
    let service = create_test_service();

    let result = service.get_capabilities().await;
    assert!(result.is_ok(), "Getting capabilities should succeed");

    if let Ok(caps) = result {
        assert!(
            !caps.supported_algorithms.is_empty(),
            "Should have some algorithms"
        );
    }
}

#[tokio::test]
async fn test_get_health() {
    let service = create_test_service();

    let result = service.get_health().await;
    assert!(result.is_ok(), "Health check should succeed");

    // Modern Rust: u64 is always non-negative, no need to check >= 0
    if let Ok(health) = result {
        // Verify health data exists (uptime_seconds is u64, always valid)
        let _ = health.uptime_seconds;
    }
}

// =============================================================================
// Stress Tests
// =============================================================================

#[tokio::test]
async fn test_many_sequential_operations() {
    let service = create_test_service();

    for i in 0..100 {
        let data = format!("message {i}");
        let options = EncryptOptions {
            key_id: format!("seq-{i}"),
            associated_data: None,
        };

        let result = service
            .encrypt(data.as_bytes(), CryptoAlgorithm::Aes256Gcm, options)
            .await;

        assert!(result.is_ok(), "Sequential operation {i} should succeed");
    }
}

#[tokio::test]
async fn test_rapid_key_generation() {
    let service = create_test_service();

    for i in 0..50 {
        let options = KeyGenOptions {
            key_id: None, // Auto-generate key IDs for rapid tests
            use_hsm: false,
            use_genetic: false,
            purpose: None,
            metadata: std::collections::HashMap::new(),
        };

        let result = service.generate_key(KeyAlgorithm::Aes256, options).await;
        assert!(result.is_ok(), "Rapid key gen {i} should succeed");
    }
}

// =============================================================================
// Configuration Edge Cases
// =============================================================================

#[tokio::test]
async fn test_service_with_audit_enabled() {
    let config = CryptoServiceConfig {
        service_name: "audit-test".to_string(),
        audit_enabled: true,
        max_data_size: 1024 * 1024,
        genetic_enabled: false,
        hsm_enabled: false,
    };
    let service = BearDogCryptoService::new(config).unwrap();

    let options = EncryptOptions {
        key_id: "audit-key".to_string(),
        associated_data: None,
    };

    let result = service
        .encrypt(b"test", CryptoAlgorithm::Aes256Gcm, options)
        .await;

    assert!(result.is_ok(), "Service with audit should work");
}

#[tokio::test]
async fn test_service_with_custom_name() {
    let config = CryptoServiceConfig {
        service_name: "custom-service-name".to_string(),
        audit_enabled: false,
        max_data_size: 1024 * 1024,
        genetic_enabled: false,
        hsm_enabled: false,
    };
    let service = BearDogCryptoService::new(config).unwrap();

    let result = service.get_health().await;
    assert!(result.is_ok(), "Custom named service should work");
}

#[tokio::test]
async fn test_operation_counter_increments() {
    let service = create_test_service();

    // Perform several operations
    for i in 0..10 {
        let options = EncryptOptions {
            key_id: format!("counter-test-{i}"),
            associated_data: None,
        };

        let _ = service
            .encrypt(b"data", CryptoAlgorithm::Aes256Gcm, options)
            .await;
    }

    // Check that operations were counted
    let health = service.get_health().await.unwrap();
    assert!(
        health.operations_completed >= 10,
        "Operations should be counted"
    );
}
