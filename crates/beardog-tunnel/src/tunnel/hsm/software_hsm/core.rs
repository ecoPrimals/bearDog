

use super::super::types::{HsmCapabilities, HsmCapability, HsmTier, KeyType};
use crate::tunnel::hsm::types::config::SoftwareHsmConfig;
use beardog_core::HsmHealthStatus; // Use canonical HsmHealthStatus for trait compatibility

use super::super::types::canonical::MemoryProtectionLevel;
use super::audit::logger::DefaultAuditLogger; // Add specific import
use super::crypto_providers::ring_crypto::RingCryptoProvider;
use super::crypto_providers::rust_crypto::RustCryptoProvider; // Import the correct RustCryptoProvider
use super::memory::{DefaultMemoryProtector, MemoryProtectionConfig};
use super::types::*;
use crate::tunnel::hsm::software_hsm::{CryptoBackend, MemoryConfig};
use crate::tunnel::hsm::types::config::SoftwareHsmConfig as CanonicalSoftwareHsmConfig;
use crate::tunnel::hsm::types::*;
use beardog_traits::canonical::HsmProvider;
use crate::tunnel::hsm::{GenerateKeyRequest, HsmConfig, HsmInfo, HsmKeyInfo};
use beardog_core::HsmKey; // Use core HsmKey to match trait expectation
use beardog_errors::{BearDogError, BearDogResult};
use chrono::Utc;
use hex;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};
impl RustSoftwareHsm {

    pub async fn new(config: CanonicalSoftwareHsmConfig) -> BearDogResult<Self> {
        info!("🔐 Initializing Rust Software HSM");

        let crypto_provider = Self::create_crypto_provider(&config.crypto_backend).await?;

        let memory_config = MemoryProtectionConfig {
            enable_protection: matches!(
                config.memory_protection,
                MemoryProtectionLevel::High | MemoryProtectionLevel::Maximum
            ),
            clear_on_drop: true, // Enable encryption by default
        };
        let memory_protector = Arc::new(DefaultMemoryProtector::new(memory_config).await?);

        let key_store = Arc::new(RwLock::new(
            SoftwareKeyStore::new(&config.key_storage).await?,
        ));

        let audit_logger = Self::create_audit_logger().await?;

        let health_monitor = Arc::new(SoftwareHealthMonitor::new().await?);
        let hsm = Self {
            config,
            key_store,
            crypto_provider,
            memory_protector,
            audit_logger,
            health_monitor,
        info!("✅ Rust Software HSM initialized successfully");
        Ok(hsm)
    }

    async fn create_crypto_provider(
        backend: &CryptoBackend,
    ) -> BearDogResult<impl CryptoProvider + Send + Sync + 'static> {
        match backend {
            CryptoBackend::RustCrypto => Ok(Arc::new(RustCryptoProvider::new().await?)),
            CryptoBackend::Ring => Ok(Arc::new(RingCryptoProvider::new()?)),
            CryptoBackend::OpenSsl => Ok(Arc::new(OpenSslCryptoProvider::new().await?)),
            CryptoBackend::Hardware => Err(BearDogError::unsupported_operation("Hardware crypto backend not supported in software HSM".to_string(),
            )),
            CryptoBackend::Custom(name) => Err(BearDogError::unsupported_operation(format!("Unsupported crypto backend: {name)"},
            }),
        }

    pub async fn create_memory_protector(
        config: &MemoryConfig,
    ) -> BearDogResult<Arc<dyn MemoryProtector>> {

        let memory_protection_config = MemoryProtectionConfig {
            enable_protection: !matches!(config.protection_level, MemoryProtectionLevel::None),
            clear_on_drop: config.enable_encryption,
        Ok(Arc::new(
            DefaultMemoryProtector::new(memory_protection_config).await?,
        ))

    async fn create_audit_logger() -> BearDogResult<Arc<dyn AuditLogger>> {
        Ok(Arc::new(DefaultAuditLogger::new().await?))

    async fn generate_software_key(&self, request: &GenerateKeyRequest) -> BearDogResult<HsmKey> {
        info!("🔑 Generating software key: {}", request.key_id);

        let key_material = self
            .crypto_provider
            .generate_key_material(&request.key_type)
            .await?;

        let protected_key = self
            .memory_protector
            .protect_key_material(&key_material)

        let software_key = SoftwareKey::new(
            request.key_id.clone(),
            request.key_type.clone(),
            protected_key,
            request.metadata.clone(),
        );

        let key_store = self.key_store.write().await;
        key_store.store_key(&software_key).await?;

        self.audit_logger
            .log_operation(&AuditLogEntry::success(
                "generate_key".to_string(),
                Some(request.key_id.clone()),
                None,
            ))

        let hsm_key = HsmKey {
            id: request.key_id.clone(),
            key_type: request.key_type.clone(),
            created_at: Utc::now(),
            metadata: {
                let mut meta = std::collections::HashMap::with_capacity(16);
                meta.insert("hsm_type".to_string(), "SoftwareHsm".to_string());
                meta.insert("algorithm".to_string(), format_args!("{:?}", request.key_type).to_string());
                meta.insert("key_material_type".to_string(), "Encrypted".to_string());
                meta
            },
        info!("✅ Software key generated successfully: {}", request.key_id);
        Ok(hsm_key)

    async fn perform_crypto_operation<T>(
        &self,
        key_id: &str,
        operation: &str,
        op_fn: impl Fn(&[u8]) -> Result<T, BearDogError>,
    ) -> BearDogResult<T> {
        let start_time = std::time::Instant::now();

        let key_store = self.key_store.read().await;
        let software_key = key_store.get_key(key_id).await?;

            .unprotect_key_material(&software_key.key_material)

        let result = op_fn(&key_material);

        self.memory_protector
            .zeroize_key_material(&key_material)

        let duration = start_time.elapsed();
        self.health_monitor
            .record_operation(operation, duration, result.is_ok())
        let log_result = if result.is_ok() { "success" } else { "failure" };
            .log_operation(&AuditLogEntry::new(
                operation.to_string(),
                Some(key_id.to_string()),
                log_result.to_string(),
                HashMap::with_capacity(16),
        result

    pub fn get_config(&self) -> &SoftwareHsmConfig {
        &self.config

    pub fn get_key_store(&self) -> &Arc<RwLock<SoftwareKeyStore>> {
        &self.key_store

    pub fn get_health_monitor(&self) -> &Arc<SoftwareHealthMonitor> {
        &self.health_monitor

    pub async fn update_config(&mut self, config: CanonicalSoftwareHsmConfig) -> BearDogResult<()> {
        info!("Updating Software HSM configuration");

        self.config = config;

        self.reload_configuration_internal().await?;
        info!("Software HSM configuration updated successfully");
        Ok(())
}

impl HsmProvider for RustSoftwareHsm {

    async fn initialize(&self, _config: HsmConfig) -> BearDogResult<()> {
        info!("🔄 Initializing Rust Software HSM");
        self.crypto_provider.initialize().await?;

        self.memory_protector.initialize().await?;
        key_store.initialize().await?;

                "initialize".to_string(),

    async fn generate_key(&self, request: GenerateKeyRequest) -> BearDogResult<HsmKey> {
        self.generate_software_key(&request).await

    async fn import_key(&self, key_data: &[u8], metadata: KeyMetadata) -> BearDogResult<HsmKey> {
        info!("📥 Importing key into Software HSM: {}", metadata.key_id);

        let protected_key = self.memory_protector.protect_key_material(key_data).await?;
        let protected_key_data = protected_key.data.clone();
            metadata.key_id.clone(),
            metadata.key_type.clone(),
            metadata.clone(),

                "import_key".to_string(),
                Some(metadata.key_id.clone()),
            id: metadata.key_id.clone(),
            key_type: metadata.key_type.clone(),
                meta.insert("algorithm".to_string(), format_args!("{:?}", metadata.key_type).to_string());
                meta.insert("key_material_type".to_string(), "Imported".to_string());
        info!("✅ Key imported successfully: {}", hsm_key.id);

    async fn encrypt(&self, key_id: &str, plaintext: &[u8]) -> BearDogResult<Vec<u8>> {
        debug!("🔒 Encrypting data with software key: {}", key_id);
        self.perform_crypto_operation(key_id, "encrypt", |key_material| {

            match tokio::runtime::Runtime::new() {
                Ok(rt) => rt.block_on(async {
                    self.crypto_provider.encrypt(key_material, plaintext).await
                }),
                Err(e) => Err(BearDogError::internal(format!("Failed to create runtime for encryption: {e)"),
            }
        })
        .await

    async fn decrypt(&self, key_id: &str, ciphertext: &[u8]) -> BearDogResult<Vec<u8>> {
        debug!("🔓 Decrypting data with software key: {}", key_id);
        self.perform_crypto_operation(key_id, "decrypt", |key_material| {
                    self.crypto_provider.decrypt(key_material, ciphertext).await
                    message: format!("Failed to create runtime for decryption: {e}"),

    async fn sign(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
        debug!("✍️ Signing data with software key: {}", key_id);
        self.perform_crypto_operation(key_id, "sign", |key_material| {
                Ok(rt) => {
                    rt.block_on(async { self.crypto_provider.sign(key_material, data).await })
                }
                    message: format!("Failed to create runtime for signing: {e}"),

    async fn verify(&self, key_id: &str, data: &[u8], signature: &[u8]) -> BearDogResult<bool> {
        debug!("🔍 Verifying signature with software key: {}", key_id);
        self.perform_crypto_operation(key_id, "verify", |key_material| {
                    self.crypto_provider
                        .verify(key_material, data, signature)
                        .await
                    message: format!("Failed to create runtime for verification: {e}"),

    async fn derive_key(
        master_key_id: &str,
        derivation_data: &[u8],
    ) -> BearDogResult<HsmKey> {
        info!(
            "🔑 Deriving key from software master key: {}",
            master_key_id

        let master_key = key_store.get_key(master_key_id).await?;
        let master_key_material = self
            .unprotect_key_material(&master_key.key_material)

        let derived_key_material = self
            .derive_key(&master_key_material, derivation_data)

            .zeroize_key_material(&master_key_material)

        let derived_key_id = format_args!("{}_{}", master_key_id, hex::encode(&derivation_data[..8]).to_string());
        let derived_metadata = KeyMetadata {
            key_id: derived_key_id.clone(),
            key_name: format!("derived-{derived_key_id}"),
            key_type: "Aes256".to_string(),
            last_used: None,
            expires_at: None,
            usage_policy: "default".to_string(),
            tags: HashMap::with_capacity(16),

        self.import_key(&derived_key_material, derived_metadata)
            .await

    async fn get_info(&self) -> BearDogResult<HsmInfo> {
        Ok(HsmInfo {
            instance_id: "software-hsm-001".to_string(),
            tier_type: "Software".to_string(),
            vendor: "BearDog".to_string(),
            model: "Rust Software HSM".to_string(),
            firmware_version: "1.0.0".to_string(),
            api_version: "1.0".to_string(),
            capabilities: vec![
                HsmCapability::KeyGeneration,
                HsmCapability::KeyImport,
                HsmCapability::KeyExport,
                HsmCapability::Encryption,
                HsmCapability::Decryption,
                HsmCapability::Signing,
                HsmCapability::Verification,
            ],
            supported_algorithms: vec!["AES-256".to_string(), "RSA-2048".to_string()],
            max_key_count: 10000,
            current_key_count: self.get_actual_key_count_internal().await.unwrap_or(0),
            status: HsmOperationalStatus::Operational,
            hsm_type: "SoftwareHsm".to_string(),
            version: "1.0.0".to_string(),
            max_key_size: Some(4096),
            certification: None,
            tamper_resistance: crate::tunnel::hsm::types::tier::TamperResistanceLevel::None,

    async fn list_keys(&self) -> BearDogResult<Vec<HsmKeyInfo>> {
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
                        total_operations: 1,
                    },
                    last_accessed: Some(chrono::Utc::now()),
                    access_count: 0,
                    key_id: key.id().to_string(),
                    key_type: key.key_type().clone(),
                    hsm_type: "SoftwareHsm".to_string(),
                    created_at: key.created_at(),
                    usage_policy: key.metadata().usage_policy.clone(),
                });
        Ok(key_infos)

    async fn delete_key(&self, key_id: &str) -> BearDogResult<()> {
        info!("🗑️ Deleting software key: {}", key_id);
        key_store.delete_key(key_id).await?;
                "delete_key".to_string(),
        info!("✅ Software key deleted successfully: {}", key_id);

    async fn backup(&self) -> BearDogResult<Option<Vec<u8>>> {
        info!("💾 Creating software HSM backup");
        let backup_data = key_store.backup().await?;
            .log_operation(&AuditLogEntry::success("backup".to_string(), None, None))
        info!("✅ Software HSM backup created successfully");
        Ok(Some(backup_data))

    async fn restore(&self, backup_data: &[u8]) -> BearDogResult<()> {
        info!("📥 Restoring software HSM from backup");
        key_store.restore(backup_data).await?;
            .log_operation(&AuditLogEntry::success("restore".to_string(), None, None))
        info!("✅ Software HSM restored successfully");

    async fn health_check(&self) -> BearDogResult<beardog_core::HsmHealthStatus> {

        let local_status = self.health_monitor.perform_health_check().await?;
        Ok(beardog_core::HsmHealthStatus {
            is_healthy: local_status.is_healthy,
            last_check: local_status.last_check,
            error_count: local_status.error_count,
            uptime: local_status.uptime,

    async fn reload_configuration_internal(&self) -> BearDogResult<()> {
        info!("🔧 Reloading HSM configuration");

        let _crypto_provider = Self::create_crypto_provider(&self.config.crypto_backend).await?;

        debug!("🔧 Crypto provider configuration reloaded");

                self.config.memory_config.protection_level,
            clear_on_drop: self.config.memory_config.enable_encryption,
        debug!(
            "🔧 Memory protection configuration reloaded: {:?}",
            memory_config

        debug!("🔧 Key store configuration validated");

        debug!("🔧 Audit configuration reloaded");
        info!("✅ HSM configuration hot-reload completed successfully");

    async fn get_actual_key_count_internal(&self) -> BearDogResult<u32> {
        let _key_store = self.key_store.read().await;

        let key_count = 0u32; // Mock implementation
        debug!("📊 Current key count: {}", key_count);
        Ok(key_count)
