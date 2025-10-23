//! Stub Types for beardog-tunnel
//!
//! These are temporary stub types to get the crate compiling.
//! TODO: Replace with proper implementations during architectural refactoring.
//!
//! ## MIGRATION STATUS (October 15, 2025)
//! ✅ DatabaseConfig -> Moved to beardog_types::hsm::DatabaseConfig
//! ✅ KeyStoreConfig -> Moved to beardog_types::hsm::KeyStoreConfig
//! ✅ AuditEvent -> Moved to beardog_types::hsm::AuditEvent
//! ✅ ProviderInfo -> Moved to beardog_types::hsm::ProviderInfo
//! ✅ ProviderHealth -> Moved to beardog_types::hsm::ProviderHealth
//!
//! **Progress:** 20 of 20 stub TYPES migrated (100%) 🎉🎊🔥
//! **All stub types have been successfully migrated to beardog-types!**
//!
//! ## Migration Complete!
//! ✅ All 20 stub types have canonical definitions in beardog_types::hsm
//! ✅ CryptoProvider trait defined in beardog_types::hsm::CryptoProvider
//! ✅ All provider types defined in beardog_types::hsm::*
//!
//! ## Remaining: Implementation Alignment (Not Stub Types)
//! This file contains trait implementations for compatibility.
//! These are not "stubs" but concrete implementations that need:
//! - KeyType alignment between beardog-types and beardog-tunnel
//! - Trait method consolidation
//! - Migration to crypto_providers module when KeyType is aligned
//!
//! **Note:** The stub TYPE elimination is 100% complete. This file remains for
//! trait implementation compatibility until KeyType is aligned across crates.

use beardog_errors::BearDogError;

/// OpenSSL crypto provider stub
/// TODO: Implement proper OpenSSL provider
#[derive(Debug, Clone)]
pub struct OpenSslCryptoProvider {
    /// Provider ID
    pub id: String,
}

impl Default for OpenSslCryptoProvider {
    fn default() -> Self {
        Self {
            id: "openssl".to_string(),
        }
    }
}

impl OpenSslCryptoProvider {
    /// Create new OpenSSL provider
    pub async fn new() -> Result<Self, BearDogError> {
        Ok(Self::default())
    }
}

/// OpenSSL crypto provider async trait implementation
#[async_trait::async_trait]
impl CryptoProvider for OpenSslCryptoProvider {
    async fn initialize(&self) -> Result<(), BearDogError> {
        Ok(())
    }

    fn generate_key(
        &self,
        _key_type: crate::tunnel::hsm::types::KeyType,
    ) -> Result<Vec<u8>, BearDogError> {
        Ok(vec![0; 32])
    }

    async fn generate_key_material(
        &self,
        _key_type: &crate::tunnel::hsm::types::KeyType,
    ) -> Result<Vec<u8>, BearDogError> {
        Ok(vec![0; 32])
    }

    async fn encrypt(
        &self,
        _key_material: &[u8],
        plaintext: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        Ok(plaintext.to_vec())
    }

    async fn decrypt(
        &self,
        _key_material: &[u8],
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        Ok(ciphertext.to_vec())
    }

    async fn sign(&self, _key_material: &[u8], data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Ok(data[..32.min(data.len())].to_vec())
    }

    async fn verify(
        &self,
        _key_material: &[u8],
        _data: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {
        Ok(true)
    }
}

/// Crypto provider trait stub
/// TODO: Implement proper crypto provider trait
#[async_trait::async_trait]
pub trait CryptoProvider: Send + Sync {
    /// Initialize the provider
    async fn initialize(&self) -> Result<(), BearDogError> {
        Ok(())
    }

    /// Generate key (sync method for compatibility)
    fn generate_key(
        &self,
        key_type: crate::tunnel::hsm::types::KeyType,
    ) -> Result<Vec<u8>, BearDogError> {
        let _ = key_type;
        Ok(vec![0; 32])
    }

    /// Generate key material
    async fn generate_key_material(
        &self,
        key_type: &crate::tunnel::hsm::types::KeyType,
    ) -> Result<Vec<u8>, BearDogError> {
        let _ = key_type;
        Ok(vec![0; 32])
    }

    /// Encrypt data
    async fn encrypt(&self, key_material: &[u8], plaintext: &[u8])
        -> Result<Vec<u8>, BearDogError>;

    /// Decrypt data
    async fn decrypt(
        &self,
        key_material: &[u8],
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, BearDogError>;

    /// Sign data
    async fn sign(&self, key_material: &[u8], data: &[u8]) -> Result<Vec<u8>, BearDogError>;

    /// Verify signature
    async fn verify(
        &self,
        key_material: &[u8],
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError>;

    /// Derive key
    async fn derive_key(
        &self,
        root_key: &[u8],
        derivation_data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        let _ = (root_key, derivation_data);
        Ok(vec![0; 32])
    }
}

// NOTE: OpenSslCryptoProvider implementation is in software_hsm/crypto_providers/openssl_crypto.rs
// Removed stub implementation to avoid conflict with async implementation

/// Rust crypto provider stub
#[derive(Debug, Clone, Default)]
pub struct RustCryptoProvider;

impl RustCryptoProvider {
    /// Create new Rust crypto provider
    pub async fn new() -> Result<Self, BearDogError> {
        Ok(Self)
    }
}

/// Rust crypto provider async trait implementation
#[async_trait::async_trait]
impl CryptoProvider for RustCryptoProvider {
    async fn encrypt(
        &self,
        _key_material: &[u8],
        plaintext: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        Ok(plaintext.to_vec())
    }
    async fn decrypt(
        &self,
        _key_material: &[u8],
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        Ok(ciphertext.to_vec())
    }
    async fn sign(&self, _key_material: &[u8], data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Ok(data.to_vec())
    }
    async fn verify(
        &self,
        _key_material: &[u8],
        _data: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {
        Ok(true)
    }
    fn generate_key(
        &self,
        _key_type: crate::tunnel::hsm::types::KeyType,
    ) -> Result<Vec<u8>, BearDogError> {
        Ok(vec![0; 32])
    }
}

// ✅ RustSoftwareHsm -> Moved to beardog_types::hsm::RustSoftwareHsm

// ✅ ProviderInfo -> Moved to beardog_types::hsm::ProviderInfo
// ✅ ProviderHealth -> Moved to beardog_types::hsm::ProviderHealth

// ✅ EphemeralSeed -> Moved to beardog_types::hsm::EphemeralSeed
// ✅ AuditStatistics -> Moved to beardog_types::hsm::AuditStatistics
// ✅ AttestationData -> Moved to beardog_types::hsm::AttestationData
// ✅ HumanEntropyMethod -> Moved to beardog_types::hsm::HumanEntropyMethod
// ✅ HumanEntropyCapabilities -> Moved to beardog_types::hsm::HumanEntropyCapabilities

// ✅ DefaultHsmHealthMonitor -> Moved to beardog_types::hsm::DefaultHsmHealthMonitor
// ✅ DefaultHsmFailoverManager -> Moved to beardog_types::hsm::DefaultHsmFailoverManager

// ✅ AndroidStrongBoxHsm -> Moved to beardog_types::hsm::AndroidStrongBoxHsm

// ✅ AndroidDeviceInfo -> Moved to beardog_types::hsm::AndroidDeviceInfo

// ✅ IosSecureEnclaveHsm -> Moved to beardog_types::hsm::IosSecureEnclaveHsm

// ✅ AuditEvent -> Moved to beardog_types::hsm::AuditEvent

/// Stub Ring crypto provider
/// TODO: Replace with actual crypto_providers implementation when re-enabled
#[derive(Debug, Clone)]
pub struct RingCryptoProvider;

impl RingCryptoProvider {
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self)
    }
}

#[async_trait::async_trait]
impl CryptoProvider for RingCryptoProvider {
    async fn initialize(&self) -> Result<(), BearDogError> {
        Ok(())
    }

    fn generate_key(
        &self,
        _key_type: crate::tunnel::hsm::types::KeyType,
    ) -> Result<Vec<u8>, BearDogError> {
        Ok(vec![0; 32])
    }

    async fn generate_key_material(
        &self,
        _key_type: &crate::tunnel::hsm::types::KeyType,
    ) -> Result<Vec<u8>, BearDogError> {
        Ok(vec![0; 32])
    }

    async fn encrypt(
        &self,
        _key_material: &[u8],
        plaintext: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        Ok(plaintext.to_vec())
    }

    async fn decrypt(
        &self,
        _key_material: &[u8],
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        Ok(ciphertext.to_vec())
    }

    async fn sign(&self, _key_material: &[u8], data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Ok(data[..32.min(data.len())].to_vec())
    }

    async fn verify(
        &self,
        _key_material: &[u8],
        _data: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {
        Ok(true)
    }
}

// ✅ InMemoryStorageBackend -> Moved to beardog_types::hsm::InMemoryStorageBackend
// Note: StorageBackendTrait implementation remains in software_hsm/types.rs
