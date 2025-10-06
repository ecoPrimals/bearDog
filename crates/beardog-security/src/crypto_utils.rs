// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use beardog_errors::BearDogError;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use hex;
use hmac::{Hmac, Mac};
use rand::{distributions::Alphanumeric, thread_rng, Rng, RngCore};
use ring::pbkdf2;
use sha2::{Digest, Sha256};
use std::num::NonZeroU32;
use subtle::ConstantTimeEq;
use zeroize::Zeroize;

type HmacSha256 = Hmac<Sha256>;

pub struct BearDogCrypto;

impl BearDogCrypto {
    /// Generate Ed25519 Keypair operation.
    pub fn generate_ed25519_keypair() -> (Vec<u8>, Vec<u8>) {
        let mut csprng = OsRng;
        let mut secret_bytes = [0u8; 32];
        csprng.fill_bytes(&mut secret_bytes);
        let signing_key = SigningKey::from_bytes(&secret_bytes);
        let public_key = signing_key.verifying_key();
        (secret_bytes.to_vec(), public_key.as_bytes().to_vec())
    }

    /// Sign data with Ed25519 private key
    pub fn sign_ed25519(private_key: &[u8], message: &[u8]) -> Result<Vec<u8>, BearDogError> {
        if private_key.len() != 32 {
            return Err(BearDogError::invalid_input(
                "Ed25519 private key must be 32 bytes",
            ));
        }

        let key_bytes: [u8; 32] = private_key
            .try_into()
            .map_err(|_| BearDogError::invalid_input("Invalid private key format"))?;

        let signing_key = SigningKey::from_bytes(&key_bytes);
        let signature: Signature = signing_key.sign(message);
        Ok(signature.to_bytes().to_vec())
    }

    /// Verify Ed25519 signature
    pub fn verify_ed25519(
        public_key: &[u8],
        message: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
        if public_key.len() != 32 {
            return Err(BearDogError::invalid_input(
                "Ed25519 public key must be 32 bytes",
            ));
        }
        if signature.len() != 64 {
            return Err(BearDogError::invalid_input(
                "Ed25519 signature must be 64 bytes",
            ));
        }

        let public_key_bytes: [u8; 32] = public_key
            .try_into()
            .map_err(|_| BearDogError::invalid_input("Invalid public key format"))?;
        let signature_bytes: [u8; 64] = signature
            .try_into()
            .map_err(|_| BearDogError::invalid_input("Invalid signature format"))?;

        let verifying_key = VerifyingKey::from_bytes(&public_key_bytes)
            .map_err(|e| BearDogError::security(format!("Invalid public key: {e}")))?;
        let sig = Signature::from_bytes(&signature_bytes);

        match verifying_key.verify(message, &sig) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false), // Verification failed - not an error, just invalid signature
        }
    }

    /// Generate Secure Random operation.
    pub fn generate_secure_random(size: usize) -> Vec<u8> {
        let mut bytes = vec![0u8; size];
        thread_rng().fill_bytes(&mut bytes);
        bytes
    }

    /// Generate Secure Nonce operation.
    pub fn generate_secure_nonce(size: usize) -> Vec<u8> {
        Self::generate_secure_random(size)
    }

    /// Derive key using PBKDF2
    pub fn derive_pbkdf2_key(
        password: &[u8],
        salt: &[u8],
        iterations: u32,
        key_length: usize,
    ) -> Result<Vec<u8>, BearDogError> {
        let mut key = vec![0u8; key_length];
        let iterations = NonZeroU32::new(iterations)
            .ok_or_else(|| BearDogError::invalid_input("Iterations must be non-zero"))?;

        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            iterations,
            salt,
            password,
            &mut key,
        );

        Ok(key)
    }

    /// Encrypt data using AES-GCM
    pub fn encrypt_aes_gcm(
        key: &[u8],
        plaintext: &[u8],
        nonce_opt: Option<&[u8]>,
    ) -> Result<(Vec<u8>, Vec<u8>), BearDogError> {
        if key.len() != 32 {
            return Err(BearDogError::invalid_input("AES-256 key must be 32 bytes"));
        }

        let key = Key::<Aes256Gcm>::from_slice(key);
        let cipher = Aes256Gcm::new(key);

        let nonce_bytes = if let Some(nonce) = nonce_opt {
            if nonce.len() != 12 {
                return Err(BearDogError::invalid_input(
                    "AES-GCM nonce must be 12 bytes",
                ));
            }
            nonce.to_vec()
        } else {
            Self::generate_secure_nonce(12)
        };

        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| BearDogError::security(format!("Encryption failed: {e}")))?;

        Ok((ciphertext, nonce_bytes))
    }

    /// Decrypt data using AES-GCM
    pub fn decrypt_aes_gcm(
        key: &[u8],
        ciphertext: &[u8],
        nonce: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        if key.len() != 32 {
            return Err(BearDogError::invalid_input("AES-256 key must be 32 bytes"));
        }
        if nonce.len() != 12 {
            return Err(BearDogError::invalid_input(
                "AES-GCM nonce must be 12 bytes",
            ));
        }

        let key = Key::<Aes256Gcm>::from_slice(key);
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(nonce);

        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| BearDogError::security(format!("Decryption failed: {e}")))?;

        Ok(plaintext)
    }

    /// Hash Password Argon2 operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn hash_password_argon2(password: &str) -> Result<String, BearDogError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| BearDogError::security(format!("Password hashing failed: {e}")))?;

        Ok(password_hash.to_string())
    }

    /// Verify Password Argon2 operation.
    pub fn verify_password_argon2(password: &str, hash: &str) -> Result<bool, BearDogError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| BearDogError::security(format!("Invalid password hash: {e}")))?;

        let argon2 = Argon2::default();

        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Sha256 Hash operation.
    pub fn sha256_hash(input: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input);
        hex::encode(hasher.finalize())
    }

    /// Compute HMAC-SHA256
    ///
    /// Generates an HMAC (Hash-based Message Authentication Code) using SHA256.
    /// This is used for message authentication and integrity verification.
    ///
    /// # Arguments
    /// * `key` - The secret key for HMAC computation
    /// * `data` - The data to authenticate
    ///
    /// # Returns
    /// The HMAC tag as a byte vector
    pub fn hmac_sha256(key: &[u8], data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        let mut mac = HmacSha256::new_from_slice(key)
            .map_err(|e| BearDogError::security(format!("HMAC key initialization failed: {e}")))?;
        mac.update(data);
        Ok(mac.finalize().into_bytes().to_vec())
    }

    /// Verify HMAC-SHA256
    ///
    /// Verifies an HMAC tag in constant time to prevent timing attacks.
    ///
    /// # Arguments
    /// * `key` - The secret key used for HMAC
    /// * `data` - The data to verify
    /// * `expected_tag` - The expected HMAC tag
    ///
    /// # Returns
    /// `true` if the HMAC matches, `false` otherwise
    pub fn verify_hmac_sha256(key: &[u8], data: &[u8], expected_tag: &[u8]) -> Result<bool, BearDogError> {
        let computed_tag = Self::hmac_sha256(key, data)?;
        Ok(Self::constant_time_compare(&computed_tag, expected_tag))
    }

    /// Constant-time comparison
    ///
    /// Compares two byte slices in constant time to prevent timing attacks.
    /// This is critical for security-sensitive comparisons like HMAC verification.
    ///
    /// # Arguments
    /// * `a` - First byte slice
    /// * `b` - Second byte slice
    ///
    /// # Returns
    /// `true` if the slices are equal, `false` otherwise
    pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        a.ct_eq(b).into()
    }

    /// Generate secure random password
    ///
    /// Generates a cryptographically secure random password using alphanumeric characters
    /// plus special characters for enhanced security.
    ///
    /// # Arguments
    /// * `length` - The desired password length (minimum 8)
    ///
    /// # Returns
    /// A secure random password string
    pub fn generate_password(length: usize) -> Result<String, BearDogError> {
        if length < 8 {
            return Err(BearDogError::Business {
                message: "Password length must be at least 8 characters".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }

        let mut rng = thread_rng();
        let password: String = (0..length)
            .map(|_| {
                let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=[]{}|;:,.<>?";
                charset[rng.gen_range(0..charset.len())] as char
            })
            .collect();

        Ok(password)
    }

    /// Generate secure API key
    ///
    /// Generates a cryptographically secure API key with an optional prefix.
    /// The key is base64-encoded for URL-safe transmission.
    ///
    /// # Arguments
    /// * `prefix` - Optional prefix for the API key (e.g., "sk_", "pk_")
    ///
    /// # Returns
    /// A secure API key string in the format "prefix_base64(random_bytes)"
    pub fn generate_api_key(prefix: &str) -> Result<String, BearDogError> {
        let random_bytes = Self::generate_secure_random(32);
        let base64_key = base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, &random_bytes);
        
        if prefix.is_empty() {
            Ok(base64_key)
        } else {
            Ok(format!("{}_{}", prefix, base64_key))
        }
    }

    /// Zero memory securely
    ///
    /// Securely zeros out a mutable byte slice to prevent sensitive data from
    /// remaining in memory. This uses the zeroize crate for compiler-guaranteed
    /// memory clearing.
    ///
    /// # Arguments
    /// * `buffer` - The mutable buffer to zero
    pub fn zero_memory(buffer: &mut [u8]) {
        buffer.zeroize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    fn test_ed25519_signature_verification() -> Result<(), BearDogError> {
        let (private_key, public_key) = BearDogCrypto::generate_ed25519_keypair();
        let message = b"test message for signing";

        let signature = BearDogCrypto::sign_ed25519(&private_key, message)?;
        let is_valid = BearDogCrypto::verify_ed25519(&public_key, message, &signature)?;

        assert!(is_valid, "Valid signature should verify");

        let wrong_message = b"wrong message";
        let is_invalid = BearDogCrypto::verify_ed25519(&public_key, wrong_message, &signature)?;
        assert!(!is_invalid, "Invalid signature should not verify");

        Ok(())
    }

    #[tokio::test]
    fn test_secure_nonce_generation() -> Result<(), BearDogError> {
        let nonce1 = BearDogCrypto::generate_secure_nonce(32);
        let nonce2 = BearDogCrypto::generate_secure_nonce(32);

        assert_eq!(nonce1.len(), 32);
        assert_eq!(nonce2.len(), 32);
        assert_ne!(nonce1, nonce2, "Nonces should be different");
        assert_ne!(nonce1, vec![0u8; 32], "Nonce should not be all zeros");

        Ok(())
    }

    #[tokio::test]
    fn test_aes_gcm_encryption() -> Result<(), BearDogError> {
        let key = BearDogCrypto::generate_secure_random(32);
        let plaintext = b"test data for encryption";

        let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&key, plaintext, None)?;
        assert_ne!(ciphertext, plaintext.to_vec());
        assert_eq!(nonce.len(), 12);

        let decrypted = BearDogCrypto::decrypt_aes_gcm(&key, &ciphertext, &nonce)?;
        assert_eq!(decrypted, plaintext.to_vec());

        Ok(())
    }

    #[tokio::test]
    fn test_hmac_sha256() -> Result<(), BearDogError> {
        let key = b"secret_key_for_hmac_testing";
        let data = b"message to authenticate";

        let tag1 = BearDogCrypto::hmac_sha256(key, data)?;
        let tag2 = BearDogCrypto::hmac_sha256(key, data)?;

        // Same key and data should produce same tag
        assert_eq!(tag1, tag2, "HMAC should be deterministic");
        assert_eq!(tag1.len(), 32, "HMAC-SHA256 should produce 32 bytes");

        // Different data should produce different tag
        let different_data = b"different message";
        let tag3 = BearDogCrypto::hmac_sha256(key, different_data)?;
        assert_ne!(tag1, tag3, "Different data should produce different HMAC");

        Ok(())
    }

    #[tokio::test]
    fn test_verify_hmac_sha256() -> Result<(), BearDogError> {
        let key = b"secret_key_for_verification";
        let data = b"data to verify";

        let tag = BearDogCrypto::hmac_sha256(key, data)?;
        
        // Correct tag should verify
        assert!(
            BearDogCrypto::verify_hmac_sha256(key, data, &tag)?,
            "Valid HMAC should verify"
        );

        // Wrong tag should not verify
        let wrong_tag = vec![0u8; 32];
        assert!(
            !BearDogCrypto::verify_hmac_sha256(key, data, &wrong_tag)?,
            "Invalid HMAC should not verify"
        );

        // Wrong key should not verify
        let wrong_key = b"wrong_key";
        assert!(
            !BearDogCrypto::verify_hmac_sha256(wrong_key, data, &tag)?,
            "Wrong key should not verify"
        );

        Ok(())
    }

    #[tokio::test]
    fn test_constant_time_compare() -> Result<(), BearDogError> {
        let data1 = b"sensitive_data_12345";
        let data2 = b"sensitive_data_12345";
        let data3 = b"different_data_67890";
        let data4 = b"short";

        // Equal data should match
        assert!(
            BearDogCrypto::constant_time_compare(data1, data2),
            "Equal data should compare as equal"
        );

        // Different data should not match
        assert!(
            !BearDogCrypto::constant_time_compare(data1, data3),
            "Different data should not compare as equal"
        );

        // Different lengths should not match
        assert!(
            !BearDogCrypto::constant_time_compare(data1, data4),
            "Different length data should not compare as equal"
        );

        Ok(())
    }

    #[tokio::test]
    fn test_generate_password() -> Result<(), BearDogError> {
        // Test valid password generation
        let password1 = BearDogCrypto::generate_password(16)?;
        let password2 = BearDogCrypto::generate_password(16)?;

        assert_eq!(password1.len(), 16, "Password should be correct length");
        assert_eq!(password2.len(), 16, "Password should be correct length");
        assert_ne!(password1, password2, "Passwords should be unique");

        // Test minimum length validation
        let result = BearDogCrypto::generate_password(7);
        assert!(
            result.is_err(),
            "Password generation should fail for length < 8"
        );

        // Test longer password
        let long_password = BearDogCrypto::generate_password(64)?;
        assert_eq!(long_password.len(), 64, "Long password should be correct length");

        // Verify password contains various character types
        assert!(
            password1.chars().any(|c| c.is_ascii_uppercase()),
            "Password should contain uppercase letters"
        );

        Ok(())
    }

    #[tokio::test]
    fn test_generate_api_key() -> Result<(), BearDogError> {
        // Test API key with prefix
        let api_key1 = BearDogCrypto::generate_api_key("sk")?;
        assert!(api_key1.starts_with("sk_"), "API key should have prefix");

        // Test API key without prefix
        let api_key2 = BearDogCrypto::generate_api_key("")?;
        assert!(!api_key2.contains('_'), "API key without prefix should not have underscore");

        // Test uniqueness
        let api_key3 = BearDogCrypto::generate_api_key("pk")?;
        assert_ne!(api_key1, api_key3, "API keys should be unique");

        // Test different prefixes
        let api_key4 = BearDogCrypto::generate_api_key("test")?;
        assert!(api_key4.starts_with("test_"), "API key should have custom prefix");

        Ok(())
    }

    #[tokio::test]
    fn test_zero_memory() -> Result<(), BearDogError> {
        let mut sensitive_data = b"super_secret_password_12345".to_vec();
        
        // Verify data exists
        assert_ne!(sensitive_data, vec![0u8; sensitive_data.len()]);

        // Zero the memory
        BearDogCrypto::zero_memory(&mut sensitive_data);

        // Verify data is zeroed
        assert_eq!(
            sensitive_data,
            vec![0u8; 27],
            "Memory should be completely zeroed"
        );

        Ok(())
    }
}
