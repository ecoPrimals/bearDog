// SPDX-License-Identifier: AGPL-3.0-only

use crate::encryption::{EncryptionConfig, EncryptionService};
use beardog_errors::BearDogError;

#[cfg(test)]
mod tests {
    use super::*;

    // Helper functions to match old API
    fn encrypt_data(data: &[u8], key: &[u8]) -> Result<Vec<u8>, BearDogError> {
        let config = EncryptionConfig {
            key_size: key.len(), // Use provided key length
            ..Default::default()
        };
        let service = EncryptionService::new(config);
        service.encrypt_data(data, key)
    }

    fn decrypt_data(data: &[u8], key: &[u8]) -> Result<Vec<u8>, BearDogError> {
        let config = EncryptionConfig {
            key_size: key.len(), // Use provided key length
            ..Default::default()
        };
        let service = EncryptionService::new(config);
        service.decrypt_data(data, key)
    }

    // ============================================================================
    // Encryption/Decryption Round-Trip Tests
    // ============================================================================

    #[test]
    fn test_encrypt_decrypt_round_trip_empty() {
        let key = b"test_key_32bytes_long_1234567890"; // Exactly 32 bytes
        let data = b"";

        let encrypted = encrypt_data(data, key).expect("Encryption failed");
        let decrypted = decrypt_data(&encrypted, key).expect("Decryption failed");

        assert_eq!(data.to_vec(), decrypted, "Empty data round trip failed");
    }

    #[test]
    fn test_encrypt_decrypt_round_trip_small() {
        let key = b"test_key_32bytes_long_1234567890"; // Exactly 32 bytes
        let data = b"Hello, BearDog!";

        let encrypted = encrypt_data(data, key).expect("Encryption failed");
        let decrypted = decrypt_data(&encrypted, key).expect("Decryption failed");

        assert_eq!(data.to_vec(), decrypted, "Small data round trip failed");
    }

    #[test]
    fn test_encrypt_decrypt_round_trip_medium() {
        let key = b"test_key_32bytes_long_1234567890";
        let data = b"This is a medium-sized message that should encrypt and decrypt correctly with AES-256.";

        let encrypted = encrypt_data(data, key).expect("Encryption failed");
        let decrypted = decrypt_data(&encrypted, key).expect("Decryption failed");

        assert_eq!(data.to_vec(), decrypted, "Medium data round trip failed");
    }

    #[test]
    fn test_encrypt_decrypt_round_trip_large() {
        let key = b"test_key_32bytes_long_1234567890";
        let data = vec![0u8; 10_000]; // 10KB

        let encrypted = encrypt_data(&data, key).expect("Encryption failed");
        let decrypted = decrypt_data(&encrypted, key).expect("Decryption failed");

        assert_eq!(data, decrypted, "Large data round trip failed");
    }

    #[test]
    fn test_encrypt_decrypt_round_trip_binary() {
        let key = b"test_key_32bytes_long_1234567890";
        let data: Vec<u8> = (0..=255).collect(); // All byte values

        let encrypted = encrypt_data(&data, key).expect("Encryption failed");
        let decrypted = decrypt_data(&encrypted, key).expect("Decryption failed");

        assert_eq!(data, decrypted, "Binary data round trip failed");
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical

    #[test]
    fn test_encrypt_decrypt_round_trip_unicode() {
        let key = b"test_key_32bytes_long_1234567890";
        let data = "Hello 世界! 🦀 Rust is awesome! 🐻".as_bytes();

        let encrypted = encrypt_data(data, key).expect("Encryption failed");
        let decrypted = decrypt_data(&encrypted, key).expect("Decryption failed");
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical

        assert_eq!(data.to_vec(), decrypted, "Unicode data round trip failed");
    }

    // ============================================================================
    // Encryption Properties Tests
    // ============================================================================

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
    #[test]
    fn test_encryption_produces_different_output() {
        let key = b"test_key_32bytes_long_1234567890";
        let data = b"Same input data";

        let encrypted1 = encrypt_data(data, key).expect("First encryption failed");
        let encrypted2 = encrypt_data(data, key).expect("Second encryption failed");

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical
        // With IV, same plaintext should produce different ciphertext
        assert_ne!(
            encrypted1, encrypted2,
            "Encryptions should differ due to IV"
        );
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
    fn test_encrypted_data_different_from_plaintext() {
        let key = b"test_key_32bytes_long_1234567890";
        let data = b"Test data for encryption";

        let encrypted = encrypt_data(data, key).expect("Encryption failed");

        assert_ne!(
            data.to_vec(),
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: security
            // TEST_PRIORITY: critical
            encrypted,
            "Encrypted data should differ from plaintext"
        );
        assert!(
            encrypted.len() > data.len(),
            "Encrypted data should be larger (IV + padding)"
        );
    }

    #[test]
    fn test_different_keys_produce_different_ciphertext() {
        let key1 = b"key_one_32bytes_long_12345678901"; // Exactly 32 bytes
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical
        let key2 = b"key_two_32bytes_long_09876543210"; // Exactly 32 bytes
        let data = b"Test data";

        let encrypted1 = encrypt_data(data, key1).expect("Encryption with key1 failed");
        let encrypted2 = encrypt_data(data, key2).expect("Encryption with key2 failed");

        assert_ne!(
            encrypted1, encrypted2,
            "Different keys should produce different ciphertext"
        );
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
    // ============================================================================
    // Decryption Error Cases
    // ============================================================================

    #[test]
    fn test_decrypt_with_wrong_key_fails() {
        let correct_key = b"correct_key_32bytes_123456789012"; // Exactly 32 bytes
        let wrong_key = b"wrong_key_32bytes_long_987654321"; // Exactly 32 bytes
        let data = b"Secret message";

        let encrypted = encrypt_data(data, correct_key).expect("Encryption failed");
        let result = decrypt_data(&encrypted, wrong_key);

        assert!(result.is_err(), "Decryption with wrong key should fail");
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    #[test]
    fn test_decrypt_corrupted_data_fails() {
        let key = b"test_key_32bytes_long_1234567890";
        let data = b"Test data";

        let mut encrypted = encrypt_data(data, key).expect("Encryption failed");

        // Corrupt the ciphertext
        if encrypted.len() > 20 {
            encrypted[20] ^= 0xFF;
        }

        let result = decrypt_data(&encrypted, key);
        assert!(result.is_err(), "Decryption of corrupted data should fail");
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical

    #[test]
    fn test_decrypt_truncated_data_fails() {
        let key = b"test_key_32bytes_long_1234567890";
        let data = b"Test data for truncation";

        let encrypted = encrypt_data(data, key).expect("Encryption failed");

        // Truncate the ciphertext
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical
        let truncated = &encrypted[..encrypted.len() / 2];

        let result = decrypt_data(truncated, key);
        assert!(result.is_err(), "Decryption of truncated data should fail");
    }

    // ============================================================================
    // Key Size Tests
    // ============================================================================

    #[test]
    fn test_various_valid_key_sizes() {
        let data = b"Test data";
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical

        // Test AES-256-GCM (currently only 32-byte keys supported)
        let keys = vec![
            &b"key_32_bytes_long_12345678901234"[..], // 32 bytes (256-bit)
            &b"another_32byte_key_4567890123456"[..], // 32 bytes (256-bit)
        ];

        for key in keys {
            let encrypted = encrypt_data(data, key).expect("Encryption should work");
            let decrypted = decrypt_data(&encrypted, key).expect("Decryption should work");
            assert_eq!(data.to_vec(), decrypted);
        }
    }

    // ============================================================================
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    // Edge Cases
    // ============================================================================

    #[test]
    fn test_encrypt_decrypt_exact_block_size() {
        let key = b"test_key_32bytes_long_1234567890";
        let data = vec![0u8; 16]; // Exactly one AES block

        let encrypted = encrypt_data(&data, key).expect("Encryption failed");
        let decrypted = decrypt_data(&encrypted, key).expect("Decryption failed");

        assert_eq!(data, decrypted);
    }

    #[test]
    fn test_encrypt_decrypt_multiple_blocks() {
        let key = b"test_key_32bytes_long_1234567890";
        let data = vec![0u8; 48]; // Exactly 3 AES blocks
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical

        let encrypted = encrypt_data(&data, key).expect("Encryption failed");
        let decrypted = decrypt_data(&encrypted, key).expect("Decryption failed");

        assert_eq!(data, decrypted);
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
    fn test_encrypt_decrypt_off_block_boundary() {
        let key = b"test_key_32bytes_long_1234567890";

        // Test various sizes that don't align with 16-byte blocks
        for size in [15, 17, 31, 33, 47, 49] {
            let data = vec![0u8; size];
            let encrypted = encrypt_data(&data, key).expect("Encryption failed");
            let decrypted = decrypt_data(&encrypted, key).expect("Decryption failed");
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: security
            // TEST_PRIORITY: critical
            assert_eq!(data, decrypted, "Failed at size {size}");
        }
    }

    // ============================================================================
    // Performance Baseline Tests
    // ============================================================================

    #[test]
    fn test_encryption_performance_baseline() {
        use std::time::Instant;

        let key = b"test_key_32bytes_long_1234567890";
        let data = vec![0u8; 1_000_000]; // 1MB
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical

        let start = Instant::now();
        let encrypted = encrypt_data(&data, key).expect("Encryption failed");
        let encrypt_duration = start.elapsed();

        let start = Instant::now();
        let decrypted = decrypt_data(&encrypted, key).expect("Decryption failed");
        let decrypt_duration = start.elapsed();

        assert_eq!(data, decrypted);

        // Should complete in reasonable time (< 1 second for 1MB on modern hardware)
        assert!(
            encrypt_duration.as_secs() < 5,
            "Encryption should be reasonably fast"
        );
        assert!(
            decrypt_duration.as_secs() < 5,
            "Decryption should be reasonably fast"
        );
    }

    // ============================================================================
    // Multiple Round-Trip Tests
    // ============================================================================

    #[test]
    fn test_multiple_encryption_rounds() {
        let key = b"test_key_32bytes_long_1234567890";
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical
        let mut data = b"Original data".to_vec();

        // Encrypt and decrypt multiple times
        for _ in 0..5 {
            let encrypted = encrypt_data(&data, key).expect("Encryption failed");
            data = decrypt_data(&encrypted, key).expect("Decryption failed");
        }

        assert_eq!(
            data, b"Original data",
            "Multiple rounds should preserve data"
        );
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
    #[test]
    fn test_encrypt_already_encrypted_data() {
        let key = b"test_key_32bytes_long_1234567890";
        let data = b"Test data";

        let encrypted1 = encrypt_data(data, key).expect("First encryption failed");
        let encrypted2 = encrypt_data(&encrypted1, key).expect("Second encryption failed");

        let decrypted1 = decrypt_data(&encrypted2, key).expect("First decryption failed");
        let decrypted2 = decrypt_data(&decrypted1, key).expect("Second decryption failed");

        assert_eq!(
            data.to_vec(),
            decrypted2,
            "Double encryption round trip failed"
        );
    }
}
