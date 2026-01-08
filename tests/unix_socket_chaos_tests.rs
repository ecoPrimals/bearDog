//! Chaos Tests for Unix Socket IPC Evolution
//!
//! Tests the resilience of the lock-free concurrent Unix socket implementation
//! under chaotic conditions:
//! - Concurrent connection storms
//! - Rapid connect/disconnect cycles
//! - Race conditions in readiness checks
//! - Simultaneous shutdown and connection attempts
//! - High load concurrent operations

use anyhow::Result;
use serde_json::json;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tempfile::TempDir;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

use beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer;
use beardog_tunnel::btsp_provider::BeardogBtspProvider;
use beardog_tunnel::tunnel::hsm::HsmManager;
use beardog_genetics::EcosystemGeneticEngine;

// Helper to create test socket
fn test_socket() -> (TempDir, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let socket_path = dir.path().join("chaos-test.sock");
    (dir, socket_path)
}

// Helper to create test BTSP provider
async fn create_test_btsp_provider() -> Arc<BeardogBtspProvider> {
    std::env::set_var("BEARDOG_HSM_MODE", "software");
    let hsm = Arc::new(HsmManager::auto_initialize().await.expect("HSM init"));
    let genetics = Arc::new(EcosystemGeneticEngine::new().expect("Genetics init"));
    Arc::new(BeardogBtspProvider::new(hsm, genetics).await.expect("BTSP init"))
}

// Helper to start server
async fn start_server_ready(
    socket_path: PathBuf,
) -> (tokio::task::JoinHandle<()>, Arc<AtomicBool>, Arc<UnixSocketIpcServer>) {
    let btsp_provider = create_test_btsp_provider().await;
    let server = Arc::new(UnixSocketIpcServer::new(socket_path, btsp_provider).await.unwrap());
    let ready_flag = server.readiness_flag();
    let server_clone = Arc::clone(&server);
    
    let handle = tokio::spawn(async move {
        server_clone.start().await.unwrap();
    });
    
    assert!(UnixSocketIpcServer::wait_ready_flag(&ready_flag, Duration::from_secs(10)).await);
    (handle, ready_flag, server)
}

// Helper to send request
async fn send_request(stream: &mut UnixStream, request: serde_json::Value) -> Result<serde_json::Value> {
    let request_str = serde_json::to_string(&request)?;
    stream.write_all(request_str.as_bytes()).await?;
    stream.write_all(b"\n").await?;
    stream.flush().await?;

    let (reader, _writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();
    reader.read_line(&mut line).await?;
    
    Ok(serde_json::from_str(&line)?)
}

#[tokio::test]
async fn chaos_test_connection_storm() {
    // Test: 100 concurrent connections hitting the server simultaneously
    let (_dir, socket_path) = test_socket();
    let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;
    
    let success_count = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    // Storm: 100 concurrent clients
    for i in 0..100 {
        let socket_path = socket_path.clone();
        let success_count = Arc::clone(&success_count);
        
        let handle = tokio::spawn(async move {
            if let Ok(mut stream) = UnixStream::connect(&socket_path).await {
                let request = json!({
                    "jsonrpc": "2.0",
                    "method": "health",
                    "id": i
                });
                
                if let Ok(response) = send_request(&mut stream, request).await {
                    if response["result"]["status"] == "healthy" {
                        success_count.fetch_add(1, Ordering::SeqCst);
                    }
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all clients
    for handle in handles {
        handle.await.unwrap();
    }
    
    let successes = success_count.load(Ordering::SeqCst);
    
    // Should handle most connections gracefully
    assert!(successes >= 90, "Expected >= 90 successes, got {}", successes);
    
    server.stop().await.unwrap();
    server_handle.abort();
}

#[tokio::test]
async fn chaos_test_rapid_connect_disconnect() {
    // Test: Rapid connect/disconnect cycles to test connection cleanup
    let (_dir, socket_path) = test_socket();
    let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;
    
    // Rapid cycles: connect, send, disconnect
    for i in 0..50 {
        let mut stream = UnixStream::connect(&socket_path).await.unwrap();
        
        let request = json!({
            "jsonrpc": "2.0",
            "method": "health",
            "id": i
        });
        
        let response = send_request(&mut stream, request).await.unwrap();
        assert_eq!(response["result"]["status"], "healthy");
        
        // Immediate disconnect
        drop(stream);
        
        // No delay - immediate next connection
    }
    
    // Server should still be responsive
    let mut stream = UnixStream::connect(&socket_path).await.unwrap();
    let request = json!({"jsonrpc": "2.0", "method": "health", "id": 999});
    let response = send_request(&mut stream, request).await.unwrap();
    assert_eq!(response["result"]["status"], "healthy");
    
    server.stop().await.unwrap();
    server_handle.abort();
}

#[tokio::test]
async fn chaos_test_readiness_race_condition() {
    // Test: Multiple threads racing to check readiness
    let (_dir, socket_path) = test_socket();
    let btsp_provider = create_test_btsp_provider().await;
    let server = Arc::new(UnixSocketIpcServer::new(socket_path, btsp_provider).await.unwrap());
    
    let ready_flag = server.readiness_flag();
    let race_count = Arc::new(AtomicUsize::new(0));
    
    // Start server in background
    let server_clone = Arc::clone(&server);
    let server_handle = tokio::spawn(async move {
        server_clone.start().await.unwrap();
    });
    
    // Race: 50 concurrent threads checking readiness
    let mut handles = vec![];
    for _ in 0..50 {
        let ready_flag = Arc::clone(&ready_flag);
        let race_count = Arc::clone(&race_count);
        
        let handle = tokio::spawn(async move {
            if UnixSocketIpcServer::wait_ready_flag(&ready_flag, Duration::from_secs(10)).await {
                race_count.fetch_add(1, Ordering::SeqCst);
            }
        });
        
        handles.push(handle);
    }
    
    // All should succeed
    for handle in handles {
        handle.await.unwrap();
    }
    
    assert_eq!(race_count.load(Ordering::SeqCst), 50, "All readiness checks should succeed");
    
    server.stop().await.unwrap();
    server_handle.abort();
}

#[tokio::test]
async fn chaos_test_shutdown_during_connections() {
    // Test: Shutdown server while connections are being established
    let (_dir, socket_path) = test_socket();
    let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;
    
    let stop_flag = Arc::new(AtomicBool::new(false));
    let connection_count = Arc::new(AtomicUsize::new(0));
    
    // Spawn connection threads
    let mut handles = vec![];
    for _ in 0..20 {
        let socket_path = socket_path.clone();
        let stop_flag = Arc::clone(&stop_flag);
        let connection_count = Arc::clone(&connection_count);
        
        let handle = tokio::spawn(async move {
            while !stop_flag.load(Ordering::SeqCst) {
                if let Ok(mut stream) = UnixStream::connect(&socket_path).await {
                    let request = json!({"jsonrpc": "2.0", "method": "health", "id": 1});
                    if send_request(&mut stream, request).await.is_ok() {
                        connection_count.fetch_add(1, Ordering::SeqCst);
                    }
                }
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        });
        
        handles.push(handle);
    }
    
    // Let connections run for a bit
    tokio::time::sleep(Duration::from_millis(200)).await;
    
    // Graceful shutdown while connections are active
    server.stop().await.unwrap();
    stop_flag.store(true, Ordering::SeqCst);
    server_handle.abort();
    
    // Wait for connection threads
    for handle in handles {
        handle.await.unwrap();
    }
    
    // Should have handled some connections
    assert!(connection_count.load(Ordering::SeqCst) > 0, "Should have handled some connections");
}

#[tokio::test]
async fn chaos_test_concurrent_request_flood() {
    // Test: Flood server with concurrent requests from multiple connections
    let (_dir, socket_path) = test_socket();
    let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;
    
    let total_requests = Arc::new(AtomicUsize::new(0));
    let successful_requests = Arc::new(AtomicUsize::new(0));
    
    // 10 connections, each sending 50 requests concurrently
    let mut handles = vec![];
    for conn_id in 0..10 {
        let socket_path = socket_path.clone();
        let total_requests = Arc::clone(&total_requests);
        let successful_requests = Arc::clone(&successful_requests);
        
        let handle = tokio::spawn(async move {
            if let Ok(mut stream) = UnixStream::connect(&socket_path).await {
                for req_id in 0..50 {
                    total_requests.fetch_add(1, Ordering::SeqCst);
                    
                    let request = json!({
                        "jsonrpc": "2.0",
                        "method": "health",
                        "id": format!("{}:{}", conn_id, req_id)
                    });
                    
                    if let Ok(response) = send_request(&mut stream, request).await {
                        if response["result"]["status"] == "healthy" {
                            successful_requests.fetch_add(1, Ordering::SeqCst);
                        }
                    }
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all requests
    for handle in handles {
        handle.await.unwrap();
    }
    
    let total = total_requests.load(Ordering::SeqCst);
    let successful = successful_requests.load(Ordering::SeqCst);
    
    // Should handle most requests (allow some failures under extreme load)
    let success_rate = (successful as f64) / (total as f64);
    assert!(success_rate >= 0.95, "Success rate {} below 95%", success_rate);
    
    server.stop().await.unwrap();
    server_handle.abort();
}

#[tokio::test]
async fn chaos_test_atomic_readiness_under_load() {
    // Test: Verify atomic readiness flag remains consistent under load
    let (_dir, socket_path) = test_socket();
    let btsp_provider = create_test_btsp_provider().await;
    let server = Arc::new(UnixSocketIpcServer::new(socket_path.clone(), btsp_provider).await.unwrap());
    
    let ready_flag = server.readiness_flag();
    let inconsistency_count = Arc::new(AtomicUsize::new(0));
    
    // Start server
    let server_clone = Arc::clone(&server);
    let server_handle = tokio::spawn(async move {
        server_clone.start().await.unwrap();
    });
    
    // Wait for ready
    UnixSocketIpcServer::wait_ready_flag(&ready_flag, Duration::from_secs(10)).await;
    
    // Spawn threads that continuously check readiness
    let mut handles = vec![];
    for _ in 0..20 {
        let ready_flag = Arc::clone(&ready_flag);
        let inconsistency_count = Arc::clone(&inconsistency_count);
        
        let handle = tokio::spawn(async move {
            for _ in 0..1000 {
                let is_ready = ready_flag.load(Ordering::Acquire);
                // Once ready, should stay ready until explicit stop
                if !is_ready {
                    inconsistency_count.fetch_add(1, Ordering::SeqCst);
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for checks
    for handle in handles {
        handle.await.unwrap();
    }
    
    // Should have zero inconsistencies (atomic guarantees)
    assert_eq!(
        inconsistency_count.load(Ordering::SeqCst),
        0,
        "Atomic readiness flag should be consistent"
    );
    
    server.stop().await.unwrap();
    server_handle.abort();
}

#[tokio::test]
async fn chaos_test_error_handling_under_pressure() {
    // Test: Mix of valid and invalid requests under load
    let (_dir, socket_path) = test_socket();
    let (server_handle, _ready_flag, server) = start_server_ready(socket_path.clone()).await;
    
    let valid_responses = Arc::new(AtomicUsize::new(0));
    let error_responses = Arc::new(AtomicUsize::new(0));
    
    // 50 concurrent clients with mixed requests
    let mut handles = vec![];
    for i in 0..50 {
        let socket_path = socket_path.clone();
        let valid_responses = Arc::clone(&valid_responses);
        let error_responses = Arc::clone(&error_responses);
        
        let handle = tokio::spawn(async move {
            if let Ok(mut stream) = UnixStream::connect(&socket_path).await {
                // Send mix of valid and invalid requests
                let request = if i % 3 == 0 {
                    // Invalid: wrong JSON-RPC version
                    json!({"jsonrpc": "1.0", "method": "health", "id": i})
                } else if i % 3 == 1 {
                    // Invalid: unknown method
                    json!({"jsonrpc": "2.0", "method": "invalid.method", "id": i})
                } else {
                    // Valid
                    json!({"jsonrpc": "2.0", "method": "health", "id": i})
                };
                
                if let Ok(response) = send_request(&mut stream, request).await {
                    if response["result"].is_object() {
                        valid_responses.fetch_add(1, Ordering::SeqCst);
                    } else if response["error"].is_object() {
                        error_responses.fetch_add(1, Ordering::SeqCst);
                    }
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all
    for handle in handles {
        handle.await.unwrap();
    }
    
    let valid = valid_responses.load(Ordering::SeqCst);
    let errors = error_responses.load(Ordering::SeqCst);
    
    // Should have proper mix of valid and error responses
    assert!(valid > 0, "Should have valid responses");
    assert!(errors > 0, "Should have error responses");
    assert_eq!(valid + errors, 50, "Should handle all requests");
    
    server.stop().await.unwrap();
    server_handle.abort();
}

