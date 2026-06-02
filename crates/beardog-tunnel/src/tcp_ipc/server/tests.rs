// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use crate::btsp_handshake::BtspSecurityMode;
use crate::method_gate::EnforcementMode;
use crate::test_helpers::mocks::create_minimal_beardog_provider;
use crate::unix_socket_ipc::handlers::HandlerRegistry;
use beardog_types::primal_identity::PrimalIdentity;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

fn test_gate() -> Arc<MethodGate> {
    Arc::new(MethodGate::new(
        EnforcementMode::Permissive,
        "beardog",
        "test-node",
    ))
}

#[tokio::test]
async fn tcp_ipc_health_ping_roundtrip() {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind TCP listener for health ping test");
    let addr = listener
        .local_addr()
        .expect("local_addr after bind for health ping test");
    let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
    let registry = HandlerRegistry::new(identity);
    let provider = create_minimal_beardog_provider().await;

    let gate = test_gate();
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.expect("accept health ping client");
        TcpIpcServer::handle_connection(
            stream,
            registry,
            provider,
            BtspSecurityMode::Development,
            gate,
        )
        .await
    });

    let mut client = tokio::net::TcpStream::connect(addr)
        .await
        .expect("connect health ping client");
    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "ping",
        "id": 1
    });
    let line = serde_json::to_string(&request).expect("serialize ping request") + "\n";
    client
        .write_all(line.as_bytes())
        .await
        .expect("write ping request");

    let mut reader = BufReader::new(client);
    let mut buf = String::new();
    reader
        .read_line(&mut buf)
        .await
        .expect("read ping response line");
    let resp: serde_json::Value =
        serde_json::from_str(buf.trim()).expect("parse ping JSON-RPC response");
    assert_eq!(resp["jsonrpc"], "2.0");
    assert_eq!(resp["id"], 1);
    assert!(resp.get("result").is_some());

    drop(reader);
    server.abort();
}

#[tokio::test]
async fn tcp_ipc_invalid_json_returns_parse_error() {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind TCP listener for invalid JSON test");
    let addr = listener
        .local_addr()
        .expect("local_addr after bind for invalid JSON test");
    let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
    let registry = HandlerRegistry::new(identity);
    let provider = create_minimal_beardog_provider().await;

    let gate = test_gate();
    let server = tokio::spawn(async move {
        let (stream, _) = listener
            .accept()
            .await
            .expect("accept invalid JSON test client");
        TcpIpcServer::handle_connection(
            stream,
            registry,
            provider,
            BtspSecurityMode::Development,
            gate,
        )
        .await
    });

    let mut client = tokio::net::TcpStream::connect(addr)
        .await
        .expect("connect invalid JSON test client");
    client
        .write_all(b"not json at all\n")
        .await
        .expect("write invalid payload");

    let mut reader = BufReader::new(client);
    let mut buf = String::new();
    reader
        .read_line(&mut buf)
        .await
        .expect("read parse-error response line");
    let resp: serde_json::Value =
        serde_json::from_str(buf.trim()).expect("parse parse-error JSON-RPC response");
    assert_eq!(resp["jsonrpc"], "2.0");
    assert_eq!(resp["error"]["code"], -32700);

    drop(reader);
    server.abort();
}

#[tokio::test]
async fn tcp_ipc_unknown_method_returns_error() {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind TCP listener for unknown method test");
    let addr = listener
        .local_addr()
        .expect("local_addr after bind for unknown method test");
    let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
    let registry = HandlerRegistry::new(identity);
    let provider = create_minimal_beardog_provider().await;

    let gate = test_gate();
    let server = tokio::spawn(async move {
        let (stream, _) = listener
            .accept()
            .await
            .expect("accept unknown method test client");
        TcpIpcServer::handle_connection(
            stream,
            registry,
            provider,
            BtspSecurityMode::Development,
            gate,
        )
        .await
    });

    let mut client = tokio::net::TcpStream::connect(addr)
        .await
        .expect("connect unknown method test client");
    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "definitely.not.a.registered.method",
        "id": 42
    });
    let req_line =
        serde_json::to_string(&request).expect("serialize unknown-method request") + "\n";
    client
        .write_all(req_line.as_bytes())
        .await
        .expect("write unknown-method request");

    let mut reader = BufReader::new(client);
    let mut buf = String::new();
    reader
        .read_line(&mut buf)
        .await
        .expect("read unknown-method error line");
    let resp: serde_json::Value =
        serde_json::from_str(buf.trim()).expect("parse unknown-method JSON-RPC response");
    assert_eq!(resp["jsonrpc"], "2.0");
    assert_eq!(resp["id"], 42);
    assert_eq!(resp["error"]["code"], -32601);

    drop(reader);
    server.abort();
}

#[tokio::test]
async fn tcp_ipc_server_new_bound_addr_starts_none() {
    let identity = Arc::new(PrimalIdentity::for_test("fam", "node"));
    let provider = create_minimal_beardog_provider().await;
    let addr: std::net::SocketAddr = "127.0.0.1:0".parse().expect("parse loopback :0 SocketAddr");
    let server = TcpIpcServer::new(addr, provider, identity, BtspSecurityMode::Development);
    assert!(server.get_bound_addr().await.is_none());
}
