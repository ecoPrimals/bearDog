//! Tests for ecosystem storage configuration

use crate::ecosystem_storage::config::EcosystemStorageConfig;
use crate::ecosystem_storage::types::{CacheEvictionPolicy, StorageType};
use std::path::PathBuf;

#[test]
fn test_config_default() {
    let config = EcosystemStorageConfig::default();
    assert!(!config.storage_id.is_empty());
    assert_eq!(config.storage_id, "ecosystem-storage");
}

#[test]
fn test_config_default_paths() {
    let config = EcosystemStorageConfig::default();
    assert_eq!(
        config.primary_storage_path,
        PathBuf::from("./storage/primary")
    );
    assert_eq!(config.cache_storage_path, PathBuf::from("./storage/cache"));
    assert!(!config.backup_storage_paths.is_empty());
}

#[test]
fn test_config_default_sizes() {
    let config = EcosystemStorageConfig::default();
    assert!(config.max_storage_size_bytes > 0);
    assert!(config.cache_size_limit_bytes > 0);
    assert!(config.max_storage_size_bytes > config.cache_size_limit_bytes);
}

#[test]
fn test_config_default_flags() {
    let config = EcosystemStorageConfig::default();
    assert!(config.enable_compression);
    assert!(config.enable_encryption);
}

#[test]
fn test_config_default_replication() {
    let config = EcosystemStorageConfig::default();
    assert!(config.replication_factor > 0);
    assert!(config.backup_interval_secs > 0);
}

#[test]
fn test_config_default_policies() {
    let config = EcosystemStorageConfig::default();
    assert_eq!(config.cache_eviction_policy, CacheEvictionPolicy::Lru);
    assert_eq!(config.storage_type, StorageType::LocalFilesystem);
}

#[test]
fn test_config_clone() {
    let config = EcosystemStorageConfig::default();
    let cloned = config.clone();
    assert_eq!(config.storage_id, cloned.storage_id);
    assert_eq!(config.primary_storage_path, cloned.primary_storage_path);
    assert_eq!(config.max_storage_size_bytes, cloned.max_storage_size_bytes);
}

#[test]
fn test_config_serialization() {
    let config = EcosystemStorageConfig::default();
    let serialized = serde_json::to_string(&config).expect("serialize");
    assert!(serialized.contains("ecosystem-storage"));

    let deserialized: EcosystemStorageConfig =
        serde_json::from_str(&serialized).expect("deserialize");
    assert_eq!(config.storage_id, deserialized.storage_id);
}

#[test]
fn test_config_custom_values() {
    let config = EcosystemStorageConfig {
        storage_id: "custom-storage".to_string(),
        primary_storage_path: PathBuf::from("/data/primary"),
        backup_storage_paths: vec![
            PathBuf::from("/data/backup1"),
            PathBuf::from("/data/backup2"),
        ],
        cache_storage_path: PathBuf::from("/data/cache"),
        max_storage_size_bytes: 500_000_000_000,
        cache_size_limit_bytes: 5_000_000_000,
        enable_compression: false,
        enable_encryption: true,
        replication_factor: 5,
        backup_interval_secs: 7200,
        cache_eviction_policy: CacheEvictionPolicy::Fifo,
        storage_type: StorageType::DistributedFilesystem,
    };

    assert_eq!(config.storage_id, "custom-storage");
    assert_eq!(config.backup_storage_paths.len(), 2);
    assert!(!config.enable_compression);
    assert_eq!(config.replication_factor, 5);
}
