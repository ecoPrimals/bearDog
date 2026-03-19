// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive tests for EncryptionService
//!
//! Coverage boost: Tests encryption/decryption operations, algorithms, error paths

#[cfg(test)]
mod tests {
    use crate::encryption::{EncryptionAlgorithm, EncryptionConfig, EncryptionService};
    use beardog_errors::BearDogError;

    fn get_test_key() -> Vec<u8> {
        vec![0u8; 32] // 32-byte key for AES-256
    }

    #[test]
    fn test_create_service_with_defaults() {
        let service = EncryptionService::new(EncryptionConfig::default());
        assert!(service.is_initialized());
    }

    #[test]
    fn test_create_service_aes_gcm() {
        let config = EncryptionConfig {
            algorithm: EncryptionAlgorithm::Aes256Gcm,
            key_size: 32,
        };
        let service = EncryptionService::new(config);
        assert!(service.is_initialized());
    }

    #[test]
    fn test_create_service_chacha20() {
        let config = EncryptionConfig {
            algorithm: EncryptionAlgorithm::ChaCha20Poly1305,
            key_size: 32,
        };
        let service = EncryptionService::new(config);
        assert!(service.is_initialized());
    }

    #[test]
    fn test_encrypt_decrypt_round_trip() -> Result<(), BearDogError> {
        let service = EncryptionService::new(EncryptionConfig::default());
        let key = get_test_key();
        let plaintext = b"Hello, BearDog!";

        let ciphertext = service.encrypt(plaintext, &key)?;
        assert_ne!(ciphertext, plaintext);
        assert!(ciphertext.len() > plaintext.len()); // Includes nonce + auth tag

        let decrypted = service.decrypt(&ciphertext, &key)?;
        assert_eq!(decrypted, plaintext);

        Ok(())
    }

    #[test]
    fn test_encrypt_empty_data() -> Result<(), BearDogError> {
        let service = EncryptionService::new(EncryptionConfig::default());
        let key = get_test_key();
        let plaintext = b"";

        let ciphertext = service.encrypt(plaintext, &key)?;
        let decrypted = service.decrypt(&ciphertext, &key)?;
        assert_eq!(decrypted, plaintext);

        Ok(())
    }

    #[test]
    fn test_encrypt_large_data() -> Result<(), BearDogError> {
        let service = EncryptionService::new(EncryptionConfig::default());
        let key = get_test_key();
        let plaintext = vec![0x42; 1024 * 1024]; // 1 MB

        let ciphertext = service.encrypt(&plaintext, &key)?;
        let decrypted = service.decrypt(&ciphertext, &key)?;
        assert_eq!(decrypted, plaintext);

        Ok(())
    }

    #[test]
    fn test_encrypt_with_invalid_key_size() {
        let service = EncryptionService::new(EncryptionConfig::default());
        let invalid_key = vec![0u8; 16]; // Wrong size (should be 32)
        let plaintext = b"test data";

        let result = service.encrypt(plaintext, &invalid_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_with_invalid_key_size() {
        let service = EncryptionService::new(EncryptionConfig::default());
        let key = get_test_key();
        let plaintext = b"test data";

        let ciphertext = service.encrypt(plaintext, &key).unwrap();

        let invalid_key = vec![0u8; 16];
        let result = service.decrypt(&ciphertext, &invalid_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_with_wrong_key() {
        let service = EncryptionService::new(EncryptionConfig::default());
        let key1 = get_test_key();
        let key2 = vec![0xFFu8; 32]; // Different key
        let plaintext = b"secret data";

        let ciphertext = service.encrypt(plaintext, &key1).unwrap();
        let result = service.decrypt(&ciphertext, &key2);

        // Should fail authentication
        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_corrupted_ciphertext() {
        let service = EncryptionService::new(EncryptionConfig::default());
        let key = get_test_key();
        let plaintext = b"test data";

        let mut ciphertext = service.encrypt(plaintext, &key).unwrap();

        // Corrupt the ciphertext
        if ciphertext.len() > 20 {
            ciphertext[20] ^= 0xFF;
        }

        let result = service.decrypt(&ciphertext, &key);
        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_too_short_ciphertext() {
        let service = EncryptionService::new(EncryptionConfig::default());
        let key = get_test_key();
        let invalid_ciphertext = vec![0u8; 5]; // Too short to contain nonce + tag

        let result = service.decrypt(&invalid_ciphertext, &key);
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_encryptions_different_nonces() -> Result<(), BearDogError> {
        let service = EncryptionService::new(EncryptionConfig::default());
        let key = get_test_key();
        let plaintext = b"same data";

        let ciphertext1 = service.encrypt(plaintext, &key)?;
        let ciphertext2 = service.encrypt(plaintext, &key)?;

        // Same plaintext should produce different ciphertexts (due to random nonces)
        assert_ne!(ciphertext1, ciphertext2);

        // Both should decrypt correctly
        let decrypted1 = service.decrypt(&ciphertext1, &key)?;
        let decrypted2 = service.decrypt(&ciphertext2, &key)?;
        assert_eq!(decrypted1, plaintext);
        assert_eq!(decrypted2, plaintext);

        Ok(())
    }

    #[test]
    fn test_encrypt_decrypt_binary_data() -> Result<(), BearDogError> {
        let service = EncryptionService::new(EncryptionConfig::default());
        let key = get_test_key();
        let plaintext: Vec<u8> = (0..=255).collect();

        let ciphertext = service.encrypt(&plaintext, &key)?;
        let decrypted = service.decrypt(&ciphertext, &key)?;
        assert_eq!(decrypted, plaintext);

        Ok(())
    }

    #[test]
    fn test_encrypt_decrypt_null_bytes() -> Result<(), BearDogError> {
        let service = EncryptionService::new(EncryptionConfig::default());
        let key = get_test_key();
        let plaintext = vec![0u8; 100];

        let ciphertext = service.encrypt(&plaintext, &key)?;
        let decrypted = service.decrypt(&ciphertext, &key)?;
        assert_eq!(decrypted, plaintext);

        Ok(())
    }

    #[test]
    fn test_encrypt_decrypt_max_bytes() -> Result<(), BearDogError> {
        let service = EncryptionService::new(EncryptionConfig::default());
        let key = get_test_key();
        let plaintext = vec![0xFFu8; 100];

        let ciphertext = service.encrypt(&plaintext, &key)?;
        let decrypted = service.decrypt(&ciphertext, &key)?;
        assert_eq!(decrypted, plaintext);

        Ok(())
    }

    #[test]
    fn test_encrypt_decrypt_unicode() -> Result<(), BearDogError> {
        let service = EncryptionService::new(EncryptionConfig::default());
        let key = get_test_key();
        let plaintext = "Hello 世界 🐻".as_bytes();

        let ciphertext = service.encrypt(plaintext, &key)?;
        let decrypted = service.decrypt(&ciphertext, &key)?;
        assert_eq!(decrypted, plaintext);
        assert_eq!(String::from_utf8(decrypted).unwrap(), "Hello 世界 🐻");

        Ok(())
    }

    #[test]
    fn test_encrypt_data_method() -> Result<(), BearDogError> {
        let service = EncryptionService::new(EncryptionConfig::default());
        let key = get_test_key();
        let plaintext = b"test via encrypt_data";

        let ciphertext = service.encrypt_data(plaintext, &key)?;
        let decrypted = service.decrypt_data(&ciphertext, &key)?;
        assert_eq!(decrypted, plaintext);

        Ok(())
    }

    #[test]
    fn test_algorithm_serialization() {
        let aes = EncryptionAlgorithm::Aes256Gcm;
        let chacha = EncryptionAlgorithm::ChaCha20Poly1305;

        // Test Debug
        assert_eq!(format!("{:?}", aes), "Aes256Gcm");
        assert_eq!(format!("{:?}", chacha), "ChaCha20Poly1305");

        // Test Clone
        let aes_clone = aes.clone();
        assert_eq!(format!("{:?}", aes_clone), "Aes256Gcm");
    }

    #[test]
    fn test_algorithm_default() {
        let default_algo = EncryptionAlgorithm::default();
        assert_eq!(format!("{:?}", default_algo), "Aes256Gcm");
    }

    #[test]
    fn test_config_default() {
        let config = EncryptionConfig::default();
        assert_eq!(config.key_size, 32);
        assert_eq!(format!("{:?}", config.algorithm), "Aes256Gcm");
    }

    #[test]
    fn test_different_key_sizes_in_config() {
        // Config allows different key sizes but encryption validates
        let config = EncryptionConfig {
            algorithm: EncryptionAlgorithm::Aes256Gcm,
            key_size: 16, // Non-standard
        };
        let service = EncryptionService::new(config);

        let key = vec![0u8; 16];
        let result = service.encrypt(b"test", &key);

        // Should fail because AES-256-GCM requires 32-byte key
        assert!(result.is_err());
    }

    #[test]
    fn test_concurrent_encryptions() -> Result<(), BearDogError> {
        use std::sync::Arc;
        use std::thread;

        let service = Arc::new(EncryptionService::new(EncryptionConfig::default()));
        let key = Arc::new(get_test_key());

        let mut handles = vec![];
        for i in 0..10 {
            let service = Arc::clone(&service);
            let key = Arc::clone(&key);

            let handle = thread::spawn(move || {
                let plaintext = format!("message {}", i);
                let ciphertext = service.encrypt(plaintext.as_bytes(), &key).unwrap();
                service.decrypt(&ciphertext, &key).unwrap()
            });
            handles.push(handle);
        }

        for handle in handles {
            let _ = handle.join().unwrap();
        }

        Ok(())
    }

    #[test]
    fn test_ciphertext_structure() -> Result<(), BearDogError> {
        let service = EncryptionService::new(EncryptionConfig::default());
        let key = get_test_key();
        let plaintext = b"test";

        let ciphertext = service.encrypt(plaintext, &key)?;

        // AES-256-GCM: nonce (12 bytes) + ciphertext + auth tag (16 bytes)
        // Minimum size should be 12 + 4 + 16 = 32 bytes
        assert!(ciphertext.len() >= 12 + plaintext.len() + 16);

        Ok(())
    }

    #[test]
    fn test_encrypt_decrypt_edge_case_sizes() -> Result<(), BearDogError> {
        let service = EncryptionService::new(EncryptionConfig::default());
        let key = get_test_key();

        // Test various sizes
        for size in [1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129] {
            let plaintext = vec![0x42u8; size];
            let ciphertext = service.encrypt(&plaintext, &key)?;
            let decrypted = service.decrypt(&ciphertext, &key)?;
            assert_eq!(decrypted, plaintext, "Failed for size {}", size);
        }

        Ok(())
    }
}
