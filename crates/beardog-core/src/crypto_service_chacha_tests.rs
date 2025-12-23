//! ChaCha20-Poly1305 Implementation Tests
//!
//! Comprehensive tests for the ChaCha20-Poly1305 AEAD cipher implementation.

use crate::crypto_service::{BearDogCryptoService, CryptoService, CryptoServiceConfig};
use beardog_types::crypto_service::*;

/// Test ChaCha20-Poly1305 basic encryption/decryption
#[tokio::test]
async fn test_chacha20poly1305_encrypt_decrypt() {
    let config = CryptoServiceConfig::default();
    let service = BearDogCryptoService::new(config).expect("Failed to create service");

    let plaintext = b"The quick brown fox jumps over the lazy dog";
    let key_id = "chacha-test-key-1".to_string();

    // Encrypt with ChaCha20-Poly1305
    let encrypted = service
        .encrypt(
            plaintext,
            CryptoAlgorithm::ChaCha20Poly1305,
            EncryptOptions {
                key_id: key_id.clone(),
                associated_data: None,
            },
        )
        .await
        .expect("Encryption failed");

    // Verify metadata
    assert_eq!(encrypted.algorithm, CryptoAlgorithm::ChaCha20Poly1305);
    assert!(!encrypted.ciphertext.is_empty());
    assert_eq!(encrypted.metadata.nonce.len(), 12); // ChaCha20 uses 96-bit nonce
    assert!(encrypted.metadata.tag.is_some());
    assert_eq!(encrypted.metadata.tag.as_ref().unwrap().len(), 16); // 128-bit tag

    // Ciphertext should be different from plaintext
    assert_ne!(encrypted.ciphertext.as_slice(), plaintext);

    // Decrypt
    let decrypted = service
        .decrypt(
            &encrypted,
            DecryptOptions {
                key_id,
                associated_data: None,
            },
        )
        .await
        .expect("Decryption failed");

    // Verify perfect roundtrip
    assert_eq!(decrypted, plaintext);
}

/// Test ChaCha20-Poly1305 with empty data
#[tokio::test]
async fn test_chacha20poly1305_empty_data() {
    let config = CryptoServiceConfig::default();
    let service = BearDogCryptoService::new(config).expect("Failed to create service");

    let encrypted = service
        .encrypt(
            &[],
            CryptoAlgorithm::ChaCha20Poly1305,
            EncryptOptions {
                key_id: "empty-key".to_string(),
                associated_data: None,
            },
        )
        .await
        .expect("Empty data encryption should succeed");

    assert_eq!(encrypted.algorithm, CryptoAlgorithm::ChaCha20Poly1305);

    let decrypted = service
        .decrypt(
            &encrypted,
            DecryptOptions {
                key_id: "empty-key".to_string(),
                associated_data: None,
            },
        )
        .await
        .expect("Empty data decryption should succeed");

    assert_eq!(decrypted.len(), 0);
}

/// Test ChaCha20-Poly1305 with large data
#[tokio::test]
async fn test_chacha20poly1305_large_data() {
    let config = CryptoServiceConfig::default();
    let service = BearDogCryptoService::new(config).expect("Failed to create service");

    // 1MB of data
    let large_data = vec![42u8; 1024 * 1024];

    let encrypted = service
        .encrypt(
            &large_data,
            CryptoAlgorithm::ChaCha20Poly1305,
            EncryptOptions {
                key_id: "large-data-key".to_string(),
                associated_data: None,
            },
        )
        .await
        .expect("Large data encryption should succeed");

    let decrypted = service
        .decrypt(
            &encrypted,
            DecryptOptions {
                key_id: "large-data-key".to_string(),
                associated_data: None,
            },
        )
        .await
        .expect("Large data decryption should succeed");

    assert_eq!(decrypted, large_data);
}

/// Test ChaCha20-Poly1305 with AAD (Additional Authenticated Data)
#[tokio::test]
async fn test_chacha20poly1305_with_aad() {
    let config = CryptoServiceConfig::default();
    let service = BearDogCryptoService::new(config).expect("Failed to create service");

    let plaintext = b"secret message";
    let aad = b"authentication context";

    let encrypted = service
        .encrypt(
            plaintext,
            CryptoAlgorithm::ChaCha20Poly1305,
            EncryptOptions {
                key_id: "aad-test-key".to_string(),
                associated_data: Some(aad.to_vec()),
            },
        )
        .await
        .expect("Encryption with AAD should succeed");

    // Decrypt with correct AAD
    let decrypted = service
        .decrypt(
            &encrypted,
            DecryptOptions {
                key_id: "aad-test-key".to_string(),
                associated_data: Some(aad.to_vec()),
            },
        )
        .await
        .expect("Decryption with correct AAD should succeed");

    assert_eq!(decrypted, plaintext);

    // Decrypt with wrong AAD should fail
    let result = service
        .decrypt(
            &encrypted,
            DecryptOptions {
                key_id: "aad-test-key".to_string(),
                associated_data: Some(b"wrong aad".to_vec()),
            },
        )
        .await;

    assert!(result.is_err(), "Decryption with wrong AAD should fail");
}

/// Test ChaCha20-Poly1305 nonce uniqueness
#[tokio::test]
async fn test_chacha20poly1305_nonce_uniqueness() {
    let config = CryptoServiceConfig::default();
    let service = BearDogCryptoService::new(config).expect("Failed to create service");

    let data = b"same data";
    let key_id = "nonce-test-key".to_string();

    // Encrypt same data twice
    let encrypted1 = service
        .encrypt(
            data,
            CryptoAlgorithm::ChaCha20Poly1305,
            EncryptOptions {
                key_id: key_id.clone(),
                associated_data: None,
            },
        )
        .await
        .expect("First encryption failed");

    let encrypted2 = service
        .encrypt(
            data,
            CryptoAlgorithm::ChaCha20Poly1305,
            EncryptOptions {
                key_id,
                associated_data: None,
            },
        )
        .await
        .expect("Second encryption failed");

    // Nonces MUST be different (critical for security)
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

/// Test ChaCha20-Poly1305 tampering detection
#[tokio::test]
async fn test_chacha20poly1305_tampering_detection() {
    let config = CryptoServiceConfig::default();
    let service = BearDogCryptoService::new(config).expect("Failed to create service");

    let mut encrypted = service
        .encrypt(
            b"test data",
            CryptoAlgorithm::ChaCha20Poly1305,
            EncryptOptions {
                key_id: "tamper-test".to_string(),
                associated_data: None,
            },
        )
        .await
        .expect("Encryption failed");

    // Tamper with ciphertext
    if !encrypted.ciphertext.is_empty() {
        encrypted.ciphertext[0] ^= 0xFF;
    }

    // Decryption should fail due to authentication failure
    let result = service
        .decrypt(
            &encrypted,
            DecryptOptions {
                key_id: "tamper-test".to_string(),
                associated_data: None,
            },
        )
        .await;

    assert!(result.is_err(), "Decryption of tampered data should fail");
}

/// Test ChaCha20-Poly1305 wrong key detection
#[tokio::test]
async fn test_chacha20poly1305_wrong_key() {
    let config = CryptoServiceConfig::default();
    let service = BearDogCryptoService::new(config).expect("Failed to create service");

    let encrypted = service
        .encrypt(
            b"test data",
            CryptoAlgorithm::ChaCha20Poly1305,
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

    assert!(result.is_err(), "Decryption with wrong key should fail");
}

/// Compare ChaCha20-Poly1305 vs AES-256-GCM
#[tokio::test]
async fn test_algorithm_comparison() {
    let config = CryptoServiceConfig::default();
    let service = BearDogCryptoService::new(config).expect("Failed to create service");

    let plaintext = b"The quick brown fox jumps over the lazy dog";
    let key_id = "compare-key".to_string();

    // Encrypt with ChaCha20-Poly1305
    let chacha_encrypted = service
        .encrypt(
            plaintext,
            CryptoAlgorithm::ChaCha20Poly1305,
            EncryptOptions {
                key_id: key_id.clone(),
                associated_data: None,
            },
        )
        .await
        .expect("ChaCha20 encryption failed");

    // Encrypt with AES-256-GCM
    let aes_encrypted = service
        .encrypt(
            plaintext,
            CryptoAlgorithm::Aes256Gcm,
            EncryptOptions {
                key_id: key_id.clone(),
                associated_data: None,
            },
        )
        .await
        .expect("AES encryption failed");

    // Both should have same nonce size (96 bits = 12 bytes)
    assert_eq!(chacha_encrypted.metadata.nonce.len(), 12);
    assert_eq!(aes_encrypted.metadata.nonce.len(), 12);

    // Both should have 128-bit tags
    assert_eq!(chacha_encrypted.metadata.tag.as_ref().unwrap().len(), 16);
    assert_eq!(aes_encrypted.metadata.tag.as_ref().unwrap().len(), 16);

    // Ciphertexts should be similar length (with slight variations)
    let chacha_len = chacha_encrypted.ciphertext.len();
    let aes_len = aes_encrypted.ciphertext.len();
    let diff = chacha_len.abs_diff(aes_len);
    assert!(
        diff < 32,
        "Ciphertext lengths should be similar (diff: {diff})"
    );

    // Both should decrypt correctly
    let chacha_decrypted = service
        .decrypt(
            &chacha_encrypted,
            DecryptOptions {
                key_id: key_id.clone(),
                associated_data: None,
            },
        )
        .await
        .expect("ChaCha20 decryption failed");

    let aes_decrypted = service
        .decrypt(
            &aes_encrypted,
            DecryptOptions {
                key_id,
                associated_data: None,
            },
        )
        .await
        .expect("AES decryption failed");

    assert_eq!(chacha_decrypted, plaintext);
    assert_eq!(aes_decrypted, plaintext);
}

/// Test ChaCha20-Poly1305 performance characteristics
#[tokio::test]
async fn test_chacha20poly1305_performance() {
    let config = CryptoServiceConfig::default();
    let service = BearDogCryptoService::new(config).expect("Failed to create service");

    // Test with various data sizes
    for size in &[16, 256, 1024, 16384] {
        let data = vec![42u8; *size];
        let key_id = format!("perf-key-{size}");

        let start = std::time::Instant::now();

        let encrypted = service
            .encrypt(
                &data,
                CryptoAlgorithm::ChaCha20Poly1305,
                EncryptOptions {
                    key_id: key_id.clone(),
                    associated_data: None,
                },
            )
            .await
            .expect("Encryption failed");

        let encrypt_time = start.elapsed();

        let start = std::time::Instant::now();

        let _ = service
            .decrypt(
                &encrypted,
                DecryptOptions {
                    key_id,
                    associated_data: None,
                },
            )
            .await
            .expect("Decryption failed");

        let decrypt_time = start.elapsed();

        // Performance should be reasonable (< 10ms for small data)
        if *size <= 1024 {
            assert!(
                encrypt_time.as_millis() < 100,
                "Encryption of {size} bytes took {encrypt_time:?}"
            );
            assert!(
                decrypt_time.as_millis() < 100,
                "Decryption of {size} bytes took {decrypt_time:?}"
            );
        }
    }
}
