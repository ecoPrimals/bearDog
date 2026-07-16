// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use beardog_core::capabilities::{Capability, IpcEndpoint};
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Notify;

#[test]
fn test_json_rpc_request_serialization() -> Result<(), serde_json::Error> {
    let request = JsonRpcRequest {
        jsonrpc: Cow::Borrowed(JSONRPC_VERSION),
        method: "primal.ping".into(),
        params: None,
        id: serde_json::Value::from(1),
    };

    let json = serde_json::to_string(&request)?;
    assert!(json.contains("primal.ping"));
    assert!(json.contains("\"id\":1"));
    Ok(())
}

#[test]
fn test_json_rpc_response_deserialization() -> Result<(), serde_json::Error> {
    let json = r#"{"jsonrpc":"2.0","result":{"pong":true},"id":1}"#;
    let response: JsonRpcResponse = serde_json::from_str(json)?;

    assert_eq!(response.jsonrpc, "2.0");
    assert_eq!(response.id, 1);
    assert!(response.result.is_some());
    assert!(response.error.is_none());
    Ok(())
}

#[test]
fn test_primal_info_deserialization() -> Result<(), serde_json::Error> {
    let sock = std::env::temp_dir().join("beardog-nat0.sock");
    let json = serde_json::json!({
        "primal_id": "beardog",
        "family_id": "nat0",
        "node_id": "tower1",
        "capabilities": ["encryption", "trust"],
        "socket_path": sock.to_string_lossy(),
    })
    .to_string();

    let info: PrimalInfo = serde_json::from_str(&json)?;
    assert_eq!(info.primal_id, "beardog");
    assert_eq!(info.family_id, Some("nat0".to_string()));
    assert_eq!(info.capabilities.len(), 2);
    Ok(())
}

#[test]
fn test_zero_vendor_hardcoding() {
    let path = std::env::temp_dir().join("any-registry.sock");
    let client = PrimalRegistryClient::new(path.clone());

    assert_eq!(client.socket_path(), path);
}

#[tokio::test]
async fn test_not_connected_errors() {
    let mut client =
        PrimalRegistryClient::new(std::env::temp_dir().join("unused-nonexistent.sock"));
    let caps = beardog_core::capabilities::BearDogCapabilities::new(
        Some("fam".to_string()),
        "node".to_string(),
    );
    let err = client.register(&caps).await.expect_err("not connected");
    assert!(err.to_string().contains("Not connected") || err.to_string().contains("registry"));
}

fn unique_registry_sock() -> PathBuf {
    std::env::temp_dir().join(format!(
        "beardog_registry_test_{}_{}.sock",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ))
}

fn spawn_registry_mock(path: PathBuf, ready: Arc<Notify>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let _ = std::fs::remove_file(&path);
        let listener = tokio::net::UnixListener::bind(&path).expect("bind registry mock");
        ready.notify_one();
        loop {
            let Ok((stream, _)) = listener.accept().await else {
                break;
            };
            tokio::spawn(async move {
                let p1_sock_display = std::env::temp_dir()
                    .join("p1.sock")
                    .to_string_lossy()
                    .into_owned();
                let mut reader = BufReader::new(stream);
                loop {
                    let mut line = String::new();
                    if reader.read_line(&mut line).await.is_err() || line.trim().is_empty() {
                        break;
                    }
                    let Ok(v) = serde_json::from_str::<serde_json::Value>(line.trim()) else {
                        break;
                    };
                    let method = v["method"].as_str().unwrap_or("");
                    let id = v["id"].as_u64().unwrap_or(1);
                    let cap = v["params"]["capability"].as_str().unwrap_or("");
                    let response = match method {
                        "primal.register" => serde_json::json!({
                            "jsonrpc": "2.0",
                            "result": { "ok": true },
                            "id": id
                        }),
                        "primal.ping" => serde_json::json!({
                            "jsonrpc": "2.0",
                            "result": { "pong": true },
                            "id": id
                        }),
                        "primal.get_provider" if cap == "none" => serde_json::json!({
                            "jsonrpc": "2.0",
                            "error": { "code": -1, "message": "missing" },
                            "id": id
                        }),
                        "primal.get_provider" => serde_json::json!({
                            "jsonrpc": "2.0",
                            "result": {
                                "primal_id": "p1",
                                "family_id": null,
                                "node_id": "n1",
                                "capabilities": ["encryption"],
                                "socket_path": p1_sock_display.as_str(),
                            },
                            "id": id
                        }),
                        "primal.list_all" => serde_json::json!({
                            "jsonrpc": "2.0",
                            "result": [{
                                "primal_id": "p1",
                                "family_id": null,
                                "node_id": "n1",
                                "capabilities": [],
                                "socket_path": p1_sock_display.as_str(),
                            }],
                            "id": id
                        }),
                        "primal.unregister" => serde_json::json!({
                            "jsonrpc": "2.0",
                            "result": { "ok": true },
                            "id": id
                        }),
                        _ => serde_json::json!({
                            "jsonrpc": "2.0",
                            "error": { "code": -32601, "message": "unknown" },
                            "id": id
                        }),
                    };
                    let mut stream = reader.into_inner();
                    let line = serde_json::to_string(&response)
                        .expect("serialize mock JSON-RPC response in test");
                    let _ = stream.write_all(format!("{line}\n").as_bytes()).await;
                    reader = BufReader::new(stream);
                }
            });
        }
    })
}

#[tokio::test]
async fn test_connect_ping_list_get_provider_unregister() {
    let path = unique_registry_sock();
    let ready = Arc::new(Notify::new());
    let _srv = spawn_registry_mock(path.clone(), Arc::clone(&ready));
    ready.notified().await;

    let mut client = PrimalRegistryClient::new(path.clone());
    client.connect().await.expect("connect");

    client.ping().await.expect("ping");

    let primals = client.list_all().await.expect("list");
    assert_eq!(primals.len(), 1);
    assert_eq!(primals[0].primal_id, "p1");

    let p = client
        .get_provider("encryption")
        .await
        .expect("get_provider");
    assert_eq!(p.primal_id, "p1");

    let err = client.get_provider("none").await.expect_err("err");
    assert!(err.to_string().contains("missing") || err.to_string().contains("capability"));

    let caps = beardog_core::capabilities::BearDogCapabilities::new(
        Some("fam".to_string()),
        "node".to_string(),
    );
    client.register(&caps).await.expect("register");

    client.unregister("p1").await.expect("unregister");
    let _ = std::fs::remove_file(&path);
}

#[tokio::test]
async fn test_register_error_response() {
    let path = unique_registry_sock();
    let path_clone = path.clone();
    let ready = Arc::new(Notify::new());
    let ready_clone = Arc::clone(&ready);
    let _srv = tokio::spawn(async move {
        let _ = std::fs::remove_file(&path_clone);
        let listener = tokio::net::UnixListener::bind(&path_clone)
            .expect("bind test registry socket for error-branch test");
        ready_clone.notify_one();
        let Ok((mut stream, _)) = listener.accept().await else {
            return;
        };
        let mut reader = BufReader::new(&mut stream);
        let mut line = String::new();
        let _ = reader.read_line(&mut line).await;
        let stream = reader.into_inner();
        let resp = serde_json::json!({
            "jsonrpc": "2.0",
            "error": { "code": -1, "message": "register failed" },
            "id": 1
        });
        let _ = stream.write_all(format!("{resp}\n").as_bytes()).await;
    });
    ready.notified().await;

    let mut client = PrimalRegistryClient::new(path.clone());
    client
        .connect()
        .await
        .expect("connect registry client for register error test");
    let caps = beardog_core::capabilities::BearDogCapabilities::new(None, "n".to_string());
    let err = client.register(&caps).await.expect_err("register err");
    assert!(err.to_string().contains("register") || err.to_string().contains("Registry"));
    let _ = std::fs::remove_file(&path);
}

#[tokio::test]
async fn test_list_all_error_branch() {
    let path = unique_registry_sock();
    let path_clone = path.clone();
    let ready = Arc::new(Notify::new());
    let ready_clone = Arc::clone(&ready);
    let _srv = tokio::spawn(async move {
        let _ = std::fs::remove_file(&path_clone);
        let listener = tokio::net::UnixListener::bind(&path_clone)
            .expect("bind test registry socket for error-branch test");
        ready_clone.notify_one();
        loop {
            let Ok((mut stream, _)) = listener.accept().await else {
                break;
            };
            let mut reader = BufReader::new(&mut stream);
            let mut line = String::new();
            let _ = reader.read_line(&mut line).await;
            let v: serde_json::Value = serde_json::from_str(line.trim()).unwrap_or_default();
            let id = v["id"].as_u64().unwrap_or(1);
            let stream = reader.into_inner();
            let resp = serde_json::json!({
                "jsonrpc": "2.0",
                "error": { "code": -1, "message": "list failed" },
                "id": id
            });
            let _ = stream.write_all(format!("{resp}\n").as_bytes()).await;
        }
    });
    ready.notified().await;

    let mut client = PrimalRegistryClient::new(path.clone());
    client
        .connect()
        .await
        .expect("connect for list_all error branch test");
    let err = client.list_all().await.expect_err("list err");
    assert!(err.to_string().contains("list") || err.to_string().contains("primals"));
    let _ = std::fs::remove_file(&path);
}

#[tokio::test]
async fn test_ping_error_branch() {
    let path = unique_registry_sock();
    let path_clone = path.clone();
    let ready = Arc::new(Notify::new());
    let ready_clone = Arc::clone(&ready);
    let _srv = tokio::spawn(async move {
        let _ = std::fs::remove_file(&path_clone);
        let listener = tokio::net::UnixListener::bind(&path_clone)
            .expect("bind test registry socket for error-branch test");
        ready_clone.notify_one();
        let Ok((mut stream, _)) = listener.accept().await else {
            return;
        };
        let mut reader = BufReader::new(&mut stream);
        let mut line = String::new();
        let _ = reader.read_line(&mut line).await;
        let stream = reader.into_inner();
        let resp = serde_json::json!({
            "jsonrpc": "2.0",
            "error": { "code": -1, "message": "bad" },
            "id": 1
        });
        let _ = stream.write_all(format!("{resp}\n").as_bytes()).await;
    });
    ready.notified().await;

    let mut client = PrimalRegistryClient::new(path.clone());
    client
        .connect()
        .await
        .expect("connect for ping error branch test");
    let err = client.ping().await.expect_err("ping err");
    assert!(err.to_string().contains("Ping") || err.to_string().contains("ping"));
    let _ = std::fs::remove_file(&path);
}

#[tokio::test]
async fn test_unregister_error_branch() {
    let path = unique_registry_sock();
    let path_clone = path.clone();
    let ready = Arc::new(Notify::new());
    let ready_clone = Arc::clone(&ready);
    let _srv = tokio::spawn(async move {
        let _ = std::fs::remove_file(&path_clone);
        let listener = tokio::net::UnixListener::bind(&path_clone)
            .expect("bind test registry socket for error-branch test");
        ready_clone.notify_one();
        let Ok((mut stream, _)) = listener.accept().await else {
            return;
        };
        let mut reader = BufReader::new(&mut stream);
        let mut line = String::new();
        let _ = reader.read_line(&mut line).await;
        let stream = reader.into_inner();
        let resp = serde_json::json!({
            "jsonrpc": "2.0",
            "error": { "code": -1, "message": "nope" },
            "id": 1
        });
        let _ = stream.write_all(format!("{resp}\n").as_bytes()).await;
    });
    ready.notified().await;

    let mut client = PrimalRegistryClient::new(path.clone());
    client
        .connect()
        .await
        .expect("connect for unregister error branch test");
    let err = client.unregister("x").await.expect_err("unreg err");
    assert!(err.to_string().contains("unregister") || err.to_string().contains("Unregister"));
    let _ = std::fs::remove_file(&path);
}

#[tokio::test]
async fn test_register_uses_default_socket_path_when_no_unix_endpoint() {
    let path = unique_registry_sock();
    let ready = Arc::new(Notify::new());
    let _srv = spawn_registry_mock(path.clone(), Arc::clone(&ready));
    ready.notified().await;

    let mut client = PrimalRegistryClient::new(path.clone());
    client
        .connect()
        .await
        .expect("connect for default socket path registration test");

    let caps = beardog_core::capabilities::BearDogCapabilities {
        primal_id: "beardog".to_string(),
        family_id: None,
        node_id: "n".to_string(),
        provides: vec![Capability::Custom {
            name: "custom_cap".to_string(),
            version: "1".to_string(),
            properties: BTreeMap::new(),
        }],
        requires: vec![],
        endpoints: vec![IpcEndpoint::Http {
            bind_addr: "127.0.0.1:9".to_string(),
            tls: false,
        }],
        metadata: BTreeMap::new(),
    };
    client
        .register(&caps)
        .await
        .expect("register with http-only endpoint");
    let _ = std::fs::remove_file(&path);
}
