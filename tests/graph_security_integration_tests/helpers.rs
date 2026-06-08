// SPDX-License-Identifier: AGPL-3.0-or-later

//! Shared test helpers for graph security integration tests.

use beardog_tunnel::btsp_provider::BeardogBtspProvider;
use serde_json::json;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

/// Generates a unique Unix socket path for test isolation.
pub fn unique_unix_socket() -> PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_micros();

    std::env::temp_dir().join(format!("beardog-test-{pid}-{timestamp}-{id}.sock"))
}

pub async fn create_test_btsp_provider() -> Arc<BeardogBtspProvider> {
    use beardog_genetics::EcosystemGeneticEngine;
    use beardog_tunnel::tunnel::hsm::manager::HsmManager;

    let hsm = Arc::new(HsmManager::auto_initialize().await.expect("HSM init"));
    let genetics = Arc::new(EcosystemGeneticEngine::new().expect("Genetics init"));

    Arc::new(
        BeardogBtspProvider::new(hsm, genetics)
            .await
            .expect("BTSP provider init"),
    )
}

pub async fn send_jsonrpc_request(
    method: &str,
    params: serde_json::Value,
    socket_path: &str,
) -> serde_json::Value {
    let mut stream = UnixStream::connect(socket_path)
        .await
        .unwrap_or_else(|e| panic!("Failed to connect to {socket_path}: {e}"));

    let request = json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1,
    });

    let request_str = serde_json::to_string(&request).unwrap();
    eprintln!("Sending request: {request_str}");
    stream.write_all(request_str.as_bytes()).await.unwrap();
    stream.write_all(b"\n").await.unwrap();
    stream.flush().await.unwrap();

    let mut reader = BufReader::new(&mut stream);
    let mut response_line = String::new();
    reader.read_line(&mut response_line).await.unwrap();

    serde_json::from_str(&response_line).unwrap()
}

/// RAII guard that removes the socket file on drop, preventing stale socket leaks.
pub struct SocketGuard {
    pub path: std::path::PathBuf,
}

impl Drop for SocketGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

/// Spin up a test server on an isolated socket.
pub async fn start_test_server() -> (String, tokio::task::JoinHandle<()>, SocketGuard) {
    use beardog_core::socket_config::IpcCapabilitySymlinksConfig;
    use beardog_tunnel::btsp_handshake::BtspSecurityMode;
    use beardog_types::primal_identity::PrimalIdentity;

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
        std::time::Duration::from_secs(10),
    )
    .await;
    assert!(ready, "Server must become ready within 10s");

    let guard = SocketGuard { path: socket_path };

    (path_str, server_task, guard)
}
