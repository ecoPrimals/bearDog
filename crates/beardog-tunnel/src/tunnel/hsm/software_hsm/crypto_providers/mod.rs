// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Crypto Providers Module
///
/// This module provides different cryptographic provider implementations for the Software HSM.
/// Each provider offers different cryptographic backends and capabilities.
/// ## Available Providers
/// - **RustCryptoProvider**: Uses standard Rust crypto crates (`aes-gcm`, `ed25519-dalek`)
/// - **RingCryptoProvider**: Uses the Ring cryptographic library for high performance
/// - **OpenSslCryptoProvider**: Uses OpenSSL bindings for broad compatibility
/// ## Usage
/// ```rust
/// use crate::tunnel::hsm::software_hsm::crypto_providers::*;
/// // Create a crypto provider
/// let provider = create_crypto_provider(&CryptoBackend::Ring).await?;
/// // Check capabilities
/// let capabilities = get_crypto_provider_capabilities(&CryptoBackend::Ring);
/// ```

pub mod factory;
pub mod openssl_crypto;
pub mod ring_crypto;
pub mod rust_crypto;
// Re-export all public types from factory
pub use factory::{
    create_crypto_provider, get_crypto_backend_by_name, get_crypto_provider_capabilities,
    get_recommended_crypto_backend, get_supported_crypto_backends, get_supported_storage_backends,
    is_crypto_backend_supported, CryptoProviderCapabilities,
};
// Re-export provider implementations (they're already implemented in the types module)
pub use super::types::{CryptoProvider, OpenSslCryptoProvider, RustCryptoProvider};
pub use ring_crypto::RingCryptoProvider;
#[cfg(test)]
mod tests {};


    use super::*;
    use crate::tunnel::hsm::types::*;
    use tokio;
    #[tokio::test]
    async fn test_all_crypto_providers() -> beardog_errors::BearDogResult<()> {
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
    async fn test_crypto_provider_operations() -> beardog_errors::BearDogResult<()> {
        // Use Rust crypto provider since it's fully implemented
        let provider = create_crypto_provider(&CryptoBackend::RustCrypto)
            .await
            .map_err(|e| {
        // Test key generation
        let key = provider
            .generate_key_material(&KeyType::Aes256)
        assert_eq!(key.len(), 32);
        // Test encryption/decryption
        let plaintext = b"Hello, World!";
        let ciphertext = provider.encrypt(&key, plaintext).await.map_err(|e| {
            tracing::error!("Operation failed: {e:?}");
            beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        })?;
        let decrypted = provider.decrypt(&key, &ciphertext).await.map_err(|e| {
        assert_eq!(plaintext, decrypted.as_slice());
        // Test signing/verification
        let ecc_key = provider
            .generate_key_material(&KeyType::EccP256)
        let signature = provider.sign(&ecc_key, plaintext).await.map_err(|e| {
        let is_valid = provider
            .verify(&ecc_key, plaintext, &signature)
        assert!(is_valid);
    #[test]
    fn test_capabilities_comparison() -> beardog_errors::BearDogResult<()> {
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
        assert!(openssl_caps.supports_hardware_acceleration);}


    fn test_backend_utilities() -> beardog_errors::BearDogResult<()> {
        assert!(is_crypto_backend_supported(&CryptoBackend::Ring));
        assert!(!is_crypto_backend_supported(&CryptoBackend::Custom(
            "unknown".to_string()
        )));
        assert_eq!(get_recommended_crypto_backend(), CryptoBackend::Ring);
        assert_eq!(
            get_crypto_backend_by_name("ring"),
            Some(CryptoBackend::Ring)
        );
        assert_eq!(get_crypto_backend_by_name("unknown"), None);
}
