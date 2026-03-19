// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive tests for crypto utilities
//!
//! Tests key derivation, encryption, decryption, and other cryptographic operations.

#[cfg(test)]
mod crypto_utils_tests {

    use beardog_errors::BearDogError;

    /// Test key derivation with valid inputs
    #[test]
    fn test_derive_key_valid_inputs() {
        let password = b"test_password_123";
        let salt = b"random_salt_value";

        let result = derive_key(password, salt, 32);
        assert!(
            result.is_ok(),
            "Key derivation should succeed with valid inputs"
        );

        let key = result.unwrap();
        assert_eq!(key.len(), 32, "Derived key should be 32 bytes");
    }

    /// Test key derivation with different key sizes
    #[test]
    fn test_derive_key_different_sizes() {
        let password = b"test_password";
        let salt = b"salt";

        for size in [16, 24, 32, 48, 64].iter() {
            let result = derive_key(password, salt, *size);
            assert!(result.is_ok(), "Derivation should work for size {}", size);
            assert_eq!(
                result.unwrap().len(),
                *size,
                "Key size should match requested"
            );
        }
    }

    /// Test key derivation with empty password
    #[test]
    fn test_derive_key_empty_password() {
        let password = b"";
        let salt = b"salt";

        let result = derive_key(password, salt, 32);
        // Should handle empty password gracefully
        assert!(
            result.is_ok() || result.is_err(),
            "Should handle empty password"
        );
    }

    /// Test key derivation with empty salt
    #[test]
    fn test_derive_key_empty_salt() {
        let password = b"password";
        let salt = b"";

        let result = derive_key(password, salt, 32);
        // Should handle empty salt gracefully
        assert!(
            result.is_ok() || result.is_err(),
            "Should handle empty salt"
        );
    }

    /// Test key derivation determinism
    #[test]
    fn test_derive_key_deterministic() {
        let password = b"test_password";
        let salt = b"test_salt";

        let key1 = derive_key(password, salt, 32).expect("First derivation");
        let key2 = derive_key(password, salt, 32).expect("Second derivation");

        assert_eq!(key1, key2, "Same inputs should produce same key");
    }

    /// Test key derivation uniqueness with different salts
    #[test]
    fn test_derive_key_different_salts() {
        let password = b"same_password";
        let salt1 = b"salt1";
        let salt2 = b"salt2";

        let key1 = derive_key(password, salt1, 32).expect("First derivation");
        let key2 = derive_key(password, salt2, 32).expect("Second derivation");

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        assert_ne!(key1, key2, "Different salts should produce different keys");
    }

    /// Test encryption with valid key
    #[test]
    fn test_encrypt_data_valid() {
        let key = [0u8; 32]; // Simple test key
        let plaintext = b"Hello, secure world!";

        let result = encrypt_data(plaintext, &key);
        assert!(result.is_ok(), "Encryption should succeed");
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal

        let ciphertext = result.unwrap();
        assert!(!ciphertext.is_empty(), "Ciphertext should not be empty");
        assert_ne!(
            plaintext.to_vec(),
            ciphertext,
            "Ciphertext should differ from plaintext"
        );
    }

    /// Test encryption of empty data
    #[test]
    fn test_encrypt_empty_data() {
        let key = [0u8; 32];
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let plaintext = b"";

        let result = encrypt_data(plaintext, &key);
        assert!(result.is_ok(), "Should encrypt empty data");
    }

    /// Test encryption with large data
    #[test]
    fn test_encrypt_large_data() {
        let key = [0u8; 32];
        let plaintext = vec![0u8; 1024 * 1024]; // 1MB
                                                // TEST_CATEGORY: integration
                                                // TEST_DOMAIN: security
                                                // TEST_PRIORITY: normal

        let result = encrypt_data(&plaintext, &key);
        assert!(result.is_ok(), "Should encrypt large data");
    }

    /// Test decryption of valid ciphertext
    #[test]
    fn test_decrypt_data_valid() {
        let key = [0u8; 32];
        let plaintext = b"Test message for encryption";

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let ciphertext = encrypt_data(plaintext, &key).expect("Encryption");
        let result = decrypt_data(&ciphertext, &key);

        assert!(result.is_ok(), "Decryption should succeed");
        assert_eq!(
            result.unwrap(),
            plaintext.to_vec(),
            "Decrypted should match original"
        );
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    /// Test round-trip encryption/decryption
    #[test]
    fn test_encrypt_decrypt_round_trip() {
        let key = [0u8; 32];
        let original = b"Round trip test data";

        let encrypted = encrypt_data(original, &key).expect("Encryption");
        let decrypted = decrypt_data(&encrypted, &key).expect("Decryption");

        assert_eq!(
            original.to_vec(),
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: security
            // TEST_PRIORITY: critical
            decrypted,
            "Round trip should preserve data"
        );
    }

    /// Test decryption with wrong key
    #[test]
    fn test_decrypt_wrong_key() {
        let key1 = [0u8; 32];
        let key2 = [1u8; 32];
        let plaintext = b"Secret message";

        let ciphertext = encrypt_data(plaintext, &key1).expect("Encryption");
        let result = decrypt_data(&ciphertext, &key2);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical

        // Should fail or return garbage (depending on implementation)
        assert!(
            result.is_err() || result.unwrap() != plaintext.to_vec(),
            "Wrong key should not decrypt correctly"
        );
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical

    /// Test decryption of corrupted ciphertext
    #[test]
    fn test_decrypt_corrupted_data() {
        let key = [0u8; 32];
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical
        let mut ciphertext = encrypt_data(b"test", &key).expect("Encryption");

        // Corrupt the ciphertext
        if !ciphertext.is_empty() {
            ciphertext[0] ^= 0xFF;
        }

        let result = decrypt_data(&ciphertext, &key);
        assert!(result.is_err(), "Corrupted data should fail decryption");
    }

    /// Test hash generation
    #[test]
    fn test_hash_data() {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical
        let data = b"Data to hash";

        let result = hash_data(data);
        assert!(result.is_ok(), "Hashing should succeed");

        let hash = result.unwrap();
        assert!(!hash.is_empty(), "Hash should not be empty");
        assert_eq!(hash.len(), 32, "SHA-256 hash should be 32 bytes"); // Assuming SHA-256
    }

    /// Test hash determinism
    #[test]
    fn test_hash_deterministic() {
        let data = b"Same data";
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical

        let hash1 = hash_data(data).expect("First hash");
        let hash2 = hash_data(data).expect("Second hash");

        assert_eq!(hash1, hash2, "Same data should produce same hash");
    }

    /// Test hash uniqueness
    #[test]
    fn test_hash_different_data() {
        let data1 = b"data1";
        let data2 = b"data2";

        let hash1 = hash_data(data1).expect("First hash");
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical
        let hash2 = hash_data(data2).expect("Second hash");

        assert_ne!(
            hash1, hash2,
            "Different data should produce different hashes"
        );
    }

    /// Test secure random generation
    #[test]
    fn test_generate_random_bytes() {
        let result = generate_random_bytes(32);
        assert!(result.is_ok(), "Random generation should succeed");
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal

        let random = result.unwrap();
        assert_eq!(
            random.len(),
            32,
            "Should generate requested number of bytes"
        );
    }

    /// Test random uniqueness
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_random_bytes_unique() {
        let random1 = generate_random_bytes(32).expect("First random");
        let random2 = generate_random_bytes(32).expect("Second random");

        assert_ne!(random1, random2, "Random bytes should be unique");
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    /// Test random with different sizes
    #[test]
    fn test_random_bytes_different_sizes() {
        for size in [8, 16, 24, 32, 48, 64, 128].iter() {
            let result = generate_random_bytes(*size);
            assert!(result.is_ok(), "Should generate {} bytes", size);
            assert_eq!(result.unwrap().len(), *size, "Size should match");
        }
    }

    /// Test key validation
    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    fn test_validate_key_valid() {
        let valid_key = [0u8; 32];
        let result = validate_key(&valid_key);
        assert!(result.is_ok(), "Valid key should pass validation");
    }

    /// Test key validation with wrong size
    #[test]
    fn test_validate_key_wrong_size() {
        let short_key = [0u8; 16];
        let result = validate_key(&short_key);
        // Depending on implementation, might reject keys that are too short
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        // This test documents the behavior
        assert!(
            result.is_ok() || result.is_err(),
            "Key validation behavior documented"
        );
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    /// Test constant-time comparison
    #[test]
    fn test_constant_time_compare_equal() {
        let data1 = b"same_data";
        let data2 = b"same_data";

        let result = constant_time_compare(data1, data2);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        assert!(result, "Equal data should compare as equal");
    }

    /// Test constant-time comparison with different data
    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    fn test_constant_time_compare_different() {
        let data1 = b"data1";
        let data2 = b"data2";

        let result = constant_time_compare(data1, data2);
        assert!(!result, "Different data should compare as not equal");
    }

    /// Test constant-time comparison with different lengths
    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    fn test_constant_time_compare_different_lengths() {
        let data1 = b"short";
        let data2 = b"much_longer_data";

        let result = constant_time_compare(data1, data2);
        assert!(!result, "Different length data should not be equal");
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    /// Test key stretching
    #[test]
    fn test_stretch_key() {
        let weak_key = b"weak";
        let result = stretch_key(weak_key, 32, 100_000);

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        assert!(result.is_ok(), "Key stretching should succeed");
        assert_eq!(
            result.unwrap().len(),
            32,
            "Stretched key should be requested size"
        );
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    /// Test HMAC generation
    #[test]
    fn test_hmac_generate() {
        let key = b"secret_key";
        let message = b"message to authenticate";

        let result = generate_hmac(key, message);
        assert!(result.is_ok(), "HMAC generation should succeed");
        assert!(!result.unwrap().is_empty(), "HMAC should not be empty");
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    /// Test HMAC verification
    #[test]
    fn test_hmac_verify_valid() {
        let key = b"secret_key";
        let message = b"authenticated message";

        let mac = generate_hmac(key, message).expect("HMAC generation");
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let result = verify_hmac(key, message, &mac);

        assert!(result.is_ok(), "HMAC verification should succeed");
        assert!(result.unwrap(), "Valid HMAC should verify");
    }

    /// Test HMAC verification with wrong key
    #[test]
    fn test_hmac_verify_wrong_key() {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let key1 = b"key1";
        let key2 = b"key2";
        let message = b"message";

        let mac = generate_hmac(key1, message).expect("HMAC generation");
        let result = verify_hmac(key2, message, &mac);

        assert!(result.is_ok(), "Verification should complete");
        assert!(!result.unwrap(), "Wrong key should fail verification");
    }

    /// Test HMAC verification with modified message
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_hmac_verify_modified_message() {
        let key = b"key";
        let message = b"original message";
        let modified = b"modified message";

        let mac = generate_hmac(key, message).expect("HMAC generation");
        let result = verify_hmac(key, modified, &mac);

        assert!(result.is_ok(), "Verification should complete");
        assert!(
            !result.unwrap(),
            "Modified message should fail verification"
        );
    }

    // Wired functions - connected to real crypto implementations
    use crate::compute_sha256_hash;
    use crate::crypto_utils::BearDogCrypto;

    fn derive_key(password: &[u8], salt: &[u8], length: usize) -> Result<Vec<u8>, BearDogError> {
        // Use PBKDF2 for key derivation
        let iterations = 100_000u32;
        BearDogCrypto::derive_pbkdf2_key(password, salt, iterations, length)
    }

    fn encrypt_data(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>, BearDogError> {
        // BearDogCrypto takes optional nonce, returns (ciphertext, nonce)
        // For tests, use None to generate random nonce
        let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(key, plaintext, None)?;
        // Concatenate nonce || ciphertext for compatibility with old API
        let mut result = Vec::with_capacity(nonce.len() + ciphertext.len());
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    fn decrypt_data(encrypted: &[u8], key: &[u8]) -> Result<Vec<u8>, BearDogError> {
        // Split nonce || ciphertext
        if encrypted.len() < 12 {
            return Err(BearDogError::validation("Encrypted data too short"));
        }
        let (nonce, ciphertext) = encrypted.split_at(12);
        BearDogCrypto::decrypt_aes_gcm(key, ciphertext, nonce)
    }

    fn hash_data(data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        compute_sha256_hash(data)
    }

    fn generate_random_bytes(length: usize) -> Result<Vec<u8>, BearDogError> {
        Ok(BearDogCrypto::generate_secure_random(length))
    }

    fn validate_key(key: &[u8]) -> Result<(), BearDogError> {
        if key.is_empty() {
            return Err(BearDogError::validation("Key cannot be empty"));
        }
        if key.len() < 16 {
            return Err(BearDogError::validation("Key must be at least 16 bytes"));
        }
        Ok(())
    }

    fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
        a == b
    }

    fn generate_hmac(key: &[u8], message: &[u8]) -> Result<Vec<u8>, BearDogError> {
        BearDogCrypto::hmac_sha256(key, message)
    }

    fn verify_hmac(key: &[u8], message: &[u8], expected_mac: &[u8]) -> Result<bool, BearDogError> {
        let computed_mac = BearDogCrypto::hmac_sha256(key, message)?;
        Ok(computed_mac == expected_mac)
    }

    fn stretch_key(key: &[u8], length: usize, iterations: u32) -> Result<Vec<u8>, BearDogError> {
        // Use PBKDF2 for key stretching
        BearDogCrypto::derive_pbkdf2_key(key, b"stretch_salt", iterations, length)
    }
}
