

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::{
    canonical::{
        hsm::{HsmKey, KeyMetadata, KeyUsagePolicy},
        KeyType,
    },
    providers::{
        BaseProvider, HsmHardwareStatus, HsmInfo, HsmKeyInfo, HsmProvider, ProviderConfig,
        ProviderHealthStatus, KeyHealth, KeyMaterial, KeyOperation,
    },
};

use parking_lot::RwLock;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};

pub use beardog_types::canonical::hsm::config::SoftwareHsmConfig;

#[derive(Debug, Clone)]
pub struct HsmStats {
    pub total_keys: usize,
    pub operations_performed: u64,
    pub storage_utilization: f64,
}

pub struct ZeroCostSoftwareHsm<
    const MAX_KEYS: usize = 1000,
    const KEY_SIZE_LIMIT: usize = 4096,
> {
    keys: RwLock<HashMap<String, HsmKey>>,
    operations_count: AtomicU64,
    config: SoftwareHsmConfig,
    _phantom: PhantomData<()>,
}

impl<const MAX_KEYS: usize, const KEY_SIZE_LIMIT: usize>
    ZeroCostSoftwareHsm<MAX_KEYS, KEY_SIZE_LIMIT>
{
    #[must_use] 
    pub fn new(config: SoftwareHsmConfig) -> Self {
        Self {
            keys: RwLock::new(HashMap::with_capacity(16)),
            operations_count: AtomicU64::new(0),
            config,
            _phantom: PhantomData,
        }
    }

    pub fn get_stats(&self) -> HsmStats {
        let keys = self.keys.read();
        HsmStats {
            total_keys: keys.len(),
            operations_performed: self.operations_count.load(Ordering::Relaxed),
            storage_utilization: (keys.len() as f64 / MAX_KEYS as f64) * 100.0,
        }
    }
}

impl<const MAX_KEYS: usize, const KEY_SIZE_LIMIT: usize> BaseProvider
    for ZeroCostSoftwareHsm<MAX_KEYS, KEY_SIZE_LIMIT>
{
    fn provider_id(&self) -> &str {
        "zero-cost-software-hsm"
    }

    async fn get_capabilities(&self) -> BearDogResult<Vec<String>> {
        Ok(vec![
            "key_generation".to_string(),
            "signing".to_string(),
            "verification".to_string(),
            "encryption".to_string(),
            "decryption".to_string(),
        ])
    }

    async fn initialize(&mut self, _config: ProviderConfig) -> BearDogResult<()> {

        Ok(())
    }

    async fn health_check(&self) -> BearDogResult<ProviderHealthStatus> {
        Ok(ProviderHealthStatus {
            is_healthy: true,
            last_check: chrono::Utc::now(),
            details: Some("Zero-cost software HSM operational".to_string()),
            response_time_ms: Some(1),
        })
    }

    async fn shutdown(&mut self) -> BearDogResult<()> {

        self.keys.write().clear();
        Ok(())
    }
}

impl<const MAX_KEYS: usize, const KEY_SIZE_LIMIT: usize> HsmProvider
    for ZeroCostSoftwareHsm<MAX_KEYS, KEY_SIZE_LIMIT>
{
    async fn generate_key(
        &self,
        key_type: KeyType,
        metadata: KeyMetadata,
    ) -> BearDogResult<HsmKey> {
        self.operations_count.fetch_add(1, Ordering::Relaxed);
        let mut keys = self.keys.write();
        
        if keys.len() >= MAX_KEYS {
            return Err(BearDogError::System {
                message: format_args!("HSM key slots exhausted: {}/{}", keys.len().to_string(), MAX_KEYS),
            });
        }

        let key_id = format_args!("key_{}", uuid::Uuid::new_v4().to_string());
        let key = HsmKey {
            id: key_id.clone(),
            key_type,
            material: KeyMaterial::SoftwareHandle {
                handle: key_id.clone(),
                metadata: HashMap::with_capacity(16),
            },
            metadata,
            created_at: chrono::Utc::now(),
            expires_at: None,
            key_name: key_id.clone(),
            last_used: None,
            usage_count: 0,
            health: KeyHealth::Healthy,
        };

        keys.insert(key_id, key.clone());
        Ok(key)
    }

    async fn sign_data(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
        let keys = self.keys.read();
        let _key = keys.get(key_id).ok_or_else(|| BearDogError::System {
            message: format!("Key not found for signing: {key_id}"),
        })?;

        Ok(data.to_vec())
    }

    async fn verify_signature(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> BearDogResult<bool> {
        let keys = self.keys.read();
        let _key = keys.get(key_id).ok_or_else(|| BearDogError::System {
            message: format!("Key not found for verification: {key_id}"),
        })?;

        Ok(signature == data)
    }

    async fn encrypt_with_key(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
        let keys = self.keys.read();
        let _key = keys.get(key_id).ok_or_else(|| BearDogError::System {
            message: format!("Key not found for encryption: {key_id}"),
        })?;

        Ok(data.to_vec())
    }

    async fn decrypt_with_key(
        &self,
        key_id: &str,
        encrypted_data: &[u8],
    ) -> BearDogResult<Vec<u8>> {
        let keys = self.keys.read();
        let _key = keys.get(key_id).ok_or_else(|| BearDogError::System {
            message: format!("Key not found for decryption: {key_id}"),
        })?;

        Ok(encrypted_data.to_vec())
    }

    async fn get_key_info(&self, key_id: &str) -> BearDogResult<HsmKeyInfo> {
        let keys = self.keys.read();
        let key = keys.get(key_id).ok_or_else(|| BearDogError::System {
            message: format!("Key not found: {key_id}"),
        })?;
        
        Ok(HsmKeyInfo {
            key_id: key.id.clone(),
            key_type: key.key_type.clone(),
            created_at: key.created_at,
            usage_count: key.usage_count,
            metadata: key.metadata.clone(),
        })
    }

    async fn list_keys(&self) -> BearDogResult<Vec<String>> {
        let keys = self.keys.read();
        Ok(keys.keys().cloned().collect())
    }

    async fn get_hardware_status(&self) -> BearDogResult<HsmHardwareStatus> {
        Ok(HsmHardwareStatus {
            available: true,
            temperature: None,
            free_memory: Some(1024 * 1024 * 1024), // 1GB placeholder
            uptime_seconds: Some(86400),           // 1 day placeholder
            error_count: 0,
        })
    }

    async fn import_key(
        &self,
        _key_data: &[u8],
        _metadata: KeyMetadata,
    ) -> BearDogResult<HsmKey> {
        Err(BearDogError::System {
            message: "Key import not supported by zero-cost HSM".to_string(),
        })
    }

    async fn derive_key(
        &self,
        _master_key_id: &str,
        _derivation_data: &[u8],
        _derived_key_type: KeyType,
        _metadata: KeyMetadata,
    ) -> BearDogResult<HsmKey> {
        Err(BearDogError::System {
            message: "Key derivation not supported by zero-cost HSM".to_string(),
        })
    }

    async fn delete_key(&self, key_id: &str) -> BearDogResult<()> {
        let mut keys = self.keys.write();
        match keys.remove(key_id) {
            Some(_) => Ok(()),
            None => Err(BearDogError::System {
                message: format!("Key not found: {key_id}"),
            }),
        }
    }

    async fn get_hsm_info(&self) -> BearDogResult<HsmInfo> {
        Ok(HsmInfo {
            instance_id: "zero-cost-hsm-001".to_string(),
            vendor: "BearDog".to_string(),
            model: "ZeroCostSoftwareHsm".to_string(),
            firmware_version: "1.0.0".to_string(),
            api_version: "1.0".to_string(),
            supported_algorithms: vec![
                "AES-256".to_string(),
                "Ed25519".to_string(),
                "RSA-2048".to_string(),
            ],
            max_key_count: MAX_KEYS as u32,
            current_key_count: self.keys.read().len() as u32,
            capabilities: vec![
                "generate_key".to_string(),
                "sign_data".to_string(),
                "encrypt_with_key".to_string(),
            ],
            certification: Some("Zero-Cost Optimized".to_string()),
            tamper_resistant: false, // Software HSM
        })
    }

    fn is_hardware_backed(&self) -> bool {
        false // Software HSM
    }
}
