// SPDX-License-Identifier: AGPL-3.0-only

//! Edge case tests for crypto operations
//!
//! These tests cover boundary conditions, error paths, and edge cases
//! for cryptographic operations to improve test coverage.

use crate::crypto_utils::BearDogCrypto;
use beardog_errors::BearDogError;

#[cfg(test)]
mod crypto_edge_cases {
    use super::*;

    // ============================================================================
    // Ed25519 Edge Cases
    // ============================================================================

    #[test]
    fn test_sign_ed25519_with_invalid_key_length() {
        let invalid_key = vec![0u8; 16]; // Wrong length (should be 32)
        let message = b"test message";
        
        let result = BearDogCrypto::sign_ed25519(&invalid_key, message);
        
        assert!(result.is_err());
        if let Err(BearDogError::InvalidInput { message: msg, .. }) = result {
            assert!(msg.contains("32 bytes"));
        } else {
            panic!("Expected InvalidInput error");
        }
    }

    #[test]
    fn test_sign_ed25519_with_empty_message() {
        let (private_key, _) = BearDogCrypto::generate_ed25519_keypair();
        let empty_message = b"";
        
        let result = BearDogCrypto::sign_ed25519(&private_key, empty_message);
        
        // Should succeed - signing empty messages is valid
        assert!(result.is_ok());
        let signature = result.unwrap();
        assert_eq!(signature.len(), 64); // Ed25519 signatures are 64 bytes
    }

    #[test]
    fn test_sign_ed25519_with_large_message() {
        let (private_key, _) = BearDogCrypto::generate_ed25519_keypair();
        let large_message = vec![0u8; 1024 * 1024]; // 1MB message
        
        let result = BearDogCrypto::sign_ed25519(&private_key, &large_message);
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_ed25519_with_wrong_public_key() {
        let (private_key, _) = BearDogCrypto::generate_ed25519_keypair();
        let (_, wrong_public_key) = BearDogCrypto::generate_ed25519_keypair();
        let message = b"test message";
        
        let signature = BearDogCrypto::sign_ed25519(&private_key, message).unwrap();
        let result = BearDogCrypto::verify_ed25519(&wrong_public_key, message, &signature);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_ed25519_with_modified_message() {
        let (private_key, public_key) = BearDogCrypto::generate_ed25519_keypair();
        let message = b"original message";
        let modified_message = b"modified message";
        
        let signature = BearDogCrypto::sign_ed25519(&private_key, message).unwrap();
        let result = BearDogCrypto::verify_ed25519(&public_key, modified_message, &signature);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_ed25519_with_invalid_signature_length() {
        let (_, public_key) = BearDogCrypto::generate_ed25519_keypair();
        let message = b"test message";
        let invalid_signature = vec![0u8; 32]; // Wrong length (should be 64)
        
        let result = BearDogCrypto::verify_ed25519(&public_key, message, &invalid_signature);
        
        assert!(result.is_err());
    }
 // TEST_CATEGORY: integration
 // TEST_DOMAIN: security
 // TEST_PRIORITY: important

    // ============================================================================
    // AES-GCM Edge Cases
    // ============================================================================

    #[test]
    fn test_aes_encrypt_with_empty_data() {
        let key = vec![0u8; 32]; // 256-bit key
        let empty_data = b"";
        
        let result = BearDogCrypto::aes_gcm_encrypt(&key, empty_data);
        
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        // Should succeed - encrypting empty data is valid
        assert!(result.is_ok());
    }

    #[test]
    fn test_aes_encrypt_with_invalid_key_length() {
        let invalid_key = vec![0u8; 16]; // Wrong length for AES-256 (should be 32)
        let data = b"test data";
        
        let result = BearDogCrypto::aes_gcm_encrypt(&invalid_key, data);
         // TEST_CATEGORY: integration
         // TEST_DOMAIN: security
         // TEST_PRIORITY: normal
        
        assert!(result.is_err());
    }

    #[test]
    fn test_aes_decrypt_with_wrong_key() {
        let key1 = vec![1u8; 32];
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let key2 = vec![2u8; 32];
        let data = b"test data";
        
        let encrypted = BearDogCrypto::aes_gcm_encrypt(&key1, data).unwrap();
        let result = BearDogCrypto::aes_gcm_decrypt(&key2, &encrypted);
        
        assert!(result.is_err());
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_aes_decrypt_with_corrupted_ciphertext() {
        let key = vec![0u8; 32];
        let data = b"test data";
        
        let mut encrypted = BearDogCrypto::aes_gcm_encrypt(&key, data).unwrap();
        // Corrupt the ciphertext
        if !encrypted.is_empty() {
            encrypted[0] ^= 0xFF;
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: important
        }
        
        let result = BearDogCrypto::aes_gcm_decrypt(&key, &encrypted);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_aes_decrypt_with_truncated_ciphertext() {
        let key = vec![0u8; 32];
        let data = b"test data";
        
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical
        let encrypted = BearDogCrypto::aes_gcm_encrypt(&key, data).unwrap();
        let truncated = &encrypted[..encrypted.len().saturating_sub(5)];
        
        let result = BearDogCrypto::aes_gcm_decrypt(&key, truncated);
        
        assert!(result.is_err());
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
    // ============================================================================
    // Hashing Edge Cases
    // ============================================================================

    #[test]
    fn test_hash_empty_data() {
        let empty_data = b"";
         // TEST_CATEGORY: integration
         // TEST_DOMAIN: security
         // TEST_PRIORITY: critical
        
        let result = BearDogCrypto::compute_sha256(empty_data);
        
        assert!(result.is_ok());
        let hash = result.unwrap();
        assert_eq!(hash.len(), 32); // SHA-256 produces 32 bytes
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
    fn test_hash_large_data() {
        let large_data = vec![0xAA; 10 * 1024 * 1024]; // 10MB
        
        let result = BearDogCrypto::compute_sha256(&large_data);
        
        assert!(result.is_ok());
        let hash = result.unwrap();
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_hash_consistency() {
        let data = b"test data";
         // TEST_CATEGORY: integration
         // TEST_DOMAIN: security
         // TEST_PRIORITY: critical
        
        let hash1 = BearDogCrypto::compute_sha256(data).unwrap();
        let hash2 = BearDogCrypto::compute_sha256(data).unwrap();
        
        assert_eq!(hash1, hash2);
    }

    // ============================================================================
    // HMAC Edge Cases
    // ============================================================================

    #[test]
    fn test_hmac_with_empty_key() {
        let empty_key = b"";
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let data = b"test data";
        
        let result = BearDogCrypto::compute_hmac_sha256(empty_key, data);
        
        // HMAC should work with empty keys (though not recommended)
        assert!(result.is_ok());
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_hmac_with_empty_data() {
        let key = b"secret key";
        let empty_data = b"";
        
        let result = BearDogCrypto::compute_hmac_sha256(key, empty_data);
        
        assert!(result.is_ok());
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    }

    #[test]
    fn test_hmac_verify_with_wrong_key() {
        let key1 = b"key1";
        let key2 = b"key2";
        let data = b"test data";
        
        let hmac = BearDogCrypto::compute_hmac_sha256(key1, data).unwrap();
        let result = BearDogCrypto::verify_hmac_sha256(key2, data, &hmac);
        
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        assert!(result.is_err());
    }

    #[test]
    fn test_hmac_verify_with_modified_data() {
        let key = b"secret key";
        let original_data = b"original data";
        let modified_data = b"modified data";
         // TEST_CATEGORY: integration
         // TEST_DOMAIN: security
         // TEST_PRIORITY: normal
        
        let hmac = BearDogCrypto::compute_hmac_sha256(key, original_data).unwrap();
        let result = BearDogCrypto::verify_hmac_sha256(key, modified_data, &hmac);
        
        assert!(result.is_err());
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    // ============================================================================
    // Password Hashing Edge Cases
    // ============================================================================

    #[test]
    fn test_password_hash_empty_password() {
        let empty_password = "";
        
        let result = BearDogCrypto::hash_password(empty_password);
         // TEST_CATEGORY: integration
         // TEST_DOMAIN: security
         // TEST_PRIORITY: normal
        
        // Should succeed - empty passwords are technically valid
        assert!(result.is_ok());
    }

    #[test]
    fn test_password_hash_very_long_password() {
        let long_password = "a".repeat(1000);
        
        let result = BearDogCrypto::hash_password(&long_password);
        
        assert!(result.is_ok());
    }
 // TEST_CATEGORY: integration
 // TEST_DOMAIN: security
 // TEST_PRIORITY: normal

    #[test]
    fn test_password_verify_wrong_password() {
        let correct_password = "correct_password";
        let wrong_password = "wrong_password";
        
        let hash = BearDogCrypto::hash_password(correct_password).unwrap();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let result = BearDogCrypto::verify_password(wrong_password, &hash);
        
        assert!(result.is_err());
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    fn test_password_verify_corrupted_hash() {
        let password = "test_password";
        let corrupted_hash = "invalid_hash_format";
        
        let result = BearDogCrypto::verify_password(password, corrupted_hash);
        
        assert!(result.is_err());
    }
 // TEST_CATEGORY: integration
 // TEST_DOMAIN: security
 // TEST_PRIORITY: normal

    // ============================================================================
    // Random Generation Edge Cases
    // ============================================================================

    #[test]
    fn test_generate_random_bytes_zero_length() {
        let result = BearDogCrypto::generate_random_bytes(0);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    }

    #[test]
    fn test_generate_random_bytes_large_amount() {
        let result = BearDogCrypto::generate_random_bytes(1024 * 1024); // 1MB
         // TEST_CATEGORY: integration
         // TEST_DOMAIN: security
         // TEST_PRIORITY: normal
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1024 * 1024);
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_generate_random_bytes_uniqueness() {
        let bytes1 = BearDogCrypto::generate_random_bytes(32).unwrap();
        let bytes2 = BearDogCrypto::generate_random_bytes(32).unwrap();
        
        // Should be different (statistically almost certain)
        assert_ne!(bytes1, bytes2);
    }
}

