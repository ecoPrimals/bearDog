// SPDX-License-Identifier: AGPL-3.0-or-later

//! Targeted coverage tests for low-coverage tunnel modules:
//! `aliases_and_beardog` router, TCP client, and modes/client.

use serde_json::json;

// -- aliases_and_beardog::route --

mod alias_router {
    use super::*;
    use crate::unix_socket_ipc::handlers::crypto_handler::aliases_and_beardog::route;

    #[tokio::test]
    async fn crypto_hash_alias() {
        let params = json!({ "data": "aGVsbG8=" });
        let result = route("crypto.hash", Some(&params)).await;
        assert!(
            result.is_ok(),
            "crypto.hash should route to blake3: {result:?}"
        );
        assert!(result.as_ref().expect("ok").is_some());
    }

    #[tokio::test]
    async fn crypto_generate_keypair_alias() {
        let result = route("crypto.generate_keypair", None).await;
        assert!(result.is_ok(), "crypto.generate_keypair: {result:?}");
        assert!(result.as_ref().expect("ok").is_some());
    }

    #[tokio::test]
    async fn beardog_crypto_ed25519_generate_keypair() {
        let result = route("beardog.crypto.ed25519_generate_keypair", None).await;
        assert!(
            result.is_ok(),
            "beardog.crypto.ed25519_generate_keypair: {result:?}"
        );
        assert!(result.as_ref().expect("ok").is_some());
    }

    #[tokio::test]
    async fn beardog_crypto_blake3_hash() {
        let params = json!({ "data": "dGVzdA==" });
        let result = route("beardog.crypto.blake3_hash", Some(&params)).await;
        assert!(result.is_ok(), "beardog.crypto.blake3_hash: {result:?}");
        assert!(result.as_ref().expect("ok").is_some());
    }

    #[tokio::test]
    async fn beardog_crypto_generate_onion_identity() {
        let result = route("beardog.crypto.generate_onion_identity", None).await;
        assert!(
            result.is_ok(),
            "beardog.crypto.generate_onion_identity: {result:?}"
        );
        assert!(result.as_ref().expect("ok").is_some());
    }

    #[tokio::test]
    async fn unknown_method_returns_none() {
        let result = route("nonexistent.method.xyz", None).await;
        assert!(result.is_ok());
        assert!(result.expect("ok").is_none());
    }
}

// -- TCP client with local echo server --

mod tcp_client_test {
    use crate::tcp_ipc::client::TcpIpcClient;
    use serde_json::json;
    use std::net::SocketAddr;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::TcpListener;

    async fn mock_jsonrpc_server(listener: TcpListener) {
        let (stream, _) = listener.accept().await.expect("accept");
        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);

        // Consume riboCipher signal prefix
        let mut sig = [0u8; 2];
        tokio::io::AsyncReadExt::read_exact(&mut reader.get_mut(), &mut sig)
            .await
            .expect("read riboCipher signal");

        let mut line = String::new();
        reader.read_line(&mut line).await.expect("read request");

        let request: serde_json::Value = serde_json::from_str(&line).expect("parse client request");
        let id = request.get("id").cloned().unwrap_or(json!(1));

        let response = json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": { "status": "ok" }
        });
        let mut resp_bytes = serde_json::to_vec(&response).expect("serialize response");
        resp_bytes.push(b'\n');
        writer.write_all(&resp_bytes).await.expect("write response");
    }

    #[tokio::test]
    async fn tcp_client_call_happy_path() {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind ephemeral");
        let addr: SocketAddr = listener.local_addr().expect("local_addr");

        tokio::spawn(mock_jsonrpc_server(listener));

        let client = TcpIpcClient::new(addr);
        let result = client.call("test.ping", None).await.expect("call");
        assert_eq!(result["status"], "ok");
    }

    #[tokio::test]
    async fn tcp_client_connection_refused() {
        let addr: SocketAddr = "127.0.0.1:1".parse().expect("parse addr");
        let client = TcpIpcClient::new(addr);
        let err = client
            .call("test.ping", None)
            .await
            .expect_err("should fail to connect");
        let msg = format!("{err}");
        assert!(
            msg.contains("connect") || msg.contains("Connection refused"),
            "unexpected error: {msg}"
        );
    }

    async fn mock_error_server(listener: TcpListener) {
        let (stream, _) = listener.accept().await.expect("accept");
        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);

        // Consume riboCipher signal prefix
        let mut sig = [0u8; 2];
        tokio::io::AsyncReadExt::read_exact(&mut reader.get_mut(), &mut sig)
            .await
            .expect("read riboCipher signal");

        let mut line = String::new();
        reader.read_line(&mut line).await.expect("read");

        let response = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "error": { "code": -32601, "message": "Method not found" }
        });
        let mut resp = serde_json::to_vec(&response).expect("serialize");
        resp.push(b'\n');
        writer.write_all(&resp).await.expect("write");
    }

    #[tokio::test]
    async fn tcp_client_receives_rpc_error() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr: SocketAddr = listener.local_addr().expect("addr");

        tokio::spawn(mock_error_server(listener));

        let client = TcpIpcClient::new(addr);
        let err = client
            .call("nonexistent", None)
            .await
            .expect_err("should propagate RPC error");
        let msg = format!("{err}");
        assert!(msg.contains("RPC error") || msg.contains("Method not found"));
    }
}

// -- modes::client::run --

mod modes_client_test {
    use crate::modes::client::run;

    #[tokio::test]
    async fn client_run_with_command_returns_err_when_server_absent() {
        let result = run(
            Some("/tmp/beardog-wave10-cov-absent.sock".to_string()),
            Some("health".to_string()),
        )
        .await;
        assert!(result.is_err(), "should error when server is not running");
    }

    #[tokio::test]
    async fn client_run_interactive_banner() {
        let result = run(
            Some("unix:///tmp/beardog-wave10-cov.sock".to_string()),
            None,
        )
        .await;
        assert!(result.is_ok(), "client run interactive should return Ok");
    }
}
