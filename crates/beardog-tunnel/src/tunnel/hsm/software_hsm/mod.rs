

use crate::tunnel::hsm::types::{KeyStorageType, KeyType, MemoryProtectionLevel};
use beardog_errors::BearDogError;

#[derive(Debug, Clone)]
pub enum CryptoBackend {
    Ring,
    OpenSsl,
    RustCrypto,
}
pub enum Algorithm {
    Aes256Gcm,
    ChaCha20Poly1305,
    EccP256,
    EccP384,
    EcdsaSha256,
    RsaSha256,
    HkdfSha256,}

pub struct KeyStoreConfig {
    pub storage_type: KeyStorageType,
    pub encryption_key_source: KeySource,
    pub backup_enabled: bool,
    pub cache_size: usize,
    pub file_config: Option<FileConfig>,
    pub db_config: Option<DatabaseConfig>,
}

pub enum KeySource {
    Derived,
    Hardware,
    External,}

pub struct MemoryConfig {
    pub protection_level: MemoryProtectionLevel,
    pub secure_allocation: bool,
    pub clear_on_dealloc: bool,
    pub lock_memory: bool,
    pub guard_pages: bool,
pub struct FileConfig {
    pub base_path: String,
    pub file_permissions: u32,
    pub backup_path: Option<String>,
}

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

pub mod audit;

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

pub async fn create_default_software_hsm() -> Result<RustSoftwareHsm, BearDogError> {
    let config = SoftwareHsmConfig {};

        implementation: format_args!("{:?}", SoftwareHsmType::RustSoftwareHsm).to_string(),
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
                connection_pool_size: 10,
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
    RustSoftwareHsm::new(config).await

pub async fn create_file_software_hsm() -> Result<RustSoftwareHsm, BearDogError> {
        implementation: "RustSoftwareHsm".to_string(),
        key_storage_path: "/tmp/beardog_file_hsm_keys".to_string(),

pub async fn create_database_software_hsm() -> Result<RustSoftwareHsm, BearDogError> {
        key_storage_path: "/tmp/beardog_database_hsm_keys".to_string(),

pub fn validate_config(config: &SoftwareHsmConfig) -> Result<(), BearDogError> {

    if config.key_store_config.cache_size == 0 {
        return Err(beardog_errors::BearDogError::configuration("Cache size must be greater than 0".to_string(),
        ));
    }

    if matches!(
        config.memory_config.protection_level,
        MemoryProtectionLevel::Maximum
    ) {

    Ok(())

pub fn get_capabilities_summary() -> SoftwareHsmCapabilities {
    SoftwareHsmCapabilities {
        supported_key_types: vec![
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
        supported_storage_backends: get_supported_storage_backends(),
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
#[cfg(test)]
mod tests {
    use super::*;
    use *;

    #[tokio::test]
    async fn test_create_default_software_hsm() -> Result<(), BearDogError> {
        let hsm = create_default_software_hsm().await;
        assert!(hsm.is_ok());
        Ok(())}

    async fn test_key_store_creation() -> Result<(), BearDogError> {
        let config = KeyStoreConfig {
            storage_type: KeyStorageType::InMemory,
            backup_enabled: false,
            cache_size: 1024,
            db_config: None,
        };
        let key_store = SoftwareKeyStore::new(&config).await;
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
