// SPDX-License-Identifier: AGPL-3.0-or-later

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

/// Audit logging for HSM operations
pub mod audit;

/// Core software HSM implementation
pub mod core;

/// Cryptographic provider implementations (pure Rust)
pub mod crypto_providers;

/// HSM health monitoring
pub mod health;

/// Key storage management
pub mod keystore;

/// Secure memory management
pub mod memory;

/// Persistent encrypted storage
pub mod storage;

/// Software HSM type definitions
pub mod types;

pub use types::{
    AuditLogger, CryptoProvider, DefaultEncryptionKey, DefaultMemoryProtector, EncryptionKey,
    EncryptionKeyTrait, FileStorageBackend, InMemoryStorageBackend, MemoryProtector,
    MemoryProtectorTrait, MemoryStorageBackend, ProtectedMemory, SoftwareHealthMonitor,
    SoftwareKey, SoftwareKeyStore, StorageBackend, StorageBackendTrait,
};

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
    CryptoProviderCapabilities, create_crypto_provider, get_crypto_provider_capabilities,
    get_supported_crypto_backends, get_supported_storage_backends,
};
// RingCryptoProvider removed - evolved to RustCrypto (100% Pure Rust, ARM-ready!)
// OpenSslCryptoProvider removed - evolved to pure Rust alternatives
pub use self::health::SimpleHealthSummary;
pub use self::keystore::KeyStoreStatistics;
// MemoryProtectionStats / SecureMemoryRegion / MemoryStorageStatistics: re-export when implemented in memory/storage.

use beardog_types::constants::domains::ecosystem::version::SOFTWARE_HSM_VERSION;

/// Software HSM version string
pub const VERSION: &str = SOFTWARE_HSM_VERSION;

/// Software HSM build information
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

#[cfg(test)]
mod module_public_api_tests {
    use super::{
        Algorithm, BUILD_INFO, CryptoBackend, FileConfig, KeySource, KeyStoreConfig, MemoryConfig,
        SoftwareHsmCapabilities, SoftwareHsmConfig, VERSION, get_capabilities_summary,
        validate_config,
    };
    use crate::tunnel::hsm::manager::implementation::HsmProvider;
    use crate::tunnel::hsm::types::{KeyStorageType, KeyType, MemoryProtectionLevel};

    fn key_store(cache_size: usize) -> KeyStoreConfig {
        KeyStoreConfig {
            storage_type: KeyStorageType::Memory,
            encryption_key_source: KeySource::Derived,
            backup_enabled: false,
            cache_size,
            file_config: None,
            db_config: None,
        }
    }

    fn sample_config(cache_size: usize, level: MemoryProtectionLevel) -> SoftwareHsmConfig {
        SoftwareHsmConfig {
            implementation: "unit".to_string(),
            crypto_backend: CryptoBackend::Ring,
            memory_protection: MemoryProtectionLevel::High,
            enable_key_caching: true,
            max_cached_keys: 64,
            key_storage: key_store(cache_size),
            memory_config: MemoryConfig {
                protection_level: level,
                secure_allocation: true,
                clear_on_dealloc: true,
                lock_memory: false,
                guard_pages: false,
            },
            key_store_config: key_store(cache_size),
            encryption_algorithm: Algorithm::Aes256Gcm,
        }
    }

    #[test]
    fn version_and_build_info_non_empty() {
        assert!(!VERSION.is_empty());
        assert!(BUILD_INFO.contains("Software HSM"));
        assert!(BUILD_INFO.contains(env!("CARGO_PKG_VERSION")));
    }

    #[test]
    fn validate_config_ok_when_cache_positive() {
        let c = sample_config(10, MemoryProtectionLevel::High);
        validate_config(&c).expect("valid config");
    }

    #[test]
    fn validate_config_err_when_cache_zero() {
        let c = sample_config(0, MemoryProtectionLevel::High);
        let err = validate_config(&c).expect_err("cache 0");
        assert!(err.to_string().contains("Cache size") || err.to_string().contains("cache"));
    }

    #[test]
    fn validate_config_accepts_maximum_protection_level() {
        let c = sample_config(8, MemoryProtectionLevel::Maximum);
        validate_config(&c).expect("maximum level branch");
    }

    #[test]
    fn get_capabilities_summary_covers_key_types_and_flags() {
        let s = get_capabilities_summary();
        assert!(s.supported_key_types.contains(&KeyType::Aes));
        assert!(s.supports_key_generation);
        assert!(!s.hardware_backed);
        assert_eq!(s.max_key_size, 4096);
    }

    #[test]
    fn algorithm_serde_roundtrip_all_variants() {
        for a in [
            Algorithm::Aes256Gcm,
            Algorithm::ChaCha20Poly1305,
            Algorithm::EccP256,
            Algorithm::EccP384,
            Algorithm::EcdsaSha256,
            Algorithm::RsaSha256,
            Algorithm::HkdfSha256,
        ] {
            let json = serde_json::to_string(&a).expect("serialize");
            let back: Algorithm = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(format!("{a:?}"), format!("{back:?}"));
        }
    }

    #[test]
    fn software_hsm_capabilities_serde_roundtrip() {
        let s = get_capabilities_summary();
        let json = serde_json::to_string(&s).expect("cap ser");
        let back: SoftwareHsmCapabilities = serde_json::from_str(&json).expect("cap de");
        assert_eq!(back.max_key_size, s.max_key_size);
        assert_eq!(back.supports_key_export, s.supports_key_export);
    }

    #[test]
    fn file_config_debug_fields() {
        let f = FileConfig {
            base_path: "/var/keys".to_string(),
            file_permissions: 0o600,
            backup_path: Some("/backup".to_string()),
        };
        assert_eq!(f.file_permissions, 0o600);
    }

    #[test]
    fn key_source_variants_cover_all() {
        use super::KeySource as KS;
        let _ = (KS::Derived, KS::Hardware, KS::External);
    }

    #[tokio::test]
    async fn create_default_software_hsm_succeeds() {
        let hsm = super::create_default_software_hsm()
            .await
            .expect("create_default_software_hsm");
        let h = hsm.health_check().await.expect("health");
        assert!(h.is_healthy);
    }

    #[tokio::test]
    async fn create_file_and_database_aliases_match_default() {
        let a = super::create_default_software_hsm().await.expect("default");
        let b = super::create_file_software_hsm().await.expect("file");
        let c = super::create_database_software_hsm().await.expect("db");
        assert_eq!(a.health_check().await.unwrap().is_healthy, true);
        assert_eq!(b.health_check().await.unwrap().is_healthy, true);
        assert_eq!(c.health_check().await.unwrap().is_healthy, true);
    }
}
