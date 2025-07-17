//! Cryptographic utilities for BearDog
//!
//! This module provides secure cryptographic operations including:
//! - Ed25519 signature verification
//! - Secure random nonce generation
//! - Key derivation functions
//! - Cryptographic hashing

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use ed25519_dalek::{SecretKey, Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::{thread_rng, RngCore};
use sha2::{Digest, Sha256};
use std::convert::TryInto;

use beardog_errors::{BearDogError, BearDogResult};

/// Cryptographic utilities for BearDog security operations
pub struct BearDogCrypto;

impl BearDogCrypto {
    /// Verify Ed25519 signature
    ///
    /// # Arguments
    /// * `public_key` - The Ed25519 public key bytes (32 bytes)
    /// * `message` - The message that was signed
    /// * `signature` - The signature bytes (64 bytes)
    ///
    /// # Returns
    /// * `Ok(true)` if signature is valid
    /// * `Ok(false)` if signature is invalid
    /// * `Err(BearDogError)` if there's a cryptographic error
    pub fn verify_ed25519_signature(
        public_key: &[u8],
        message: &[u8],
        signature: &[u8],
    ) -> BearDogResult<bool> {
        // Validate input lengths
        if public_key.len() != 32 {
            return Err(BearDogError::Crypto {
                message: format!(
                    "Invalid public key length: expected 32 bytes, got {}",
                    public_key.len()
                ),
            });
        }

        if signature.len() != 64 {
            return Err(BearDogError::Crypto {
                message: format!(
                    "Invalid signature length: expected 64 bytes, got {}",
                    signature.len()
                ),
            });
        }

        // Convert to fixed-size arrays
        let public_key_array: [u8; 32] =
            public_key.try_into().map_err(|_| BearDogError::Crypto {
                message: "Failed to convert public key to array".to_string(),
            })?;

        let signature_array: [u8; 64] = signature.try_into().map_err(|_| BearDogError::Crypto {
            message: "Failed to convert signature to array".to_string(),
        })?;

        // Create verifying key
        let verifying_key =
            VerifyingKey::from_bytes(&public_key_array).map_err(|e| BearDogError::Crypto {
                message: format!("Invalid public key: {e}"),
            })?;

        // Create signature
        let signature_obj = Signature::from_bytes(&signature_array);

        // Verify signature
        match verifying_key.verify(message, &signature_obj) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false), // Invalid signature, not an error
        }
    }

    /// Generate Ed25519 signature
    ///
    /// # Arguments
    /// * `private_key` - The Ed25519 private key bytes (32 bytes)
    /// * `message` - The message to sign
    ///
    /// # Returns
    /// * `Ok(signature)` - The 64-byte signature
    /// * `Err(BearDogError)` if there's a cryptographic error
    pub fn sign_ed25519(private_key: &[u8], message: &[u8]) -> BearDogResult<Vec<u8>> {
        if private_key.len() != 32 {
            return Err(BearDogError::Crypto {
                message: format!(
                    "Invalid private key length: expected 32 bytes, got {}",
                    private_key.len()
                ),
            });
        }

        let private_key_array: [u8; 32] =
            private_key.try_into().map_err(|_| BearDogError::Crypto {
                message: "Failed to convert private key to array".to_string(),
            })?;

        let signing_key = SigningKey::from_bytes(&private_key_array);
        let signature = signing_key.sign(message);

        Ok(signature.to_bytes().to_vec())
    }

    /// Generate a new Ed25519 keypair
    ///
    /// # Returns
    /// * `Ok((private_key, public_key))` - Both as 32-byte vectors
    pub fn generate_ed25519_keypair() -> BearDogResult<(Vec<u8>, Vec<u8>)> {
        let mut secret_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut secret_bytes);
        let secret_key = SecretKey::from(secret_bytes);
        let signing_key = SigningKey::from_bytes(&secret_key);
        let verifying_key = signing_key.verifying_key();

        Ok((
            signing_key.to_bytes().to_vec(),
            verifying_key.to_bytes().to_vec(),
        ))
    }

    /// Generate secure random nonce
    ///
    /// # Arguments
    /// * `size` - Size of nonce in bytes (typically 12 for AES-GCM, 24 for ChaCha20)
    ///
    /// # Returns
    /// * `Ok(nonce)` - Cryptographically secure random nonce
    pub fn generate_secure_nonce(size: usize) -> BearDogResult<Vec<u8>> {
        if size == 0 || size > 64 {
            return Err(BearDogError::InvalidInput {
                message: format!("Invalid nonce size: {size} (must be 1-64 bytes)"),
            });
        }

        let mut nonce = vec![0u8; size];
        thread_rng().fill_bytes(&mut nonce);
        Ok(nonce)
    }

    /// Generate secure random bytes
    ///
    /// # Arguments
    /// * `size` - Size of random bytes to generate
    ///
    /// # Returns
    /// * `Ok(bytes)` - Cryptographically secure random bytes
    pub fn secure_random_bytes(size: usize) -> BearDogResult<Vec<u8>> {
        Self::generate_secure_nonce(size)
    }

    /// Generate secure random bytes
    ///
    /// # Arguments
    /// * `size` - Number of random bytes to generate
    ///
    /// # Returns
    /// * `Ok(bytes)` - Cryptographically secure random bytes
    pub fn generate_secure_random(size: usize) -> BearDogResult<Vec<u8>> {
        if size == 0 || size > 1024 {
            return Err(BearDogError::InvalidInput {
                message: format!("Invalid random size: {size} (must be 1-1024 bytes)"),
            });
        }

        let mut bytes = vec![0u8; size];
        thread_rng().fill_bytes(&mut bytes);
        Ok(bytes)
    }

    /// Derive key using PBKDF2 with SHA256
    ///
    /// # Arguments
    /// * `password` - The password/passphrase
    /// * `salt` - The salt bytes
    /// * `iterations` - Number of PBKDF2 iterations (minimum 10000)
    /// * `key_length` - Desired key length in bytes
    ///
    /// # Returns
    /// * `Ok(key)` - Derived key bytes
    pub fn derive_key_pbkdf2(
        password: &[u8],
        salt: &[u8],
        iterations: u32,
        key_length: usize,
    ) -> BearDogResult<Vec<u8>> {
        if iterations < 10000 {
            return Err(BearDogError::Crypto {
                message: format!("Insufficient PBKDF2 iterations: {iterations} (minimum 10000)"),
            });
        }

        if key_length == 0 || key_length > 128 {
            return Err(BearDogError::InvalidInput {
                message: format!("Invalid key length: {key_length} (must be 1-128 bytes)"),
            });
        }

        let mut key = vec![0u8; key_length];
        pbkdf2::pbkdf2_hmac::<sha2::Sha256>(password, salt, iterations, &mut key);
        Ok(key)
    }

    /// Hash password using Argon2id
    ///
    /// # Arguments
    /// * `password` - The password to hash
    ///
    /// # Returns
    /// * `Ok(hash)` - Argon2id password hash string
    pub fn hash_password_argon2(password: &str) -> BearDogResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| BearDogError::Crypto {
                message: format!("Password hashing failed: {e}"),
            })?;

        Ok(password_hash.to_string())
    }

    /// Verify password against Argon2id hash
    ///
    /// # Arguments
    /// * `password` - The password to verify
    /// * `hash` - The Argon2id hash string
    ///
    /// # Returns
    /// * `Ok(true)` if password matches hash
    /// * `Ok(false)` if password doesn't match
    pub fn verify_password_argon2(password: &str, hash: &str) -> BearDogResult<bool> {
        let parsed_hash = PasswordHash::new(hash).map_err(|e| BearDogError::Crypto {
            message: format!("Invalid password hash format: {e}"),
        })?;

        let argon2 = Argon2::default();
        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false), // Invalid password, not an error
        }
    }

    /// Secure hash using SHA256
    ///
    /// # Arguments
    /// * `data` - Data to hash
    ///
    /// # Returns
    /// * `Ok(hash)` - SHA256 hash bytes (32 bytes)
    pub fn sha256_hash(data: &[u8]) -> BearDogResult<Vec<u8>> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        Ok(hasher.finalize().to_vec())
    }

    /// Secure hash with salt using SHA256
    ///
    /// # Arguments
    /// * `data` - Data to hash
    /// * `salt` - Salt bytes
    ///
    /// # Returns
    /// * `Ok(hash)` - SHA256 hash bytes (32 bytes)
    pub fn sha256_hash_with_salt(data: &[u8], salt: &[u8]) -> BearDogResult<Vec<u8>> {
        let mut hasher = Sha256::new();
        hasher.update(salt);
        hasher.update(data);
        Ok(hasher.finalize().to_vec())
    }

    /// Generate cryptographically secure UUID v4
    ///
    /// # Returns
    /// * `Ok(uuid)` - UUID string
    pub fn generate_secure_uuid() -> BearDogResult<String> {
        use uuid::Uuid;
        Ok(Uuid::new_v4().to_string())
    }

    /// Constant-time comparison of byte arrays
    ///
    /// # Arguments
    /// * `a` - First byte array
    /// * `b` - Second byte array
    ///
    /// # Returns
    /// * `Ok(true)` if arrays are equal
    /// * `Ok(false)` if arrays are not equal or different lengths
    pub fn constant_time_compare(a: &[u8], b: &[u8]) -> BearDogResult<bool> {
        if a.len() != b.len() {
            return Ok(false);
        }

        use subtle::ConstantTimeEq;
        Ok(a.ct_eq(b).into())
    }

    /// Derive deterministic key from seed
    ///
    /// # Arguments
    /// * `seed` - Master seed
    /// * `context` - Context string for key derivation
    /// * `key_length` - Desired key length
    ///
    /// # Returns
    /// * `Ok(key)` - Derived key bytes
    pub fn derive_key_from_seed(
        seed: &[u8],
        context: &str,
        key_length: usize,
    ) -> BearDogResult<Vec<u8>> {
        if seed.len() < 16 {
            return Err(BearDogError::Crypto {
                message: "Seed too short (minimum 16 bytes)".to_string(),
            });
        }

        if key_length == 0 || key_length > 128 {
            return Err(BearDogError::InvalidInput {
                message: format!("Invalid key length: {key_length} (must be 1-128 bytes)"),
            });
        }

        // Use HKDF-like derivation
        let mut hasher = Sha256::new();
        hasher.update(seed);
        hasher.update(context.as_bytes());
        let initial_hash = hasher.finalize();

        // Expand to desired length
        let mut key = Vec::new();
        let mut counter = 0u32;

        while key.len() < key_length {
            let mut expand_hasher = Sha256::new();
            expand_hasher.update(initial_hash);
            expand_hasher.update(counter.to_le_bytes());
            let chunk = expand_hasher.finalize();

            let needed = std::cmp::min(key_length - key.len(), chunk.len());
            key.extend_from_slice(&chunk[..needed]);
            counter += 1;
        }

        Ok(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ed25519_signature_verification() {
        let message = b"Hello, BearDog!";

        // Generate keypair
        let (private_key, public_key) = BearDogCrypto::generate_ed25519_keypair()
            .expect("Ed25519 keypair generation should never fail in tests");

        // Sign message
        let signature = BearDogCrypto::sign_ed25519(&private_key, message)
            .expect("Ed25519 signing should never fail with valid keypair");

        // Verify signature
        let is_valid = BearDogCrypto::verify_ed25519_signature(&public_key, message, &signature)
            .expect("Ed25519 signature verification should never fail with valid inputs");
        assert!(is_valid);

        // Test with wrong message
        let wrong_message = b"Wrong message";
        let is_invalid =
            BearDogCrypto::verify_ed25519_signature(&public_key, wrong_message, &signature).expect(
                "Ed25519 signature verification should handle invalid signatures gracefully",
            );
        assert!(!is_invalid);
    }

    #[test]
    fn test_secure_nonce_generation() {
        let nonce1 = BearDogCrypto::generate_secure_nonce(12)
            .expect("Secure nonce generation should never fail with valid size");
        let nonce2 = BearDogCrypto::generate_secure_nonce(12)
            .expect("Secure nonce generation should never fail with valid size");

        assert_eq!(nonce1.len(), 12);
        assert_eq!(nonce2.len(), 12);
        assert_ne!(nonce1, nonce2); // Should be different
        assert_ne!(nonce1, vec![0u8; 12]); // Should not be all zeros
    }

    #[test]
    fn test_password_hashing() {
        let password = "secure_password_123";
        let hash = BearDogCrypto::hash_password_argon2(password)
            .expect("Argon2 password hashing should never fail with valid input");

        // Verify correct password
        let is_valid = BearDogCrypto::verify_password_argon2(password, &hash)
            .expect("Argon2 password verification should never fail with valid hash");
        assert!(is_valid);

        // Verify wrong password
        let is_invalid = BearDogCrypto::verify_password_argon2("wrong_password", &hash)
            .expect("Argon2 password verification should handle wrong passwords gracefully");
        assert!(!is_invalid);
    }

    #[test]
    fn test_key_derivation() {
        let seed = b"master_seed_for_testing_purposes";
        let context = "genetic_spawning_key";

        let key1 = BearDogCrypto::derive_key_from_seed(seed, context, 32)
            .expect("Key derivation should never fail with valid seed and context");
        let key2 = BearDogCrypto::derive_key_from_seed(seed, context, 32)
            .expect("Key derivation should never fail with valid seed and context");
        let key3 = BearDogCrypto::derive_key_from_seed(seed, "different_context", 32)
            .expect("Key derivation should never fail with valid seed and context");

        assert_eq!(key1.len(), 32);
        assert_eq!(key1, key2); // Same seed + context = same key
        assert_ne!(key1, key3); // Different context = different key
    }

    #[test]
    fn test_constant_time_compare() {
        let data1 = b"secret_data";
        let data2 = b"secret_data";
        let data3 = b"different_secret";

        assert!(BearDogCrypto::constant_time_compare(data1, data2)
            .expect("Constant time comparison should never fail"));
        assert!(!BearDogCrypto::constant_time_compare(data1, data3)
            .expect("Constant time comparison should never fail"));
    }

    #[test]
    fn test_input_validation() {
        // Test invalid public key length
        let result = BearDogCrypto::verify_ed25519_signature(&[0u8; 31], b"message", &[0u8; 64]);
        assert!(result.is_err());

        // Test invalid signature length
        let result = BearDogCrypto::verify_ed25519_signature(&[0u8; 32], b"message", &[0u8; 63]);
        assert!(result.is_err());

        // Test invalid nonce size
        let result = BearDogCrypto::generate_secure_nonce(0);
        assert!(result.is_err());

        let result = BearDogCrypto::generate_secure_nonce(65);
        assert!(result.is_err());
    }
}
