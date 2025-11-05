//! Comprehensive Security Operations Tests
//!
//! Tests for core security operations including encryption, decryption, signing,
//! verification, key management, and cryptographic primitives.
//! Created: October 25, 2025 (Week 2 Test Expansion - Day 2)

#[cfg(test)]
mod encryption_operations_tests {
    use crate::encryption::{EncryptionAlgorithm, EncryptionEngine};

    #[test]
    fn test_encryption_engine_creation() {
        let engine = EncryptionEngine::new();
        assert!(engine.is_ok(), "Encryption engine should be created");
    }

    #[test]
    fn test_encryption_with_aes256() {
        let engine = EncryptionEngine::new().expect("Engine creation");
        let data = b"test data";
        let result = engine.encrypt(data, Some(EncryptionAlgorithm::Aes256Gcm));
        assert!(result.is_ok(), "AES256 encryption should succeed");
    }

    #[test]
    fn test_encryption_with_chacha20() {
        let engine = EncryptionEngine::new().expect("Engine creation");
        let data = b"test data";
        let result = engine.encrypt(data, Some(EncryptionAlgorithm::ChaCha20Poly1305));
        assert!(result.is_ok(), "ChaCha20 encryption should succeed");
    }

    #[test]
    fn test_encryption_empty_data() {
        let engine = EncryptionEngine::new().expect("Engine creation");
        let data = b"";
        let result = engine.encrypt(data, Some(EncryptionAlgorithm::Aes256Gcm));
        assert!(result.is_ok(), "Empty data encryption should succeed");
    }

    #[test]
    fn test_encryption_large_data() {
        let engine = EncryptionEngine::new().expect("Engine creation");
        let data = vec![0u8; 1024 * 1024]; // 1MB
        let result = engine.encrypt(&data, Some(EncryptionAlgorithm::Aes256Gcm));
        assert!(result.is_ok(), "Large data encryption should succeed");
    }

    #[test]
    fn test_encryption_decryption_roundtrip() {
        let engine = EncryptionEngine::new().expect("Engine creation");
        let data = b"test data for roundtrip";
        let encrypted = engine.encrypt(data, Some(EncryptionAlgorithm::Aes256Gcm))
            .expect("Encryption");
        let decrypted = engine.decrypt(&encrypted).expect("Decryption");
        assert_eq!(data.as_slice(), decrypted.as_slice());
    }

    #[test]
    fn test_encryption_with_default_algorithm() {
        let engine = EncryptionEngine::new().expect("Engine creation");
        let data = b"test data";
        let result = engine.encrypt(data, None);
        assert!(result.is_ok(), "Default algorithm encryption should succeed");
    }

    #[test]
    fn test_encryption_deterministic_output_size() {
        let engine = EncryptionEngine::new().expect("Engine creation");
        let data = b"test data";
        let encrypted1 = engine.encrypt(data, Some(EncryptionAlgorithm::Aes256Gcm))
            .expect("Encryption 1");
        let encrypted2 = engine.encrypt(data, Some(EncryptionAlgorithm::Aes256Gcm))
            .expect("Encryption 2");
        // Encrypted data should have consistent structure (even if different due to nonce)
        assert!(encrypted1.ciphertext.len() >= data.len());
        assert!(encrypted2.ciphertext.len() >= data.len());
    }
}

#[cfg(test)]
mod key_management_operations_tests {
    use crate::key_management::{KeyManager, KeyType};

    #[test]
    fn test_key_manager_creation() {
        let manager = KeyManager::new();
        assert!(manager.is_ok(), "Key manager should be created");
    }

    #[test]
    fn test_generate_symmetric_key() {
        let manager = KeyManager::new().expect("Manager creation");
        let result = manager.generate_key(KeyType::Symmetric, 256);
        assert!(result.is_ok(), "Symmetric key generation should succeed");
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
    #[test]
    fn test_generate_asymmetric_key_pair() {
        let manager = KeyManager::new().expect("Manager creation");
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical
        let result = manager.generate_key_pair(KeyType::Ed25519);
        assert!(result.is_ok(), "Key pair generation should succeed");
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
    fn test_key_storage_and_retrieval() {
        let manager = KeyManager::new().expect("Manager creation");
        let key_id = "test_key";
        let key = manager.generate_key(KeyType::Symmetric, 256)
            .expect("Key generation");
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical
        let store_result = manager.store_key(key_id, &key);
        assert!(store_result.is_ok(), "Key storage should succeed");
        
        let retrieve_result = manager.get_key(key_id);
        assert!(retrieve_result.is_ok(), "Key retrieval should succeed");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
    }

    #[test]
    fn test_key_deletion() {
        let manager = KeyManager::new().expect("Manager creation");
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical
        let key_id = "test_key_delete";
        let key = manager.generate_key(KeyType::Symmetric, 256)
            .expect("Key generation");
        manager.store_key(key_id, &key).expect("Storage");
        
        let delete_result = manager.delete_key(key_id);
        assert!(delete_result.is_ok(), "Key deletion should succeed");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
    }

    #[test]
    fn test_key_rotation() {
        let manager = KeyManager::new().expect("Manager creation");
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical
        let key_id = "test_key_rotate";
        let key = manager.generate_key(KeyType::Symmetric, 256)
            .expect("Key generation");
        manager.store_key(key_id, &key).expect("Storage");
        
        let rotation_result = manager.rotate_key(key_id);
        assert!(rotation_result.is_ok(), "Key rotation should succeed");
    }

    #[test]
    fn test_multiple_key_types() {
        let manager = KeyManager::new().expect("Manager creation");
        let sym_key = manager.generate_key(KeyType::Symmetric, 256);
        let ed25519_pair = manager.generate_key_pair(KeyType::Ed25519);
        let rsa_pair = manager.generate_key_pair(KeyType::Rsa2048);
         // TEST_CATEGORY: integration
         // TEST_DOMAIN: security
         // TEST_PRIORITY: normal
        
        assert!(sym_key.is_ok());
        assert!(ed25519_pair.is_ok());
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        assert!(rsa_pair.is_ok());
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    fn test_key_metadata_tracking() {
        let manager = KeyManager::new().expect("Manager creation");
        let key_id = "test_key_metadata";
        let key = manager.generate_key(KeyType::Symmetric, 256)
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: security
            // TEST_PRIORITY: normal
            .expect("Key generation");
        manager.store_key(key_id, &key).expect("Storage");
        
        let metadata = manager.get_key_metadata(key_id);
        assert!(metadata.is_ok(), "Metadata retrieval should succeed");
    }
}

#[cfg(test)]
mod cryptographic_primitives_tests {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    use crate::crypto_utils::BearDogCrypto;

    #[test]
    fn test_hash_sha256() {
        let data = b"test data";
        let result = BearDogCrypto::hash_sha256(data);
        assert!(result.is_ok(), "SHA256 hash should succeed");
        assert_eq!(result.unwrap().len(), 32);
    }
 // TEST_CATEGORY: integration
 // TEST_DOMAIN: security
 // TEST_PRIORITY: normal

    #[test]
    fn test_hash_sha512() {
        let data = b"test data";
        let result = BearDogCrypto::hash_sha512(data);
        assert!(result.is_ok(), "SHA512 hash should succeed");
        assert_eq!(result.unwrap().len(), 64);
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_hash_blake3() {
        let data = b"test data";
        let result = BearDogCrypto::hash_blake3(data);
        assert!(result.is_ok(), "BLAKE3 hash should succeed");
    }

    #[test]
    fn test_pbkdf2_key_derivation() {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let password = b"password123";
        let salt = b"random_salt";
        let result = BearDogCrypto::derive_key_pbkdf2(password, salt, 10000, 32);
        assert!(result.is_ok(), "PBKDF2 derivation should succeed");
        assert_eq!(result.unwrap().len(), 32);
    }

    #[test]
    fn test_ed25519_keypair_generation() {
        let result = BearDogCrypto::generate_ed25519_keypair();
        assert!(result.is_ok(), "Ed25519 keypair generation should succeed");
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    fn test_ed25519_sign_verify() {
        let (private_key, public_key) = BearDogCrypto::generate_ed25519_keypair()
            .expect("Keypair generation");
        let message = b"test message";
        
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let signature = BearDogCrypto::sign_ed25519(&private_key, message)
            .expect("Signing");
        let is_valid = BearDogCrypto::verify_ed25519_signature(&public_key, message, &signature)
            .expect("Verification");
        
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        assert!(is_valid, "Signature should be valid");
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    fn test_random_bytes_generation() {
        let result = BearDogCrypto::generate_random_bytes(32);
        assert!(result.is_ok(), "Random bytes generation should succeed");
        assert_eq!(result.unwrap().len(), 32);
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_constant_time_comparison() {
        let a = b"secret_value";
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let b = b"secret_value";
        let c = b"different_value";
        
        assert!(BearDogCrypto::constant_time_compare(a, b));
        assert!(!BearDogCrypto::constant_time_compare(a, c));
    }
}

#[cfg(test)]
mod authentication_operations_tests {
    use crate::authentication::{AuthenticationManager, Credential};
 // TEST_CATEGORY: integration
 // TEST_DOMAIN: security
 // TEST_PRIORITY: normal

    #[test]
    fn test_authentication_manager_creation() {
        let manager = AuthenticationManager::new();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        assert!(manager.is_ok(), "Auth manager should be created");
    }

    #[test]
    fn test_credential_validation() {
        let manager = AuthenticationManager::new().expect("Manager creation");
        let credential = Credential::new("user", "password");
        let result = manager.validate_credential(&credential);
        assert!(result.is_ok(), "Credential validation should complete");
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
    fn test_token_generation() {
        let manager = AuthenticationManager::new().expect("Manager creation");
        let user_id = "test_user";
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let result = manager.generate_token(user_id);
        assert!(result.is_ok(), "Token generation should succeed");
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    fn test_token_validation() {
        let manager = AuthenticationManager::new().expect("Manager creation");
        let user_id = "test_user";
        let token = manager.generate_token(user_id).expect("Token generation");
        let result = manager.validate_token(&token);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        assert!(result.is_ok(), "Token validation should succeed");
    }

    #[test]
    fn test_token_expiration() {
        let manager = AuthenticationManager::new().expect("Manager creation");
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let token = manager.generate_token("user").expect("Token generation");
        let is_expired = manager.is_token_expired(&token);
        assert!(is_expired.is_ok(), "Expiration check should succeed");
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
    #[test]
    fn test_multi_factor_authentication() {
        let manager = AuthenticationManager::new().expect("Manager creation");
        let result = manager.setup_mfa("user");
        assert!(result.is_ok(), "MFA setup should succeed");
    }
}

// Test Summary:
// - Encryption Operations: 8 tests
// - Key Management Operations: 8 tests
// - Cryptographic Primitives: 8 tests
// - Authentication Operations: 6 tests
// Total: 30 new tests for security operations coverage

