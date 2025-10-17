// Storage types for ecosystem integration

use beardog_types::canonical::HealthStatus;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Storage types supported
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
/// Types of storage
pub enum StorageType {
    /// Local filesystem storage
    LocalFilesystem,
    /// Distributed filesystem
    DistributedFilesystem,
    /// Object storage (S3-compatible)
    ObjectStorage,
    /// Database storage
    Database,
    /// In-memory storage
    InMemory,
    /// Hybrid storage (multiple backends)
    Hybrid,
}

/// Cache eviction policies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CacheEvictionPolicy {
    /// Least Recently Used
    Lru,
    /// Least Frequently Used
    Lfu,
    /// First In First Out
    Fifo,
    /// Time-based expiration
    TimeToLive,
    /// Custom policy
    Custom(String),
}

/// Storage operation types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageOperation {
    /// Store data
    Store,
    /// Retrieve data
    Retrieve,
    /// Delete data
    Delete,
    /// List stored items
    List,
    /// Copy data
    Copy,
    /// Move data
    Move,
    /// Backup data
    Backup,
    /// Restore data
    Restore,
}

/// Storage operation status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageStatus {
    /// Operation pending
    Pending,
    /// Operation in progress
    InProgress,
    /// Operation completed successfully
    Success,
    /// Operation failed
    Failed,
    /// Operation cancelled
    Cancelled,
    /// Operation timed out
    Timeout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageLocationInfo {
    /// Location identifier
    pub location_id: String,
    /// Storage backend type
    /// The backend type value
    pub backend_type: StorageType,
    /// Location path or endpoint
    /// The path value
    pub path: String,
    /// Available space in bytes
    /// Number of `available_space_bytes`
    pub available_space_bytes: u64,
    /// Used space in bytes
    /// Number of `used_space_bytes`
    pub used_space_bytes: u64,
    /// Location health status
    /// Current status of the health
    pub health_status: HealthStatus,
    /// Last health check timestamp
    /// The last health check value
    pub last_health_check: chrono::DateTime<Utc>,
}

/// Storage operation metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageOperationMetrics {
    /// Operation type
    /// The operation value
    pub operation: StorageOperation,
    /// Total operations count
    /// Number of `total_operations`
    pub total_operations: u64,
    /// Successful operations count
    /// Number of `successful_operations`
    pub successful_operations: u64,
    /// Failed operations count
    /// Number of `failed_operations`
    pub failed_operations: u64,
    /// Average operation duration in milliseconds
    /// The avg duration ms value
    pub avg_duration_ms: f64,
    /// Total bytes processed
    /// Number of `total_bytes_processed`
    pub total_bytes_processed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageItem {
    /// Item identifier
    pub item_id: String,
    /// Item key/path
    /// The key value
    pub key: String,
    /// Item size in bytes
    /// Number of `size_bytes`
    pub size_bytes: u64,
    /// Content type/MIME type
    /// Optional content type
    pub content_type: Option<String>,
    /// Creation timestamp
    /// The created at value
    pub created_at: chrono::DateTime<Utc>,
    /// Last modified timestamp
    /// The modified at value
    pub modified_at: chrono::DateTime<Utc>,
    /// Last accessed timestamp
    /// Optional accessed at
    pub accessed_at: Option<chrono::DateTime<Utc>>,
    /// Item metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    /// Storage locations where item is stored
    /// Collection of locations
    pub locations: Vec<String>,
    /// Optional checksum
    pub checksum: Option<String>,
}

/// Replication health status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReplicationHealth {
    /// All replicas are healthy
    Healthy,
    /// Some replicas are degraded
    Degraded,
    /// Critical replication issues
    Critical,
    /// Replication is offline
    Offline,
}

#[derive(Debug, Clone)]
pub struct CacheEntry {
    /// Cache key
    /// The key value
    pub key: String,
    /// The data value
    pub data: std::sync::Arc<Vec<u8>>,
    /// Entry creation time
    /// The created at value
    pub created_at: chrono::DateTime<Utc>,
    /// Last access time
    /// The last accessed value
    pub last_accessed: chrono::DateTime<Utc>,
    /// Access count
    /// Number of access
    pub access_count: u64,
    /// Entry size in bytes
    /// Number of `size_bytes`
    pub size_bytes: u64,
    /// Time-to-live (optional)
    /// Optional ttl
    pub ttl: Option<chrono::DateTime<Utc>>,
}

/// Ecosystem storage operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EcosystemStorageOperation {
    Store { key: String },
    Retrieve { key: String },
    Delete { key: String },
    List,
}

/// Ecosystem storage request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemStorageRequest {
    /// Operation to perform
    pub operation: EcosystemStorageOperation,
    /// Data payload
    pub data: Vec<u8>,
}

impl EcosystemStorageRequest {
    #[must_use]
    pub const fn new(operation: EcosystemStorageOperation, data: Vec<u8>) -> Self {
        Self { operation, data }
    }
}

// MIGRATION NOTE: These constants are still in use in manager.rs
// They will be migrated when manager.rs is updated to use canonical constants
// Use: beardog_types::constants::domains::storage::*
#[deprecated(
    since = "3.6.0",
    note = "Use beardog_types::constants::domains::storage::messages::NO_BACKEND_AVAILABLE"
)]
pub const STORAGE_BACKEND_AVAILABLE: &str = "No storage backend available";
