

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use beardog_errors::BearDogError;
use beardog_types::canonical::hsm::keys::{KeyHealth, KeyMetadata, KeyUsagePolicy};
use beardog_types::canonical::HsmKey;
use super::memory::SecureMemory;

#[derive(Debug)]
pub struct KeyStore {
    keys: Arc<RwLock<HashMap<String, StoredKey>>>,
    max_keys: usize,
}

#[derive(Debug, Clone)]
struct StoredKey {
    key: HsmKey,
    secure_material: SecureMemory,
    created_at: DateTime<Utc>,
    last_used: DateTime<Utc>,
    usage_count: u64,
impl KeyStore {

    pub fn new(max_keys: usize) -> Self {
        Self {
            keys: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            max_keys,
        }
    }

    pub async fn store_key(
        &self,
        key_id: &str,
        key: HsmKey,
        key_material: Vec<u8>,
    ) -> Result<(), BearDogError> {
        let mut keys = self.keys.write().await;
        if keys.len() >= self.max_keys {
            return Err(BearDogError::Hsm("Key store capacity exceeded".to_string()));
        let now = Utc::now();
        let stored_key = StoredKey {
            key,
            secure_material: SecureMemory::from_data(key_material),
            created_at: now,
            last_used: now,
            usage_count: 0,
        };
        keys.insert(key_id, stored_key);
        Ok(())

    pub async fn get_key(&self, key_id: &str) -> Result<Option<HsmKey>, BearDogError>> {
        if let Some(stored_key) = keys.get_mut(key_id) {
            stored_key.last_used = Utc::now();
            stored_key.usage_count += 1;
            Ok(Some(stored_key.key.clone()))
        } else {
            Ok(None)

    pub async fn get_key_material(&self, key_id: &str) -> Result<Option<Vec<u8>, BearDogError>>> {
            Ok(Some(stored_key.secure_material.as_slice().to_vec()))

    pub async fn delete_key(&self, key_id: &str) -> Result<bool, BearDogError> {
        Ok(keys.remove(key_id).is_some())

    pub async fn list_keys(&self) -> Result<Vec<String>, BearDogError>> {
        let keys = self.keys.read().await;
        Ok(keys.keys().cloned().collect())

    pub async fn get_key_metadata(&self, key_id: &str) -> Result<Option<KeyMetadata>, BearDogError>> {
        if let Some(stored_key) = keys.get(key_id) {
            let metadata = KeyMetadata {

                created_by: "software_hsm".to_string(),
                purpose: "cryptographic_operations".to_string(),
                usage_policy: KeyUsagePolicy::default(),
                tags: stored_key.key.metadata.tags.clone(),
                compliance_info: None,
                backup_info: None,
                compliance_tags: vec![],
                health: KeyHealth::Healthy,

                is_hardware_backed: false,
                user_presence_required: false,
                algorithm: format_args!("{:?}", stored_key.key.key_type).to_string(),
                attestation_available: false,
                created_at: stored_key.created_at,
                last_accessed: Some(stored_key.last_used),
                access_count: stored_key.usage_count,
                hsm_type: "software".to_string(),
                hsm_tier: "software".to_string(),
                key_type: format_args!("{:?}", stored_key.key.key_type).to_string(),
                key_id: key_id.to_string(),
                health_status: "healthy".to_string(),
                performance_metrics: None,
                provider_attributes: std::collections::HashMap::with_capacity(16),
                derivation_path: None,

                custom: std::collections::HashMap::with_capacity(16),
                attributes: std::collections::HashMap::with_capacity(16),
                attributes: { let mut attrs = HashMap::with_capacity(16); attrs.insert("key_name".to_string(), key_id.to_string(.to_string()); attrs }),
                key_size: None,
                expires_at: None,
                creation_time: Some(stored_key.created_at),
                last_used: Some(stored_key.last_used),
                usage_count: Some(stored_key.usage_count),
                is_exportable: Some(false),

            };
            Ok(Some(metadata))

    pub async fn update_key_usage(&self, key_id: &str) -> Result<(), BearDogError> {

    pub async fn get_statistics(&self) -> KeyStoreStatistics {
        let total_keys = keys.len();
        let total_usage = keys.values().map(|k| k.usage_count).sum();
        let avg_usage = if total_keys > 0 {
            total_usage as f64 / total_keys as f64
            0.0

        let key_types: HashMap<String, usize> =
            keys.values().fold(HashMap::with_capacity(16), |mut acc, stored_key| {
                let key_type_str = format_args!("{:?}", stored_key.key.key_type).to_string();
                *acc.entry(key_type_str).or_insert(0) += 1;
                acc
            });
        KeyStoreStatistics {
            total_keys,
            max_capacity: self.max_keys,
            total_usage,
            average_usage: avg_usage,
            key_types,

    pub async fn cleanup_keys(
        max_age_days: u32,
        min_usage_threshold: u64,
    ) -> Result<usize, BearDogError> {
        let cutoff_date = Utc::now() - chrono::Duration::days(max_age_days as i64);
        let keys_to_remove: Vec<String> = keys
            .iter()
            .filter(|(_, stored_key)| {
                stored_key.created_at < cutoff_date && stored_key.usage_count < min_usage_threshold
            })
            .map(|(key_id, _)| key_id.clone())
            .collect();
        let removed_count = keys_to_remove.len();
        for key_id in keys_to_remove {
            keys.remove(&key_id);
        Ok(removed_count)

    pub async fn key_exists(&self, key_id: &str) -> bool {
        keys.contains_key(key_id)

    pub async fn key_count(&self) -> usize {
        keys.len()

    pub async fn is_at_capacity(&self) -> bool {
        keys.len() >= self.max_keys

pub struct KeyStoreStatistics {
    pub total_keys: usize,
    pub max_capacity: usize,
    pub total_usage: u64,
    pub average_usage: f64,
    pub key_types: HashMap<String, usize>,
#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::canonical::crypto::KeyType;
    #[tokio::test]}

    async fn test_key_store_operations() -> Result<(), BearDogError> {
        let keystore = KeyStore::new(100);
        let key = HsmKey {
            key_id: "test-key".to_string(),
            key_type: KeyType::Ed25519,
            public_key: vec![1, 2, 3, 4],
            tags: HashMap::with_capacity(16),

        keystore
            .store_key("test-key".to_string(), key.clone(), vec![5, 6, 7, 8])
            .await
            .map_err(|e| {
                tracing::error!("Operation failed: {e:?}");
                beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
            })?;

        let retrieved = keystore.get_key("test-key").await.map_err(|e| {
            tracing::error!("Operation failed: {e:?}");
            beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        })?;
        assert!(retrieved.is_some());
        assert_eq!(
            retrieved
                .map_err(|e| {
                    tracing::error!("Operation failed: {e:?}");
                    beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
                })?
                .key_id,
            "test-key"
        );

        assert!(keystore.key_exists("test-key").await);

        let stats = keystore.get_statistics().await;
        assert_eq!(stats.total_keys, 1);
    async fn test_key_store_capacity() -> Result<(), BearDogError> {
        let keystore = KeyStore::new(2);
        let key1 = HsmKey {
            key_id: "key1".to_string(),
        let key2 = HsmKey {
            key_id: "key2".to_string(),
            public_key: vec![5, 6, 7, 8],
        let key3 = HsmKey {
            key_id: "key3".to_string(),
            public_key: vec![9, 10, 11, 12],

            .store_key("key1".to_string(), key1, vec![1])
            .store_key("key2".to_string(), key2, vec![2])

        let result = keystore.store_key("key3".to_string(), key3, vec![3]).await;
        assert!(result.is_err());
