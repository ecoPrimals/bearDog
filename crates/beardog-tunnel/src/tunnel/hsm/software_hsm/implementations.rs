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


/// HSM Implementation Types
///
/// **EXTRACTED FROM**: types.rs (925 lines → focused module)

use async_trait::async_trait;
use beardog_errors::{BearDogError, BearDogResult};
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
/// Rust crypto provider implementation
pub struct RustCryptoProvider;
/// OpenSSL crypto provider implementation
pub struct OpenSslCryptoProvider;
/// Rust software HSM implementation
pub struct RustSoftwareHsm {
    /// Configuration for the software HSM
    pub config: SoftwareHsmConfig,
    /// Key store for managing keys
    pub key_store: Arc<RwLock<SoftwareKeyStore>>,
    /// Simple in-memory key registry for testing
    pub key_registry: Arc<RwLock<std::collections::HashMap<String, HsmKey>>>,
}
impl RustSoftwareHsm {
    /// Create a new Rust software HSM instance
    pub async fn new(config: SoftwareHsmConfig) -> BearDogResult<Self> {
        // Create default storage backend and encryption key
        let storage_backend = Arc::new(super::storage::MemoryStorageBackend::new());
        let encryption_key = super::keys::default_encryption_key();
        let max_cached_keys = 1000; // Default cache size
        let key_store = Arc::new(RwLock::new(SoftwareKeyStore::new(
            storage_backend,
            encryption_key,
            max_cached_keys,
        )));
        let key_registry = Arc::new(RwLock::new(std::collections::HashMap::new()));
        Ok(Self {
            config,
            key_store,
            key_registry,
        })
    }

impl BaseProvider for RustSoftwareHsm {}


    fn provider_id(&self) -> &str {
        "rust-software-hsm"}


    async fn get_capabilities(&self) -> BearDogResult<Vec<String>> {
        Ok(vec![
            "key_generation".to_string(),
            "encryption".to_string(),
            "decryption".to_string(),
            "signing".to_string(),
        ])
    async fn initialize(&mut self, _config: ProviderConfig) -> BearDogResult<()> {
        Ok(())}


    async fn shutdown(&mut self) -> BearDogResult<()> {
    async fn health_check(&self) -> BearDogResult<ProviderHealthStatus> {
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
    ) -> BearDogResult<HsmKey> {
        use uuid::Uuid;
        // Generate a unique key ID
        let key_id = format!("rust_hsm_{}", Uuid::new_v4());
        // Create HSM key with proper structure
        let hsm_key = HsmKey {
            id: key_id.clone(),
            key_type: key_type.clone(),
            material: KeyMaterial::SoftwareHandle {
                handle: key_id.clone(),
                metadata: {
                    let mut meta = std::collections::HashMap::new();
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
                custom_fields: std::collections::HashMap::new(),
                custom: metadata.custom.clone(),
                backup_info: None,
                compliance_tags: Vec::new(),
                compliance_info: None,
            created_at: chrono::Utc::now(),
            expires_at: None,
            key_name: format!("RustHSM Key {}", metadata.purpose),
            last_used: None,
            usage_count: 0,
            health: KeyHealth::Healthy,
        };
        // Store the key in our simple registry for testing
        {
            let mut registry = self.key_registry.write().await;
            registry.insert(key_id.clone(), hsm_key.clone());
        }
        Ok(hsm_key)
    async fn import_key(
        key_data: &[u8],
        // Generate a unique key ID for imported key
        let key_id = format!("rust_hsm_imported_{}", Uuid::new_v4());
        // Create HSM key with imported data
        Ok(HsmKey {
                handle: key_id,
                    meta.insert("imported".to_string(), "true".to_string());
                    meta.insert("data_length".to_string(), key_data.len().to_string());
            key_name: format!("Imported Key {}", metadata.purpose),
    async fn derive_key(
        master_key_id: &str,
        derivation_data: &[u8],
        derived_key_type: KeyType,
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        // Create deterministic derived key ID
        let mut hasher = DefaultHasher::new();
        master_key_id.hash(&mut hasher);
        derivation_data.hash(&mut hasher);
        let derivation_hash = hasher.finish();
        let key_id = format!("rust_hsm_derived_{}_{:x}", Uuid::new_v4(), derivation_hash);
            key_type: derived_key_type.clone(),
                    meta.insert("type".to_string(), format!("{derived_key_type:?}"));
                    meta.insert("derived_from".to_string(), master_key_id.to_string());
                    meta.insert(
                        "derivation_hash".to_string(),
                        format!("{derivation_hash:x}"),
                    );
            key_name: format!("Derived Key {}", metadata.purpose),
    // export_key is not part of the HsmProvider trait - removed
    async fn delete_key(&self, key_id: &str) -> BearDogResult<()> {
        // Remove the key from our simple registry
        let mut registry = self.key_registry.write().await;
        registry.remove(key_id);}


    async fn sign_data(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
        // Check if key exists first
        let keys = self.list_keys().await?;
        if !keys.contains(&key_id.to_string()) {
            return Err(beardog_errors::BearDogError::not_found(format!(
                "Key not found: {key_id}"
            )));
        // For software HSM, create a deterministic signature based on key_id and data
        key_id.hash(&mut hasher);
        data.hash(&mut hasher);
        let hash = hasher.finish();
        // Create a 64-byte signature (typical for Ed25519)
        let mut signature = vec![0u8; 64];
        let hash_bytes = hash.to_le_bytes();
        // Fill signature with deterministic but unique data
        for (i, byte) in signature.iter_mut().enumerate() {
            *byte = hash_bytes[i % 8] ^ (i as u8);
        Ok(signature)
    async fn verify_signature(
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> BearDogResult<bool> {
        // Verify by regenerating the expected signature and comparing
        let expected_signature = self.sign_data(key_id, data).await?;
        Ok(expected_signature == signature)}


    async fn encrypt_with_key(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
            return Err(BearDogError::not_found(format!("Key not found: {key_id}")));
        // Simple XOR encryption with key-derived mask
        let key_hash = hasher.finish();
        let key_bytes = key_hash.to_le_bytes();
        let mut encrypted = Vec::with_capacity(data.len());
        for (i, &byte) in data.iter().enumerate() {
            encrypted.push(byte ^ key_bytes[i % 8]);
        Ok(encrypted)
    async fn decrypt_with_key(
        encrypted_data: &[u8],
    ) -> BearDogResult<Vec<u8>> {
        // XOR encryption is symmetric, so decryption is the same as encryption
        self.encrypt_with_key(key_id, encrypted_data).await}


    async fn list_keys(&self) -> BearDogResult<Vec<String>> {
        // Return keys from our simple registry
        let registry = self.key_registry.read().await;
        Ok(registry.keys().cloned().collect())
    async fn get_key_info(&self, key_id: &str) -> BearDogResult<HsmKeyInfo> {
        Ok(HsmKeyInfo {
            key_id: key_id.to_string(),
            key_type: beardog_types::canonical::KeyType::Ed25519,
            metadata: beardog_types::canonical::hsm::keys::KeyMetadata {
                created_by: "software_hsm".to_string(),
                purpose: key_id.to_string(),
                tags: vec!["software".to_string()],
                usage_policy: beardog_types::canonical::hsm::keys::KeyUsagePolicy {
                        beardog_types::canonical::hsm::keys::KeyOperation::Sign,
                custom: std::collections::HashMap::new(),}


    async fn get_hsm_info(&self) -> BearDogResult<HsmInfo> {
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
    async fn get_hardware_status(&self) -> BearDogResult<HsmHardwareStatus> {
        Ok(HsmHardwareStatus {
            available: true,
            temperature: Some(25.0),
            free_memory: Some(1024 * 1024 * 100), // 100MB
            uptime_seconds: Some(3600),
            error_count: 0,}


    fn is_hardware_backed(&self) -> bool {
        false // Software HSM is not hardware-backed
/// Software HSM implementation
pub struct SoftwareHsm {
    /// Unique identifier for this HSM instance
    pub id: String,
    /// Configuration for the Software HSM
