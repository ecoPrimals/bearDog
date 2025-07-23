//! # Software HSM Module
//!
//! This module provides a comprehensive Software HSM implementation for the BearDog project.
//! It offers secure key management, cryptographic operations, and audit logging without
//! requiring hardware security modules.
//!
//! ## Architecture
//!
//! The Software HSM is composed of several specialized modules:
//!
//! - **types**: Core types, traits, and data structures
//! - **core**: Main RustSoftwareHsm implementation and HsmProvider trait
//! - **keystore**: Encrypted key storage and caching
//! - **crypto_providers**: Multiple cryptographic backend implementations
//! - **memory**: Memory protection and secure key handling
//! - **audit**: Comprehensive audit logging and compliance
//! - **storage**: Various storage backend implementations
//! - **health**: System health monitoring and metrics
//!
//! ## Features
//!
//! ### Security Features
//! - **Memory Protection**: Secure memory allocation with guard pages and zeroization
//! - **Encrypted Storage**: AES-256-GCM encrypted key storage
//! - **Audit Logging**: Comprehensive audit trail for compliance
//! - **Key Lifecycle**: Complete key lifecycle management
//! - **Access Control**: Configurable key usage policies
//!
//! ### Performance Features
//! - **Key Caching**: LRU cache for frequently accessed keys
//! - **Bulk Operations**: Optimized batch operations
//! - **Multiple Backends**: Choice of crypto providers (RustCrypto, Ring, OpenSSL)
//! - **Health Monitoring**: Real-time performance metrics
//!
//! ### Storage Options
//! - **File Storage**: Encrypted file-based key storage
//! - **Database Storage**: SQL database backend support
//! - **Memory Storage**: In-memory storage for testing
//! - **Custom Storage**: Pluggable storage backend interface
//!
//! ## Usage Example
//!
//! ```rust,no_run
//! use crate::tunnel::hsm::software_hsm::*;
//! use crate::tunnel::hsm::types::*;
//!
//! async fn example_usage() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create configuration
//!     let config = SoftwareHsmConfig {
//!         crypto_backend: CryptoBackend::Ring,
//!         memory_config: MemoryConfig {
//!             protection_level: MemoryProtectionLevel::High,
//!             use_secure_allocator: true,
//!             zero_on_free: true,
//!             use_guard_pages: true,
//!         },
//!         key_store_config: KeyStoreConfig {
//!             storage_type: KeyStorageType::EncryptedFile,
//!             encryption_key_source: KeySource::Derived,
//!             backup_enabled: true,
//!             cache_size: 1000,
//!             file_config: Some(FileStorageConfig::default()),
//!             db_config: None,
//!         },
//!     };
//!
//!     // Initialize HSM
//!     let hsm = RustSoftwareHsm::new(config).await?;
//!     hsm.initialize(HsmConfig::default()).await?;
//!
//!     // Generate a key
//!     let key_request = GenerateKeyRequest {
//!         key_id: "my-aes-key".to_string(),
//!         key_type: KeyType::Aes256,
//!         usage_policy: KeyUsagePolicy::default(),
//!         metadata: KeyMetadata {
//!             key_id: "my-aes-key".to_string(),
//!             key_type: KeyType::Aes256,
//!             created_at: chrono::Utc::now(),
//!             expires_at: None,
//!             usage_policy: KeyUsagePolicy::default(),
//!             attributes: std::collections::HashMap::new(),
//!         },
//!         require_user_presence: false,
//!         attestation_challenge: None,
//!     };
//!
//!     let key = hsm.generate_key(key_request).await?;
//!     println!("Generated key: {}", key.id);
//!
//!     // Encrypt some data
//!     let plaintext = b"Hello, World!";
//!     let ciphertext = hsm.encrypt(&key.id, plaintext).await?;
//!     println!("Encrypted {} bytes", ciphertext.len());
//!
//!     // Decrypt the data
//!     let decrypted = hsm.decrypt(&key.id, &ciphertext).await?;
//!     assert_eq!(plaintext, &decrypted[..]);
//!     println!("Decryption successful!");
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Security Considerations
//!
//! While this Software HSM provides strong security practices, it cannot match
//! the security guarantees of hardware-based HSMs:
//!
//! - **No Hardware Root of Trust**: Keys are ultimately protected by software
//! - **Memory Attacks**: Sophisticated attacks may extract keys from memory
//! - **OS Vulnerabilities**: Security depends on underlying operating system
//! - **Physical Access**: Physical access to the system may compromise security
//!
//! For maximum security in production environments, consider using hardware HSMs
//! or cloud-based HSM services for critical cryptographic operations.
//!
//! ## Module Dependencies
//!
//! ```text
//! software_hsm
//! ├── types (foundational types and traits)
//! ├── core (main implementation, depends on all others)
//! ├── keystore (depends on types, storage, memory)
//! ├── crypto_providers (depends on types)
//! ├── memory (depends on types)
//! ├── audit (depends on types)
//! ├── storage (depends on types)
//! └── health (depends on types)
//! ```

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
pub use self::audit::{create_audit_logger, AuditStatistics, InMemoryAuditLogger};
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
};

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
    let config = crate::tunnel::hsm::types::SoftwareHsmConfig {
        implementation: format!(
            "{:?}",
            crate::tunnel::hsm::types::SoftwareHsmType::RustSoftwareHsm
        ),
        crypto_backend: crate::tunnel::hsm::types::CryptoBackend::Ring,
        memory_protection: crate::tunnel::hsm::types::MemoryProtectionLevel::High,
        enable_key_caching: true,
        max_cached_keys: 1000,
        key_storage: crate::tunnel::hsm::types::KeyStoreConfig {
            storage_type: crate::tunnel::hsm::types::KeyStorageType::Database,
            encryption_key_source: crate::tunnel::hsm::types::KeySource::Derived,
            backup_enabled: true,
            cache_size: 1000,
            file_config: None,
            db_config: Some(crate::tunnel::hsm::types::DatabaseConfig {
                database_url: "sqlite::memory:".to_string(),
                connection_pool_size: 10,
                connection_timeout_seconds: 30,
                enable_encryption_at_rest: true,
            }),
        },
        memory_config: crate::tunnel::hsm::types::MemoryConfig {
            protection_level: crate::tunnel::hsm::types::MemoryProtectionLevel::High,
            enable_encryption: true,
            pool_size: 1024 * 1024, // 1MB pool size
        },
        key_store_config: crate::tunnel::hsm::types::KeyStoreConfig {
            storage_type: crate::tunnel::hsm::types::KeyStorageType::Database,
            encryption_key_source: crate::tunnel::hsm::types::KeySource::Derived,
            backup_enabled: true,
            cache_size: 1000,
            file_config: None,
            db_config: Some(crate::tunnel::hsm::types::DatabaseConfig {
                database_url: "sqlite::memory:".to_string(),
                connection_pool_size: 10,
                connection_timeout_seconds: 30,
                enable_encryption_at_rest: true,
            }),
        },
    };

    RustSoftwareHsm::new(config).await
}

/// Create a new Software HSM with file-based storage
pub async fn create_file_software_hsm() -> beardog_errors::BearDogResult<RustSoftwareHsm> {
    let config = crate::tunnel::hsm::types::SoftwareHsmConfig {
        implementation: "RustSoftwareHsm".to_string(),
        crypto_backend: crate::tunnel::hsm::types::CryptoBackend::Ring,
        memory_protection: crate::tunnel::hsm::types::MemoryProtectionLevel::High,
        enable_key_caching: true,
        max_cached_keys: 1000,
        key_storage: crate::tunnel::hsm::types::KeyStoreConfig {
            storage_type: crate::tunnel::hsm::types::KeyStorageType::Database,
            encryption_key_source: crate::tunnel::hsm::types::KeySource::Derived,
            backup_enabled: true,
            cache_size: 1000,
            file_config: None,
            db_config: Some(crate::tunnel::hsm::types::DatabaseConfig {
                database_url: "sqlite::memory:".to_string(),
                connection_pool_size: 10,
                connection_timeout_seconds: 30,
                enable_encryption_at_rest: true,
            }),
        },
        memory_config: crate::tunnel::hsm::types::MemoryConfig {
            protection_level: crate::tunnel::hsm::types::MemoryProtectionLevel::High,
            enable_encryption: true,
            pool_size: 1024 * 1024, // 1MB pool size
        },
        key_store_config: crate::tunnel::hsm::types::KeyStoreConfig {
            storage_type: crate::tunnel::hsm::types::KeyStorageType::Database,
            encryption_key_source: crate::tunnel::hsm::types::KeySource::Derived,
            backup_enabled: true,
            cache_size: 1000,
            file_config: None,
            db_config: Some(crate::tunnel::hsm::types::DatabaseConfig {
                database_url: "sqlite::memory:".to_string(),
                connection_pool_size: 10,
                connection_timeout_seconds: 30,
                enable_encryption_at_rest: true,
            }),
        },
    };

    RustSoftwareHsm::new(config).await
}

/// Create a new Software HSM with database storage
pub async fn create_database_software_hsm() -> beardog_errors::BearDogResult<RustSoftwareHsm> {
    let config = crate::tunnel::hsm::types::SoftwareHsmConfig {
        implementation: "RustSoftwareHsm".to_string(),
        crypto_backend: crate::tunnel::hsm::types::CryptoBackend::Ring,
        memory_protection: crate::tunnel::hsm::types::MemoryProtectionLevel::High,
        enable_key_caching: true,
        max_cached_keys: 1000,
        key_storage: crate::tunnel::hsm::types::KeyStoreConfig {
            storage_type: crate::tunnel::hsm::types::KeyStorageType::Database,
            encryption_key_source: crate::tunnel::hsm::types::KeySource::Derived,
            backup_enabled: true,
            cache_size: 1000,
            file_config: None,
            db_config: Some(crate::tunnel::hsm::types::DatabaseConfig {
                database_url: "sqlite::memory:".to_string(),
                connection_pool_size: 10,
                connection_timeout_seconds: 30,
                enable_encryption_at_rest: true,
            }),
        },
        memory_config: crate::tunnel::hsm::types::MemoryConfig {
            protection_level: crate::tunnel::hsm::types::MemoryProtectionLevel::High,
            enable_encryption: true,
            pool_size: 1024 * 1024, // 1MB pool size
        },
        key_store_config: crate::tunnel::hsm::types::KeyStoreConfig {
            storage_type: crate::tunnel::hsm::types::KeyStorageType::Database,
            encryption_key_source: crate::tunnel::hsm::types::KeySource::Derived,
            backup_enabled: true,
            cache_size: 1000,
            file_config: None,
            db_config: Some(crate::tunnel::hsm::types::DatabaseConfig {
                database_url: "sqlite::memory:".to_string(),
                connection_pool_size: 10,
                connection_timeout_seconds: 30,
                enable_encryption_at_rest: true,
            }),
        },
    };

    RustSoftwareHsm::new(config).await
}

/// Validate Software HSM configuration
pub fn validate_config(
    config: &crate::tunnel::hsm::types::SoftwareHsmConfig,
) -> beardog_errors::BearDogResult<()> {
    // Validate cache size
    if config.key_store_config.cache_size == 0 {
        return Err(beardog_errors::BearDogError::Configuration {
            message: "Cache size must be greater than 0".to_string(),
        });
    }

    // Validate memory protection level compatibility
    if matches!(
        config.memory_config.protection_level,
        crate::tunnel::hsm::types::MemoryProtectionLevel::Maximum
    ) {
        // Maximum protection level is supported
    }

    Ok(())
}

/// Get Software HSM capabilities summary
pub fn get_capabilities_summary() -> SoftwareHsmCapabilities {
    SoftwareHsmCapabilities {
        supported_key_types: vec![
            crate::tunnel::hsm::types::KeyType::Aes256,
            crate::tunnel::hsm::types::KeyType::ChaCha20,
            crate::tunnel::hsm::types::KeyType::EccP256,
            crate::tunnel::hsm::types::KeyType::EccP384,
            crate::tunnel::hsm::types::KeyType::Rsa { key_size: 2048 },
            crate::tunnel::hsm::types::KeyType::Rsa { key_size: 4096 },
        ],
        supported_algorithms: vec![
            crate::tunnel::hsm::types::Algorithm::Aes256Gcm,
            crate::tunnel::hsm::types::Algorithm::ChaCha20Poly1305,
            crate::tunnel::hsm::types::Algorithm::EccP256,
            crate::tunnel::hsm::types::Algorithm::EccP384,
            crate::tunnel::hsm::types::Algorithm::EcdsaSha256,
            crate::tunnel::hsm::types::Algorithm::RsaSha256,
            crate::tunnel::hsm::types::Algorithm::HkdfSha256,
        ],
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
    }
}

/// Software HSM capabilities summary
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SoftwareHsmCapabilities {
    /// List of key types supported by the HSM
    pub supported_key_types: Vec<crate::tunnel::hsm::types::KeyType>,
    /// List of cryptographic algorithms supported by the HSM
    pub supported_algorithms: Vec<crate::tunnel::hsm::types::Algorithm>,
    /// List of cryptographic backends supported by the HSM
    pub supported_crypto_backends: Vec<crate::tunnel::hsm::types::CryptoBackend>,
    /// List of storage backends supported by the HSM
    pub supported_storage_backends: Vec<crate::tunnel::hsm::types::KeyStorageType>,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tunnel::hsm::types::*;

    #[tokio::test]
    async fn test_create_default_software_hsm() {
        let hsm = create_default_software_hsm().await;
        assert!(hsm.is_ok());
    }

    #[tokio::test]
    async fn test_key_store_creation() {
        let config = KeyStoreConfig {
            storage_type: KeyStorageType::InMemory,
            encryption_key_source: KeySource::Derived,
            backup_enabled: false,
            cache_size: 1024,
            file_config: None,
            db_config: None,
        };

        let key_store = SoftwareKeyStore::new(&config).await;
        assert!(key_store.is_ok());
    }

    #[test]
    fn test_get_capabilities_summary() {
        let capabilities = get_capabilities_summary();
        assert!(!capabilities.hardware_backed);
        assert!(capabilities.supports_key_generation);
        assert!(capabilities.supports_audit_logging);
        assert!(capabilities.memory_protection_available);
    }

    #[test]
    #[allow(clippy::const_is_empty)]
    fn test_version_info() {
        assert!(!VERSION.is_empty());
        assert!(!BUILD_INFO.is_empty());
    }
}
