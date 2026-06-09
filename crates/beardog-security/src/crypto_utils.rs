// SPDX-License-Identifier: AGPL-3.0-or-later

//! Cryptographic utility functions for the BearDog security system
//!
//! This module provides core cryptographic operations including:
//! - Ed25519 key generation and signing
//! - HMAC operations
//! - Password hashing and verification
//! - AES-GCM encryption
//!
//! All operations use industry-standard, well-audited cryptographic libraries.

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

// ecoPrimals: Migration plan - aes-gcm 0.10 uses deprecated generic-array.
// When aes-gcm 0.11 is stable, upgrade and remove this allow.
#![allow(
    deprecated,
    reason = "aes-gcm 0.10 uses deprecated generic-array; upgrade to 0.11 when stable"
)]

use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, KeyInit, OsRng},
};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use beardog_errors::BearDogError;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use hex;
use hmac::{Hmac, Mac};
use pbkdf2;
use rand::{Rng, RngCore, rng};
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;
use zeroize::Zeroize;

type HmacSha256 = Hmac<Sha256>;

/// Core cryptographic operations provider for `BearDog`
///
/// Provides methods for key generation, signing, verification,
/// and other cryptographic primitives.
pub struct BearDogCrypto;

impl BearDogCrypto {
    /// Generate Ed25519 Keypair operation.
    pub fn generate_ed25519_keypair() -> (Vec<u8>, Vec<u8>) {
        let mut csprng = OsRng;
        let mut secret_bytes = [0u8; 32];
        aes_gcm::aead::rand_core::RngCore::fill_bytes(&mut csprng, &mut secret_bytes);
        let signing_key = SigningKey::from_bytes(&secret_bytes);
        let public_key = signing_key.verifying_key();
        (secret_bytes.to_vec(), public_key.as_bytes().to_vec())
    }

    /// Sign data with Ed25519 private key
    ///
    /// # Errors
    ///
    /// Returns an error if the private key is not 32 bytes or malformed.
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
    ///
    /// # Errors
    ///
    /// Returns an error if the public key or signature length is invalid, or the public key bytes
    /// are not a valid Ed25519 key.
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
        rng().fill_bytes(&mut bytes);
        bytes
    }

    /// Generate Secure Nonce operation.
    pub fn generate_secure_nonce(size: usize) -> Vec<u8> {
        Self::generate_secure_random(size)
    }

    /// Derive key using PBKDF2
    ///
    /// # Errors
    ///
    /// Returns an error if `iterations` is zero.
    pub fn derive_pbkdf2_key(
        password: &[u8],
        salt: &[u8],
        iterations: u32,
        key_length: usize,
    ) -> Result<Vec<u8>, BearDogError> {
        if iterations == 0 {
            return Err(BearDogError::invalid_input("Iterations must be non-zero"));
        }

        let mut key = vec![0u8; key_length];
        // 100% Pure Rust PBKDF2-HMAC-SHA256
        pbkdf2::pbkdf2_hmac::<Sha256>(password, salt, iterations, &mut key);

        Ok(key)
    }

    /// Encrypt data using AES-GCM
    ///
    /// # Errors
    ///
    /// Returns an error if the key or nonce length is invalid, or encryption fails.
    pub fn encrypt_aes_gcm(
        key: &[u8],
        plaintext: &[u8],
        nonce_opt: Option<&[u8]>,
    ) -> Result<(Vec<u8>, Vec<u8>), BearDogError> {
        if key.len() != 32 {
            return Err(BearDogError::invalid_input("AES-256 key must be 32 bytes"));
        }

        // Convert slice to Key
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));

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

        // Convert Vec to array for Nonce
        let mut nonce_array = [0u8; 12];
        nonce_array.copy_from_slice(&nonce_bytes);
        let nonce = Nonce::from(nonce_array);

        let ciphertext = cipher
            .encrypt(&nonce, plaintext)
            .map_err(|e| BearDogError::security(format!("Encryption failed: {e}")))?;

        Ok((ciphertext, nonce_bytes))
    }

    /// Decrypt data using AES-GCM
    ///
    /// # Errors
    ///
    /// Returns an error if the key or nonce length is invalid, authentication fails, or decryption fails.
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

        // Convert slice to Key
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
        // Convert slice to array for Nonce
        let mut nonce_array = [0u8; 12];
        nonce_array.copy_from_slice(nonce);
        let nonce = Nonce::from(nonce_array);

        let plaintext = cipher
            .decrypt(&nonce, ciphertext)
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
    ///
    /// # Errors
    ///
    /// Returns an error if the stored hash string is not a valid Argon2 password hash.
    pub fn verify_password_argon2(password: &str, hash: &str) -> Result<bool, BearDogError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| BearDogError::security(format!("Invalid password hash: {e}")))?;

        let argon2 = Argon2::default();

        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// SHA-256 hash returning raw bytes.
    pub fn sha256_hash_bytes(input: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(input);
        hasher.finalize().to_vec()
    }

    /// SHA-256 hash returning a hex-encoded string.
    pub fn sha256_hash(input: &[u8]) -> String {
        hex::encode(Self::sha256_hash_bytes(input))
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
    ///
    /// # Errors
    ///
    /// Returns an error if the HMAC key length is invalid for the MAC implementation.
    pub fn hmac_sha256(key: &[u8], data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        let mut mac = <HmacSha256 as hmac::Mac>::new_from_slice(key)
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
    ///
    /// # Errors
    ///
    /// Returns an error if HMAC computation fails (e.g. invalid key length).
    pub fn verify_hmac_sha256(
        key: &[u8],
        data: &[u8],
        expected_tag: &[u8],
    ) -> Result<bool, BearDogError> {
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
    ///
    /// # Errors
    ///
    /// Returns an error if `length` is less than 8.
    pub fn generate_password(length: usize) -> Result<String, BearDogError> {
        if length < 8 {
            return Err(BearDogError::Business {
                message: "Password length must be at least 8 characters".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }

        let mut rng = rng();
        let password: String = (0..length)
            .map(|_| {
                let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=[]{}|;:,.<>?";
                charset[rng.random_range(0..charset.len())] as char
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
    /// A secure API key string in the format "`prefix_base64(random_bytes)`"
    ///
    /// # Errors
    ///
    /// Currently infallible; the `Result` is reserved for future validation.
    pub fn generate_api_key(prefix: &str) -> Result<String, BearDogError> {
        let random_bytes = Self::generate_secure_random(32);
        let base64_key = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            &random_bytes,
        );

        if prefix.is_empty() {
            Ok(base64_key)
        } else {
            Ok(format!("{prefix}_{base64_key}"))
        }
    }

    /// Encrypt data using ChaCha20-Poly1305
    ///
    /// # Errors
    ///
    /// Returns an error if the key length is invalid or encryption fails.
    pub fn encrypt_chacha20_poly1305(
        key: &[u8],
        plaintext: &[u8],
        nonce_opt: Option<&[u8]>,
    ) -> Result<(Vec<u8>, Vec<u8>), BearDogError> {
        use chacha20poly1305::{ChaCha20Poly1305, KeyInit, Nonce, aead::Aead};

        if key.len() != 32 {
            return Err(BearDogError::invalid_input(
                "ChaCha20-Poly1305 key must be 32 bytes",
            ));
        }

        let cipher = ChaCha20Poly1305::new_from_slice(key)
            .map_err(|e| BearDogError::security(format!("Invalid ChaCha20 key: {e}")))?;

        let nonce_bytes = if let Some(nonce) = nonce_opt {
            if nonce.len() != 12 {
                return Err(BearDogError::invalid_input(
                    "ChaCha20-Poly1305 nonce must be 12 bytes",
                ));
            }
            nonce.to_vec()
        } else {
            Self::generate_secure_nonce(12)
        };

        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher.encrypt(nonce, plaintext).map_err(|e| {
            BearDogError::security(format!("ChaCha20-Poly1305 encryption failed: {e}"))
        })?;

        Ok((ciphertext, nonce_bytes))
    }

    /// Decrypt data using ChaCha20-Poly1305
    ///
    /// # Errors
    ///
    /// Returns an error if the key or nonce length is invalid, authentication fails, or
    /// decryption fails.
    pub fn decrypt_chacha20_poly1305(
        key: &[u8],
        ciphertext: &[u8],
        nonce: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        use chacha20poly1305::{ChaCha20Poly1305, KeyInit, Nonce, aead::Aead};

        if key.len() != 32 {
            return Err(BearDogError::invalid_input(
                "ChaCha20-Poly1305 key must be 32 bytes",
            ));
        }
        if nonce.len() != 12 {
            return Err(BearDogError::invalid_input(
                "ChaCha20-Poly1305 nonce must be 12 bytes",
            ));
        }

        let cipher = ChaCha20Poly1305::new_from_slice(key)
            .map_err(|e| BearDogError::security(format!("Invalid ChaCha20 key: {e}")))?;
        let nonce = Nonce::from_slice(nonce);

        cipher.decrypt(nonce, ciphertext).map_err(|e| {
            BearDogError::security(format!("ChaCha20-Poly1305 decryption failed: {e}"))
        })
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
    // Tests live in `tests/crypto_primitives_tests.rs`.
}
