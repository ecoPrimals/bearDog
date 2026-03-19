// SPDX-License-Identifier: AGPL-3.0-only

//! Tests for ecosystem storage metrics

use crate::ecosystem_storage::metrics::StorageMetrics;
use crate::ecosystem_storage::types::StorageOperationMetrics;
use std::collections::HashMap;

#[test]
fn test_metrics_default() {
    let metrics = StorageMetrics::default();
    assert_eq!(metrics.total_capacity_bytes, 0);
    assert_eq!(metrics.used_space_bytes, 0);
    assert_eq!(metrics.available_space_bytes, 0);
    assert_eq!(metrics.utilization_percentage, 0.0);
    assert_eq!(metrics.total_items, 0);
}

#[test]
fn test_metrics_default_empty_operations() {
    let metrics = StorageMetrics::default();
    assert!(metrics.operation_metrics.is_empty());
}

#[test]
fn test_metrics_clone() {
    let mut metrics = StorageMetrics::default();
    metrics.total_capacity_bytes = 1_000_000_000;
    metrics.used_space_bytes = 500_000_000;
    metrics.total_items = 1000;

    let cloned = metrics.clone();
    assert_eq!(metrics.total_capacity_bytes, cloned.total_capacity_bytes);
    assert_eq!(metrics.used_space_bytes, cloned.used_space_bytes);
    assert_eq!(metrics.total_items, cloned.total_items);
}

#[test]
fn test_metrics_serialization() {
    let mut metrics = StorageMetrics::default();
    metrics.total_capacity_bytes = 100_000_000;
    metrics.used_space_bytes = 50_000_000;
    metrics.available_space_bytes = 50_000_000;
    metrics.utilization_percentage = 0.5;
    metrics.total_items = 500;

    let serialized = serde_json::to_string(&metrics).expect("serialize");
    assert!(serialized.contains("100000000"));

    let deserialized: StorageMetrics = serde_json::from_str(&serialized).expect("deserialize");
    assert_eq!(
        metrics.total_capacity_bytes,
        deserialized.total_capacity_bytes
    );
}

#[test]
fn test_metrics_with_operations() {
    use crate::ecosystem_storage::types::StorageOperation;

    let mut metrics = StorageMetrics::default();

    let read_ops = StorageOperationMetrics {
        operation: StorageOperation::Retrieve,
        total_operations: 1000,
        successful_operations: 995,
        failed_operations: 5,
        avg_duration_ms: 10.5,
        total_bytes_processed: 1_000_000,
    };

    let write_ops = StorageOperationMetrics {
        operation: StorageOperation::Store,
        total_operations: 500,
        successful_operations: 498,
        failed_operations: 2,
        avg_duration_ms: 25.0,
        total_bytes_processed: 500_000,
    };

    metrics
        .operation_metrics
        .insert("read".to_string(), read_ops);
    metrics
        .operation_metrics
        .insert("write".to_string(), write_ops);

    assert_eq!(metrics.operation_metrics.len(), 2);
    assert!(metrics.operation_metrics.contains_key("read"));
    assert!(metrics.operation_metrics.contains_key("write"));

    let read = metrics.operation_metrics.get("read").expect("read ops");
    assert_eq!(read.total_operations, 1000);
    assert_eq!(read.successful_operations, 995);
}

#[test]
fn test_metrics_utilization_calculation() {
    let metrics = StorageMetrics {
        total_capacity_bytes: 1_000_000_000,
        used_space_bytes: 750_000_000,
        available_space_bytes: 250_000_000,
        utilization_percentage: 0.75,
        operation_metrics: HashMap::new(),
        total_items: 10000,
        avg_item_size_bytes: 75000.0,
        timestamp: chrono::Utc::now(),
    };

    assert_eq!(metrics.utilization_percentage, 0.75);
    assert_eq!(
        metrics.total_capacity_bytes,
        metrics.used_space_bytes + metrics.available_space_bytes
    );
}

#[test]
fn test_metrics_timestamp() {
    let before = chrono::Utc::now();
    let metrics = StorageMetrics::default();
    let after = chrono::Utc::now();

    assert!(metrics.timestamp >= before);
    assert!(metrics.timestamp <= after);
}
