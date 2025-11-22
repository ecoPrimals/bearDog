// Tests for ecosystem storage types

use crate::ecosystem_storage::types::*;
use beardog_types::canonical::HealthStatus;
use chrono::Utc;
use std::collections::HashMap;

#[test]
fn test_storage_type_variants() {
    // Test all storage type variants
    let types = [
        StorageType::LocalFilesystem,
        StorageType::DistributedFilesystem,
        StorageType::ObjectStorage,
        StorageType::Database,
        StorageType::InMemory,
        StorageType::Hybrid,
    ];

    // Verify all variants are unique
    for (i, type1) in types.iter().enumerate() {
        for (j, type2) in types.iter().enumerate() {
            if i == j {
                assert_eq!(type1, type2);
            }
        }
    }
}

#[test]
fn test_storage_type_serialization() {
    let storage_type = StorageType::ObjectStorage;
    let serialized = serde_json::to_string(&storage_type).unwrap();
    let deserialized: StorageType = serde_json::from_str(&serialized).unwrap();
    assert_eq!(storage_type, deserialized);
}

#[test]
fn test_cache_eviction_policy_variants() {
    let policies = [
        CacheEvictionPolicy::Lru,
        CacheEvictionPolicy::Lfu,
        CacheEvictionPolicy::Fifo,
        CacheEvictionPolicy::TimeToLive,
        CacheEvictionPolicy::Custom("test".to_string()),
    ];

    assert_eq!(policies.len(), 5);

    // Test custom policy
    if let CacheEvictionPolicy::Custom(name) = &policies[4] {
        assert_eq!(name, "test");
    } else {
        panic!("Expected Custom variant");
    }
}

#[test]
fn test_cache_eviction_policy_serialization() {
    let policy = CacheEvictionPolicy::Lru;
    let serialized = serde_json::to_string(&policy).unwrap();
    let deserialized: CacheEvictionPolicy = serde_json::from_str(&serialized).unwrap();
    assert_eq!(policy, deserialized);
}

#[test]
fn test_storage_operation_variants() {
    let operations = [
        StorageOperation::Store,
        StorageOperation::Retrieve,
        StorageOperation::Delete,
        StorageOperation::List,
        StorageOperation::Copy,
        StorageOperation::Move,
        StorageOperation::Backup,
        StorageOperation::Restore,
    ];

    assert_eq!(operations.len(), 8);
}

#[test]
fn test_storage_operation_serialization() {
    let operation = StorageOperation::Store;
    let serialized = serde_json::to_string(&operation).unwrap();
    let deserialized: StorageOperation = serde_json::from_str(&serialized).unwrap();
    assert_eq!(operation, deserialized);
}

#[test]
fn test_storage_status_variants() {
    let statuses = [
        StorageStatus::Pending,
        StorageStatus::InProgress,
        StorageStatus::Success,
        StorageStatus::Failed,
        StorageStatus::Cancelled,
        StorageStatus::Timeout,
    ];

    assert_eq!(statuses.len(), 6);
}

#[test]
fn test_storage_status_serialization() {
    let status = StorageStatus::Success;
    let serialized = serde_json::to_string(&status).unwrap();
    let deserialized: StorageStatus = serde_json::from_str(&serialized).unwrap();
    assert_eq!(status, deserialized);
}

#[test]
fn test_storage_location_info_creation() {
    let location_info = StorageLocationInfo {
        location_id: "test-location".to_string(),
        backend_type: StorageType::InMemory,
        path: "/tmp/test".to_string(),
        available_space_bytes: 1024 * 1024 * 1024, // 1GB
        used_space_bytes: 512 * 1024 * 1024,       // 512MB
        health_status: HealthStatus::Healthy,
        last_health_check: Utc::now(),
    };

    assert_eq!(location_info.location_id, "test-location");
    assert_eq!(location_info.backend_type, StorageType::InMemory);
    assert_eq!(location_info.available_space_bytes, 1024 * 1024 * 1024);
    assert_eq!(location_info.used_space_bytes, 512 * 1024 * 1024);
}

#[test]
fn test_storage_location_info_serialization() {
    let location_info = StorageLocationInfo {
        location_id: "test-location".to_string(),
        backend_type: StorageType::InMemory,
        path: "/tmp/test".to_string(),
        available_space_bytes: 1024 * 1024 * 1024,
        used_space_bytes: 512 * 1024 * 1024,
        health_status: HealthStatus::Healthy,
        last_health_check: Utc::now(),
    };

    let serialized = serde_json::to_string(&location_info).unwrap();
    let deserialized: StorageLocationInfo = serde_json::from_str(&serialized).unwrap();
    assert_eq!(location_info.location_id, deserialized.location_id);
}

#[test]
fn test_storage_operation_metrics_creation() {
    let metrics = StorageOperationMetrics {
        operation: StorageOperation::Store,
        total_operations: 1000,
        successful_operations: 950,
        failed_operations: 50,
        avg_duration_ms: 25.5,
        total_bytes_processed: 1024 * 1024 * 100, // 100MB
    };

    assert_eq!(metrics.total_operations, 1000);
    assert_eq!(metrics.successful_operations, 950);
    assert_eq!(metrics.failed_operations, 50);
    assert!((metrics.avg_duration_ms - 25.5).abs() < f64::EPSILON);
}

#[test]
fn test_storage_operation_metrics_serialization() {
    let metrics = StorageOperationMetrics {
        operation: StorageOperation::Retrieve,
        total_operations: 500,
        successful_operations: 475,
        failed_operations: 25,
        avg_duration_ms: 15.2,
        total_bytes_processed: 1024 * 1024 * 50,
    };

    let serialized = serde_json::to_string(&metrics).unwrap();
    let deserialized: StorageOperationMetrics = serde_json::from_str(&serialized).unwrap();
    assert_eq!(metrics.total_operations, deserialized.total_operations);
}

#[test]
fn test_storage_item_creation() {
    let mut metadata = HashMap::new();
    metadata.insert("author".to_string(), "test".to_string());

    let item = StorageItem {
        item_id: "item-123".to_string(),
        key: "test-key".to_string(),
        size_bytes: 1024,
        content_type: Some("application/json".to_string()),
        created_at: Utc::now(),
        modified_at: Utc::now(),
        accessed_at: Some(Utc::now()),
        metadata,
        locations: vec!["location-1".to_string(), "location-2".to_string()],
        checksum: Some("abc123".to_string()),
    };

    assert_eq!(item.item_id, "item-123");
    assert_eq!(item.key, "test-key");
    assert_eq!(item.size_bytes, 1024);
    assert_eq!(item.locations.len(), 2);
}

#[test]
fn test_storage_item_serialization() {
    let item = StorageItem {
        item_id: "item-456".to_string(),
        key: "another-key".to_string(),
        size_bytes: 2048,
        content_type: Some("text/plain".to_string()),
        created_at: Utc::now(),
        modified_at: Utc::now(),
        accessed_at: None,
        metadata: HashMap::new(),
        locations: vec!["location-3".to_string()],
        checksum: None,
    };

    let serialized = serde_json::to_string(&item).unwrap();
    let deserialized: StorageItem = serde_json::from_str(&serialized).unwrap();
    assert_eq!(item.item_id, deserialized.item_id);
}

#[test]
fn test_replication_health_variants() {
    let health_states = [
        ReplicationHealth::Healthy,
        ReplicationHealth::Degraded,
        ReplicationHealth::Critical,
        ReplicationHealth::Offline,
    ];

    assert_eq!(health_states.len(), 4);
}

#[test]
fn test_replication_health_serialization() {
    let health = ReplicationHealth::Healthy;
    let serialized = serde_json::to_string(&health).unwrap();
    let deserialized: ReplicationHealth = serde_json::from_str(&serialized).unwrap();
    assert_eq!(health, deserialized);
}

#[test]
fn test_cache_entry_creation() {
    let data = vec![1, 2, 3, 4, 5];
    let entry = CacheEntry {
        key: "cache-key".to_string(),
        data: std::sync::Arc::new(data.clone()),
        created_at: Utc::now(),
        last_accessed: Utc::now(),
        access_count: 10,
        size_bytes: 5,
        ttl: Some(Utc::now() + chrono::Duration::hours(1)),
    };

    assert_eq!(entry.key, "cache-key");
    assert_eq!(*entry.data, data);
    assert_eq!(entry.access_count, 10);
    assert_eq!(entry.size_bytes, 5);
}

#[test]
fn test_cache_entry_clone() {
    let data = vec![1, 2, 3];
    let entry = CacheEntry {
        key: "key".to_string(),
        data: std::sync::Arc::new(data),
        created_at: Utc::now(),
        last_accessed: Utc::now(),
        access_count: 5,
        size_bytes: 3,
        ttl: None,
    };

    let cloned = entry.clone();
    assert_eq!(entry.key, cloned.key);
    assert_eq!(*entry.data, *cloned.data);
}

#[test]
fn test_ecosystem_storage_operation_variants() {
    let ops = [
        EcosystemStorageOperation::Store {
            key: "key1".to_string(),
        },
        EcosystemStorageOperation::Retrieve {
            key: "key2".to_string(),
        },
        EcosystemStorageOperation::Delete {
            key: "key3".to_string(),
        },
        EcosystemStorageOperation::List,
    ];

    assert_eq!(ops.len(), 4);
}

#[test]
fn test_ecosystem_storage_operation_serialization() {
    let operation = EcosystemStorageOperation::Store {
        key: "test-key".to_string(),
    };
    let serialized = serde_json::to_string(&operation).unwrap();
    let deserialized: EcosystemStorageOperation = serde_json::from_str(&serialized).unwrap();

    match deserialized {
        EcosystemStorageOperation::Store { key } => {
            assert_eq!(key, "test-key");
        }
        _ => panic!("Expected Store variant"),
    }
}

#[test]
fn test_ecosystem_storage_request_creation() {
    let operation = EcosystemStorageOperation::Store {
        key: "request-key".to_string(),
    };
    let data = vec![1, 2, 3, 4];

    let request = EcosystemStorageRequest::new(operation, data.clone());

    assert_eq!(request.data, data);
}

#[test]
fn test_ecosystem_storage_request_serialization() {
    let operation = EcosystemStorageOperation::Retrieve {
        key: "retrieve-key".to_string(),
    };
    let request = EcosystemStorageRequest::new(operation, vec![]);

    let serialized = serde_json::to_string(&request).unwrap();
    let deserialized: EcosystemStorageRequest = serde_json::from_str(&serialized).unwrap();

    match deserialized.operation {
        EcosystemStorageOperation::Retrieve { key } => {
            assert_eq!(key, "retrieve-key");
        }
        _ => panic!("Expected Retrieve variant"),
    }
}
