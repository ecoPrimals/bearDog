// SPDX-License-Identifier: AGPL-3.0-only

/// Crypto provider factory and capability detection
pub mod factory;
/// Genetic crypto provider (100% Pure Rust, zero FFI)
pub mod genetic_crypto;
// openssl_crypto removed - using pure Rust alternatives (GeneticCrypto, RustCrypto)
// pub mod ring_crypto;  // REMOVED: Has C dependencies, use RustCrypto instead (100% Pure Rust!)
/// RustCrypto-based provider (100% Pure Rust)
pub mod rust_crypto;

#[cfg(test)]
mod comprehensive_tests;

pub use factory::{
    CryptoProviderCapabilities, create_crypto_provider, get_crypto_backend_by_name,
    get_crypto_provider_capabilities, get_recommended_crypto_backend,
    get_supported_crypto_backends, get_supported_storage_backends, is_crypto_backend_supported,
};

// Use canonical CryptoProvider trait from beardog-types
pub use beardog_types::hsm::CryptoProvider;

// ✅ Export all crypto provider implementations (ordered by recommendation)
pub use genetic_crypto::GeneticCryptoProvider; // RECOMMENDED (100% Pure Rust)
// OpenSslCryptoProvider removed - pure Rust alternatives available
// pub use ring_crypto::RingCryptoProvider;  // REMOVED: C dependencies, use RustCryptoProvider instead!
pub use rust_crypto::RustCryptoProvider;
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tunnel::hsm::types::KeyType;
    use crate::tunnel::hsm::types::config::CryptoBackend;
    use beardog_errors::BearDogError;

    #[tokio::test]
    async fn test_all_crypto_providers() -> Result<(), BearDogError> {
        let backends = get_supported_crypto_backends();
        for backend in backends {
            let provider = create_crypto_provider(&backend).await.map_err(|e| {
                tracing::error!("Operation failed: {e:?}");
                beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
            })?;
            assert!(provider.initialize().await.is_ok());
            let capabilities = get_crypto_provider_capabilities(&backend);
            assert!(capabilities.supports_aes);
            assert!(capabilities.supports_ecc);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_crypto_provider_operations() -> Result<(), BearDogError> {
        let provider = create_crypto_provider(&CryptoBackend::RustCrypto)
            .await
            .map_err(|e| {
                tracing::error!("Failed to create provider: {e:?}");
                beardog_errors::BearDogError::internal(format!("Failed to create provider: {e:?}"))
            })?;

        let key = provider.generate_key_material(&KeyType::Aes).await?;
        assert_eq!(key.len(), 32);

        let plaintext = b"Hello, World!";
        let ciphertext = provider.encrypt(&key, plaintext).await.map_err(|e| {
            tracing::error!("Operation failed: {e:?}");
            beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        })?;
        let decrypted = provider.decrypt(&key, &ciphertext).await.map_err(|e| {
            tracing::error!("Decryption failed: {e:?}");
            beardog_errors::BearDogError::internal(format!("Decryption failed: {e:?}"))
        })?;
        assert_eq!(plaintext, decrypted.as_slice());

        // Ed25519 signing/verification test
        let signing_key = provider.generate_key_material(&KeyType::Ed25519).await?;
        let signature = provider.sign(&signing_key, plaintext).await.map_err(|e| {
            tracing::error!("Signing failed: {e:?}");
            beardog_errors::BearDogError::internal(format!("Signing failed: {e:?}"))
        })?;

        // Derive public key for verification (Ed25519 signing key is 32 bytes, verifying key is also 32 bytes)
        use ed25519_dalek::SigningKey;
        let key_array: [u8; 32] = signing_key.clone().try_into().map_err(|_| {
            BearDogError::crypto_error("Invalid key length for Ed25519".to_string())
        })?;
        let sk = SigningKey::from_bytes(&key_array);
        let verifying_key = sk.verifying_key();

        let is_valid = provider
            .verify(verifying_key.as_bytes(), plaintext, &signature)
            .await
            .map_err(|e| {
                tracing::error!("Verification failed: {e:?}");
                beardog_errors::BearDogError::internal(format!("Verification failed: {e:?}"))
            })?;
        assert!(is_valid, "Ed25519 signature verification should succeed");
        Ok(())
    }

    #[test]
    fn test_capabilities_comparison() -> Result<(), BearDogError> {
        let rust_caps = get_crypto_provider_capabilities(&CryptoBackend::RustCrypto);
        let genetic_caps = get_crypto_provider_capabilities(&CryptoBackend::GeneticCrypto);

        // Ring removed - 100% Pure Rust now!
        // let ring_caps = get_crypto_provider_capabilities(&CryptoBackend::Ring);
        // OpenSSL removed - 100% Pure Rust now!
        // let openssl_caps = get_crypto_provider_capabilities(&CryptoBackend::OpenSsl);

        assert!(rust_caps.supports_aes);
        assert!(genetic_caps.supports_aes);
        // assert!(ring_caps.supports_aes);
        // assert!(openssl_caps.supports_aes);

        assert!(!rust_caps.supports_hardware_acceleration);
        assert!(genetic_caps.supports_hardware_acceleration); // GeneticCrypto uses AES-NI, AVX2 CPU intrinsics
        // assert!(ring_caps.supports_hardware_acceleration);
        // assert!(openssl_caps.supports_hardware_acceleration);
        Ok(())
    }

    #[test]
    fn test_backend_utilities() -> Result<(), BearDogError> {
        // Test GeneticCrypto is supported (the new default)
        assert!(is_crypto_backend_supported(&CryptoBackend::GeneticCrypto));
        // Ring removed - 100% Pure Rust now!
        // assert!(is_crypto_backend_supported(&CryptoBackend::Ring));

        // Verify GeneticCrypto is now the recommended backend (100% Pure Rust!)
        assert_eq!(
            get_recommended_crypto_backend(),
            CryptoBackend::GeneticCrypto
        );

        // Test backend name resolution
        assert_eq!(
            get_crypto_backend_by_name("genetic"),
            Some(CryptoBackend::GeneticCrypto)
        );
        // Ring removed!
        // assert_eq!(
        //     get_crypto_backend_by_name("ring"),
        //     Some(CryptoBackend::Ring)
        // );
        assert_eq!(get_crypto_backend_by_name("unknown"), None);
        Ok(())
    }
}
