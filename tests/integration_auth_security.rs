//! Integration tests for BearDog Authentication + Security
//!
//! Tests the integration between authentication handlers and security operations

use beardog_security::{compute_sha256_hash, crypto_utils::BearDogCrypto};

// ============================================================================
// Authentication + Hashing Integration Tests
// ============================================================================

#[test]
fn test_password_hashing_for_auth() {
    let password = b"secure_password_123";

    let result = compute_sha256_hash(password);

    assert!(result.is_ok());
    let hash = result.unwrap();

    // SHA-256 produces 32 bytes
    assert_eq!(hash.len(), 32);
}

#[test]
fn test_hash_consistency_for_auth() {
    let password = b"same_password";

    let hash1 = compute_sha256_hash(password).unwrap();
    let hash2 = compute_sha256_hash(password).unwrap();

    // Same password should produce same hash
    assert_eq!(hash1, hash2);
}

#[test]
fn test_different_passwords_different_hashes() {
    let pass1 = b"password1";
    let pass2 = b"password2";

    let hash1 = compute_sha256_hash(pass1).unwrap();
    let hash2 = compute_sha256_hash(pass2).unwrap();

    // Different passwords should produce different hashes
    assert_ne!(hash1, hash2);
}

// ============================================================================
// Token Generation Integration Tests
// ============================================================================

#[test]
fn test_generate_secure_random_for_session_token() {
    let bytes1 = BearDogCrypto::generate_secure_random(32);
    let bytes2 = BearDogCrypto::generate_secure_random(32);

    // Both should be 32 bytes
    assert_eq!(bytes1.len(), 32);
    assert_eq!(bytes2.len(), 32);

    // Should be different (astronomically unlikely to be same)
    assert_ne!(bytes1, bytes2);
}

#[test]
fn test_session_token_entropy() {
    // Generate multiple tokens to verify randomness
    let tokens: Vec<Vec<u8>> = (0..10)
        .map(|_| BearDogCrypto::generate_secure_random(32))
        .collect();

    // All tokens should be unique
    for i in 0..tokens.len() {
        for j in (i + 1)..tokens.len() {
            assert_ne!(tokens[i], tokens[j], "Tokens should be unique");
        }
    }
}

// ============================================================================
// Encryption Integration for Secure Sessions
// ============================================================================

#[test]
fn test_encrypt_session_data() {
    let session_data = b"user_id=12345&permissions=admin&expires=2025-12-31";
    let key = BearDogCrypto::generate_secure_random(32);

    let result = BearDogCrypto::encrypt_aes_gcm(&key, session_data, None);

    assert!(result.is_ok());
    let (ciphertext, nonce) = result.unwrap();

    // Ciphertext should be different from plaintext
    assert_ne!(&ciphertext[..], session_data);

    // Nonce should be 12 bytes for AES-GCM
    assert_eq!(nonce.len(), 12);
}

#[test]
fn test_decrypt_session_data() {
    let session_data = b"user_id=67890&permissions=user";
    let key = BearDogCrypto::generate_secure_random(32);

    // Encrypt
    let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, session_data, None).unwrap();

    // Decrypt
    let result = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce);

    assert!(result.is_ok());
    let decrypted = result.unwrap();

    assert_eq!(decrypted, session_data);
}

#[test]
fn test_session_encryption_with_wrong_key_fails() {
    let session_data = b"sensitive_session_info";
    let key1 = BearDogCrypto::generate_secure_random(32);
    let key2 = BearDogCrypto::generate_secure_random(32);

    // Encrypt with key1
    let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key1, session_data, None).unwrap();

    // Try to decrypt with key2 (wrong key)
    let result = BearDogCrypto::decrypt_aes_gcm(&key2, &ciphertext, &nonce);

    // Should fail with wrong key
    assert!(result.is_err());
}

// ============================================================================
// Authentication Token HMAC Verification
// ============================================================================

#[test]
fn test_hmac_for_token_verification() {
    let token_data = b"user_token_abc123xyz";
    let secret_key = BearDogCrypto::generate_secure_random(32);

    // Generate HMAC for token
    let hmac1 = BearDogCrypto::hmac_sha256(&secret_key, token_data).unwrap();

    // Regenerate HMAC to verify
    let hmac2 = BearDogCrypto::hmac_sha256(&secret_key, token_data).unwrap();

    // Should match for valid token
    assert_eq!(hmac1, hmac2);
}

#[test]
fn test_hmac_detects_token_tampering() {
    let original_token = b"user_token_original";
    let tampered_token = b"user_token_tampered";
    let secret_key = BearDogCrypto::generate_secure_random(32);

    let hmac_original = BearDogCrypto::hmac_sha256(&secret_key, original_token).unwrap();
    let hmac_tampered = BearDogCrypto::hmac_sha256(&secret_key, tampered_token).unwrap();

    // HMACs should be different, detecting tampering
    assert_ne!(hmac_original, hmac_tampered);
}

// ============================================================================
// Password Derivation for Auth
// ============================================================================

#[test]
fn test_derive_key_from_password() {
    let password = b"user_password_secure";
    let salt = BearDogCrypto::generate_secure_random(16);

    let result = BearDogCrypto::derive_pbkdf2_key(password, &salt, 10000, 32);

    assert!(result.is_ok());
    let derived_key = result.unwrap();

    assert_eq!(derived_key.len(), 32);
}

#[test]
fn test_same_password_same_salt_same_key() {
    let password = b"test_password";
    let salt = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

    let key1 = BearDogCrypto::derive_pbkdf2_key(password, &salt, 10000, 32).unwrap();
    let key2 = BearDogCrypto::derive_pbkdf2_key(password, &salt, 10000, 32).unwrap();

    assert_eq!(key1, key2);
}

#[test]
fn test_different_salt_different_key() {
    let password = b"same_password";
    let salt1 = vec![1; 16];
    let salt2 = vec![2; 16];

    let key1 = BearDogCrypto::derive_pbkdf2_key(password, &salt1, 10000, 32).unwrap();
    let key2 = BearDogCrypto::derive_pbkdf2_key(password, &salt2, 10000, 32).unwrap();

    // Different salts should produce different keys
    assert_ne!(key1, key2);
}

// ============================================================================
// Multi-Factor Token Generation
// ============================================================================

#[test]
fn test_generate_mfa_token() {
    // Generate a 6-digit MFA token (simulation)
    let random_bytes = BearDogCrypto::generate_secure_random(4);

    // Convert to number and take modulo to get 6 digits
    let token_num = u32::from_be_bytes([
        random_bytes[0],
        random_bytes[1],
        random_bytes[2],
        random_bytes[3],
    ]) % 1_000_000;

    // Should be within valid range
    assert!(token_num < 1_000_000);
}

// ============================================================================
// Secure Session Storage
// ============================================================================

#[test]
fn test_encrypt_multiple_session_fields() {
    let key = BearDogCrypto::generate_secure_random(32);

    // Encrypt different session components
    let user_id = b"user_12345";
    let permissions = b"admin,read,write";
    let expiry = b"2025-12-31T23:59:59Z";

    let (enc_user, nonce1) = BearDogCrypto::encrypt_aes_gcm(&key, user_id, None).unwrap();
    let (enc_perms, nonce2) = BearDogCrypto::encrypt_aes_gcm(&key, permissions, None).unwrap();
    let (enc_expiry, nonce3) = BearDogCrypto::encrypt_aes_gcm(&key, expiry, None).unwrap();

    // All should encrypt successfully
    assert!(!enc_user.is_empty());
    assert!(!enc_perms.is_empty());
    assert!(!enc_expiry.is_empty());

    // Decrypt and verify
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

#[test]
fn test_encryption_with_invalid_key_size() {
    let short_key = vec![1, 2, 3]; // Too short (need 32 bytes)
    let data = b"test data";

    let result = BearDogCrypto::encrypt_aes_gcm(&short_key, data, None);

    // Should fail with invalid key size
    assert!(result.is_err());
}

#[test]
fn test_decryption_with_wrong_nonce() {
    let key = BearDogCrypto::generate_secure_random(32);
    let data = b"secret data";

    let (ciphertext, _correct_nonce) = BearDogCrypto::encrypt_aes_gcm(&key, data, None).unwrap();
    let wrong_nonce = vec![0; 12]; // Wrong nonce

    let result = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &wrong_nonce);

    // Should fail with authentication error
    assert!(result.is_err());
}

// ============================================================================
// Performance and Scalability Tests
// ============================================================================

#[test]
fn test_batch_session_encryption() {
    let key = BearDogCrypto::generate_secure_random(32);

    // Encrypt 100 simulated sessions
    let mut encrypted_sessions = Vec::new();

    for i in 0..100 {
        let session = format!("session_data_{}", i);
        let result = BearDogCrypto::encrypt_aes_gcm(&key, session.as_bytes(), None);
        assert!(result.is_ok());
        encrypted_sessions.push(result.unwrap());
    }

    assert_eq!(encrypted_sessions.len(), 100);
}

#[test]
fn test_concurrent_hash_generation() {
    // Simulate concurrent password hashing
    let passwords: Vec<&[u8]> = vec![
        b"password1",
        b"password2",
        b"password3",
        b"password4",
        b"password5",
    ];

    let hashes: Vec<Vec<u8>> = passwords
        .iter()
        .map(|p| compute_sha256_hash(p).unwrap())
        .collect();

    // All should succeed
    assert_eq!(hashes.len(), 5);

    // All should be unique (different passwords)
    for i in 0..hashes.len() {
        for j in (i + 1)..hashes.len() {
            assert_ne!(hashes[i], hashes[j]);
        }
    }
}

// ============================================================================
// Security Property Verification
// ============================================================================

#[test]
fn test_zero_plaintext_leakage() {
    let sensitive_data = b"super_secret_password";
    let key = BearDogCrypto::generate_secure_random(32);

    let (ciphertext, _nonce) = BearDogCrypto::encrypt_aes_gcm(&key, sensitive_data, None).unwrap();

    // Ciphertext should not contain plaintext
    let ciphertext_str = String::from_utf8_lossy(&ciphertext);
    assert!(!ciphertext_str.contains("super_secret_password"));
}

#[test]
fn test_nonce_uniqueness() {
    let key = BearDogCrypto::generate_secure_random(32);
    let data = b"same data";

    // Encrypt same data multiple times
    let (_, nonce1) = BearDogCrypto::encrypt_aes_gcm(&key, data, None).unwrap();
    let (_, nonce2) = BearDogCrypto::encrypt_aes_gcm(&key, data, None).unwrap();
    let (_, nonce3) = BearDogCrypto::encrypt_aes_gcm(&key, data, None).unwrap();

    // All nonces should be different
    assert_ne!(nonce1, nonce2);
    assert_ne!(nonce2, nonce3);
    assert_ne!(nonce1, nonce3);
}
