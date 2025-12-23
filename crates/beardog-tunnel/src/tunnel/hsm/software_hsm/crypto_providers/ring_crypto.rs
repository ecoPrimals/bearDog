use crate::tunnel::hsm::software_hsm::CryptoProvider;
use crate::tunnel::hsm::types::KeyType;
use arrayref::array_ref;
use beardog_errors::BearDogError;
use ring::aead::{LessSafeKey, Nonce, UnboundKey, AES_256_GCM, NONCE_LEN};
use ring::rand::{SecureRandom, SystemRandom};
use ring::signature::{Ed25519KeyPair, UnparsedPublicKey, ED25519, ED25519_PUBLIC_KEY_LEN};
use std::sync::Arc;
use tracing::{debug, info};

/// Ring crypto provider with hardware acceleration
#[derive(Debug, Clone)]
pub struct RingCryptoProvider {
    rng: Arc<SystemRandom>,
    name: String,
}

impl RingCryptoProvider {
    /// Create new Ring crypto provider
    ///
    /// # Errors
    /// Returns an error if the provider cannot be created
    pub fn new() -> Result<Self, BearDogError> {
        info!("🔧 Initializing Ring crypto provider with hardware acceleration");
        Ok(Self {
            rng: Arc::new(SystemRandom::new()),
            name: "Ring-Hardware-Accelerated".to_string(),
        })
    }

    /// Get provider name
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[async_trait::async_trait]
impl CryptoProvider<KeyType> for RingCryptoProvider {
    async fn initialize(&self) -> Result<(), BearDogError> {
        info!("🚀 Ring crypto provider initialized successfully");
        Ok(())
    }

    async fn generate_key_material(&self, key_type: &KeyType) -> Result<Vec<u8>, BearDogError> {
        debug!("🔑 Generating {:?} key with Ring provider", key_type);
        let key_length = match key_type {
            KeyType::Aes | KeyType::ChaCha20 => 32, // Symmetric keys
            KeyType::Ed25519 | KeyType::X25519 | KeyType::EllipticCurve => 32, // EC keys
            KeyType::Rsa | KeyType::Generic | KeyType::Custom(_) => {
                return Err(BearDogError::unsupported_operation(format!(
                    "Ring provider only supports AES, ChaCha20, Ed25519, X25519, and ECC. Got: {key_type:?}"
                )))
            }
        };
        let mut key_material = vec![0u8; key_length];
        self.rng
            .fill(&mut key_material)
            .map_err(|e| BearDogError::crypto_error(format!("Failed to generate key: {e:?}")))?;
        debug!("✅ Generated {} byte key", key_material.len());
        Ok(key_material)
    }

    async fn encrypt(
        &self,
        key_material: &[u8],
        plaintext: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        debug!(
            "🔒 Encrypting {} bytes with Ring AES-256-GCM",
            plaintext.len()
        );

        if key_material.len() != 32 {
            return Err(BearDogError::crypto_error(format!(
                "Invalid key size: expected 32 bytes, got {}",
                key_material.len()
            )));
        }

        let unbound_key = UnboundKey::new(&AES_256_GCM, key_material).map_err(|e| {
            BearDogError::crypto_error(format!("Failed to create AES-256-GCM key: {e:?}"))
        })?;
        let key = LessSafeKey::new(unbound_key);

        let mut nonce_bytes = vec![0u8; NONCE_LEN];
        self.rng
            .fill(&mut nonce_bytes)
            .map_err(|e| BearDogError::crypto_error(format!("Failed to generate nonce: {e:?}")))?;
        let nonce = Nonce::assume_unique_for_key(*array_ref![nonce_bytes, 0, NONCE_LEN]);

        let mut in_out = plaintext.to_vec();
        key.seal_in_place_append_tag(nonce, ring::aead::Aad::empty(), &mut in_out)
            .map_err(|e| BearDogError::crypto_error(format!("Failed to encrypt data: {e:?}")))?;

        let mut result = nonce_bytes;
        result.extend(in_out);
        debug!("✅ Encrypted to {} bytes", result.len());
        Ok(result)
    }

    async fn decrypt(
        &self,
        key_material: &[u8],
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        debug!(
            "🔓 Decrypting {} bytes with Ring AES-256-GCM",
            ciphertext.len()
        );

        if ciphertext.len() < NONCE_LEN {
            return Err(BearDogError::crypto_error(format!(
                "Ciphertext too short: expected at least {NONCE_LEN} bytes, got {}",
                ciphertext.len()
            )));
        }

        let nonce = Nonce::assume_unique_for_key(*array_ref![ciphertext, 0, NONCE_LEN]);
        let mut in_out = ciphertext[NONCE_LEN..].to_vec();

        let unbound_key = UnboundKey::new(&AES_256_GCM, key_material).map_err(|e| {
            BearDogError::crypto_error(format!("Failed to create AES-256-GCM key: {e:?}"))
        })?;
        let key = LessSafeKey::new(unbound_key);

        let plaintext = key
            .open_in_place(nonce, ring::aead::Aad::empty(), &mut in_out)
            .map_err(|e| BearDogError::crypto_error(format!("Failed to decrypt data: {e:?}")))?;
        debug!("✅ Decrypted to {} bytes", plaintext.len());
        Ok(plaintext.to_vec())
    }

    async fn sign(&self, key_material: &[u8], data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        debug!("✍️ Signing {} bytes with Ring Ed25519", data.len());

        if key_material.len() != 32 {
            return Err(BearDogError::crypto_error(format!(
                "Invalid private key length: expected 32, got {}",
                key_material.len()
            )));
        }

        let key_pair = Ed25519KeyPair::from_seed_unchecked(key_material).map_err(|e| {
            BearDogError::crypto_error(format!("Failed to create Ed25519 key pair: {e:?}"))
        })?;
        let signature = key_pair.sign(data);
        debug!(
            "✅ Generated signature of {} bytes",
            signature.as_ref().len()
        );
        Ok(signature.as_ref().to_vec())
    }

    async fn verify(
        &self,
        key_material: &[u8],
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
        debug!(
            "🔍 Verifying signature for {} bytes with Ring Ed25519",
            data.len()
        );

        if key_material.len() != ED25519_PUBLIC_KEY_LEN {
            return Err(BearDogError::crypto_error(format!(
                "Invalid public key length: expected {}, got {}",
                ED25519_PUBLIC_KEY_LEN,
                key_material.len()
            )));
        }

        let public_key = UnparsedPublicKey::new(&ED25519, key_material);
        match public_key.verify(data, signature) {
            Ok(()) => {
                debug!("✅ Signature valid");
                Ok(true)
            }
            Err(_) => {
                debug!("❌ Signature invalid");
                Ok(false)
            }
        }
    }

    async fn derive_key(
        &self,
        root_key: &[u8],
        derivation_data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        debug!("🔑 Deriving key with Ring HMAC-SHA256");

        if root_key.is_empty() {
            return Err(BearDogError::crypto_error(
                "Root key cannot be empty".to_string(),
            ));
        }

        let key = ring::hmac::Key::new(ring::hmac::HMAC_SHA256, root_key);
        let tag = ring::hmac::sign(&key, derivation_data);
        debug!("✅ Derived key of {} bytes", tag.as_ref().len());
        Ok(tag.as_ref().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ring_provider_creation() -> Result<(), BearDogError> {
        let provider = RingCryptoProvider::new()?;
        assert_eq!(provider.name(), "Ring-Hardware-Accelerated");
        Ok(())
    }

    #[tokio::test]
    async fn test_key_generation() -> Result<(), BearDogError> {
        let provider = RingCryptoProvider::new()?;
        let key = provider.generate_key_material(&KeyType::Aes).await?;
        assert_eq!(key.len(), 32);
        Ok(())
    }

    #[tokio::test]
    async fn test_ring_encrypt_decrypt() -> Result<(), BearDogError> {
        let provider = RingCryptoProvider::new()?;
        let key = provider.generate_key_material(&KeyType::Aes).await?;
        let plaintext = b"Ring encryption test";
        let ciphertext = provider.encrypt(&key, plaintext).await?;
        let decrypted = provider.decrypt(&key, &ciphertext).await?;
        assert_eq!(plaintext.to_vec(), decrypted);
        Ok(())
    }

    #[tokio::test]
    async fn test_ring_sign_verify() -> Result<(), BearDogError> {
        let provider = RingCryptoProvider::new()?;
        let private_key = provider.generate_key_material(&KeyType::Ed25519).await?;

        // Sign a message with the private key
        let message = b"Ring crypto signature test";
        let signature = provider.sign(&private_key, message).await?;

        // For Ring Ed25519: the public key is separate from the private key
        // In this test, we need to extract/derive the public key from the keypair
        // Ring's Ed25519KeyPair can be used to get the public key bytes
        let _key_pair = Ed25519KeyPair::from_seed_unchecked(&private_key)
            .map_err(|e| BearDogError::internal(format!("Key pair creation failed: {e}")))?;

        // The peer_public_key_from_seed function or similar would give us the public key
        // For Ed25519, the public key can be derived using standard Ed25519 operations
        // Since Ring's API doesn't directly expose this in older versions,
        // we use ed25519_dalek to derive the public key for testing
        use ed25519_dalek::SigningKey;
        let signing_key = SigningKey::from_bytes(
            &private_key
                .try_into()
                .map_err(|_| BearDogError::internal("Invalid private key length".to_string()))?,
        );
        let public_key_bytes = signing_key.verifying_key().to_bytes();

        // Now verify using the public key
        let is_valid = provider
            .verify(&public_key_bytes, message, &signature)
            .await?;
        assert!(is_valid);

        let different_message = b"Different message";
        let is_valid_wrong = provider
            .verify(&public_key_bytes, different_message, &signature)
            .await?;
        assert!(!is_valid_wrong);
        Ok(())
    }

    #[tokio::test]
    async fn test_ring_derive_key() -> Result<(), BearDogError> {
        let provider = RingCryptoProvider::new()?;
        let root_key = b"ring_root_key_for_derivation";
        let context1 = b"context1";
        let context2 = b"context2";

        let derived1 = provider.derive_key(root_key, context1).await?;
        let derived2 = provider.derive_key(root_key, context1).await?;
        assert_eq!(derived1, derived2);

        let derived3 = provider.derive_key(root_key, context2).await?;
        assert_ne!(derived1, derived3);
        Ok(())
    }
}
