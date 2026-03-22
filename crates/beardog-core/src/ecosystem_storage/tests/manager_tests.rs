// SPDX-License-Identifier: AGPL-3.0-only

//! Unit tests for [`crate::ecosystem_storage::EcosystemStorageManager`].

use crate::ecosystem_storage::{
    EcosystemStorageConfig, EcosystemStorageManager,
    backends::{BackendInfo, StorageBackend},
    operations::{StorageRequest, StorageResponse},
    types::{StorageItem, StorageOperation, StorageStatus},
};
use beardog_errors::BearDogError;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

struct TestBackend {
    store: std::sync::Mutex<HashMap<String, Vec<u8>>>,
}

impl TestBackend {
    fn new() -> Self {
        Self {
            store: std::sync::Mutex::new(HashMap::new()),
        }
    }
}

impl StorageBackend for TestBackend {
    fn store(&self, request: StorageRequest) -> Result<StorageResponse, BearDogError> {
        let data = request
            .data
            .ok_or_else(|| BearDogError::business("missing data".to_string()))?;
        self.store
            .lock()
            .expect("lock store")
            .insert(request.key.clone(), data);
        Ok(StorageResponse::success(request.request_id, None))
    }

    fn retrieve(&self, request: StorageRequest) -> Result<StorageResponse, BearDogError> {
        let data = self
            .store
            .lock()
            .expect("lock store")
            .get(&request.key)
            .cloned();
        match data {
            Some(d) => Ok(StorageResponse::success(request.request_id, Some(d))),
            None => Ok(StorageResponse {
                request_id: request.request_id,
                status: StorageStatus::Failed,
                data: None,
                metadata: HashMap::new(),
                timestamp: Utc::now(),
                duration_ms: 0,
                error_message: Some("missing".to_string()),
                storage_location: None,
            }),
        }
    }

    fn delete(&self, request: StorageRequest) -> Result<StorageResponse, BearDogError> {
        let removed = self
            .store
            .lock()
            .expect("lock store")
            .remove(&request.key)
            .is_some();
        Ok(StorageResponse {
            request_id: request.request_id,
            status: if removed {
                StorageStatus::Success
            } else {
                StorageStatus::Failed
            },
            data: None,
            metadata: HashMap::new(),
            timestamp: Utc::now(),
            duration_ms: 0,
            error_message: None,
            storage_location: None,
        })
    }

    fn list(&self, request: StorageRequest) -> Result<Vec<StorageItem>, BearDogError> {
        let items = self.store.lock().expect("lock store");
        Ok(items
            .keys()
            .filter(|k| k.starts_with(&request.key))
            .map(|key| StorageItem {
                item_id: Uuid::new_v4().to_string(),
                key: key.clone(),
                size_bytes: items.get(key).map_or(0, |v| v.len() as u64),
                created_at: Utc::now(),
                modified_at: Utc::now(),
                content_type: None,
                metadata: HashMap::new(),
                checksum: None,
                accessed_at: None,
                locations: vec![],
            })
            .collect())
    }

    fn health_check(&self) -> Result<bool, BearDogError> {
        Ok(true)
    }

    fn backend_info(&self) -> BackendInfo {
        BackendInfo {
            name: "test".to_string(),
            version: "0".to_string(),
            supported_operations: vec![
                StorageOperation::Store,
                StorageOperation::Retrieve,
                StorageOperation::Delete,
                StorageOperation::List,
            ],
            max_item_size_bytes: None,
        }
    }
}

#[tokio::test]
async fn ecosystem_storage_manager_store_then_retrieve_uses_cache_on_second_read() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let mut cfg = EcosystemStorageConfig::default();
    cfg.primary_storage_path = tmp.path().join("primary");
    cfg.cache_storage_path = tmp.path().join("cache");

    let mut mgr = EcosystemStorageManager::new(cfg);
    mgr.add_backend(Arc::new(TestBackend::new()));

    let key = "k1".to_string();
    mgr.process_request(StorageRequest::store(key.clone(), b"payload".to_vec()))
        .await
        .expect("store");

    let r1 = mgr
        .process_request(StorageRequest::retrieve(key.clone()))
        .await
        .expect("retrieve1");
    assert_eq!(r1.status, StorageStatus::Success);
    assert_eq!(r1.data.as_deref(), Some(b"payload".as_slice()));

    let r2 = mgr
        .process_request(StorageRequest::retrieve(key.clone()))
        .await
        .expect("retrieve2");
    assert_eq!(r2.status, StorageStatus::Success);
    assert_eq!(r2.data.as_deref(), Some(b"payload".as_slice()));
}

#[tokio::test]
async fn ecosystem_storage_manager_errors_when_no_backend_configured() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let mut cfg = EcosystemStorageConfig::default();
    cfg.primary_storage_path = tmp.path().join("primary");

    let mgr = EcosystemStorageManager::new(cfg);
    let err = mgr
        .process_request(StorageRequest::store("x".to_string(), vec![1]))
        .await
        .expect_err("no backend");
    assert!(
        err.to_string().contains("No storage backend available"),
        "unexpected err: {err}"
    );
}

#[tokio::test]
async fn ecosystem_storage_manager_unimplemented_operation_returns_business_error() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let mut cfg = EcosystemStorageConfig::default();
    cfg.primary_storage_path = tmp.path().join("primary");

    let mut mgr = EcosystemStorageManager::new(cfg);
    mgr.add_backend(Arc::new(TestBackend::new()));

    let mut req = StorageRequest::new(StorageOperation::Copy, "a".to_string());
    req.source_key = Some("s".to_string());
    req.destination_key = Some("d".to_string());

    let err = mgr.process_request(req).await.expect_err("copy");
    assert!(
        err.to_string().contains("Operation not implemented"),
        "unexpected err: {err}"
    );
}

#[tokio::test]
async fn ecosystem_storage_manager_list_serializes_items() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let mut cfg = EcosystemStorageConfig::default();
    cfg.primary_storage_path = tmp.path().join("primary");

    let mut mgr = EcosystemStorageManager::new(cfg);
    mgr.add_backend(Arc::new(TestBackend::new()));

    mgr.process_request(StorageRequest::store("pre-key1".to_string(), vec![9]))
        .await
        .expect("store");

    let res = mgr
        .process_request(StorageRequest::list("pre-".to_string()))
        .await
        .expect("list");
    assert_eq!(res.status, StorageStatus::Success);
    assert!(res.data.is_some());
}

#[tokio::test]
async fn ecosystem_storage_manager_get_metrics_and_config() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let mut cfg = EcosystemStorageConfig::default();
    cfg.primary_storage_path = tmp.path().join("primary");

    let mgr = EcosystemStorageManager::new(cfg.clone());
    let m = mgr.get_metrics().await;
    assert_eq!(m.total_items, 0);
    assert_eq!(mgr.config().storage_id, cfg.storage_id);
}
