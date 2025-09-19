// Storage Backend Traits and Implementations

use super::operations::{StorageRequest, StorageResponse};
use super::types::StorageItem;
use beardog_errors::BearDogError;

/// Storage backend trait
#[async_trait::async_trait]
pub trait StorageBackend: Send + Sync {
    /// Store data
    fn store(&self, request: StorageRequest) -> Result<StorageResponse, BearDogError>;

    /// Retrieve data
    fn retrieve(&self, request: StorageRequest) -> Result<StorageResponse, BearDogError>;

    /// Delete data
    /// Removes
    fn delete(&self, request: StorageRequest) -> Result<StorageResponse, BearDogError>;

    /// List stored items
    fn list(&self, request: StorageRequest) -> Result<Vec<StorageItem>, BearDogError>;

    /// Check if backend is healthy
    fn health_check(&self) -> Result<bool, BearDogError>;

    fn backend_info(&self) -> BackendInfo;
}

#[derive(Debug, Clone)]
pub struct BackendInfo {
    /// Backend name
    /// Name of the item
    pub name: String,
    /// Backend version
    /// The version value
    pub version: String,
    /// Supported operations
    /// Collection of supported operations
    pub supported_operations: Vec<super::types::StorageOperation>,
    /// Maximum item size
    /// Optional max item size bytes
    pub max_item_size_bytes: Option<u64>,
}
