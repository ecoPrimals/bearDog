// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use beardog_core::capabilities::{
    Capability, CapabilityRequest, CapabilityResponse, ResponseStatus,
};

struct TestHandler;

#[async_trait::async_trait]
impl IpcHandler for TestHandler {
    async fn handle_capability_request(
        &self,
        request: CapabilityRequest,
    ) -> Result<CapabilityResponse, BearDogError> {
        Ok(CapabilityResponse {
            request_id: request.request_id,
            status: ResponseStatus::Success,
            data: Some(serde_json::json!({"message": "test"})),
            error: None,
        })
    }

    async fn handle_register(
        &self,
        _primal_id: String,
        _capabilities: Vec<String>,
    ) -> Result<(), BearDogError> {
        Ok(())
    }

    async fn handle_event(
        &self,
        _event_type: String,
        _data: serde_json::Value,
    ) -> Result<(), BearDogError> {
        Ok(())
    }
}

#[test]
fn ipc_message_register_roundtrips_json() -> Result<(), BearDogError> {
    let msg = IpcMessage::Register {
        primal_id: "p-a".to_string(),
        capabilities: vec!["crypto.sign".to_string()],
    };
    let json = serde_json::to_string(&msg)
        .map_err(|e| BearDogError::serialization(&format!("Register serde: {e}")))?;
    let back: IpcMessage = serde_json::from_str(&json)
        .map_err(|e| BearDogError::serialization(&format!("Register de: {e}")))?;
    match back {
        IpcMessage::Register {
            primal_id,
            capabilities,
        } => {
            assert_eq!(primal_id, "p-a");
            assert_eq!(capabilities, vec!["crypto.sign".to_string()]);
        }
        other => {
            return Err(BearDogError::invalid_input(&format!(
                "expected Register, got {other:?}"
            )));
        }
    }
    Ok(())
}

#[test]
fn ipc_message_event_roundtrips_json() -> Result<(), BearDogError> {
    let msg = IpcMessage::Event {
        event_type: "audit".to_string(),
        data: serde_json::json!({"k": 1}),
    };
    let json = serde_json::to_string(&msg)
        .map_err(|e| BearDogError::serialization(&format!("Event serde: {e}")))?;
    let back: IpcMessage = serde_json::from_str(&json)
        .map_err(|e| BearDogError::serialization(&format!("Event de: {e}")))?;
    match back {
        IpcMessage::Event { event_type, data } => {
            assert_eq!(event_type, "audit");
            assert_eq!(data["k"], 1);
        }
        other => {
            return Err(BearDogError::invalid_input(&format!(
                "expected Event, got {other:?}"
            )));
        }
    }
    Ok(())
}

#[test]
fn ipc_message_pong_roundtrips_json() -> Result<(), BearDogError> {
    let msg = IpcMessage::Pong {
        to: "peer-1".to_string(),
    };
    let json = serde_json::to_string(&msg)
        .map_err(|e| BearDogError::serialization(&format!("Pong serde: {e}")))?;
    let back: IpcMessage = serde_json::from_str(&json)
        .map_err(|e| BearDogError::serialization(&format!("Pong de: {e}")))?;
    match back {
        IpcMessage::Pong { to } => assert_eq!(to, "peer-1"),
        other => {
            return Err(BearDogError::invalid_input(&format!(
                "expected Pong, got {other:?}"
            )));
        }
    }
    Ok(())
}

#[test]
fn ipc_server_new_stores_socket_path() {
    use std::path::PathBuf;
    let p = PathBuf::from("/tmp/beardog-ipc-coverage.sock");
    let handler: Arc<dyn IpcHandler> = Arc::new(TestHandler);
    let srv = IpcServer::new(p.clone(), handler);
    assert_eq!(srv.socket_path, p);
}

#[test]
fn ipc_message_capability_response_roundtrips_json() -> Result<(), BearDogError> {
    let resp = CapabilityResponse {
        request_id: "r99".to_string(),
        status: ResponseStatus::Success,
        data: Some(serde_json::json!({"ok": true})),
        error: None,
    };
    let msg = IpcMessage::CapabilityResponse(resp);
    let json = serde_json::to_string(&msg)
        .map_err(|e| BearDogError::serialization(&format!("cap resp serde: {e}")))?;
    let back: IpcMessage = serde_json::from_str(&json)
        .map_err(|e| BearDogError::serialization(&format!("cap resp de: {e}")))?;
    match back {
        IpcMessage::CapabilityResponse(r) => {
            assert_eq!(r.request_id, "r99");
            assert!(matches!(r.status, ResponseStatus::Success));
        }
        other => {
            return Err(BearDogError::invalid_input(&format!(
                "expected CapabilityResponse, got {other:?}"
            )));
        }
    }
    Ok(())
}

#[test]
fn test_ipc_message_serialization() -> Result<(), BearDogError> {
    let msg = IpcMessage::Ping {
        from: "test".to_string(),
    };

    let json = serde_json::to_string(&msg).map_err(|e| {
        BearDogError::serialization(&format!("IpcMessage Ping serializes to JSON: {e}"))
    })?;
    assert!(json.contains("ping"));

    let deserialized: IpcMessage = serde_json::from_str(&json).map_err(|e| {
        BearDogError::serialization(&format!("IpcMessage Ping roundtrips from JSON: {e}"))
    })?;
    let IpcMessage::Ping { from } = deserialized else {
        return Err(BearDogError::invalid_input(&format!(
            "Wrong message type: expected Ping, got {:?}",
            deserialized
        )));
    };
    assert_eq!(from, "test");
    Ok(())
}

#[test]
fn test_capability_request_serialization() -> Result<(), BearDogError> {
    let req = CapabilityRequest {
        from_primal: "test_primal".to_string(),
        capability: Capability::Encryption {
            algorithms: vec!["ChaCha20".to_string()],
            key_types: vec!["X25519".to_string()],
        },
        params: std::collections::HashMap::new(),
        request_id: "req_123".to_string(),
    };

    let msg = IpcMessage::CapabilityRequest(req);
    let json = serde_json::to_string(&msg).map_err(|e| {
        BearDogError::serialization(&format!("IpcMessage CapabilityRequest serializes: {e}"))
    })?;

    let deserialized: IpcMessage = serde_json::from_str(&json).map_err(|e| {
        BearDogError::serialization(&format!("IpcMessage CapabilityRequest roundtrips: {e}"))
    })?;
    let IpcMessage::CapabilityRequest(req) = deserialized else {
        return Err(BearDogError::invalid_input(&format!(
            "Wrong message type: expected CapabilityRequest, got {:?}",
            deserialized
        )));
    };
    assert_eq!(req.from_primal, "test_primal");
    assert_eq!(req.request_id, "req_123");
    Ok(())
}

#[tokio::test]
async fn handle_message_capability_request_ok() -> Result<(), BearDogError> {
    let handler: Arc<dyn IpcHandler> = Arc::new(TestHandler);
    let connections = Arc::new(RwLock::new(Vec::new()));
    let req = CapabilityRequest {
        from_primal: "a".to_string(),
        capability: Capability::Encryption {
            algorithms: vec!["aes".to_string()],
            key_types: vec!["x25519".to_string()],
        },
        params: std::collections::HashMap::new(),
        request_id: "r1".to_string(),
    };
    let out =
        IpcServer::handle_message(IpcMessage::CapabilityRequest(req), &handler, &connections).await;
    let Some(response_msg) = out else {
        return Err(BearDogError::invalid_input(
            "handle_message returned None for CapabilityRequest",
        ));
    };
    match response_msg {
        IpcMessage::CapabilityResponse(resp) => {
            assert_eq!(resp.request_id, "r1");
            assert!(matches!(resp.status, ResponseStatus::Success));
        }
        other => {
            return Err(BearDogError::invalid_input(&format!(
                "expected capability response, got {:?}",
                other
            )));
        }
    }
    Ok(())
}

#[tokio::test]
async fn handle_message_register_ok_returns_pong() {
    let handler: Arc<dyn IpcHandler> = Arc::new(TestHandler);
    let connections = Arc::new(RwLock::new(Vec::new()));
    let out = IpcServer::handle_message(
        IpcMessage::Register {
            primal_id: "p1".to_string(),
            capabilities: vec!["c".to_string()],
        },
        &handler,
        &connections,
    )
    .await;
    assert!(matches!(out, Some(IpcMessage::Pong { ref to }) if to == "p1"));
    assert_eq!(*connections.read().await, vec!["p1".to_string()]);
}

struct FailingRegisterHandler;

#[async_trait::async_trait]
impl IpcHandler for FailingRegisterHandler {
    async fn handle_capability_request(
        &self,
        _request: CapabilityRequest,
    ) -> Result<CapabilityResponse, BearDogError> {
        unreachable!()
    }

    async fn handle_register(
        &self,
        _primal_id: String,
        _capabilities: Vec<String>,
    ) -> Result<(), BearDogError> {
        Err(BearDogError::business("register failed".to_string()))
    }

    async fn handle_event(
        &self,
        _event_type: String,
        _data: serde_json::Value,
    ) -> Result<(), BearDogError> {
        Ok(())
    }
}

#[tokio::test]
async fn handle_message_register_err_returns_none() {
    let handler: Arc<dyn IpcHandler> = Arc::new(FailingRegisterHandler);
    let connections = Arc::new(RwLock::new(Vec::new()));
    let out = IpcServer::handle_message(
        IpcMessage::Register {
            primal_id: "p1".to_string(),
            capabilities: vec![],
        },
        &handler,
        &connections,
    )
    .await;
    assert!(out.is_none());
    assert!(connections.read().await.is_empty());
}

#[tokio::test]
async fn handle_message_ping_returns_pong() {
    let handler: Arc<dyn IpcHandler> = Arc::new(TestHandler);
    let connections = Arc::new(RwLock::new(Vec::new()));
    let out = IpcServer::handle_message(
        IpcMessage::Ping {
            from: "alice".to_string(),
        },
        &handler,
        &connections,
    )
    .await;
    assert!(matches!(out, Some(IpcMessage::Pong { ref to }) if to == "alice"));
}

#[tokio::test]
async fn handle_message_event_returns_none() {
    let handler: Arc<dyn IpcHandler> = Arc::new(TestHandler);
    let connections = Arc::new(RwLock::new(Vec::new()));
    let out = IpcServer::handle_message(
        IpcMessage::Event {
            event_type: "e".to_string(),
            data: serde_json::json!({}),
        },
        &handler,
        &connections,
    )
    .await;
    assert!(out.is_none());
}

#[tokio::test]
async fn handle_message_response_variants_yield_none() {
    let handler: Arc<dyn IpcHandler> = Arc::new(TestHandler);
    let connections = Arc::new(RwLock::new(Vec::new()));
    assert!(
        IpcServer::handle_message(
            IpcMessage::Pong {
                to: "x".to_string()
            },
            &handler,
            &connections,
        )
        .await
        .is_none()
    );
    let resp = CapabilityResponse {
        request_id: "r".to_string(),
        status: ResponseStatus::Success,
        data: None,
        error: None,
    };
    assert!(
        IpcServer::handle_message(IpcMessage::CapabilityResponse(resp), &handler, &connections,)
            .await
            .is_none()
    );
}

struct FailingEventHandler;

#[async_trait::async_trait]
impl IpcHandler for FailingEventHandler {
    async fn handle_capability_request(
        &self,
        _request: CapabilityRequest,
    ) -> Result<CapabilityResponse, BearDogError> {
        unreachable!()
    }

    async fn handle_register(
        &self,
        _primal_id: String,
        _capabilities: Vec<String>,
    ) -> Result<(), BearDogError> {
        Ok(())
    }

    async fn handle_event(
        &self,
        _event_type: String,
        _data: serde_json::Value,
    ) -> Result<(), BearDogError> {
        Err(BearDogError::business("event failed".to_string()))
    }
}

#[tokio::test]
async fn handle_message_event_error_returns_none() {
    let handler: Arc<dyn IpcHandler> = Arc::new(FailingEventHandler);
    let connections = Arc::new(RwLock::new(Vec::new()));
    let out = IpcServer::handle_message(
        IpcMessage::Event {
            event_type: "bad".to_string(),
            data: serde_json::json!({}),
        },
        &handler,
        &connections,
    )
    .await;
    assert!(out.is_none());
}

struct FailingCapabilityHandler;

#[async_trait::async_trait]
impl IpcHandler for FailingCapabilityHandler {
    async fn handle_capability_request(
        &self,
        _request: CapabilityRequest,
    ) -> Result<CapabilityResponse, BearDogError> {
        Err(BearDogError::business("capability failed".to_string()))
    }

    async fn handle_register(
        &self,
        _primal_id: String,
        _capabilities: Vec<String>,
    ) -> Result<(), BearDogError> {
        Ok(())
    }

    async fn handle_event(
        &self,
        _event_type: String,
        _data: serde_json::Value,
    ) -> Result<(), BearDogError> {
        Ok(())
    }
}

#[tokio::test]
async fn handle_message_capability_request_err_returns_none() {
    let handler: Arc<dyn IpcHandler> = Arc::new(FailingCapabilityHandler);
    let connections = Arc::new(RwLock::new(Vec::new()));
    let req = CapabilityRequest {
        from_primal: "a".to_string(),
        capability: Capability::Encryption {
            algorithms: vec!["aes".to_string()],
            key_types: vec!["x25519".to_string()],
        },
        params: std::collections::HashMap::new(),
        request_id: "fail-cap".to_string(),
    };
    let out =
        IpcServer::handle_message(IpcMessage::CapabilityRequest(req), &handler, &connections).await;
    assert!(out.is_none());
}

#[cfg(unix)]
#[tokio::test]
async fn handle_connection_ping_roundtrip_returns_pong_line() -> Result<(), BearDogError> {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    let (mut local, remote) = tokio::net::UnixStream::pair()
        .map_err(|e| BearDogError::system(format!("unix pair: {e}")))?;
    let handler: Arc<dyn IpcHandler> = Arc::new(TestHandler);
    let connections = Arc::new(RwLock::new(Vec::new()));
    let h = Arc::clone(&handler);
    let c = Arc::clone(&connections);
    let serve = tokio::spawn(async move {
        IpcServer::handle_connection(remote, h, c)
            .await
            .expect("handle")
    });

    let ping = serde_json::to_string(&IpcMessage::Ping {
        from: "unit-test-peer".to_string(),
    })
    .map_err(|e| BearDogError::serialization(&format!("serialize ping: {e}")))?;
    local
        .write_all(ping.as_bytes())
        .await
        .map_err(|e| BearDogError::system(format!("write: {e}")))?;
    local
        .write_all(b"\n")
        .await
        .map_err(|e| BearDogError::system(format!("newline: {e}")))?;

    let mut reader = BufReader::new(&mut local);
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .await
        .map_err(|e| BearDogError::system(format!("read response: {e}")))?;
    let pong: IpcMessage = serde_json::from_str(line.trim())
        .map_err(|e| BearDogError::serialization(&format!("pong json: {e}")))?;
    match pong {
        IpcMessage::Pong { to } => assert_eq!(to, "unit-test-peer"),
        other => {
            return Err(BearDogError::invalid_input(&format!(
                "expected pong, got {:?}",
                other
            )));
        }
    }

    drop(reader);
    serve.abort();
    Ok(())
}

#[cfg(unix)]
#[tokio::test]
async fn handle_connection_malformed_json_line_is_skipped_without_panic() {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    let (mut local, remote) = tokio::net::UnixStream::pair().expect("unix pair");
    let handler: Arc<dyn IpcHandler> = Arc::new(TestHandler);
    let connections = Arc::new(RwLock::new(Vec::new()));
    let h = Arc::clone(&handler);
    let c = Arc::clone(&connections);
    let serve = tokio::spawn(async move {
        IpcServer::handle_connection(remote, h, c)
            .await
            .expect("handle")
    });

    local
        .write_all(b"not-json-at-all\n")
        .await
        .expect("write garbage");
    let valid = serde_json::to_string(&IpcMessage::Ping {
        from: "after-garbage".to_string(),
    })
    .expect("serialize");
    local.write_all(valid.as_bytes()).await.expect("write");
    local.write_all(b"\n").await.expect("nl");

    let mut reader = BufReader::new(&mut local);
    let mut out = String::new();
    reader.read_line(&mut out).await.expect("read");
    let msg: IpcMessage = serde_json::from_str(out.trim()).expect("second line");
    assert!(
        matches!(msg, IpcMessage::Pong { ref to } if to == "after-garbage"),
        "{msg:?}"
    );

    drop(reader);
    serve.abort();
}

/// Remote closed the connection before sending a line: `read_line` returns `Ok(0)`.
#[cfg(unix)]
#[tokio::test]
async fn handle_connection_eof_without_request_exits_ok() {
    let (local, remote) = tokio::net::UnixStream::pair().expect("unix pair");
    drop(local);

    let handler: Arc<dyn IpcHandler> = Arc::new(TestHandler);
    let connections = Arc::new(RwLock::new(Vec::new()));
    IpcServer::handle_connection(remote, handler, connections)
        .await
        .expect("EOF should complete without error");
}
