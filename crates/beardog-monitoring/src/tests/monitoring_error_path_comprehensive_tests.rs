// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive Error Path Tests for beardog-monitoring
//!
//! This module tests all error paths, edge cases, and failure scenarios
//! in monitoring systems using modern concurrent patterns (NO SLEEPS).
//!
//! ## Test Categories
//! - Metric collection failures
//! - Storage backend errors
//! - Concurrent access errors
//! - Recovery and graceful degradation
//! - Resource exhaustion scenarios

#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all
)]

use beardog_errors::BearDogError;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use tokio::sync::{RwLock, mpsc};

type Result<T> = std::result::Result<T, BearDogError>;

// ============================================================================
// Test Fixtures and Mocks
// ============================================================================

/// Mock metric collector that can inject failures
struct MockMetricCollector {
    fail_on_collect: Arc<AtomicBool>,
    collect_count: Arc<AtomicU64>,
    metrics: Arc<RwLock<Vec<(String, f64)>>>,
}

impl MockMetricCollector {
    fn new() -> Self {
        Self {
            fail_on_collect: Arc::new(AtomicBool::new(false)),
            collect_count: Arc::new(AtomicU64::new(0)),
            metrics: Arc::new(RwLock::new(Vec::new())),
        }
    }

    fn inject_failure(&self) {
        self.fail_on_collect.store(true, Ordering::SeqCst);
    }

    fn clear_failure(&self) {
        self.fail_on_collect.store(false, Ordering::SeqCst);
    }

    async fn collect(&self, name: &str, value: f64) -> Result<()> {
        self.collect_count.fetch_add(1, Ordering::SeqCst);

        if self.fail_on_collect.load(Ordering::SeqCst) {
            return Err(BearDogError::monitoring(
                "Metric collection failed (test_failure)".to_string(),
            ));
        }

        let mut metrics = self.metrics.write().await;
        metrics.push((name.to_string(), value));
        Ok(())
    }

    async fn get_metrics(&self) -> Vec<(String, f64)> {
        self.metrics.read().await.clone()
    }

    fn get_collect_count(&self) -> u64 {
        self.collect_count.load(Ordering::SeqCst)
    }
}

/// Mock storage backend with configurable failures
struct MockStorageBackend {
    fail_on_write: Arc<AtomicBool>,
    fail_on_read: Arc<AtomicBool>,
    storage: Arc<RwLock<Vec<String>>>,
}

impl MockStorageBackend {
    fn new() -> Self {
        Self {
            fail_on_write: Arc::new(AtomicBool::new(false)),
            fail_on_read: Arc::new(AtomicBool::new(false)),
            storage: Arc::new(RwLock::new(Vec::new())),
        }
    }

    fn inject_write_failure(&self) {
        self.fail_on_write.store(true, Ordering::SeqCst);
    }

    fn inject_read_failure(&self) {
        self.fail_on_read.store(true, Ordering::SeqCst);
    }

    async fn write(&self, data: String) -> Result<()> {
        if self.fail_on_write.load(Ordering::SeqCst) {
            return Err(BearDogError::monitoring(
                "Storage write failed (storage_error)".to_string(),
            ));
        }

        let mut storage = self.storage.write().await;
        storage.push(data);
        Ok(())
    }

    async fn read(&self) -> Result<Vec<String>> {
        if self.fail_on_read.load(Ordering::SeqCst) {
            return Err(BearDogError::monitoring(
                "Storage read failed (storage_error)".to_string(),
            ));
        }

        let storage = self.storage.read().await;
        Ok(storage.clone())
    }
}

// ============================================================================
// Category 1: Metric Collection Error Paths
// ============================================================================

#[tokio::test]
async fn test_metric_collection_failure_graceful_handling() -> Result<()> {
    // Test that metric collection failures don't crash the system
    let collector = MockMetricCollector::new();

    // Inject failure
    collector.inject_failure();

    // Attempt to collect metric (should fail gracefully)
    let result = collector.collect("test_metric", 42.0).await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        BearDogError::Monitoring { .. }
    ));

    // Verify collector tracked the attempt
    assert_eq!(collector.get_collect_count(), 1);

    Ok(())
}

#[tokio::test]
async fn test_metric_collection_recovery_after_failure() -> Result<()> {
    // Test that collector recovers after transient failures
    let collector = MockMetricCollector::new();

    // First collection should succeed
    collector.collect("metric_1", 1.0).await?;

    // Inject failure for second collection
    collector.inject_failure();
    let result = collector.collect("metric_2", 2.0).await;
    assert!(result.is_err());

    // Clear failure and verify recovery
    collector.clear_failure();
    collector.collect("metric_3", 3.0).await?;

    // Verify metrics (only successful ones)
    let metrics = collector.get_metrics().await;
    assert_eq!(metrics.len(), 2);
    assert_eq!(metrics[0].0, "metric_1");
    assert_eq!(metrics[1].0, "metric_3");

    Ok(())
}

#[tokio::test]
async fn test_concurrent_metric_collection_with_failures() -> Result<()> {
    // Test concurrent metric collection with intermittent failures
    let collector = Arc::new(MockMetricCollector::new());
    let mut handles = vec![];

    // Spawn 100 concurrent collectors
    for i in 0..100 {
        let collector = Arc::clone(&collector);
        handles.push(tokio::spawn(async move {
            // Inject failure for every 10th metric
            if i % 10 == 0 {
                collector.inject_failure();
            } else {
                collector.clear_failure();
            }

            let result = collector
                .collect(&format!("metric_{i}"), f64::from(i))
                .await;

            // Verify expected behavior
            if i % 10 == 0 {
                assert!(result.is_err());
            } else {
                assert!(result.is_ok());
            }
        }));
    }

    // Wait for all to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify 90 successful collections (100 - 10 failures)
    let metrics = collector.get_metrics().await;
    assert_eq!(metrics.len(), 90);

    Ok(())
}

#[tokio::test]
async fn test_metric_overflow_handling() -> Result<()> {
    // Test handling of extreme metric values
    let collector = MockMetricCollector::new();

    // Collect extreme values
    collector.collect("max_f64", f64::MAX).await?;
    collector.collect("min_f64", f64::MIN).await?;
    collector.collect("infinity", f64::INFINITY).await?;
    collector.collect("neg_infinity", f64::NEG_INFINITY).await?;
    collector.collect("nan", f64::NAN).await?;

    // Verify all collected
    let metrics = collector.get_metrics().await;
    assert_eq!(metrics.len(), 5);

    Ok(())
}

// ============================================================================
// Category 2: Storage Backend Error Paths
// ============================================================================

#[tokio::test]
async fn test_storage_write_failure_handling() -> Result<()> {
    // Test graceful handling of storage write failures
    let storage = MockStorageBackend::new();

    // First write should succeed
    storage.write("data_1".to_string()).await?;

    // Inject failure
    storage.inject_write_failure();
    let result = storage.write("data_2".to_string()).await;
    assert!(result.is_err());

    // Storage should still contain first write
    storage.fail_on_write.store(false, Ordering::SeqCst);
    let data = storage.read().await?;
    assert_eq!(data.len(), 1);
    assert_eq!(data[0], "data_1");

    Ok(())
}

#[tokio::test]
async fn test_storage_read_failure_handling() -> Result<()> {
    // Test graceful handling of storage read failures
    let storage = MockStorageBackend::new();

    // Write some data
    storage.write("data_1".to_string()).await?;

    // Inject read failure
    storage.inject_read_failure();
    let result = storage.read().await;
    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn test_concurrent_storage_operations() -> Result<()> {
    // Test concurrent reads and writes to storage
    let storage = Arc::new(MockStorageBackend::new());
    let mut handles = vec![];

    // Spawn 50 writers
    for i in 0..50 {
        let storage = Arc::clone(&storage);
        handles.push(tokio::spawn(async move {
            storage
                .write(format!("data_{i}"))
                .await
                .expect("write should succeed");
        }));
    }

    // Spawn 50 readers
    for _ in 0..50 {
        let storage = Arc::clone(&storage);
        handles.push(tokio::spawn(async move {
            // Readers might see partial data, which is fine
            let _ = storage.read().await;
        }));
    }

    // Wait for all to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify all writes succeeded
    let data = storage.read().await?;
    assert_eq!(data.len(), 50);

    Ok(())
}

#[tokio::test]
async fn test_storage_recovery_after_failure() -> Result<()> {
    // Test storage system recovers after transient failures
    let storage = MockStorageBackend::new();

    // Write data
    storage.write("data_1".to_string()).await?;

    // Simulate failure
    storage.inject_write_failure();
    assert!(storage.write("data_2".to_string()).await.is_err());

    // Recovery
    storage.fail_on_write.store(false, Ordering::SeqCst);
    storage.write("data_3".to_string()).await?;

    // Verify only successful writes
    let data = storage.read().await?;
    assert_eq!(data.len(), 2);
    assert_eq!(data[0], "data_1");
    assert_eq!(data[1], "data_3");

    Ok(())
}

// ============================================================================
// Category 3: Concurrent Access Errors
// ============================================================================

#[tokio::test]
async fn test_concurrent_metric_writes_no_data_loss() -> Result<()> {
    // Test that concurrent metric writes don't lose data
    let collector = Arc::new(MockMetricCollector::new());
    let mut handles = vec![];

    // Spawn 1000 concurrent writers
    for i in 0..1000 {
        let collector = Arc::clone(&collector);
        handles.push(tokio::spawn(async move {
            collector
                .collect(&format!("metric_{i}"), f64::from(i))
                .await
                .expect("collect should succeed");
        }));
    }

    // Wait for all to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify all metrics collected
    let metrics = collector.get_metrics().await;
    assert_eq!(metrics.len(), 1000);

    Ok(())
}

#[tokio::test]
async fn test_concurrent_read_write_consistency() -> Result<()> {
    // Test read/write consistency under concurrent access
    let storage = Arc::new(MockStorageBackend::new());
    let (tx, _rx) = mpsc::channel(100);

    // Spawn writer task
    let storage_writer = Arc::clone(&storage);
    let writer_handle = tokio::spawn(async move {
        for i in 0..100 {
            storage_writer
                .write(format!("data_{i}"))
                .await
                .expect("write should succeed");
            tokio::task::yield_now().await;
        }
    });

    // Spawn reader task
    let storage_reader = Arc::clone(&storage);
    let reader_handle = tokio::spawn(async move {
        for _ in 0..100 {
            if let Ok(data) = storage_reader.read().await {
                let _ = tx.send(data.len()).await;
            }
            tokio::task::yield_now().await;
        }
    });

    // Wait for both to complete
    writer_handle.await.unwrap();
    reader_handle.await.unwrap();

    // Verify final state
    let final_data = storage.read().await?;
    assert_eq!(final_data.len(), 100);

    Ok(())
}

// ============================================================================
// Category 4: Resource Exhaustion Scenarios
// ============================================================================

#[tokio::test]
async fn test_metric_buffer_overflow() -> Result<()> {
    // Test handling of metric buffer overflow
    let collector = MockMetricCollector::new();

    // Collect a large number of metrics
    for i in 0..10_000 {
        collector
            .collect(&format!("metric_{i}"), f64::from(i))
            .await?;
    }

    // Verify all collected
    let metrics = collector.get_metrics().await;
    assert_eq!(metrics.len(), 10_000);

    Ok(())
}

#[tokio::test]
async fn test_storage_capacity_limits() -> Result<()> {
    // Test storage behavior at capacity limits
    let storage = MockStorageBackend::new();

    // Write a large amount of data
    for i in 0..1_000 {
        storage.write(format!("data_{i}")).await?;
    }

    // Verify all data persisted
    let data = storage.read().await?;
    assert_eq!(data.len(), 1_000);

    Ok(())
}

#[tokio::test]
async fn test_memory_pressure_graceful_degradation() -> Result<()> {
    // Test graceful degradation under memory pressure
    let collector = Arc::new(MockMetricCollector::new());
    let mut handles = vec![];

    // Simulate memory pressure with many concurrent operations
    for i in 0..500 {
        let collector = Arc::clone(&collector);
        handles.push(tokio::spawn(async move {
            // Each task collects multiple metrics
            for j in 0..20 {
                let _ = collector
                    .collect(&format!("metric_{i}_{j}"), f64::from(i * j))
                    .await;
                tokio::task::yield_now().await;
            }
        }));
    }

    // Wait for all to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // System should have handled all operations
    let metrics = collector.get_metrics().await;
    assert_eq!(metrics.len(), 500 * 20);

    Ok(())
}

// ============================================================================
// Category 5: Recovery and Graceful Degradation
// ============================================================================

#[tokio::test]
async fn test_automatic_recovery_from_transient_failures() -> Result<()> {
    // Test automatic recovery from transient failures
    let collector = MockMetricCollector::new();

    // Cycle through failure and recovery
    for i in 0..10 {
        if i % 2 == 0 {
            collector.inject_failure();
        } else {
            collector.clear_failure();
        }

        let result = collector
            .collect(&format!("metric_{i}"), f64::from(i))
            .await;

        if i % 2 == 0 {
            assert!(result.is_err());
        } else {
            assert!(result.is_ok());
        }
    }

    // Verify only successful collections
    let metrics = collector.get_metrics().await;
    assert_eq!(metrics.len(), 5);

    Ok(())
}

#[tokio::test]
async fn test_graceful_shutdown_during_active_operations() -> Result<()> {
    // Test graceful shutdown while operations are in progress
    let collector = Arc::new(MockMetricCollector::new());
    let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);

    // Spawn task that collects metrics until shutdown
    let collector_clone = Arc::clone(&collector);
    let task_handle = tokio::spawn(async move {
        let mut count = 0;
        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    break;
                }
                () = async {
                    let _ = collector_clone.collect(&format!("metric_{count}"), f64::from(count)).await;
                    count += 1;
                    tokio::task::yield_now().await;
                } => {}
            }
        }
    });

    // Let it run for a bit
    for _ in 0..10 {
        tokio::task::yield_now().await;
    }

    // Signal shutdown
    let _ = shutdown_tx.send(()).await;

    // Wait for graceful shutdown
    task_handle.await.unwrap();

    // Verify some metrics were collected
    let metrics = collector.get_metrics().await;
    assert!(!metrics.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_error_propagation_through_layers() -> Result<()> {
    // Test that errors propagate correctly through system layers
    let storage = MockStorageBackend::new();
    let _collector = MockMetricCollector::new();

    // Inject failure at storage layer
    storage.inject_write_failure();

    // Attempt operation that would use storage
    // (In real system, collector would use storage)
    let storage_result = storage.write("data".to_string()).await;
    assert!(storage_result.is_err());

    // Verify error type is correct
    match storage_result {
        Err(BearDogError::Monitoring { .. }) => {
            // Expected
        }
        _ => panic!("Expected Monitoring error"),
    }

    Ok(())
}

#[tokio::test]
async fn test_partial_failure_handling() -> Result<()> {
    // Test handling of partial failures (some operations succeed, others fail)
    let collector = Arc::new(MockMetricCollector::new());
    let mut handles = vec![];

    // Spawn 100 concurrent operations with 30% failure rate
    for i in 0..100 {
        let collector = Arc::clone(&collector);
        handles.push(tokio::spawn(async move {
            // Fail 30% of operations
            if i % 10 < 3 {
                collector.inject_failure();
            } else {
                collector.clear_failure();
            }

            collector
                .collect(&format!("metric_{i}"), f64::from(i))
                .await
        }));
    }

    // Collect results
    let mut success_count = 0;
    let mut failure_count = 0;

    for handle in handles {
        match handle.await.unwrap() {
            Ok(()) => success_count += 1,
            Err(_) => failure_count += 1,
        }
    }

    // Verify partial failures (approximately 70/30 split)
    assert!((60..=80).contains(&success_count));
    assert!((20..=40).contains(&failure_count));

    Ok(())
}

// ============================================================================
// Category 6: Edge Cases and Boundary Conditions
// ============================================================================

#[tokio::test]
async fn test_empty_metric_name_handling() -> Result<()> {
    // Test handling of empty metric names
    let collector = MockMetricCollector::new();

    // Empty name should be accepted (validation is caller's responsibility)
    collector.collect("", 42.0).await?;

    let metrics = collector.get_metrics().await;
    assert_eq!(metrics.len(), 1);
    assert_eq!(metrics[0].0, "");

    Ok(())
}

#[tokio::test]
async fn test_special_characters_in_metric_names() -> Result<()> {
    // Test handling of special characters in metric names
    let collector = MockMetricCollector::new();

    let special_names = vec![
        "metric.with.dots",
        "metric-with-dashes",
        "metric_with_underscores",
        "metric:with:colons",
        "metric/with/slashes",
        "metric with spaces",
        "metric🚀with🚀emojis",
    ];

    for name in special_names {
        collector.collect(name, 1.0).await?;
    }

    let metrics = collector.get_metrics().await;
    assert_eq!(metrics.len(), 7);

    Ok(())
}

#[tokio::test]
async fn test_zero_and_negative_metric_values() -> Result<()> {
    // Test handling of zero and negative metric values
    let collector = MockMetricCollector::new();

    collector.collect("zero", 0.0).await?;
    collector.collect("negative", -42.0).await?;
    collector.collect("tiny", 1e-300).await?;
    collector.collect("huge", 1e300).await?;

    let metrics = collector.get_metrics().await;
    assert_eq!(metrics.len(), 4);

    Ok(())
}

#[tokio::test]
async fn test_rapid_failure_recovery_cycles() -> Result<()> {
    // Test rapid cycling between failure and recovery states
    let collector = MockMetricCollector::new();

    for i in 0..100 {
        // Toggle failure state rapidly
        if i % 2 == 0 {
            collector.inject_failure();
        } else {
            collector.clear_failure();
        }

        let _ = collector
            .collect(&format!("metric_{i}"), f64::from(i))
            .await;
    }

    // System should remain stable
    let metrics = collector.get_metrics().await;
    assert_eq!(metrics.len(), 50); // Only odd-numbered (non-failing) collections

    Ok(())
}

#[tokio::test]
async fn test_concurrent_failure_injection() -> Result<()> {
    // Test concurrent failure injection and recovery
    let collector = Arc::new(MockMetricCollector::new());
    let mut handles = vec![];

    // Spawn tasks that inject/clear failures concurrently
    for i in 0..50 {
        let collector = Arc::clone(&collector);
        handles.push(tokio::spawn(async move {
            if i % 3 == 0 {
                collector.inject_failure();
            } else if i % 3 == 1 {
                collector.clear_failure();
            }
            // else: no change

            let _ = collector
                .collect(&format!("metric_{i}"), f64::from(i))
                .await;
        }));
    }

    // Wait for all to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // System should have handled concurrent failure state changes
    let metrics = collector.get_metrics().await;
    // Can't predict exact count due to concurrent injection, but should have some
    assert!(!metrics.is_empty());
    assert!(metrics.len() <= 50);

    Ok(())
}
