// SPDX-License-Identifier: AGPL-3.0-only

//! Coverage expansion tests for beardog-ipc
//!
//! Comprehensive tests for protocol detection, routing, types, discovery,
//! and error handling. All tests are concurrent-safe — no sleeps, no serial.

use crate::error::{IpcError, IpcResult};
use crate::isomorphic::IpcEndpoint;
use crate::protocol_router::{Protocol, ProtocolCapabilities, ProtocolDetector, RouterConfig};
use crate::registry_client::{
    JsonRpcError, JsonRpcRequest, JsonRpcResponse, PrimalInfo, PrimalRegistryClient,
};
use crate::types::{Capability, DiscoveryQuery, ServiceInfo};
use std::collections::HashMap;
use std::path::PathBuf;

// ============================================================================
// IpcError comprehensive tests
// ============================================================================

#[test]
fn test_ipc_error_display_connection() {
    let err = IpcError::Connection("conn failed".to_string());
    assert_eq!(format!("{}", err), "Connection error: conn failed");
}

#[test]
fn test_ipc_error_display_protocol() {
    let err = IpcError::Protocol("bad protocol".to_string());
    assert_eq!(format!("{}", err), "Protocol error: bad protocol");
}

#[test]
fn test_ipc_error_display_serialization() {
    let err = IpcError::Serialization("bad json".to_string());
    assert_eq!(format!("{}", err), "Serialization error: bad json");
}

#[test]
fn test_ipc_error_display_service_not_found() {
    let err = IpcError::ServiceNotFound("crypto".to_string());
    assert_eq!(format!("{}", err), "Service not found: crypto");
}

#[test]
fn test_ipc_error_display_timeout() {
    let err = IpcError::Timeout;
    assert_eq!(format!("{}", err), "Operation timed out");
}

#[test]
fn test_ipc_error_debug_all_variants() {
    let err = IpcError::Connection("test".to_string());
    assert!(format!("{:?}", err).contains("Connection"));

    let err = IpcError::Protocol("test".to_string());
    assert!(format!("{:?}", err).contains("Protocol"));

    let err = IpcError::Serialization("test".to_string());
    assert!(format!("{:?}", err).contains("Serialization"));

    let err = IpcError::ServiceNotFound("test".to_string());
    assert!(format!("{:?}", err).contains("ServiceNotFound"));

    let err = IpcError::Timeout;
    assert!(format!("{:?}", err).contains("Timeout"));
}

#[test]
fn test_ipc_error_from_io() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
    let ipc_err: IpcError = io_err.into();
    assert!(format!("{}", ipc_err).contains("I/O error"));
}

#[test]
fn test_ipc_result_ok() {
    let ok: IpcResult<u32> = Ok(42);
    assert!(ok.is_ok());
    assert_eq!(ok.unwrap(), 42);
}

#[test]
fn test_ipc_result_err() {
    let err: IpcResult<u32> = Err(IpcError::Timeout);
    assert!(err.is_err());
}

// ============================================================================
// Capability comprehensive tests
// ============================================================================

#[test]
fn test_capability_crypto_as_str() {
    assert_eq!(Capability::Crypto.as_str(), "crypto");
}

#[test]
fn test_capability_btsp_as_str() {
    assert_eq!(Capability::BTSP.as_str(), "btsp");
}

#[test]
fn test_capability_ed25519_as_str() {
    assert_eq!(Capability::Ed25519.as_str(), "ed25519");
}

#[test]
fn test_capability_x25519_as_str() {
    assert_eq!(Capability::X25519.as_str(), "x25519");
}

#[test]
fn test_capability_chacha20poly1305_as_str() {
    assert_eq!(Capability::ChaCha20Poly1305.as_str(), "chacha20poly1305");
}

#[test]
fn test_capability_aesgcm_as_str() {
    assert_eq!(Capability::AesGcm.as_str(), "aesgcm");
}

#[test]
fn test_capability_storage_as_str() {
    assert_eq!(Capability::Storage.as_str(), "storage");
}

#[test]
fn test_capability_ai_as_str() {
    assert_eq!(Capability::AI.as_str(), "ai");
}

#[test]
fn test_capability_discovery_as_str() {
    assert_eq!(Capability::Discovery.as_str(), "discovery");
}

#[test]
fn test_capability_custom_as_str() {
    assert_eq!(Capability::Custom("my_cap".to_string()).as_str(), "my_cap");
}

#[test]
fn test_capability_serialization_all_variants() {
    let caps = vec![
        Capability::Crypto,
        Capability::BTSP,
        Capability::Ed25519,
        Capability::X25519,
        Capability::ChaCha20Poly1305,
        Capability::AesGcm,
        Capability::Storage,
        Capability::AI,
        Capability::Discovery,
        Capability::Custom("test_custom".to_string()),
    ];
    for cap in &caps {
        let json = serde_json::to_string(cap).unwrap();
        let restored: Capability = serde_json::from_str(&json).unwrap();
        assert_eq!(cap, &restored);
    }
}

#[test]
fn test_capability_hash_and_eq() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(Capability::Crypto);
    set.insert(Capability::BTSP);
    set.insert(Capability::Crypto); // duplicate
    assert_eq!(set.len(), 2);
}

#[test]
fn test_capability_clone_and_debug() {
    let cap = Capability::Custom("cloned".to_string());
    let cloned = cap.clone();
    assert_eq!(cap, cloned);
    assert!(format!("{:?}", cap).contains("cloned"));
}

// ============================================================================
// ServiceInfo tests
// ============================================================================

#[test]
fn test_service_info_serialization_roundtrip() {
    let info = ServiceInfo {
        name: "beardog".to_string(),
        endpoint: "/primal/beardog".to_string(),
        capabilities: vec!["crypto".to_string(), "btsp".to_string()],
        version: "0.9.0".to_string(),
        available: true,
        metadata: HashMap::new(),
    };
    let json = serde_json::to_string(&info).unwrap();
    let restored: ServiceInfo = serde_json::from_str(&json).unwrap();
    assert_eq!(info.name, restored.name);
    assert_eq!(info.endpoint, restored.endpoint);
    assert_eq!(info.capabilities, restored.capabilities);
    assert!(restored.available);
}

#[test]
fn test_service_info_default_available() {
    let json = r#"{"name":"test","endpoint":"/test","capabilities":[],"version":"1.0"}"#;
    let info: ServiceInfo = serde_json::from_str(json).unwrap();
    assert!(info.available);
}

#[test]
fn test_service_info_not_available() {
    let json =
        r#"{"name":"test","endpoint":"/test","capabilities":[],"version":"1.0","available":false}"#;
    let info: ServiceInfo = serde_json::from_str(json).unwrap();
    assert!(!info.available);
}

#[test]
fn test_service_info_with_metadata() {
    let mut metadata = HashMap::new();
    metadata.insert("description".to_string(), serde_json::json!("Test service"));
    let info = ServiceInfo {
        name: "test".to_string(),
        endpoint: "/test".to_string(),
        capabilities: vec![],
        version: "1.0".to_string(),
        available: false,
        metadata,
    };
    let json = serde_json::to_string(&info).unwrap();
    assert!(json.contains("description"));
}

#[test]
fn test_service_info_debug() {
    let info = ServiceInfo {
        name: "test".to_string(),
        endpoint: "/test".to_string(),
        capabilities: vec![],
        version: "1.0".to_string(),
        available: true,
        metadata: HashMap::new(),
    };
    let debug = format!("{:?}", info);
    assert!(debug.contains("test"));
}

// ============================================================================
// DiscoveryQuery comprehensive tests
// ============================================================================

#[test]
fn test_discovery_query_new_is_empty() {
    let q = DiscoveryQuery::new();
    assert!(q.primal.is_none());
    assert!(q.capabilities.is_empty());
    assert!(q.filters.is_empty());
}

#[test]
fn test_discovery_query_default() {
    let q = DiscoveryQuery::default();
    assert!(q.primal.is_none());
}

#[test]
fn test_discovery_query_primal() {
    let q = DiscoveryQuery::primal("beardog");
    assert_eq!(q.primal.unwrap(), "beardog");
    assert!(q.capabilities.is_empty());
}

#[test]
fn test_discovery_query_capability() {
    let q = DiscoveryQuery::capability(Capability::Crypto);
    assert!(q.primal.is_none());
    assert_eq!(q.capabilities.len(), 1);
    assert_eq!(q.capabilities[0], "crypto");
}

#[test]
fn test_discovery_query_chained_capabilities() {
    let q = DiscoveryQuery::capability(Capability::Crypto)
        .with_capability(Capability::Ed25519)
        .with_capability(Capability::X25519);
    assert_eq!(q.capabilities.len(), 3);
    assert_eq!(q.capabilities[0], "crypto");
    assert_eq!(q.capabilities[1], "ed25519");
    assert_eq!(q.capabilities[2], "x25519");
}

#[test]
fn test_discovery_query_with_filter() {
    let q = DiscoveryQuery::new()
        .with_filter("region".to_string(), serde_json::json!("us-east-1"))
        .with_filter("version".to_string(), serde_json::json!("0.9.0"));
    assert_eq!(q.filters.len(), 2);
    assert_eq!(q.filters["region"], serde_json::json!("us-east-1"));
}

#[test]
fn test_discovery_query_serialization() {
    let q = DiscoveryQuery::primal("beardog");
    let json = serde_json::to_string(&q).unwrap();
    let restored: DiscoveryQuery = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.primal.unwrap(), "beardog");
}

#[test]
fn test_discovery_query_debug() {
    let q = DiscoveryQuery::primal("beardog");
    assert!(format!("{:?}", q).contains("beardog"));
}

#[test]
fn test_discovery_query_clone() {
    let q = DiscoveryQuery::primal("beardog").with_capability(Capability::Crypto);
    let cloned = q.clone();
    assert_eq!(cloned.primal, q.primal);
    assert_eq!(cloned.capabilities, q.capabilities);
}

// ============================================================================
// Protocol detection comprehensive tests
// ============================================================================

#[test]
fn test_detect_tarpc_binary_frame() {
    // Simulate tarpc binary: 4-byte length prefix + non-ASCII payload
    let bytes = vec![0x10, 0x00, 0x00, 0x00, 0x01];
    assert_eq!(ProtocolDetector::detect_from_bytes(&bytes), Protocol::Tarpc);
}

#[test]
fn test_detect_json_rpc_with_whitespace() {
    let bytes = b"  \n\t{\"jsonrpc\": \"2.0\"}";
    assert_eq!(
        ProtocolDetector::detect_from_bytes(bytes),
        Protocol::JsonRpc
    );
}

#[test]
fn test_detect_plain_json() {
    let bytes = b"{\"data\": 42}";
    assert_eq!(
        ProtocolDetector::detect_from_bytes(bytes),
        Protocol::JsonRpc
    );
}

#[test]
fn test_detect_http_get() {
    assert_eq!(
        ProtocolDetector::detect_from_bytes(b"GET / HTTP/1.1\r\n"),
        Protocol::Http
    );
}

#[test]
fn test_detect_http_post() {
    assert_eq!(
        ProtocolDetector::detect_from_bytes(b"POST /api HTTP/1.1\r\n"),
        Protocol::Http
    );
}

#[test]
fn test_detect_http_put() {
    assert_eq!(
        ProtocolDetector::detect_from_bytes(b"PUT /data HTTP/1.1\r\n"),
        Protocol::Http
    );
}

#[test]
fn test_detect_http_delete() {
    assert_eq!(
        ProtocolDetector::detect_from_bytes(b"DELETE /item HTTP/1.1\r\n"),
        Protocol::Http
    );
}

#[test]
fn test_detect_http_patch() {
    assert_eq!(
        ProtocolDetector::detect_from_bytes(b"PATCH /item HTTP/1.1\r\n"),
        Protocol::Http
    );
}

#[test]
fn test_detect_http_head() {
    assert_eq!(
        ProtocolDetector::detect_from_bytes(b"HEAD / HTTP/1.1\r\n"),
        Protocol::Http
    );
}

#[test]
fn test_detect_http_options() {
    assert_eq!(
        ProtocolDetector::detect_from_bytes(b"OPTIONS / HTTP/1.1\r\n"),
        Protocol::Http
    );
}

#[test]
fn test_detect_http_connect() {
    assert_eq!(
        ProtocolDetector::detect_from_bytes(b"CONNECT host:443 HTTP/1.1\r\n"),
        Protocol::Http
    );
}

#[test]
fn test_detect_http_response() {
    assert_eq!(
        ProtocolDetector::detect_from_bytes(b"HTTP/1.1 200 OK\r\n"),
        Protocol::Http
    );
}

#[test]
fn test_detect_empty() {
    assert_eq!(ProtocolDetector::detect_from_bytes(&[]), Protocol::Unknown);
}

#[test]
fn test_detect_unknown_binary() {
    assert_eq!(
        ProtocolDetector::detect_from_bytes(&[0xFF, 0xFF]),
        Protocol::Unknown
    );
}

#[test]
fn test_detect_short_binary() {
    // Less than 4 bytes — can't be tarpc
    assert_eq!(
        ProtocolDetector::detect_from_bytes(&[0x01, 0x02]),
        Protocol::Unknown
    );
}

// ============================================================================
// Protocol enum comprehensive tests
// ============================================================================

#[test]
fn test_protocol_priority_ordering() {
    assert!(Protocol::Tarpc.priority() > Protocol::JsonRpc.priority());
    assert!(Protocol::JsonRpc.priority() > Protocol::Http.priority());
    assert!(Protocol::Http.priority() > Protocol::Unknown.priority());
}

#[test]
fn test_protocol_names() {
    assert_eq!(Protocol::Tarpc.name(), "tarpc");
    assert_eq!(Protocol::JsonRpc.name(), "json-rpc");
    assert_eq!(Protocol::Http.name(), "http");
    assert_eq!(Protocol::Unknown.name(), "unknown");
}

#[test]
fn test_protocol_is_high_performance() {
    assert!(Protocol::Tarpc.is_high_performance());
    assert!(!Protocol::JsonRpc.is_high_performance());
    assert!(!Protocol::Http.is_high_performance());
    assert!(!Protocol::Unknown.is_high_performance());
}

#[test]
fn test_protocol_display() {
    assert_eq!(format!("{}", Protocol::Tarpc), "tarpc");
    assert_eq!(format!("{}", Protocol::JsonRpc), "json-rpc");
    assert_eq!(format!("{}", Protocol::Http), "http");
    assert_eq!(format!("{}", Protocol::Unknown), "unknown");
}

#[test]
fn test_protocol_eq_and_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(Protocol::Tarpc);
    set.insert(Protocol::JsonRpc);
    set.insert(Protocol::Tarpc); // duplicate
    assert_eq!(set.len(), 2);
}

#[test]
fn test_protocol_clone_copy() {
    let p = Protocol::JsonRpc;
    let c = p; // Copy
    assert_eq!(p, c);
}

// ============================================================================
// ProtocolDetector tests
// ============================================================================

#[test]
fn test_protocol_detector_new() {
    let d = ProtocolDetector::new();
    assert!(format!("{:?}", d).contains("16")); // default peek_size
}

#[test]
fn test_protocol_detector_custom_peek_size() {
    let d = ProtocolDetector::with_peek_size(32);
    assert!(format!("{:?}", d).contains("32"));
}

#[test]
fn test_protocol_detector_default() {
    let d = ProtocolDetector::default();
    assert!(format!("{:?}", d).contains("ProtocolDetector"));
}

// ============================================================================
// RouterConfig comprehensive tests
// ============================================================================

#[test]
fn test_router_config_default() {
    let c = RouterConfig::default();
    assert!(c.enable_tarpc);
    assert!(c.enable_jsonrpc);
    assert!(c.enable_http);
    assert_eq!(c.preferred, Protocol::Tarpc);
}

#[test]
fn test_router_config_tarpc_only() {
    let c = RouterConfig::tarpc_only();
    assert!(c.enable_tarpc);
    assert!(!c.enable_jsonrpc);
    assert!(!c.enable_http);
    assert_eq!(c.preferred, Protocol::Tarpc);
}

#[test]
fn test_router_config_jsonrpc_only() {
    let c = RouterConfig::jsonrpc_only();
    assert!(!c.enable_tarpc);
    assert!(c.enable_jsonrpc);
    assert!(!c.enable_http);
    assert_eq!(c.preferred, Protocol::JsonRpc);
}

#[test]
fn test_router_config_development() {
    let c = RouterConfig::development();
    assert!(c.enable_tarpc);
    assert!(c.enable_jsonrpc);
    assert!(c.enable_http);
    assert_eq!(c.preferred, Protocol::JsonRpc);
}

#[test]
fn test_router_config_production() {
    let c = RouterConfig::production();
    assert_eq!(c.preferred, Protocol::Tarpc);
}

#[test]
fn test_router_config_supported_protocols_all() {
    let c = RouterConfig::default();
    let protos = c.supported_protocols();
    assert_eq!(protos.len(), 3);
    assert!(protos.contains(&Protocol::Tarpc));
    assert!(protos.contains(&Protocol::JsonRpc));
    assert!(protos.contains(&Protocol::Http));
}

#[test]
fn test_router_config_is_supported() {
    let c = RouterConfig::tarpc_only();
    assert!(c.is_supported(Protocol::Tarpc));
    assert!(!c.is_supported(Protocol::JsonRpc));
    assert!(!c.is_supported(Protocol::Http));
    assert!(!c.is_supported(Protocol::Unknown));
}

#[test]
fn test_router_config_supported_protocols_jsonrpc_only() {
    let c = RouterConfig::jsonrpc_only();
    let protos = c.supported_protocols();
    assert_eq!(protos.len(), 1);
    assert_eq!(protos[0], Protocol::JsonRpc);
}

#[test]
fn test_router_config_debug_and_clone() {
    let c = RouterConfig::default();
    let c2 = c.clone();
    assert_eq!(c.enable_tarpc, c2.enable_tarpc);
    assert!(format!("{:?}", c).contains("RouterConfig"));
}

// ============================================================================
// ProtocolCapabilities tests
// ============================================================================

#[test]
fn test_capabilities_from_default_config() {
    let config = RouterConfig::default();
    let caps = ProtocolCapabilities::from_config(&config);
    assert!(caps.supported.contains(&"tarpc".to_string()));
    assert!(caps.supported.contains(&"json-rpc".to_string()));
    assert!(caps.supported.contains(&"http".to_string()));
    assert!(caps.high_performance.contains(&"tarpc".to_string()));
    assert_eq!(caps.recommended, "tarpc");
    assert!(caps.versions.contains_key("tarpc"));
    assert!(caps.versions.contains_key("json-rpc"));
    assert!(caps.versions.contains_key("http"));
}

#[test]
fn test_capabilities_from_jsonrpc_only() {
    let config = RouterConfig::jsonrpc_only();
    let caps = ProtocolCapabilities::from_config(&config);
    assert_eq!(caps.supported.len(), 1);
    assert!(caps.high_performance.is_empty());
    assert_eq!(caps.recommended, "json-rpc");
}

#[test]
fn test_capabilities_from_tarpc_only() {
    let config = RouterConfig::tarpc_only();
    let caps = ProtocolCapabilities::from_config(&config);
    assert_eq!(caps.supported.len(), 1);
    assert_eq!(caps.high_performance.len(), 1);
    assert_eq!(caps.recommended, "tarpc");
}

#[test]
fn test_capabilities_to_json() {
    let config = RouterConfig::default();
    let caps = ProtocolCapabilities::from_config(&config);
    let json = caps.to_json();
    assert!(json.contains("tarpc"));
    assert!(json.contains("json-rpc"));
    assert!(json.contains("supported"));
    assert!(json.contains("recommended"));
}

#[test]
fn test_capabilities_serialization_roundtrip() {
    let config = RouterConfig::default();
    let caps = ProtocolCapabilities::from_config(&config);
    let json = serde_json::to_string(&caps).unwrap();
    let restored: ProtocolCapabilities = serde_json::from_str(&json).unwrap();
    assert_eq!(caps.supported, restored.supported);
    assert_eq!(caps.recommended, restored.recommended);
    assert_eq!(caps.high_performance, restored.high_performance);
}

// ============================================================================
// IpcEndpoint tests
// ============================================================================

#[test]
fn test_ipc_endpoint_unix_display() {
    let ep = IpcEndpoint::UnixSocket(PathBuf::from("/tmp/beardog.sock"));
    assert_eq!(ep.display(), "unix:/tmp/beardog.sock");
    assert!(ep.is_optimal());
}

#[test]
fn test_ipc_endpoint_tcp_display() {
    let ep = IpcEndpoint::TcpLocal("127.0.0.1:9090".parse().unwrap());
    assert_eq!(ep.display(), "tcp:127.0.0.1:9090");
    assert!(!ep.is_optimal());
}

#[test]
fn test_ipc_endpoint_clone_and_debug() {
    let ep = IpcEndpoint::UnixSocket(PathBuf::from("/tmp/test.sock"));
    let cloned = ep.clone();
    assert!(format!("{:?}", cloned).contains("UnixSocket"));
}

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
    assert!(format!("{:?}", req).contains("primal.ping"));
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
    assert!(format!("{:?}", info).contains("beardog"));
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

// ============================================================================
// Constants and module-level tests
// ============================================================================

#[test]
fn test_protocol_version() {
    assert_eq!(crate::PROTOCOL_VERSION, "1.0");
}

#[test]
fn test_discovery_socket_fallback() {
    assert_eq!(crate::DISCOVERY_SOCKET_FALLBACK, "/primal/discovery");
}

#[test]
fn test_default_heartbeat_interval() {
    assert_eq!(
        crate::DEFAULT_HEARTBEAT_INTERVAL,
        std::time::Duration::from_secs(30)
    );
}

// ============================================================================
// Neural API registration tests
// ============================================================================

#[test]
fn test_discover_neural_api_socket_exercises_path() {
    let result = crate::discover_neural_api_socket();
    // Result is Some(path) if socket exists or env is set, None otherwise
    // Just exercise the code path — the actual result depends on environment
    let _ = result;
}

// ============================================================================
// Concurrent tests
// ============================================================================

#[test]
fn test_protocol_detector_concurrent() {
    let handles: Vec<_> = (0..8)
        .map(|i| {
            std::thread::spawn(move || {
                let bytes: Vec<u8> = match i % 4 {
                    0 => br#"{"jsonrpc":"2.0"}"#.to_vec(),
                    1 => b"GET / HTTP/1.1\r\n".to_vec(),
                    2 => vec![0x10, 0x00, 0x00, 0x00, 0x01],
                    _ => vec![],
                };
                let protocol = ProtocolDetector::detect_from_bytes(&bytes);
                match i % 4 {
                    0 => assert_eq!(protocol, Protocol::JsonRpc),
                    1 => assert_eq!(protocol, Protocol::Http),
                    2 => assert_eq!(protocol, Protocol::Tarpc),
                    _ => assert_eq!(protocol, Protocol::Unknown),
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}

#[test]
fn test_router_config_concurrent_creation() {
    let handles: Vec<_> = (0..8)
        .map(|i| {
            std::thread::spawn(move || {
                let config = match i % 4 {
                    0 => RouterConfig::default(),
                    1 => RouterConfig::tarpc_only(),
                    2 => RouterConfig::jsonrpc_only(),
                    _ => RouterConfig::development(),
                };
                let caps = ProtocolCapabilities::from_config(&config);
                assert!(!caps.supported.is_empty());
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}

#[test]
fn test_capability_serde_concurrent() {
    let handles: Vec<_> = (0..8)
        .map(|i| {
            std::thread::spawn(move || {
                let cap = match i % 5 {
                    0 => Capability::Crypto,
                    1 => Capability::BTSP,
                    2 => Capability::Ed25519,
                    3 => Capability::Storage,
                    _ => Capability::Custom(format!("custom_{}", i)),
                };
                let json = serde_json::to_string(&cap).unwrap();
                let restored: Capability = serde_json::from_str(&json).unwrap();
                assert_eq!(cap, restored);
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}
