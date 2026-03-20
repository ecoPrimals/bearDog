// SPDX-License-Identifier: AGPL-3.0-only

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
    /// Number of `max_storage_size_bytes`
    pub max_storage_size_bytes: u64,
    /// Cache size limit in bytes
    /// Number of `cache_size_limit_bytes`
    pub cache_size_limit_bytes: u64,
    /// Enable data compression
    /// Whether `enable_compression` is enabled
    pub enable_compression: bool,
    /// Enable encryption at rest
    /// Whether `enable_encryption` is enabled
    pub enable_encryption: bool,
    /// Replication factor
    /// Number of `replication_factor`
    pub replication_factor: u32,
    /// Backup interval in seconds
    /// Number of `backup_interval_secs`
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
            max_storage_size_bytes: beardog_errors::process_env::var(
                "BEARDOG_MAX_STORAGE_SIZE_BYTES",
            )
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1024 * 1024 * 1024 * 100), // 100GB default
            cache_size_limit_bytes: beardog_errors::process_env::var(
                "BEARDOG_CACHE_SIZE_LIMIT_BYTES",
            )
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1024 * 1024 * 1024), // 1GB default
            enable_compression: true,
            enable_encryption: true,
            replication_factor: beardog_errors::process_env::var("BEARDOG_REPLICATION_FACTOR")
                .ok()
                .and_then(|r| r.parse().ok())
                .unwrap_or(3),
            backup_interval_secs: beardog_errors::process_env::var("BEARDOG_BACKUP_INTERVAL_SECS")
                .ok()
                .and_then(|i| i.parse().ok())
                .unwrap_or(3600), // 1 hour default
            cache_eviction_policy: CacheEvictionPolicy::Lru,
            storage_type: StorageType::LocalFilesystem,
        }
    }
}
