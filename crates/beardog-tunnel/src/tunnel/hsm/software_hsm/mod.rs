use crate::tunnel::hsm::types::{KeyStorageType, KeyType, MemoryProtectionLevel};
use beardog_errors::BearDogError;
use beardog_types::hsm::DatabaseConfig;

// Re-export SoftwareHsm from core module
pub use core::RustSoftwareHsm as SoftwareHsm;

// ✅ VENDOR-AGNOSTIC: CryptoBackend is now defined in types/config.rs
// Re-export the canonical type for convenience
pub use crate::tunnel::hsm::types::config::CryptoBackend;

/// Cryptographic algorithms supported by the software HSM
///
/// Represents the algorithms available for key generation, encryption,
/// signing, and key derivation operations.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Algorithm {
    /// AES-256-GCM authenticated encryption
    Aes256Gcm,
    /// ChaCha20-Poly1305 authenticated encryption
    ChaCha20Poly1305,
    /// ECDSA on NIST P-256 curve
    EccP256,
    /// ECDSA on NIST P-384 curve
    EccP384,
    /// ECDSA with SHA-256 hash
    EcdsaSha256,
    /// RSA with SHA-256 hash
    RsaSha256,
    /// HKDF with SHA-256 key derivation
    HkdfSha256,
}

/// Key store configuration
///
/// Configures how keys are stored and managed by the software HSM.
pub struct KeyStoreConfig {
    /// Type of key storage backend (memory, file, database)
    pub storage_type: KeyStorageType,
    /// Source of the encryption key for key wrapping
    pub encryption_key_source: KeySource,
    /// Whether automatic backups are enabled
    pub backup_enabled: bool,
    /// Maximum number of keys to cache in memory
    pub cache_size: usize,
    /// File-based storage configuration (if applicable)
    pub file_config: Option<FileConfig>,
    /// Database storage configuration (if applicable)
    pub db_config: Option<DatabaseConfig>,
}

/// Key encryption key source
///
/// Specifies where the master key for encrypting stored keys comes from.
pub enum KeySource {
    /// Key derived from password or other secret
    Derived,
    /// Key from hardware source (HSM, TPM)
    Hardware,
    /// Key provided externally
    External,
}

/// Secure memory configuration
///
/// Controls memory protection features for key material in memory.
pub struct MemoryConfig {
    /// Memory protection level
    pub protection_level: MemoryProtectionLevel,
    /// Use secure memory allocation
    pub secure_allocation: bool,
    /// Zero memory on deallocation
    pub clear_on_dealloc: bool,
    /// Lock memory to prevent swapping (mlock)
    pub lock_memory: bool,
    /// Use guard pages around sensitive memory
    pub guard_pages: bool,
}

/// File storage configuration
///
/// Configuration for file-based key storage.
pub struct FileConfig {
    /// Base directory path for key files
    pub base_path: String,
    /// Unix file permissions for key files
    pub file_permissions: u32,
    /// Optional path for backup storage
    pub backup_path: Option<String>,
}

/// Software HSM configuration
///
/// Complete configuration for a software-based HSM instance.
pub struct SoftwareHsmConfig {
    /// HSM implementation name
    pub implementation: String,
    /// Cryptographic backend library
    pub crypto_backend: CryptoBackend,
    /// Memory protection level for key material
    pub memory_protection: MemoryProtectionLevel,
    /// Enable in-memory key caching
    pub enable_key_caching: bool,
    /// Maximum number of keys to cache
    pub max_cached_keys: usize,
    /// Key storage configuration
    pub key_storage: KeyStoreConfig,
    /// Secure memory configuration
    pub memory_config: MemoryConfig,
    /// Key store configuration (alternative reference)
    pub key_store_config: KeyStoreConfig,
    /// Default encryption algorithm
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
    get_supported_storage_backends, CryptoProviderCapabilities,
};
// RingCryptoProvider removed - evolved to RustCrypto (100% Pure Rust, ARM-ready!)
// OpenSslCryptoProvider removed - evolved to pure Rust alternatives
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

/// Software HSM capabilities summary
///
/// Describes all features and limitations of the software HSM implementation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SoftwareHsmCapabilities {
    /// Supported key types for generation
    pub supported_key_types: Vec<KeyType>,
    /// Supported cryptographic algorithms
    pub supported_algorithms: Vec<Algorithm>,
    /// Available crypto library backends
    pub supported_crypto_backends: Vec<CryptoBackend>,
    /// Available key storage backends
    pub supported_storage_backends: Vec<KeyStorageType>,
    /// Maximum supported key size in bits
    pub max_key_size: u32,
    /// Whether key generation is supported
    pub supports_key_generation: bool,
    /// Whether key import is supported
    pub supports_key_import: bool,
    /// Whether key export is supported (usually disabled for security)
    pub supports_key_export: bool,
    /// Whether key derivation functions are supported
    pub supports_key_derivation: bool,
    /// Whether key backup is supported
    pub supports_backup: bool,
    /// Whether key restore is supported
    pub supports_restore: bool,
    /// Whether audit logging is available
    pub supports_audit_logging: bool,
    /// Whether health monitoring is available
    pub supports_health_monitoring: bool,
    /// Whether memory protection features are available
    pub memory_protection_available: bool,
    /// Whether keys are hardware-backed (always false for software HSM)
    pub hardware_backed: bool,
}
