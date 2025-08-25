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


/// # Software HSM Module
///
/// This module provides a comprehensive Software HSM implementation for the BearDog project.
/// It offers secure key management, cryptographic operations, and audit logging without
/// requiring hardware security modules.
/// ## Architecture
/// The Software HSM is composed of several specialized modules:
/// - **types**: Core types, traits, and data structures
/// - **core**: Main RustSoftwareHsm implementation and HsmProvider trait
/// - **keystore**: Encrypted key storage and caching
/// - **crypto_providers**: Multiple cryptographic backend implementations
/// - **memory**: Memory protection and secure key handling
/// - **audit**: Comprehensive audit logging and compliance
/// - **storage**: Various storage backend implementations
/// - **health**: System health monitoring and metrics
/// ## Features
/// ### Security Features
/// - **Memory Protection**: Secure memory allocation with guard pages and zeroization
/// - **Encrypted Storage**: AES-256-GCM encrypted key storage
/// - **Audit Logging**: Comprehensive audit trail for compliance
/// - **Key Lifecycle**: Complete key lifecycle management
/// - **Access Control**: Configurable key usage policies
/// ### Performance Features
/// - **Key Caching**: LRU cache for frequently accessed keys
/// - **Bulk Operations**: Optimized batch operations
/// - **Multiple Backends**: Choice of crypto providers (RustCrypto, Ring, OpenSSL)
/// - **Health Monitoring**: Real-time performance metrics
/// ### Storage Options
/// - **File Storage**: Encrypted file-based key storage
/// - **Database Storage**: SQL database backend support
/// - **Memory Storage**: In-memory storage for testing
/// - **Custom Storage**: Pluggable storage backend interface
/// ## Usage Example
/// ```rust,no_run
/// use crate::tunnel::hsm::software_hsm::*;
/// use *;
/// async fn example_usage() -> Result<(), Box<dyn std::error::Error>> {
///     // Create configuration
///     let config = SoftwareHsmConfig {
///         crypto_backend: CryptoBackend::Ring,
///         memory_config: MemoryConfig {
///             protection_level: MemoryProtectionLevel::High,
///             use_secure_allocator: true,
///             zero_on_free: true,
///             use_guard_pages: true,
///         },
///         key_store_config: KeyStoreConfig {
///             storage_type: KeyStorageType::EncryptedFile,
///             encryption_key_source: KeySource::Derived,
///             backup_enabled: true,
///             cache_size: 1000,
///             file_config: Some(FileStorageConfig::default()),
///             db_config: None,
///     };
///     // Initialize HSM
///     let hsm = RustSoftwareHsm::new(config).await?;
///     hsm.initialize(HsmConfig::default()).await?;
///     // Generate a key
///     let key_request = GenerateKeyRequest {
///         key_id: "my-aes-key".to_string(),
///         key_type: KeyType::Aes256,
///         usage_policy: KeyUsagePolicy::default(),
///         metadata: KeyMetadata {
///             key_id: "my-aes-key".to_string(),
///             key_type: KeyType::Aes256,
///             created_at: chrono::Utc::now(),
///             expires_at: None,
///             usage_policy: KeyUsagePolicy::default(),
///             attributes: std::collections::HashMap::new(),
///         require_user_presence: false,
///         attestation_challenge: None,
///     let key = hsm.generate_key(key_request).await?;
///     println!("Generated key: {}", key.id);
///     // Encrypt some data
///     let plaintext = b"Hello, World!";
///     let ciphertext = hsm.encrypt(&key.id, plaintext).await?;
///     println!("Encrypted {} bytes", ciphertext.len());
///     // Decrypt the data
///     let decrypted = hsm.decrypt(&key.id, &ciphertext).await?;
///     assert_eq!(plaintext, &decrypted[..]);
///     println!("Decryption successful!");
///     Ok(())
/// }
/// ```
/// ## Security Considerations
/// While this Software HSM provides strong security practices, it cannot match
/// the security guarantees of hardware-based HSMs:
/// - **No Hardware Root of Trust**: Keys are ultimately protected by software
/// - **Memory Attacks**: Sophisticated attacks may extract keys from memory
/// - **OS Vulnerabilities**: Security depends on underlying operating system
/// - **Physical Access**: Physical access to the system may compromise security
/// For maximum security in production environments, consider using hardware HSMs
/// or cloud-based HSM services for critical cryptographic operations.
/// ## Module Dependencies
/// ```text
/// software_hsm
/// ├── types (foundational types and traits)
/// ├── core (main implementation, depends on all others)
/// ├── keystore (depends on types, storage, memory)
/// ├── crypto_providers (depends on types)
/// ├── memory (depends on types)
/// ├── audit (depends on types)
/// ├── storage (depends on types)
/// └── health (depends on types)

// Import necessary types and traits
use crate::tunnel::hsm::types::{KeyStorageType, KeyType, MemoryProtectionLevel};
use beardog_errors::BearDogResult;
// Temporary type definitions (will be moved to proper location)
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


// MIGRATED: DatabaseConfig -> use beardog_types::config::UnifiedDatabaseConfig;


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


// MIGRATED: PerformanceConfig -> use beardog_types::config::UnifiedPerformanceConfig;


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
// Module declarations
/// Audit logging and compliance tracking
pub mod audit;
/// Core HSM implementation
pub mod core;
/// Cryptographic provider implementations
pub mod crypto_providers;
/// Health monitoring and diagnostics
pub mod health;
/// Keystore management and key lifecycle
pub mod keystore;
/// Memory management and protection
pub mod memory;
/// Data storage and persistence
pub mod storage;
/// Type definitions and structures
pub mod types;
// Re-export main types and functions for convenience
pub use types::*;
// Re-export commonly used items from submodules
pub use self::audit::{AuditLogEntry, AuditLogFilter, DefaultAuditLogger, PersistentAuditStorage};
pub use self::crypto_providers::{
    create_crypto_provider, get_crypto_provider_capabilities, get_supported_crypto_backends,
    get_supported_storage_backends, CryptoProviderCapabilities,
};
pub use self::health::SimpleHealthSummary;
pub use self::keystore::KeyStoreStatistics;
pub use self::memory::{create_memory_protection_stats, MemoryProtectionStats, SecureMemoryRegion};
pub use self::storage::{
    // create_storage_backend, get_storage_backend_capabilities, get_supported_storage_backends,
    MemoryStorageStatistics, // StorageBackendCapabilities, StorageScalability,
/// Software HSM module version
pub const VERSION: &str = "1.0.0";
/// Software HSM module build information
pub const BUILD_INFO: &str = concat!(
    "BearDog Software HSM v",
    env!("CARGO_PKG_VERSION"),
    " (built on unknown)"
);
/// Create a new Software HSM with default configuration
pub async fn create_default_software_hsm() -> beardog_errors::BearDogResult<RustSoftwareHsm> {
    let config = SoftwareHsmConfig {};


        implementation: format!("{:?}", SoftwareHsmType::RustSoftwareHsm),
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
        // Add missing fields for canonical modernization
        encryption_algorithm: Algorithm::Aes256Gcm,
        key_derivation_rounds: 100000,
        key_storage_path: "/tmp/beardog_software_hsm_keys".to_string(),
    };
    RustSoftwareHsm::new(config).await
/// Create a new Software HSM with file-based storage
pub async fn create_file_software_hsm() -> beardog_errors::BearDogResult<RustSoftwareHsm> {
        implementation: "RustSoftwareHsm".to_string(),
        key_storage_path: "/tmp/beardog_file_hsm_keys".to_string(),
/// Create a new Software HSM with database storage}


pub async fn create_database_software_hsm() -> beardog_errors::BearDogResult<RustSoftwareHsm> {
        key_storage_path: "/tmp/beardog_database_hsm_keys".to_string(),
/// Validate Software HSM configuration}


pub fn validate_config(config: &SoftwareHsmConfig) -> beardog_errors::BearDogResult<()> {
    // Validate cache size
    if config.key_store_config.cache_size == 0 {
        return Err(beardog_errors::BearDogError::configuration("Cache size must be greater than 0".to_string(),
        ));
    }
    // Validate memory protection level compatibility
    if matches!(
        config.memory_config.protection_level,
        MemoryProtectionLevel::Maximum
    ) {
        // Maximum protection level is supported
    Ok(())
/// Get Software HSM capabilities summary
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
/// Software HSM capabilities summary
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SoftwareHsmCapabilities {
    /// List of key types supported by the HSM
    pub supported_key_types: Vec<KeyType>,
    /// List of cryptographic algorithms supported by the HSM
    pub supported_algorithms: Vec<Algorithm>,
    /// List of cryptographic backends supported by the HSM
    pub supported_crypto_backends: Vec<CryptoBackend>,
    /// List of storage backends supported by the HSM
    pub supported_storage_backends: Vec<KeyStorageType>,
    /// Maximum key size in bits supported by the HSM
    pub max_key_size: u32,
    /// Whether the HSM supports key generation
    pub supports_key_generation: bool,
    /// Whether the HSM supports key import
    pub supports_key_import: bool,
    /// Whether the HSM supports key export
    pub supports_key_export: bool,
    /// Whether the HSM supports key derivation
    pub supports_key_derivation: bool,
    /// Whether the HSM supports backup operations
    pub supports_backup: bool,
    /// Whether the HSM supports restore operations
    pub supports_restore: bool,
    /// Whether the HSM supports audit logging
    pub supports_audit_logging: bool,
    /// Whether the HSM supports health monitoring
    pub supports_health_monitoring: bool,
    /// Whether memory protection features are available
    pub memory_protection_available: bool,
    /// Whether the HSM is hardware-backed
    pub hardware_backed: bool,
#[cfg(test)]
mod tests {
    use super::*;
    use *;
// CANONICAL IMPORT: use beardog_types::config::UnifiedPerformanceConfig;
// CANONICAL IMPORT: use beardog_types::config::UnifiedDatabaseConfig;
    #[tokio::test]
    async fn test_create_default_software_hsm() -> beardog_errors::BearDogResult<()> {
        let hsm = create_default_software_hsm().await;
        assert!(hsm.is_ok());
        Ok(())}


    async fn test_key_store_creation() -> beardog_errors::BearDogResult<()> {
        let config = KeyStoreConfig {
            storage_type: KeyStorageType::InMemory,
            backup_enabled: false,
            cache_size: 1024,
            db_config: None,
        };
        let key_store = SoftwareKeyStore::new(&config).await;
        assert!(key_store.is_ok());
    #[test]
    fn test_get_capabilities_summary() -> beardog_errors::BearDogResult<()> {
        let capabilities = get_capabilities_summary();
        assert!(!capabilities.hardware_backed);
        assert!(capabilities.supports_key_generation);
        assert!(capabilities.supports_audit_logging);
        assert!(capabilities.memory_protection_available);
    #[allow(clippy::const_is_empty)]}


    fn test_version_info() -> beardog_errors::BearDogResult<()> {
        assert!(!VERSION.is_empty());
        assert!(!BUILD_INFO.is_empty());
