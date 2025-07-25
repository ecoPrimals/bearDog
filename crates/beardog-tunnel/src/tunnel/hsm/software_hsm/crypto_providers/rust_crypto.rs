//! # Rust Crypto Provider
//!
//! This module provides the Rust-based crypto provider implementation using standard Rust crypto crates.
//! It uses `aes-gcm` for AES-256-GCM encryption/decryption and `ed25519-dalek` for Ed25519 signing/verification.

use super::super::super::types::{KeyType, HsmKey, HsmCapabilities};
use crate::tunnel::hsm::types::*;
use async_trait::async_trait;
use beardog_errors::{BearDogError, BearDogResult};
use ed25519_dalek::{Signature, Signer, SigningKey};
use tracing::{debug, info};
use std::collections::HashMap;

/// Rust-based crypto provider using standard Rust crypto crates
impl RustCryptoProvider {
    /// Create a new Rust crypto provider
    pub async fn new() -> BearDogResult<Self> {
        info!("Creating Rust crypto provider");
        Ok(Self)
    }
}

#[async_trait]
impl CryptoProvider for RustCryptoProvider {
    /// Initialize the crypto provider
    async fn initialize(&self) -> BearDogResult<()> {
        info!("Initializing Rust crypto provider");
        Ok(())
    }

    /// Generate secure key material
    async fn generate_key_material(&self, key_type: &KeyType) -> BearDogResult<Vec<u8>> {
        use rand::RngCore;

        match key_type {
            KeyType::Aes256 | KeyType::ChaCha20 => {
                let mut key_material = vec![0u8; 32];
                rand::thread_rng().fill_bytes(&mut key_material);
                debug!(
                    "Generated symmetric key material for {:?}: {} bytes",
                    key_type,
                    key_material.len()
                );
                Ok(key_material)
            }
            KeyType::EccP256 => {
                // For ECC P256, generate a proper Ed25519 keypair for signing
                let mut seed = [0u8; 32];
                rand::thread_rng().fill_bytes(&mut seed);
                let signing_key = SigningKey::from_bytes(&seed);
                debug!("Generated Ed25519 signing key for {:?}: 32 bytes", key_type);
                Ok(signing_key.to_bytes().to_vec())
            }
            KeyType::EccP384 => {
                let mut key_material = vec![0u8; 48];
                rand::thread_rng().fill_bytes(&mut key_material);
                debug!(
                    "Generated key material for {:?}: {} bytes",
                    key_type,
                    key_material.len()
                );
                Ok(key_material)
            }
            KeyType::Rsa { key_size } => {
                let mut key_material = vec![0u8; (key_size / 8) as usize];
                rand::thread_rng().fill_bytes(&mut key_material);
                debug!(
                    "Generated key material for {:?}: {} bytes",
                    key_type,
                    key_material.len()
                );
                Ok(key_material)
            }
            _ => {
                let mut key_material = vec![0u8; 32];
                rand::thread_rng().fill_bytes(&mut key_material);
                debug!(
                    "Generated default key material for {:?}: {} bytes",
                    key_type,
                    key_material.len()
                );
                Ok(key_material)
            }
        }
    }

    /// Encrypt data with key
    async fn encrypt(&self, key_material: &[u8], plaintext: &[u8]) -> BearDogResult<Vec<u8>> {
        use aes_gcm::{aead::Aead, Aes256Gcm, Key, KeyInit, Nonce};
        use rand::RngCore;

        debug!(
            "Encrypting {} bytes with Rust crypto provider (AES-256-GCM)",
            plaintext.len()
        );

        // Ensure key is correct size for AES-256
        if key_material.len() != 32 {
            return Err(BearDogError::Crypto {
                message: format!(
                    "Invalid key size for AES-256: expected 32 bytes, got {}",
                    key_material.len()
                ),
            });
        }

        // Create cipher
        let key = Key::<Aes256Gcm>::from_slice(key_material);
        let cipher = Aes256Gcm::new(key);

        // Generate random nonce
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt
        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| BearDogError::Crypto {
                message: format!("AES-256-GCM encryption failed: {e}"),
            })?;

        // Prepend nonce to ciphertext for easy decryption
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    /// Decrypt data with key
    async fn decrypt(&self, key_material: &[u8], ciphertext: &[u8]) -> BearDogResult<Vec<u8>> {
        use aes_gcm::{aead::Aead, Aes256Gcm, Key, KeyInit, Nonce};

        debug!(
            "Decrypting {} bytes with Rust crypto provider (AES-256-GCM)",
            ciphertext.len()
        );

        // Ensure key is correct size for AES-256
        if key_material.len() != 32 {
            return Err(BearDogError::Crypto {
                message: format!(
                    "Invalid key size for AES-256: expected 32 bytes, got {}",
                    key_material.len()
                ),
            });
        }

        // Extract nonce and ciphertext
        if ciphertext.len() < 12 {
            return Err(BearDogError::Crypto {
                message: "Ciphertext too short - missing nonce".to_string(),
            });
        }

        let nonce = Nonce::from_slice(&ciphertext[..12]);
        let ciphertext_data = &ciphertext[12..];

        // Create cipher
        let key = Key::<Aes256Gcm>::from_slice(key_material);
        let cipher = Aes256Gcm::new(key);

        // Decrypt
        let plaintext =
            cipher
                .decrypt(nonce, ciphertext_data)
                .map_err(|e| BearDogError::Crypto {
                    message: format!("AES-256-GCM decryption failed: {e}"),
                })?;

        Ok(plaintext)
    }

    /// Sign data with key
    async fn sign(&self, key_material: &[u8], data: &[u8]) -> BearDogResult<Vec<u8>> {
        debug!(
            "Signing {} bytes with Rust crypto provider (Ed25519)",
            data.len()
        );

        // Ensure key is correct size for Ed25519
        if key_material.len() != 32 {
            return Err(BearDogError::Crypto {
                message: format!(
                    "Invalid key size for Ed25519: expected 32 bytes, got {}",
                    key_material.len()
                ),
            });
        }

        // Create signing key
        let signing_key =
            SigningKey::from_bytes(key_material.try_into().map_err(|_| BearDogError::Crypto {
                message: "Failed to convert key material to Ed25519 key".to_string(),
            })?);

        // Sign the data
        let signature = signing_key
            .try_sign(data)
            .map_err(|e| BearDogError::Crypto {
                message: format!("Failed to sign data: {e}"),
            })?;

        Ok(signature.to_bytes().to_vec())
    }

    /// Verify signature with key
    async fn verify(
        &self,
        key_material: &[u8],
        data: &[u8],
        signature: &[u8],
    ) -> BearDogResult<bool> {
        debug!(
            "Verifying signature for {} bytes with Rust crypto provider (Ed25519)",
            data.len()
        );

        // Ensure key is correct size for Ed25519
        if key_material.len() != 32 {
            return Err(BearDogError::Crypto {
                message: format!(
                    "Invalid key size for Ed25519: expected 32 bytes, got {}",
                    key_material.len()
                ),
            });
        }

        // Ensure signature is correct size
        if signature.len() != 64 {
            return Err(BearDogError::Crypto {
                message: format!(
                    "Invalid signature size for Ed25519: expected 64 bytes, got {}",
                    signature.len()
                ),
            });
        }

        // Create signing key first, then derive verifying key
        let signing_key =
            SigningKey::from_bytes(key_material.try_into().map_err(|_| BearDogError::Crypto {
                message: "Failed to convert key material to Ed25519 signing key".to_string(),
            })?);

        // Derive the verifying key (public key) from the signing key (private key)
        let verifying_key = signing_key.verifying_key();

        // Create signature
        let signature_obj =
            Signature::from_bytes(signature.try_into().map_err(|_| BearDogError::Crypto {
                message: "Failed to convert signature bytes to Ed25519 signature".to_string(),
            })?);

        // Verify signature
        match verifying_key.verify_strict(data, &signature_obj) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false), // Invalid signature, not an error
        }
    }

    /// Derive key from master key
    async fn derive_key(
        &self,
        master_key: &[u8],
        derivation_data: &[u8],
    ) -> BearDogResult<Vec<u8>> {
        debug!("Deriving key with HKDF using Rust crypto provider");

        // Use HKDF (HMAC-based Key Derivation Function) for secure key derivation
        use hkdf::Hkdf;
        use sha2::Sha256;

        // Create HKDF instance
        let hkdf = Hkdf::<Sha256>::new(None, master_key);

        // Derive 32-byte key (256 bits) suitable for AES-256
        let mut derived_key = vec![0u8; 32];
        hkdf.expand(derivation_data, &mut derived_key)
            .map_err(|e| BearDogError::Crypto {
                message: format!("Key derivation failed: {e}"),
            })?;

        debug!("Successfully derived {}-byte key", derived_key.len());
        Ok(derived_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_rust_crypto_provider_creation() {
        let provider = RustCryptoProvider::new().await.unwrap();
        assert!(provider.initialize().await.is_ok());
    }

    #[tokio::test]
    async fn test_key_generation() {
        let provider = RustCryptoProvider::new().await.unwrap();

        let key_material = provider
            .generate_key_material(&KeyType::Aes256)
            .await
            .unwrap();
        assert_eq!(key_material.len(), 32);

        let ecc_key = provider
            .generate_key_material(&KeyType::EccP256)
            .await
            .unwrap();
        assert_eq!(ecc_key.len(), 32);
    }

    #[tokio::test]
    async fn test_encryption_decryption() {
        let provider = RustCryptoProvider::new().await.unwrap();
        let key_material = provider
            .generate_key_material(&KeyType::Aes256)
            .await
            .unwrap();

        let plaintext = b"Hello, World!";
        let ciphertext = provider.encrypt(&key_material, plaintext).await.unwrap();
        let decrypted = provider.decrypt(&key_material, &ciphertext).await.unwrap();

        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[tokio::test]
    async fn test_signing_verification() {
        let provider = RustCryptoProvider::new().await.unwrap();
        let key_material = provider
            .generate_key_material(&KeyType::EccP256)
            .await
            .unwrap();

        let data = b"Test data to sign";
        let signature = provider.sign(&key_material, data).await.unwrap();
        let is_valid = provider
            .verify(&key_material, data, &signature)
            .await
            .unwrap();

        assert!(is_valid);

        // Test with different data
        let different_data = b"Different data";
        let is_invalid = provider
            .verify(&key_material, different_data, &signature)
            .await
            .unwrap();
        assert!(!is_invalid);
    }
}
