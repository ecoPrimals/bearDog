// SPDX-License-Identifier: AGPL-3.0-only

//! Crypto Primitives Tests
//!
//! Comprehensive testing of core cryptographic operations including:
//! - Ed25519 signing and verification
//! - AES-GCM encryption and decryption
//! - Password hashing with Argon2
//! - Key derivation with PBKDF2
//! - Random number generation
//! - SHA-256 hashing
//! - HMAC operations

use crate::crypto_utils::BearDogCrypto;
use crate::*;

// ============================================================================
// Ed25519 Signature Tests
// ============================================================================

#[test]
fn test_ed25519_keypair_generation() {
    let (private_key, public_key) = BearDogCrypto::generate_ed25519_keypair();

    // Verify key lengths
    assert_eq!(private_key.len(), 32, "Private key must be 32 bytes");
    assert_eq!(public_key.len(), 32, "Public key must be 32 bytes");

    // Verify keys are non-zero
    assert_ne!(
        private_key,
        vec![0u8; 32],
        "Private key should not be all zeros"
    );
    assert_ne!(
        public_key,
        vec![0u8; 32],
        "Public key should not be all zeros"
    );
}

#[test]
fn test_ed25519_sign_and_verify() {
    let (private_key, public_key) = BearDogCrypto::generate_ed25519_keypair();
    let message = b"Hello, BearDog!";

    // Sign the message
    let signature =
        BearDogCrypto::sign_ed25519(&private_key, message).expect("Signing should succeed");

    // Verify signature length
    assert_eq!(signature.len(), 64, "Signature must be 64 bytes");

    // Verify the signature
    let is_valid = BearDogCrypto::verify_ed25519(&public_key, message, &signature)
        .expect("Verification should succeed");
    assert!(is_valid, "Valid signature should verify successfully");
}

#[test]
fn test_ed25519_verify_invalid_signature() {
    let (private_key, public_key) = BearDogCrypto::generate_ed25519_keypair();
    let message = b"Hello, BearDog!";

    let signature =
        BearDogCrypto::sign_ed25519(&private_key, message).expect("Signing should succeed");

    // Modify the message
    let wrong_message = b"Wrong message!";

    let is_valid = BearDogCrypto::verify_ed25519(&public_key, wrong_message, &signature)
        .expect("Verification should succeed");
    assert!(!is_valid, "Invalid signature should fail verification");
}

#[test]
fn test_ed25519_sign_with_invalid_key_length() {
    let invalid_key = vec![0u8; 16]; // Wrong length
    let message = b"Test message";

    let result = BearDogCrypto::sign_ed25519(&invalid_key, message);
    assert!(result.is_err(), "Should fail with invalid key length");
}

#[test]
fn test_ed25519_verify_with_invalid_key_length() {
    let invalid_public_key = vec![0u8; 16]; // Wrong length
    let message = b"Test message";
    let fake_signature = vec![0u8; 64];

    let result = BearDogCrypto::verify_ed25519(&invalid_public_key, message, &fake_signature);
    assert!(
        result.is_err(),
        "Should fail with invalid public key length"
    );
}

#[test]
fn test_ed25519_verify_with_invalid_signature_length() {
    let (_private_key, public_key) = BearDogCrypto::generate_ed25519_keypair();
    let message = b"Test message";
    let invalid_signature = vec![0u8; 32]; // Wrong length

    let result = BearDogCrypto::verify_ed25519(&public_key, message, &invalid_signature);
    assert!(result.is_err(), "Should fail with invalid signature length");
}

#[test]
fn test_ed25519_multiple_messages() {
    let (private_key, public_key) = BearDogCrypto::generate_ed25519_keypair();

    let messages = vec![
        b"Message 1".as_slice(),
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        b"Message 2".as_slice(),
        b"A much longer message with more content".as_slice(),
        b"".as_slice(), // Empty message
    ];

    for message in messages {
        let signature =
            BearDogCrypto::sign_ed25519(&private_key, message).expect("Signing should succeed");

        let is_valid = BearDogCrypto::verify_ed25519(&public_key, message, &signature)
            .expect("Verification should succeed");
        assert!(
            is_valid,
            "Signature should be valid for message: {message:?}"
        );
    }
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal

// ============================================================================
// AES-GCM Encryption/Decryption Tests
// ============================================================================

#[test]
fn test_aes_gcm_encrypt_decrypt() {
    let key = BearDogCrypto::generate_secure_random(32); // 256-bit key
    let plaintext = b"Sensitive data to encrypt";

    // Encrypt
    let (ciphertext, nonce) =
        BearDogCrypto::encrypt_aes_gcm(&key, plaintext, None).expect("Encryption should succeed");

    // Verify ciphertext is different from plaintext
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: important
    assert_ne!(
        ciphertext, plaintext,
        "Ciphertext should differ from plaintext"
    );
    assert_eq!(nonce.len(), 12, "Nonce must be 12 bytes");

    // Decrypt
    let decrypted = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce)
        .expect("Decryption should succeed");

    assert_eq!(decrypted, plaintext, "Decrypted data should match original");
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: important
#[test]
fn test_aes_gcm_with_custom_nonce() {
    let key = BearDogCrypto::generate_secure_random(32);
    let plaintext = b"Test data";
    let nonce = BearDogCrypto::generate_secure_nonce(12);

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: important
    let (ciphertext, returned_nonce) =
        BearDogCrypto::encrypt_aes_gcm(&key, plaintext, Some(&nonce))
            .expect("Encryption should succeed");

    assert_eq!(
        returned_nonce, nonce,
        "Returned nonce should match provided nonce"
    );

    let decrypted = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce)
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: important
        .expect("Decryption should succeed");

    assert_eq!(decrypted, plaintext, "Decrypted data should match original");
}

#[test]
fn test_aes_gcm_encrypt_with_invalid_key_length() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let invalid_key = vec![0u8; 16]; // Wrong length (should be 32)
    let plaintext = b"Test data";

    let result = BearDogCrypto::encrypt_aes_gcm(&invalid_key, plaintext, None);
    assert!(result.is_err(), "Should fail with invalid key length");
}

#[test]
fn test_aes_gcm_decrypt_with_invalid_key() {
    let key1 = BearDogCrypto::generate_secure_random(32);
    let key2 = BearDogCrypto::generate_secure_random(32);
    let plaintext = b"Secret data";

    let (ciphertext, nonce) =
        BearDogCrypto::encrypt_aes_gcm(&key1, plaintext, None).expect("Encryption should succeed");

    // Try to decrypt with wrong key
    let result = BearDogCrypto::decrypt_aes_gcm(&key2, &ciphertext, &nonce);
    assert!(result.is_err(), "Decryption with wrong key should fail");
}

#[test]
fn test_aes_gcm_decrypt_with_invalid_nonce_length() {
    let key = BearDogCrypto::generate_secure_random(32);
    let invalid_nonce = vec![0u8; 16]; // Wrong length
    let ciphertext = vec![0u8; 32];
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical

    let result = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &invalid_nonce);
    assert!(result.is_err(), "Should fail with invalid nonce length");
}

#[test]
fn test_aes_gcm_empty_plaintext() {
    let key = BearDogCrypto::generate_secure_random(32);
    let plaintext = b"";

    let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, plaintext, None)
        .expect("Encryption of empty data should succeed");

    let decrypted = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce)
        .expect("Decryption should succeed");

    assert_eq!(decrypted, plaintext, "Decrypted empty data should match");
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
fn test_aes_gcm_large_plaintext() {
    let key = BearDogCrypto::generate_secure_random(32);
    let plaintext = vec![0xABu8; 10_000]; // 10KB of data

    let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, &plaintext, None)
        .expect("Encryption of large data should succeed");

    let decrypted = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce)
        .expect("Decryption should succeed");

    assert_eq!(decrypted, plaintext, "Decrypted large data should match");
}

// ============================================================================
// Password Hashing (Argon2) Tests
// ============================================================================

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: critical
fn test_argon2_password_hashing() {
    let password = "MySecurePassword123!";

    let hash =
        BearDogCrypto::hash_password_argon2(password).expect("Password hashing should succeed");

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
    assert!(!hash.is_empty(), "Hash should not be empty");
    assert!(hash.starts_with("$argon2"), "Hash should be Argon2 format");
}

#[test]
fn test_argon2_password_verification_success() {
    let password = "TestPassword456!";

    let hash =
        BearDogCrypto::hash_password_argon2(password).expect("Password hashing should succeed");

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
    let is_valid = BearDogCrypto::verify_password_argon2(password, &hash)
        .expect("Password verification should succeed");

    assert!(is_valid, "Correct password should verify successfully");
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
fn test_argon2_password_verification_failure() {
    let password = "CorrectPassword";
    let wrong_password = "WrongPassword";

    let hash =
        BearDogCrypto::hash_password_argon2(password).expect("Password hashing should succeed");

    let is_valid = BearDogCrypto::verify_password_argon2(wrong_password, &hash)
        .expect("Password verification should succeed");

    assert!(!is_valid, "Wrong password should fail verification");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
}

#[test]
fn test_argon2_different_hashes_for_same_password() {
    let password = "SamePassword";

    let hash1 = BearDogCrypto::hash_password_argon2(password).expect("First hash should succeed");
    let hash2 = BearDogCrypto::hash_password_argon2(password).expect("Second hash should succeed");

    // Different salts should produce different hashes
    assert_ne!(
        hash1, hash2,
        "Same password should produce different hashes (different salts)"
    );

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    // But both should verify correctly
    assert!(BearDogCrypto::verify_password_argon2(password, &hash1).unwrap());
    assert!(BearDogCrypto::verify_password_argon2(password, &hash2).unwrap());
}

#[test]
fn test_argon2_empty_password() {
    let password = "";
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    let hash = BearDogCrypto::hash_password_argon2(password)
        .expect("Hashing empty password should succeed");

    let is_valid = BearDogCrypto::verify_password_argon2(password, &hash)
        .expect("Verification should succeed");

    assert!(is_valid, "Empty password should verify correctly");
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: important
// ============================================================================
// PBKDF2 Key Derivation Tests
// ============================================================================

#[test]
fn test_pbkdf2_key_derivation() {
    let password = b"MyPassword";
    let salt = b"UniqueSalt";
    let iterations = 10000;
    let key_length = 32;

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let key = BearDogCrypto::derive_pbkdf2_key(password, salt, iterations, key_length)
        .expect("Key derivation should succeed");

    assert_eq!(
        key.len(),
        key_length,
        "Derived key should have requested length"
    );
    assert_ne!(
        key,
        vec![0u8; key_length],
        "Derived key should not be all zeros"
    );
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
#[test]
fn test_pbkdf2_deterministic() {
    let password = b"TestPassword";
    let salt = b"TestSalt";
    let iterations = 5000;
    let key_length = 32;

    let key1 = BearDogCrypto::derive_pbkdf2_key(password, salt, iterations, key_length)
        .expect("First derivation should succeed");
    let key2 = BearDogCrypto::derive_pbkdf2_key(password, salt, iterations, key_length)
        .expect("Second derivation should succeed");

    assert_eq!(key1, key2, "Same inputs should produce same key");
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal

#[test]
fn test_pbkdf2_different_salts_produce_different_keys() {
    let password = b"Password";
    let salt1 = b"Salt1";
    let salt2 = b"Salt2";
    let iterations = 5000;
    let key_length = 32;

    let key1 = BearDogCrypto::derive_pbkdf2_key(password, salt1, iterations, key_length)
        .expect("First derivation should succeed");
    let key2 = BearDogCrypto::derive_pbkdf2_key(password, salt2, iterations, key_length)
        .expect("Second derivation should succeed");

    assert_ne!(key1, key2, "Different salts should produce different keys");
}

#[test]
fn test_pbkdf2_zero_iterations_fails() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let password = b"Password";
    let salt = b"Salt";
    let iterations = 0; // Invalid
    let key_length = 32;

    let result = BearDogCrypto::derive_pbkdf2_key(password, salt, iterations, key_length);
    assert!(result.is_err(), "Zero iterations should fail");
}

// ============================================================================
// Random Generation Tests
// ============================================================================
// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal

#[test]
fn test_secure_random_generation() {
    let size = 32;
    let random1 = BearDogCrypto::generate_secure_random(size);
    let random2 = BearDogCrypto::generate_secure_random(size);

    assert_eq!(random1.len(), size, "Random should have requested size");
    assert_eq!(random2.len(), size, "Random should have requested size");
    assert_ne!(random1, random2, "Two random values should be different");
    assert_ne!(random1, vec![0u8; size], "Random should not be all zeros");
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: important
#[test]
fn test_secure_random_various_sizes() {
    let sizes = vec![0, 1, 16, 32, 64, 128, 256, 1024];

    for size in sizes {
        let random = BearDogCrypto::generate_secure_random(size);
        assert_eq!(random.len(), size, "Random should have size {size}");
    }
}

#[test]
fn test_secure_nonce_generation() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let size = 12; // Typical nonce size for AES-GCM
    let nonce1 = BearDogCrypto::generate_secure_nonce(size);
    let nonce2 = BearDogCrypto::generate_secure_nonce(size);

    assert_eq!(nonce1.len(), size, "Nonce should have requested size");
    assert_eq!(nonce2.len(), size, "Nonce should have requested size");
    assert_ne!(nonce1, nonce2, "Two nonces should be different");
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
// ============================================================================
// SHA-256 Hashing Tests
// ============================================================================

#[test]
fn test_sha256_hash() {
    let data = b"Hello, BearDog!";
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    let hash = compute_sha256_hash(data).expect("SHA-256 hashing should succeed");

    assert_eq!(hash.len(), 32, "SHA-256 hash should be 32 bytes");
}

#[test]
fn test_sha256_deterministic() {
    let data = b"Test data";

    let hash1 = compute_sha256_hash(data).unwrap();
    let hash2 = compute_sha256_hash(data).unwrap();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    assert_eq!(hash1, hash2, "Same data should produce same hash");
}

#[test]
fn test_sha256_different_data() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let data1 = b"Data 1";
    let data2 = b"Data 2";

    let hash1 = compute_sha256_hash(data1).unwrap();
    let hash2 = compute_sha256_hash(data2).unwrap();

    assert_ne!(
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        hash1,
        hash2,
        "Different data should produce different hashes"
    );
}

#[test]
fn test_sha256_empty_data() {
    let data = b"";

    let hash = compute_sha256_hash(data).expect("Hashing empty data should succeed");

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    assert_eq!(hash.len(), 32, "Empty data hash should still be 32 bytes");
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_end_to_end_secure_message_flow() {
    // Generate key pair for signing
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let (signing_key, verifying_key) = BearDogCrypto::generate_ed25519_keypair();

    // Generate encryption key
    let encryption_key = BearDogCrypto::generate_secure_random(32);

    // Original message
    let message = b"Confidential message";

    // 1. Encrypt the message
    let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&encryption_key, message, None)
        .expect("Encryption should succeed");

    // 2. Sign the ciphertext
    let signature =
        BearDogCrypto::sign_ed25519(&signing_key, &ciphertext).expect("Signing should succeed");

    // 3. Verify the signature
    let is_valid = BearDogCrypto::verify_ed25519(&verifying_key, &ciphertext, &signature)
        .expect("Verification should succeed");
    assert!(is_valid, "Signature should be valid");

    // 4. Decrypt the message
    let decrypted = BearDogCrypto::decrypt_aes_gcm(&encryption_key, &ciphertext, &nonce)
        .expect("Decryption should succeed");

    assert_eq!(
        decrypted, message,
        "End-to-end flow should preserve message"
    );
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: critical
#[test]
fn test_key_derivation_for_encryption() {
    // Derive encryption key from password
    let password = b"UserPassword123";
    let salt = BearDogCrypto::generate_secure_random(16);
    let encryption_key = BearDogCrypto::derive_pbkdf2_key(password, &salt, 10000, 32)
        .expect("Key derivation should succeed");

    // Use derived key for encryption
    let plaintext = b"Secret data";
    let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&encryption_key, plaintext, None)
        .expect("Encryption should succeed");

    // Derive same key again
    let encryption_key2 = BearDogCrypto::derive_pbkdf2_key(password, &salt, 10000, 32)
        .expect("Key derivation should succeed");

    // Decrypt with re-derived key
    let decrypted = BearDogCrypto::decrypt_aes_gcm(&encryption_key2, &ciphertext, &nonce)
        .expect("Decryption should succeed");

    assert_eq!(
        decrypted, plaintext,
        "Password-based encryption should work"
    );
}
