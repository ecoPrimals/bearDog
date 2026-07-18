// SPDX-License-Identifier: AGPL-3.0-or-later

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

/// Information about a storage location
///
/// Describes storage location details including backend type, capacity, usage, and health.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageLocationInfo {
    /// Unique location identifier
    pub location_id: String,
    /// Type of storage backend (memory, filesystem, distributed)
    pub backend_type: StorageType,
    /// Location path or connection endpoint
    pub path: String,
    /// Available storage space in bytes
    pub available_space_bytes: u64,
    /// Currently used storage space in bytes
    pub used_space_bytes: u64,
    /// Current health status of this location
    pub health_status: HealthStatus,
    /// Last health check timestamp
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

/// A stored item in the ecosystem storage system
///
/// Represents a single stored object with metadata, location info, and access tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageItem {
    /// Unique item identifier
    pub item_id: String,
    /// Storage key/path for this item
    pub key: String,
    /// Item size in bytes
    pub size_bytes: u64,
    /// Content type/MIME type (e.g., "application/json")
    pub content_type: Option<String>,
    /// When the item was created
    pub created_at: chrono::DateTime<Utc>,
    /// When the item was last modified
    pub modified_at: chrono::DateTime<Utc>,
    /// When the item was last accessed
    pub accessed_at: Option<chrono::DateTime<Utc>>,
    /// Additional item metadata
    pub metadata: HashMap<String, String>,
    /// Storage locations where this item is replicated
    pub locations: Vec<String>,
    /// Data integrity checksum (if available)
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

/// A cache entry with zero-copy data sharing
///
/// Represents cached data with access tracking, TTL, and efficient memory sharing via `Arc`.
#[derive(Debug, Clone)]
pub struct CacheEntry {
    /// Cache key identifier
    pub key: String,
    /// Cached data (zero-copy via Arc)
    pub data: std::sync::Arc<Vec<u8>>,
    /// When the entry was created
    pub created_at: chrono::DateTime<Utc>,
    /// Last time the entry was accessed
    pub last_accessed: chrono::DateTime<Utc>,
    /// Number of times this entry has been accessed
    pub access_count: u64,
    /// Entry size in bytes
    pub size_bytes: u64,
    /// Time-to-live expiration (if set)
    pub ttl: Option<chrono::DateTime<Utc>>,
}

/// Storage operations for ecosystem data management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EcosystemStorageOperation {
    /// Store data with given key
    Store {
        /// Storage key identifier
        key: String,
    },
    /// Retrieve data by key
    Retrieve {
        /// Storage key identifier
        key: String,
    },
    /// Delete data by key
    Delete {
        /// Storage key identifier
        key: String,
    },
    /// List all stored keys
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
    /// Creates a new storage request with the specified operation and data
    #[must_use]
    pub const fn new(operation: EcosystemStorageOperation, data: Vec<u8>) -> Self {
        Self { operation, data }
    }
}
