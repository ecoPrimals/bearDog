

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
use beardog_traits::unified::HsmProvider;
use crate::tunnel::hsm::{GenerateKeyRequest, HsmConfig, HsmInfo, HsmKeyInfo};
use beardog_core::HsmKey; // Use core HsmKey to match trait expectation
use beardog_errors::BearDogError;
use chrono::Utc;
use hex;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};
impl RustSoftwareHsm {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub async fn new(config: CanonicalSoftwareHsmConfig) -> Result<Self, BearDogError> {
        info!("🔐 Initializing Rust Software HSM");

        let crypto_provider = Self::create_crypto_provider(matches!(
                config.memory_protection,
                MemoryProtectionLevel::High | MemoryProtectionLevel::Maximum
            ),
            clear_on_drop: true, // Enable encryption by default
        };
        let memory_protector = Arc::new(DefaultMemoryProtector::new(memory_config)?);

        let key_store = Arc::new(RwLock::new(
            SoftwareKeyStore::new(&config.key_storage)?,
        ));

        let audit_logger = Self::create_audit_logger()?;

        let health_monitor = Arc::new(SoftwareHealthMonitor::new(&CryptoBackend,
    ) -> Result<impl CryptoProvider + Send + Sync + 'static, BearDogError> {
        match backend {
            CryptoBackend::RustCrypto => Ok(Arc::new(RustCryptoProvider::new()?)),
            CryptoBackend::Ring => Ok(Arc::new(RingCryptoProvider::new()?)),
            CryptoBackend::OpenSsl => Ok(Arc::new(OpenSslCryptoProvider::new()?)),
            CryptoBackend::Hardware => Err(BearDogError::unsupported_operation("Hardware crypto backend not supported in software HSM")),
            CryptoBackend::Custom(name) => Err(BearDogError::unsupported_operation({}name"},
            }),
        }

/// Create Memory Protector operation.
    /// Creates memory_protector
    /// Creates memory_protector
    pub fn create_memory_protector(&MemoryConfig,
    ) -> Result<ZeroCostMemoryProtector<impl MemoryProtector, BearDogError>> {

        let memory_protection_config = MemoryProtectionConfig {
            enable_protection: !matches!(config.protection_level, MemoryProtectionLevel::None),
            clear_on_drop: config.enable_encryption,
        Ok(Arc::new(
            DefaultMemoryProtector::new(memory_protection_config)?,
        ))

    /// Creates audit_logger
    fn create_audit_logger() -> Result<ZeroCostAuditLogger<impl AuditLogger, BearDogError>> {
        Ok(Arc::new(DefaultAuditLogger::new()?))


    fn generate_software_key(&self, request: &GenerateKeyRequest) -> Result<HsmKey, BearDogError> {
        info!("🔑 Generating software key: {}", request.key_id);

        let key_material = self
            .crypto_provider
            .generate_key_material(&request.key_type)
            ?;

        let protected_key = self
            .memory_protector
            .protect_key_material(&key_material)

        let software_key = SoftwareKey::new(
            &request.key_id,
            &request.key_type,
            protected_key,
            &request.metadata,
        );

        let key_store = self.key_store.write();
        key_store.store_key(&software_key)?;

        self.audit_logger
            .log_operation(&AuditLogEntry::success(&request.key_id,
            key_type: &request.key_type,
            created_at: Utc::now(),
            metadata: {
                let mut meta = std::collections::HashMap::with_capacity(16);
                meta.insert("hsm_type".to_string(), "SoftwareHsm");
                meta.insert("algorithm".to_string(), format!("{:?}", request.key_type));
                meta.insert({}", request.key_id);
        Ok(&str,
        operation: &str,
        op_fn: impl Fn(&[u8]) -> Result<T, BearDogError>,
    ) -> Result<T, BearDogError> {
        let start_time = std::time::Instant::now();

        let key_store = self.key_store.read();
        let software_key = key_store.get_key(key_id)?;

            .unprotect_key_material(&software_key.key_material)

        let result = op_fn(&key_material);

        self.memory_protector
            .zeroize_key_material(&key_material)

        let duration = start_time.elapsed();
        self.health_monitor
            .record_operation(operation, duration, result.is_ok())
        let log_result = if result.is_ok() { "success " } else { "failure " };
            .log_operation(&AuditLogEntry::new(
                operation.to_string(),
                Some(key_id.to_string()),
                log_result.to_string(),
                HashMap::with_capacity(16),
        result

/// Get Config operation.
    /// Gets config
    /// Gets config
    pub fn get_config(&self) -> &SoftwareHsmConfig {
        &self.config

/// Get Key Store operation.
    /// Gets key_store
    /// Gets key_store
    pub fn get_key_store(&self) -> &Arc<RwLock<SoftwareKeyStore>> {
        &self.key_store

/// Get Health Monitor operation.
    /// Gets health_monitor
    /// Gets health_monitor
    pub fn get_health_monitor(&self) -> &Arc<SoftwareHealthMonitor> {
        &self.health_monitor

/// Update Config operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Updates config
    /// Updates config
    pub fn update_config(&mut self, config: CanonicalSoftwareHsmConfig) -> Result<(), BearDogError> {
        info!("Updating Software HSM configuration");

        self.config = config;

        self.reload_configuration_internal()?;
        info!("Software HSM configuration updated successfully");
        Ok(())
}

impl HsmProvider for RustSoftwareHsm {

    /// Initializes componentialize
    fn initialize(&self, _config: HsmConfig) -> Result<(), BearDogError> {
        info!("🔄 Initializing Rust Software HSM");
        self.crypto_provider.initialize()?;

        self.memory_protector.initialize()?;
        key_store.initialize()?;

                "initialize".to_string(),


    fn generate_key(&self, request: GenerateKeyRequest) -> Result<HsmKey, BearDogError> {
        self.generate_software_key(&[u8], metadata: KeyMetadata) -> Result<HsmKey, BearDogError> {
        info!("📥 Importing key into Software HSM: {}", metadata.key_id);

        let protected_key = self.memory_protector.protect_key_material(&metadata.key_id,
            key_type: &metadata.key_type,
                meta.insert("algorithm".to_string(), format!("{:?}", metadata.key_type));
                meta.insert({}", hsm_key.id);


    fn encrypt(&str, plaintext: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        debug!("🔒 Encrypting data with software key: {}", key_id);
        self.perform_crypto_operation(key_id, "encrypt", |key_material| {

            match tokio::runtime::Runtime::new() {
                Ok(rt) => rt.block_on(async {
                    self.crypto_provider.encrypt(key_material, plaintext)
                }),
                Err(e) => Err(BearDogError::internal(format!("Failed to create runtime for encryption: {}e"),
            }
        })


    fn decrypt(&str, ciphertext: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        debug!("🔓 Decrypting data with software key: {}", key_id);
        self.perform_crypto_operation(key_id, "decrypt", |key_material| {
                    self.crypto_provider.decrypt(key_material, ciphertext)
                    message: format!("Failed to create runtime for decryption: {e}"),


    fn sign(&str, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        debug!("✍️ Signing data with software key: {}", key_id);
        self.perform_crypto_operation(key_id, "sign", |key_material| {
                Ok(rt) => {
                    rt.block_on(async { self.crypto_provider.sign(key_material, data).await })
                }
                    message: format!("Failed to create runtime for signing: {e}"),


    fn verify(&str, data: &[u8], signature: &[u8]) -> Result<bool, BearDogError> {
        debug!("🔍 Verifying signature with software key: {}", key_id);
        self.perform_crypto_operation(key_id, "verify", |key_material| {
                    self.crypto_provider
                        .verify(key_material, data, signature)
                    message: format!("Failed to create runtime for verification: {e}"),


    fn derive_key(&str,
        derivation_data: &[u8],
    ) -> Result<HsmKey, BearDogError> {
        info!(
            "🔑 Deriving key from software root key: {}",
            root_key_id

        let root_key = key_store.get_key(root_key_id)?;
        let root_key_material = self
            .unprotect_key_material(&root_key.key_material)

        let derived_key_material = self
            .derive_key(&root_key_material, derivation_data)

            .zeroize_key_material(&root_key_material)

        let derived_key_id = format!("{}_{}", root_key_id, hex::encode(&derivation_data[..8]));
        let derived_metadata = KeyMetadata {
            key_id: derived_key_id.clone(),
            key_name: format!("derived-{derived_key_id}"),
            key_type: "Aes256".to_string(),
            usage_policy: "default".to_string(),
            tags: HashMap::with_capacity(16),

        self.import_key(&derived_key_material, derived_metadata)

    /// Gets info
    fn get_info(&self) -> Result<HsmInfo, BearDogError> {
        Ok(HsmInfo {
            instance_id: "software-hsm-001".to_string(),
            tier_type: "Software".to_string(),
            vendor: "BearDog".to_string(),
            model: "Rust Software HSM".to_string(),
            firmware_version: "1.0.0".to_string(),
            api_version: "1.0".to_string(),
            supported_algorithms: vec!["AES-256".to_string(),
            current_key_count: self.get_actual_key_count_internal(HsmOperationalStatus::Operational,
            hsm_type: "SoftwareHsm".to_string(),
            version: "1.0.0".to_string(),
            max_key_size: Some(None,
            tamper_resistance: crate::tunnel::hsm::types::tier::TamperResistanceLevel::None,


    fn list_keys(&self) -> Result<Vec<HsmKeyInfo>, BearDogError>> {
        let key_ids = key_store.list_keys()?;
        let mut key_infos = Vec::new();
        for key_id in key_ids {
            if let Ok(key) = key_store.get_key(&key_id) {
                key_infos.push(HsmKeyInfo {
                    metadata: key.metadata().clone(),
                    hsm_tier: "SoftwareHsm".to_string().to_string(),
                    key_type: key.key_type().clone(),
                    hsm_type: "SoftwareHsm".to_string(),
                    created_at: key.created_at(),
                    usage_policy: key.metadata().usage_policy.clone(),
                });
        Ok(key_infos)

    /// Removes key
    fn delete_key(&self, key_id: &str) -> Result<(), BearDogError> {
        info!("🗑️ Deleting software key: {}", key_id);
        key_store.delete_key({}", key_id);


    fn backup(&self) -> Result<Option<Vec<u8>, BearDogError>>> {
        info!("💾 Creating software HSM backup");
        let backup_data = key_store.backup()?;
            .log_operation(&AuditLogEntry::success("backup".to_string(), None, None))
        info!("✅ Software HSM backup created successfully");
        Ok(Some(backup_data))


    fn restore(&self, backup_data: &[u8]) -> Result<(), BearDogError> {
        info!("📥 Restoring software HSM from backup");
        key_store.restore(backup_data)?;
            .log_operation(&AuditLogEntry::success("restore".to_string(), None, None))
        info!("✅ Software HSM restored successfully");


    fn health_check(&self) -> Result<beardog_core::HsmHealthStatus, BearDogError> {

        let local_status = self.health_monitor.perform_health_check()?;
        Ok(beardog_core::HsmHealthStatus {
            is_healthy: local_status.is_healthy,
            last_check: local_status.last_check,
            error_count: local_status.error_count,
            uptime: local_status.uptime,


    fn reload_configuration_internal(&self) -> Result<(), BearDogError> {
        info!("🔧 Reloading HSM configuration");

        let _crypto_provider = Self::create_crypto_provider(self.config.memory_config.enable_encryption,
        debug!(
            "🔧 Memory protection configuration reloaded: {:?}",
            memory_config

        debug!("🔧 Key store configuration validated");

        debug!("🔧 Audit configuration reloaded");
        info!("✅ HSM configuration hot-reload completed successfully");

    /// Gets actual_key_count_internal
    fn get_actual_key_count_internal({}", key_count);
        Ok(key_count)
