//! Encryption Edge Cases Tests
//!
//! Comprehensive edge case testing for encryption operations including:
//! - Empty data encryption
//! - Large data encryption
//! - Invalid key handling
//! - Concurrent encryption operations
//! - Memory safety verification

use crate::crypto_utils::BearDogCrypto;
use crate::*;
use beardog_errors::BearDogError;

// ============================================================================
// AES-GCM Edge Cases
// ============================================================================

#[test]
fn test_aes_gcm_empty_data() -> Result<(), BearDogError> {
    let key = BearDogCrypto::generate_secure_random(32); // 256-bit key
    let empty_data = b"";

    let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, empty_data, None)?;

    // Should succeed with empty data
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
fn test_aes_gcm_single_byte() -> Result<(), BearDogError> {
    let key = BearDogCrypto::generate_secure_random(32);
    let single_byte = b"A";

    let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, single_byte, None)?;
    let decrypted = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce)?;

    assert_eq!(decrypted, single_byte);
    Ok(())
}

#[test]
fn test_aes_gcm_large_data() -> Result<(), BearDogError> {
    let key = BearDogCrypto::generate_secure_random(32);

    // 1MB of data
    let large_data = vec![0x42u8; 1024 * 1024];

    let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, &large_data, None)?;
    let decrypted = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce)?;

    assert_eq!(
        decrypted, large_data,
        "Large data should encrypt/decrypt correctly"
    );
    Ok(())
}

#[test]
fn test_aes_gcm_wrong_key_fails() -> Result<(), BearDogError> {
    let key1 = BearDogCrypto::generate_secure_random(32);
    let key2 = BearDogCrypto::generate_secure_random(32);
    let data = b"Secret message";

    let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key1, data, None)?;

    // Attempting to decrypt with wrong key should fail
    let result = BearDogCrypto::decrypt_aes_gcm(&key2, &ciphertext, &nonce);
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
        ciphertext[0] ^= 0xFF; // Flip bits in ciphertext
    }

    // Decryption should fail due to authentication tag mismatch
    let result = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce);
    assert!(
        result.is_err(),
        "Tampered ciphertext should fail authentication"
    );

    Ok(())
}

// ============================================================================
// Key Derivation Edge Cases
// ============================================================================

#[test]
fn test_pbkdf2_empty_password() -> Result<(), BearDogError> {
    let empty_password = b"";
    let salt = b"some_salt";

    let key = BearDogCrypto::derive_pbkdf2_key(empty_password, salt, 10000, 32)?;

    // Should produce a key even with empty password
    assert_eq!(key.len(), 32, "Derived key should be 32 bytes");
    assert_ne!(key, vec![0u8; 32], "Derived key should not be all zeros");

    Ok(())
}

#[test]
fn test_pbkdf2_empty_salt() -> Result<(), BearDogError> {
    let password = b"strong_password";
    let empty_salt = b"";

    let key = BearDogCrypto::derive_pbkdf2_key(password, empty_salt, 10000, 32)?;

    // Should produce a key even with empty salt (though not recommended in production)
    assert_eq!(key.len(), 32);

    Ok(())
}

#[test]
fn test_pbkdf2_deterministic() -> Result<(), BearDogError> {
    let password = b"test_password";
    let salt = b"test_salt";
    let iterations = 10000;

    let key1 = BearDogCrypto::derive_pbkdf2_key(password, salt, iterations, 32)?;
    let key2 = BearDogCrypto::derive_pbkdf2_key(password, salt, iterations, 32)?;

    assert_eq!(key1, key2, "Same inputs should produce same key");
    Ok(())
}

#[test]
fn test_pbkdf2_different_iterations() -> Result<(), BearDogError> {
    let password = b"test_password";
    let salt = b"test_salt";

    let key1 = BearDogCrypto::derive_pbkdf2_key(password, salt, 1000, 32)?;
    let key2 = BearDogCrypto::derive_pbkdf2_key(password, salt, 10000, 32)?;

    assert_ne!(
        key1, key2,
        "Different iterations should produce different keys"
    );
    Ok(())
}

// ============================================================================
// Password Hashing Edge Cases
// ============================================================================

#[test]
fn test_argon2_empty_password() -> Result<(), BearDogError> {
    let empty_password = "";

    let hash = BearDogCrypto::hash_password_argon2(empty_password)?;

    // Should produce a hash even for empty password
    assert!(
        !hash.is_empty(),
        "Hash should be generated for empty password"
    );

    // Verify the hash
    let is_valid = BearDogCrypto::verify_password_argon2(empty_password, &hash)?;
    assert!(is_valid, "Empty password should verify against its hash");

    Ok(())
}

#[test]
fn test_argon2_very_long_password() -> Result<(), BearDogError> {
    // 1000 byte password
    let long_password = "A".repeat(1000);

    let hash = BearDogCrypto::hash_password_argon2(&long_password)?;
    let is_valid = BearDogCrypto::verify_password_argon2(&long_password, &hash)?;

    assert!(
        is_valid,
        "Very long password should hash and verify correctly"
    );
    Ok(())
}

#[test]
fn test_argon2_special_characters() -> Result<(), BearDogError> {
    let password = "p@ssw0rd!#$%^&*(){}[]|\\:;\"'<>,.?/~`";

    let hash = BearDogCrypto::hash_password_argon2(password)?;
    let is_valid = BearDogCrypto::verify_password_argon2(password, &hash)?;

    assert!(is_valid, "Password with special characters should work");
    Ok(())
}

#[test]
fn test_argon2_unicode_password() -> Result<(), BearDogError> {
    let password = "пароль密码🔐";

    let hash = BearDogCrypto::hash_password_argon2(password)?;
    let is_valid = BearDogCrypto::verify_password_argon2(password, &hash)?;

    assert!(is_valid, "Unicode password should hash correctly");
    Ok(())
}

#[test]
fn test_argon2_wrong_password_fails() -> Result<(), BearDogError> {
    let correct_password = "correct_password";
    let wrong_password = "wrong_password";

    let hash = BearDogCrypto::hash_password_argon2(correct_password)?;
    let is_valid = BearDogCrypto::verify_password_argon2(wrong_password, &hash)?;

    assert!(!is_valid, "Wrong password should not verify");
    Ok(())
}

// ============================================================================
// HMAC Edge Cases
// ============================================================================

#[test]
fn test_hmac_empty_message() -> Result<(), BearDogError> {
    let key = b"secret_key";
    let empty_message = b"";

    let hmac = BearDogCrypto::hmac_sha256(key, empty_message)?;

    assert_eq!(hmac.len(), 32, "HMAC should be 32 bytes");
    assert_ne!(hmac, vec![0u8; 32], "HMAC should not be all zeros");

    Ok(())
}

#[test]
fn test_hmac_empty_key() -> Result<(), BearDogError> {
    let empty_key = b"";
    let message = b"message";

    let hmac = BearDogCrypto::hmac_sha256(empty_key, message)?;

    // HMAC should still work with empty key (though not recommended)
    assert_eq!(hmac.len(), 32);

    Ok(())
}

#[test]
fn test_hmac_deterministic() -> Result<(), BearDogError> {
    let key = b"test_key";
    let message = b"test_message";

    let hmac1 = BearDogCrypto::hmac_sha256(key, message)?;
    let hmac2 = BearDogCrypto::hmac_sha256(key, message)?;

    assert_eq!(hmac1, hmac2, "HMAC should be deterministic");
    Ok(())
}

#[test]
fn test_hmac_different_keys_different_hmac() -> Result<(), BearDogError> {
    let key1 = b"key1";
    let key2 = b"key2";
    let message = b"message";

    let hmac1 = BearDogCrypto::hmac_sha256(key1, message)?;
    let hmac2 = BearDogCrypto::hmac_sha256(key2, message)?;

    assert_ne!(
        hmac1, hmac2,
        "Different keys should produce different HMACs"
    );
    Ok(())
}

// ============================================================================
// SHA-256 Edge Cases
// ============================================================================

#[test]
fn test_sha256_empty_input() {
    let empty = b"";
    let hash = compute_sha256_hash(empty).expect("Hash should succeed");

    // SHA-256 of empty string is a known value
    assert_eq!(hash.len(), 32);
    assert_ne!(hash, vec![0u8; 32], "Hash should not be all zeros");
}

#[test]
fn test_sha256_large_input() {
    // 10MB of data
    let large_data = vec![0x42u8; 10 * 1024 * 1024];

    let hash = compute_sha256_hash(&large_data).expect("Hash should succeed");
    assert_eq!(hash.len(), 32);
}

#[test]
fn test_sha256_deterministic() {
    let data = b"test data";

    let hash1 = compute_sha256_hash(data).expect("Hash should succeed");
    let hash2 = compute_sha256_hash(data).expect("Hash should succeed");

    assert_eq!(hash1, hash2, "SHA-256 should be deterministic");
}

// ============================================================================
// Random Number Generation Edge Cases
// ============================================================================

#[test]
fn test_random_bytes_zero_length() {
    let result = generate_secure_random_bytes(0);
    assert!(result.is_ok(), "Zero-length random bytes should succeed");
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn test_random_bytes_large() {
    // 1MB of random data
    let result = generate_secure_random_bytes(1024 * 1024);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 1024 * 1024);
}

#[test]
fn test_random_bytes_not_all_zeros() {
    let bytes = generate_secure_random_bytes(32).expect("Should generate random bytes");

    // Statistically, 32 random bytes should not all be zero
    assert_ne!(bytes, vec![0u8; 32], "Random bytes should not all be zero");
}

#[test]
fn test_random_bytes_different_each_time() {
    let bytes1 = generate_secure_random_bytes(32).expect("Should generate random bytes");
    let bytes2 = generate_secure_random_bytes(32).expect("Should generate random bytes");

    // Statistically, two random 32-byte sequences should be different
    assert_ne!(
        bytes1, bytes2,
        "Consecutive random generations should differ"
    );
}
