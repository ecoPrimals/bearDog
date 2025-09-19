

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use beardog_errors::BearDogError;
use beardog_types::canonical::hsm::keys::{KeyHealth, KeyMetadata, KeyUsagePolicy};
use beardog_types::canonical::HsmKey;
use super::memory::SecureMemory;

#[derive(Arc<RwLock<HashMap<String, StoredKey>>>,
    max_keys: usize,
}

#[derive(Debug, Clone)]
    secure_material: SecureMemory,
    created_at: DateTime<Utc>,
    last_used: DateTime<Utc>,
    usage_count: u64,
impl KeyStore {

/// New operation.
    /// Creates a new instance
    pub fn new(max_keys: usize) -> Self {
        Self {
            keys: Arc::new(RwLock::new(HashMap::with_capacity(&str,
        key: HsmKey,
        key_material: Vec<u8>,
    ) -> Result<(), BearDogError> {
        let mut keys = self.keys.write();
        if keys.len() >= self.max_keys {
            return Err(BearDogError::Hsm("Key store capacity exceeded".to_string()));
        let now = Utc::now();
        let stored_key = StoredKey {
            key,
            secure_material: SecureMemory::from_data(now,
            last_used: now,
            usage_count: 0,
        };
        keys.insert(key_id, stored_key);
        Ok(())

/// Get Key operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets key
    /// Gets key
    pub fn get_key(&self, key_id: &str) -> Result<Option<HsmKey>, BearDogError>> {
        if let Some(stored_key) = keys.get_mut(key_id) {
            stored_key.last_used = Utc::now();
            stored_key.usage_count += 1;
            Ok(Some(stored_key.key))
        } else {
            Ok(None)

/// Get Key Material operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets key_material
    /// Gets key_material
    pub fn get_key_material(&self, key_id: &str) -> Result<Option<Vec<u8>, BearDogError>>> {
            Ok(Some(stored_key.secure_material.as_slice().to_vec()))

/// Delete Key operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Removes key
    /// Removes key
    pub fn delete_key(&self, key_id: &str) -> Result<bool, BearDogError> {
        Ok(keys.remove(key_id).is_some())

/// List Keys operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn list_keys(&self) -> Result<Vec<String>, BearDogError>> {
        let keys = self.keys.read();
        Ok(keys.keys().cloned().collect())

/// Get Key Metadata operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets key_metadata
    /// Gets key_metadata
    pub fn get_key_metadata(&self, key_id: &str) -> Result<Option<KeyMetadata>, BearDogError>> {
        if let Some(stored_key) = keys.get(key_id) {
            let metadata = KeyMetadata {

                created_by: "software_hsm".to_string(),
                purpose: "cryptographic_operations".to_string(),
                usage_policy: KeyUsagePolicy::default(stored_key.key.&metadata.tags,
                compliance_info: None,
                backup_info: None,
                compliance_tags: vec![],
                health: KeyHealth::Healthy,

                is_hardware_backed: false,
                user_presence_required: false,
                algorithm: format!("{:?}", stored_key.key.key_type),
                attestation_available: false,
                created_at: stored_key.created_at,
                last_accessed: Some(stored_key.usage_count,
                hsm_type: "software".to_string(),
                hsm_tier: "software".to_string(),
                key_id: key_id.to_string(),
                health_status: "healthy".to_string(),

                custom: std::collections::HashMap::with_capacity(16),
                attributes: std::collections::HashMap::with_capacity(16),
                attributes: { let mut attrs = HashMap::with_capacity(None,
                expires_at: None,
                creation_time: Some(stored_key.created_at),
                last_used: Some(stored_key.last_used),
                usage_count: Some(stored_key.usage_count),
                is_exportable: Some(false),

            };
            Ok(Some(metadata))

/// Update Key Usage operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Updates key_usage
    /// Updates key_usage
    pub fn update_key_usage(&self, key_id: &str) -> Result<(), BearDogError> {

/// Get Statistics operation.
    /// Gets statistics
    /// Gets statistics
    pub fn get_statistics(HashMap<String, usize> =
            keys.values().fold(HashMap::with_capacity(16), |mut acc, stored_key| {
                let key_type_str = format!("{:?}", stored_key.key.key_type);
                *acc.entry(self.max_keys,
            total_usage,
            average_usage: avg_usage,
            key_types,

/// Cleanup Keys operation.
    /// Cleans up keys
    /// Cleans up keys
    pub fn cleanup_keys(u32,
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

/// Key Exists operation.
    pub fn key_exists(&self, key_id: &str) -> bool {
        keys.contains_key(usize,
    /// Number of max_capacity
    pub max_capacity: usize,
    /// Number of total_usage
    pub total_usage: u64,
    /// The average usage value
    pub average_usage: f64,
    /// Mapping of key types
    pub key_types: HashMap<String, usize>,
#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::canonical::crypto::KeyType;
    #[tokio::test]}


    fn test_key_store_operations() -> Result<(), BearDogError> {
        let keystore = KeyStore::new(100);
        let key = HsmKey {
            key_id: "test-key".to_string(),
            tags: HashMap::with_capacity(16),

        keystore
            .store_key("test-key".to_string(), key.clone(), vec![5, 6, 7, 8])
            .map_err(|e| {
                tracing::error!("Operation failed: {e:?}");
                beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
            })?;

        let retrieved = keystore.get_key("test-key").map_err(|e| {
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

        assert!(keystore.key_exists("test-key"));

        let stats = keystore.get_statistics();
        assert_eq!(stats.total_keys, 1);
    fn test_key_store_capacity() -> Result<(), BearDogError> {
        let keystore = KeyStore::new(2);
        let key1 = HsmKey {
            key_id: "key1".to_string(),
        let key2 = HsmKey {
            key_id: "key2".to_string(),

            .store_key("key1".to_string(), key1, vec![1])
            .store_key("key2".to_string(), key2, vec![2])

        let result = keystore.store_key("key3".to_string(), key3, vec![3]);
        assert!(result.is_err());
