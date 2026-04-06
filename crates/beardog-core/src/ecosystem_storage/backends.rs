// SPDX-License-Identifier: AGPL-3.0-or-later

// Storage Backend Traits and Implementations

use super::operations::{StorageRequest, StorageResponse};
use super::types::StorageItem;
use beardog_errors::BearDogError;

/// Storage backend trait
#[async_trait::async_trait]
pub trait StorageBackend: Send + Sync {
    /// Store data
    ///
    /// # Errors
    /// Returns an error if the storage operation fails
    fn store(&self, request: StorageRequest) -> Result<StorageResponse, BearDogError>;

    /// Retrieve data
    ///
    /// # Errors
    /// Returns an error if the retrieval operation fails
    fn retrieve(&self, request: StorageRequest) -> Result<StorageResponse, BearDogError>;

    /// Delete data
    ///
    /// # Errors
    /// Returns an error if the deletion operation fails
    fn delete(&self, request: StorageRequest) -> Result<StorageResponse, BearDogError>;

    /// List stored items
    ///
    /// # Errors
    /// Returns an error if the listing operation fails
    fn list(&self, request: StorageRequest) -> Result<Vec<StorageItem>, BearDogError>;

    /// Check if backend is healthy
    ///
    /// # Errors
    /// Returns an error if the health check fails
    fn health_check(&self) -> Result<bool, BearDogError>;

    /// Returns information about this storage backend
    fn backend_info(&self) -> BackendInfo;
}

/// Information about a storage backend
///
/// Describes backend capabilities, version, supported operations, and limits.
#[derive(Debug, Clone)]
pub struct BackendInfo {
    /// Backend name (e.g., "memory", "filesystem", "distributed")
    pub name: String,
    /// Backend version string
    pub version: String,
    /// Operations supported by this backend
    pub supported_operations: Vec<super::types::StorageOperation>,
    /// Maximum item size in bytes (if limited)
    pub max_item_size_bytes: Option<u64>,
}
