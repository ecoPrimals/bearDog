//! # Software HSM Core Implementation
//!
//! This module provides the core Software HSM implementation including the main
//! RustSoftwareHsm struct and HsmProvider trait implementation.

use super::memory::{DefaultMemoryProtector, MemoryProtectionConfig};
use super::types::*;
use beardog_errors::{BearDogError, BearDogResult};
use crate::tunnel::hsm::types::*;
use crate::tunnel::hsm::HsmProvider;
use async_trait::async_trait;
use chrono::Utc;
use hex;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

impl RustSoftwareHsm {
    /// Create a new Rust Software HSM instance
    ///
    /// # Arguments
    /// * `config` - Software HSM configuration
    ///
    /// # Returns
    /// * `Ok(RustSoftwareHsm)` - Successfully initialized HSM
    /// * `Err(BearDogError)` - Initialization failure
    pub async fn new(config: SoftwareHsmConfig) -> BearDogResult<Self> {
        info!("🔐 Initializing Rust Software HSM");

        // Initialize crypto provider
        let crypto_provider = Self::create_crypto_provider(&config.crypto_backend).await?;

        // Create memory protector with conversion from MemoryConfig to MemoryProtectionConfig
        let memory_config = MemoryProtectionConfig {
            enable_protection: match config.memory_config.protection_level {
                MemoryProtectionLevel::None => false,
                _ => true,
            },
            clear_on_drop: config.memory_config.zero_on_free,
        };
        let memory_protector = Arc::new(DefaultMemoryProtector::new(memory_config).await?);

        // Initialize key store
        let key_store = Arc::new(RwLock::new(
            SoftwareKeyStore::new(&config.key_store_config).await?,
        ));

        // Initialize audit logger
        let audit_logger = Self::create_audit_logger().await?;

        // Initialize health monitor
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

    /// Create crypto provider based on backend type
    async fn create_crypto_provider(
        backend: &CryptoBackend,
    ) -> BearDogResult<Arc<dyn CryptoProvider>> {
        match backend {
            CryptoBackend::RustCrypto => Ok(Arc::new(RustCryptoProvider::new().await?)),
            CryptoBackend::Ring => Ok(Arc::new(RingCryptoProvider::new().await?)),
            CryptoBackend::OpenSsl => Ok(Arc::new(OpenSslCryptoProvider::new().await?)),
            CryptoBackend::Custom(name) => {
                Err(BearDogError::UnsupportedCryptoBackend {
                    backend: name.clone(),
                })
            }
        }
    }

    /// Create memory protector based on configuration
    async fn create_memory_protector(
        config: &MemoryConfig,
    ) -> BearDogResult<Arc<dyn MemoryProtector>> {
        // Convert MemoryConfig to MemoryProtectionConfig
        let memory_protection_config = MemoryProtectionConfig {
            enable_protection: match config.protection_level {
                MemoryProtectionLevel::None => false,
                _ => true,
            },
            clear_on_drop: config.zero_on_free,
        };

        Ok(Arc::new(
            DefaultMemoryProtector::new(memory_protection_config).await?,
        ))
    }

    /// Create audit logger
    async fn create_audit_logger() -> BearDogResult<Arc<dyn AuditLogger>> {
        Ok(Arc::new(DefaultAuditLogger::new().await?))
    }

    /// Generate software key
    async fn generate_software_key(&self, request: &GenerateKeyRequest) -> BearDogResult<HsmKey> {
        info!("🔑 Generating software key: {}", request.key_id);

        // Generate key material
        let key_material = self
            .crypto_provider
            .generate_key_material(&request.key_type)
            .await?;

        // Protect key material in memory
        let protected_key = self
            .memory_protector
            .protect_key_material(&key_material)
            .await?;

        // Create software key
        let software_key = SoftwareKey::new(
            request.key_id.clone(),
            request.key_type.clone(),
            protected_key,
            request.metadata.clone(),
        );

        // Store key in key store
        let key_store = self.key_store.write().await;
        key_store.store_key(&software_key).await?;

        // Log operation
        self.audit_logger
            .log_operation(&AuditLogEntry::success(
                "generate_key".to_string(),
                Some(request.key_id.clone()),
                None,
            ))
            .await?;

        // Create HSM key
        let hsm_key = HsmKey {
            id: request.key_id.clone(),
            hsm_type: "SoftwareHsm".to_string(), // Convert HsmTier to String
            key_type: request.key_type.clone(),
            metadata: request.metadata.clone(),
            key_material: KeyMaterial::Encrypted {
                encrypted_data: key_material,
                encryption_algorithm: "AES-256-GCM".to_string(),
                key_derivation_info: None,
            },
            hsm_tier: "SoftwareHsm".to_string(),
            health_status: KeyHealthStatus::Healthy,
            attestation: None, // Software HSM doesn't provide hardware attestation
            created_at: Utc::now(),
        };

        info!("✅ Software key generated successfully: {}", request.key_id);
        Ok(hsm_key)
    }

    /// Perform cryptographic operation with key
    async fn perform_crypto_operation<T>(
        &self,
        key_id: &str,
        operation: &str,
        op_fn: impl Fn(&[u8]) -> Result<T, BearDogError>,
    ) -> BearDogResult<T> {
        let start_time = std::time::Instant::now();

        // Get key from store
        let key_store = self.key_store.read().await;
        let software_key = key_store.get_key(key_id).await?;

        // Unprotect key material
        let key_material = self
            .memory_protector
            .unprotect_key_material(&software_key.key_material)
            .await?;

        // Perform operation
        let result = op_fn(&key_material);

        // Zeroize key material
        self.memory_protector
            .zeroize_key_material(&key_material)
            .await?;

        // Record performance metrics
        let duration = start_time.elapsed();
        self.health_monitor
            .record_operation(operation, duration, result.is_ok())
            .await?;

        // Log operation
        let log_result = if result.is_ok() { "success" } else { "failure" };
        self.audit_logger
            .log_operation(&AuditLogEntry::new(
                operation.to_string(),
                Some(key_id.to_string()),
                None,
                log_result.to_string(),
                HashMap::new(),
            ))
            .await?;

        result
    }

    /// Get HSM configuration
    pub fn get_config(&self) -> &SoftwareHsmConfig {
        &self.config
    }

    /// Get key store reference
    pub fn get_key_store(&self) -> &Arc<RwLock<SoftwareKeyStore>> {
        &self.key_store
    }

    /// Get health monitor reference
    pub fn get_health_monitor(&self) -> &Arc<SoftwareHealthMonitor> {
        &self.health_monitor
    }

    /// Update configuration
    pub async fn update_config(&mut self, config: SoftwareHsmConfig) -> BearDogResult<()> {
        info!("Updating Software HSM configuration");

        // Update configuration
        self.config = config;

        // Reinitialize components if needed
        // TODO: Implement configuration hot-reload

        info!("Software HSM configuration updated successfully");
        Ok(())
    }
}

#[async_trait]
impl HsmProvider for RustSoftwareHsm {
    /// Initialize the HSM connection
    async fn initialize(&self, _config: HsmConfig) -> BearDogResult<()> {
        info!("🔄 Initializing Rust Software HSM");

        // Initialize crypto provider
        self.crypto_provider.initialize().await?;

        // Initialize memory protector
        self.memory_protector.initialize().await?;

        // Initialize key store
        let key_store = self.key_store.read().await;
        key_store.initialize().await?;

        // Log initialization
        self.audit_logger
            .log_operation(&AuditLogEntry::success(
                "initialize".to_string(),
                None,
                None,
            ))
            .await?;

        info!("✅ Rust Software HSM initialized successfully");
        Ok(())
    }

    /// Generate a new key in the HSM
    async fn generate_key(&self, request: GenerateKeyRequest) -> BearDogResult<HsmKey> {
        self.generate_software_key(&request).await
    }

    /// Import an existing key into the HSM
    async fn import_key(&self, key_data: &[u8], metadata: KeyMetadata) -> BearDogResult<HsmKey> {
        info!("📥 Importing key into Software HSM: {}", metadata.key_id);

        // Protect key material
        let protected_key = self.memory_protector.protect_key_material(key_data).await?;

        // Create software key
        let software_key = SoftwareKey::new(
            metadata.key_id.clone(),
            metadata.key_type.clone(),
            protected_key,
            metadata.clone(),
        );

        // Store key
        let key_store = self.key_store.write().await;
        key_store.store_key(&software_key).await?;

        // Log operation
        self.audit_logger
            .log_operation(&AuditLogEntry::success(
                "import_key".to_string(),
                Some(metadata.key_id.clone()),
                None,
            ))
            .await?;

        // Create HSM key
        let hsm_key = HsmKey {
            id: metadata.key_id.clone(),
            hsm_type: HsmTier::SoftwareHsm {
                implementation: SoftwareHsmType::RustSoftwareHsm,
                key_storage: self.config.key_store_config.storage_type.clone(),
                encryption_at_rest: true,
                memory_protection: self.config.memory_config.protection_level.clone(),
            },
            key_type: metadata.key_type.clone(),
            attestation: None,
            created_at: Utc::now(),
            metadata,
        };

        info!("✅ Key imported successfully: {}", hsm_key.id);
        Ok(hsm_key)
    }

    /// Encrypt data using HSM key
    async fn encrypt(&self, key_id: &str, plaintext: &[u8]) -> BearDogResult<Vec<u8>> {
        debug!("🔒 Encrypting data with software key: {}", key_id);

        self.perform_crypto_operation(key_id, "encrypt", |key_material| {
            // This would be async in real implementation
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                self.crypto_provider.encrypt(key_material, plaintext).await
            })
        })
        .await
    }

    /// Decrypt data using HSM key
    async fn decrypt(&self, key_id: &str, ciphertext: &[u8]) -> BearDogResult<Vec<u8>> {
        debug!("🔓 Decrypting data with software key: {}", key_id);

        self.perform_crypto_operation(key_id, "decrypt", |key_material| {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                self.crypto_provider.decrypt(key_material, ciphertext).await
            })
        })
        .await
    }

    /// Sign data using HSM key
    async fn sign(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
        debug!("✍️ Signing data with software key: {}", key_id);

        self.perform_crypto_operation(key_id, "sign", |key_material| {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                self.crypto_provider.sign(key_material, data).await
            })
        })
        .await
    }

    /// Verify signature using HSM key
    async fn verify(&self, key_id: &str, data: &[u8], signature: &[u8]) -> BearDogResult<bool> {
        debug!("🔍 Verifying signature with software key: {}", key_id);

        self.perform_crypto_operation(key_id, "verify", |key_material| {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                self.crypto_provider
                    .verify(key_material, data, signature)
                    .await
            })
        })
        .await
    }

    /// Derive key using HSM-based KDF
    async fn derive_key(
        &self,
        master_key_id: &str,
        derivation_data: &[u8],
    ) -> BearDogResult<HsmKey> {
        info!(
            "🔑 Deriving key from software master key: {}",
            master_key_id
        );

        // Get master key
        let key_store = self.key_store.read().await;
        let master_key = key_store.get_key(master_key_id).await?;
        let master_key_material = self
            .memory_protector
            .unprotect_key_material(&master_key.key_material)
            .await?;

        // Derive new key material
        let derived_key_material = self
            .crypto_provider
            .derive_key(&master_key_material, derivation_data)
            .await?;

        // Zeroize master key material
        self.memory_protector
            .zeroize_key_material(&master_key_material)
            .await?;

        // Create derived key
        let derived_key_id = format!("{}_{}", master_key_id, hex::encode(&derivation_data[..8]));
        let derived_metadata = KeyMetadata {
            key_id: derived_key_id.clone(),
            key_type: KeyType::Aes256, // Default for derived keys
            created_at: Utc::now(),
            expires_at: None,
            usage_policy: KeyUsagePolicy::default(),
            attributes: HashMap::new(),
        };

        // Import derived key
        self.import_key(&derived_key_material, derived_metadata)
            .await
    }

    /// Get HSM information and capabilities
    async fn get_info(&self) -> BearDogResult<HsmInfo> {
        Ok(HsmInfo {
            hsm_type: HsmTier::SoftwareHsm {
                implementation: SoftwareHsmType::RustSoftwareHsm,
                key_storage: self.config.key_store_config.storage_type.clone(),
                encryption_at_rest: true,
                memory_protection: self.config.memory_config.protection_level.clone(),
            },
            vendor: "BearDog".to_string(),
            model: "Rust Software HSM".to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec![
                HsmCapability::KeyGeneration,
                HsmCapability::KeyImport,
                HsmCapability::KeyExport,
                HsmCapability::Encryption,
                HsmCapability::Decryption,
                HsmCapability::Signing,
                HsmCapability::Verification,
                HsmCapability::KeyDerivation,
                HsmCapability::SecureBackup,
                HsmCapability::SecureRestore,
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
            max_key_size: Some(4096),
            certification: None,
            tamper_resistance: TamperResistanceLevel::Software,
        })
    }

    /// List keys stored in HSM
    async fn list_keys(&self) -> BearDogResult<Vec<HsmKeyInfo>> {
        let key_store = self.key_store.read().await;
        let key_ids = key_store.list_keys().await?;

        let mut key_infos = Vec::new();
        for key_id in key_ids {
            if let Ok(key) = key_store.get_key(&key_id).await {
                key_infos.push(HsmKeyInfo {
                    metadata: key.metadata().clone(),
                    hsm_tier: "SoftwareHsm".to_string(),
                    health_status: KeyHealthStatus::Healthy,
                    performance_metrics: KeyPerformanceMetrics {
                        avg_latency_ms: 1.0,
                        ops_per_second: 1000.0,
                        error_rate: 0.0,
                        success_rate: 1.0,
                    },
                    last_accessed: Some(chrono::Utc::now()),
                    access_count: 0,
                    key_id: key.id().to_string(),
                    key_type: key.key_type().clone(),
                    hsm_type: "SoftwareHsm".to_string(),
                    created_at: key.created_at(),
                    usage_policy: key.metadata().usage_policy.clone(),
                });
            }
        }

        Ok(key_infos)
    }

    /// Delete key from HSM
    async fn delete_key(&self, key_id: &str) -> BearDogResult<()> {
        info!("🗑️ Deleting software key: {}", key_id);

        let key_store = self.key_store.write().await;
        key_store.delete_key(key_id).await?;

        // Log operation
        self.audit_logger
            .log_operation(&AuditLogEntry::success(
                "delete_key".to_string(),
                Some(key_id.to_string()),
                None,
            ))
            .await?;

        info!("✅ Software key deleted successfully: {}", key_id);
        Ok(())
    }

    /// Backup HSM state
    async fn backup(&self) -> BearDogResult<Option<Vec<u8>>> {
        info!("💾 Creating software HSM backup");

        let key_store = self.key_store.read().await;
        let backup_data = key_store.backup().await?;

        // Log operation
        self.audit_logger
            .log_operation(&AuditLogEntry::success("backup".to_string(), None, None))
            .await?;

        info!("✅ Software HSM backup created successfully");
        Ok(Some(backup_data))
    }

    /// Restore HSM state
    async fn restore(&self, backup_data: &[u8]) -> BearDogResult<()> {
        info!("📥 Restoring software HSM from backup");

        let key_store = self.key_store.read().await;
        key_store.restore(backup_data).await?;

        // Log operation
        self.audit_logger
            .log_operation(&AuditLogEntry::success("restore".to_string(), None, None))
            .await?;

        info!("✅ Software HSM restored successfully");
        Ok(())
    }

    /// Get HSM health status
    async fn health_check(&self) -> BearDogResult<HsmHealthStatus> {
        self.health_monitor.perform_health_check().await
    }
}
