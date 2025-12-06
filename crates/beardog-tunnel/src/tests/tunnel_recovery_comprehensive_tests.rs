//! Comprehensive Tunnel Recovery Tests
//!
//! Tests all network failure, reconnection, and state recovery scenarios
//! for tunnel operations using modern concurrent patterns (NO SLEEPS).
//!
//! ## Test Categories
//! - Connection interruption handling
//! - Packet loss and out-of-order delivery
//! - Network partition recovery
//! - Timeout and retry logic
//! - State synchronization after reconnect

#![allow(clippy::unwrap_used, clippy::expect_used)]

use beardog_errors::BearDogError;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

type Result<T> = std::result::Result<T, BearDogError>;

// ============================================================================
// Test Fixtures and Mocks
// ============================================================================

/// Mock tunnel connection that can simulate various failure modes
#[allow(dead_code)]
#[derive(Clone)]
struct MockTunnelConnection {
    connection_id: String,
    is_connected: Arc<AtomicBool>,
    fail_on_send: Arc<AtomicBool>,
    fail_on_receive: Arc<AtomicBool>,
    packets_sent: Arc<AtomicU64>,
    packets_received: Arc<AtomicU64>,
    packets_dropped: Arc<AtomicU64>,
    message_queue: Arc<RwLock<Vec<Vec<u8>>>>,
}

impl MockTunnelConnection {
    fn new(connection_id: &str) -> Self {
        Self {
            connection_id: connection_id.to_string(),
            is_connected: Arc::new(AtomicBool::new(false)),
            fail_on_send: Arc::new(AtomicBool::new(false)),
            fail_on_receive: Arc::new(AtomicBool::new(false)),
            packets_sent: Arc::new(AtomicU64::new(0)),
            packets_received: Arc::new(AtomicU64::new(0)),
            packets_dropped: Arc::new(AtomicU64::new(0)),
            message_queue: Arc::new(RwLock::new(Vec::new())),
        }
    }

    async fn connect(&self) -> Result<()> {
        if self.is_connected.load(Ordering::SeqCst) {
            return Ok(()); // Already connected
        }

        self.is_connected.store(true, Ordering::SeqCst);
        Ok(())
    }

    async fn disconnect(&self) -> Result<()> {
        self.is_connected.store(false, Ordering::SeqCst);
        Ok(())
    }

    fn inject_send_failure(&self) {
        self.fail_on_send.store(true, Ordering::SeqCst);
    }

    fn inject_receive_failure(&self) {
        self.fail_on_receive.store(true, Ordering::SeqCst);
    }

    fn clear_failures(&self) {
        self.fail_on_send.store(false, Ordering::SeqCst);
        self.fail_on_receive.store(false, Ordering::SeqCst);
    }

    async fn send(&self, data: Vec<u8>) -> Result<()> {
        if !self.is_connected.load(Ordering::SeqCst) {
            return Err(BearDogError::network(
                "Connection not established".to_string(),
            ));
        }

        if self.fail_on_send.load(Ordering::SeqCst) {
            self.packets_dropped.fetch_add(1, Ordering::SeqCst);
            return Err(BearDogError::network("Packet send failed".to_string()));
        }

        self.packets_sent.fetch_add(1, Ordering::SeqCst);
        let mut queue = self.message_queue.write().await;
        queue.push(data);
        Ok(())
    }

    async fn receive(&self) -> Result<Option<Vec<u8>>> {
        if !self.is_connected.load(Ordering::SeqCst) {
            return Err(BearDogError::network(
                "Connection not established".to_string(),
            ));
        }

        if self.fail_on_receive.load(Ordering::SeqCst) {
            return Err(BearDogError::network("Packet receive failed".to_string()));
        }

        let mut queue = self.message_queue.write().await;
        if let Some(data) = queue.pop() {
            self.packets_received.fetch_add(1, Ordering::SeqCst);
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    fn is_connected(&self) -> bool {
        self.is_connected.load(Ordering::SeqCst)
    }

    fn get_packets_sent(&self) -> u64 {
        self.packets_sent.load(Ordering::SeqCst)
    }

    #[allow(dead_code)]
    fn get_packets_received(&self) -> u64 {
        self.packets_received.load(Ordering::SeqCst)
    }

    fn get_packets_dropped(&self) -> u64 {
        self.packets_dropped.load(Ordering::SeqCst)
    }
}

// ============================================================================
// Category 1: Connection Interruption Handling
// ============================================================================

#[tokio::test]
async fn test_tunnel_connection_drop() -> Result<()> {
    // Test handling of sudden connection drop
    let tunnel = MockTunnelConnection::new("test_tunnel");
    tunnel.connect().await?;

    // Send some data
    tunnel.send(b"test_data".to_vec()).await?;
    assert_eq!(tunnel.get_packets_sent(), 1);

    // Simulate connection drop
    tunnel.disconnect().await?;

    // Attempt to send should fail
    let result = tunnel.send(b"more_data".to_vec()).await;
    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn test_tunnel_reconnection_after_drop() -> Result<()> {
    // Test successful reconnection after connection drop
    let tunnel = MockTunnelConnection::new("test_tunnel");

    // Connect, send, disconnect
    tunnel.connect().await?;
    tunnel.send(b"data1".to_vec()).await?;
    tunnel.disconnect().await?;

    // Reconnect and send again
    tunnel.connect().await?;
    tunnel.send(b"data2".to_vec()).await?;

    assert_eq!(tunnel.get_packets_sent(), 2);
    Ok(())
}

#[tokio::test]
async fn test_tunnel_connection_during_active_transfer() -> Result<()> {
    // Test connection drop during active data transfer
    let tunnel = Arc::new(MockTunnelConnection::new("test_tunnel"));
    tunnel.connect().await?;

    let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);

    // Spawn sender task
    let tunnel_clone = Arc::clone(&tunnel);
    let sender_handle = tokio::spawn(async move {
        let mut count = 0;
        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    break count;
                }
                _ = async {
                    let _ = tunnel_clone.send(format!("data_{}", count).into_bytes()).await;
                    count += 1;
                    tokio::task::yield_now().await;
                } => {}
            }
        }
    });

    // Let it send some packets
    for _ in 0..5 {
        tokio::task::yield_now().await;
    }

    // Simulate connection drop
    tunnel.disconnect().await?;

    // Signal shutdown
    let _ = shutdown_tx.send(()).await;
    let packets_attempted = sender_handle.await.unwrap();

    // Some packets should have been sent before disconnect
    assert!(tunnel.get_packets_sent() > 0);
    // After disconnect, some sends would have failed (or test was fast enough that all succeeded before disconnect)
    assert!(packets_attempted >= tunnel.get_packets_sent());

    Ok(())
}

// ============================================================================
// Category 2: Packet Loss and Delivery Issues
// ============================================================================

#[tokio::test]
async fn test_tunnel_packet_loss_handling() -> Result<()> {
    // Test graceful handling of packet loss
    let tunnel = MockTunnelConnection::new("test_tunnel");
    tunnel.connect().await?;

    // Send some packets successfully
    tunnel.send(b"packet1".to_vec()).await?;
    tunnel.send(b"packet2".to_vec()).await?;

    // Inject send failure to simulate packet loss
    tunnel.inject_send_failure();
    let result = tunnel.send(b"packet3".to_vec()).await;
    assert!(result.is_err());
    assert_eq!(tunnel.get_packets_dropped(), 1);

    // Clear failure and continue
    tunnel.clear_failures();
    tunnel.send(b"packet4".to_vec()).await?;

    assert_eq!(tunnel.get_packets_sent(), 3); // packet1, packet2, packet4
    assert_eq!(tunnel.get_packets_dropped(), 1); // packet3

    Ok(())
}

#[tokio::test]
async fn test_tunnel_receive_failure_handling() -> Result<()> {
    // Test handling of receive failures
    let tunnel = MockTunnelConnection::new("test_tunnel");
    tunnel.connect().await?;

    // Send some data
    tunnel.send(b"test_data".to_vec()).await?;

    // Inject receive failure
    tunnel.inject_receive_failure();
    let result = tunnel.receive().await;
    assert!(result.is_err());

    // Clear failure and receive successfully
    tunnel.clear_failures();
    let data = tunnel.receive().await?;
    assert!(data.is_some());
    assert_eq!(data.unwrap(), b"test_data");

    Ok(())
}

#[tokio::test]
async fn test_tunnel_partial_packet_loss() -> Result<()> {
    // Test handling of intermittent packet loss
    let tunnel = MockTunnelConnection::new("test_tunnel");
    tunnel.connect().await?;

    let mut successful = 0;
    let mut failed = 0;

    // Send 50 packets with 30% failure rate
    for i in 0..50 {
        if i % 10 < 3 {
            tunnel.inject_send_failure();
        } else {
            tunnel.clear_failures();
        }

        match tunnel.send(format!("packet_{}", i).into_bytes()).await {
            Ok(_) => successful += 1,
            Err(_) => failed += 1,
        }
    }

    // Verify approximately 70/30 split
    assert!((30..=40).contains(&successful));
    assert!((10..=20).contains(&failed));

    Ok(())
}

// ============================================================================
// Category 3: Network Partition Recovery
// ============================================================================

#[tokio::test]
async fn test_tunnel_network_partition_recovery() -> Result<()> {
    // Test recovery after network partition
    let tunnel = MockTunnelConnection::new("test_tunnel");
    tunnel.connect().await?;

    // Send data before partition
    tunnel.send(b"before_partition".to_vec()).await?;

    // Simulate network partition
    tunnel.disconnect().await?;

    // Attempt operations during partition (should fail)
    assert!(tunnel.send(b"during_partition".to_vec()).await.is_err());

    // Recover from partition
    tunnel.connect().await?;

    // Operations should succeed after recovery
    tunnel.send(b"after_partition".to_vec()).await?;
    assert_eq!(tunnel.get_packets_sent(), 2);

    Ok(())
}

#[tokio::test]
async fn test_tunnel_rapid_partition_recovery_cycles() -> Result<()> {
    // Test rapid cycles of partition and recovery
    let tunnel = MockTunnelConnection::new("test_tunnel");

    for i in 0..20 {
        if i % 2 == 0 {
            tunnel.connect().await?;
            let _ = tunnel.send(format!("data_{}", i).into_bytes()).await;
        } else {
            tunnel.disconnect().await?;
        }
    }

    // Should have sent approximately 10 packets (during connected periods)
    assert!(tunnel.get_packets_sent() >= 8 && tunnel.get_packets_sent() <= 12);

    Ok(())
}

// ============================================================================
// Category 4: Timeout and Retry Logic
// ============================================================================

#[tokio::test]
async fn test_tunnel_operation_retry_after_failure() -> Result<()> {
    // Test retry logic after transient failure
    let tunnel = MockTunnelConnection::new("test_tunnel");
    tunnel.connect().await?;

    // First attempt fails
    tunnel.inject_send_failure();
    assert!(tunnel.send(b"data".to_vec()).await.is_err());

    // Retry succeeds
    tunnel.clear_failures();
    tunnel.send(b"data".to_vec()).await?;

    assert_eq!(tunnel.get_packets_sent(), 1);
    assert_eq!(tunnel.get_packets_dropped(), 1);

    Ok(())
}

#[tokio::test]
async fn test_tunnel_exponential_backoff_simulation() -> Result<()> {
    // Simulate exponential backoff retry pattern
    let tunnel = MockTunnelConnection::new("test_tunnel");
    tunnel.connect().await?;

    let max_retries = 5;
    let mut retry_count = 0;

    // Simulate retries with increasing delays (using yield instead of sleep)
    while retry_count < max_retries {
        tunnel.inject_send_failure();

        if tunnel.send(b"test".to_vec()).await.is_err() {
            retry_count += 1;
            // Exponential backoff simulation (yield more times)
            for _ in 0..(1 << retry_count) {
                tokio::task::yield_now().await;
            }
        } else {
            break;
        }
    }

    // Verify retries were attempted
    assert_eq!(retry_count, max_retries);
    assert_eq!(tunnel.get_packets_dropped(), max_retries as u64);

    // Final attempt should succeed
    tunnel.clear_failures();
    tunnel.send(b"test".to_vec()).await?;
    assert_eq!(tunnel.get_packets_sent(), 1);

    Ok(())
}

// ============================================================================
// Category 5: State Synchronization After Reconnect
// ============================================================================

#[tokio::test]
async fn test_tunnel_state_preservation_across_reconnects() -> Result<()> {
    // Test that state is preserved across reconnects
    let tunnel = MockTunnelConnection::new("test_tunnel");

    // Connect and send
    tunnel.connect().await?;
    tunnel.send(b"data1".to_vec()).await?;
    let sent_before = tunnel.get_packets_sent();

    // Disconnect and reconnect
    tunnel.disconnect().await?;
    tunnel.connect().await?;

    // Send more data
    tunnel.send(b"data2".to_vec()).await?;

    // Verify state was preserved
    assert_eq!(tunnel.get_packets_sent(), sent_before + 1);

    Ok(())
}

#[tokio::test]
async fn test_tunnel_message_queue_integrity() -> Result<()> {
    // Test that message queue remains intact after connection issues
    let tunnel = MockTunnelConnection::new("test_tunnel");
    tunnel.connect().await?;

    // Send multiple messages
    tunnel.send(b"msg1".to_vec()).await?;
    tunnel.send(b"msg2".to_vec()).await?;
    tunnel.send(b"msg3".to_vec()).await?;

    // Disconnect
    tunnel.disconnect().await?;

    // Reconnect
    tunnel.connect().await?;

    // Verify messages can still be received
    let msg1 = tunnel.receive().await?;
    let msg2 = tunnel.receive().await?;
    let msg3 = tunnel.receive().await?;

    assert!(msg1.is_some());
    assert!(msg2.is_some());
    assert!(msg3.is_some());

    Ok(())
}

// ============================================================================
// Category 6: Concurrent Connection Management
// ============================================================================

#[tokio::test]
async fn test_tunnel_concurrent_send_operations() -> Result<()> {
    // Test concurrent send operations
    let tunnel = Arc::new(MockTunnelConnection::new("test_tunnel"));
    tunnel.connect().await?;

    let mut handles = vec![];

    // Spawn 100 concurrent senders
    for i in 0..100 {
        let tunnel = Arc::clone(&tunnel);
        handles.push(tokio::spawn(async move {
            tunnel
                .send(format!("data_{}", i).into_bytes())
                .await
                .expect("send should succeed");
        }));
    }

    // Wait for all to complete
    for handle in handles {
        handle.await.unwrap();
    }

    assert_eq!(tunnel.get_packets_sent(), 100);

    Ok(())
}

#[tokio::test]
async fn test_tunnel_concurrent_connect_disconnect() -> Result<()> {
    // Test concurrent connect/disconnect operations
    let tunnel = Arc::new(MockTunnelConnection::new("test_tunnel"));
    let mut handles = vec![];

    // Spawn 50 tasks that connect/disconnect concurrently
    for i in 0..50 {
        let tunnel = Arc::clone(&tunnel);
        handles.push(tokio::spawn(async move {
            if i % 2 == 0 {
                let _ = tunnel.connect().await;
            } else {
                let _ = tunnel.disconnect().await;
            }
        }));
    }

    // Wait for all to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Tunnel should be in a valid state
    // (Either connected or disconnected, but not corrupted)
    Ok(())
}

#[tokio::test]
async fn test_tunnel_concurrent_send_receive() -> Result<()> {
    // Test concurrent send and receive operations
    let tunnel = Arc::new(MockTunnelConnection::new("test_tunnel"));
    tunnel.connect().await?;

    // Spawn sender task
    let tunnel_sender = Arc::clone(&tunnel);
    let sender_handle = tokio::spawn(async move {
        for i in 0..50 {
            let _ = tunnel_sender.send(format!("msg_{}", i).into_bytes()).await;
            tokio::task::yield_now().await;
        }
    });

    // Spawn receiver task
    let tunnel_receiver = Arc::clone(&tunnel);
    let receiver_handle = tokio::spawn(async move {
        let mut received_count = 0;
        for _ in 0..50 {
            if tunnel_receiver.receive().await.ok().flatten().is_some() {
                received_count += 1;
            }
            tokio::task::yield_now().await;
        }
        received_count
    });

    // Wait for both to complete
    sender_handle.await.unwrap();
    let received = receiver_handle.await.unwrap();

    // Should have sent and received approximately the same number
    assert_eq!(tunnel.get_packets_sent(), 50);
    assert!(received > 0); // At least some were received

    Ok(())
}

// ============================================================================
// Category 7: Error Propagation and Recovery
// ============================================================================

#[tokio::test]
async fn test_tunnel_graceful_degradation() -> Result<()> {
    // Test graceful degradation under failure conditions
    let tunnel = MockTunnelConnection::new("test_tunnel");
    tunnel.connect().await?;

    // Inject failure
    tunnel.inject_send_failure();

    // Multiple operations should fail gracefully
    for i in 0..10 {
        let result = tunnel.send(format!("data_{}", i).into_bytes()).await;
        assert!(result.is_err());
    }

    // Clear failure
    tunnel.clear_failures();

    // Should be able to send successfully
    tunnel.send(b"recovery_data".to_vec()).await?;
    assert_eq!(tunnel.get_packets_sent(), 1);
    assert_eq!(tunnel.get_packets_dropped(), 10);

    Ok(())
}

#[tokio::test]
async fn test_tunnel_error_recovery_workflow() -> Result<()> {
    // Test complete error recovery workflow
    let tunnel = MockTunnelConnection::new("test_tunnel");

    // 1. Connect
    tunnel.connect().await?;

    // 2. Send successfully
    tunnel.send(b"data1".to_vec()).await?;

    // 3. Experience failure
    tunnel.inject_send_failure();
    assert!(tunnel.send(b"data2".to_vec()).await.is_err());

    // 4. Detect failure and initiate recovery
    tunnel.clear_failures();

    // 5. Retry operation
    tunnel.send(b"data2_retry".to_vec()).await?;

    // 6. Verify recovery
    assert_eq!(tunnel.get_packets_sent(), 2);
    assert!(tunnel.is_connected());

    Ok(())
}

// ============================================================================
// Category 8: Edge Cases and Stress Tests
// ============================================================================

#[tokio::test]
async fn test_tunnel_empty_data_transmission() -> Result<()> {
    // Test transmission of empty data
    let tunnel = MockTunnelConnection::new("test_tunnel");
    tunnel.connect().await?;

    tunnel.send(vec![]).await?;
    assert_eq!(tunnel.get_packets_sent(), 1);

    Ok(())
}

#[tokio::test]
async fn test_tunnel_large_data_transmission() -> Result<()> {
    // Test transmission of large data
    let tunnel = MockTunnelConnection::new("test_tunnel");
    tunnel.connect().await?;

    let large_data = vec![0u8; 1_000_000];
    tunnel.send(large_data).await?;
    assert_eq!(tunnel.get_packets_sent(), 1);

    Ok(())
}

#[tokio::test]
async fn test_tunnel_stress_test() -> Result<()> {
    // Stress test with many operations
    let tunnel = Arc::new(MockTunnelConnection::new("test_tunnel"));
    tunnel.connect().await?;

    let mut handles = vec![];

    // Spawn 200 tasks, each sending 10 packets
    for i in 0..200 {
        let tunnel = Arc::clone(&tunnel);
        handles.push(tokio::spawn(async move {
            for j in 0..10 {
                let _ = tunnel.send(format!("data_{}_{}", i, j).into_bytes()).await;
                tokio::task::yield_now().await;
            }
        }));
    }

    // Wait for all to complete
    for handle in handles {
        handle.await.unwrap();
    }

    assert_eq!(tunnel.get_packets_sent(), 2_000);

    Ok(())
}

#[tokio::test]
async fn test_tunnel_multiple_reconnection_cycles() -> Result<()> {
    // Test multiple reconnection cycles
    let tunnel = MockTunnelConnection::new("test_tunnel");

    for cycle in 0..10 {
        // Connect
        tunnel.connect().await?;
        assert!(tunnel.is_connected());

        // Send some data
        tunnel.send(format!("cycle_{}", cycle).into_bytes()).await?;

        // Disconnect
        tunnel.disconnect().await?;
        assert!(!tunnel.is_connected());
    }

    assert_eq!(tunnel.get_packets_sent(), 10);

    Ok(())
}

#[tokio::test]
async fn test_tunnel_idempotent_operations() -> Result<()> {
    // Test that operations are idempotent
    let tunnel = MockTunnelConnection::new("test_tunnel");

    // Multiple connects should be safe
    tunnel.connect().await?;
    tunnel.connect().await?;
    tunnel.connect().await?;
    assert!(tunnel.is_connected());

    // Multiple disconnects should be safe
    tunnel.disconnect().await?;
    tunnel.disconnect().await?;
    tunnel.disconnect().await?;
    assert!(!tunnel.is_connected());

    Ok(())
}
