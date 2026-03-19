// SPDX-License-Identifier: AGPL-3.0-only

// Key Rotation and Lifecycle Tests
// Tests critical key rotation scenarios for security

use crate::crypto_utils::BearDogCrypto;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_generation_uniqueness() {
        // Ensure each generated key is unique
        let key1 = BearDogCrypto::generate_secure_random(32);
        let key2 = BearDogCrypto::generate_secure_random(32);
        let key3 = BearDogCrypto::generate_secure_random(32);

        assert_ne!(key1, key2, "Keys should be unique");
        assert_ne!(key2, key3, "Keys should be unique");
        assert_ne!(key1, key3, "Keys should be unique");
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_key_rotation_preserves_old_data_decryption() {
        // When rotating keys, old data with old key should still decrypt
        let old_key = BearDogCrypto::generate_secure_random(32);
        let plaintext = b"Data encrypted with old key";

        let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&old_key, plaintext, None)
            .expect("Encryption with old key should succeed");

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical
        // Even after generating new key, old key should still decrypt old data
        let _new_key = BearDogCrypto::generate_secure_random(32);

        let decrypted = BearDogCrypto::decrypt_aes_gcm(&old_key, &ciphertext, &nonce)
            .expect("Old key should decrypt old data");

        assert_eq!(decrypted, plaintext, "Old key must still decrypt old data");
    }

    #[test]
    fn test_multiple_key_sizes_supported() {
        // Verify different key sizes work correctly
        let key_16 = BearDogCrypto::generate_secure_random(16); // AES-128
        let key_24 = BearDogCrypto::generate_secure_random(24); // AES-192
        let key_32 = BearDogCrypto::generate_secure_random(32); // AES-256
                                                                // TEST_CATEGORY: integration
                                                                // TEST_DOMAIN: security
                                                                // TEST_PRIORITY: normal

        assert_eq!(key_16.len(), 16);
        assert_eq!(key_24.len(), 24);
        assert_eq!(key_32.len(), 32);

        // All should be different
        assert_ne!(&key_16[..], &key_24[..16]);
        assert_ne!(&key_24[..], &key_32[..24]);
    }

    #[test]
    fn test_key_derivation_consistency() {
        // Same password + salt should always derive same key
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let password = b"StrongPassword123!";
        let salt = BearDogCrypto::generate_secure_random(16);

        let key1 = BearDogCrypto::derive_pbkdf2_key(password, &salt, 10000, 32)
            .expect("Key derivation should succeed");
        let key2 = BearDogCrypto::derive_pbkdf2_key(password, &salt, 10000, 32)
            .expect("Key derivation should succeed");

        assert_eq!(key1, key2, "Same inputs should derive same key");
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_key_derivation_different_salts() {
        // Different salts should produce different keys
        let password = b"StrongPassword123!";
        let salt1 = BearDogCrypto::generate_secure_random(16);
        let salt2 = BearDogCrypto::generate_secure_random(16);

        let key1 = BearDogCrypto::derive_pbkdf2_key(password, &salt1, 10000, 32)
            .expect("Key derivation should succeed");
        let key2 = BearDogCrypto::derive_pbkdf2_key(password, &salt2, 10000, 32)
            .expect("Key derivation should succeed");

        assert_ne!(key1, key2, "Different salts should produce different keys");
    }
}
