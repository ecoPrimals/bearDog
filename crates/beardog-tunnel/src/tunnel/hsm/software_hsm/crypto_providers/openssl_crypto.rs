use super::super::types::*;
use crate::tunnel::hsm::types::KeyType;
use beardog_errors::BearDogError;
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rand::rand_bytes;
use openssl::sign::Signer;
use openssl::symm::{Cipher, Crypter, Mode};
use tracing::{debug, info};

/// OpenSSL-based crypto provider with hardware acceleration
///
/// Uses OpenSSL for cryptographic operations, providing hardware acceleration
/// where available and broad algorithm support.
#[derive(Debug, Clone)]
pub struct OpenSslCryptoProvider;

impl OpenSslCryptoProvider {
    /// Create new OpenSSL crypto provider
    ///
    /// # Errors
    /// Returns an error if the provider cannot be created
    pub async fn new() -> Result<Self, BearDogError> {
        info!("Creating OpenSSL crypto provider");
        Ok(Self)
    }
}

#[async_trait::async_trait]
impl CryptoProvider<KeyType> for OpenSslCryptoProvider {
    async fn initialize(&self) -> Result<(), BearDogError> {
        info!("Initializing OpenSSL crypto provider");
        Ok(())
    }

    async fn generate_key_material(&self, key_type: &KeyType) -> Result<Vec<u8>, BearDogError> {
        use rand::RngCore;
        let key_size = match key_type {
            KeyType::Aes | KeyType::ChaCha20 => 32, // AES-256 or ChaCha20
            KeyType::EllipticCurve | KeyType::Ed25519 | KeyType::X25519 => 32, // EC keys
            KeyType::Rsa => 256,                    // RSA-2048 (256 bytes)
            KeyType::Generic | KeyType::Custom(_) => 32, // Default
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

    async fn encrypt(
        &self,
        key_material: &[u8],
        plaintext: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        info!(
            "Encrypting {} bytes with OpenSSL crypto provider (AES-256-GCM)",
            plaintext.len()
        );

        if key_material.len() != 32 {
            return Err(BearDogError::crypto_error(format!(
                "Invalid key size for AES-256-GCM: expected 32 bytes, got {}",
                key_material.len()
            )));
        }

        let mut nonce = vec![0u8; 12];
        rand_bytes(&mut nonce).map_err(|e| {
            BearDogError::crypto_error(format!("Failed to generate secure nonce: {e}"))
        })?;

        let cipher = Cipher::aes_256_gcm();
        let mut crypter =
            Crypter::new(cipher, Mode::Encrypt, key_material, Some(&nonce)).map_err(|e| {
                BearDogError::crypto_error(format!("Failed to create AES-256-GCM encryptor: {e}"))
            })?;

        let mut ciphertext = vec![0u8; plaintext.len() + cipher.block_size()];
        let mut count = crypter.update(plaintext, &mut ciphertext).map_err(|e| {
            BearDogError::crypto_error(format!("AES-256-GCM encryption failed: {e}"))
        })?;
        count += crypter.finalize(&mut ciphertext[count..]).map_err(|e| {
            BearDogError::crypto_error(format!("AES-256-GCM finalization failed: {e}"))
        })?;
        ciphertext.truncate(count);

        let mut tag = vec![0u8; 16];
        crypter.get_tag(&mut tag).map_err(|e| {
            BearDogError::crypto_error(format!("Failed to get authentication tag: {e}"))
        })?;

        let mut result = nonce;
        result.extend(ciphertext);
        result.extend(tag);
        Ok(result)
    }

    async fn decrypt(
        &self,
        key_material: &[u8],
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        info!(
            "Decrypting {} bytes with OpenSSL crypto provider (AES-256-GCM)",
            ciphertext.len()
        );

        if ciphertext.len() < 12 + 16 {
            return Err(BearDogError::crypto_error(format!(
                "Ciphertext too short: expected at least 28 bytes, got {}",
                ciphertext.len()
            )));
        }

        let nonce = &ciphertext[..12];
        let tag = &ciphertext[ciphertext.len() - 16..];
        let encrypted_data = &ciphertext[12..ciphertext.len() - 16];

        let cipher = Cipher::aes_256_gcm();
        let mut crypter =
            Crypter::new(cipher, Mode::Decrypt, key_material, Some(nonce)).map_err(|e| {
                BearDogError::crypto_error(format!("Failed to create AES-256-GCM decryptor: {e}"))
            })?;

        crypter.set_tag(tag).map_err(|e| {
            BearDogError::crypto_error(format!("Failed to set authentication tag: {e}"))
        })?;

        let mut plaintext = vec![0u8; encrypted_data.len() + cipher.block_size()];
        let mut count = crypter
            .update(encrypted_data, &mut plaintext)
            .map_err(|e| {
                BearDogError::crypto_error(format!("AES-256-GCM decryption failed: {e}"))
            })?;
        count += crypter.finalize(&mut plaintext[count..]).map_err(|e| {
            BearDogError::crypto_error(format!("AES-256-GCM authentication failed: {e}"))
        })?;
        plaintext.truncate(count);
        Ok(plaintext)
    }

    async fn sign(&self, key_material: &[u8], data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        info!(
            "Signing {} bytes with OpenSSL crypto provider (Ed25519)",
            data.len()
        );

        if key_material.len() != 32 {
            return Err(BearDogError::crypto_error(format!(
                "Invalid key size for Ed25519: expected 32 bytes, got {}",
                key_material.len()
            )));
        }

        // OpenSSL signing implementation placeholder
        Err(BearDogError::crypto_error(
            "OpenSSL Ed25519 signing not yet implemented".to_string(),
        ))
    }

    async fn verify(
        &self,
        _key_material: &[u8],
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
        info!(
            "Verifying signature for {} bytes with OpenSSL crypto provider (Ed25519)",
            data.len()
        );

        if signature.len() != 64 {
            return Err(BearDogError::crypto_error(format!(
                "Invalid signature size for Ed25519: expected 64 bytes, got {}",
                signature.len()
            )));
        }

        // OpenSSL verification implementation placeholder
        Err(BearDogError::crypto_error(
            "OpenSSL Ed25519 verification not yet implemented".to_string(),
        ))
    }

    async fn derive_key(
        &self,
        root_key: &[u8],
        derivation_data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        debug!("Deriving key with OpenSSL crypto provider (HMAC-SHA256)");

        if root_key.is_empty() {
            return Err(BearDogError::crypto_error(
                "Root key cannot be empty".to_string(),
            ));
        }

        let key = PKey::hmac(root_key)
            .map_err(|e| BearDogError::crypto_error(format!("Failed to create HMAC key: {e}")))?;

        let mut signer = Signer::new(MessageDigest::sha256(), &key).map_err(|e| {
            BearDogError::crypto_error(format!("Failed to create HMAC signer: {e}"))
        })?;

        signer
            .update(derivation_data)
            .map_err(|e| BearDogError::crypto_error(format!("HMAC update failed: {e}")))?;

        let derived_key = signer
            .sign_to_vec()
            .map_err(|e| BearDogError::crypto_error(format!("HMAC key derivation failed: {e}")))?;

        Ok(derived_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_openssl_crypto_provider_creation() -> Result<(), BearDogError> {
        let provider = OpenSslCryptoProvider::new().await?;
        assert!(provider.initialize().await.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_key_generation() -> Result<(), BearDogError> {
        let provider = OpenSslCryptoProvider::new().await?;
        let key_material = provider.generate_key_material(&KeyType::Aes).await?;
        assert_eq!(key_material.len(), 32);

        let ecc_key = provider
            .generate_key_material(&KeyType::EllipticCurve)
            .await?;
        assert_eq!(ecc_key.len(), 32);
        Ok(())
    }

    #[tokio::test]
    async fn test_encryption_decryption() -> Result<(), BearDogError> {
        let provider = OpenSslCryptoProvider::new().await?;
        let key_material = provider.generate_key_material(&KeyType::Aes).await?;
        let plaintext = b"Hello, World!";

        let ciphertext = provider.encrypt(&key_material, plaintext).await?;
        let decrypted = provider.decrypt(&key_material, &ciphertext).await?;

        assert_eq!(plaintext, decrypted.as_slice());
        Ok(())
    }

    #[tokio::test]
    async fn test_key_derivation() -> Result<(), BearDogError> {
        let provider = OpenSslCryptoProvider::new().await?;
        let root_key = b"root_key_for_derivation_test";
        let derivation_data = b"derivation_context";

        let derived_key1 = provider.derive_key(root_key, derivation_data).await?;
        let derived_key2 = provider.derive_key(root_key, derivation_data).await?;

        assert_eq!(derived_key1, derived_key2);
        assert!(!derived_key1.is_empty());

        let different_derivation_data = b"different_context";
        let different_key = provider
            .derive_key(root_key, different_derivation_data)
            .await?;

        assert_ne!(derived_key1, different_key);
        Ok(())
    }
}
