// SPDX-License-Identifier: AGPL-3.0-only

//! Enhanced tests for crypto_utils
//!
//! Coverage boost: Tests hash functions, key derivation, and edge cases

#[cfg(test)]
mod tests {
    use crate::crypto_utils::BearDogCrypto;
    use beardog_errors::BearDogError;

    #[test]
    fn test_encrypt_aes_gcm_basic() -> Result<(), BearDogError> {
        let key = [0u8; 32];
        let plaintext = b"test data";

        let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, plaintext, None)?;
        
        assert!(!ciphertext.is_empty());
        assert_eq!(nonce.len(), 12); // AES-GCM nonce is 12 bytes
        assert_ne!(ciphertext, plaintext);

        Ok(())
    }

    #[test]
    fn test_encrypt_aes_gcm_with_custom_nonce() -> Result<(), BearDogError> {
        let key = [0u8; 32];
        let plaintext = b"test data";
        let custom_nonce = [0x42u8; 12];

        let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, plaintext, Some(&custom_nonce))?;
        
        assert_eq!(nonce, custom_nonce);
        assert!(!ciphertext.is_empty());

        Ok(())
    }

    #[test]
    fn test_decrypt_aes_gcm_round_trip() -> Result<(), BearDogError> {
        let key = [0u8; 32];
        let plaintext = b"round trip test";

        let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, plaintext, None)?;
        let decrypted = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce, None)?;
        
        assert_eq!(decrypted, plaintext);

        Ok(())
    }

    #[test]
    fn test_decrypt_aes_gcm_with_aad() -> Result<(), BearDogError> {
        let key = [0u8; 32];
        let plaintext = b"authenticated data";
        let aad = b"additional auth data";

        let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, plaintext, None)?;
        let decrypted = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce, Some(aad))?;
        
        assert_eq!(decrypted, plaintext);

        Ok(())
    }

    #[test]
    fn test_decrypt_aes_gcm_wrong_key_fails() {
        let key1 = [0u8; 32];
        let key2 = [0xFFu8; 32];
        let plaintext = b"secret";

        let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key1, plaintext, None).unwrap();
        let result = BearDogCrypto::decrypt_aes_gcm(&key2, &ciphertext, &nonce, None);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_aes_gcm_corrupted_ciphertext_fails() {
        let key = [0u8; 32];
        let plaintext = b"data to corrupt";

        let (mut ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, plaintext, None).unwrap();
        
        // Corrupt the ciphertext
        if !ciphertext.is_empty() {
            ciphertext[0] ^= 0xFF;
        }
        
        let result = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_random_bytes_default_size() -> Result<(), BearDogError> {
        let random1 = BearDogCrypto::generate_random_bytes(32)?;
        let random2 = BearDogCrypto::generate_random_bytes(32)?;
        
        assert_eq!(random1.len(), 32);
        assert_eq!(random2.len(), 32);
        assert_ne!(random1, random2); // Should be different

        Ok(())
    }

    #[test]
    fn test_generate_random_bytes_various_sizes() -> Result<(), BearDogError> {
        for size in [8, 16, 24, 32, 48, 64, 128, 256] {
            let random = BearDogCrypto::generate_random_bytes(size)?;
            assert_eq!(random.len(), size);
        }

        Ok(())
    }

    #[test]
    fn test_generate_random_bytes_zero_size() -> Result<(), BearDogError> {
        let random = BearDogCrypto::generate_random_bytes(0)?;
        assert_eq!(random.len(), 0);

        Ok(())
    }

    #[test]
    fn test_hash_sha256_basic() -> Result<(), BearDogError> {
        let data = b"hello world";
        let hash = BearDogCrypto::hash_sha256(data)?;
        
        assert_eq!(hash.len(), 32); // SHA-256 is 32 bytes

        Ok(())
    }

    #[test]
    fn test_hash_sha256_consistency() -> Result<(), BearDogError> {
        let data = b"consistent data";
        
        let hash1 = BearDogCrypto::hash_sha256(data)?;
        let hash2 = BearDogCrypto::hash_sha256(data)?;
        
        assert_eq!(hash1, hash2);

        Ok(())
    }

    #[test]
    fn test_hash_sha256_different_inputs() -> Result<(), BearDogError> {
        let data1 = b"input 1";
        let data2 = b"input 2";
        
        let hash1 = BearDogCrypto::hash_sha256(data1)?;
        let hash2 = BearDogCrypto::hash_sha256(data2)?;
        
        assert_ne!(hash1, hash2);

        Ok(())
    }

    #[test]
    fn test_hash_sha256_empty_input() -> Result<(), BearDogError> {
        let hash = BearDogCrypto::hash_sha256(b"")?;
        assert_eq!(hash.len(), 32);

        Ok(())
    }

    #[test]
    fn test_hash_sha512_basic() -> Result<(), BearDogError> {
        let data = b"sha512 test";
        let hash = BearDogCrypto::hash_sha512(data)?;
        
        assert_eq!(hash.len(), 64); // SHA-512 is 64 bytes

        Ok(())
    }

    #[test]
    fn test_hash_sha512_consistency() -> Result<(), BearDogError> {
        let data = b"sha512 consistent";
        
        let hash1 = BearDogCrypto::hash_sha512(data)?;
        let hash2 = BearDogCrypto::hash_sha512(data)?;
        
        assert_eq!(hash1, hash2);

        Ok(())
    }

    #[test]
    fn test_derive_key_from_password_basic() -> Result<(), BearDogError> {
        let password = b"strong_password";
        let salt = b"random_salt";
        
        let key = BearDogCrypto::derive_key_from_password(password, salt, 1000)?;
        
        assert!(!key.is_empty());

        Ok(())
    }

    #[test]
    fn test_derive_key_from_password_consistency() -> Result<(), BearDogError> {
        let password = b"password123";
        let salt = b"salt123";
        
        let key1 = BearDogCrypto::derive_key_from_password(password, salt, 1000)?;
        let key2 = BearDogCrypto::derive_key_from_password(password, salt, 1000)?;
        
        assert_eq!(key1, key2);

        Ok(())
    }

    #[test]
    fn test_derive_key_from_password_different_salts() -> Result<(), BearDogError> {
        let password = b"password";
        let salt1 = b"salt1";
        let salt2 = b"salt2";
        
        let key1 = BearDogCrypto::derive_key_from_password(password, salt1, 1000)?;
        let key2 = BearDogCrypto::derive_key_from_password(password, salt2, 1000)?;
        
        assert_ne!(key1, key2);

        Ok(())
    }

    #[test]
    fn test_derive_key_from_password_different_iterations() -> Result<(), BearDogError> {
        let password = b"password";
        let salt = b"salt";
        
        let key1 = BearDogCrypto::derive_key_from_password(password, salt, 1000)?;
        let key2 = BearDogCrypto::derive_key_from_password(password, salt, 2000)?;
        
        assert_ne!(key1, key2);

        Ok(())
    }

    #[test]
    fn test_constant_time_compare_equal() {
        let a = b"same_value";
        let b = b"same_value";
        
        assert!(BearDogCrypto::constant_time_compare(a, b));
    }

    #[test]
    fn test_constant_time_compare_different() {
        let a = b"value1";
        let b = b"value2";
        
        assert!(!BearDogCrypto::constant_time_compare(a, b));
    }

    #[test]
    fn test_constant_time_compare_different_lengths() {
        let a = b"short";
        let b = b"longer_value";
        
        assert!(!BearDogCrypto::constant_time_compare(a, b));
    }

    #[test]
    fn test_constant_time_compare_empty() {
        let a = b"";
        let b = b"";
        
        assert!(BearDogCrypto::constant_time_compare(a, b));
    }

    #[test]
    fn test_encrypt_decrypt_large_data() -> Result<(), BearDogError> {
        let key = [0u8; 32];
        let plaintext = vec![0x42u8; 10000]; // 10 KB

        let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, &plaintext, None)?;
        let decrypted = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce, None)?;
        
        assert_eq!(decrypted, plaintext);

        Ok(())
    }

    #[test]
    fn test_hash_large_data() -> Result<(), BearDogError> {
        let data = vec![0xFFu8; 100000]; // 100 KB
        
        let hash = BearDogCrypto::hash_sha256(&data)?;
        assert_eq!(hash.len(), 32);

        Ok(())
    }

    #[test]
    fn test_concurrent_hash_operations() -> Result<(), BearDogError> {
        use std::thread;
        
        let handles: Vec<_> = (0..10)
            .map(|i| {
                thread::spawn(move || {
                    let data = format!("data_{}", i);
                    BearDogCrypto::hash_sha256(data.as_bytes()).unwrap()
                })
            })
            .collect();

        for handle in handles {
            let hash = handle.join().unwrap();
            assert_eq!(hash.len(), 32);
        }

        Ok(())
    }

    #[test]
    fn test_encrypt_with_max_bytes() -> Result<(), BearDogError> {
        let key = [0xFFu8; 32];
        let plaintext = [0xFFu8; 100];

        let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, &plaintext, None)?;
        let decrypted = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce, None)?;
        
        assert_eq!(decrypted.as_slice(), &plaintext);

        Ok(())
    }

    #[test]
    fn test_hash_binary_data() -> Result<(), BearDogError> {
        let data: Vec<u8> = (0..=255).collect();
        
        let hash = BearDogCrypto::hash_sha256(&data)?;
        assert_eq!(hash.len(), 32);

        Ok(())
    }
}

