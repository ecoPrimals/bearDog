// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::tunnel::hsm::types::KeyType;
use beardog_errors::BearDogError;
use beardog_types::hsm::CryptoProvider; // Import the canonical trait
use chacha20poly1305::{
    ChaCha20Poly1305,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use hkdf::Hkdf;
use rand::RngCore;
use sha2::Sha256;
use std::future::Future;
use tracing::{debug, info};

/// HSM-layer crypto provider using pure-Rust `RustCrypto` crates.
///
/// Implements `beardog_types::hsm::CryptoProvider<KeyType>` for the software HSM
/// key lifecycle (generate, encrypt, sign, verify). This is distinct from the
/// universal `RustCryptoProvider` in `tunnel::hsm::crypto::providers` which
/// implements the higher-level `UniversalCryptoProvider` trait.
#[derive(Debug, Clone)]
pub struct SoftwareHsmCryptoProvider;

impl SoftwareHsmCryptoProvider {
    /// Create new Rust crypto provider
    ///
    /// # Errors
    /// Returns an error if the provider cannot be created
    pub async fn new() -> Result<Self, BearDogError> {
        info!("Creating pure Rust crypto provider");
        Ok(Self)
    }
}

impl CryptoProvider<KeyType> for SoftwareHsmCryptoProvider {
    async fn initialize(&self) -> Result<(), BearDogError> {
        info!("Initializing Rust crypto provider");
        Ok(())
    }

    fn generate_key_material(
        &self,
        key_type: &KeyType,
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send + '_ {
        let key_type = key_type.clone();
        async move {
            debug!("Generating {:?} key with Rust crypto", key_type);
            let key_length = match &key_type {
                KeyType::Aes | KeyType::ChaCha20 => 32, // Symmetric keys
                KeyType::EllipticCurve | KeyType::Ed25519 | KeyType::X25519 => 32, // EC keys
                KeyType::Rsa => 256,                    // RSA-2048
                KeyType::Generic | KeyType::Custom(_) => 32, // Default
            };
            #[expect(
                clippy::cast_sign_loss,
                reason = "key sizes from KeyType are positive byte lengths"
            )]
            let mut key_material = vec![0u8; key_length as usize];
            rand::rng().fill_bytes(&mut key_material);
            debug!("Generated {} byte key", key_material.len());
            Ok(key_material)
        }
    }

    fn encrypt(
        &self,
        key_material: &[u8],
        plaintext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send + '_ {
        let key_material = key_material.to_vec();
        let plaintext = plaintext.to_vec();
        async move {
            debug!(
                "Encrypting {} bytes with ChaCha20-Poly1305",
                plaintext.len()
            );

            if key_material.len() != 32 {
                return Err(BearDogError::crypto_error(format!(
                    "Invalid key size for ChaCha20-Poly1305: expected 32 bytes, got {}",
                    key_material.len()
                )));
            }

            let key: [u8; 32] = key_material
                .try_into()
                .map_err(|_| BearDogError::crypto_error("Invalid key length".to_string()))?;

            let cipher = ChaCha20Poly1305::new(&key.into());
            let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

            let ciphertext = cipher
                .encrypt(&nonce, plaintext.as_slice())
                .map_err(|e| BearDogError::crypto_error(format!("Encryption failed: {e}")))?;

            let mut result = nonce.to_vec();
            result.extend(ciphertext);
            debug!("Encrypted to {} bytes", result.len());
            Ok(result)
        }
    }

    fn decrypt(
        &self,
        key_material: &[u8],
        ciphertext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send + '_ {
        let key_material = key_material.to_vec();
        let ciphertext = ciphertext.to_vec();
        async move {
            debug!(
                "Decrypting {} bytes with ChaCha20-Poly1305",
                ciphertext.len()
            );

            if ciphertext.len() < 12 {
                return Err(BearDogError::crypto_error(format!(
                    "Ciphertext too short: expected at least 12 bytes, got {}",
                    ciphertext.len()
                )));
            }

            let key: [u8; 32] = key_material
                .try_into()
                .map_err(|_| BearDogError::crypto_error("Invalid key length".to_string()))?;

            let cipher = ChaCha20Poly1305::new(&key.into());
            let (nonce_bytes, encrypted_data) = ciphertext.split_at(12);
            let nonce = nonce_bytes.into();

            let plaintext = cipher
                .decrypt(nonce, encrypted_data)
                .map_err(|e| BearDogError::crypto_error(format!("Decryption failed: {e}")))?;

            debug!("Decrypted to {} bytes", plaintext.len());
            Ok(plaintext)
        }
    }

    fn sign(
        &self,
        key_material: &[u8],
        data: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send + '_ {
        let key_material = key_material.to_vec();
        let data = data.to_vec();
        async move {
            debug!("Signing {} bytes with Ed25519", data.len());

            if key_material.len() != 32 {
                return Err(BearDogError::crypto_error(format!(
                    "Invalid key size for Ed25519: expected 32 bytes, got {}",
                    key_material.len()
                )));
            }

            let key_arr: [u8; 32] = key_material
                .try_into()
                .map_err(|_| BearDogError::crypto_error("Invalid signing key".to_string()))?;
            let signing_key = SigningKey::from_bytes(&key_arr);

            let signature = signing_key.sign(data.as_slice());
            debug!(
                "Generated signature of {} bytes",
                signature.to_bytes().len()
            );
            Ok(signature.to_bytes().to_vec())
        }
    }

    fn verify(
        &self,
        key_material: &[u8],
        data: &[u8],
        signature: &[u8],
    ) -> impl Future<Output = Result<bool, BearDogError>> + Send + '_ {
        let key_material = key_material.to_vec();
        let data = data.to_vec();
        let signature = signature.to_vec();
        async move {
            debug!("Verifying signature for {} bytes with Ed25519", data.len());

            if signature.len() != 64 {
                return Err(BearDogError::crypto_error(format!(
                    "Invalid signature size for Ed25519: expected 64 bytes, got {}",
                    signature.len()
                )));
            }

            let vk_bytes: [u8; 32] = key_material
                .try_into()
                .map_err(|_| BearDogError::crypto_error("Invalid verifying key".to_string()))?;
            let verifying_key = VerifyingKey::from_bytes(&vk_bytes).map_err(|e| {
                BearDogError::crypto_error(format!("Failed to create verifying key: {e}"))
            })?;

            let sig_bytes: [u8; 64] = signature
                .try_into()
                .map_err(|_| BearDogError::crypto_error("Invalid signature length".to_string()))?;
            let sig = ed25519_dalek::Signature::from_bytes(&sig_bytes);

            if matches!(verifying_key.verify(data.as_slice(), &sig), Ok(())) {
                debug!("Signature valid");
                Ok(true)
            } else {
                debug!("Signature invalid");
                Ok(false)
            }
        }
    }

    fn derive_key(
        &self,
        root_key: &[u8],
        derivation_data: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send + '_ {
        let root_key = root_key.to_vec();
        let derivation_data = derivation_data.to_vec();
        async move {
            debug!("Deriving key with HKDF-SHA256");

            if root_key.is_empty() {
                return Err(BearDogError::crypto_error(
                    "Root key cannot be empty".to_string(),
                ));
            }

            let hkdf = Hkdf::<Sha256>::new(None, root_key.as_slice());
            let mut derived_key = vec![0u8; 32];
            hkdf.expand(derivation_data.as_slice(), &mut derived_key)
                .map_err(|e| BearDogError::crypto_error(format!("Key derivation failed: {e}")))?;

            debug!("Derived key of {} bytes", derived_key.len());
            Ok(derived_key)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rust_crypto_provider_creation() -> Result<(), BearDogError> {
        let provider = SoftwareHsmCryptoProvider::new().await?;
        assert!(provider.initialize().await.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_key_generation() -> Result<(), BearDogError> {
        let provider = SoftwareHsmCryptoProvider::new().await?;
        let key = provider.generate_key_material(&KeyType::Aes).await?; // Vendor-agnostic
        assert_eq!(key.len(), 32);

        let chacha_key = provider.generate_key_material(&KeyType::ChaCha20).await?;
        assert_eq!(chacha_key.len(), 32);
        Ok(())
    }

    #[tokio::test]
    async fn test_encryption_decryption() -> Result<(), BearDogError> {
        let provider = SoftwareHsmCryptoProvider::new().await?;
        let key = provider.generate_key_material(&KeyType::ChaCha20).await?;
        let plaintext = b"Rust crypto test";

        let ciphertext = provider.encrypt(&key, plaintext).await?;
        let decrypted = provider.decrypt(&key, &ciphertext).await?;

        assert_eq!(plaintext.to_vec(), decrypted);
        Ok(())
    }

    #[tokio::test]
    async fn test_signing_verification() -> Result<(), BearDogError> {
        let provider = SoftwareHsmCryptoProvider::new().await?;
        let key_material = provider.generate_key_material(&KeyType::Ed25519).await?;

        // Convert Vec<u8> to [u8; 32] for SigningKey
        let key_array: [u8; 32] = key_material.clone().try_into().map_err(|_| {
            BearDogError::crypto_error("Invalid key material length for Ed25519".to_string())
        })?;
        let signing_key = SigningKey::from_bytes(&key_array);
        let verifying_key = signing_key.verifying_key();

        let data = b"Data to sign";
        let signature = provider.sign(&key_material, data).await?;
        let is_valid = provider
            .verify(verifying_key.as_bytes(), data, &signature)
            .await?;

        assert!(is_valid);

        let different_data = b"Different data";
        let is_invalid = provider
            .verify(verifying_key.as_bytes(), different_data, &signature)
            .await?;

        assert!(!is_invalid);
        Ok(())
    }

    #[tokio::test]
    async fn test_key_derivation() -> Result<(), BearDogError> {
        let provider = SoftwareHsmCryptoProvider::new().await?;
        let root_key = b"rust_root_key_for_derivation";
        let context = b"derivation_context";

        let derived1 = provider.derive_key(root_key, context).await?;
        let derived2 = provider.derive_key(root_key, context).await?;

        assert_eq!(derived1, derived2);
        assert!(!derived1.is_empty());

        let different_context = b"different_context";
        let derived3 = provider.derive_key(root_key, different_context).await?;

        assert_ne!(derived1, derived3);
        Ok(())
    }
}
