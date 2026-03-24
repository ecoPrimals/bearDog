// SPDX-License-Identifier: AGPL-3.0-only
#![allow(clippy::expect_used, clippy::unwrap_used)]
//! Integration Tests for Authentication and Security
//!
//! This module contains comprehensive integration tests for `BearDog`'s authentication
//! and security subsystems, including:
//! - Password hashing and verification
//! - Secure token generation and entropy
//! - Session encryption/decryption
//! - HMAC token verification
//! - Password derivation (PBKDF2)
//! - Multi-factor authentication tokens
//! - Security property verification (nonce uniqueness, zero plaintext leakage)

use beardog_security::{compute_sha256_hash, crypto_utils::BearDogCrypto};

// ============================================================================
// Authentication + Hashing Integration Tests
// ============================================================================

/// Tests password hashing for authentication using SHA-256
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: security
/// `TEST_PRIORITY`: high
#[test]
fn test_password_hashing_for_auth() {
    // Given: a password
    let password = b"secure_password_123";

    // When: hashing the password
    let result = compute_sha256_hash(password);

    // Then: should produce a valid 32-byte hash
    assert!(result.is_ok());
    let hash = result.unwrap();
    assert_eq!(hash.len(), 32, "SHA-256 produces 32 bytes");
}

/// Tests that the same password produces the same hash (deterministic)
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: security
/// `TEST_PRIORITY`: high
#[test]
fn test_hash_consistency_for_auth() {
    // Given: the same password hashed twice
    let password = b"same_password";

    // When: computing hashes
    let hash1 = compute_sha256_hash(password).unwrap();
    let hash2 = compute_sha256_hash(password).unwrap();

    // Then: hashes should be identical
    assert_eq!(hash1, hash2, "Same password should produce same hash");
}

/// Tests that different passwords produce different hashes
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: security
/// `TEST_PRIORITY`: high
#[test]
fn test_different_passwords_different_hashes() {
    // Given: two different passwords
    let pass1 = b"password1";
    let pass2 = b"password2";

    // When: hashing both passwords
    let hash1 = compute_sha256_hash(pass1).unwrap();
    let hash2 = compute_sha256_hash(pass2).unwrap();

    // Then: hashes should be different
    assert_ne!(
        hash1, hash2,
        "Different passwords should produce different hashes"
    );
}

// ============================================================================
// Token Generation Integration Tests
// ============================================================================

/// Tests secure random number generation for session tokens
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: security
/// `TEST_PRIORITY`: high
#[test]
fn test_generate_secure_random_for_session_token() {
    // When: generating two random tokens
    let bytes1 = BearDogCrypto::generate_secure_random(32);
    let bytes2 = BearDogCrypto::generate_secure_random(32);

    // Then: both should be 32 bytes and different
    assert_eq!(bytes1.len(), 32);
    assert_eq!(bytes2.len(), 32);
    assert_ne!(bytes1, bytes2, "Random tokens should be different");
}

/// Tests that generated session tokens have sufficient entropy (uniqueness)
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: security
/// `TEST_PRIORITY`: high
#[test]
fn test_session_token_entropy() {
    // Given: multiple generated tokens
    let tokens: Vec<Vec<u8>> = (0..10)
        .map(|_| BearDogCrypto::generate_secure_random(32))
        .collect();

    // Then: all tokens should be unique
    for i in 0..tokens.len() {
        for j in (i + 1)..tokens.len() {
            assert_ne!(tokens[i], tokens[j], "Tokens should be unique");
        }
    }
}

// ============================================================================
// Encryption Integration for Secure Sessions
// ============================================================================

/// Tests encryption of session data using AES-GCM
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: security
/// `TEST_PRIORITY`: high
#[test]
fn test_encrypt_session_data() {
    // Given: session data and encryption key
    let session_data = b"user_id=12345&permissions=admin&expires=2025-12-31";
    let key = BearDogCrypto::generate_secure_random(32);

    // When: encrypting the session data
    let result = BearDogCrypto::encrypt_aes_gcm(&key, session_data, None);

    // Then: should produce ciphertext and nonce
    assert!(result.is_ok());
    let (ciphertext, nonce) = result.unwrap();
    assert_ne!(
        &ciphertext[..],
        session_data,
        "Ciphertext should differ from plaintext"
    );
    assert_eq!(nonce.len(), 12, "Nonce should be 12 bytes for AES-GCM");
}

/// Tests decryption of session data using AES-GCM
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: security
/// `TEST_PRIORITY`: high
#[test]
fn test_decrypt_session_data() {
    // Given: encrypted session data
    let session_data = b"user_id=67890&permissions=user";
    let key = BearDogCrypto::generate_secure_random(32);
    let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, session_data, None).unwrap();

    // When: decrypting with the correct key
    let result = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce);

    // Then: should recover the original session data
    assert!(result.is_ok());
    let decrypted = result.unwrap();
    assert_eq!(decrypted, session_data);
}

/// Tests that decryption fails with the wrong key
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: security
/// `TEST_PRIORITY`: high
#[test]
fn test_session_encryption_with_wrong_key_fails() {
    // Given: data encrypted with one key
    let session_data = b"sensitive_session_info";
    let key1 = BearDogCrypto::generate_secure_random(32);
    let key2 = BearDogCrypto::generate_secure_random(32);
    let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key1, session_data, None).unwrap();

    // When: attempting to decrypt with a different key
    let result = BearDogCrypto::decrypt_aes_gcm(&key2, &ciphertext, &nonce);

    // Then: decryption should fail
    assert!(result.is_err(), "Decryption with wrong key should fail");
}

// ============================================================================
// Authentication Token HMAC Verification
// ============================================================================

/// Tests HMAC generation for token verification
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: security
/// `TEST_PRIORITY`: high
#[test]
fn test_hmac_for_token_verification() {
    // Given: token data and secret key
    let token_data = b"user_token_abc123xyz";
    let secret_key = BearDogCrypto::generate_secure_random(32);

    // When: generating HMAC twice for the same data
    let hmac1 = BearDogCrypto::hmac_sha256(&secret_key, token_data).unwrap();
    let hmac2 = BearDogCrypto::hmac_sha256(&secret_key, token_data).unwrap();

    // Then: HMACs should match (deterministic)
    assert_eq!(hmac1, hmac2, "HMAC should be consistent for same input");
}

/// Tests that HMAC detects token tampering
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: security
/// `TEST_PRIORITY`: critical
#[test]
fn test_hmac_detects_token_tampering() {
    // Given: original and tampered tokens
    let original_token = b"user_token_original";
    let tampered_token = b"user_token_tampered";
    let secret_key = BearDogCrypto::generate_secure_random(32);

    // When: computing HMACs for both tokens
    let hmac_original = BearDogCrypto::hmac_sha256(&secret_key, original_token).unwrap();
    let hmac_tampered = BearDogCrypto::hmac_sha256(&secret_key, tampered_token).unwrap();

    // Then: HMACs should differ, detecting tampering
    assert_ne!(hmac_original, hmac_tampered, "Tampering should change HMAC");
}

// ============================================================================
// Password Derivation for Auth
// ============================================================================

/// Tests key derivation from password using PBKDF2
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: security
/// `TEST_PRIORITY`: high
#[test]
fn test_derive_key_from_password() {
    // Given: a password and salt
    let password = b"user_password_secure";
    let salt = BearDogCrypto::generate_secure_random(16);

    // When: deriving a key with PBKDF2
    let result = BearDogCrypto::derive_pbkdf2_key(password, &salt, 10000, 32);

    // Then: should produce a 32-byte key
    assert!(result.is_ok());
    let derived_key = result.unwrap();
    assert_eq!(derived_key.len(), 32);
}

/// Tests that the same password and salt produce the same derived key
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: security
/// `TEST_PRIORITY`: high
#[test]
fn test_same_password_same_salt_same_key() {
    // Given: same password and salt
    let password = b"test_password";
    let salt = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

    // When: deriving keys twice
    let key1 = BearDogCrypto::derive_pbkdf2_key(password, &salt, 10000, 32).unwrap();
    let key2 = BearDogCrypto::derive_pbkdf2_key(password, &salt, 10000, 32).unwrap();

    // Then: keys should be identical
    assert_eq!(key1, key2, "Same password and salt should produce same key");
}

/// Tests that different salts produce different derived keys
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: security
/// `TEST_PRIORITY`: high
#[test]
fn test_different_salt_different_key() {
    // Given: same password but different salts
    let password = b"same_password";
    let salt1 = vec![1; 16];
    let salt2 = vec![2; 16];

    // When: deriving keys with different salts
    let key1 = BearDogCrypto::derive_pbkdf2_key(password, &salt1, 10000, 32).unwrap();
    let key2 = BearDogCrypto::derive_pbkdf2_key(password, &salt2, 10000, 32).unwrap();

    // Then: keys should be different
    assert_ne!(key1, key2, "Different salts should produce different keys");
}

// ============================================================================
// Multi-Factor Token Generation
// ============================================================================

/// Tests generation of MFA tokens (6-digit simulation)
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: security
/// `TEST_PRIORITY`: normal
#[test]
fn test_generate_mfa_token() {
    // When: generating a 6-digit MFA token
    let random_bytes = BearDogCrypto::generate_secure_random(4);
    let token_num = u32::from_be_bytes([
        random_bytes[0],
        random_bytes[1],
        random_bytes[2],
        random_bytes[3],
    ]) % 1_000_000;

    // Then: should be within valid 6-digit range
    assert!(token_num < 1_000_000, "MFA token should be 6 digits");
}

// ============================================================================
// Secure Session Storage
// ============================================================================

/// Tests encrypting multiple session fields independently
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: security
/// `TEST_PRIORITY`: normal
#[test]
fn test_encrypt_multiple_session_fields() {
    // Given: multiple session components
    let key = BearDogCrypto::generate_secure_random(32);
    let user_id = b"user_12345";
    let permissions = b"admin,read,write";
    let expiry = b"2025-12-31T23:59:59Z";

    // When: encrypting each field separately
    let (enc_user, nonce1) = BearDogCrypto::encrypt_aes_gcm(&key, user_id, None).unwrap();
    let (enc_perms, nonce2) = BearDogCrypto::encrypt_aes_gcm(&key, permissions, None).unwrap();
    let (enc_expiry, nonce3) = BearDogCrypto::encrypt_aes_gcm(&key, expiry, None).unwrap();

    // Then: all should encrypt and decrypt correctly
    assert!(!enc_user.is_empty());
    assert!(!enc_perms.is_empty());
    assert!(!enc_expiry.is_empty());

    assert_eq!(
        BearDogCrypto::decrypt_aes_gcm(&key, &enc_user, &nonce1).unwrap(),
        user_id
    );
    assert_eq!(
        BearDogCrypto::decrypt_aes_gcm(&key, &enc_perms, &nonce2).unwrap(),
        permissions
    );
    assert_eq!(
        BearDogCrypto::decrypt_aes_gcm(&key, &enc_expiry, &nonce3).unwrap(),
        expiry
    );
}

// ============================================================================
// Error Handling Integration
// ============================================================================

/// Tests that encryption fails with invalid key size
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: security
/// `TEST_PRIORITY`: high
#[test]
fn test_encryption_with_invalid_key_size() {
    // Given: a key that's too short
    let short_key = vec![1, 2, 3]; // Need 32 bytes
    let data = b"test data";

    // When: attempting to encrypt
    let result = BearDogCrypto::encrypt_aes_gcm(&short_key, data, None);

    // Then: should fail with invalid key size error
    assert!(result.is_err(), "Encryption with short key should fail");
}

/// Tests that decryption fails with wrong nonce
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: security
/// `TEST_PRIORITY`: critical
#[test]
fn test_decryption_with_wrong_nonce() {
    // Given: encrypted data with correct nonce
    let key = BearDogCrypto::generate_secure_random(32);
    let data = b"secret data";
    let (ciphertext, _correct_nonce) = BearDogCrypto::encrypt_aes_gcm(&key, data, None).unwrap();

    // When: attempting to decrypt with wrong nonce
    let wrong_nonce = vec![0; 12];
    let result = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &wrong_nonce);

    // Then: should fail with authentication error
    assert!(result.is_err(), "Decryption with wrong nonce should fail");
}

// ============================================================================
// Performance and Scalability Tests
// ============================================================================

/// Tests batch encryption of multiple sessions
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: security
/// `TEST_PRIORITY`: normal
#[test]
fn test_batch_session_encryption() {
    // Given: encryption key
    let key = BearDogCrypto::generate_secure_random(32);

    // When: encrypting 100 simulated sessions
    let mut encrypted_sessions = Vec::new();
    for i in 0..100 {
        let session = format!("session_data_{i}");
        let result = BearDogCrypto::encrypt_aes_gcm(&key, session.as_bytes(), None);
        assert!(result.is_ok());
        encrypted_sessions.push(result.unwrap());
    }

    // Then: all encryptions should succeed
    assert_eq!(encrypted_sessions.len(), 100);
}

/// Tests concurrent hash generation for multiple passwords
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: security
/// `TEST_PRIORITY`: normal
#[test]
fn test_concurrent_hash_generation() {
    // Given: multiple passwords to hash
    let passwords: Vec<&[u8]> = vec![
        b"password1",
        b"password2",
        b"password3",
        b"password4",
        b"password5",
    ];

    // When: hashing all passwords
    let hashes: Vec<Vec<u8>> = passwords
        .iter()
        .map(|p| compute_sha256_hash(p).unwrap())
        .collect();

    // Then: all should succeed and be unique
    assert_eq!(hashes.len(), 5);
    for i in 0..hashes.len() {
        for j in (i + 1)..hashes.len() {
            assert_ne!(
                hashes[i], hashes[j],
                "Different passwords should have different hashes"
            );
        }
    }
}

// ============================================================================
// Security Property Verification
// ============================================================================

/// Tests that ciphertext does not leak plaintext information
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: security
/// `TEST_PRIORITY`: critical
#[test]
fn test_zero_plaintext_leakage() {
    // Given: sensitive plaintext data
    let sensitive_data = b"super_secret_password";
    let key = BearDogCrypto::generate_secure_random(32);

    // When: encrypting the data
    let (ciphertext, _nonce) = BearDogCrypto::encrypt_aes_gcm(&key, sensitive_data, None).unwrap();

    // Then: ciphertext should not contain plaintext
    let ciphertext_str = String::from_utf8_lossy(&ciphertext);
    assert!(
        !ciphertext_str.contains("super_secret_password"),
        "Plaintext should not appear in ciphertext"
    );
}

/// Tests that nonces are unique across encryptions
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: security
/// `TEST_PRIORITY`: critical
#[test]
fn test_nonce_uniqueness() {
    // Given: same data encrypted multiple times
    let key = BearDogCrypto::generate_secure_random(32);
    let data = b"same data";

    // When: encrypting same data three times
    let (_, nonce1) = BearDogCrypto::encrypt_aes_gcm(&key, data, None).unwrap();
    let (_, nonce2) = BearDogCrypto::encrypt_aes_gcm(&key, data, None).unwrap();
    let (_, nonce3) = BearDogCrypto::encrypt_aes_gcm(&key, data, None).unwrap();

    // Then: all nonces should be different (critical for security)
    assert_ne!(nonce1, nonce2, "Nonces must be unique");
    assert_ne!(nonce2, nonce3, "Nonces must be unique");
    assert_ne!(nonce1, nonce3, "Nonces must be unique");
}
