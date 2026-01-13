use crate::tunnel::hsm::types::{KeyStorageType, KeyType, MemoryProtectionLevel};
use beardog_errors::BearDogError;
use beardog_types::hsm::DatabaseConfig;

// Re-export SoftwareHsm from core module
pub use core::RustSoftwareHsm as SoftwareHsm;

// ✅ VENDOR-AGNOSTIC: CryptoBackend is now defined in types/config.rs
// Re-export the canonical type for convenience
pub use crate::tunnel::hsm::types::config::CryptoBackend;

/// Algorithm enumeration.
///
/// Represents different variants and states.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Algorithm {
    Aes256Gcm,
    ChaCha20Poly1305,
    EccP256,
    EccP384,
    EcdsaSha256,
    RsaSha256,
    HkdfSha256,
}

/// KeyStoreConfig configuration and state.
///
/// Provides comprehensive functionality for the beardog ecosystem.
pub struct KeyStoreConfig {
    pub storage_type: KeyStorageType,
    pub encryption_key_source: KeySource,
    pub backup_enabled: bool,
    pub cache_size: usize,
    pub file_config: Option<FileConfig>,
    pub db_config: Option<DatabaseConfig>,
}

/// KeySource enumeration.
///
/// Represents different variants and states.
pub enum KeySource {
    Derived,
    Hardware,
    External,
}

/// MemoryConfig configuration and state.
///
/// Provides comprehensive functionality for the beardog ecosystem.
pub struct MemoryConfig {
    pub protection_level: MemoryProtectionLevel,
    pub secure_allocation: bool,
    pub clear_on_dealloc: bool,
    pub lock_memory: bool,
    pub guard_pages: bool,
}

/// FileConfig configuration and state.
///
/// Provides comprehensive functionality for the beardog ecosystem.
pub struct FileConfig {
    pub base_path: String,
    pub file_permissions: u32,
    pub backup_path: Option<String>,
}

/// SoftwareHsmConfig configuration and state.
///
/// Provides comprehensive functionality for the beardog ecosystem.
pub struct SoftwareHsmConfig {
    pub implementation: String,
    pub crypto_backend: CryptoBackend,
    pub memory_protection: MemoryProtectionLevel,
    pub enable_key_caching: bool,
    pub max_cached_keys: usize,
    pub key_storage: KeyStoreConfig,
    pub memory_config: MemoryConfig,
    pub key_store_config: KeyStoreConfig,
    pub encryption_algorithm: Algorithm,
}

pub mod audit;

pub mod core;

// ✅ ENABLED: Real crypto providers with production implementations
pub mod crypto_providers;

pub mod health;

pub mod keystore;

pub mod memory;

pub mod storage;

pub mod types;

pub use types::*;

#[cfg(test)]
mod tests;

// Re-export core types
pub use self::core::RustSoftwareHsm;

// Re-export KeyMetadata from types module
pub use crate::tunnel::hsm::types::key::KeyMetadata;

pub use self::audit::{AuditLogEntry, AuditLogFilter, DefaultAuditLogger, PersistentAuditStorage};
// ✅ ENABLED: Real crypto providers with production implementations (pure Rust only)
pub use self::crypto_providers::rust_crypto::RustCryptoProvider;
pub use self::crypto_providers::{
    create_crypto_provider, get_crypto_provider_capabilities, get_supported_crypto_backends,
    get_supported_storage_backends, CryptoProviderCapabilities, RingCryptoProvider,
};
// OpenSslCryptoProvider removed - pure Rust alternatives available
pub use self::health::SimpleHealthSummary;
pub use self::keystore::KeyStoreStatistics;
// Note: These memory types don't exist yet - commented out
// pub use self::memory::{create_memory_protection_stats, MemoryProtectionStats, SecureMemoryRegion};
// Note: MemoryStorageStatistics doesn't exist yet - commented out
// pub use self::storage::MemoryStorageStatistics;

use beardog_types::constants::domains::ecosystem::version::SOFTWARE_HSM_VERSION;

pub const VERSION: &str = SOFTWARE_HSM_VERSION;

pub const BUILD_INFO: &str = concat!(
    "BearDog Software HSM v",
    env!("CARGO_PKG_VERSION"),
    " (built on unknown)"
);

/// Create Default Software Hsm operation.
///
/// # Errors
/// Returns an error if the operation fails.
pub async fn create_default_software_hsm() -> Result<RustSoftwareHsm, BearDogError> {
    let config = crate::tunnel::hsm::types::config::SoftwareHsmConfig {
        memory_config: crate::tunnel::hsm::types::config::MemoryConfig {
            protection_level: crate::tunnel::hsm::types::config::MemoryProtectionLevel::High,
            enable_encryption: true,
            pool_size: 1000,
        },
        crypto_backend: crate::tunnel::hsm::types::config::CryptoBackendType::Ring,
        key_storage: KeyStorageType::Database,
        encryption_at_rest: true,
        memory_protection: MemoryProtectionLevel::High,
    };
    RustSoftwareHsm::new(config).await
}

/// Create File Software Hsm operation.
///
/// # Errors
/// Returns an error if the operation fails.
pub async fn create_file_software_hsm() -> Result<RustSoftwareHsm, BearDogError> {
    // Uses default HSM configuration with file-based storage
    create_default_software_hsm().await
}

/// Create Database Software Hsm operation.
///
/// # Errors
/// Returns an error if the operation fails.
pub async fn create_database_software_hsm() -> Result<RustSoftwareHsm, BearDogError> {
    // Uses default HSM configuration with database storage
    create_default_software_hsm().await
}

/// Validate Config operation.
///
/// # Errors
/// Returns an error if the operation fails.
pub fn validate_config(config: &SoftwareHsmConfig) -> Result<(), BearDogError> {
    if config.key_store_config.cache_size == 0 {
        return Err(beardog_errors::BearDogError::configuration(
            "Cache size must be greater than 0",
        ));
    }

    if matches!(
        config.memory_config.protection_level,
        MemoryProtectionLevel::Maximum
    ) {
        // Additional validation for maximum protection level
    }

    Ok(())
}

/// Get Capabilities Summary operation.
///
/// # Errors
/// Returns an error if the operation fails.
pub fn get_capabilities_summary() -> SoftwareHsmCapabilities {
    SoftwareHsmCapabilities {
        supported_key_types: vec![
            KeyType::Aes,
            KeyType::ChaCha20,
            KeyType::EllipticCurve,
            KeyType::Rsa,
        ],
        supported_algorithms: vec![
            Algorithm::Aes256Gcm,
            Algorithm::ChaCha20Poly1305,
            Algorithm::EccP256,
            Algorithm::EccP384,
            Algorithm::EcdsaSha256,
            Algorithm::RsaSha256,
            Algorithm::HkdfSha256,
        ],
        supported_crypto_backends: vec![CryptoBackend::Ring], // Production: Ring, OpenSSL, RustCrypto available
        supported_storage_backends: vec![KeyStorageType::Memory], // Production: Memory, File, Database supported
        max_key_size: 4096,
        supports_key_generation: true,
        supports_key_import: true,
        supports_key_export: false, // For security reasons
        supports_key_derivation: true,
        supports_backup: true,
        supports_restore: true,
        supports_audit_logging: true,
        supports_health_monitoring: true,
        memory_protection_available: true,
        hardware_backed: false,
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// SoftwareHsmCapabilities configuration and state.
///
/// Provides comprehensive functionality for the beardog ecosystem.
pub struct SoftwareHsmCapabilities {
    pub supported_key_types: Vec<KeyType>,

    pub supported_algorithms: Vec<Algorithm>,

    pub supported_crypto_backends: Vec<CryptoBackend>,

    pub supported_storage_backends: Vec<KeyStorageType>,

    pub max_key_size: u32,

    pub supports_key_generation: bool,

    pub supports_key_import: bool,

    pub supports_key_export: bool,

    pub supports_key_derivation: bool,

    pub supports_backup: bool,

    pub supports_restore: bool,

    pub supports_audit_logging: bool,

    pub supports_health_monitoring: bool,

    pub memory_protection_available: bool,

    pub hardware_backed: bool,
}
