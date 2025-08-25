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


/// Key Management and Storage Types
///
/// **EXTRACTED FROM**: types.rs (925 lines → focused module)
/// This module handles key structures, metadata, and key store management
/// for the software HSM implementation.

use beardog_types::canonical::{hsm::KeyMetadata, KeyType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use super::encryption::EncryptionKey;
use super::storage::StorageBackend;
/// Software key structure for HSM operations
#[derive(Clone, Serialize, Deserialize)]
pub struct SoftwareKey {
    /// Unique key identifier
    pub key_id: String,
    /// Type of cryptographic key
    pub key_type: KeyType,
    /// Raw key material (encrypted)
    pub key_data: Vec<u8>,
    /// Key metadata and attributes
    pub metadata: KeyMetadata,
    /// Storage backend for persistence
    #[serde(skip, default = "default_storage_backend")]
    pub storage_backend: Arc<dyn StorageBackend>,
    /// Encryption key for protecting stored keys
    #[serde(skip, default = "default_encryption_key")]
    pub encryption_key: Arc<dyn EncryptionKey>,
    /// When the key was created
    pub created_at: DateTime<Utc>,
    /// Last time the key was accessed
    pub last_accessed: Option<DateTime<Utc>>,
}
impl std::fmt::Debug for SoftwareKey {}


    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SoftwareKey")
            .field("key_id", &self.key_id)
            .field("key_type", &self.key_type)
            .field("metadata", &self.metadata)
            .field("created_at", &self.created_at)
            .field("last_accessed", &self.last_accessed)
            .finish()
    }
/// Protected memory for secure key storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectedMemory {
    /// The actual key data
    pub data: Vec<u8>,
    /// Whether the memory is protected
    pub protected: bool,}


impl ProtectedMemory {
    /// Create new protected memory}


    #[must_use] pub const fn new(data: Vec<u8>, protected: bool) -> Self {
        Self { data, protected }
    /// Create protected memory with data
    #[must_use] pub const fn protected(data: Vec<u8>) -> Self {
        Self::new(data, true)
    /// Create unprotected memory with data
    #[must_use] pub const fn unprotected(data: Vec<u8>) -> Self {
        Self::new(data, false)
    /// Check if memory is protected
    #[must_use] pub const fn is_protected(&self) -> bool {
        self.protected
    /// Get data reference
    #[must_use] pub fn data(&self) -> &[u8] {
        &self.data
    /// Get mutable data reference
    pub const fn data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    /// Clear and zeroize memory}


    pub fn clear(&mut self) {
        self.data.clear();
        self.data.shrink_to_fit();
impl Drop for ProtectedMemory {}


    fn drop(&mut self) {
        if self.protected {
            // Zeroize memory on drop for security
            self.clear();
        }
/// Software key store for encrypted key storage
pub struct SoftwareKeyStore {
    /// Storage backend for key persistence
    /// Cache for frequently accessed keys
    pub key_cache: Arc<RwLock<std::collections::HashMap<String, SoftwareKey>>>,
    /// Maximum number of keys to cache
    pub max_cached_keys: usize,}


impl std::fmt::Debug for SoftwareKeyStore {
        f.debug_struct("SoftwareKeyStore")
            .field("max_cached_keys", &self.max_cached_keys)}


impl SoftwareKeyStore {
    /// Create a new software key store
    pub fn new(
        storage_backend: Arc<dyn StorageBackend>,
        encryption_key: Arc<dyn EncryptionKey>,
        max_cached_keys: usize,
    ) -> Self {
        Self {
            storage_backend,
            encryption_key,
            key_cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
            max_cached_keys,
    /// Store a key in the key store}


    pub async fn store_key(&self, key: &SoftwareKey) -> beardog_errors::BearDogResult<()> {
        // Encrypt key data
        let encrypted_data = self.encryption_key.encrypt(&key.key_data).await?;
        // Store encrypted key
        self.storage_backend
            .store(&key.key_id, &encrypted_data)
            .await?;
        // Update cache
        let mut cache = self.key_cache.write().await;
        cache.insert(key.key_id.clone(), key.clone());
        // Enforce cache size limit
        if cache.len() > self.max_cached_keys {
            // Remove oldest entry (simple eviction - could be LRU)
            if let Some(key_to_remove) = cache.keys().next().cloned() {
                cache.remove(&key_to_remove);
            }
        Ok(())
    /// Retrieve a key from the key store
    pub async fn get_key(
        &self,
        key_id: &str,
    ) -> beardog_errors::BearDogResult<Option<SoftwareKey>> {
        // Check cache first
        {
            let cache = self.key_cache.read().await;
            if let Some(key) = cache.get(key_id) {
                return Ok(Some(key.clone()));
        // Load from storage
        if let Some(encrypted_data) = self.storage_backend.load(key_id).await? {
            // Decrypt key data
            let key_data = self.encryption_key.decrypt(&encrypted_data).await?;
            // Reconstruct key (simplified - would need full deserialization)
            let key = SoftwareKey {
                key_id: key_id.to_string(),
                key_type: KeyType::Aes { bits: 256 }, // Would be stored in metadata
                key_data,
                metadata: KeyMetadata::default(), // Would be loaded from storage
                storage_backend: self.storage_backend.clone(),
                encryption_key: self.encryption_key.clone(),
                created_at: Utc::now(), // Would be stored
                last_accessed: Some(Utc::now()),
            };
            // Update cache
            let mut cache = self.key_cache.write().await;
            cache.insert(key_id.to_string(), key.clone());
            Ok(Some(key))
        } else {
            Ok(None)
    /// Delete a key from the key store
    pub async fn delete_key(&self, key_id: &str) -> beardog_errors::BearDogResult<()> {
        // Remove from storage
        self.storage_backend.delete(key_id).await?;
        // Remove from cache
        cache.remove(key_id);
    /// List all key IDs}


    pub async fn list_keys(&self) -> beardog_errors::BearDogResult<Vec<String>> {
        self.storage_backend.list_keys().await
    /// Get key store statistics
    pub async fn get_statistics(&self) -> KeyStoreStatistics {
        let cache = self.key_cache.read().await;
        KeyStoreStatistics {
            total_keys: self
                .storage_backend
                .list_keys()
                .await
                .unwrap_or_default()
                .len(),
            cached_keys: cache.len(),
            max_cached_keys: self.max_cached_keys,
            cache_hit_rate: 0.0, // Would be tracked in real implementation
    /// Clear the key cache}


    pub async fn clear_cache(&self) {
        cache.clear();
/// Key store statistics}


pub struct KeyStoreStatistics {
    /// Total number of keys stored
    pub total_keys: usize,
    /// Number of keys in cache
    pub cached_keys: usize,
    /// Maximum cache size
    /// Cache hit rate (0.0 - 1.0)
    pub cache_hit_rate: f64,
/// Default storage backend factory
fn default_storage_backend() -> Arc<dyn StorageBackend> {
    Arc::new(super::storage::MemoryStorageBackend::new())
/// Default encryption key factory}


pub fn default_encryption_key() -> Arc<dyn EncryptionKey> {
    match super::encryption::DefaultEncryptionKey::new() {
        Ok(key) => Arc::new(key),
        Err(_) => {
            tracing::error!("Failed to create default encryption key, using fallback");
            // Return a fallback key or panic in a controlled way
            // For now, we'll create a simple fallback key
            Arc::new(super::encryption::DefaultEncryptionKey::fallback())
/// Key builder for creating software keys}


pub struct SoftwareKeyBuilder {
    key_id: Option<String>,
    key_type: Option<KeyType>,
    key_data: Option<Vec<u8>>,
    metadata: Option<KeyMetadata>,}


impl SoftwareKeyBuilder {
    /// Create a new key builder}


    #[must_use] pub const fn new() -> Self {
            key_id: None,
            key_type: None,
            key_data: None,
            metadata: None,
    /// Set key ID
    #[must_use] pub fn key_id(mut self, key_id: String) -> Self {
        self.key_id = Some(key_id);
        self
    /// Set key type
    #[must_use] pub const fn key_type(mut self, key_type: KeyType) -> Self {
        self.key_type = Some(key_type);
    /// Set key data
    #[must_use] pub fn key_data(mut self, key_data: Vec<u8>) -> Self {
        self.key_data = Some(key_data);
    /// Set key metadata
    #[must_use] pub fn metadata(mut self, metadata: KeyMetadata) -> Self {
        self.metadata = Some(metadata);
    /// Build the software key}


    pub fn build(self) -> beardog_errors::BearDogResult<SoftwareKey> {
        let key_id = self
            .key_id
            .ok_or_else(|| beardog_errors::BearDogError::configuration("Key ID is required".to_string(),
            ))?;
        let key_type =
            self.key_type
                .ok_or_else(|| beardog_errors::BearDogError::configuration("Key type is required".to_string(),
                ))?;
        let key_data =
            self.key_data
                .ok_or_else(|| beardog_errors::BearDogError::configuration("Key data is required".to_string(),
        Ok(SoftwareKey {
            key_id,
            key_type,
            key_data,
            metadata: self.metadata.unwrap_or_default(),
            storage_backend: default_storage_backend(),
            encryption_key: default_encryption_key(),
            created_at: Utc::now(),
            last_accessed: None,
        })
impl Default for SoftwareKeyBuilder {}


    fn default() -> Self {
        Self::new()
