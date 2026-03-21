// SPDX-License-Identifier: AGPL-3.0-only

// Ecosystem Storage Manager Implementation

use super::backends::StorageBackend;
use super::cache::CacheManager;
use super::config::EcosystemStorageConfig;
use super::metrics::StorageMetrics;
use super::operations::{StorageRequest, StorageResponse};
use super::types::{StorageOperation, StorageStatus};

use beardog_errors::BearDogError;
use beardog_types::constants::domains::storage::messages::NO_BACKEND_AVAILABLE;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Ecosystem storage manager
pub struct EcosystemStorageManager {
    /// Storage configuration
    config: EcosystemStorageConfig,
    /// Storage backends
    backends: Vec<Arc<dyn StorageBackend>>,
    /// Cache manager
    cache: Arc<RwLock<CacheManager>>,
    /// Storage metrics
    metrics: Arc<RwLock<StorageMetrics>>,
}

impl std::fmt::Debug for EcosystemStorageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EcosystemStorageManager")
            .field("config", &self.config)
            .field("backends_count", &self.backends.len())
            .field("cache", &"<CacheManager>")
            .field("metrics", &"<StorageMetrics>")
            .finish()
    }
}

impl EcosystemStorageManager {
    /// Create a new storage manager
    /// Creates a new instance
    #[must_use]
    pub fn new(config: EcosystemStorageConfig) -> Self {
        let cache_config = super::cache::CacheConfig {
            max_size_bytes: config.cache_size_limit_bytes,
            ..Default::default()
        };

        Self {
            config,
            backends: Vec::new(),
            cache: Arc::new(RwLock::new(CacheManager::new(cache_config))),
            metrics: Arc::new(RwLock::new(StorageMetrics::default())),
        }
    }

    /// Process a storage request
    /// Processes request
    /// Processes request
    ///
    /// # Errors
    /// Returns an error if the storage operation fails
    pub async fn process_request(
        &self,
        request: StorageRequest,
    ) -> Result<StorageResponse, BearDogError> {
        match request.operation {
            StorageOperation::Store => self.handle_store(request).await,
            StorageOperation::Retrieve => self.handle_retrieve(request).await,
            StorageOperation::Delete => self.handle_delete(request).await,
            StorageOperation::List => self.handle_list(&request),
            _ => Err(BearDogError::business(
                "Operation not implemented".to_string(),
            )),
        }
    }

    /// Add a storage backend
    pub fn add_backend(&mut self, backend: Arc<dyn StorageBackend>) {
        self.backends.push(backend);
    }

    /// Get storage metrics
    /// Gets metrics
    /// Gets metrics
    pub async fn get_metrics(&self) -> StorageMetrics {
        self.metrics.read().await.clone()
    }

    /// Get configuration
    #[must_use]
    pub const fn config(&self) -> &EcosystemStorageConfig {
        &self.config
    }

    /// Handles store
    async fn handle_store(&self, request: StorageRequest) -> Result<StorageResponse, BearDogError> {
        // Try to store in the first available backend
        if let Some(backend) = self.backends.first() {
            let response = backend.store(request.clone())?;

            // Cache the stored data if successful
            if response.status == StorageStatus::Success
                && let Some(data) = &request.data
            {
                let mut cache = self.cache.write().await;
                cache.put(request.key, data.clone());
            }

            Ok(response)
        } else {
            Err(BearDogError::business(NO_BACKEND_AVAILABLE.to_string()))
        }
    }

    /// Handles retrieve
    async fn handle_retrieve(
        &self,
        request: StorageRequest,
    ) -> Result<StorageResponse, BearDogError> {
        // Try cache first
        {
            let mut cache = self.cache.write().await;
            if let Some(data) = cache.get(&request.key) {
                // Convert Arc<Vec<u8>> back to Vec<u8> for response compatibility
                return Ok(StorageResponse::success(
                    request.request_id,
                    Some((*data).clone()),
                ));
            }
        }

        // Try backends
        if let Some(backend) = self.backends.first() {
            let response = backend.retrieve(request.clone())?;

            // Cache the retrieved data if successful
            if response.status == StorageStatus::Success
                && let Some(data) = &response.data
            {
                let mut cache = self.cache.write().await;
                cache.put(request.key, data.clone());
            }

            Ok(response)
        } else {
            Err(BearDogError::business(NO_BACKEND_AVAILABLE.to_string()))
        }
    }

    /// Handles delete
    async fn handle_delete(
        &self,
        request: StorageRequest,
    ) -> Result<StorageResponse, BearDogError> {
        // Remove from cache
        {
            let mut cache = self.cache.write().await;
            cache.remove(&request.key);
        }

        // Delete from backend
        self.backends.first().map_or_else(
            || Err(BearDogError::business(NO_BACKEND_AVAILABLE.to_string())),
            |backend| backend.delete(request),
        )
    }

    /// Handles list
    fn handle_list(&self, request: &StorageRequest) -> Result<StorageResponse, BearDogError> {
        if let Some(backend) = self.backends.first() {
            let items = backend.list(request.clone())?;
            let response_data = serde_json::to_vec(&items)
                .map_err(|e| BearDogError::business(format!("Serialization error: {e}")))?;

            Ok(StorageResponse::success(
                request.request_id,
                Some(response_data),
            ))
        } else {
            Err(BearDogError::business(NO_BACKEND_AVAILABLE.to_string()))
        }
    }
}
