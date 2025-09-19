

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use crate::tunnel::hsm::types::{KeyStorageType, KeyType, MemoryProtectionLevel};
use beardog_errors::BearDogError;

#[derive(Debug, Clone)]
    /// The encryption key source value
    pub encryption_key_source: KeySource,
    /// Whether backup is enabled
    pub backup_enabled: bool,
    /// Number of cache_size
    pub cache_size: usize,
    pub file_config: Option<FileConfig>,
    pub db_config: Option<DatabaseConfig>,
}

pub enum KeySource {
    /// State indicating derived
    Derived,
    /// Represents hardware variant
    Hardware,
    External,}
    External,}
    External,}

pub struct MemoryConfig {
    /// The protection level value
    pub protection_level: MemoryProtectionLevel,
    /// Whether secure_allocation is enabled
    pub secure_allocation: bool,
    /// Whether clear_on_dealloc is enabled
    pub clear_on_dealloc: bool,
    /// Whether lock_memory is enabled
    pub lock_memory: bool,
    /// Whether guard_pages is enabled
    pub guard_pages: bool,
pub struct FileConfig {
    /// The base path value
    pub base_path: String,
    /// Number of file_permissions
    pub file_permissions: u32,
    /// Optional backup path
    pub backup_path: Option<String>,
}

pub struct SoftwareHsmConfig {
    /// The implementation value
    pub implementation: String,
    /// The crypto backend value
    pub crypto_backend: CryptoBackend,
    /// The memory protection value
    pub memory_protection: MemoryProtectionLevel,
    /// Whether enable_key_caching is enabled
    pub enable_key_caching: bool,
    /// Number of max_cached_keys
    pub max_cached_keys: usize,
    /// The key storage value
    pub key_storage: KeyStoreConfig,
    pub memory_config: MemoryConfig,
    pub key_store_config: KeyStoreConfig,
    /// The encryption algorithm value
    pub encryption_algorithm: Algorithm,

pub mod audit;
 /// Core functionality
 /// Core functionality
pub mod core;

pub mod crypto_providers;

pub mod health;

pub mod keystore;

pub mod memory;

pub mod storage;

pub mod types;

pub use types::*;

pub use self::audit::{AuditLogEntry, AuditLogFilter, DefaultAuditLogger, PersistentAuditStorage};
pub use self::crypto_providers::{
    create_crypto_provider, get_crypto_provider_capabilities, get_supported_crypto_backends,
    get_supported_storage_backends, CryptoProviderCapabilities,
};
pub use self::health::SimpleHealthSummary;
pub use self::keystore::KeyStoreStatistics;
pub use self::memory::{create_memory_protection_stats, MemoryProtectionStats, SecureMemoryRegion};
pub use self::storage::{


    MemoryStorageStatistics, // StorageBackendCapabilities, StorageScalability,


pub const VERSION: &str = "1.0.0";


pub const BUILD_INFO: &str = concat!(
    "BearDog Software HSM v",
    env!("CARGO_PKG_VERSION"),
    " (built on unknown)"
);

/// Create Default Software Hsm operation.
///
/// # Errors
/// Returns an error if the operation fails.
/// Creates default_software_hsm
pub fn create_default_software_hsm(format!("{:?}", SoftwareHsmType::RustSoftwareHsm),
        crypto_backend: CryptoBackend::Ring,
        memory_protection: MemoryProtectionLevel::High,
        enable_key_caching: true,
        max_cached_keys: 1000,
        key_storage: KeyStoreConfig {
            storage_type: KeyStorageType::Database,
            encryption_key_source: KeySource::Derived,
            backup_enabled: true,
            cache_size: 1000,
            file_config: None,
            db_config: Some(DatabaseConfig {
                database_url: "sqlite::memory:".to_string(),
                connection_timeout_seconds: 30,
                enable_encryption_at_rest: true,
            }),
        },
        memory_config: MemoryConfig {
            protection_level: MemoryProtectionLevel::High,
            enable_encryption: true,
            pool_size: 1024 * 1024, // 1MB pool size
        key_store_config: KeyStoreConfig {

        encryption_algorithm: Algorithm::Aes256Gcm,
        key_derivation_rounds: 100000,
        key_storage_path: "/tmp/beardog_software_hsm_keys".to_string(),
    };
    RustSoftwareHsm::new(config)

/// Create File Software Hsm operation.
///
/// # Errors
/// Returns an error if the operation fails.
/// Creates file_software_hsm
pub async fn create_file_software_hsm() -> Result<RustSoftwareHsm, BearDogError> {
        implementation: "RustSoftwareHsm".to_string(),
        key_storage_path: "/tmp/beardog_file_hsm_keys".to_string(),

/// Create Database Software Hsm operation.
///
/// # Errors
/// Returns an error if the operation fails.
/// Creates database_software_hsm
pub async fn create_database_software_hsm() -> Result<RustSoftwareHsm, BearDogError> {
        key_storage_path: "/tmp/beardog_database_hsm_keys".to_string(),

/// Validate Config operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Validates config
    /// Validates config
    pub fn validate_config(config: &SoftwareHsmConfig) -> Result<(), BearDogError> {

    if config.key_store_config.cache_size == 0 {
        return Err(beardog_errors::BearDogError::configuration("Cache size must be greater than 0"));
    }

    if matches!(
        config.memory_config.protection_level,
        MemoryProtectionLevel::Maximum
    ) {

    Ok(vec![
            KeyType::Aes256,
            KeyType::ChaCha20,
            KeyType::EccP256,
            KeyType::EccP384,
            KeyType::Rsa { key_size: 2048 },
            KeyType::Rsa { key_size: 4096 },
        ],
        supported_algorithms: vec![
            Algorithm::Aes256Gcm,
            Algorithm::ChaCha20Poly1305,
            Algorithm::EccP256,
            Algorithm::EccP384,
            Algorithm::EcdsaSha256,
            Algorithm::RsaSha256,
            Algorithm::HkdfSha256,
        supported_crypto_backends: get_supported_crypto_backends(),
        supported_storage_backends: get_supported_storage_backends(4096,
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SoftwareHsmCapabilities {

    /// Collection of supported key types
    pub supported_key_types: Vec<KeyType>,

    /// Collection of supported algorithms
    pub supported_algorithms: Vec<Algorithm>,

    /// Collection of supported crypto backends
    pub supported_crypto_backends: Vec<CryptoBackend>,

    /// Collection of supported storage backends
    pub supported_storage_backends: Vec<KeyStorageType>,

    /// Number of max_key_size
    pub max_key_size: u32,

    /// Whether supports_key_generation is enabled
    pub supports_key_generation: bool,

    /// Whether supports_key_import is enabled
    pub supports_key_import: bool,

    /// Whether supports_key_export is enabled
    pub supports_key_export: bool,

    /// Whether supports_key_derivation is enabled
    pub supports_key_derivation: bool,

    /// Whether supports_backup is enabled
    pub supports_backup: bool,

    /// Whether supports_restore is enabled
    pub supports_restore: bool,

    /// Whether supports_audit_logging is enabled
    pub supports_audit_logging: bool,

    /// Whether supports_health_monitoring is enabled
    pub supports_health_monitoring: bool,

    /// Whether memory_protection_available is enabled
    pub memory_protection_available: bool,

    /// Whether hardware_backed is enabled
    pub hardware_backed: bool,
#[cfg(test)]
mod tests {
    use super::*;
    use *;

    #[tokio::test]
    fn test_create_default_software_hsm(KeyStorageType::InMemory,
            backup_enabled: false,
            cache_size: 1024,
            db_config: None,
        };
        let key_store = SoftwareKeyStore::new(&config);
        assert!(key_store.is_ok());
    #[test]
    fn test_get_capabilities_summary() -> Result<(), BearDogError> {
        let capabilities = get_capabilities_summary();
        assert!(!capabilities.hardware_backed);
        assert!(capabilities.supports_key_generation);
        assert!(capabilities.supports_audit_logging);
        assert!(capabilities.memory_protection_available);
    #[allow(clippy::const_is_empty)]}


    fn test_version_info() -> Result<(), BearDogError> {
        assert!(!VERSION.is_empty());
        assert!(!BUILD_INFO.is_empty());
