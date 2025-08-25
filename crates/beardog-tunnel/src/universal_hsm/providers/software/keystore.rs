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


/// # Key Storage Management
///
/// Secure key storage and management functionality for the software HSM.

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::hsm::keys::{KeyHealth, KeyMetadata, KeyUsagePolicy};
use beardog_types::canonical::HsmKey;
use super::memory::SecureMemory;
/// Secure key storage implementation
#[derive(Debug)]
pub struct KeyStore {
    keys: Arc<RwLock<HashMap<String, StoredKey>>>,
    max_keys: usize,
}
/// Internal key storage structure
#[derive(Debug, Clone)]
struct StoredKey {
    key: HsmKey,
    secure_material: SecureMemory,
    created_at: DateTime<Utc>,
    last_used: DateTime<Utc>,
    usage_count: u64,
impl KeyStore {
    /// Create a new key store}


    pub fn new(max_keys: usize) -> Self {
        Self {
            keys: Arc::new(RwLock::new(HashMap::new())),
            max_keys,
        }
    }
    /// Store a new key
    pub async fn store_key(
        &self,
        key_id: String,
        key: HsmKey,
        key_material: Vec<u8>,
    ) -> BearDogResult<()> {
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
    /// Retrieve a key
    pub async fn get_key(&self, key_id: &str) -> BearDogResult<Option<HsmKey>> {
        if let Some(stored_key) = keys.get_mut(key_id) {
            stored_key.last_used = Utc::now();
            stored_key.usage_count += 1;
            Ok(Some(stored_key.key.clone()))
        } else {
            Ok(None)
    /// Get key material for cryptographic operations
    pub async fn get_key_material(&self, key_id: &str) -> BearDogResult<Option<Vec<u8>>> {
            Ok(Some(stored_key.secure_material.as_slice().to_vec()))
    /// Delete a key}


    pub async fn delete_key(&self, key_id: &str) -> BearDogResult<bool> {
        Ok(keys.remove(key_id).is_some())
    /// List all key IDs
    pub async fn list_keys(&self) -> BearDogResult<Vec<String>> {
        let keys = self.keys.read().await;
        Ok(keys.keys().cloned().collect())
    /// Get key metadata}


    pub async fn get_key_metadata(&self, key_id: &str) -> BearDogResult<Option<KeyMetadata>> {
        if let Some(stored_key) = keys.get(key_id) {
            let metadata = KeyMetadata {
                // Core fields
                created_by: "software_hsm".to_string(),
                purpose: "cryptographic_operations".to_string(),
                usage_policy: KeyUsagePolicy::default(),
                tags: stored_key.key.metadata.tags.clone(),
                compliance_info: None,
                backup_info: None,
                compliance_tags: vec![],
                health: KeyHealth::Healthy,
                // Tunnel compatibility fields
                is_hardware_backed: false,
                user_presence_required: false,
                algorithm: format!("{:?}", stored_key.key.key_type),
                attestation_available: false,
                created_at: stored_key.created_at,
                last_accessed: Some(stored_key.last_used),
                access_count: stored_key.usage_count,
                hsm_type: "software".to_string(),
                hsm_tier: "software".to_string(),
                key_type: format!("{:?}", stored_key.key.key_type),
                key_id: key_id.to_string(),
                health_status: "healthy".to_string(),
                performance_metrics: None,
                provider_attributes: std::collections::HashMap::new(),
                derivation_path: None,
                // Legacy compatibility fields
                custom: std::collections::HashMap::new(),
                attributes: std::collections::HashMap::new(),
                attributes: { let mut attrs = HashMap::new(); attrs.insert("key_name".to_string(), key_id.to_string(.to_string()); attrs }),
                key_size: None,
                expires_at: None,
                creation_time: Some(stored_key.created_at),
                last_used: Some(stored_key.last_used),
                usage_count: Some(stored_key.usage_count),
                is_exportable: Some(false),
                // hardware_backed info moved to is_hardware_backed field,
            };
            Ok(Some(metadata))
    /// Update key usage statistics
    pub async fn update_key_usage(&self, key_id: &str) -> BearDogResult<()> {
    /// Get key store statistics}


    pub async fn get_statistics(&self) -> KeyStoreStatistics {
        let total_keys = keys.len();
        let total_usage = keys.values().map(|k| k.usage_count).sum();
        let avg_usage = if total_keys > 0 {
            total_usage as f64 / total_keys as f64
            0.0
        // Collect key type counts as strings (KeyType doesn't implement Hash)
        let key_types: HashMap<String, usize> =
            keys.values().fold(HashMap::new(), |mut acc, stored_key| {
                let key_type_str = format!("{:?}", stored_key.key.key_type);
                *acc.entry(key_type_str).or_insert(0) += 1;
                acc
            });
        KeyStoreStatistics {
            total_keys,
            max_capacity: self.max_keys,
            total_usage,
            average_usage: avg_usage,
            key_types,
    /// Cleanup old or unused keys
    pub async fn cleanup_keys(
        max_age_days: u32,
        min_usage_threshold: u64,
    ) -> BearDogResult<usize> {
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
    /// Check if key exists
    pub async fn key_exists(&self, key_id: &str) -> bool {
        keys.contains_key(key_id)
    /// Get current key count}


    pub async fn key_count(&self) -> usize {
        keys.len()
    /// Check if key store is at capacity
    pub async fn is_at_capacity(&self) -> bool {
        keys.len() >= self.max_keys
/// Key store statistics
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


    async fn test_key_store_operations() -> beardog_errors::BearDogResult<()> {
        let keystore = KeyStore::new(100);
        let key = HsmKey {
            key_id: "test-key".to_string(),
            key_type: KeyType::Ed25519,
            public_key: vec![1, 2, 3, 4],
            tags: HashMap::new(),
        // Store key
        keystore
            .store_key("test-key".to_string(), key.clone(), vec![5, 6, 7, 8])
            .await
            .map_err(|e| {
                tracing::error!("Operation failed: {e:?}");
                beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
            })?;
        // Retrieve key
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
        // Check key exists
        assert!(keystore.key_exists("test-key").await);
        // Get statistics
        let stats = keystore.get_statistics().await;
        assert_eq!(stats.total_keys, 1);
    async fn test_key_store_capacity() -> beardog_errors::BearDogResult<()> {
        let keystore = KeyStore::new(2);
        let key1 = HsmKey {
            key_id: "key1".to_string(),
        let key2 = HsmKey {
            key_id: "key2".to_string(),
            public_key: vec![5, 6, 7, 8],
        let key3 = HsmKey {
            key_id: "key3".to_string(),
            public_key: vec![9, 10, 11, 12],
        // Store first two keys
            .store_key("key1".to_string(), key1, vec![1])
            .store_key("key2".to_string(), key2, vec![2])
        // Third key should fail due to capacity
        let result = keystore.store_key("key3".to_string(), key3, vec![3]).await;
        assert!(result.is_err());
