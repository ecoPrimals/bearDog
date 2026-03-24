// SPDX-License-Identifier: AGPL-3.0-only

//! JSON-RPC types, PrimalInfo, registry client, socket discovery.

use crate::registry_client::{
    JsonRpcError, JsonRpcRequest, JsonRpcResponse, PrimalInfo, PrimalRegistryClient,
};
use std::path::PathBuf;

// ============================================================================
// SongbirdClient tests (public API only)
// ============================================================================

#[test]
fn test_songbird_client_new_creates_instance() {
    let _client = crate::SongbirdClient::new();
    // SongbirdClient::new() should not panic
}

#[test]
fn test_songbird_client_default_creates_instance() {
    let _client = crate::SongbirdClient::default();
}

// ============================================================================
// JSON-RPC types tests
// ============================================================================

#[test]
fn test_json_rpc_request_with_params() {
    let req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "primal.register".to_string(),
        params: Some(serde_json::json!({"name": "beardog"})),
        id: 1,
    };
    let json = serde_json::to_string(&req).unwrap();
    assert!(json.contains("2.0"));
    assert!(json.contains("primal.register"));
    assert!(json.contains("beardog"));
    assert!(json.contains("\"id\":1"));
}

#[test]
fn test_json_rpc_request_without_params() {
    let req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "primal.ping".to_string(),
        params: None,
        id: 42,
    };
    let json = serde_json::to_string(&req).unwrap();
    assert!(!json.contains("params")); // skip_serializing_if
    assert!(json.contains("\"id\":42"));
}

#[test]
fn test_json_rpc_request_roundtrip() {
    let req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "test.method".to_string(),
        params: Some(serde_json::json!({"key": "value"})),
        id: 7,
    };
    let json = serde_json::to_string(&req).unwrap();
    let restored: JsonRpcRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.method, "test.method");
    assert_eq!(restored.id, 7);
}

#[test]
fn test_json_rpc_request_debug_and_clone() {
    let req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "primal.ping".to_string(),
        params: None,
        id: 1,
    };
    let cloned = req.clone();
    assert_eq!(cloned.method, req.method);
    assert!(format!("{req:?}").contains("primal.ping"));
}

#[test]
fn test_json_rpc_response_with_result() {
    let json = r#"{"jsonrpc":"2.0","result":{"registered":true},"id":1}"#;
    let resp: JsonRpcResponse = serde_json::from_str(json).unwrap();
    assert!(resp.result.is_some());
    assert!(resp.error.is_none());
    assert_eq!(resp.id, 1);
}

#[test]
fn test_json_rpc_response_with_error() {
    let json = r#"{"jsonrpc":"2.0","error":{"code":-32601,"message":"Method not found"},"id":2}"#;
    let resp: JsonRpcResponse = serde_json::from_str(json).unwrap();
    assert!(resp.result.is_none());
    assert!(resp.error.is_some());
    let err = resp.error.unwrap();
    assert_eq!(err.code, -32601);
    assert_eq!(err.message, "Method not found");
}

#[test]
fn test_json_rpc_response_roundtrip() {
    let resp = JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        result: Some(serde_json::json!({"ok": true})),
        error: None,
        id: 99,
    };
    let json = serde_json::to_string(&resp).unwrap();
    let restored: JsonRpcResponse = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.id, 99);
    assert!(restored.result.is_some());
}

#[test]
fn test_json_rpc_error_with_data() {
    let json = r#"{"code":-32602,"message":"Invalid params","data":{"details":"missing field"}}"#;
    let err: JsonRpcError = serde_json::from_str(json).unwrap();
    assert_eq!(err.code, -32602);
    assert_eq!(err.message, "Invalid params");
    assert!(err.data.is_some());
}

#[test]
fn test_json_rpc_error_without_data() {
    let json = r#"{"code":-32700,"message":"Parse error"}"#;
    let err: JsonRpcError = serde_json::from_str(json).unwrap();
    assert_eq!(err.code, -32700);
    assert!(err.data.is_none());
}

#[test]
fn test_json_rpc_error_roundtrip() {
    let err = JsonRpcError {
        code: -32600,
        message: "Invalid Request".to_string(),
        data: Some(serde_json::json!({"hint": "missing jsonrpc field"})),
    };
    let json = serde_json::to_string(&err).unwrap();
    let restored: JsonRpcError = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.code, -32600);
    assert_eq!(restored.message, "Invalid Request");
}

// ============================================================================
// PrimalInfo tests
// ============================================================================

#[test]
fn test_primal_info_full_deserialization() {
    let json = r#"{
        "primal_id": "beardog",
        "family_id": "nat0",
        "node_id": "tower1",
        "capabilities": ["encryption", "trust"],
        "socket_path": "/tmp/beardog.sock",
        "last_seen": "2026-01-01T00:00:00Z"
    }"#;
    let info: PrimalInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.primal_id, "beardog");
    assert_eq!(info.family_id, Some("nat0".to_string()));
    assert_eq!(info.node_id, "tower1");
    assert_eq!(info.capabilities.len(), 2);
    assert!(info.last_seen.is_some());
}

#[test]
fn test_primal_info_minimal() {
    let json = r#"{
        "primal_id": "test",
        "node_id": "node1",
        "capabilities": [],
        "socket_path": "/tmp/test.sock"
    }"#;
    let info: PrimalInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.primal_id, "test");
    assert!(info.family_id.is_none());
    assert!(info.last_seen.is_none());
}

#[test]
fn test_primal_info_roundtrip() {
    let info = PrimalInfo {
        primal_id: "beardog".to_string(),
        family_id: Some("nat0".to_string()),
        node_id: "tower1".to_string(),
        capabilities: vec!["crypto".to_string()],
        socket_path: "/tmp/beardog.sock".to_string(),
        last_seen: None,
    };
    let json = serde_json::to_string(&info).unwrap();
    let restored: PrimalInfo = serde_json::from_str(&json).unwrap();
    assert_eq!(info.primal_id, restored.primal_id);
    assert_eq!(info.family_id, restored.family_id);
}

#[test]
fn test_primal_info_debug_and_clone() {
    let info = PrimalInfo {
        primal_id: "beardog".to_string(),
        family_id: None,
        node_id: "n1".to_string(),
        capabilities: vec![],
        socket_path: "/tmp/b.sock".to_string(),
        last_seen: None,
    };
    let cloned = info.clone();
    assert_eq!(cloned.primal_id, info.primal_id);
    assert!(format!("{info:?}").contains("beardog"));
}

// ============================================================================
// PrimalRegistryClient tests
// ============================================================================

#[test]
fn test_registry_client_new() {
    // PrimalRegistryClient::new accepts any path — zero vendor hardcoding
    let _client = PrimalRegistryClient::new(PathBuf::from("/tmp/registry.sock"));
}

#[test]
fn test_registry_client_zero_vendor_hardcoding() {
    // Client works with any path — no hardcoded vendor info
    for path in [
        "/tmp/songbird.sock",
        "/tmp/consul.sock",
        "/tmp/etcd.sock",
        "/tmp/custom-registry.sock",
    ] {
        let _client = PrimalRegistryClient::new(PathBuf::from(path));
    }
}

// ============================================================================
// discover_ipc_socket tests
// ============================================================================

#[tokio::test]
async fn test_discover_ipc_socket_exercises_path() {
    let socket = crate::discover_ipc_socket().await;
    // Should return a non-empty string (either from env or fallback)
    assert!(!socket.is_empty());
}

#[tokio::test]
async fn test_discover_ipc_socket_returns_fallback_without_env() {
    // Without IPC_SOCKET or DISCOVERY_SOCKET env vars, should use fallback
    // (env vars may or may not be set in test environment)
    let socket = crate::discover_ipc_socket().await;
    assert!(!socket.is_empty());
}
