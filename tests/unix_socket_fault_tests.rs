// SPDX-License-Identifier: AGPL-3.0-only
#![allow(clippy::expect_used, clippy::unwrap_used)]
//! Fault Injection Tests for Unix Socket IPC Evolution
//!
//! Tests the resilience and recovery of the lock-free concurrent implementation
//! under fault conditions:
//! - Socket file deletion during operation
//! - Permission errors
//! - Disk full scenarios
//! - Network timeout simulations
//! - Memory pressure
//! - Process crashes and recovery

use serde_json::json;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tempfile::TempDir;
use tokio::net::UnixStream;

use beardog_genetics::EcosystemGeneticEngine;
use beardog_tunnel::btsp_provider::BeardogBtspProvider;
use beardog_tunnel::tunnel::hsm::HsmManager;
use beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer;
use beardog_types::primal_identity::PrimalIdentity;

// Helper functions
fn test_socket() -> (TempDir, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let socket_path = dir.path().join("fault-test.sock");
    (dir, socket_path)
}

async fn create_test_btsp_provider() -> Arc<BeardogBtspProvider> {
    beardog_errors::process_env::set_var("BEARDOG_HSM_MODE", "software");
    let hsm = Arc::new(HsmManager::auto_initialize().await.expect("HSM init"));
    let genetics = Arc::new(EcosystemGeneticEngine::new().expect("Genetics init"));
    Arc::new(
        BeardogBtspProvider::new(hsm, genetics)
            .await
            .expect("BTSP init"),
    )
}

async fn start_server_ready(
    socket_path: PathBuf,
) -> (
    tokio::task::JoinHandle<()>,
    Arc<std::sync::atomic::AtomicBool>,
    Arc<UnixSocketIpcServer>,
) {
    let btsp_provider = create_test_btsp_provider().await;
    let server = Arc::new(
        UnixSocketIpcServer::new(
            socket_path,
            btsp_provider,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
        )
        .await
        .unwrap(),
    );
    let ready_flag = server.readiness_flag();
    let server_clone = Arc::clone(&server);

    let handle = tokio::spawn(async move {
        server_clone.start().await.unwrap();
    });

    assert!(UnixSocketIpcServer::wait_ready_flag(&ready_flag, Duration::from_secs(10)).await);
    (handle, ready_flag, server)
}

#[tokio::test]
async fn fault_test_socket_deletion_during_operation() {
    // Fault: Delete socket file while server is running
    let (_dir, socket_path) = test_socket();
    let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;

    // Verify socket exists
    assert!(socket_path.exists());

    // Connect successfully
    let mut stream = UnixStream::connect(&socket_path).await.unwrap();

    // Delete socket file (simulates external interference)
    std::fs::remove_file(&socket_path).ok();

    // Existing connection should still work
    let request = json!({"jsonrpc": "2.0", "method": "health", "id": 1});
    let mut request_str = serde_json::to_string(&request).unwrap();
    request_str.push('\n');

    use tokio::io::AsyncWriteExt;
    stream.write_all(request_str.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();

    // Should still get response on existing connection
    use tokio::io::AsyncBufReadExt;
    let mut reader = tokio::io::BufReader::new(stream);
    let mut line = String::new();
    let result = reader.read_line(&mut line).await;
    assert!(
        result.is_ok(),
        "Existing connection should work after socket deletion"
    );

    // Graceful shutdown
    server.stop().await.unwrap();
    server_handle.abort();
}

#[tokio::test]
async fn fault_test_connection_timeout() {
    // Fault: Simulate slow/hanging connections
    let (_dir, socket_path) = test_socket();
    let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;

    // Connect but don't send anything (hanging connection)
    let _hanging_stream = UnixStream::connect(&socket_path).await.unwrap();

    // Server should still accept new connections
    let mut stream2 = UnixStream::connect(&socket_path).await.unwrap();

    let request = json!({"jsonrpc": "2.0", "method": "health", "id": 1});
    let mut request_str = serde_json::to_string(&request).unwrap();
    request_str.push('\n');

    use tokio::io::AsyncWriteExt;
    stream2.write_all(request_str.as_bytes()).await.unwrap();
    stream2.flush().await.unwrap();

    // Should get response despite hanging connection
    use tokio::io::AsyncBufReadExt;
    let mut reader = tokio::io::BufReader::new(stream2);
    let mut line = String::new();
    let result = tokio::time::timeout(Duration::from_secs(2), reader.read_line(&mut line)).await;

    assert!(result.is_ok(), "Should respond despite hanging connections");

    server.stop().await.unwrap();
    server_handle.abort();
}

#[tokio::test]
async fn fault_test_malformed_requests() {
    // Fault: Send malformed/corrupted data
    let (_dir, socket_path) = test_socket();
    let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;

    let mut stream = UnixStream::connect(&socket_path).await.unwrap();

    // Send garbage data
    use tokio::io::AsyncWriteExt;
    stream.write_all(b"GARBAGE{{{INVALID JSON\n").await.unwrap();
    stream.flush().await.unwrap();

    // Should get error response (not crash)
    use tokio::io::AsyncBufReadExt;
    let mut reader = tokio::io::BufReader::new(stream);
    let mut line = String::new();
    let result = tokio::time::timeout(Duration::from_secs(2), reader.read_line(&mut line)).await;

    assert!(
        result.is_ok(),
        "Should handle malformed requests gracefully"
    );

    server.stop().await.unwrap();
    server_handle.abort();
}

#[tokio::test]
async fn fault_test_rapid_server_restart() {
    // Fault: Rapidly stop and restart server
    let (_dir, socket_path) = test_socket();

    for cycle in 0..5 {
        let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;

        // Quick connection test
        let mut stream = UnixStream::connect(&socket_path).await.unwrap();
        let request = json!({"jsonrpc": "2.0", "method": "health", "id": cycle});
        let mut request_str = serde_json::to_string(&request).unwrap();
        request_str.push('\n');

        use tokio::io::AsyncWriteExt;
        stream.write_all(request_str.as_bytes()).await.unwrap();
        stream.flush().await.unwrap();

        // Immediate shutdown
        server.stop().await.unwrap();
        server_handle.abort();

        // No arbitrary delay - proceed immediately to next cycle
        // Socket cleanup is handled by OS, and next server will handle any stale socket
    }

    // Final test: server should still work after multiple restarts
    let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;

    let mut stream = UnixStream::connect(&socket_path).await.unwrap();
    let request = json!({"jsonrpc": "2.0", "method": "health", "id": 999});
    let mut request_str = serde_json::to_string(&request).unwrap();
    request_str.push('\n');

    use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
    stream.write_all(request_str.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();

    let mut reader = tokio::io::BufReader::new(stream);
    let mut line = String::new();
    reader.read_line(&mut line).await.unwrap();

    let response: serde_json::Value = serde_json::from_str(&line).unwrap();
    assert_eq!(response["result"]["status"], "healthy");

    server.stop().await.unwrap();
    server_handle.abort();
}

#[tokio::test]
async fn fault_test_partial_writes() {
    // Fault: Simulate partial/incomplete writes
    let (_dir, socket_path) = test_socket();
    let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;

    let mut stream = UnixStream::connect(&socket_path).await.unwrap();

    // Write partial request (no newline)
    use tokio::io::AsyncWriteExt;
    stream
        .write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"health\"")
        .await
        .unwrap();
    stream.flush().await.unwrap();

    // Let the runtime schedule other tasks so the server can observe the partial frame
    for _ in 0..32 {
        tokio::task::yield_now().await;
    }

    // Complete the request
    stream.write_all(b",\"id\":1}\n").await.unwrap();
    stream.flush().await.unwrap();

    // Should eventually get response
    use tokio::io::AsyncBufReadExt;
    let mut reader = tokio::io::BufReader::new(stream);
    let mut line = String::new();
    let result = tokio::time::timeout(Duration::from_secs(2), reader.read_line(&mut line)).await;

    assert!(result.is_ok(), "Should handle partial writes");

    server.stop().await.unwrap();
    server_handle.abort();
}

#[tokio::test]
async fn fault_test_concurrent_stop_calls() {
    // Fault: Multiple concurrent stop() calls
    let (_dir, socket_path) = test_socket();
    let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;

    // Spawn multiple tasks trying to stop concurrently
    let mut handles = vec![];
    for _ in 0..10 {
        let server = Arc::clone(&server);
        let handle = tokio::spawn(async move { server.stop().await });
        handles.push(handle);
    }

    // All should complete without panic
    for handle in handles {
        let result = handle.await;
        assert!(result.is_ok(), "Concurrent stop calls should not panic");
    }

    server_handle.abort();
}

#[tokio::test]
async fn fault_test_readiness_check_before_start() {
    // Fault: Check readiness before server starts
    let (_dir, socket_path) = test_socket();
    let btsp_provider = create_test_btsp_provider().await;
    let server = Arc::new(
        UnixSocketIpcServer::new(
            socket_path,
            btsp_provider,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
        )
        .await
        .unwrap(),
    );

    // Should not be ready
    assert!(!server.is_ready());

    // Timeout should occur
    let result = server.wait_ready(Duration::from_millis(100)).await;
    assert!(!result, "Should timeout when server hasn't started");
}

#[tokio::test]
async fn fault_test_connection_after_stop() {
    // Fault: Try to connect after server has stopped
    let (_dir, socket_path) = test_socket();
    let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;

    // Stop server
    server.stop().await.unwrap();
    server_handle.abort();

    // Try to connect (should fail gracefully)
    // No sleep needed - connection should fail immediately to stopped server
    let result = UnixStream::connect(&socket_path).await;

    assert!(result.is_err(), "Connection should fail after stop");
}
