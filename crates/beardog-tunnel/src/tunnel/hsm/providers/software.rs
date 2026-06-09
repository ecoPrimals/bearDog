// SPDX-License-Identifier: AGPL-3.0-or-later

//! Software HSM Provider
//!
//! Universal provider implementation for software-based HSM functionality.
//!
//! This module provides REAL cryptographic operations using the software HSM crypto providers.

use beardog_errors::BearDogError;
use beardog_types::hsm::CryptoProvider;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info};

// Import real crypto providers (100% Pure Rust!)
use crate::tunnel::hsm::CryptoProviderBackend;
use crate::tunnel::hsm::software_hsm::crypto_providers::SoftwareHsmCryptoProvider;
// RingCryptoProvider removed - evolved to RustCrypto (100% Pure Rust, ARM-ready!)
// OpenSslCryptoProvider removed - evolved to pure Rust alternatives
use crate::tunnel::hsm::types::KeyType;

/// Software Universal HSM Provider
pub struct SoftwareUniversalProvider {
    /// HSM capabilities
    capabilities: Option<SoftwareCapabilities>,
    /// Crypto provider type
    crypto_provider: CryptoProviderType,
    /// Provider metadata
    metadata: HashMap<String, String>,
    /// Real crypto provider implementation
    crypto_impl: Arc<CryptoProviderBackend>,
    /// Key storage (maps `key_id` -> `key_material`)
    keys: HashMap<String, Vec<u8>>,
}

/// Software-specific HSM capabilities
#[derive(Debug, Clone)]
pub struct SoftwareCapabilities {
    /// Supported algorithms
    pub supported_algorithms: Vec<String>,
    /// Maximum key size
    pub max_key_size: u32,
    /// Encryption support
    pub encryption_supported: bool,
    /// Signing support
    pub signing_supported: bool,
    /// Key derivation support
    pub key_derivation_supported: bool,
}

// Re-export canonical CryptoProviderType from beardog-types
// Note: Custom(String) variant removed - use provider configuration instead
pub use beardog_types::hsm::providers::CryptoProviderType;

/// Parse a provider type string into a `CryptoProviderType`
// Helper function for legacy code migration
pub fn parse_provider_type(s: &str) -> CryptoProviderType {
    match s {
        "rust-crypto" | "RustCrypto" => CryptoProviderType::Software,
        "openssl" | "OpenSsl" => CryptoProviderType::OpenSsl,
        "ring" | "Ring" => CryptoProviderType::Ring,
        _ => CryptoProviderType::Software, // Default to software for unknown
    }
}

impl SoftwareUniversalProvider {
    /// Create a new software HSM provider
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub async fn new(provider_type: CryptoProviderType) -> Result<Self, BearDogError> {
        // Create real crypto provider based on type
        let crypto_impl: Arc<CryptoProviderBackend> = match &provider_type {
            CryptoProviderType::Software => {
                let provider = SoftwareHsmCryptoProvider::new().await?;
                Arc::new(CryptoProviderBackend::RustCrypto(provider))
            }
            CryptoProviderType::OpenSsl => {
                // OpenSSL evolved to RustCrypto (100% Pure Rust sovereignty!)
                tracing::warn!(
                    "OpenSSL backend evolved to RustCrypto (100% Pure Rust, ARM-ready!)"
                );
                let provider = SoftwareHsmCryptoProvider::new().await?;
                Arc::new(CryptoProviderBackend::RustCrypto(provider))
            }
            CryptoProviderType::Ring => {
                // Ring evolved to RustCrypto (100% Pure Rust, ARM-ready!)
                tracing::warn!("Ring backend evolved to RustCrypto (100% Pure Rust, ARM-ready!)");
                let provider = SoftwareHsmCryptoProvider::new().await?;
                Arc::new(CryptoProviderBackend::RustCrypto(provider))
            }
            CryptoProviderType::Hardware => {
                return Err(BearDogError::unsupported_operation(
                    "Hardware crypto provider not yet implemented in software module".to_string(),
                ));
            }
            CryptoProviderType::CloudKms => {
                return Err(BearDogError::unsupported_operation(
                    "Cloud KMS provider not yet implemented in software module".to_string(),
                ));
            }
        };

        // Initialize the crypto provider
        crypto_impl.initialize().await?;

        let mut provider = Self {
            capabilities: None,
            crypto_provider: provider_type,
            metadata: HashMap::with_capacity(16),
            crypto_impl,
            keys: HashMap::with_capacity(64),
        };

        // Initialize metadata
        provider.initialize_metadata();

        let capabilities = provider.discover_capabilities().await?;
        provider.capabilities = Some(capabilities);

        info!("✅ Software HSM provider initialized with real crypto");
        Ok(provider)
    }

    /// Initialize provider metadata
    fn initialize_metadata(&mut self) {
        self.metadata
            .insert("platform".to_string(), std::env::consts::OS.to_string());
        self.metadata
            .insert("arch".to_string(), std::env::consts::ARCH.to_string());
        self.metadata.insert(
            "provider_type".to_string(),
            format!("{:?}", self.crypto_provider),
        );
    }

    /// Discover software HSM capabilities
    async fn discover_capabilities(&self) -> Result<SoftwareCapabilities, BearDogError> {
        let supported_algorithms = vec![
            "AES-256-GCM".to_string(),
            "ChaCha20-Poly1305".to_string(),
            "Ed25519".to_string(),
            "HMAC-SHA256".to_string(),
            "PBKDF2".to_string(),
        ];

        Ok(SoftwareCapabilities {
            supported_algorithms,
            max_key_size: 4096, // RSA-4096 or equivalent
            encryption_supported: true,
            signing_supported: true,
            key_derivation_supported: true,
        })
    }

    /// Get security level (software is level 1)
    pub const fn get_security_level(&self) -> u8 {
        1 // Software HSM
    }

    /// Get vendor information
    pub fn get_vendor_info(&self) -> VendorInfo {
        VendorInfo {
            name: "BearDog".to_string(),
            product: "Software Universal HSM".to_string(),
            version: "1.0.0".to_string(),
            metadata: self.metadata.clone(),
        }
    }

    /// Get crypto provider type
    pub const fn crypto_provider(&self) -> &CryptoProviderType {
        &self.crypto_provider
    }

    /// Get capabilities
    pub const fn capabilities(&self) -> Option<&SoftwareCapabilities> {
        self.capabilities.as_ref()
    }

    /// # Errors
    ///
    /// Returns an error if the provider is not registered.
    /// Generate a key (REAL IMPLEMENTATION)
    pub async fn generate_key(&mut self, key_id: &str, key_type: &str) -> Result<(), BearDogError> {
        info!(
            "🔑 Generating REAL cryptographic key: {} (type: {})",
            key_id, key_type
        );

        // Map string key type to KeyType enum
        let key_type_enum = match key_type {
            "AES-256" | "AES" => KeyType::Aes,
            "ChaCha20" => KeyType::ChaCha20,
            "Ed25519" => KeyType::Ed25519,
            "X25519" => KeyType::X25519,
            "ECC" | "ECDSA" => KeyType::EllipticCurve,
            "RSA" | "RSA-2048" => KeyType::Rsa,
            _ => KeyType::Generic,
        };

        // Generate REAL key material using crypto provider
        let key_material = self
            .crypto_impl
            .generate_key_material(&key_type_enum)
            .await?;

        debug!(
            "✅ Generated {} bytes of key material for key: {}",
            key_material.len(),
            key_id
        );

        // Store key material securely
        self.keys.insert(key_id.to_string(), key_material);
        self.metadata
            .insert(format!("key_{key_id}"), key_type.to_string());

        Ok(())
    }

    /// # Errors
    ///
    /// Returns an error if the provider is not registered.
    /// Encrypt data (REAL IMPLEMENTATION)
    pub async fn encrypt(&self, key_id: &str, plaintext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        info!("🔒 REAL encryption with software key: {}", key_id);

        // Retrieve key material
        let key_material = self
            .keys
            .get(key_id)
            .ok_or_else(|| BearDogError::not_found(format!("Key not found: {key_id}")))?;

        // Perform REAL encryption using crypto provider
        let ciphertext = self.crypto_impl.encrypt(key_material, plaintext).await?;

        debug!(
            "✅ Encrypted {} bytes -> {} bytes",
            plaintext.len(),
            ciphertext.len()
        );

        Ok(ciphertext)
    }

    /// # Errors
    ///
    /// Returns an error if the provider is not registered.
    /// Decrypt data (REAL IMPLEMENTATION)
    pub async fn decrypt(&self, key_id: &str, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError> {
        info!("🔓 REAL decryption with software key: {}", key_id);

        // Retrieve key material
        let key_material = self
            .keys
            .get(key_id)
            .ok_or_else(|| BearDogError::not_found(format!("Key not found: {key_id}")))?;

        // Perform REAL decryption using crypto provider
        let plaintext = self.crypto_impl.decrypt(key_material, ciphertext).await?;

        debug!(
            "✅ Decrypted {} bytes -> {} bytes",
            ciphertext.len(),
            plaintext.len()
        );

        Ok(plaintext)
    }

    /// # Errors
    ///
    /// Returns an error if the provider is not registered.
    /// Sign data (REAL IMPLEMENTATION)
    pub async fn sign(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        info!("✍️ REAL signing with software key: {}", key_id);

        // Retrieve key material
        let key_material = self
            .keys
            .get(key_id)
            .ok_or_else(|| BearDogError::not_found(format!("Key not found: {key_id}")))?;

        // Perform REAL signing using crypto provider
        let signature = self.crypto_impl.sign(key_material, data).await?;

        debug!(
            "✅ Signed {} bytes, signature: {} bytes",
            data.len(),
            signature.len()
        );

        Ok(signature)
    }

    /// # Errors
    ///
    /// Returns an error if the provider is not registered.
    /// Verify signature (REAL IMPLEMENTATION)
    pub async fn verify(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
        info!(
            "🔍 REAL signature verification with software key: {}",
            key_id
        );

        // Retrieve key material (private key)
        let key_material = self
            .keys
            .get(key_id)
            .ok_or_else(|| BearDogError::not_found(format!("Key not found: {key_id}")))?;

        // For Ed25519: derive public key from private key for verification
        // Check key type from metadata
        let key_type = self.metadata.get(&format!("key_{key_id}"));
        let public_key = if key_type == Some(&"Ed25519".to_string()) {
            // Ed25519: derive public key from private key
            use ed25519_dalek::SigningKey;
            let signing_key: [u8; 32] = key_material.clone().try_into().map_err(|_| {
                BearDogError::crypto_error("Invalid Ed25519 private key length".to_string())
            })?;
            let sk = SigningKey::from_bytes(&signing_key);
            sk.verifying_key().to_bytes().to_vec()
        } else {
            // For other key types, use the key material directly
            key_material.clone()
        };

        // Perform REAL verification using crypto provider with public key
        let valid = self
            .crypto_impl
            .verify(&public_key, data, signature)
            .await?;

        debug!(
            "✅ Signature verification result: {}",
            if valid { "VALID" } else { "INVALID" }
        );

        Ok(valid)
    }
}

/// Vendor information
#[derive(Debug, Clone)]
pub struct VendorInfo {
    /// Vendor name
    pub name: String,
    /// Product name
    pub product: String,
    /// Version string
    pub version: String,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_software_provider_creation() -> Result<(), Box<dyn std::error::Error>> {
        let provider = SoftwareUniversalProvider::new(CryptoProviderType::Software).await;
        assert!(provider.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_capabilities_detection() -> Result<(), Box<dyn std::error::Error>> {
        let provider = SoftwareUniversalProvider::new(CryptoProviderType::Software).await?;
        let caps = provider.capabilities();
        assert!(caps.is_some());

        if let Some(caps) = caps {
            assert!(caps.encryption_supported);
            assert!(caps.signing_supported);
            assert_eq!(caps.max_key_size, 4096);
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_security_level() -> Result<(), Box<dyn std::error::Error>> {
        let provider = SoftwareUniversalProvider::new(CryptoProviderType::Software).await?;
        assert_eq!(provider.get_security_level(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_vendor_info() -> Result<(), Box<dyn std::error::Error>> {
        let provider = SoftwareUniversalProvider::new(CryptoProviderType::Software).await?;

        let info = provider.get_vendor_info();
        assert_eq!(info.name, "BearDog");
        assert_eq!(info.product, "Software Universal HSM");
        assert_eq!(info.version, "1.0.0");
        assert!(!info.metadata.is_empty());
        Ok(())
    }

    #[test]
    fn test_crypto_provider_types() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(CryptoProviderType::Software, CryptoProviderType::Software);
        assert_ne!(CryptoProviderType::Software, CryptoProviderType::OpenSsl);

        // Test variant coverage
        let _hardware = CryptoProviderType::Hardware;
        let _cloud = CryptoProviderType::CloudKms;
        Ok(())
    }

    #[tokio::test]
    async fn test_key_generation() -> Result<(), Box<dyn std::error::Error>> {
        let mut provider = SoftwareUniversalProvider::new(CryptoProviderType::Software).await?;

        let result = provider.generate_key("test-key", "AES-256").await;
        assert!(result.is_ok(), "Key generation should succeed for AES-256");
        Ok(())
    }

    #[tokio::test]
    async fn test_encryption_decryption() -> Result<(), Box<dyn std::error::Error>> {
        let mut provider = SoftwareUniversalProvider::new(CryptoProviderType::Software).await?;

        provider.generate_key("test-key", "AES-256").await?;

        let plaintext = b"Hello, BearDog!";
        let ciphertext = provider.encrypt("test-key", plaintext).await?;
        let decrypted = provider.decrypt("test-key", &ciphertext).await?;

        assert_eq!(plaintext, &decrypted[..]);
        Ok(())
    }

    #[tokio::test]
    async fn test_signing_verification() -> Result<(), Box<dyn std::error::Error>> {
        let mut provider = SoftwareUniversalProvider::new(CryptoProviderType::Software).await?;

        provider.generate_key("test-key", "Ed25519").await?;

        let data = b"Message to sign";
        let signature = provider.sign("test-key", data).await?;
        let valid = provider.verify("test-key", data, &signature).await?;

        assert!(valid);
        Ok(())
    }

    #[tokio::test]
    async fn test_supported_algorithms() -> Result<(), Box<dyn std::error::Error>> {
        let provider = SoftwareUniversalProvider::new(CryptoProviderType::Software).await?;

        if let Some(caps) = provider.capabilities() {
            assert!(
                caps.supported_algorithms
                    .contains(&"AES-256-GCM".to_string())
            );
            assert!(caps.supported_algorithms.contains(&"Ed25519".to_string()));
            assert!(caps.supported_algorithms.len() >= 5);
        }
        Ok(())
    }
}
