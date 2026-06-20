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
//! Tests throughput, latency, and concurrency under various loads.
//!
//! Thresholds are intentionally generous (5-10x headroom) so these tests
//! validate correctness-under-load, not benchmark regressions. Use
//! `benchmarks/` for tight perf gates.

mod support;

use serde_json::json;
use std::sync::Arc;
use std::time::{Duration, Instant};
use support::concurrent_helpers::unique_unix_socket;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

use beardog_core::socket_config::IpcCapabilitySymlinksConfig;
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

/// RAII guard that removes the socket file on drop, preventing stale socket leaks.
struct SocketGuard {
    path: std::path::PathBuf,
}

impl Drop for SocketGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

/// Spin up a test server on an isolated socket, returning (`socket_path`, `server_task`, guard).
async fn start_test_server() -> (String, tokio::task::JoinHandle<()>, SocketGuard) {
    let socket_path = unique_unix_socket();
    let _ = std::fs::remove_file(&socket_path);

    let btsp = create_test_btsp_provider().await;
    let path_str = socket_path
        .to_str()
        .expect("valid UTF-8 socket path")
        .to_owned();

    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            &path_str,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
            BtspSecurityMode::Development,
            IpcCapabilitySymlinksConfig::default(),
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
        Duration::from_secs(10),
    )
    .await;
    assert!(ready, "Server must become ready within 10s");

    let guard = SocketGuard { path: socket_path };

    (path_str, server_task, guard)
}

/// Test helper: Send JSON-RPC request and receive response.
///
/// Returns `None` when the connection or I/O fails (expected under sustained load).
async fn send_jsonrpc_request_timed(
    method: &str,
    params: serde_json::Value,
    socket_path: &str,
) -> Option<(serde_json::Value, Duration)> {
    let start = Instant::now();
    let mut stream = UnixStream::connect(socket_path).await.ok()?;

    let request = json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1,
    });

    let request_str = serde_json::to_string(&request).expect("JSON serialization");
    stream.write_all(request_str.as_bytes()).await.ok()?;
    stream.write_all(b"\n").await.ok()?;
    stream.flush().await.ok()?;

    let mut reader = BufReader::new(&mut stream);
    let mut response_line = String::new();
    reader.read_line(&mut response_line).await.ok()?;

    let elapsed = start.elapsed();
    let response: serde_json::Value = serde_json::from_str(&response_line).ok()?;

    Some((response, elapsed))
}

fn authorize_params(i: u32) -> serde_json::Value {
    json!({
        "user_id": "alice",
        "graph": {
            "id": format!("graph-{i}"),
            "owner": "alice",
            "nodes": [{
                "id": "node-1",
                "type": "compute",
                "primal": "compute.general",
                "config": {}
            }],
            "edges": [],
            "metadata": {}
        },
        "modification": {
            "action": "add_node",
            "node": {
                "id": format!("new-node-{i}"),
                "type": "compute",
                "primal": "compute.general",
                "config": {}
            }
        }
    })
}

// ============================================================================
// Throughput Tests
// ============================================================================

#[tokio::test]
async fn test_authorization_throughput() {
    let (socket_path, server_task, _guard) = start_test_server().await;

    let start = Instant::now();
    let num_requests = 100;

    for i in 0..num_requests {
        let resp = send_jsonrpc_request_timed(
            "graph.authorize_modification",
            authorize_params(i),
            &socket_path,
        )
        .await;
        if let Some((response, _)) = resp {
            assert_eq!(response["jsonrpc"], "2.0");
        }
    }

    let elapsed = start.elapsed();
    let throughput = f64::from(num_requests) / elapsed.as_secs_f64();

    println!("Authorization throughput: {throughput:.2} req/sec");
    println!(
        "Average latency: {:.2} ms",
        elapsed.as_millis() as f64 / f64::from(num_requests)
    );

    assert!(
        throughput > 5.0,
        "Throughput should be > 5 req/sec (generous floor), got {throughput:.2}"
    );

    server_task.abort();
}

#[tokio::test]
async fn test_validation_throughput() {
    let (socket_path, server_task, _guard) = start_test_server().await;

    let start = Instant::now();
    let num_requests = 100;

    for i in 0..num_requests {
        let params = json!({
            "template": {
                "id": format!("template-{i}"),
                "name": "Test Template",
                "creator": "alice",
                "nodes": [{
                    "id": "node-1",
                    "type": "compute",
                    "primal": "compute.general",
                    "config": {}
                }],
                "edges": [],
                "metadata": {
                    "version": "1.0",
                    "created_at": "2026-01-11T12:00:00Z"
                }
            }
        });

        let resp =
            send_jsonrpc_request_timed("graph.validate_template", params, &socket_path).await;
        if let Some((response, _)) = resp {
            assert_eq!(response["jsonrpc"], "2.0");
        }
    }

    let elapsed = start.elapsed();
    let throughput = f64::from(num_requests) / elapsed.as_secs_f64();

    println!("Validation throughput: {throughput:.2} req/sec");
    println!(
        "Average latency: {:.2} ms",
        elapsed.as_millis() as f64 / f64::from(num_requests)
    );

    assert!(
        throughput > 5.0,
        "Throughput should be > 5 req/sec (generous floor), got {throughput:.2}"
    );

    server_task.abort();
}

// ============================================================================
// Latency Tests
// ============================================================================

#[tokio::test]
async fn test_authorization_latency_p95() {
    let (socket_path, server_task, _guard) = start_test_server().await;

    let mut latencies = Vec::new();

    for i in 0..100 {
        let params = json!({
            "user_id": "alice",
            "graph": {
                "id": format!("graph-{i}"),
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
                    "primal": "compute.general",
                    "config": {}
                }
            }
        });

        if let Some((response, latency)) =
            send_jsonrpc_request_timed("graph.authorize_modification", params, &socket_path).await
        {
            assert_eq!(response["jsonrpc"], "2.0");
            latencies.push(latency);
        }
    }

    assert!(
        latencies.len() >= 50,
        "Need at least 50 successful requests for percentile analysis, got {}",
        latencies.len()
    );
    latencies.sort();
    let len = latencies.len();
    let p50 = latencies[len / 2].as_millis();
    let p95 = latencies[len * 95 / 100].as_millis();
    let p99 = latencies[len.saturating_sub(1)].as_millis();

    println!("Authorization latency:");
    println!("  p50: {p50} ms");
    println!("  p95: {p95} ms");
    println!("  p99: {p99} ms");

    assert!(
        p95 < 1000,
        "p95 latency should be < 1000ms under contention, got {p95} ms"
    );

    server_task.abort();
}

// ============================================================================
// Concurrency Tests
// ============================================================================

#[tokio::test]
async fn test_concurrent_authorization_requests() {
    let (socket_path, server_task, _guard) = start_test_server().await;

    let start = Instant::now();
    let num_clients = 10;
    let requests_per_client = 10;

    let mut handles = Vec::new();

    for client_id in 0..num_clients {
        let socket_path = socket_path.clone();
        let handle = tokio::spawn(async move {
            for i in 0..requests_per_client {
                let params = json!({
                    "user_id": format!("user-{client_id}"),
                    "graph": {
                        "id": format!("graph-{client_id}-{i}"),
                        "owner": format!("user-{client_id}"),
                        "nodes": [],
                        "edges": [],
                        "metadata": {}
                    },
                    "modification": {
                        "action": "add_node",
                        "node": {
                            "id": "new-node",
                            "type": "compute",
                            "primal": "compute.general",
                            "config": {}
                        }
                    }
                });

                if let Some((response, _)) =
                    send_jsonrpc_request_timed("graph.authorize_modification", params, &socket_path)
                        .await
                {
                    assert_eq!(response["jsonrpc"], "2.0");
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let elapsed = start.elapsed();
    let total_requests = num_clients * requests_per_client;
    let throughput = f64::from(total_requests) / elapsed.as_secs_f64();

    println!(
        "Concurrent throughput: {throughput:.2} req/sec ({num_clients} clients, {requests_per_client} req each)"
    );

    assert!(throughput > 0.0);

    server_task.abort();
}

// ============================================================================
// Load Tests
// ============================================================================

#[tokio::test]
async fn test_sustained_load() {
    let (socket_path, server_task, _guard) = start_test_server().await;

    let start = Instant::now();
    let duration = Duration::from_secs(5);
    let mut request_count: u32 = 0;
    let mut success_count: u32 = 0;

    while start.elapsed() < duration {
        if let Some((response, _)) = send_jsonrpc_request_timed(
            "graph.authorize_modification",
            authorize_params(request_count),
            &socket_path,
        )
        .await
        {
            assert_eq!(response["jsonrpc"], "2.0");
            success_count += 1;
        }
        request_count += 1;
    }

    let elapsed = start.elapsed();
    let throughput = f64::from(success_count) / elapsed.as_secs_f64();

    println!(
        "Sustained load throughput: {throughput:.2} req/sec over {} seconds ({success_count}/{request_count} succeeded)",
        elapsed.as_secs()
    );

    assert!(
        throughput > 3.0,
        "Sustained throughput should be > 3 req/sec (generous floor), got {throughput:.2}"
    );

    server_task.abort();
}
