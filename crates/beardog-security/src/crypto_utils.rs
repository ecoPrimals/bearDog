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
use rand::{thread_rng, RngCore};
use ring::pbkdf2;
use sha2::{Digest, Sha256};
use std::num::NonZeroU32;

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
}
