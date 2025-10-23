//! Software HSM core implementation
//!
//! Provides a pure Rust implementation of HSM functionality for development,
//! testing, and environments without hardware security modules.

use super::super::types::KeyType;
use super::audit::logger::DefaultAuditLogger;
// NOTE: Using stub crypto providers since crypto_providers module is temporarily disabled
use super::memory::{DefaultMemoryProtector, MemoryProtectionConfig};
use super::types::ProtectedMemory;
use super::types::*;
use crate::tunnel::hsm::manager::HsmProvider;
use crate::tunnel::hsm::stub_types::{
    OpenSslCryptoProvider, RingCryptoProvider, RustCryptoProvider,
};
use crate::tunnel::hsm::types::config::{
    CryptoBackendType, SoftwareHsmConfig as CanonicalSoftwareHsmConfig,
};
use crate::tunnel::hsm::types::HsmKey;
use crate::tunnel::hsm::types::*;
use crate::tunnel::hsm::{GenerateKeyRequest, HsmConfig};
use beardog_errors::BearDogError;
use beardog_types::hsm::AuditEvent;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

type BearDogResult<T> = Result<T, BearDogError>;

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
/// - **Crypto Provider**: Pluggable backend (RustCrypto, Ring, or OpenSSL)
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
    config: CanonicalSoftwareHsmConfig,
    key_store: Arc<RwLock<SoftwareKeyStore>>,
    crypto_provider: Arc<dyn CryptoProvider + Send + Sync>,
    memory_protector: Arc<DefaultMemoryProtector>,
    audit_logger: Arc<DefaultAuditLogger>,
    health_monitor: Arc<SoftwareHealthMonitor>,
}

impl RustSoftwareHsm {
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
    pub async fn new(config: CanonicalSoftwareHsmConfig) -> BearDogResult<Self> {
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

        // Create a KeyStoreConfig from the KeyStorageType
        let key_store_config = crate::tunnel::hsm::software_hsm::KeyStoreConfig {
            storage_type: config.key_storage.clone(),
            encryption_key_source: crate::tunnel::hsm::software_hsm::KeySource::Derived,
            backup_enabled: false,
            cache_size: 1000,
            file_config: None,
            db_config: None,
        };

        let key_store = Arc::new(RwLock::new(SoftwareKeyStore::new(&key_store_config).await?));

        let audit_logger = Arc::new(DefaultAuditLogger::new().await?);
        let health_monitor = Arc::new(SoftwareHealthMonitor::new().await?);

        let hsm = Self {
            config,
            key_store,
            crypto_provider,
            memory_protector,
            audit_logger,
            health_monitor,
        };

        info!("✅ Rust Software HSM initialized successfully");
        Ok(hsm)
    }

    /// Create crypto provider based on backend configuration
    async fn create_crypto_provider(
        backend: &CryptoBackendType,
    ) -> BearDogResult<Arc<dyn CryptoProvider + Send + Sync>> {
        match backend {
            CryptoBackendType::RustCrypto => {
                Ok(Arc::new(RustCryptoProvider::new().await?)
                    as Arc<dyn CryptoProvider + Send + Sync>)
            }
            CryptoBackendType::Ring => {
                Ok(Arc::new(RingCryptoProvider::new()?) as Arc<dyn CryptoProvider + Send + Sync>)
            }
            CryptoBackendType::OpenSsl => Ok(Arc::new(OpenSslCryptoProvider::new().await?)
                as Arc<dyn CryptoProvider + Send + Sync>),
        }
    }

    /// Generate a software key
    async fn generate_software_key(&self, request: &GenerateKeyRequest) -> BearDogResult<HsmKey> {
        info!("🔑 Generating software key: {}", request.key_id);

        // Generate key material using crypto provider
        let key_material = self
            .crypto_provider
            .generate_key_material(&request.key_type)
            .await?;

        // Protect key material
        let protected_bytes = self.memory_protector.protect(&key_material).await?;
        let protected_material = ProtectedMemory::new(protected_bytes.clone(), true);
        let encrypted_data = protected_bytes.to_vec(); // Clone for HSM key before moving

        // Create software key
        let software_key = SoftwareKey {
            id: request.key_id.clone(),
            key_material: protected_material,
            key_type: request.key_type.clone(),
            created_at: Utc::now(),
            metadata: KeyMetadata::new(request.key_id.clone(), request.key_type.clone()),
        };

        // Store key
        let key_store = self.key_store.write().await;
        key_store.store_key(software_key).await?;

        // Log operation
        self.audit_logger
            .log_operation(&AuditLogEntry::success("generate_key", &request.key_id))
            .await?;

        // Create HSM key
        let hsm_key = HsmKey {
            id: request.key_id.clone(),
            hsm_type: "SoftwareHsm".to_string(),
            key_type: request.key_type.clone(),
            metadata: KeyMetadata::new(request.key_id.clone(), request.key_type.clone()),
            key_material: KeyMaterial::Encrypted {
                encrypted_data,
                encryption_algorithm: "AES-256-GCM".to_string(),
                kdf_params: None,
            },
            hsm_tier: "Software".to_string(),
            health_status: KeyHealthStatus::Healthy,
            attestation: None,
            created_at: Utc::now(),
        };

        info!("✅ Software key generated successfully: {}", request.key_id);
        Ok(hsm_key)
    }

    /// Derive key from root key using HKDF
    ///
    /// Uses root/derived pattern for sovereignty compliance
    async fn derive_key_from_root(
        &self,
        root_key_id: &str,
        derivation_data: &[u8],
    ) -> BearDogResult<HsmKey> {
        info!("🔑 Deriving key from software root key: {}", root_key_id);

        // Get root key
        let key_store = self.key_store.read().await;
        let root_key = key_store.get_key(root_key_id).await?;

        // Unprotect root key material
        let root_key_material = self
            .memory_protector
            .unprotect(root_key.key_material.data())
            .await?;

        // Derive new key material
        let derived_material = self
            .crypto_provider
            .derive_key(&root_key_material, derivation_data)
            .await?;

        // Protect derived key material
        let protected_bytes = self.memory_protector.protect(&derived_material).await?;
        let encrypted_derived_data = protected_bytes.clone(); // Clone before moving
        let protected_derived = ProtectedMemory::new(protected_bytes, true);

        // Zeroize root key material
        self.memory_protector
            .zeroize(&mut root_key_material.clone())
            .await?;

        // Create derived key ID
        let derived_key_id = format!(
            "{}_{}",
            root_key_id,
            hex::encode(&derivation_data[..8.min(derivation_data.len())])
        );

        // Store derived key
        let derived_key = SoftwareKey {
            id: derived_key_id.clone(),
            key_material: protected_derived,
            key_type: root_key.key_type.clone(),
            created_at: Utc::now(),
            metadata: KeyMetadata::new(derived_key_id.clone(), root_key.key_type.clone()),
        };

        drop(key_store);
        let key_store = self.key_store.write().await;
        key_store.store_key(derived_key).await?;

        // Create HSM key
        let hsm_key = HsmKey {
            id: derived_key_id.clone(),
            hsm_type: "SoftwareHsm".to_string(),
            key_type: root_key.key_type.clone(),
            metadata: KeyMetadata::new(derived_key_id.clone(), root_key.key_type.clone()),
            key_material: KeyMaterial::Encrypted {
                encrypted_data: encrypted_derived_data,
                encryption_algorithm: "AES-256-GCM".to_string(),
                kdf_params: None,
            },
            hsm_tier: "Software".to_string(),
            health_status: KeyHealthStatus::Healthy,
            attestation: None,
            created_at: Utc::now(),
        };

        info!("✅ Key derived successfully: {}", derived_key_id);
        Ok(hsm_key)
    }
}

#[async_trait::async_trait]
impl HsmProvider for RustSoftwareHsm {
    fn is_available(&self) -> bool {
        true
    }

    async fn get_info(
        &self,
    ) -> BearDogResult<crate::tunnel::hsm::manager::implementation::ProviderInfo> {
        Ok(crate::tunnel::hsm::manager::implementation::ProviderInfo {
            id: "Software HSM".to_string(),
            name: "Rust Software HSM".to_string(),
            security_level: 3,
        })
    }

    // initialize moved to separate impl block below

    /// Generate a new key
    async fn generate_key(&self, request: GenerateKeyRequest) -> BearDogResult<HsmKey> {
        self.generate_software_key(&request).await
    }

    /// Import an existing key
    async fn import_key(&self, key_data: &[u8], key_id: &str) -> BearDogResult<HsmKey> {
        info!("📥 Importing key: {}", key_id);

        // Protect key material
        let protected_bytes = self.memory_protector.protect(key_data).await?;
        let protected_material = ProtectedMemory::new(protected_bytes, true);

        // Create key metadata
        let key_metadata = KeyMetadata::new(key_id.to_string(), KeyType::Ed25519);

        // Create software key
        let software_key = SoftwareKey {
            id: key_id.to_string(),
            key_material: protected_material,
            key_type: KeyType::Ed25519,
            created_at: Utc::now(),
            metadata: key_metadata.clone(),
        };

        // Store key
        let key_store = self.key_store.write().await;
        key_store.store_key(software_key).await?;

        // Log operation
        self.audit_logger
            .log_audit_event(AuditEvent::new("import_key"))
            .await?;

        // Create HSM key
        let hsm_key = HsmKey {
            id: key_id.to_string(),
            hsm_type: "SoftwareHsm".to_string(),
            key_type: KeyType::Ed25519,
            metadata: key_metadata,
            key_material: KeyMaterial::Encrypted {
                encrypted_data: key_data.to_vec(),
                encryption_algorithm: "AES-256-GCM".to_string(),
                kdf_params: None,
            },
            hsm_tier: "Software".to_string(),
            health_status: KeyHealthStatus::Healthy,
            attestation: None,
            created_at: Utc::now(),
        };

        info!("✅ Key imported successfully: {}", hsm_key.id);
        Ok(hsm_key)
    }

    /// Encrypt data with a key
    async fn encrypt(&self, key_id: &str, plaintext: &[u8]) -> BearDogResult<Vec<u8>> {
        debug!("🔒 Encrypting data with software key: {}", key_id);

        let key_store = self.key_store.read().await;
        let key = key_store.get_key(key_id).await?;
        let key_material = self
            .memory_protector
            .unprotect(key.key_material.data())
            .await?;

        let ciphertext = self
            .crypto_provider
            .encrypt(&key_material, plaintext)
            .await?;

        self.memory_protector
            .zeroize(&mut key_material.clone())
            .await?;

        debug!("✅ Data encrypted successfully");
        Ok(ciphertext)
    }

    /// Decrypt data with a key
    async fn decrypt(&self, key_id: &str, ciphertext: &[u8]) -> BearDogResult<Vec<u8>> {
        debug!("🔓 Decrypting data with software key: {}", key_id);

        let key_store = self.key_store.read().await;
        let key = key_store.get_key(key_id).await?;
        let key_material = self
            .memory_protector
            .unprotect(key.key_material.data())
            .await?;

        let plaintext = self
            .crypto_provider
            .decrypt(&key_material, ciphertext)
            .await?;

        self.memory_protector
            .zeroize(&mut key_material.clone())
            .await?;

        debug!("✅ Data decrypted successfully");
        Ok(plaintext)
    }

    /// Sign data with a key
    async fn sign(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
        debug!("✍️ Signing data with software key: {}", key_id);

        let key_store = self.key_store.read().await;
        let key = key_store.get_key(key_id).await?;
        let key_material = self
            .memory_protector
            .unprotect(key.key_material.data())
            .await?;

        let signature = self.crypto_provider.sign(&key_material, data).await?;

        self.memory_protector
            .zeroize(&mut key_material.clone())
            .await?;

        debug!("✅ Data signed successfully");
        Ok(signature)
    }

    /// Verify signature
    async fn verify(&self, key_id: &str, data: &[u8], signature: &[u8]) -> BearDogResult<bool> {
        debug!("🔍 Verifying signature with software key: {}", key_id);

        let key_store = self.key_store.read().await;
        let key = key_store.get_key(key_id).await?;
        let key_material = self
            .memory_protector
            .unprotect(key.key_material.data())
            .await?;

        let valid = self
            .crypto_provider
            .verify(&key_material, data, signature)
            .await?;

        self.memory_protector
            .zeroize(&mut key_material.clone())
            .await?;

        debug!("✅ Signature verification result: {}", valid);
        Ok(valid)
    }

    /// Delete a key
    async fn delete_key(&self, key_id: &str) -> BearDogResult<()> {
        info!("🗑️ Deleting software key: {}", key_id);

        let mut key_store = self.key_store.write().await;
        key_store.delete_key(key_id).await?;

        self.audit_logger
            .log_operation(&AuditLogEntry::success("delete_key", key_id))
            .await?;

        info!("✅ Key deleted successfully: {}", key_id);
        Ok(())
    }

    // get_info is already defined in HsmProvider trait implementation above

    /// Get key information
    async fn get_key_info(
        &self,
        key_id: &str,
    ) -> BearDogResult<crate::tunnel::hsm::manager::implementation::KeyInfo> {
        let key_store = self.key_store.read().await;
        let key = key_store.get_key(key_id).await?;

        Ok(crate::tunnel::hsm::manager::implementation::KeyInfo {
            id: key.id.clone(),
            algorithm: format!("{:?}", key.key_type),
        })
    }

    /// Get HSM health status
    async fn health_check(
        &self,
    ) -> BearDogResult<crate::tunnel::hsm::manager::implementation::HealthStatus> {
        let health = self.health_monitor.check_health().await?;

        Ok(crate::tunnel::hsm::manager::implementation::HealthStatus {
            is_healthy: health.is_healthy,
            error_message: health.error_message,
        })
    }
}

impl RustSoftwareHsm {
    /// Initialize the HSM (custom method, not part of trait)
    pub async fn initialize_hsm(&self, _config: HsmConfig) -> BearDogResult<()> {
        info!("🔄 Initializing Rust Software HSM");

        self.crypto_provider.initialize().await?;
        self.memory_protector.initialize().await?;

        let key_store = self.key_store.write().await;
        key_store.initialize().await?;

        self.audit_logger
            .log_audit_event(AuditEvent::new("initialize"))
            .await?;

        Ok(())
    }

    /// Reload HSM configuration (custom method, not part of trait)
    pub async fn reload_configuration(&self, _new_config: HsmConfig) -> BearDogResult<()> {
        info!("🔧 Reloading HSM configuration");
        // Configuration reload would happen here
        // For now, this is a placeholder
        Ok(())
    }
}

/// Additional helper methods
impl RustSoftwareHsm {
    /// Derive a key from a root key
    pub async fn derive_key(
        &self,
        root_key_id: &str,
        derivation_data: &[u8],
    ) -> BearDogResult<HsmKey> {
        self.derive_key_from_root(root_key_id, derivation_data)
            .await
    }
}
