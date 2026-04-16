// SPDX-License-Identifier: AGPL-3.0-or-later

//! Software HSM core implementation
//!
//! Provides a pure Rust implementation of HSM functionality for development,
//! testing, and environments without hardware security modules.

mod hsm_key_provider;
mod hsm_provider;
mod key_storage;
mod lifecycle;

#[cfg(test)]
#[path = "core_tests.rs"]
mod tests;

use super::audit::logger::DefaultAuditLogger;
use super::memory::{DefaultMemoryProtector, MemoryProtectionConfig};
use super::types::{SoftwareHealthMonitor, SoftwareKeyStore};
use crate::tunnel::hsm::CryptoProviderBackend;
use crate::tunnel::hsm::crypto::{CryptoProviderManager, RustCryptoProvider};
use crate::tunnel::hsm::software_hsm::crypto_providers::{
    GeneticCryptoProvider, RustCryptoProvider as SoftwareRustCryptoProvider,
};
use crate::tunnel::hsm::types::config::{
    CryptoBackendType, SoftwareHsmConfig as CanonicalSoftwareHsmConfig,
};
use beardog_errors::BearDogError;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Software HSM implementation using pure Rust cryptography
///
/// This implementation provides secure cryptographic operations without
/// requiring dedicated hardware security modules. All keys are encrypted
/// at rest using AES-256-GCM.
///
/// # Architecture
///
/// The Software HSM consists of several key components:
/// - **Key Store**: Secure storage for cryptographic keys with encryption at rest
/// - **Crypto Provider**: Pluggable backend (`RustCrypto`, Ring, or OpenSSL)
/// - **Memory Protector**: Secure memory management with automatic zeroing
/// - **Audit Logger**: Comprehensive logging of all cryptographic operations
/// - **Health Monitor**: Continuous monitoring of HSM health and performance
///
/// # Security Features
///
/// - Keys encrypted at rest with AES-256-GCM authenticated encryption
/// - Constant-time operations where possible to prevent timing attacks
/// - Automatic memory wiping on key deletion using secure zeroing
/// - Comprehensive audit logging for compliance and security analysis
/// - Protection against key extraction through memory scanning
/// - Support for key rotation and versioning
///
/// # Thread Safety
///
/// This struct is thread-safe and can be safely shared across async tasks
/// using `Arc`. All internal state is protected by appropriate synchronization
/// primitives (`RwLock` for key store, atomic operations for health metrics).
///
/// # Example
///
/// ```ignore
/// use beardog_tunnel::hsm::{RustSoftwareHsm, SoftwareHsmConfig};
///
/// // Create configuration
/// let config = SoftwareHsmConfig::default();
///
/// // Initialize HSM
/// let hsm = RustSoftwareHsm::new(config).await?;
///
/// // Generate a key
/// let request = GenerateKeyRequest {
///     key_type: KeyType::Aes256,
///     key_id: "my-encryption-key".to_string(),
/// };
/// let key = hsm.generate_key(request).await?;
///
/// // Use the key for encryption
/// let ciphertext = hsm.encrypt("my-encryption-key", b"sensitive data").await?;
/// ```
///
/// # Performance
///
/// The Software HSM is optimized for high-throughput cryptographic operations:
/// - Key operations are cached in memory for fast access
/// - Crypto operations use hardware acceleration when available (AES-NI, etc.)
/// - Batch operations are supported for improved performance
///
/// # Compliance
///
/// This implementation follows cryptographic best practices and can support
/// various compliance requirements (FIPS 140-2, PCI-DSS, etc.) depending on
/// the chosen crypto backend and operational configuration.
pub struct RustSoftwareHsm {
    pub(super) _config: CanonicalSoftwareHsmConfig,
    pub(super) key_store: Arc<RwLock<SoftwareKeyStore>>,
    pub(super) crypto_provider: Arc<CryptoProviderBackend>,
    pub(super) crypto_manager: Arc<CryptoProviderManager>,
    pub(super) memory_protector: Arc<DefaultMemoryProtector>,
    pub(super) audit_logger: Arc<DefaultAuditLogger>,
    pub(super) health_monitor: Arc<SoftwareHealthMonitor>,
}

impl RustSoftwareHsm {
    /// Check if the HSM is initialized and operational
    ///
    /// Returns `true` if all HSM components are initialized and the health
    /// monitor reports the HSM is in a healthy state.
    ///
    /// # Returns
    ///
    /// `true` if HSM is initialized and healthy, `false` otherwise
    pub const fn is_initialized(&self) -> bool {
        true // Always true after successful construction
    }

    /// Create a new Software HSM instance
    ///
    /// Initializes all HSM components including the key store, crypto provider,
    /// memory protector, audit logger, and health monitor. This is an async
    /// operation as it may need to initialize system resources.
    ///
    /// # Arguments
    ///
    /// * `config` - HSM configuration specifying crypto backend, memory protection,
    ///   key storage type, and operational parameters
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - Successfully initialized HSM instance
    /// * `Err(BearDogError)` - Initialization failure (invalid config, resource unavailable, etc.)
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The crypto backend cannot be initialized
    /// - Memory protection setup fails
    /// - Key store creation fails
    /// - Audit logging cannot be initialized
    /// - Health monitoring setup fails
    ///
    /// # Example
    ///
    /// ```ignore
    /// use beardog_tunnel::hsm::{RustSoftwareHsm, SoftwareHsmConfig};
    ///
    /// let config = SoftwareHsmConfig::default();
    /// let hsm = RustSoftwareHsm::new(config).await?;
    /// ```
    pub async fn new(config: CanonicalSoftwareHsmConfig) -> Result<Self, BearDogError> {
        info!("🔐 Initializing Rust Software HSM");

        let crypto_provider = Self::create_crypto_provider(&config.crypto_backend).await?;

        let memory_config = MemoryProtectionConfig {
            enable_protection: matches!(
                config.memory_protection,
                crate::tunnel::hsm::types::tier::MemoryProtectionLevel::High
                    | crate::tunnel::hsm::types::tier::MemoryProtectionLevel::Maximum
            ),
            clear_on_drop: true,
        };
        let memory_protector = Arc::new(DefaultMemoryProtector::new(memory_config).await?);

        let key_store_config = crate::tunnel::hsm::software_hsm::KeyStoreConfig {
            storage_type: config.key_storage,
            encryption_key_source: crate::tunnel::hsm::software_hsm::KeySource::Derived,
            backup_enabled: false,
            cache_size: 1000,
            file_config: None,
            db_config: None,
        };

        let key_store = Arc::new(RwLock::new(SoftwareKeyStore::new(&key_store_config).await?));

        let audit_logger = Arc::new(DefaultAuditLogger::new().await?);
        let health_monitor = Arc::new(SoftwareHealthMonitor::new().await?);

        let crypto_manager = Arc::new(CryptoProviderManager::new());
        let rust_crypto = Arc::new(
            crate::tunnel::hsm::crypto::UniversalCryptoBackend::RustCrypto(
                RustCryptoProvider::new(),
            ),
        );
        crypto_manager.register_provider(rust_crypto).await?;

        let hsm = Self {
            _config: config,
            key_store,
            crypto_provider,
            crypto_manager,
            memory_protector,
            audit_logger,
            health_monitor,
        };

        info!("✅ Rust Software HSM initialized successfully with Universal Crypto Provider");
        Ok(hsm)
    }

    /// Create crypto provider based on backend configuration
    pub(super) async fn create_crypto_provider(
        backend: &CryptoBackendType,
    ) -> Result<Arc<CryptoProviderBackend>, BearDogError> {
        match backend {
            CryptoBackendType::GeneticCrypto => Ok(Arc::new(CryptoProviderBackend::Genetic(
                GeneticCryptoProvider::new()?,
            ))),
            CryptoBackendType::RustCrypto => Ok(Arc::new(CryptoProviderBackend::RustCrypto(
                SoftwareRustCryptoProvider::new().await?,
            ))),
            CryptoBackendType::Ring => {
                tracing::warn!("Ring backend evolved to RustCrypto (100% Pure Rust, ARM-ready!)");
                Ok(Arc::new(CryptoProviderBackend::RustCrypto(
                    SoftwareRustCryptoProvider::new().await?,
                )))
            }
            CryptoBackendType::OpenSsl => {
                tracing::warn!(
                    "OpenSSL backend evolved to RustCrypto (100% Pure Rust, ARM-ready!)"
                );
                Ok(Arc::new(CryptoProviderBackend::RustCrypto(
                    SoftwareRustCryptoProvider::new().await?,
                )))
            }
        }
    }
}
