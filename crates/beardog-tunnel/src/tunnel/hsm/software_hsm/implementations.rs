

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use beardog_types::canonical::hsm::config::SoftwareHsmConfig;
use beardog_types::canonical::hsm::{HsmKey, KeyHealth, KeyMaterial};
use beardog_types::canonical::{KeyMetadata, KeyOperation, KeyType, KeyUsagePolicy};
use beardog_types::canonical::providers_unified::migration::{
    ProviderHealthStatus,
};
use beardog_types::canonical::providers_unified::traits::{
    ProviderInfo as HsmInfo, ProviderHealth as HsmHardwareStatus,
};
use beardog_types::canonical::providers_unified::{
    CanonicalProviderConfig as ProviderConfig,
};
use beardog_types::canonical::hsm::KeyInfo as HsmKeyInfo;
use std::sync::Arc;
use tokio::sync::RwLock;
use super::keys::SoftwareKeyStore;

pub struct RustCryptoProvider;

pub struct OpenSslCryptoProvider;

pub struct RustSoftwareHsm {


    pub config: SoftwareHsmConfig,

    /// The key store value
    pub key_store: Arc<RwLock<SoftwareKeyStore>>,

    /// The key registry value
    pub key_registry: Arc<RwLock<std::collections::HashMap<String, HsmKey>>>,
}
impl RustSoftwareHsm {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new(config: SoftwareHsmConfig) -> Result<Self, BearDogError> {

        let storage_backend = Arc::new(super::storage::MemoryStorageBackend::new());
        let encryption_key = super::keys::default_encryption_key();
        let max_cached_keys = 1000; // Default cache size
        let key_store = Arc::new(RwLock::new(SoftwareKeyStore::new(
            storage_backend,
            encryption_key,
            max_cached_keys,
        )));
        let key_registry = Arc::new(RwLock::new(std::collections::HashMap::with_capacity(16)));
        Ok(Self {
            config,
            key_store,
            key_registry,
        })
    }

impl BaseProvider for RustSoftwareHsm {}


    fn provider_id(&self) -> &str {
        "rust-software-hsm"}

    /// Gets capabilities
    fn get_capabilities(&self) -> Result<Vec<String>, BearDogError>> {
        Ok(vec![
            "key_generation".to_string(),
            "encryption ".to_string(),
            "decryption".to_string(),
            "signing".to_string(),
        ])
    /// Initializes componentialize
    fn initialize(&mut self, _config: ProviderConfig) -> Result<(), BearDogError> {
        Ok(true,
            last_check: chrono::Utc::now(),
            details: Some("Software HSM operational".to_string()),
            response_time_ms: Some(KeyType,
        metadata: KeyMetadata,
    ) -> Result<HsmKey, BearDogError> {
        use uuid::Uuid;

        let key_id = format!("rust_hsm_{}", Uuid::new_v4());

        let hsm_key = HsmKey {
            id: key_id.clone(),
            key_type: key_type.clone(),
            material: KeyMaterial::SoftwareHandle {
                handle: key_id.clone(),
                metadata: {
                    let mut meta = std::collections::HashMap::with_capacity(16);
                    meta.insert("provider".to_string(), "RustSoftwareHsm");
                    meta.insert("type".to_string(), format!("{key_type:?}"));
                    meta
                },
            },
            metadata: KeyMetadata {
                created_by: "RustSoftwareHsm".to_string(),
            last_used: None,
            usage_count: 0,
            health: KeyHealth::Healthy,
        };

        {
            let mut registry = self.key_registry.write(&[u8],

        let key_id = format!("rust_hsm_imported_{}", Uuid::new_v4(key_id,
                    meta.insert(format!("Imported Key {}", metadata.purpose),
    fn derive_key(&str,
        derivation_data: &[u8],
        derived_key_type: KeyType,
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        master_key_id.hash(&mut hasher);
        derivation_data.hash(&mut hasher);
        let derivation_hash = hasher.finish();
        let key_id = format!("rust_hsm_derived_{}_{:x}", Uuid::new_v4(), derivation_hash);
            key_type: derived_key_type.clone(),
                    meta.insert("type".to_string(), format!("{derived_key_type:?}"));
                    meta.insert("derived_from".to_string(), master_key_id);
                    meta.insert(
                        "derivation_hash".to_string(),
                        format!("{derivation_hash:x}"),
                    );
            key_name: format!("Derived Key {}", metadata.purpose),

    /// Removes key
    fn delete_key(&self, key_id: &str) -> Result<(), BearDogError> {

        let mut registry = self.key_registry.write(&str, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {

        let keys = self.list_keys()?;
        if !keys.contains(&key_id.to_string()) {
            return Err(beardog_errors::BearDogError::not_found(format!(
                "Key not found: {key_id}"
            )));

        key_id.hash(&str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {

        let expected_signature = self.sign_data(&str, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
            return Err(BearDogError::not_found(format!("Key not found: {key_id}")));

        let key_hash = hasher.finish();
        let key_bytes = key_hash.to_le_bytes();
        let mut encrypted = Vec::with_capacity(&[u8],
    ) -> Result<Vec<u8>, BearDogError>> {

        self.encrypt_with_key(key_id, encrypted_data)}


    fn list_keys(&self) -> Result<Vec<String>, BearDogError>> {

        let registry = self.key_registry.read();
        Ok(registry.keys().cloned().collect())
    /// Gets key_info
    fn get_key_info(&self, key_id: &str) -> Result<HsmKeyInfo, BearDogError> {
        Ok(HsmKeyInfo {
            key_id: key_id.to_string() -> Result<HsmInfo, BearDogError> {
        Ok(HsmInfo {
            instance_id: "rust-software-hsm-001".to_string(),
            vendor: "`BearDog`".to_string(),}

            model: "Rust Software HSM".to_string(),
            firmware_version: "1.0.0".to_string(),
            api_version: "1.0".to_string(),
            supported_algorithms: vec!["AES256".to_string(),
            current_key_count: 0,
            capabilities: vec!["key_generation".to_string(),
            temperature: Some(25.0),
            free_memory: Some(1024 * 1024 * 100), // 100MB
            uptime_seconds: Some(0,}

    /// Checks if hardware backed
    fn is_hardware_backed(String,

