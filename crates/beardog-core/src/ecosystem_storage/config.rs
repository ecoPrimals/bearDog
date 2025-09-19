// Configuration for Ecosystem Storage System

use super::types::{CacheEvictionPolicy, StorageType};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Ecosystem storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemStorageConfig {
    /// Storage identifier
    pub storage_id: String,
    /// Primary storage path
    /// The primary storage path value
    pub primary_storage_path: PathBuf,
    /// Backup storage paths
    /// Collection of backup storage paths
    pub backup_storage_paths: Vec<PathBuf>,
    /// Cache storage path
    /// The cache storage path value
    pub cache_storage_path: PathBuf,
    /// Maximum storage size in bytes
    /// Number of max_storage_size_bytes
    pub max_storage_size_bytes: u64,
    /// Cache size limit in bytes
    /// Number of cache_size_limit_bytes
    pub cache_size_limit_bytes: u64,
    /// Enable data compression
    /// Whether enable_compression is enabled
    pub enable_compression: bool,
    /// Enable encryption at rest
    /// Whether enable_encryption is enabled
    pub enable_encryption: bool,
    /// Replication factor
    /// Number of replication_factor
    pub replication_factor: u32,
    /// Backup interval in seconds
    /// Number of backup_interval_secs
    pub backup_interval_secs: u64,
    /// Cache eviction policy
    /// The cache eviction policy value
    pub cache_eviction_policy: CacheEvictionPolicy,
    /// Storage type
    /// The storage type value
    pub storage_type: StorageType,
}

impl Default for EcosystemStorageConfig {
    fn default() -> Self {
        Self {
            storage_id: "ecosystem-storage".to_string(),
            primary_storage_path: PathBuf::from("./storage/primary"),
            backup_storage_paths: vec![PathBuf::from("./storage/backup")],
            cache_storage_path: PathBuf::from("./storage/cache"),
            max_storage_size_bytes: 1024 * 1024 * 1024 * 100, // 100GB
            cache_size_limit_bytes: 1024 * 1024 * 1024,       // 1GB
            enable_compression: true,
            enable_encryption: true,
            replication_factor: 3,
            backup_interval_secs: 3600, // 1 hour
            cache_eviction_policy: CacheEvictionPolicy::Lru,
            storage_type: StorageType::LocalFilesystem,
        }
    }
}
