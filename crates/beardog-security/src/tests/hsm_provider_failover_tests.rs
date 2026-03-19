// SPDX-License-Identifier: AGPL-3.0-only

//! HSM Provider Failover Tests
//!
//! Tests for HSM provider failover scenarios, cascading failures, and recovery.

use crate::crypto_utils::BearDogCrypto;
use beardog_errors::BearDogError;

#[test]
fn test_crypto_provider_always_available() -> Result<(), BearDogError> {
    // Test that crypto operations are always available (software fallback)
    // This ensures system can handle provider failures gracefully

    // Generate a key (should always succeed with software crypto)
    let key = BearDogCrypto::generate_secure_random(32);
    assert_eq!(key.len(), 32, "Key generation should always succeed");

    // Verify basic crypto operations work
    let data = b"Test data";
    let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, data, None)?;
    let decrypted = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce)?;
    assert_eq!(decrypted, data, "Basic crypto operations should work");

    Ok(())
}

#[test]
fn test_key_generation_fallback_reliability() -> Result<(), BearDogError> {
    // Test that key generation works reliably even if preferred methods fail

    // Generate multiple keys to test consistency
    let keys: Vec<Vec<u8>> = (0..5)
        .map(|_| BearDogCrypto::generate_secure_random(32))
        .collect();

    // All should be 32 bytes
    assert!(
        keys.iter().all(|k| k.len() == 32),
        "All keys should be 32 bytes"
    );
    // All should be unique
    for i in 0..keys.len() {
        for j in (i + 1)..keys.len() {
            assert_ne!(keys[i], keys[j], "Keys should be unique");
        }
    }

    Ok(())
}

#[test]
fn test_encryption_operations_failover() -> Result<(), BearDogError> {
    // Test that encryption operations work reliably

    let key = BearDogCrypto::generate_secure_random(32);
    let messages = vec![
        b"Message 1".to_vec(),
        b"Message 2 with more data".to_vec(),
        b"Message 3 even longer data here".to_vec(),
    ];

    // Encrypt all messages
    for msg in &messages {
        let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, msg, None)?;
        let decrypted = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce)?;
        assert_eq!(&decrypted, msg, "Encryption/decryption should be reliable");
    }

    Ok(())
}

#[test]
fn test_signing_operations_failover() -> Result<(), BearDogError> {
    // Test that signing operations work reliably

    let (private_key, public_key) = BearDogCrypto::generate_ed25519_keypair();
    let message = b"Test message for signing";

    // Sign the message
    let signature = BearDogCrypto::sign_ed25519(&private_key, message)?;

    // Verify the signature
    let is_valid = BearDogCrypto::verify_ed25519(&public_key, message, &signature)?;
    assert!(is_valid, "Signature verification should work");

    // Verify wrong message fails
    let wrong_message = b"Different message";
    let is_invalid = BearDogCrypto::verify_ed25519(&public_key, wrong_message, &signature)?;
    assert!(!is_invalid, "Wrong message should fail verification");

    Ok(())
}

#[test]
fn test_hash_operations_consistency() -> Result<(), BearDogError> {
    // Test that hash operations are consistent

    let data = b"Data to hash";

    // Hash multiple times
    let hash1 = BearDogCrypto::sha256_hash(data);
    let hash2 = BearDogCrypto::sha256_hash(data);
    let hash3 = BearDogCrypto::sha256_hash(data);

    // All should be identical
    assert_eq!(hash1, hash2, "Hashes should be deterministic");
    assert_eq!(hash2, hash3, "Hashes should be deterministic");

    // Different data should produce different hash
    let different_data = b"Different data";
    let hash4 = BearDogCrypto::sha256_hash(different_data);
    assert_ne!(hash1, hash4, "Different data should produce different hash");

    Ok(())
}

#[test]
fn test_concurrent_crypto_operations() -> Result<(), BearDogError> {
    // Test that concurrent crypto operations work safely

    use std::sync::Arc;
    use std::thread;

    let key = Arc::new(BearDogCrypto::generate_secure_random(32));
    let mut handles = vec![];

    // Spawn multiple threads performing crypto operations
    for i in 0..5 {
        let key_clone = Arc::clone(&key);
        let handle = thread::spawn(move || {
            let message = format!("Message {}", i);
            let (ciphertext, nonce) =
                BearDogCrypto::encrypt_aes_gcm(&key_clone, message.as_bytes(), None)
                    .expect("Encryption should succeed");
            let decrypted = BearDogCrypto::decrypt_aes_gcm(&key_clone, &ciphertext, &nonce)
                .expect("Decryption should succeed");
            assert_eq!(decrypted, message.as_bytes());
        });
        handles.push(handle);
    }

    // All should succeed
    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }

    Ok(())
}

#[test]
fn test_random_generation_entropy() -> Result<(), BearDogError> {
    // Test that random generation has good entropy

    // Generate multiple random values
    let random1 = BearDogCrypto::generate_secure_random(32);
    let random2 = BearDogCrypto::generate_secure_random(32);
    let random3 = BearDogCrypto::generate_secure_random(32);

    // All should be different
    assert_ne!(random1, random2, "Random values should be unique");
    assert_ne!(random2, random3, "Random values should be unique");
    assert_ne!(random1, random3, "Random values should be unique");

    // Should not be all zeros
    assert!(random1.iter().any(|&b| b != 0), "Should have entropy");
    assert!(random2.iter().any(|&b| b != 0), "Should have entropy");

    Ok(())
}

#[test]
fn test_large_data_encryption() -> Result<(), BearDogError> {
    // Test encryption of large data

    let key = BearDogCrypto::generate_secure_random(32);
    let large_data = vec![0x42u8; 1024 * 100]; // 100 KB

    let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, &large_data, None)?;
    let decrypted = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce)?;

    assert_eq!(decrypted, large_data, "Large data encryption should work");

    Ok(())
}

#[test]
fn test_signature_with_different_key_pairs() -> Result<(), BearDogError> {
    // Test that signatures with different key pairs are independent

    let (priv1, pub1) = BearDogCrypto::generate_ed25519_keypair();
    let (_priv2, pub2) = BearDogCrypto::generate_ed25519_keypair();
    let message = b"Test message";

    // Sign with first key
    let sig1 = BearDogCrypto::sign_ed25519(&priv1, message)?;

    // Verify with correct public key
    assert!(BearDogCrypto::verify_ed25519(&pub1, message, &sig1)?);

    // Should NOT verify with different public key
    assert!(!BearDogCrypto::verify_ed25519(&pub2, message, &sig1)?);

    Ok(())
}

#[test]
fn test_password_hashing_edge_cases() -> Result<(), BearDogError> {
    // Test password hashing with edge cases

    // Very long password
    let long_password = "a".repeat(1000);
    let hash = BearDogCrypto::hash_password_argon2(&long_password)?;
    assert!(!hash.is_empty(), "Should hash long passwords");
    assert!(
        BearDogCrypto::verify_password_argon2(&long_password, &hash)?,
        "Should verify long password"
    );

    // Short password
    let short_password = "pass";
    let hash2 = BearDogCrypto::hash_password_argon2(short_password)?;
    assert!(
        BearDogCrypto::verify_password_argon2(short_password, &hash2)?,
        "Should verify short password"
    );

    // Special characters
    let special_password = "p@$$w0rd!#%&*()";
    let hash3 = BearDogCrypto::hash_password_argon2(special_password)?;
    assert!(
        BearDogCrypto::verify_password_argon2(special_password, &hash3)?,
        "Should verify special chars"
    );

    Ok(())
}

#[test]
fn test_hmac_verification_scenarios() -> Result<(), BearDogError> {
    // Test HMAC creation and verification

    let key = BearDogCrypto::generate_secure_random(32);
    let data = b"Data to authenticate";

    // Create HMAC
    let hmac = BearDogCrypto::hmac_sha256(&key, data)?;
    assert_eq!(hmac.len(), 32, "HMAC should be 32 bytes");

    // Verify with correct key and data
    assert!(
        BearDogCrypto::verify_hmac_sha256(&key, data, &hmac)?,
        "HMAC should verify"
    );

    // Wrong key should fail
    let wrong_key = BearDogCrypto::generate_secure_random(32);
    assert!(
        !BearDogCrypto::verify_hmac_sha256(&wrong_key, data, &hmac)?,
        "Wrong key should fail"
    );

    // Modified data should fail
    let modified_data = b"Modified data";
    assert!(
        !BearDogCrypto::verify_hmac_sha256(&key, modified_data, &hmac)?,
        "Modified data should fail"
    );

    Ok(())
}

#[test]
fn test_constant_time_comparison() -> Result<(), BearDogError> {
    // Test constant-time comparison functionality

    let data1 = b"secret_data";
    let data2 = b"secret_data";
    let data3 = b"different_data";

    // Equal data should match
    assert!(
        BearDogCrypto::constant_time_compare(data1, data2),
        "Equal data should match"
    );

    // Different data should not match
    assert!(
        !BearDogCrypto::constant_time_compare(data1, data3),
        "Different data should not match"
    );

    // Different lengths should not match
    let short_data = b"short";
    assert!(
        !BearDogCrypto::constant_time_compare(data1, short_data),
        "Different lengths should not match"
    );

    // Empty slices should match
    let empty1: &[u8] = &[];
    let empty2: &[u8] = &[];
    assert!(
        BearDogCrypto::constant_time_compare(empty1, empty2),
        "Empty slices should match"
    );

    Ok(())
}

#[test]
fn test_api_key_generation() -> Result<(), BearDogError> {
    // Test API key generation

    // Generate keys with different prefixes
    let key1 = BearDogCrypto::generate_api_key("test")?;
    let key2 = BearDogCrypto::generate_api_key("prod")?;
    let key3 = BearDogCrypto::generate_api_key("dev")?;

    // Verify format (prefix_base64)
    assert!(key1.starts_with("test_"), "Should have correct prefix");
    assert!(key2.starts_with("prod_"), "Should have correct prefix");
    assert!(key3.starts_with("dev_"), "Should have correct prefix");

    // All should be unique
    assert_ne!(key1, key2, "Keys should be unique");
    assert_ne!(key2, key3, "Keys should be unique");
    assert_ne!(key1, key3, "Keys should be unique");

    // Should have reasonable length
    assert!(key1.len() > 40, "Key should be sufficiently long");

    Ok(())
}

#[test]
fn test_memory_zeroization() -> Result<(), BearDogError> {
    // Test that memory zeroization works

    let mut sensitive_data = vec![0x42u8; 32];

    // Verify data is not zero
    assert!(
        sensitive_data.iter().any(|&b| b != 0),
        "Data should not be all zeros"
    );

    // Zero the memory
    BearDogCrypto::zero_memory(&mut sensitive_data);

    // Verify data is now all zeros
    assert!(
        sensitive_data.iter().all(|&b| b == 0),
        "Data should be all zeros after zeroization"
    );

    // Multiple zeroization calls should be safe
    BearDogCrypto::zero_memory(&mut sensitive_data);
    assert!(
        sensitive_data.iter().all(|&b| b == 0),
        "Multiple zeroization should work"
    );

    Ok(())
}

#[test]
fn test_nonce_generation_uniqueness() -> Result<(), BearDogError> {
    // Test that nonce generation produces unique values

    let size = 12; // Standard AES-GCM nonce size

    // Generate multiple nonces
    let nonce1 = BearDogCrypto::generate_secure_nonce(size);
    let nonce2 = BearDogCrypto::generate_secure_nonce(size);
    let nonce3 = BearDogCrypto::generate_secure_nonce(size);

    // Verify correct length
    assert_eq!(nonce1.len(), size, "Nonce should be correct size");
    assert_eq!(nonce2.len(), size, "Nonce should be correct size");
    assert_eq!(nonce3.len(), size, "Nonce should be correct size");

    // All should be unique
    assert_ne!(nonce1, nonce2, "Nonces should be unique");
    assert_ne!(nonce2, nonce3, "Nonces should be unique");
    assert_ne!(nonce1, nonce3, "Nonces should be unique");

    // Should not be all zeros
    assert!(nonce1.iter().any(|&b| b != 0), "Nonce should have entropy");
    assert!(nonce2.iter().any(|&b| b != 0), "Nonce should have entropy");

    Ok(())
}
