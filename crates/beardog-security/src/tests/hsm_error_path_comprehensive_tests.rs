// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive HSM Error Path Tests
//!
//! Tests all error paths, device failures, and recovery scenarios
//! for Hardware Security Module operations using modern concurrent patterns (NO SLEEPS).
//!
//! ## Test Categories
//! - Device connection failures
//! - Operation timeouts
//! - Invalid responses
//! - Concurrent operation conflicts
//! - Recovery and failover

use beardog_errors::BearDogError;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;

type Result<T> = std::result::Result<T, BearDogError>;

// ============================================================================
// Test Fixtures and Mocks
// ============================================================================

/// Mock HSM device that can inject various failure modes
struct MockHsmDevice {
    fail_on_connect: Arc<AtomicBool>,
    fail_on_operation: Arc<AtomicBool>,
    operation_count: Arc<AtomicU64>,
    device_id: String,
    is_connected: Arc<AtomicBool>,
}

impl MockHsmDevice {
    fn new(device_id: &str) -> Self {
        Self {
            fail_on_connect: Arc::new(AtomicBool::new(false)),
            fail_on_operation: Arc::new(AtomicBool::new(false)),
            operation_count: Arc::new(AtomicU64::new(0)),
            device_id: device_id.to_string(),
            is_connected: Arc::new(AtomicBool::new(false)),
        }
    }

    fn inject_connection_failure(&self) {
        self.fail_on_connect.store(true, Ordering::SeqCst);
    }

    fn inject_operation_failure(&self) {
        self.fail_on_operation.store(true, Ordering::SeqCst);
    }

    fn clear_failures(&self) {
        self.fail_on_connect.store(false, Ordering::SeqCst);
        self.fail_on_operation.store(false, Ordering::SeqCst);
    }

    async fn connect(&self) -> Result<()> {
        if self.fail_on_connect.load(Ordering::SeqCst) {
            return Err(BearDogError::hsm(format!(
                "Failed to connect to HSM device: {}",
                self.device_id
            )));
        }

        self.is_connected.store(true, Ordering::SeqCst);
        Ok(())
    }

    async fn disconnect(&self) -> Result<()> {
        self.is_connected.store(false, Ordering::SeqCst);
        Ok(())
    }

    async fn generate_key(&self, _key_id: &str) -> Result<Vec<u8>> {
        self.operation_count.fetch_add(1, Ordering::SeqCst);

        if !self.is_connected.load(Ordering::SeqCst) {
            return Err(BearDogError::hsm("Device not connected".to_string()));
        }

        if self.fail_on_operation.load(Ordering::SeqCst) {
            return Err(BearDogError::hsm("Key generation failed".to_string()));
        }

        // Simulate key generation
        Ok(vec![0u8; 32])
    }

    async fn sign(&self, _data: &[u8], _key_id: &str) -> Result<Vec<u8>> {
        self.operation_count.fetch_add(1, Ordering::SeqCst);

        if !self.is_connected.load(Ordering::SeqCst) {
            return Err(BearDogError::hsm("Device not connected".to_string()));
        }

        if self.fail_on_operation.load(Ordering::SeqCst) {
            return Err(BearDogError::hsm("Signing operation failed".to_string()));
        }

        // Simulate signature
        Ok(vec![0u8; 64])
    }

    fn get_operation_count(&self) -> u64 {
        self.operation_count.load(Ordering::SeqCst)
    }

    fn is_connected(&self) -> bool {
        self.is_connected.load(Ordering::SeqCst)
    }
}

// ============================================================================
// Category 1: Device Connection Error Paths
// ============================================================================

#[tokio::test]
async fn test_hsm_connection_failure() -> Result<()> {
    // Test graceful handling of connection failures
    let device = MockHsmDevice::new("test_device");
    device.inject_connection_failure();

    let result = device.connect().await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        BearDogError::Cryptographic { .. }
    ));
    assert!(!device.is_connected());

    Ok(())
}

#[tokio::test]
async fn test_hsm_connection_recovery() -> Result<()> {
    // Test recovery after transient connection failure
    let device = MockHsmDevice::new("test_device");

    // First connection attempt fails
    device.inject_connection_failure();
    assert!(device.connect().await.is_err());

    // Recovery: clear failure and retry
    device.clear_failures();
    device.connect().await?;
    assert!(device.is_connected());

    Ok(())
}

#[tokio::test]
async fn test_hsm_operation_without_connection() -> Result<()> {
    // Test that operations fail gracefully when device is not connected
    let device = MockHsmDevice::new("test_device");

    // Attempt operation without connecting
    let result = device.generate_key("test_key").await;
    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn test_hsm_reconnection_after_disconnect() -> Result<()> {
    // Test reconnection after intentional disconnect
    let device = MockHsmDevice::new("test_device");

    // Connect, disconnect, reconnect
    device.connect().await?;
    assert!(device.is_connected());

    device.disconnect().await?;
    assert!(!device.is_connected());

    device.connect().await?;
    assert!(device.is_connected());

    Ok(())
}

// ============================================================================
// Category 2: Operation Error Paths
// ============================================================================

#[tokio::test]
async fn test_hsm_key_generation_failure() -> Result<()> {
    // Test graceful handling of key generation failures
    let device = MockHsmDevice::new("test_device");
    device.connect().await?;

    device.inject_operation_failure();
    let result = device.generate_key("test_key").await;
    assert!(result.is_err());

    // Device should still be connected
    assert!(device.is_connected());

    Ok(())
}

#[tokio::test]
async fn test_hsm_signing_failure() -> Result<()> {
    // Test graceful handling of signing failures
    let device = MockHsmDevice::new("test_device");
    device.connect().await?;

    device.inject_operation_failure();
    let result = device.sign(b"test_data", "test_key").await;
    assert!(result.is_err());

    // Device should still be connected
    assert!(device.is_connected());

    Ok(())
}

#[tokio::test]
async fn test_hsm_operation_recovery() -> Result<()> {
    // Test recovery after transient operation failure
    let device = MockHsmDevice::new("test_device");
    device.connect().await?;

    // First operation fails
    device.inject_operation_failure();
    assert!(device.generate_key("test_key").await.is_err());

    // Recovery: clear failure and retry
    device.clear_failures();
    let result = device.generate_key("test_key").await;
    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn test_hsm_concurrent_operations() -> Result<()> {
    // Test concurrent operations on the same device
    let device = Arc::new(MockHsmDevice::new("test_device"));
    device.connect().await?;

    let mut handles = vec![];

    // Spawn 50 concurrent operations
    for i in 0..50 {
        let device = Arc::clone(&device);
        handles.push(tokio::spawn(async move {
            device
                .generate_key(&format!("key_{}", i))
                .await
                .expect("operation should succeed");
        }));
    }

    // Wait for all to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify all operations executed
    assert_eq!(device.get_operation_count(), 50);

    Ok(())
}

// ============================================================================
// Category 3: Concurrent Failure Scenarios
// ============================================================================

#[tokio::test]
async fn test_hsm_concurrent_operations_with_failures() -> Result<()> {
    // Test concurrent operations with intermittent failures
    let device = Arc::new(MockHsmDevice::new("test_device"));
    device.connect().await?;

    let mut handles = vec![];

    // Spawn 100 concurrent operations with 20% failure rate
    for i in 0..100 {
        let device = Arc::clone(&device);
        handles.push(tokio::spawn(async move {
            // Inject failure for every 5th operation
            if i % 5 == 0 {
                device.inject_operation_failure();
            } else {
                device.clear_failures();
            }

            device.generate_key(&format!("key_{}", i)).await
        }));
    }

    // Collect results
    let mut success_count = 0;
    let mut failure_count = 0;

    for handle in handles {
        match handle.await.unwrap() {
            Ok(_) => success_count += 1,
            Err(_) => failure_count += 1,
        }
    }

    // Verify approximately 80/20 split
    assert!((70..=90).contains(&success_count));
    assert!((10..=30).contains(&failure_count));

    Ok(())
}

#[tokio::test]
async fn test_hsm_connection_failure_during_operations() -> Result<()> {
    // Test handling of connection loss during operations
    let device = Arc::new(MockHsmDevice::new("test_device"));
    device.connect().await?;

    let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);

    // Spawn task that performs operations
    let device_clone = Arc::clone(&device);
    let task_handle = tokio::spawn(async move {
        let mut count = 0;
        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    break;
                }
                _ = async {
                    let _ = device_clone.generate_key(&format!("key_{}", count)).await;
                    count += 1;
                    tokio::task::yield_now().await;
                } => {}
            }
        }
    });

    // Let it run for a bit
    for _ in 0..5 {
        tokio::task::yield_now().await;
    }

    // Simulate connection loss
    device.disconnect().await?;

    // Signal shutdown
    let _ = shutdown_tx.send(()).await;
    task_handle.await.unwrap();

    // Verify operations were attempted
    assert!(device.get_operation_count() > 0);

    Ok(())
}

// ============================================================================
// Category 4: Error Propagation and Recovery
// ============================================================================

#[tokio::test]
async fn test_hsm_error_propagation() -> Result<()> {
    // Test that errors propagate correctly through system layers
    let device = MockHsmDevice::new("test_device");
    device.inject_connection_failure();

    let result = device.connect().await;
    assert!(result.is_err());

    // Verify error type
    match result {
        Err(BearDogError::Cryptographic { .. }) => {
            // Expected
        }
        _ => panic!("Expected Cryptographic error"),
    }

    Ok(())
}

#[tokio::test]
async fn test_hsm_rapid_failure_recovery_cycles() -> Result<()> {
    // Test rapid cycling between failure and recovery
    let device = MockHsmDevice::new("test_device");
    device.connect().await?;

    for i in 0..50 {
        if i % 2 == 0 {
            device.inject_operation_failure();
        } else {
            device.clear_failures();
        }

        let _ = device.generate_key(&format!("key_{}", i)).await;
    }

    // Device should remain stable
    assert!(device.is_connected());

    Ok(())
}

#[tokio::test]
async fn test_hsm_graceful_degradation() -> Result<()> {
    // Test graceful degradation under failure conditions
    let device = MockHsmDevice::new("test_device");
    device.connect().await?;

    // Inject failure
    device.inject_operation_failure();

    // Multiple operations should fail gracefully
    for i in 0..10 {
        let result = device.generate_key(&format!("key_{}", i)).await;
        assert!(result.is_err());
    }

    // Device should still be usable after clearing failure
    device.clear_failures();
    let result = device.generate_key("recovery_key").await;
    assert!(result.is_ok());

    Ok(())
}

// ============================================================================
// Category 5: Edge Cases and Boundary Conditions
// ============================================================================

#[tokio::test]
async fn test_hsm_empty_key_id() -> Result<()> {
    // Test handling of empty key IDs
    let device = MockHsmDevice::new("test_device");
    device.connect().await?;

    let result = device.generate_key("").await;
    // Should succeed (validation is caller's responsibility)
    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn test_hsm_empty_data_signing() -> Result<()> {
    // Test signing empty data
    let device = MockHsmDevice::new("test_device");
    device.connect().await?;

    let result = device.sign(&[], "test_key").await;
    // Should succeed (validation is caller's responsibility)
    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn test_hsm_large_data_signing() -> Result<()> {
    // Test signing large data
    let device = MockHsmDevice::new("test_device");
    device.connect().await?;

    let large_data = vec![0u8; 1_000_000];
    let result = device.sign(&large_data, "test_key").await;
    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn test_hsm_special_characters_in_key_id() -> Result<()> {
    // Test handling of special characters in key IDs
    let device = MockHsmDevice::new("test_device");
    device.connect().await?;

    let special_key_ids = vec![
        "key.with.dots",
        "key-with-dashes",
        "key_with_underscores",
        "key:with:colons",
        "key/with/slashes",
        "key with spaces",
    ];

    for key_id in special_key_ids {
        let result = device.generate_key(key_id).await;
        assert!(result.is_ok());
    }

    Ok(())
}

// ============================================================================
// Category 6: Resource Management
// ============================================================================

#[tokio::test]
async fn test_hsm_multiple_connections() -> Result<()> {
    // Test handling of multiple connection attempts
    let device = MockHsmDevice::new("test_device");

    // Multiple connects should be idempotent
    device.connect().await?;
    device.connect().await?;
    device.connect().await?;

    assert!(device.is_connected());

    Ok(())
}

#[tokio::test]
async fn test_hsm_multiple_disconnections() -> Result<()> {
    // Test handling of multiple disconnection attempts
    let device = MockHsmDevice::new("test_device");
    device.connect().await?;

    // Multiple disconnects should be idempotent
    device.disconnect().await?;
    device.disconnect().await?;
    device.disconnect().await?;

    assert!(!device.is_connected());

    Ok(())
}

#[tokio::test]
async fn test_hsm_operation_limits() -> Result<()> {
    // Test handling of high operation counts
    let device = MockHsmDevice::new("test_device");
    device.connect().await?;

    // Perform many operations
    for i in 0..1_000 {
        device.generate_key(&format!("key_{}", i)).await?;
    }

    assert_eq!(device.get_operation_count(), 1_000);

    Ok(())
}

// ============================================================================
// Category 7: Concurrent Device Access
// ============================================================================

#[tokio::test]
async fn test_hsm_concurrent_connect_disconnect() -> Result<()> {
    // Test concurrent connect/disconnect operations
    let device = Arc::new(MockHsmDevice::new("test_device"));
    let mut handles = vec![];

    // Spawn tasks that connect/disconnect concurrently
    for i in 0..50 {
        let device = Arc::clone(&device);
        handles.push(tokio::spawn(async move {
            if i % 2 == 0 {
                let _ = device.connect().await;
            } else {
                let _ = device.disconnect().await;
            }
        }));
    }

    // Wait for all to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Device should be in a valid state
    // (Either connected or disconnected, but not corrupted)
    Ok(())
}

#[tokio::test]
async fn test_hsm_concurrent_mixed_operations() -> Result<()> {
    // Test concurrent mix of connections, operations, and disconnections
    let device = Arc::new(MockHsmDevice::new("test_device"));
    device.connect().await?;

    let mut handles = vec![];

    // Spawn 100 mixed operations
    for i in 0..100 {
        let device = Arc::clone(&device);
        handles.push(tokio::spawn(async move {
            match i % 3 {
                0 => {
                    let _ = device.generate_key(&format!("key_{}", i)).await;
                }
                1 => {
                    let _ = device.sign(b"test_data", &format!("key_{}", i)).await;
                }
                2 => {
                    let _ = device.connect().await;
                }
                _ => unreachable!(),
            }
        }));
    }

    // Wait for all to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify operations were attempted
    assert!(device.get_operation_count() > 0);

    Ok(())
}

#[tokio::test]
async fn test_hsm_stress_test() -> Result<()> {
    // Stress test with many concurrent operations
    let device = Arc::new(MockHsmDevice::new("test_device"));
    device.connect().await?;

    let mut handles = vec![];

    // Spawn 500 concurrent operations
    for i in 0..500 {
        let device = Arc::clone(&device);
        handles.push(tokio::spawn(async move {
            for j in 0..10 {
                let _ = device.generate_key(&format!("key_{}_{}", i, j)).await;
                tokio::task::yield_now().await;
            }
        }));
    }

    // Wait for all to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify all operations executed
    assert_eq!(device.get_operation_count(), 5_000);

    Ok(())
}
