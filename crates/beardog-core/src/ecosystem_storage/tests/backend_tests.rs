// SPDX-License-Identifier: AGPL-3.0-or-later

#![allow(
    unused_imports,
    clippy::float_cmp,
    clippy::useless_vec,
    clippy::needless_range_loop,
    clippy::uninlined_format_args,
    clippy::field_reassign_with_default,
    clippy::manual_range_contains,
    unused_variables,
    dead_code
)]

// Backend tests for ecosystem storage
//
// Tests for storage backend traits, implementations, and backend info.

use crate::ecosystem_storage::{
    backends::{BackendInfo, StorageBackend},
    operations::{StorageRequest, StorageResponse},
    types::{StorageItem, StorageOperation, StorageStatus},
};
use beardog_errors::BearDogError;
use chrono::Utc;
use std::collections::{BTreeMap, HashMap};
use uuid::Uuid;

// =============================================================================
// Mock Backend for Testing
// =============================================================================

/// Mock storage backend for testing
struct MockStorageBackend {
    name: String,
    should_fail: bool,
    stored_items: std::sync::Mutex<HashMap<String, Vec<u8>>>,
}

impl MockStorageBackend {
    fn new(name: String, should_fail: bool) -> Self {
        Self {
            name,
            should_fail,
            stored_items: std::sync::Mutex::new(HashMap::new()),
        }
    }
}

impl StorageBackend for MockStorageBackend {
    fn store(&self, request: StorageRequest) -> Result<StorageResponse, BearDogError> {
        if self.should_fail {
            return Err(BearDogError::internal("Mock store failure".to_string()));
        }

        if let Some(data) = &request.data {
            let mut items = self.stored_items.lock().expect("Failed to lock storage");
            items.insert(request.key.clone(), data.clone());
        }

        Ok(StorageResponse {
            request_id: request.request_id,
            status: StorageStatus::Success,
            data: None,
            metadata: BTreeMap::new(),
            timestamp: Utc::now(),
            duration_ms: 10,
            error_message: None,
            storage_location: Some(format!("mock://{}", request.key)),
        })
    }

    fn retrieve(&self, request: StorageRequest) -> Result<StorageResponse, BearDogError> {
        if self.should_fail {
            return Err(BearDogError::internal("Mock retrieve failure".to_string()));
        }

        let items = self.stored_items.lock().expect("Failed to lock storage");
        let data = items.get(&request.key).cloned();
        let found = data.is_some();

        Ok(StorageResponse {
            request_id: request.request_id,
            status: if found {
                StorageStatus::Success
            } else {
                StorageStatus::Failed
            },
            data,
            metadata: BTreeMap::new(),
            timestamp: Utc::now(),
            duration_ms: 5,
            error_message: if found {
                None
            } else {
                Some("Key not found".to_string())
            },
            storage_location: Some(format!("mock://{}", request.key)),
        })
    }

    fn delete(&self, request: StorageRequest) -> Result<StorageResponse, BearDogError> {
        if self.should_fail {
            return Err(BearDogError::internal("Mock delete failure".to_string()));
        }

        let mut items = self.stored_items.lock().expect("Failed to lock storage");
        let removed = items.remove(&request.key).is_some();

        Ok(StorageResponse {
            request_id: request.request_id,
            status: if removed {
                StorageStatus::Success
            } else {
                StorageStatus::Failed
            },
            data: None,
            metadata: BTreeMap::new(),
            timestamp: Utc::now(),
            duration_ms: 5,
            error_message: if removed {
                None
            } else {
                Some("Key not found".to_string())
            },
            storage_location: Some(format!("mock://{}", request.key)),
        })
    }

    fn list(&self, _request: StorageRequest) -> Result<Vec<StorageItem>, BearDogError> {
        if self.should_fail {
            return Err(BearDogError::internal("Mock list failure".to_string()));
        }

        let items = self.stored_items.lock().expect("Failed to lock storage");
        let storage_items: Vec<StorageItem> = items
            .keys()
            .map(|key| StorageItem {
                item_id: uuid::Uuid::new_v4().to_string(),
                key: key.clone(),
                size_bytes: items.get(key).map_or(0, |v| v.len() as u64),
                created_at: Utc::now(),
                modified_at: Utc::now(),
                content_type: Some("application/octet-stream".to_string()),
                metadata: HashMap::new(),
                checksum: None,
                accessed_at: None,
                locations: vec![format!("mock://{}", key)],
            })
            .collect();

        Ok(storage_items)
    }

    fn health_check(&self) -> Result<bool, BearDogError> {
        if self.should_fail {
            return Err(BearDogError::internal(
                "Mock health check failure".to_string(),
            ));
        }
        Ok(true)
    }

    fn backend_info(&self) -> BackendInfo {
        BackendInfo {
            name: self.name.clone(),
            version: "1.0.0-mock".to_string(),
            supported_operations: vec![
                StorageOperation::Store,
                StorageOperation::Retrieve,
                StorageOperation::Delete,
                StorageOperation::List,
            ],
            max_item_size_bytes: Some(1024 * 1024), // 1 MB limit for mock
        }
    }
}

// =============================================================================
// BackendInfo Tests
// =============================================================================

#[test]
fn test_backend_info_creation() {
    let info = BackendInfo {
        name: "test-backend".to_string(),
        version: "1.0.0".to_string(),
        supported_operations: vec![StorageOperation::Store, StorageOperation::Retrieve],
        max_item_size_bytes: Some(1024),
    };

    assert_eq!(info.name, "test-backend");
    assert_eq!(info.version, "1.0.0");
    assert_eq!(info.supported_operations.len(), 2);
    assert_eq!(info.max_item_size_bytes, Some(1024));
}

#[test]
fn test_backend_info_unlimited_size() {
    let info = BackendInfo {
        name: "unlimited-backend".to_string(),
        version: "2.0.0".to_string(),
        supported_operations: vec![],
        max_item_size_bytes: None, // No size limit
    };

    assert_eq!(info.name, "unlimited-backend");
    assert!(
        info.max_item_size_bytes.is_none(),
        "Should have no size limit"
    );
}

#[test]
fn test_backend_info_all_operations() {
    let info = BackendInfo {
        name: "full-featured".to_string(),
        version: "3.0.0".to_string(),
        supported_operations: vec![
            StorageOperation::Store,
            StorageOperation::Retrieve,
            StorageOperation::Delete,
            StorageOperation::List,
            StorageOperation::Copy,
            StorageOperation::Move,
            StorageOperation::Backup,
            StorageOperation::Restore,
        ],
        max_item_size_bytes: Some(10 * 1024 * 1024), // 10 MB
    };

    assert_eq!(info.supported_operations.len(), 8);
    assert!(
        info.supported_operations
            .contains(&StorageOperation::Backup)
    );
}

// =============================================================================
// Mock Backend Tests
// =============================================================================

#[test]
fn test_mock_backend_store_and_retrieve() {
    let backend = MockStorageBackend::new("test".to_string(), false);

    let store_request = StorageRequest {
        request_id: Uuid::new_v4(),
        operation: StorageOperation::Store,
        key: "test-key".to_string(),
        data: Some(b"test-value".to_vec()),
        metadata: BTreeMap::new(),
        timestamp: Utc::now(),
        timeout_secs: None,
        source_key: None,
        destination_key: None,
    };

    let store_response = backend.store(store_request).expect("Store should succeed");
    assert_eq!(store_response.status, StorageStatus::Success);

    let retrieve_request = StorageRequest {
        request_id: Uuid::new_v4(),
        operation: StorageOperation::Retrieve,
        key: "test-key".to_string(),
        data: None,
        metadata: BTreeMap::new(),
        timestamp: Utc::now(),
        timeout_secs: None,
        source_key: None,
        destination_key: None,
    };

    let retrieve_response = backend
        .retrieve(retrieve_request)
        .expect("Retrieve should succeed");
    assert_eq!(retrieve_response.status, StorageStatus::Success);
    assert_eq!(retrieve_response.data, Some(b"test-value".to_vec()));
}

#[test]
fn test_mock_backend_retrieve_nonexistent() {
    let backend = MockStorageBackend::new("test".to_string(), false);

    let request = StorageRequest {
        request_id: Uuid::new_v4(),
        operation: StorageOperation::Retrieve,
        key: "nonexistent-key".to_string(),
        data: None,
        metadata: BTreeMap::new(),
        timestamp: Utc::now(),
        timeout_secs: None,
        source_key: None,
        destination_key: None,
    };

    let response = backend
        .retrieve(request)
        .expect("Retrieve should not error");
    assert_eq!(response.status, StorageStatus::Failed);
    assert!(response.error_message.is_some());
    assert!(response.data.is_none());
}

#[test]
fn test_mock_backend_delete() {
    let backend = MockStorageBackend::new("test".to_string(), false);

    // Store first
    let store_request = StorageRequest {
        request_id: Uuid::new_v4(),
        operation: StorageOperation::Store,
        key: "delete-test".to_string(),
        data: Some(b"delete-value".to_vec()),
        metadata: BTreeMap::new(),
        timestamp: Utc::now(),
        timeout_secs: None,
        source_key: None,
        destination_key: None,
    };
    backend.store(store_request).expect("Store should succeed");

    // Delete
    let delete_request = StorageRequest {
        request_id: Uuid::new_v4(),
        operation: StorageOperation::Delete,
        key: "delete-test".to_string(),
        data: None,
        metadata: BTreeMap::new(),
        timestamp: Utc::now(),
        timeout_secs: None,
        source_key: None,
        destination_key: None,
    };

    let delete_response = backend
        .delete(delete_request)
        .expect("Delete should succeed");
    assert_eq!(delete_response.status, StorageStatus::Success);

    // Verify deleted
    let retrieve_request = StorageRequest {
        request_id: Uuid::new_v4(),
        operation: StorageOperation::Retrieve,
        key: "delete-test".to_string(),
        data: None,
        metadata: BTreeMap::new(),
        timestamp: Utc::now(),
        timeout_secs: None,
        source_key: None,
        destination_key: None,
    };

    let retrieve_response = backend
        .retrieve(retrieve_request)
        .expect("Retrieve should succeed");
    assert_eq!(retrieve_response.status, StorageStatus::Failed);
}

#[test]
fn test_mock_backend_list() {
    let backend = MockStorageBackend::new("test".to_string(), false);

    // Store multiple items
    for i in 0..3 {
        let request = StorageRequest {
            request_id: Uuid::new_v4(),
            operation: StorageOperation::Store,
            key: format!("item-{}", i),
            data: Some(format!("value-{}", i).into_bytes()),
            metadata: BTreeMap::new(),
            timestamp: Utc::now(),
            timeout_secs: None,
            source_key: None,
            destination_key: None,
        };
        backend.store(request).expect("Store should succeed");
    }

    let list_request = StorageRequest {
        request_id: Uuid::new_v4(),
        operation: StorageOperation::List,
        key: String::new(),
        data: None,
        metadata: BTreeMap::new(),
        timestamp: Utc::now(),
        timeout_secs: None,
        source_key: None,
        destination_key: None,
    };

    let items = backend.list(list_request).expect("List should succeed");
    assert_eq!(items.len(), 3);
}

#[test]
fn test_mock_backend_health_check() {
    let healthy_backend = MockStorageBackend::new("healthy".to_string(), false);
    let result = healthy_backend.health_check();
    assert!(result.is_ok());
    assert!(result.expect("Should be healthy"));

    let unhealthy_backend = MockStorageBackend::new("unhealthy".to_string(), true);
    let result = unhealthy_backend.health_check();
    assert!(result.is_err());
}

#[test]
fn test_mock_backend_info() {
    let backend = MockStorageBackend::new("info-test".to_string(), false);
    let info = backend.backend_info();

    assert_eq!(info.name, "info-test");
    assert_eq!(info.version, "1.0.0-mock");
    assert_eq!(info.supported_operations.len(), 4);
    assert_eq!(info.max_item_size_bytes, Some(1024 * 1024));
}

#[test]
fn test_mock_backend_failure_modes() {
    let backend = MockStorageBackend::new("failing".to_string(), true);

    let request = StorageRequest {
        request_id: Uuid::new_v4(),
        operation: StorageOperation::Store,
        key: "test".to_string(),
        data: Some(b"data".to_vec()),
        metadata: BTreeMap::new(),
        timestamp: Utc::now(),
        timeout_secs: None,
        source_key: None,
        destination_key: None,
    };

    // All operations should fail
    assert!(backend.store(request.clone()).is_err());
    assert!(backend.retrieve(request.clone()).is_err());
    assert!(backend.delete(request.clone()).is_err());
    assert!(backend.list(request).is_err());
}

// =============================================================================
// Backend Trait Interface Tests
// =============================================================================

#[test]
fn test_backend_trait_thread_safety() {
    // This test verifies that StorageBackend is Send + Sync
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Box<dyn StorageBackend>>();
}

#[test]
fn test_storage_location_format() {
    let backend = MockStorageBackend::new("location-test".to_string(), false);

    let request = StorageRequest {
        request_id: Uuid::new_v4(),
        operation: StorageOperation::Store,
        key: "location-key".to_string(),
        data: Some(b"data".to_vec()),
        metadata: BTreeMap::new(),
        timestamp: Utc::now(),
        timeout_secs: None,
        source_key: None,
        destination_key: None,
    };

    let response = backend.store(request).expect("Store should succeed");
    assert!(response.storage_location.is_some());
    assert!(
        response
            .storage_location
            .expect("Location should exist")
            .starts_with("mock://")
    );
}
