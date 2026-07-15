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

// Tests for storage operations

use crate::ecosystem_storage::operations::{StorageRequest, StorageResponse};
use crate::ecosystem_storage::types::{StorageOperation, StorageStatus};
use chrono::Utc;
use std::collections::BTreeMap;
use uuid::Uuid;

#[test]
fn test_storage_request_new() {
    let request = StorageRequest::new(StorageOperation::Store, "test-key".to_string());

    assert_eq!(request.operation, StorageOperation::Store);
    assert_eq!(request.key, "test-key");
    assert!(request.data.is_none());
    assert!(request.metadata.is_empty());
    assert!(request.timeout_secs.is_none());
}

#[test]
fn test_storage_request_store() {
    let data = vec![1, 2, 3, 4, 5];
    let request = StorageRequest::store("store-key".to_string(), data.clone());

    assert_eq!(request.operation, StorageOperation::Store);
    assert_eq!(request.key, "store-key");
    assert_eq!(request.data, Some(data));
}

#[test]
fn test_storage_request_retrieve() {
    let request = StorageRequest::retrieve("retrieve-key".to_string());

    assert_eq!(request.operation, StorageOperation::Retrieve);
    assert_eq!(request.key, "retrieve-key");
    assert!(request.data.is_none());
}

#[test]
fn test_storage_request_delete() {
    let request = StorageRequest::delete("delete-key".to_string());

    assert_eq!(request.operation, StorageOperation::Delete);
    assert_eq!(request.key, "delete-key");
    assert!(request.data.is_none());
}

#[test]
fn test_storage_request_list() {
    let request = StorageRequest::list("prefix/".to_string());

    assert_eq!(request.operation, StorageOperation::List);
    assert_eq!(request.key, "prefix/");
}

#[test]
fn test_storage_request_serialization() {
    let request = StorageRequest {
        request_id: Uuid::new_v4(),
        operation: StorageOperation::Store,
        key: "test-key".to_string(),
        data: Some(vec![1, 2, 3]),
        metadata: BTreeMap::new(),
        timestamp: Utc::now(),
        timeout_secs: Some(30),
        source_key: None,
        destination_key: None,
    };

    let serialized = serde_json::to_string(&request).unwrap();
    let deserialized: StorageRequest = serde_json::from_str(&serialized).unwrap();

    assert_eq!(request.request_id, deserialized.request_id);
    assert_eq!(request.operation, deserialized.operation);
    assert_eq!(request.key, deserialized.key);
}

#[test]
fn test_storage_request_with_metadata() {
    let mut metadata = BTreeMap::new();
    metadata.insert("content-type".to_string(), "application/json".to_string());
    metadata.insert("author".to_string(), "test-user".to_string());

    let mut request = StorageRequest::new(StorageOperation::Store, "key".to_string());
    request.metadata = metadata.clone();

    assert_eq!(request.metadata.len(), 2);
    assert_eq!(
        request.metadata.get("content-type"),
        Some(&"application/json".to_string())
    );
}

#[test]
fn test_storage_request_with_timeout() {
    let mut request = StorageRequest::new(StorageOperation::Retrieve, "key".to_string());
    request.timeout_secs = Some(60);

    assert_eq!(request.timeout_secs, Some(60));
}

#[test]
fn test_storage_request_with_source_destination() {
    let mut request = StorageRequest::new(StorageOperation::Copy, "key".to_string());
    request.source_key = Some("source".to_string());
    request.destination_key = Some("dest".to_string());

    assert_eq!(request.source_key, Some("source".to_string()));
    assert_eq!(request.destination_key, Some("dest".to_string()));
}

#[test]
fn test_storage_response_success() {
    let request_id = Uuid::new_v4();
    let data = vec![1, 2, 3, 4, 5];
    let response = StorageResponse::success(request_id, Some(data.clone()));

    assert_eq!(response.request_id, request_id);
    assert_eq!(response.status, StorageStatus::Success);
    assert_eq!(response.data, Some(data));
    assert!(response.error_message.is_none());
}

#[test]
fn test_storage_response_failure() {
    let request_id = Uuid::new_v4();
    let error_msg = "Storage backend unavailable".to_string();
    let response = StorageResponse::failure(request_id, error_msg.clone());

    assert_eq!(response.request_id, request_id);
    assert_eq!(response.status, StorageStatus::Failed);
    assert!(response.data.is_none());
    assert_eq!(response.error_message, Some(error_msg));
}

#[test]
fn test_storage_response_serialization() {
    let response = StorageResponse {
        request_id: Uuid::new_v4(),
        status: StorageStatus::Success,
        data: Some(vec![1, 2, 3]),
        metadata: BTreeMap::new(),
        timestamp: Utc::now(),
        duration_ms: 25,
        error_message: None,
        storage_location: Some("/storage/path".to_string()),
    };

    let serialized = serde_json::to_string(&response).unwrap();
    let deserialized: StorageResponse = serde_json::from_str(&serialized).unwrap();

    assert_eq!(response.request_id, deserialized.request_id);
    assert_eq!(response.status, deserialized.status);
    assert_eq!(response.duration_ms, deserialized.duration_ms);
}

#[test]
fn test_storage_response_with_metadata() {
    let mut metadata = BTreeMap::new();
    metadata.insert("checksum".to_string(), "abc123".to_string());
    metadata.insert("version".to_string(), "1".to_string());

    let request_id = Uuid::new_v4();
    let mut response = StorageResponse::success(request_id, None);
    response.metadata = metadata.clone();

    assert_eq!(response.metadata.len(), 2);
    assert_eq!(
        response.metadata.get("checksum"),
        Some(&"abc123".to_string())
    );
}

#[test]
fn test_storage_response_with_duration() {
    let request_id = Uuid::new_v4();
    let mut response = StorageResponse::success(request_id, None);
    response.duration_ms = 150;

    assert_eq!(response.duration_ms, 150);
}

#[test]
fn test_storage_response_with_location() {
    let request_id = Uuid::new_v4();
    let mut response = StorageResponse::success(request_id, None);
    response.storage_location = Some("/data/store/object-123".to_string());

    assert_eq!(
        response.storage_location,
        Some("/data/store/object-123".to_string())
    );
}

#[test]
fn test_request_response_lifecycle() {
    // Create a request
    let data = vec![1, 2, 3];
    let request = StorageRequest::store("lifecycle-key".to_string(), data.clone());
    let request_id = request.request_id;

    // Simulate successful response
    let response = StorageResponse::success(request_id, Some(data.clone()));

    // Verify IDs match
    assert_eq!(request.request_id, response.request_id);
    assert_eq!(response.status, StorageStatus::Success);
    assert_eq!(response.data, Some(data));
}

#[test]
fn test_multiple_operation_types() {
    let operations = vec![
        StorageRequest::store("key1".to_string(), vec![1]),
        StorageRequest::retrieve("key2".to_string()),
        StorageRequest::delete("key3".to_string()),
        StorageRequest::list("prefix/".to_string()),
    ];

    assert_eq!(operations[0].operation, StorageOperation::Store);
    assert_eq!(operations[1].operation, StorageOperation::Retrieve);
    assert_eq!(operations[2].operation, StorageOperation::Delete);
    assert_eq!(operations[3].operation, StorageOperation::List);
}

#[test]
fn test_storage_status_progression() {
    let request_id = Uuid::new_v4();

    // Create responses in different states
    let mut pending = StorageResponse::success(request_id, None);
    pending.status = StorageStatus::Pending;

    let mut in_progress = StorageResponse::success(request_id, None);
    in_progress.status = StorageStatus::InProgress;

    let success = StorageResponse::success(request_id, None);

    // Verify status progression
    assert_eq!(pending.status, StorageStatus::Pending);
    assert_eq!(in_progress.status, StorageStatus::InProgress);
    assert_eq!(success.status, StorageStatus::Success);
}

#[test]
fn test_error_conditions() {
    let request_id = Uuid::new_v4();

    let timeout = StorageResponse {
        request_id,
        status: StorageStatus::Timeout,
        data: None,
        metadata: BTreeMap::new(),
        timestamp: Utc::now(),
        duration_ms: 5000,
        error_message: Some("Operation timed out".to_string()),
        storage_location: None,
    };

    let cancelled = StorageResponse {
        request_id,
        status: StorageStatus::Cancelled,
        data: None,
        metadata: BTreeMap::new(),
        timestamp: Utc::now(),
        duration_ms: 100,
        error_message: Some("Operation cancelled by user".to_string()),
        storage_location: None,
    };

    assert_eq!(timeout.status, StorageStatus::Timeout);
    assert_eq!(cancelled.status, StorageStatus::Cancelled);
}

#[test]
fn storage_request_json_roundtrip_preserves_shape() {
    let req = StorageRequest {
        request_id: Uuid::new_v4(),
        operation: StorageOperation::Copy,
        key: "k".to_string(),
        data: Some(vec![1, 2]),
        metadata: BTreeMap::from([("m".to_string(), "v".to_string())]),
        timestamp: Utc::now(),
        timeout_secs: Some(99),
        source_key: Some("a".to_string()),
        destination_key: Some("b".to_string()),
    };
    let v = serde_json::to_value(&req).expect("serialize request");
    let back: StorageRequest = serde_json::from_value(v.clone()).expect("deserialize request");
    let v2 = serde_json::to_value(&back).expect("re-serialize");
    assert_eq!(v, v2, "JSON roundtrip must be stable");
}

#[test]
fn storage_response_json_roundtrip_preserves_shape() {
    let resp = StorageResponse {
        request_id: Uuid::new_v4(),
        status: StorageStatus::InProgress,
        data: None,
        metadata: BTreeMap::new(),
        timestamp: Utc::now(),
        duration_ms: 42,
        error_message: None,
        storage_location: Some("loc".to_string()),
    };
    let v = serde_json::to_value(&resp).expect("serialize response");
    let back: StorageResponse = serde_json::from_value(v).expect("deserialize response");
    assert_eq!(
        serde_json::to_value(&back).expect("back"),
        serde_json::to_value(&resp).expect("orig")
    );
}

#[test]
fn storage_operation_all_variants_roundtrip_json() {
    for op in [
        StorageOperation::Store,
        StorageOperation::Retrieve,
        StorageOperation::Delete,
        StorageOperation::List,
        StorageOperation::Copy,
        StorageOperation::Move,
        StorageOperation::Backup,
        StorageOperation::Restore,
    ] {
        let v = serde_json::to_value(op).expect("ser op");
        let back: StorageOperation = serde_json::from_value(v).expect("de op");
        assert_eq!(back, op);
    }
}

#[test]
fn storage_status_all_variants_roundtrip_json() {
    for st in [
        StorageStatus::Pending,
        StorageStatus::InProgress,
        StorageStatus::Success,
        StorageStatus::Failed,
        StorageStatus::Cancelled,
        StorageStatus::Timeout,
    ] {
        let v = serde_json::to_value(st).expect("ser status");
        let back: StorageStatus = serde_json::from_value(v).expect("de status");
        assert_eq!(back, st);
    }
}

#[test]
fn storage_request_debug_includes_operation() {
    let r = StorageRequest::retrieve("key-a".to_string());
    let s = format!("{r:?}");
    assert!(
        s.contains("Retrieve"),
        "debug should mention operation: {s}"
    );
}

#[test]
fn storage_response_failure_includes_message_in_debug() {
    let r = StorageResponse::failure(Uuid::nil(), "boom".to_string());
    let s = format!("{r:?}");
    assert!(s.contains("boom"), "debug should include error: {s}");
}
