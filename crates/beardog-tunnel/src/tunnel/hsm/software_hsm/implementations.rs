

use beardog_errors::BearDogError;
use beardog_types::canonical::hsm::config::SoftwareHsmConfig;
use beardog_types::canonical::hsm::{HsmKey, KeyHealth, KeyMaterial};
use beardog_types::canonical::{KeyMetadata, KeyOperation, KeyType, KeyUsagePolicy};
use beardog_types::providers::{
    BaseProvider, HsmHardwareStatus, HsmInfo, HsmKeyInfo, HsmProvider, ProviderConfig,
    ProviderHealthStatus,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use super::keys::SoftwareKeyStore;

pub struct RustCryptoProvider;

pub struct OpenSslCryptoProvider;

pub struct RustSoftwareHsm {

    pub config: SoftwareHsmConfig,

    pub key_store: Arc<RwLock<SoftwareKeyStore>>,

    pub key_registry: Arc<RwLock<std::collections::HashMap<String, HsmKey>>>,
}
impl RustSoftwareHsm {

    pub async fn new(config: SoftwareHsmConfig) -> Result<Self, BearDogError> {

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

    async fn get_capabilities(&self) -> Result<Vec<String>, BearDogError>> {
        Ok(vec![
            "key_generation".to_string(),
            "encryption".to_string(),
            "decryption".to_string(),
            "signing".to_string(),
        ])
    async fn initialize(&mut self, _config: ProviderConfig) -> Result<(), BearDogError> {
        Ok(())}

    async fn shutdown(&mut self) -> Result<(), BearDogError> {
    async fn health_check(&self) -> Result<ProviderHealthStatus, BearDogError> {
        Ok(ProviderHealthStatus {
            is_healthy: true,
            last_check: chrono::Utc::now(),
            details: Some("Software HSM operational".to_string()),
            response_time_ms: Some(1),
impl HsmProvider for RustSoftwareHsm {}

    async fn generate_key(
        &self,
        key_type: KeyType,
        metadata: KeyMetadata,
    ) -> Result<HsmKey, BearDogError> {
        use uuid::Uuid;

        let key_id = format_args!("rust_hsm_{}", Uuid::new_v4().to_string());

        let hsm_key = HsmKey {
            id: key_id.clone(),
            key_type: key_type.clone(),
            material: KeyMaterial::SoftwareHandle {
                handle: key_id.clone(),
                metadata: {
                    let mut meta = std::collections::HashMap::with_capacity(16);
                    meta.insert("provider".to_string(), "RustSoftwareHsm".to_string());
                    meta.insert("type".to_string(), format!("{key_type:?}"));
                    meta
                },
            },
            metadata: KeyMetadata {
                created_by: "RustSoftwareHsm".to_string(),
                purpose: metadata.purpose.clone(),
                usage_policy: KeyUsagePolicy {
                    allowed_operations: vec![
                        KeyOperation::Sign,
                        KeyOperation::Verify,
                        KeyOperation::Encrypt,
                        KeyOperation::Decrypt,
                    ],
                    max_uses: None,
                    usage_limits: None,
                    exportable: false,
                    extractable: false,
                    time_restrictions: None,
                    network_restrictions: None,
                tags: metadata.tags.clone(),
                custom_fields: std::collections::HashMap::with_capacity(16),
                custom: metadata.custom.clone(),
                backup_info: None,
                compliance_tags: Vec::new(),
                compliance_info: None,
            created_at: chrono::Utc::now(),
            expires_at: None,
            key_name: format_args!("RustHSM Key {}", metadata.purpose).to_string(),
            last_used: None,
            usage_count: 0,
            health: KeyHealth::Healthy,
        };

        {
            let mut registry = self.key_registry.write().await;
            registry.insert(key_id.clone(), hsm_key.clone());
        }
        Ok(hsm_key)
    async fn import_key(
        key_data: &[u8],

        let key_id = format_args!("rust_hsm_imported_{}", Uuid::new_v4().to_string());

        Ok(HsmKey {
                handle: key_id,
                    meta.insert("imported".to_string(), "true".to_string());
                    meta.insert("data_length".to_string(), key_data.len().to_string());
            key_name: format_args!("Imported Key {}", metadata.purpose).to_string(),
    async fn derive_key(
        master_key_id: &str,
        derivation_data: &[u8],
        derived_key_type: KeyType,
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        master_key_id.hash(&mut hasher);
        derivation_data.hash(&mut hasher);
        let derivation_hash = hasher.finish();
        let key_id = format_args!("rust_hsm_derived_{}_{:x}", Uuid::new_v4().to_string(), derivation_hash);
            key_type: derived_key_type.clone(),
                    meta.insert("type".to_string(), format!("{derived_key_type:?}"));
                    meta.insert("derived_from".to_string(), master_key_id.to_string());
                    meta.insert(
                        "derivation_hash".to_string(),
                        format!("{derivation_hash:x}"),
                    );
            key_name: format_args!("Derived Key {}", metadata.purpose).to_string(),

    async fn delete_key(&self, key_id: &str) -> Result<(), BearDogError> {

        let mut registry = self.key_registry.write().await;
        registry.remove(key_id);}

    async fn sign_data(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {

        let keys = self.list_keys().await?;
        if !keys.contains(&key_id.to_string()) {
            return Err(beardog_errors::BearDogError::not_found(format!(
                "Key not found: {key_id}"
            )));

        key_id.hash(&mut hasher);
        data.hash(&mut hasher);
        let hash = hasher.finish();

        let mut signature = vec![0u8; 64];
        let hash_bytes = hash.to_le_bytes();

        for (i, byte) in signature.iter_mut().enumerate() {
            *byte = hash_bytes[i % 8] ^ (i as u8);
        Ok(signature)
    async fn verify_signature(
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {

        let expected_signature = self.sign_data(key_id, data).await?;
        Ok(expected_signature == signature)}

    async fn encrypt_with_key(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
            return Err(BearDogError::not_found(format!("Key not found: {key_id}")));

        let key_hash = hasher.finish();
        let key_bytes = key_hash.to_le_bytes();
        let mut encrypted = Vec::with_capacity(data.len());
        for (i, &byte) in data.iter().enumerate() {
            encrypted.push(byte ^ key_bytes[i % 8]);
        Ok(encrypted)
    async fn decrypt_with_key(
        encrypted_data: &[u8],
    ) -> Result<Vec<u8>, BearDogError>> {

        self.encrypt_with_key(key_id, encrypted_data).await}

    async fn list_keys(&self) -> Result<Vec<String>, BearDogError>> {

        let registry = self.key_registry.read().await;
        Ok(registry.keys().cloned().collect())
    async fn get_key_info(&self, key_id: &str) -> Result<HsmKeyInfo, BearDogError> {
        Ok(HsmKeyInfo {
            key_id: key_id.to_string(),
            key_type: beardog_types::canonical::KeyType::Ed25519,
            metadata: beardog_types::canonical::hsm::keys::KeyMetadata {
                created_by: "software_hsm".to_string(),
                purpose: key_id.to_string(),
                tags: vec!["software".to_string()],
                usage_policy: beardog_types::canonical::hsm::keys::KeyUsagePolicy {
                        beardog_types::canonical::hsm::keys::KeyOperation::Sign,
                custom: std::collections::HashMap::with_capacity(16),}

    async fn get_hsm_info(&self) -> Result<HsmInfo, BearDogError> {
        Ok(HsmInfo {
            instance_id: "rust-software-hsm-001".to_string(),
            vendor: "`BearDog`".to_string(),}

            model: "Rust Software HSM".to_string(),
            firmware_version: "1.0.0".to_string(),
            api_version: "1.0".to_string(),
            supported_algorithms: vec!["AES256".to_string(), "ChaCha20".to_string()],
            max_key_count: 10000,
            current_key_count: 0,
            capabilities: vec!["key_generation".to_string(), "encryption".to_string()],
            certification: None,
            tamper_resistant: false,
    async fn get_hardware_status(&self) -> Result<HsmHardwareStatus, BearDogError> {
        Ok(HsmHardwareStatus {
            available: true,
            temperature: Some(25.0),
            free_memory: Some(1024 * 1024 * 100), // 100MB
            uptime_seconds: Some(3600),
            error_count: 0,}

    fn is_hardware_backed(&self) -> bool {
        false // Software HSM is not hardware-backed

pub struct SoftwareHsm {

    pub id: String,

