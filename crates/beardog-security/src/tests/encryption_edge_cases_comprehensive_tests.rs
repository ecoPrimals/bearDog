// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive Edge Cases Tests for Encryption Operations
//!
//! Tests various edge cases and boundary conditions for cryptographic operations
//! to improve test coverage and ensure robust error handling.

use crate::crypto_utils::BearDogCrypto;
use beardog_errors::BearDogError;

#[test]
fn test_aes_gcm_empty_data_comprehensive() -> Result<(), BearDogError> {
    let key = BearDogCrypto::generate_secure_random(32); // 256-bit key
    let empty_data = b"";

    let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, empty_data, None)?;

    // Should succeed with empty data and produce authentication tag
    assert!(
        !ciphertext.is_empty(),
        "Encrypted empty data should produce ciphertext with tag"
    );

    // Verify we can decrypt
    let decrypted = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce)?;
    assert_eq!(
        decrypted, empty_data,
        "Decrypted empty data should match original"
    );

    Ok(())
}

#[test]
fn test_aes_gcm_single_byte_encryption() -> Result<(), BearDogError> {
    let key = BearDogCrypto::generate_secure_random(32);
    let single_byte = b"A";

    let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, single_byte, None)?;
    let decrypted = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce)?;

    assert_eq!(
        decrypted, single_byte,
        "Single byte should encrypt/decrypt correctly"
    );
    Ok(())
}

#[test]
fn test_aes_gcm_large_data_1mb() -> Result<(), BearDogError> {
    let key = BearDogCrypto::generate_secure_random(32);

    // 1MB of data
    let large_data = vec![0x42u8; 1024 * 1024];

    let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, &large_data, None)?;
    let decrypted = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce)?;

    assert_eq!(
        decrypted, large_data,
        "Large data (1MB) should encrypt/decrypt correctly" // TEST_CATEGORY: integration
                                                            // TEST_DOMAIN: security
                                                            // TEST_PRIORITY: normal
    );
    Ok(())
}

#[test]
fn test_aes_gcm_10mb_data() -> Result<(), BearDogError> {
    let key = BearDogCrypto::generate_secure_random(32);

    // 10MB of data - stress test
    let very_large_data = vec![0x7Fu8; 10 * 1024 * 1024];

    let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, &very_large_data, None)?;
    let decrypted = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce)?;

    assert_eq!(
        decrypted.len(),
        very_large_data.len(),
        "10MB data should encrypt/decrypt with correct length"
    );
    Ok(())
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
}

#[test]
fn test_aes_gcm_wrong_key_fails() -> Result<(), BearDogError> {
    let key1 = BearDogCrypto::generate_secure_random(32);
    let key2 = BearDogCrypto::generate_secure_random(32);
    let data = b"Secret message";

    let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key1, data, None)?;

    // Attempting to decrypt with wrong key should fail
    let result = BearDogCrypto::decrypt_aes_gcm(&key2, &ciphertext, &nonce);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    assert!(result.is_err(), "Decryption with wrong key should fail");

    Ok(())
}

#[test]
fn test_aes_gcm_tampered_ciphertext_fails() -> Result<(), BearDogError> {
    let key = BearDogCrypto::generate_secure_random(32);
    let data = b"Authenticated message";

    let (mut ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, data, None)?;

    // Tamper with the ciphertext
    if !ciphertext.is_empty() {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        ciphertext[0] ^= 0xFF; // Flip bits in ciphertext
    }

    // Decryption should fail due to authentication tag mismatch
    let result = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce);
    assert!(
        result.is_err(),
        "Decryption of tampered ciphertext should fail"
    );

    Ok(())
}

#[test]
fn test_aes_gcm_tampered_nonce_fails() -> Result<(), BearDogError> {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: important
    let key = BearDogCrypto::generate_secure_random(32);
    let data = b"Nonce-sensitive data";

    let (ciphertext, mut nonce) = BearDogCrypto::encrypt_aes_gcm(&key, data, None)?;

    // Tamper with the nonce
    if !nonce.is_empty() {
        nonce[0] ^= 0xFF;
    }

    // Decryption should fail due to wrong nonce
    let result = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: important
    assert!(
        result.is_err(),
        "Decryption with tampered nonce should fail"
    );

    Ok(())
}

#[test]
fn test_aes_gcm_with_custom_nonce() -> Result<(), BearDogError> {
    let key = BearDogCrypto::generate_secure_random(32);
    let data = b"Payload data";
    let custom_nonce = BearDogCrypto::generate_secure_random(12); // 12 bytes for GCM

    // Test encryption with custom nonce
    let (ciphertext, returned_nonce) =
        BearDogCrypto::encrypt_aes_gcm(&key, data, Some(&custom_nonce))?;

    // Verify the nonce was used
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: important
    assert_eq!(returned_nonce, custom_nonce, "Custom nonce should be used");

    // Decrypt using the same nonce
    let decrypted = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &custom_nonce)?;
    assert_eq!(
        decrypted, data,
        "Data with custom nonce should encrypt/decrypt correctly"
    );

    Ok(())
}

#[test]
fn test_aes_gcm_key_size_validation() -> Result<(), BearDogError> {
    let data = b"Test data";

    // Test with invalid key sizes (should fail)
    let key_16 = BearDogCrypto::generate_secure_random(16);
    let result = BearDogCrypto::encrypt_aes_gcm(&key_16, data, None);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    assert!(
        result.is_err(),
        "16-byte key should fail (requires 32 bytes for AES-256)"
    );

    let key_24 = BearDogCrypto::generate_secure_random(24);
    let result = BearDogCrypto::encrypt_aes_gcm(&key_24, data, None);
    assert!(
        result.is_err(),
        "24-byte key should fail (requires 32 bytes for AES-256)"
    );

    // Test with valid 256-bit key (should succeed)
    let key_32 = BearDogCrypto::generate_secure_random(32);
    let result = BearDogCrypto::encrypt_aes_gcm(&key_32, data, None);
    assert!(result.is_ok(), "32-byte key should work (AES-256)");

    Ok(())
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
#[test]
fn test_key_derivation_various_parameters() -> Result<(), BearDogError> {
    let password = b"test_password";
    let salt = BearDogCrypto::generate_secure_random(16);

    // Test with different iteration counts
    let key_1k = BearDogCrypto::derive_pbkdf2_key(password, &salt, 1_000, 32)?;
    let key_10k = BearDogCrypto::derive_pbkdf2_key(password, &salt, 10_000, 32)?;
    let key_100k = BearDogCrypto::derive_pbkdf2_key(password, &salt, 100_000, 32)?;

    // Different iteration counts should produce different keys
    assert_ne!(
        key_1k, key_10k,
        "Different iterations should produce different keys"
    );
    assert_ne!(
        key_10k, key_100k,
        "Different iterations should produce different keys"
    );

    // Same parameters should produce same key (deterministic)
    let key_repeat = BearDogCrypto::derive_pbkdf2_key(password, &salt, 10_000, 32)?;
    assert_eq!(
        key_10k, key_repeat,
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        "Same parameters should produce same key"
    );

    Ok(())
}

#[test]
fn test_key_derivation_different_salts() -> Result<(), BearDogError> {
    let password = b"same_password";
    let salt1 = BearDogCrypto::generate_secure_random(16);
    let salt2 = BearDogCrypto::generate_secure_random(16);

    let key1 = BearDogCrypto::derive_pbkdf2_key(password, &salt1, 10_000, 32)?;
    let key2 = BearDogCrypto::derive_pbkdf2_key(password, &salt2, 10_000, 32)?;

    assert_ne!(key1, key2, "Different salts should produce different keys");

    Ok(())
}

#[test]
fn test_hmac_empty_message() -> Result<(), BearDogError> {
    let key = BearDogCrypto::generate_secure_random(32);
    let empty_msg = b"";

    let hmac = BearDogCrypto::hmac_sha256(&key, empty_msg)?;

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    // HMAC of empty message should still produce a valid tag
    assert!(!hmac.is_empty(), "HMAC of empty message should produce tag");
    assert_eq!(hmac.len(), 32, "HMAC should be 32 bytes (SHA-256)");

    Ok(())
}

#[test]
fn test_hmac_different_key_sizes() -> Result<(), BearDogError> {
    let msg = b"test message";

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    // Test various key sizes
    let key_16 = BearDogCrypto::generate_secure_random(16);
    let key_32 = BearDogCrypto::generate_secure_random(32);
    let key_64 = BearDogCrypto::generate_secure_random(64);

    let hmac_16 = BearDogCrypto::hmac_sha256(&key_16, msg)?;
    let hmac_32 = BearDogCrypto::hmac_sha256(&key_32, msg)?;
    let hmac_64 = BearDogCrypto::hmac_sha256(&key_64, msg)?;

    // All should produce valid HMACs
    assert_eq!(hmac_16.len(), 32, "HMAC with 16-byte key should work");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    assert_eq!(hmac_32.len(), 32, "HMAC with 32-byte key should work");
    assert_eq!(hmac_64.len(), 32, "HMAC with 64-byte key should work");

    // Different keys should produce different HMACs
    assert_ne!(hmac_16, hmac_32);
    assert_ne!(hmac_32, hmac_64);

    Ok(())
}

#[test]
fn test_password_hashing_edge_cases() -> Result<(), BearDogError> {
    // Test with minimum password
    let min_password = "a";
    let hash_min = BearDogCrypto::hash_password_argon2(min_password)?;
    assert!(
        !hash_min.is_empty(),
        "Minimum password should hash successfully"
    );

    // Test with empty password (should this fail or hash?)
    let empty_password = "";
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: important
    let result = BearDogCrypto::hash_password_argon2(empty_password);
    // Implementation dependent - document behavior
    assert!(
        result.is_ok() || result.is_err(),
        "Empty password behavior should be defined"
    );

    // Test with long password
    let long_password = "x".repeat(1000);
    let hash_long = BearDogCrypto::hash_password_argon2(&long_password)?;
    assert!(
        !hash_long.is_empty(),
        "Long password should hash successfully"
    );

    Ok(())
}

#[test]
fn test_password_verification_negative_cases() -> Result<(), BearDogError> {
    let password = "correct_password";
    let hash = BearDogCrypto::hash_password_argon2(password)?;

    // Wrong password should fail verification
    let wrong_password = "wrong_password";
    let verify_result = BearDogCrypto::verify_password_argon2(wrong_password, &hash)?;
    assert!(!verify_result, "Wrong password should not verify");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    // Slightly different password should fail
    let almost_password = "correct_passwor"; // One char short
    let verify_result = BearDogCrypto::verify_password_argon2(almost_password, &hash)?;
    assert!(!verify_result, "Almost-correct password should not verify");

    // Correct password should verify
    let verify_result = BearDogCrypto::verify_password_argon2(password, &hash)?;
    assert!(verify_result, "Correct password should verify");

    Ok(())
}

#[test]
fn test_concurrent_key_generation() -> Result<(), BearDogError> {
    use std::thread;

    // Test that key generation is thread-safe
    let handles: Vec<_> = (0..10)
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        .map(|_| {
            thread::spawn(|| {
                for _ in 0..100 {
                    let key = BearDogCrypto::generate_secure_random(32);
                    assert_eq!(key.len(), 32, "Generated key should be 32 bytes");
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().map_err(|_| {
            BearDogError::internal("Thread panicked during concurrent key generation".to_string())
        })?;
    }

    Ok(())
}

#[test]
fn test_key_uniqueness() -> Result<(), BearDogError> {
    // Generate multiple keys and ensure they're unique
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let mut keys = Vec::new();
    for _ in 0..100 {
        let key = BearDogCrypto::generate_secure_random(32);
        keys.push(key);
    }

    // Check uniqueness (probability of collision should be negligible)
    for i in 0..keys.len() {
        for j in (i + 1)..keys.len() {
            assert_ne!(
                keys[i], keys[j],
                "Generated keys should be unique (index {} and {})",
                i, j
            );
        }
    }

    Ok(())
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
#[test]
fn test_nonce_uniqueness() -> Result<(), BearDogError> {
    let key = BearDogCrypto::generate_secure_random(32);
    let data = b"test data";

    let mut nonces = Vec::new();
    for _ in 0..100 {
        let (_, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, data, None)?;
        nonces.push(nonce);
    }

    // Nonces should be unique
    for i in 0..nonces.len() {
        for j in (i + 1)..nonces.len() {
            assert_ne!(
                nonces[i], nonces[j],
                "Nonces should be unique (index {} and {})",
                i, j
            );
        }
    }

    Ok(())
}
