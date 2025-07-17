//! # Crypto Providers Module
//!
//! This module provides different cryptographic provider implementations for the Software HSM.
//! Each provider offers different cryptographic backends and capabilities.
//!
//! ## Available Providers
//!
//! - **RustCryptoProvider**: Uses standard Rust crypto crates (`aes-gcm`, `ed25519-dalek`)
//! - **RingCryptoProvider**: Uses the Ring cryptographic library for high performance
//! - **OpenSslCryptoProvider**: Uses OpenSSL bindings for broad compatibility
//!
//! ## Usage
//!
//! ```rust
//! use crate::tunnel::hsm::software_hsm::crypto_providers::*;
//!
//! // Create a crypto provider
//! let provider = create_crypto_provider(&CryptoBackend::Ring).await?;
//!
//! // Check capabilities
//! let capabilities = get_crypto_provider_capabilities(&CryptoBackend::Ring);
//! ```

pub mod rust_crypto;
pub mod ring_crypto;
pub mod openssl_crypto;
pub mod factory;

// Re-export all public types from factory
pub use factory::{
    create_crypto_provider,
    get_supported_crypto_backends,
    get_crypto_provider_capabilities,
    CryptoProviderCapabilities,
    is_crypto_backend_supported,
    get_recommended_crypto_backend,
    get_crypto_backend_by_name,
};

// Re-export provider implementations (they're already implemented in the types module)
pub use super::types::{
    RustCryptoProvider,
    RingCryptoProvider,
    OpenSslCryptoProvider,
    CryptoProvider,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tunnel::hsm::types::*;
    use tokio;

    #[tokio::test]
    async fn test_all_crypto_providers() {
        let backends = get_supported_crypto_backends();
        
        for backend in backends {
            let provider = create_crypto_provider(&backend).await.unwrap();
            assert!(provider.initialize().await.is_ok());
            
            let capabilities = get_crypto_provider_capabilities(&backend);
            assert!(capabilities.supports_aes);
            assert!(capabilities.supports_ecc);
        }
    }

    #[tokio::test]
    async fn test_crypto_provider_operations() {
        let provider = create_crypto_provider(&CryptoBackend::Ring).await.unwrap();
        
        // Test key generation
        let key = provider.generate_key_material(&KeyType::Aes256).await.unwrap();
        assert_eq!(key.len(), 32);
        
        // Test encryption/decryption
        let plaintext = b"Hello, World!";
        let ciphertext = provider.encrypt(&key, plaintext).await.unwrap();
        let decrypted = provider.decrypt(&key, &ciphertext).await.unwrap();
        assert_eq!(plaintext, decrypted.as_slice());
        
        // Test signing/verification
        let ecc_key = provider.generate_key_material(&KeyType::EccP256).await.unwrap();
        let signature = provider.sign(&ecc_key, plaintext).await.unwrap();
        let is_valid = provider.verify(&ecc_key, plaintext, &signature).await.unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_capabilities_comparison() {
        let rust_caps = get_crypto_provider_capabilities(&CryptoBackend::RustCrypto);
        let ring_caps = get_crypto_provider_capabilities(&CryptoBackend::Ring);
        let openssl_caps = get_crypto_provider_capabilities(&CryptoBackend::OpenSsl);
        
        // All should support basic crypto operations
        assert!(rust_caps.supports_aes);
        assert!(ring_caps.supports_aes);
        assert!(openssl_caps.supports_aes);
        
        // Only Ring and OpenSSL support hardware acceleration
        assert!(!rust_caps.supports_hardware_acceleration);
        assert!(ring_caps.supports_hardware_acceleration);
        assert!(openssl_caps.supports_hardware_acceleration);
    }

    #[test]
    fn test_backend_utilities() {
        assert!(is_crypto_backend_supported(&CryptoBackend::Ring));
        assert!(!is_crypto_backend_supported(&CryptoBackend::Custom("unknown".to_string())));
        
        assert_eq!(get_recommended_crypto_backend(), CryptoBackend::Ring);
        
        assert_eq!(get_crypto_backend_by_name("ring"), Some(CryptoBackend::Ring));
        assert_eq!(get_crypto_backend_by_name("unknown"), None);
    }
} 