// SPDX-License-Identifier: AGPL-3.0-only
//! Integration tests for Unix Socket IPC
//!
//! These tests verify the complete IPC flow for `BearDog`'s Unix socket server:
//! 1. Server startup and socket creation
//! 2. Atomic readiness flag functionality
//! 3. Client connection and JSON-RPC communication
//! 4. Concurrent connections
//! 5. Graceful shutdown
//!
//! ## Modern Concurrent Testing
//!
//! These tests use atomic readiness flags instead of sleep-based polling,
//! ensuring true concurrency and fast test execution. This is modern idiomatic Rust!

mod support;

use anyhow::Result;
use serde_json::json;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tempfile::TempDir;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

use beardog_genetics::EcosystemGeneticEngine;
use beardog_tunnel::btsp_provider::BeardogBtspProvider;
use beardog_tunnel::tunnel::hsm::manager::{HsmAutoInitConfig, HsmManager};
use beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer;
use beardog_types::primal_identity::PrimalIdentity;

use support::wait_for_deletion;

// Helper to create a test socket path with temp dir
fn test_socket() -> (TempDir, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let socket_path = dir.path().join("test-beardog.sock");
    (dir, socket_path)
}

// Helper to create minimal BTSP provider for testing
async fn create_test_btsp_provider() -> Arc<BeardogBtspProvider> {
    let hsm = Arc::new(
        HsmManager::auto_initialize_with_config(HsmAutoInitConfig {
            mode: "software".to_string(),
            auto_init: true,
        })
        .await
        .expect("HSM init failed"),
    );
    let genetics = Arc::new(EcosystemGeneticEngine::new().expect("Genetics init failed"));

    Arc::new(
        BeardogBtspProvider::new(hsm, genetics)
            .await
            .expect("BTSP provider init failed"),
    )
}

// Helper to start server and wait for readiness (atomic, no sleep!)
async fn start_server_ready(
    socket_path: PathBuf,
) -> (
    tokio::task::JoinHandle<()>,
    Arc<AtomicBool>,
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

    // Wait for server to be ready (atomic check, no filesystem polling!)
    assert!(
        UnixSocketIpcServer::wait_ready_flag(&ready_flag, tokio::time::Duration::from_secs(10))
            .await,
        "Server should become ready within 10 seconds"
    );

    (handle, ready_flag, server)
}

// Helper to send JSON-RPC request and receive response
async fn send_request(
    stream: &mut UnixStream,
    request: serde_json::Value,
) -> Result<serde_json::Value> {
    // Send request
    let request_str = serde_json::to_string(&request)?;
    stream.write_all(request_str.as_bytes()).await?;
    stream.write_all(b"\n").await?;
    stream.flush().await?;

    // Read response
    let (reader, _writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();
    reader.read_line(&mut line).await?;

    // Parse response
    let response: serde_json::Value = serde_json::from_str(&line)?;
    Ok(response)
}

#[tokio::test]
async fn test_socket_creation() {
    let (_dir, socket_path) = test_socket();
    let btsp_provider = create_test_btsp_provider().await;

    let server = UnixSocketIpcServer::new(
        socket_path.clone(),
        btsp_provider,
        Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
    )
    .await
    .unwrap();

    assert_eq!(server.socket_path(), socket_path);
    // Note: Socket is created on start(), not new()
}

#[tokio::test]
async fn test_readiness_flag() {
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

    // Should not be ready initially
    assert!(!server.is_ready());

    // Get readiness flag before starting
    let ready_flag = server.readiness_flag();

    // Start server
    let server_clone = Arc::clone(&server);
    let handle = tokio::spawn(async move {
        server_clone.start().await.unwrap();
    });

    // Wait for readiness (atomic, lock-free!)
    assert!(server.wait_ready(std::time::Duration::from_secs(10)).await);
    assert!(server.is_ready());

    // Verify flag is also set
    assert!(ready_flag.load(std::sync::atomic::Ordering::Acquire));

    handle.abort();
}

#[tokio::test]
async fn test_health_check() {
    let (_dir, socket_path) = test_socket();

    let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;

    let mut stream = UnixStream::connect(&socket_path).await.unwrap();

    let request = json!({
        "jsonrpc": "2.0",
        "method": "health",
        "id": 1
    });

    let response = send_request(&mut stream, request).await.unwrap();

    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], 1);
    assert_eq!(response["result"]["status"], "healthy");

    // Graceful shutdown
    server.stop().await.unwrap();
    server_handle.abort();
}

#[tokio::test]
async fn test_concurrent_connections() {
    let (_dir, socket_path) = test_socket();

    let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;

    // Spawn 10 concurrent clients
    let mut handles = vec![];

    for i in 0..10 {
        let socket_path = socket_path.clone();
        let handle = tokio::spawn(async move {
            let mut stream = UnixStream::connect(&socket_path).await.unwrap();

            let request = json!({
                "jsonrpc": "2.0",
                "method": "health",
                "id": i
            });

            let response = send_request(&mut stream, request).await.unwrap();
            assert_eq!(response["id"], i);
            assert_eq!(response["result"]["status"], "healthy");
        });

        handles.push(handle);
    }

    // Wait for all clients
    for handle in handles {
        handle.await.unwrap();
    }

    // Graceful shutdown
    server.stop().await.unwrap();
    server_handle.abort();
}

#[tokio::test]
async fn test_graceful_shutdown() {
    let (_dir, socket_path) = test_socket();

    let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;

    // Verify socket exists
    assert!(socket_path.exists());

    // Graceful stop
    server.stop().await.unwrap();

    // Verify socket removed
    assert!(!socket_path.exists());

    // Verify not ready anymore
    assert!(!server.is_ready());

    server_handle.abort();
}

#[tokio::test]
async fn test_invalid_json_rpc() {
    let (_dir, socket_path) = test_socket();

    let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;

    let mut stream = UnixStream::connect(&socket_path).await.unwrap();

    // Send invalid JSON-RPC (wrong version)
    let request = json!({
        "jsonrpc": "1.0",
        "method": "health",
        "id": 1
    });

    let response = send_request(&mut stream, request).await.unwrap();

    // Should get an error response
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["error"].is_object());

    server.stop().await.unwrap();
    server_handle.abort();
}

#[tokio::test]
async fn test_method_not_found() {
    let (_dir, socket_path) = test_socket();

    let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;

    let mut stream = UnixStream::connect(&socket_path).await.unwrap();

    // Call non-existent method
    let request = json!({
        "jsonrpc": "2.0",
        "method": "nonexistent.method",
        "id": 1
    });

    let response = send_request(&mut stream, request).await.unwrap();

    // Should get METHOD_NOT_FOUND error
    assert_eq!(response["jsonrpc"], "2.0");
    assert_eq!(response["id"], 1);
    assert!(response["error"].is_object());
    assert_eq!(response["error"]["code"], -32601); // METHOD_NOT_FOUND

    server.stop().await.unwrap();
    server_handle.abort();
}

#[tokio::test]
async fn test_socket_cleanup_on_crash() {
    let (_dir, socket_path) = test_socket();

    // Start first server
    {
        let (server_handle, _ready_flag, _server) = start_server_ready(socket_path.clone()).await;

        // Verify socket exists
        assert!(socket_path.exists());

        // Simulate crash (just abort without graceful stop)
        server_handle.abort();

        let _ = wait_for_deletion(&socket_path, std::time::Duration::from_secs(2)).await;

        // If socket still exists after crash, it will be cleaned up by next server
        // This is the expected behavior - no need to wait longer
    }

    // Start second server with same socket path
    // Should handle cleanup of stale socket
    let btsp_provider = create_test_btsp_provider().await;
    let server = Arc::new(
        UnixSocketIpcServer::new(
            socket_path.clone(),
            btsp_provider,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
        )
        .await
        .unwrap(),
    );

    let server_clone = Arc::clone(&server);
    let handle = tokio::spawn(async move {
        server_clone.start().await.unwrap();
    });

    // Wait for readiness
    assert!(server.wait_ready(std::time::Duration::from_secs(10)).await);
    assert!(socket_path.exists());

    // Cleanup
    server.stop().await.unwrap();
    handle.abort();
}

#[tokio::test]
async fn test_wait_ready_timeout() {
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

    // Don't start server, just try to wait
    let result = server
        .wait_ready(std::time::Duration::from_millis(100))
        .await;

    // Should timeout
    assert!(!result);
}

#[tokio::test]
async fn test_multiple_clients_sequential() {
    let (_dir, socket_path) = test_socket();

    let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;

    // Connect and disconnect multiple clients sequentially
    for i in 0..5 {
        let mut stream = UnixStream::connect(&socket_path).await.unwrap();

        let request = json!({
            "jsonrpc": "2.0",
            "method": "health",
            "id": i
        });

        let response = send_request(&mut stream, request).await.unwrap();
        assert_eq!(response["id"], i);
        assert_eq!(response["result"]["status"], "healthy");

        // Drop stream (disconnect)
        drop(stream);
    }

    // Server should still be running
    assert!(server.is_ready());

    server.stop().await.unwrap();
    server_handle.abort();
}
