//! # OpenSSL Crypto Provider
//!
//! This module provides the OpenSSL-based crypto provider implementation using OpenSSL bindings.
//! It uses OpenSSL's AES-256-GCM for encryption/decryption, Ed25519 for signing/verification, and HMAC-SHA256 for key derivation.

use super::super::types::*;
use crate::error::{BearDogError, BearDogResult};
use crate::tunnel::hsm::types::*;
use async_trait::async_trait;
use tracing::{debug, info};

/// OpenSSL-based crypto provider using OpenSSL bindings
impl OpenSslCryptoProvider {
    /// Create a new OpenSSL crypto provider
    pub async fn new() -> BearDogResult<Self> {
        info!("Creating OpenSSL crypto provider");
        Ok(Self)
    }
}

#[async_trait]
impl CryptoProvider for OpenSslCryptoProvider {
    /// Initialize the crypto provider
    async fn initialize(&self) -> BearDogResult<()> {
        info!("Initializing OpenSSL crypto provider");
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
        use openssl::symm::{Cipher, Crypter, Mode};
        use openssl::rand::rand_bytes;
        
        debug!(
            "Encrypting {} bytes with OpenSSL crypto provider (AES-256-GCM)",
            plaintext.len()
        );
        
        // Ensure key is correct size for AES-256-GCM
        if key_material.len() != 32 {
            return Err(BearDogError::Crypto {
                message: format!("Invalid key size for AES-256-GCM: expected 32 bytes, got {}", key_material.len()),
            });
        }
        
        // Generate secure nonce
        let mut nonce = vec![0u8; 12];
        rand_bytes(&mut nonce).map_err(|e| BearDogError::Crypto {
            message: format!("Failed to generate nonce: {e}"),
        })?;
        
        // Create encryptor
        let cipher = Cipher::aes_256_gcm();
        let mut crypter = Crypter::new(cipher, Mode::Encrypt, key_material, Some(&nonce))
            .map_err(|e| BearDogError::Crypto {
                message: format!("Failed to create AES-256-GCM encryptor: {e}"),
            })?;
        
        // Encrypt the data
        let mut ciphertext = vec![0u8; plaintext.len() + cipher.block_size()];
        let mut count = crypter.update(plaintext, &mut ciphertext)
            .map_err(|e| BearDogError::Crypto {
                message: format!("AES-256-GCM encryption failed: {e}"),
            })?;
        
        count += crypter.finalize(&mut ciphertext[count..])
            .map_err(|e| BearDogError::Crypto {
                message: format!("AES-256-GCM finalization failed: {e}"),
            })?;
        
        ciphertext.truncate(count);
        
        // Get authentication tag
        let mut tag = vec![0u8; 16];
        crypter.get_tag(&mut tag).map_err(|e| BearDogError::Crypto {
            message: format!("Failed to get authentication tag: {e}"),
        })?;
        
        // Combine nonce + ciphertext + tag
        let mut result = nonce;
        result.extend(ciphertext);
        result.extend(tag);
        
        Ok(result)
    }

    /// Decrypt data with key
    async fn decrypt(&self, key_material: &[u8], ciphertext: &[u8]) -> BearDogResult<Vec<u8>> {
        use openssl::symm::{Cipher, Crypter, Mode};
        
        debug!(
            "Decrypting {} bytes with OpenSSL crypto provider (AES-256-GCM)",
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
        
        // Extract nonce, ciphertext, and tag
        let nonce = &ciphertext[..12];
        let tag = &ciphertext[ciphertext.len() - 16..];
        let encrypted_data = &ciphertext[12..ciphertext.len() - 16];
        
        // Create decryptor
        let cipher = Cipher::aes_256_gcm();
        let mut crypter = Crypter::new(cipher, Mode::Decrypt, key_material, Some(nonce))
            .map_err(|e| BearDogError::Crypto {
                message: format!("Failed to create AES-256-GCM decryptor: {e}"),
            })?;
        
        // Set authentication tag
        crypter.set_tag(tag).map_err(|e| BearDogError::Crypto {
            message: format!("Failed to set authentication tag: {e}"),
        })?;
        
        // Decrypt the data
        let mut plaintext = vec![0u8; encrypted_data.len() + cipher.block_size()];
        let mut count = crypter.update(encrypted_data, &mut plaintext)
            .map_err(|e| BearDogError::Crypto {
                message: format!("AES-256-GCM decryption failed: {e}"),
            })?;
        
        count += crypter.finalize(&mut plaintext[count..])
            .map_err(|e| BearDogError::Crypto {
                message: format!("AES-256-GCM authentication failed: {e}"),
            })?;
        
        plaintext.truncate(count);
        
        Ok(plaintext)
    }

    /// Sign data with key
    async fn sign(&self, key_material: &[u8], data: &[u8]) -> BearDogResult<Vec<u8>> {
        use openssl::pkey::{PKey, Private};
        use openssl::sign::Signer;
        
        debug!("Signing {} bytes with OpenSSL crypto provider (Ed25519)", data.len());
        
        // Ensure key is correct size for Ed25519
        if key_material.len() != 32 {
            return Err(BearDogError::Crypto {
                message: format!("Invalid key size for Ed25519: expected 32 bytes, got {}", key_material.len()),
            });
        }
        
        // Create Ed25519 private key
        let private_key = PKey::private_key_from_raw_bytes(key_material, openssl::pkey::Id::ED25519)
            .map_err(|e| BearDogError::Crypto {
                message: format!("Failed to create Ed25519 private key: {e}"),
            })?;
        
        // Create signer
        let mut signer = Signer::new_without_digest(&private_key)
            .map_err(|e| BearDogError::Crypto {
                message: format!("Failed to create Ed25519 signer: {e}"),
            })?;
        
        // Sign the data
        let signature = signer.sign_oneshot_to_vec(data)
            .map_err(|e| BearDogError::Crypto {
                message: format!("Ed25519 signing failed: {e}"),
            })?;
        
        Ok(signature)
    }

    /// Verify signature with key
    async fn verify(
        &self,
        key_material: &[u8],
        data: &[u8],
        signature: &[u8],
    ) -> BearDogResult<bool> {
        use openssl::pkey::PKey;
        use openssl::sign::Verifier;
        
        debug!(
            "Verifying signature for {} bytes with OpenSSL crypto provider (Ed25519)",
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
        
        // Create Ed25519 public key
        let public_key = PKey::public_key_from_raw_bytes(key_material, openssl::pkey::Id::ED25519)
            .map_err(|e| BearDogError::Crypto {
                message: format!("Failed to create Ed25519 public key: {e}"),
            })?;
        
        // Create verifier
        let mut verifier = Verifier::new_without_digest(&public_key)
            .map_err(|e| BearDogError::Crypto {
                message: format!("Failed to create Ed25519 verifier: {e}"),
            })?;
        
        // Verify signature
        match verifier.verify_oneshot(signature, data) {
            Ok(true) => Ok(true),
            Ok(false) => Ok(false),
            Err(_) => Ok(false), // Invalid signature, not an error
        }
    }

    /// Derive key from master key
    async fn derive_key(
        &self,
        master_key: &[u8],
        derivation_data: &[u8],
    ) -> BearDogResult<Vec<u8>> {
        use openssl::hash::MessageDigest;
        use openssl::pkey::PKey;
        use openssl::sign::Signer;
        
        debug!("Deriving key with OpenSSL crypto provider (HMAC-SHA256)");
        
        // Ensure master key is not empty
        if master_key.is_empty() {
            return Err(BearDogError::Crypto {
                message: "Master key cannot be empty".to_string(),
            });
        }
        
        // Use HMAC-SHA256 for key derivation (simpler than HKDF)
        let key = PKey::hmac(master_key).map_err(|e| BearDogError::Crypto {
            message: format!("Failed to create HMAC key: {e}"),
        })?;
        
        let mut signer = Signer::new(MessageDigest::sha256(), &key)
            .map_err(|e| BearDogError::Crypto {
                message: format!("Failed to create HMAC signer: {e}"),
            })?;
        
        signer.update(derivation_data).map_err(|e| BearDogError::Crypto {
            message: format!("HMAC update failed: {e}"),
        })?;
        
        let derived_key = signer.sign_to_vec().map_err(|e| BearDogError::Crypto {
            message: format!("HMAC key derivation failed: {e}"),
        })?;
        
        Ok(derived_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_openssl_crypto_provider_creation() {
        let provider = OpenSslCryptoProvider::new().await.unwrap();
        assert!(provider.initialize().await.is_ok());
    }

    #[tokio::test]
    async fn test_key_generation() {
        let provider = OpenSslCryptoProvider::new().await.unwrap();
        
        let key_material = provider.generate_key_material(&KeyType::Aes256).await.unwrap();
        assert_eq!(key_material.len(), 32);
        
        let ecc_key = provider.generate_key_material(&KeyType::EccP256).await.unwrap();
        assert_eq!(ecc_key.len(), 32);
    }

    #[tokio::test]
    async fn test_encryption_decryption() {
        let provider = OpenSslCryptoProvider::new().await.unwrap();
        let key_material = provider.generate_key_material(&KeyType::Aes256).await.unwrap();
        
        let plaintext = b"Hello, World!";
        let ciphertext = provider.encrypt(&key_material, plaintext).await.unwrap();
        let decrypted = provider.decrypt(&key_material, &ciphertext).await.unwrap();
        
        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[tokio::test]
    async fn test_signing_verification() {
        let provider = OpenSslCryptoProvider::new().await.unwrap();
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
        let provider = OpenSslCryptoProvider::new().await.unwrap();
        
        let master_key = b"master_key_for_derivation_test";
        let derivation_data = b"derivation_context";
        
        let derived_key1 = provider.derive_key(master_key, derivation_data).await.unwrap();
        let derived_key2 = provider.derive_key(master_key, derivation_data).await.unwrap();
        
        // Same inputs should produce same outputs
        assert_eq!(derived_key1, derived_key2);
        assert!(derived_key1.len() > 0);
        
        // Different derivation data should produce different keys
        let different_derivation_data = b"different_context";
        let different_key = provider.derive_key(master_key, different_derivation_data).await.unwrap();
        assert_ne!(derived_key1, different_key);
    }
} 