//! # Ring Crypto Provider
//!
//! This module provides the Ring-based crypto provider implementation using the Ring cryptographic library.
//! It uses Ring's AES-256-GCM for encryption/decryption, Ed25519 for signing/verification, and HKDF for key derivation.

use super::super::types::*;
use crate::error::{BearDogError, BearDogResult};
use crate::tunnel::hsm::types::*;
use async_trait::async_trait;
use tracing::{debug, info};

/// Ring-based crypto provider using the Ring cryptographic library
impl RingCryptoProvider {
    /// Create a new Ring crypto provider
    pub async fn new() -> BearDogResult<Self> {
        info!("Creating Ring crypto provider");
        Ok(Self)
    }
}

#[async_trait]
impl CryptoProvider for RingCryptoProvider {
    /// Initialize the crypto provider
    async fn initialize(&self) -> BearDogResult<()> {
        info!("Initializing Ring crypto provider");
        Ok(())
    }

    /// Generate secure key material
    async fn generate_key_material(&self, key_type: &KeyType) -> BearDogResult<Vec<u8>> {
        use rand::RngCore;

        let key_size = match key_type {
            KeyType::Aes256 => 32,
            KeyType::EccP256 => 32,
            KeyType::EccP384 => 48,
            KeyType::ChaCha20 => 32,
            KeyType::Rsa { key_size } => key_size / 8,
            _ => 32,
        };

        let mut key_material = vec![0u8; key_size as usize];
        rand::thread_rng().fill_bytes(&mut key_material);

        debug!(
            "Generated key material for {:?}: {} bytes",
            key_type,
            key_material.len()
        );
        Ok(key_material)
    }

    /// Encrypt data with key
    async fn encrypt(&self, key_material: &[u8], plaintext: &[u8]) -> BearDogResult<Vec<u8>> {
        use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
        use ring::rand::{SecureRandom, SystemRandom};
        
        debug!(
            "Encrypting {} bytes with Ring crypto provider (AES-256-GCM)",
            plaintext.len()
        );
        
        // Ensure key is correct size for AES-256-GCM
        if key_material.len() != 32 {
            return Err(BearDogError::Crypto {
                message: format!("Invalid key size for AES-256-GCM: expected 32 bytes, got {}", key_material.len()),
            });
        }
        
        // Create unbound key
        let unbound_key = UnboundKey::new(&AES_256_GCM, key_material)
            .map_err(|e| BearDogError::Crypto {
                message: format!("Failed to create AES-256-GCM key: {e}"),
            })?;
        
        let key = LessSafeKey::new(unbound_key);
        
        // Generate secure nonce
        let rng = SystemRandom::new();
        let mut nonce_bytes = [0u8; 12];
        rng.fill(&mut nonce_bytes).map_err(|e| BearDogError::Crypto {
            message: format!("Failed to generate nonce: {e}"),
        })?;
        
        let nonce = Nonce::assume_unique_for_key(nonce_bytes);
        
        // Encrypt the data
        let mut ciphertext = plaintext.to_vec();
        key.seal_in_place_append_tag(nonce, Aad::empty(), &mut ciphertext)
            .map_err(|e| BearDogError::Crypto {
                message: format!("AES-256-GCM encryption failed: {e}"),
            })?;
        
        // Prepend nonce to ciphertext for decryption
        let mut result = nonce_bytes.to_vec();
        result.extend(ciphertext);
        
        Ok(result)
    }

    /// Decrypt data with key
    async fn decrypt(&self, key_material: &[u8], ciphertext: &[u8]) -> BearDogResult<Vec<u8>> {
        use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
        
        debug!(
            "Decrypting {} bytes with Ring crypto provider (AES-256-GCM)",
            ciphertext.len()
        );
        
        // Ensure key is correct size for AES-256-GCM
        if key_material.len() != 32 {
            return Err(BearDogError::Crypto {
                message: format!("Invalid key size for AES-256-GCM: expected 32 bytes, got {}", key_material.len()),
            });
        }
        
        // Ensure ciphertext is large enough to contain nonce + tag
        if ciphertext.len() < 12 + 16 {
            return Err(BearDogError::Crypto {
                message: format!("Ciphertext too short: expected at least 28 bytes, got {}", ciphertext.len()),
            });
        }
        
        // Extract nonce and ciphertext
        let nonce_bytes: [u8; 12] = ciphertext[..12].try_into()
            .map_err(|_| SoftwareHsmError::CryptoOperation {
                message: "Failed to extract nonce from ciphertext".to_string(),
            })?;
        let nonce = Nonce::assume_unique_for_key(nonce_bytes);
        let mut encrypted_data = ciphertext[12..].to_vec();
        
        // Create unbound key
        let unbound_key = UnboundKey::new(&AES_256_GCM, key_material)
            .map_err(|e| BearDogError::Crypto {
                message: format!("Failed to create AES-256-GCM key: {e}"),
            })?;
        
        let key = LessSafeKey::new(unbound_key);
        
        // Decrypt the data
        let plaintext = key.open_in_place(nonce, Aad::empty(), &mut encrypted_data)
            .map_err(|e| BearDogError::Crypto {
                message: format!("AES-256-GCM decryption failed: {e}"),
            })?;
        
        Ok(plaintext.to_vec())
    }

    /// Sign data with key
    async fn sign(&self, key_material: &[u8], data: &[u8]) -> BearDogResult<Vec<u8>> {
        use ring::signature::{Ed25519KeyPair, KeyPair};
        
        debug!("Signing {} bytes with Ring crypto provider (Ed25519)", data.len());
        
        // Ensure key is correct size for Ed25519
        if key_material.len() != 32 {
            return Err(BearDogError::Crypto {
                message: format!("Invalid key size for Ed25519: expected 32 bytes, got {}", key_material.len()),
            });
        }
        
        // Create Ed25519 key pair from seed
        let key_pair = Ed25519KeyPair::from_seed_unchecked(key_material)
            .map_err(|e| BearDogError::Crypto {
                message: format!("Failed to create Ed25519 key pair: {e}"),
            })?;
        
        // Sign the data
        let signature = key_pair.sign(data);
        
        Ok(signature.as_ref().to_vec())
    }

    /// Verify signature with key
    async fn verify(
        &self,
        key_material: &[u8],
        data: &[u8],
        signature: &[u8],
    ) -> BearDogResult<bool> {
        use ring::signature::{UnparsedPublicKey, ED25519};
        
        debug!(
            "Verifying signature for {} bytes with Ring crypto provider (Ed25519)",
            data.len()
        );
        
        // Ensure key is correct size for Ed25519
        if key_material.len() != 32 {
            return Err(BearDogError::Crypto {
                message: format!("Invalid key size for Ed25519: expected 32 bytes, got {}", key_material.len()),
            });
        }
        
        // Ensure signature is correct size
        if signature.len() != 64 {
            return Err(BearDogError::Crypto {
                message: format!("Invalid signature size for Ed25519: expected 64 bytes, got {}", signature.len()),
            });
        }
        
        // Create public key for verification
        let public_key = UnparsedPublicKey::new(&ED25519, key_material);
        
        // Verify signature
        match public_key.verify(data, signature) {
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
        use ring::hkdf::{Prk, HKDF_SHA256};
        
        debug!("Deriving key with Ring crypto provider (HKDF-SHA256)");
        
        // Ensure master key is not empty
        if master_key.is_empty() {
            return Err(BearDogError::Crypto {
                message: "Master key cannot be empty".to_string(),
            });
        }
        
        // Use HKDF to derive key
        let prk = Prk::new_less_safe(HKDF_SHA256, master_key);
        
        // Derive 32-byte key (suitable for AES-256 or Ed25519)
        let mut derived_key = vec![0u8; 32];
        prk.expand(&[derivation_data], HKDF_SHA256)
            .map_err(|e| BearDogError::Crypto {
                message: format!("HKDF expansion failed: {e}"),
            })?
            .fill(&mut derived_key)
            .map_err(|e| BearDogError::Crypto {
                message: format!("HKDF key derivation failed: {e}"),
            })?;
        
        Ok(derived_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_ring_crypto_provider_creation() {
        let provider = RingCryptoProvider::new().await.unwrap();
        assert!(provider.initialize().await.is_ok());
    }

    #[tokio::test]
    async fn test_key_generation() {
        let provider = RingCryptoProvider::new().await.unwrap();
        
        let key_material = provider.generate_key_material(&KeyType::Aes256).await.unwrap();
        assert_eq!(key_material.len(), 32);
        
        let ecc_key = provider.generate_key_material(&KeyType::EccP256).await.unwrap();
        assert_eq!(ecc_key.len(), 32);
    }

    #[tokio::test]
    async fn test_encryption_decryption() {
        let provider = RingCryptoProvider::new().await.unwrap();
        let key_material = provider.generate_key_material(&KeyType::Aes256).await.unwrap();
        
        let plaintext = b"Hello, World!";
        let ciphertext = provider.encrypt(&key_material, plaintext).await.unwrap();
        let decrypted = provider.decrypt(&key_material, &ciphertext).await.unwrap();
        
        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[tokio::test]
    async fn test_signing_verification() {
        let provider = RingCryptoProvider::new().await.unwrap();
        let key_material = provider.generate_key_material(&KeyType::EccP256).await.unwrap();
        
        let data = b"Test data to sign";
        let signature = provider.sign(&key_material, data).await.unwrap();
        let is_valid = provider.verify(&key_material, data, &signature).await.unwrap();
        
        assert!(is_valid);
        
        // Test with different data
        let different_data = b"Different data";
        let is_invalid = provider.verify(&key_material, different_data, &signature).await.unwrap();
        assert!(!is_invalid);
    }

    #[tokio::test]
    async fn test_key_derivation() {
        let provider = RingCryptoProvider::new().await.unwrap();
        
        let master_key = b"master_key_for_derivation_test";
        let derivation_data = b"derivation_context";
        
        let derived_key1 = provider.derive_key(master_key, derivation_data).await.unwrap();
        let derived_key2 = provider.derive_key(master_key, derivation_data).await.unwrap();
        
        // Same inputs should produce same outputs
        assert_eq!(derived_key1, derived_key2);
        assert_eq!(derived_key1.len(), 32);
        
        // Different derivation data should produce different keys
        let different_derivation_data = b"different_context";
        let different_key = provider.derive_key(master_key, different_derivation_data).await.unwrap();
        assert_ne!(derived_key1, different_key);
    }
} 