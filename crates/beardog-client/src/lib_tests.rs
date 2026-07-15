// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use beardog_errors::process_env;
use beardog_genetics::birdsong::LineageProof;
use chrono::Utc;
use serde_json::{Value, json};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixListener;
use tokio::sync::{Mutex, Notify};

/// Tests in this module share the global `process_env` overlay (`BEARDOG_SOCKET`).
/// Async mutex so we can `.await` while holding the lock (no `await_holding_lock` on std mutex).
static ENV_LOCK: Mutex<()> = Mutex::const_new(());

fn lineage_proof_sample() -> LineageProof {
    LineageProof {
        node_id: "n1".to_string(),
        root_id: "r0".to_string(),
        path: vec![],
        proof_chain: vec![],
        merkle_root: vec![0xab; 8],
        generation: 0,
        head_commitment: vec![],
        generated_at: Utc::now(),
    }
}

/// RAII guard that removes env vars on drop (even on panic).
struct EnvGuard(Vec<&'static str>);
impl Drop for EnvGuard {
    fn drop(&mut self) {
        for key in &self.0 {
            process_env::remove_var(key);
        }
    }
}

#[tokio::test]
async fn connect_maps_missing_socket_to_connection_error() {
    let _env = ENV_LOCK.lock().await;
    let tmp = tempfile::tempdir().expect("tempdir for BearDog client test");
    let sock = tmp.path().join("beardog-missing.sock");
    process_env::set_var("BEARDOG_SOCKET", sock.to_string_lossy().as_ref());
    let _guard = EnvGuard(vec!["BEARDOG_SOCKET"]);

    let result = BearDogClient::connect().await;
    let Err(err) = result else {
        panic!("expected connection failure");
    };
    let msg = err.to_string();
    assert!(
        msg.contains("Primal not found") || msg.contains("Connection failed"),
        "unexpected error: {msg}"
    );
}

#[tokio::test]
async fn lineage_methods_round_trip_over_json_rpc() {
    let _env = ENV_LOCK.lock().await;
    let tmp = tempfile::tempdir().expect("tempdir for BearDog client test");
    let socket_path = tmp.path().join("beardog-round-trip.sock");

    let listener =
        UnixListener::bind(&socket_path).expect("bind test Unix socket for mock server");
    let ready = Arc::new(Notify::new());
    let ready_tx = Arc::clone(&ready);

    tokio::spawn(async move {
        ready_tx.notify_one();
        let (mut stream, _) = listener
            .accept()
            .await
            .expect("mock server accept lineage round-trip client");
        let (mut read_half, mut write_half) = stream.split();
        let mut reader = BufReader::new(&mut read_half);
        loop {
            let mut line = String::new();
            let n = reader
                .read_line(&mut line)
                .await
                .expect("mock server read JSON-RPC line");
            if n == 0 {
                break;
            }
            let req: Value =
                serde_json::from_str(line.trim()).expect("mock server parse JSON-RPC request");
            let id = req["id"].clone();
            let method = req["method"]
                .as_str()
                .expect("JSON-RPC request must have string method");
            let result = match method {
                "lineage.create" => json!({ "lineage_id": "L-genesis" }),
                "lineage.verify" => json!({ "valid": true }),
                "lineage.extend" => json!({ "node_id": "child-1" }),
                "lineage.get" => json!({ "lineage_id": "L1", "nodes": [] }),
                _ => panic!("unexpected method {method}"),
            };
            let response = json!({
                "jsonrpc": "2.0",
                "result": result,
                "id": id,
            });
            let payload =
                serde_json::to_string(&response).expect("serialize JSON-RPC response");
            write_half
                .write_all(payload.as_bytes())
                .await
                .expect("write JSON-RPC response body");
            write_half
                .write_all(b"\n")
                .await
                .expect("write JSON-RPC newline");
        }
    });

    ready.notified().await;
    process_env::set_var("BEARDOG_SOCKET", socket_path.to_string_lossy().as_ref());
    let _guard = EnvGuard(vec!["BEARDOG_SOCKET"]);

    let mut client = BearDogClient::connect()
        .await
        .expect("connect to mock round-trip socket");
    let created = client
        .create_lineage("tower", None)
        .await
        .expect("create_lineage round-trip");
    assert_eq!(created["lineage_id"], "L-genesis");

    let verified = client
        .verify_lineage(&lineage_proof_sample())
        .await
        .expect("verify_lineage round-trip");
    assert_eq!(verified["valid"], true);

    let extended = client
        .extend_lineage("L1", "parent", None)
        .await
        .expect("extend_lineage round-trip");
    assert_eq!(extended["node_id"], "child-1");

    let got = client
        .get_lineage("L1")
        .await
        .expect("get_lineage round-trip");
    assert_eq!(got["lineage_id"], "L1");
}

#[tokio::test]
async fn lineage_create_and_extend_with_metadata_serializes_in_params() {
    let _env = ENV_LOCK.lock().await;
    let tmp = tempfile::tempdir().expect("tempdir for BearDog client test");
    let socket_path = tmp.path().join("beardog-meta.sock");

    let listener =
        UnixListener::bind(&socket_path).expect("bind test Unix socket for mock server");
    let ready = Arc::new(Notify::new());
    let ready_tx = Arc::clone(&ready);

    tokio::spawn(async move {
        ready_tx.notify_one();
        let (mut stream, _) = listener
            .accept()
            .await
            .expect("mock server accept metadata test client");
        let (mut read_half, mut write_half) = stream.split();
        let mut reader = BufReader::new(&mut read_half);
        for _ in 0..2 {
            let mut line = String::new();
            reader
                .read_line(&mut line)
                .await
                .expect("mock server read metadata JSON-RPC line");
            let req: Value = serde_json::from_str(line.trim())
                .expect("mock server parse metadata JSON-RPC request");
            let id = req["id"].clone();
            let method = req["method"]
                .as_str()
                .expect("JSON-RPC request must have string method");
            let params = &req["params"];
            let result = match method {
                "lineage.create" => {
                    assert_eq!(params["service_type"], "tower");
                    assert!(
                        !params["metadata"].is_null(),
                        "metadata should be present in JSON-RPC params"
                    );
                    assert_eq!(params["metadata"]["biome_type"], "test-biome");
                    json!({ "lineage_id": "L-with-meta" })
                }
                "lineage.extend" => {
                    assert_eq!(params["lineage_id"], "L-with-meta");
                    assert_eq!(params["parent_id"], "root");
                    assert_eq!(params["metadata"]["trust_level"], 0.5);
                    json!({ "node_id": "child-meta" })
                }
                _ => panic!("unexpected method {method}"),
            };
            let response = json!({
                "jsonrpc": "2.0",
                "result": result,
                "id": id,
            });
            let payload =
                serde_json::to_string(&response).expect("serialize metadata JSON-RPC response");
            write_half
                .write_all(payload.as_bytes())
                .await
                .expect("write metadata JSON-RPC response");
            write_half
                .write_all(b"\n")
                .await
                .expect("write metadata JSON-RPC newline");
        }
    });

    ready.notified().await;
    process_env::set_var("BEARDOG_SOCKET", socket_path.to_string_lossy().as_ref());
    let _guard = EnvGuard(vec!["BEARDOG_SOCKET"]);

    let mut meta = LineageMetadata::default();
    meta.biome_type = Some("test-biome".to_string());
    meta.trust_level = 0.5;

    let mut client = BearDogClient::connect()
        .await
        .expect("connect to mock metadata socket");
    let created = client
        .create_lineage("tower", Some(meta.clone()))
        .await
        .expect("create_lineage with metadata");
    assert_eq!(created["lineage_id"], "L-with-meta");

    let extended = client
        .extend_lineage("L-with-meta", "root", Some(meta))
        .await
        .expect("extend_lineage with metadata");
    assert_eq!(extended["node_id"], "child-meta");
}

#[tokio::test]
async fn api_error_maps_json_rpc_fault() {
    let _env = ENV_LOCK.lock().await;
    let tmp = tempfile::tempdir().expect("tempdir for BearDog client test");
    let socket_path = tmp.path().join("beardog-err.sock");

    let listener =
        UnixListener::bind(&socket_path).expect("bind test Unix socket for mock server");
    let ready = Arc::new(Notify::new());
    let ready_tx = Arc::clone(&ready);

    tokio::spawn(async move {
        ready_tx.notify_one();
        let (mut stream, _) = listener
            .accept()
            .await
            .expect("mock server accept fault test client");
        let (mut read_half, mut write_half) = stream.split();
        let mut reader = BufReader::new(&mut read_half);
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .await
            .expect("mock server read fault-test request");
        let response = json!({
            "jsonrpc": "2.0",
            "error": { "code": -32601, "message": "method not found" },
            "id": 1
        });
        let err_body = serde_json::to_string(&response).expect("serialize error response");
        write_half
            .write_all(err_body.as_bytes())
            .await
            .expect("write JSON-RPC error response");
        write_half
            .write_all(b"\n")
            .await
            .expect("write JSON-RPC error newline");
    });

    ready.notified().await;
    process_env::set_var("BEARDOG_SOCKET", socket_path.to_string_lossy().as_ref());
    let _guard = EnvGuard(vec!["BEARDOG_SOCKET"]);

    let mut client = BearDogClient::connect()
        .await
        .expect("connect to mock fault socket");
    let result = client.create_lineage("x", None).await;
    let Err(err) = result else {
        panic!("expected API error");
    };
    let msg = err.to_string();
    assert!(
        msg.contains("method not found") || msg.contains("Api error"),
        "unexpected: {msg}"
    );
}
