// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(clippy::expect_used, clippy::unwrap_used)]
#![allow(
    missing_docs,
    clippy::float_cmp,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::cast_possible_wrap,
    clippy::redundant_clone,
    clippy::needless_collect
)]
//! Performance tests for Collaborative Intelligence graph security
//!
//! Tests throughput, latency, and concurrency under various loads

use serde_json::json;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

use beardog_tunnel::btsp_handshake::BtspSecurityMode;
use beardog_types::primal_identity::PrimalIdentity;

/// Test helper: Create a test BTSP provider
async fn create_test_btsp_provider() -> Arc<beardog_tunnel::btsp_provider::BeardogBtspProvider> {
    use beardog_genetics::EcosystemGeneticEngine;
    use beardog_tunnel::tunnel::hsm::manager::HsmManager;

    let hsm = Arc::new(HsmManager::auto_initialize().await.expect("HSM init"));
    let genetics = Arc::new(EcosystemGeneticEngine::new().expect("Genetics init"));

    Arc::new(
        beardog_tunnel::btsp_provider::BeardogBtspProvider::new(hsm, genetics)
            .await
            .expect("BTSP provider init"),
    )
}

/// Test helper: Send JSON-RPC request and receive response
async fn send_jsonrpc_request_timed(
    method: &str,
    params: serde_json::Value,
    socket_path: &str,
) -> (serde_json::Value, Duration) {
    let start = Instant::now();
    let mut stream = UnixStream::connect(socket_path).await.unwrap();

    let request = json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1,
    });

    let request_str = serde_json::to_string(&request).unwrap();
    stream.write_all(request_str.as_bytes()).await.unwrap();
    stream.write_all(b"\n").await.unwrap();
    stream.flush().await.unwrap();

    let mut reader = BufReader::new(&mut stream);
    let mut response_line = String::new();
    reader.read_line(&mut response_line).await.unwrap();

    let elapsed = start.elapsed();
    let response = serde_json::from_str(&response_line).unwrap();

    (response, elapsed)
}

// ============================================================================
// Throughput Tests
// ============================================================================

#[tokio::test]
async fn test_authorization_throughput() {
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-perf-auth-throughput.sock";

    let _ = std::fs::remove_file(socket_path);

    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            socket_path,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
            BtspSecurityMode::Development,
        )
        .await
        .expect("Server creation"),
    );

    let ready_flag = server.readiness_flag();
    let server_clone = Arc::clone(&server);
    let server_task = tokio::spawn(async move {
        server_clone.start().await.unwrap();
    });

    let ready = beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::wait_ready_flag(
        &ready_flag,
        Duration::from_secs(5),
    )
    .await;
    assert!(ready);

    // Run 100 authorization requests
    let start = Instant::now();
    let num_requests = 100;

    for i in 0..num_requests {
        let params = json!({
            "user_id": "alice",
            "graph": {
                "id": format!("graph-{}", i),
                "owner": "alice",
                "nodes": [{
                    "id": "node-1",
                    "type": "compute",
                    "primal": "ToadStool",
                    "config": {}
                }],
                "edges": [],
                "metadata": {}
            },
            "modification": {
                "action": "add_node",
                "node": {
                    "id": format!("new-node-{}", i),
                    "type": "compute",
                    "primal": "ToadStool",
                    "config": {}
                }
            }
        });

        let (response, _) =
            send_jsonrpc_request_timed("graph.authorize_modification", params, socket_path).await;
        assert_eq!(response["jsonrpc"], "2.0");
    }

    let elapsed = start.elapsed();
    let throughput = f64::from(num_requests) / elapsed.as_secs_f64();

    println!("Authorization throughput: {throughput:.2} req/sec");
    println!(
        "Average latency: {:.2} ms",
        elapsed.as_millis() as f64 / f64::from(num_requests)
    );

    // Verify we can handle at least 50 req/sec
    assert!(
        throughput > 50.0,
        "Throughput should be > 50 req/sec, got {throughput:.2}"
    );

    // Cleanup
    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}

#[tokio::test]
async fn test_validation_throughput() {
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-perf-validate-throughput.sock";

    let _ = std::fs::remove_file(socket_path);

    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            socket_path,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
            BtspSecurityMode::Development,
        )
        .await
        .expect("Server creation"),
    );

    let ready_flag = server.readiness_flag();
    let server_clone = Arc::clone(&server);
    let server_task = tokio::spawn(async move {
        server_clone.start().await.unwrap();
    });

    let ready = beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::wait_ready_flag(
        &ready_flag,
        Duration::from_secs(5),
    )
    .await;
    assert!(ready);

    // Run 100 validation requests
    let start = Instant::now();
    let num_requests = 100;

    for i in 0..num_requests {
        let params = json!({
            "template": {
                "id": format!("template-{}", i),
                "name": "Test Template",
                "creator": "alice",
                "nodes": [{
                    "id": "node-1",
                    "type": "compute",
                    "primal": "ToadStool",
                    "config": {}
                }],
                "edges": [],
                "metadata": {
                    "version": "1.0",
                    "created_at": "2026-01-11T12:00:00Z"
                }
            }
        });

        let (response, _) =
            send_jsonrpc_request_timed("graph.validate_template", params, socket_path).await;
        assert_eq!(response["jsonrpc"], "2.0");
    }

    let elapsed = start.elapsed();
    let throughput = f64::from(num_requests) / elapsed.as_secs_f64();

    println!("Validation throughput: {throughput:.2} req/sec");
    println!(
        "Average latency: {:.2} ms",
        elapsed.as_millis() as f64 / f64::from(num_requests)
    );

    // Verify we can handle at least 50 req/sec
    assert!(
        throughput > 50.0,
        "Throughput should be > 50 req/sec, got {throughput:.2}"
    );

    // Cleanup
    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}

// ============================================================================
// Latency Tests
// ============================================================================

#[tokio::test]
async fn test_authorization_latency_p95() {
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-perf-auth-latency.sock";

    let _ = std::fs::remove_file(socket_path);

    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            socket_path,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
            BtspSecurityMode::Development,
        )
        .await
        .expect("Server creation"),
    );

    let ready_flag = server.readiness_flag();
    let server_clone = Arc::clone(&server);
    let server_task = tokio::spawn(async move {
        server_clone.start().await.unwrap();
    });

    let ready = beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::wait_ready_flag(
        &ready_flag,
        Duration::from_secs(5),
    )
    .await;
    assert!(ready);

    // Measure latency for 100 requests
    let mut latencies = Vec::new();

    for i in 0..100 {
        let params = json!({
            "user_id": "alice",
            "graph": {
                "id": format!("graph-{}", i),
                "owner": "alice",
                "nodes": [],
                "edges": [],
                "metadata": {}
            },
            "modification": {
                "action": "add_node",
                "node": {
                    "id": "new-node",
                    "type": "compute",
                    "primal": "ToadStool",
                    "config": {}
                }
            }
        });

        let (response, latency) =
            send_jsonrpc_request_timed("graph.authorize_modification", params, socket_path).await;
        assert_eq!(response["jsonrpc"], "2.0");
        latencies.push(latency);
    }

    // Calculate percentiles
    latencies.sort();
    let p50 = latencies[50].as_millis();
    let p95 = latencies[95].as_millis();
    let p99 = latencies[99].as_millis();

    println!("Authorization latency:");
    println!("  p50: {p50} ms");
    println!("  p95: {p95} ms");
    println!("  p99: {p99} ms");

    // Verify p95 is under 100ms (reasonable for development)
    assert!(p95 < 100, "p95 latency should be < 100ms, got {p95} ms");

    // Cleanup
    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}

// ============================================================================
// Concurrency Tests
// ============================================================================

#[tokio::test]
async fn test_concurrent_authorization_requests() {
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-perf-concurrent.sock";

    let _ = std::fs::remove_file(socket_path);

    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            socket_path,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
            BtspSecurityMode::Development,
        )
        .await
        .expect("Server creation"),
    );

    let ready_flag = server.readiness_flag();
    let server_clone = Arc::clone(&server);
    let server_task = tokio::spawn(async move {
        server_clone.start().await.unwrap();
    });

    let ready = beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::wait_ready_flag(
        &ready_flag,
        Duration::from_secs(5),
    )
    .await;
    assert!(ready);

    // Launch 10 concurrent clients
    let start = Instant::now();
    let num_clients = 10;
    let requests_per_client = 10;

    let mut handles = Vec::new();

    for client_id in 0..num_clients {
        let socket_path = socket_path.to_string();
        let handle = tokio::spawn(async move {
            for i in 0..requests_per_client {
                let params = json!({
                    "user_id": format!("user-{}", client_id),
                    "graph": {
                        "id": format!("graph-{}-{}", client_id, i),
                        "owner": format!("user-{}", client_id),
                        "nodes": [],
                        "edges": [],
                        "metadata": {}
                    },
                    "modification": {
                        "action": "add_node",
                        "node": {
                            "id": "new-node",
                            "type": "compute",
                            "primal": "ToadStool",
                            "config": {}
                        }
                    }
                });

                let (response, _) = send_jsonrpc_request_timed(
                    "graph.authorize_modification",
                    params,
                    &socket_path,
                )
                .await;
                assert_eq!(response["jsonrpc"], "2.0");
            }
        });
        handles.push(handle);
    }

    // Wait for all clients to complete
    for handle in handles {
        handle.await.unwrap();
    }

    let elapsed = start.elapsed();
    let total_requests = num_clients * requests_per_client;
    let throughput = f64::from(total_requests) / elapsed.as_secs_f64();

    println!(
        "Concurrent throughput: {throughput:.2} req/sec ({num_clients} clients, {requests_per_client} req each)"
    );

    // Verify concurrent requests work
    assert!(throughput > 0.0);

    // Cleanup
    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}

// ============================================================================
// Load Tests
// ============================================================================

#[tokio::test]
async fn test_sustained_load() {
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-perf-sustained.sock";

    let _ = std::fs::remove_file(socket_path);

    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            socket_path,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
            BtspSecurityMode::Development,
        )
        .await
        .expect("Server creation"),
    );

    let ready_flag = server.readiness_flag();
    let server_clone = Arc::clone(&server);
    let server_task = tokio::spawn(async move {
        server_clone.start().await.unwrap();
    });

    let ready = beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::wait_ready_flag(
        &ready_flag,
        Duration::from_secs(5),
    )
    .await;
    assert!(ready);

    // Run sustained load for 5 seconds
    let start = Instant::now();
    let duration = Duration::from_secs(5);
    let mut request_count = 0;

    while start.elapsed() < duration {
        let params = json!({
            "user_id": "alice",
            "graph": {
                "id": format!("graph-{}", request_count),
                "owner": "alice",
                "nodes": [],
                "edges": [],
                "metadata": {}
            },
            "modification": {
                "action": "add_node",
                "node": {
                    "id": "new-node",
                    "type": "compute",
                    "primal": "ToadStool",
                    "config": {}
                }
            }
        });

        let (response, _) =
            send_jsonrpc_request_timed("graph.authorize_modification", params, socket_path).await;
        assert_eq!(response["jsonrpc"], "2.0");
        request_count += 1;
    }

    let elapsed = start.elapsed();
    let throughput = f64::from(request_count) / elapsed.as_secs_f64();

    println!(
        "Sustained load throughput: {:.2} req/sec over {} seconds",
        throughput,
        elapsed.as_secs()
    );

    // Verify sustained performance
    assert!(
        throughput > 30.0,
        "Sustained throughput should be > 30 req/sec"
    );

    // Cleanup
    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}
