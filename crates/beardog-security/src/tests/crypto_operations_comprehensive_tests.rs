//! Comprehensive Cryptographic Operations Tests
//!
//! High-value security tests covering core cryptographic operations
//! including Ed25519 signatures, AES-256-GCM, ChaCha20-Poly1305, and BLAKE3.

use beardog_errors::BearDogError;

#[cfg(test)]
mod ed25519_signature_tests {
    use super::*;

    #[test]
    fn test_ed25519_keypair_generation() -> Result<(), BearDogError> {
        // Test that we can generate a valid Ed25519 keypair
        let keypair = generate_ed25519_keypair()?;

        assert!(keypair.public.len() == 32, "Public key should be 32 bytes");
        assert!(keypair.secret.len() == 32, "Secret key should be 32 bytes");

        Ok(())
    }

    #[test]
    fn test_ed25519_signature_verification_valid() -> Result<(), BearDogError> {
        // Test signing and verification with valid signature
        let keypair = generate_ed25519_keypair()?;
        let message = b"Test message for Ed25519 signature";

        let signature = sign_ed25519(&keypair.secret, message)?;
        let is_valid = verify_ed25519_signature(&keypair.public, message, &signature)?;

        assert!(is_valid, "Valid signature should verify successfully");

        Ok(())
    }

    #[test]
    fn test_ed25519_signature_verification_invalid_message() -> Result<(), BearDogError> {
        // Test that verification fails with wrong message
        let keypair = generate_ed25519_keypair()?;
        let message = b"Original message";
        let wrong_message = b"Tampered message";

        let signature = sign_ed25519(&keypair.secret, message)?;
        let is_valid = verify_ed25519_signature(&keypair.public, wrong_message, &signature)?;

        assert!(
            !is_valid,
            "Signature should fail verification with wrong message"
        );

        Ok(())
    }

    #[test]
    fn test_ed25519_signature_verification_invalid_signature() -> Result<(), BearDogError> {
        // Test that verification fails with tampered signature
        let keypair = generate_ed25519_keypair()?;
        let message = b"Test message";

        let mut signature = sign_ed25519(&keypair.secret, message)?;
        // Tamper with signature
        signature[0] ^= 0xFF;

        let is_valid = verify_ed25519_signature(&keypair.public, message, &signature)?;

        assert!(!is_valid, "Tampered signature should fail verification");

        Ok(())
    }

    #[test]
    fn test_ed25519_multiple_signatures() -> Result<(), BearDogError> {
        // Test multiple signatures with same keypair
        let keypair = generate_ed25519_keypair()?;
        let messages = vec![
            b"Message 1".as_slice(),
            b"Message 2".as_slice(),
            b"Message 3".as_slice(),
        ];

        for message in messages {
            let signature = sign_ed25519(&keypair.secret, message)?;
            let is_valid = verify_ed25519_signature(&keypair.public, message, &signature)?;
            assert!(is_valid, "Each signature should verify independently");
        }

        Ok(())
    }
}

#[cfg(test)]
// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
mod aes_gcm_encryption_tests {
    use super::*;

    #[test]
    fn test_aes_256_gcm_encryption_decryption() -> Result<(), BearDogError> {
        // Test basic AES-256-GCM encryption and decryption
        let plaintext = b"Sensitive data to encrypt with AES-256-GCM";
        let key = generate_aes_256_key()?;
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let nonce = generate_aes_nonce()?;

        let ciphertext = encrypt_aes_256_gcm(&key, &nonce, plaintext)?;
        assert_ne!(
            ciphertext.as_slice(),
            plaintext,
            "Ciphertext should differ from plaintext"
        );

        let decrypted = decrypt_aes_256_gcm(&key, &nonce, &ciphertext)?;
        assert_eq!(
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: security
            // TEST_PRIORITY: important
            decrypted.as_slice(),
            plaintext,
            "Decrypted data should match original"
        );

        Ok(())
    }

    #[test]
    fn test_aes_256_gcm_wrong_key() -> Result<(), BearDogError> {
        // Test that decryption fails with wrong key
        let plaintext = b"Secret data";
        let key = generate_aes_256_key()?;
        let wrong_key = generate_aes_256_key()?;
        let nonce = generate_aes_nonce()?;
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: important

        let ciphertext = encrypt_aes_256_gcm(&key, &nonce, plaintext)?;
        let result = decrypt_aes_256_gcm(&wrong_key, &nonce, &ciphertext);

        assert!(result.is_err(), "Decryption with wrong key should fail");

        Ok(())
    }

    #[test]
    fn test_aes_256_gcm_wrong_nonce() -> Result<(), BearDogError> {
        // Test that decryption fails with wrong nonce
        let plaintext = b"Secret data";
        let key = generate_aes_256_key()?;
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let nonce = generate_aes_nonce()?;
        let wrong_nonce = generate_aes_nonce()?;

        let ciphertext = encrypt_aes_256_gcm(&key, &nonce, plaintext)?;
        let result = decrypt_aes_256_gcm(&key, &wrong_nonce, &ciphertext);

        assert!(result.is_err(), "Decryption with wrong nonce should fail");

        Ok(())
    }

    #[test]
    fn test_aes_256_gcm_tampered_ciphertext() -> Result<(), BearDogError> {
        // Test that decryption fails with tampered ciphertext
        let plaintext = b"Secret data";
        let key = generate_aes_256_key()?;
        let nonce = generate_aes_nonce()?;

        let mut ciphertext = encrypt_aes_256_gcm(&key, &nonce, plaintext)?;
        // Tamper with ciphertext
        if !ciphertext.is_empty() {
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: security
            // TEST_PRIORITY: critical
            ciphertext[0] ^= 0xFF;
        }

        let result = decrypt_aes_256_gcm(&key, &nonce, &ciphertext);

        assert!(
            result.is_err(),
            "Decryption of tampered ciphertext should fail"
        );

        Ok(())
    }

    #[test]
    fn test_aes_256_gcm_empty_plaintext() -> Result<(), BearDogError> {
        // Test encryption/decryption of empty data
        let plaintext = b"";
        let key = generate_aes_256_key()?;
        let nonce = generate_aes_nonce()?;

        let ciphertext = encrypt_aes_256_gcm(&key, &nonce, plaintext)?;
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let decrypted = decrypt_aes_256_gcm(&key, &nonce, &ciphertext)?;

        assert_eq!(
            decrypted.as_slice(),
            plaintext,
            "Empty plaintext should round-trip"
        );

        Ok(())
    }

    #[test]
    fn test_aes_256_gcm_large_plaintext() -> Result<(), BearDogError> {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        // Test encryption/decryption of large data (1MB)
        let plaintext = vec![0x42u8; 1024 * 1024]; // 1MB
        let key = generate_aes_256_key()?;
        let nonce = generate_aes_nonce()?;

        let ciphertext = encrypt_aes_256_gcm(&key, &nonce, &plaintext)?;
        let decrypted = decrypt_aes_256_gcm(&key, &nonce, &ciphertext)?;

        assert_eq!(
            decrypted, plaintext,
            "Large plaintext should round-trip correctly"
        );

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        Ok(())
    }
}

#[cfg(test)]
mod chacha20_poly1305_tests {
    use super::*;

    #[test]
    fn test_chacha20_poly1305_encryption_decryption() -> Result<(), BearDogError> {
        // Test ChaCha20-Poly1305 encryption and decryption
        let plaintext = b"Data encrypted with ChaCha20-Poly1305";
        let key = generate_chacha20_key()?;
        let nonce = generate_chacha20_nonce()?;

        let ciphertext = encrypt_chacha20_poly1305(&key, &nonce, plaintext)?;
        assert_ne!(ciphertext.as_slice(), plaintext, "Ciphertext should differ");

        let decrypted = decrypt_chacha20_poly1305(&key, &nonce, &ciphertext)?;
        assert_eq!(decrypted.as_slice(), plaintext, "Decryption should match");
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal

        Ok(())
    }

    #[test]
    fn test_chacha20_poly1305_authentication_failure() -> Result<(), BearDogError> {
        // Test that authentication tag prevents tampering
        let plaintext = b"Authenticated data";
        let key = generate_chacha20_key()?;
        let nonce = generate_chacha20_nonce()?;

        let mut ciphertext = encrypt_chacha20_poly1305(&key, &nonce, plaintext)?;
        // Tamper with ciphertext
        if !ciphertext.is_empty() {
            ciphertext[0] ^= 0xFF;
        }
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal

        let result = decrypt_chacha20_poly1305(&key, &nonce, &ciphertext);

        assert!(result.is_err(), "Authentication should catch tampering");

        Ok(())
    }

    #[test]
    fn test_chacha20_poly1305_with_aad() -> Result<(), BearDogError> {
        // Test encryption with additional authenticated data
        let plaintext = b"Secret payload";
        let aad = b"Public header data";
        let key = generate_chacha20_key()?;
        let nonce = generate_chacha20_nonce()?;

        let ciphertext = encrypt_chacha20_poly1305_with_aad(&key, &nonce, plaintext, aad)?;
        let decrypted = decrypt_chacha20_poly1305_with_aad(&key, &nonce, &ciphertext, aad)?;

        assert_eq!(
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: security
            // TEST_PRIORITY: critical
            decrypted.as_slice(),
            plaintext,
            "AAD should not affect decryption"
        );

        Ok(())
    }

    #[test]
    fn test_chacha20_poly1305_wrong_aad() -> Result<(), BearDogError> {
        // Test that wrong AAD causes authentication failure
        let plaintext = b"Secret data";
        let aad = b"Correct AAD";
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical
        let wrong_aad = b"Wrong AAD!!";
        let key = generate_chacha20_key()?;
        let nonce = generate_chacha20_nonce()?;

        let ciphertext = encrypt_chacha20_poly1305_with_aad(&key, &nonce, plaintext, aad)?;
        let result = decrypt_chacha20_poly1305_with_aad(&key, &nonce, &ciphertext, wrong_aad);

        assert!(
            result.is_err(),
            "Wrong AAD should cause authentication failure"
        );

        Ok(())
    }
}

#[cfg(test)]
// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
mod blake3_hashing_tests {
    use super::*;

    #[test]
    fn test_blake3_hash_generation() -> Result<(), BearDogError> {
        // Test BLAKE3 hash generation
        let data = b"Data to hash with BLAKE3";

        let hash = blake3_hash(data)?;

        assert_eq!(hash.len(), 32, "BLAKE3 hash should be 32 bytes");

        Ok(())
    }

    #[test]
    fn test_blake3_hash_deterministic() -> Result<(), BearDogError> {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        // Test that same input produces same hash
        let data = b"Deterministic input";

        let hash1 = blake3_hash(data)?;
        let hash2 = blake3_hash(data)?;

        assert_eq!(hash1, hash2, "BLAKE3 should be deterministic");

        Ok(())
    }

    #[test]
    fn test_blake3_hash_different_inputs() -> Result<(), BearDogError> {
        // Test that different inputs produce different hashes
        let data1 = b"Input 1";
        let data2 = b"Input 2";

        let hash1 = blake3_hash(data1)?;
        let hash2 = blake3_hash(data2)?;

        assert_ne!(
            hash1, hash2,
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: security
            // TEST_PRIORITY: normal
            "Different inputs should produce different hashes"
        );

        Ok(())
    }

    #[test]
    fn test_blake3_hash_empty_input() -> Result<(), BearDogError> {
        // Test hashing empty data
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let data = b"";

        let hash = blake3_hash(data)?;

        assert_eq!(
            hash.len(),
            32,
            "Empty input should still produce 32-byte hash"
        );

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        Ok(())
    }

    #[test]
    fn test_blake3_hash_large_input() -> Result<(), BearDogError> {
        // Test hashing large data (10MB)
        let data = vec![0xAAu8; 10 * 1024 * 1024]; // 10MB

        let hash = blake3_hash(&data)?;

        assert_eq!(hash.len(), 32, "Large input should produce 32-byte hash");

        Ok(())
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    #[test]
    fn test_blake3_keyed_hash() -> Result<(), BearDogError> {
        // Test keyed hashing (HMAC-like)
        let data = b"Data to authenticate";
        let key = b"BLAKE3-secret-key-32bytes-here!!"; // Exactly 32 bytes

        let mac = blake3_keyed_hash(key, data)?;

        assert_eq!(mac.len(), 32, "Keyed hash should be 32 bytes");

        Ok(())
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    #[test]
    fn test_blake3_keyed_hash_different_keys() -> Result<(), BearDogError> {
        // Test that different keys produce different MACs
        let data = b"Authenticated data";
        let key1 = b"Key-1-for-BLAKE3-authentication!"; // Exactly 32 bytes
        let key2 = b"Key-2-for-BLAKE3-authentication!"; // Exactly 32 bytes

        let mac1 = blake3_keyed_hash(key1, data)?;
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let mac2 = blake3_keyed_hash(key2, data)?;

        assert_ne!(mac1, mac2, "Different keys should produce different MACs");

        Ok(())
    }
}

#[cfg(test)]
mod key_derivation_tests {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    use super::*;

    #[test]
    fn test_pbkdf2_key_derivation() -> Result<(), BearDogError> {
        // Test PBKDF2 key derivation
        let password = b"User password";
        let salt = b"Random salt value";
        let iterations = 100_000;

        let derived_key = derive_key_pbkdf2(password, salt, iterations)?;

        assert_eq!(derived_key.len(), 32, "Derived key should be 32 bytes");

        Ok(())
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    fn test_pbkdf2_deterministic() -> Result<(), BearDogError> {
        // Test that same inputs produce same key
        let password = b"password123";
        let salt = b"salt123";
        let iterations = 10_000;

        let key1 = derive_key_pbkdf2(password, salt, iterations)?;
        let key2 = derive_key_pbkdf2(password, salt, iterations)?;

        assert_eq!(key1, key2, "PBKDF2 should be deterministic");

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        Ok(())
    }

    #[test]
    fn test_pbkdf2_different_salts() -> Result<(), BearDogError> {
        // Test that different salts produce different keys
        let password = b"same password";
        let salt1 = b"salt 1";
        let salt2 = b"salt 2";
        let iterations = 10_000;

        let key1 = derive_key_pbkdf2(password, salt1, iterations)?;
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let key2 = derive_key_pbkdf2(password, salt2, iterations)?;

        assert_ne!(key1, key2, "Different salts should produce different keys");

        Ok(())
    }

    #[test]
    fn test_argon2_key_derivation() -> Result<(), BearDogError> {
        // Test Argon2 key derivation (memory-hard)
        let password = b"User password for Argon2";
        let salt = b"Random salt for Argon2!!";

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let derived_key = derive_key_argon2(password, salt)?;

        assert_eq!(derived_key.len(), 32, "Argon2 key should be 32 bytes");

        Ok(())
    }
}

#[cfg(test)]
mod key_rotation_tests {
    use super::*;

    #[test]
    fn test_key_rotation_workflow() -> Result<(), BearDogError> {
        // Test complete key rotation workflow
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let old_key = generate_aes_256_key()?;
        let new_key = generate_aes_256_key()?;
        let nonce = generate_aes_nonce()?;
        let plaintext = b"Data encrypted with old key";

        // Encrypt with old key
        let ciphertext_old = encrypt_aes_256_gcm(&old_key, &nonce, plaintext)?;

        // Decrypt with old key
        let decrypted = decrypt_aes_256_gcm(&old_key, &nonce, &ciphertext_old)?;

        // Re-encrypt with new key
        let ciphertext_new = encrypt_aes_256_gcm(&new_key, &nonce, &decrypted)?;

        // Verify new encryption works
        let final_decrypted = decrypt_aes_256_gcm(&new_key, &nonce, &ciphertext_new)?;

        assert_eq!(
            final_decrypted.as_slice(),
            plaintext,
            "Key rotation should preserve data"
        );

        Ok(())
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
    #[test]
    fn test_multi_key_decryption_capability() -> Result<(), BearDogError> {
        // Test that we can try multiple keys until one works
        let keys = vec![
            generate_aes_256_key()?,
            generate_aes_256_key()?,
            generate_aes_256_key()?,
        ];
        let correct_key = &keys[1]; // Use middle key
        let nonce = generate_aes_nonce()?;
        let plaintext = b"Data encrypted with one of multiple keys";

        let ciphertext = encrypt_aes_256_gcm(correct_key, &nonce, plaintext)?;

        // Try each key until one works
        let mut decrypted = None;
        for key in &keys {
            if let Ok(data) = decrypt_aes_256_gcm(key, &nonce, &ciphertext) {
                decrypted = Some(data);
                break;
            }
        }

        assert!(decrypted.is_some(), "Should find correct key");
        let decrypted_data = decrypted
            .ok_or_else(|| BearDogError::security("Failed to decrypt with any key".to_string()))?;
        assert_eq!(
            decrypted_data.as_slice(),
            plaintext,
            "Decryption should work"
        );

        Ok(())
    }
}

// Helper functions (stubs - implement based on actual crypto_utils API)

#[allow(dead_code)]
fn generate_ed25519_keypair() -> Result<Ed25519Keypair, BearDogError> {
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;
    use rand::RngCore;

    let mut csprng = OsRng;
    let mut seed = [0u8; 32];
    csprng.fill_bytes(&mut seed);
    let signing_key = SigningKey::from_bytes(&seed);
    let verifying_key = signing_key.verifying_key();

    Ok(Ed25519Keypair {
        public: verifying_key.to_bytes().to_vec(),
        secret: signing_key.to_bytes().to_vec(),
    })
}

#[allow(dead_code)]
fn sign_ed25519(secret: &[u8], message: &[u8]) -> Result<Vec<u8>, BearDogError> {
    use ed25519_dalek::{Signer, SigningKey};

    if secret.len() != 32 {
        return Err(BearDogError::validation(
            "Ed25519 secret key must be 32 bytes",
        ));
    }

    let secret_array: [u8; 32] = secret
        .try_into()
        .map_err(|_| BearDogError::validation("Invalid secret key length"))?;

    let signing_key = SigningKey::from_bytes(&secret_array);
    let signature = signing_key.sign(message);

    Ok(signature.to_bytes().to_vec())
}

#[allow(dead_code)]
fn verify_ed25519_signature(
    public: &[u8],
    message: &[u8],
    signature: &[u8],
) -> Result<bool, BearDogError> {
    use ed25519_dalek::{Signature, Verifier, VerifyingKey};

    if public.len() != 32 {
        return Err(BearDogError::validation(
            "Ed25519 public key must be 32 bytes",
        ));
    }

    if signature.len() != 64 {
        return Err(BearDogError::validation(
            "Ed25519 signature must be 64 bytes",
        ));
    }

    let public_array: [u8; 32] = public
        .try_into()
        .map_err(|_| BearDogError::validation("Invalid public key length"))?;

    let sig_array: [u8; 64] = signature
        .try_into()
        .map_err(|_| BearDogError::validation("Invalid signature length"))?;

    let verifying_key = VerifyingKey::from_bytes(&public_array)
        .map_err(|e| BearDogError::validation(&format!("Invalid public key: {}", e)))?;

    let sig = Signature::from_bytes(&sig_array);

    Ok(verifying_key.verify(message, &sig).is_ok())
}

#[allow(dead_code)]
fn generate_aes_256_key() -> Result<Vec<u8>, BearDogError> {
    use rand::RngCore;
    let mut key = vec![0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    Ok(key)
}

#[allow(dead_code)]
fn generate_aes_nonce() -> Result<Vec<u8>, BearDogError> {
    use rand::RngCore;
    let mut nonce = vec![0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce);
    Ok(nonce)
}

#[allow(dead_code)]
fn encrypt_aes_256_gcm(key: &[u8], nonce: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, BearDogError> {
    use aes_gcm::{aead::Aead, Aes256Gcm, Key, KeyInit, Nonce};

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Nonce::from_slice(nonce);

    cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| BearDogError::security(format!("AES-GCM encryption failed: {}", e)))
}

#[allow(dead_code)]
fn decrypt_aes_256_gcm(key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError> {
    use aes_gcm::{aead::Aead, Aes256Gcm, Key, KeyInit, Nonce};

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Nonce::from_slice(nonce);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| BearDogError::security(format!("AES-GCM decryption failed: {}", e)))
}

#[allow(dead_code)]
fn generate_chacha20_key() -> Result<Vec<u8>, BearDogError> {
    use rand::RngCore;
    let mut key = vec![0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    Ok(key)
}

#[allow(dead_code)]
fn generate_chacha20_nonce() -> Result<Vec<u8>, BearDogError> {
    use rand::RngCore;
    let mut nonce = vec![0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce);
    Ok(nonce)
}

#[allow(dead_code)]
fn encrypt_chacha20_poly1305(key: &[u8], nonce: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, BearDogError> {
    use chacha20poly1305::{aead::Aead, ChaCha20Poly1305, Key, KeyInit, Nonce};

    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(nonce);

    cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| BearDogError::security(format!("ChaCha20-Poly1305 encryption failed: {}", e)))
}

#[allow(dead_code)]
fn decrypt_chacha20_poly1305(
    key: &[u8],
    nonce: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>, BearDogError> {
    use chacha20poly1305::{aead::Aead, ChaCha20Poly1305, Key, KeyInit, Nonce};

    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(nonce);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| BearDogError::security(format!("ChaCha20-Poly1305 decryption failed: {}", e)))
}

#[allow(dead_code)]
fn encrypt_chacha20_poly1305_with_aad(
    key: &[u8],
    nonce: &[u8],
    plaintext: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>, BearDogError> {
    use chacha20poly1305::{aead::Aead, aead::Payload, ChaCha20Poly1305, Key, KeyInit, Nonce};

    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(nonce);
    let payload = Payload {
        msg: plaintext,
        aad,
    };

    cipher.encrypt(nonce, payload).map_err(|e| {
        BearDogError::security(format!("ChaCha20-Poly1305 AAD encryption failed: {}", e))
    })
}

#[allow(dead_code)]
fn decrypt_chacha20_poly1305_with_aad(
    key: &[u8],
    nonce: &[u8],
    ciphertext: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>, BearDogError> {
    use chacha20poly1305::{aead::Aead, aead::Payload, ChaCha20Poly1305, Key, KeyInit, Nonce};

    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(nonce);
    let payload = Payload {
        msg: ciphertext,
        aad,
    };

    cipher.decrypt(nonce, payload).map_err(|e| {
        BearDogError::security(format!("ChaCha20-Poly1305 AAD decryption failed: {}", e))
    })
}

#[allow(dead_code)]
fn blake3_hash(data: &[u8]) -> Result<Vec<u8>, BearDogError> {
    let hash = blake3::hash(data);
    Ok(hash.as_bytes().to_vec())
}

#[allow(dead_code)]
fn blake3_keyed_hash(key: &[u8], data: &[u8]) -> Result<Vec<u8>, BearDogError> {
    if key.len() != 32 {
        return Err(BearDogError::validation("BLAKE3 key must be 32 bytes"));
    }
    let key_array: [u8; 32] = key
        .try_into()
        .map_err(|_| BearDogError::validation("Invalid key length"))?;
    let hash = blake3::keyed_hash(&key_array, data);
    Ok(hash.as_bytes().to_vec())
}

#[allow(dead_code)]
fn derive_key_pbkdf2(password: &[u8], salt: &[u8], iterations: u32) -> Result<Vec<u8>, BearDogError> {
    use pbkdf2::pbkdf2_hmac_array;
    use sha2::Sha256;

    let key = pbkdf2_hmac_array::<Sha256, 32>(password, salt, iterations);
    Ok(key.to_vec())
}

#[allow(dead_code)]
fn derive_key_argon2(password: &[u8], salt: &[u8]) -> Result<Vec<u8>, BearDogError> {
    use argon2::password_hash::SaltString;
    use argon2::{Argon2, PasswordHasher};

    // Convert salt to base64 string format required by argon2
    let salt_str = SaltString::encode_b64(salt)
        .map_err(|e| BearDogError::validation(&format!("Invalid salt: {}", e)))?;

    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password, &salt_str)
        .map_err(|e| BearDogError::security(format!("Argon2 failed: {}", e)))?;

    // Extract the raw hash bytes
    let hash_bytes = hash
        .hash
        .ok_or_else(|| BearDogError::security("No hash produced".to_string()))?;

    Ok(hash_bytes.as_bytes().to_vec())
}

#[allow(dead_code)]
struct Ed25519Keypair {
    public: Vec<u8>,
    secret: Vec<u8>,
}
