// SPDX-License-Identifier: AGPL-3.0-only

//! Protocol detection, routing, capabilities, endpoint.

use crate::isomorphic::IpcEndpoint;
use crate::protocol_router::{Protocol, ProtocolCapabilities, ProtocolDetector, RouterConfig};
use std::path::PathBuf;

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
    assert!(format!("{d:?}").contains("16")); // default peek_size
}

#[test]
fn test_protocol_detector_custom_peek_size() {
    let d = ProtocolDetector::with_peek_size(32);
    assert!(format!("{d:?}").contains("32"));
}

#[test]
fn test_protocol_detector_default() {
    let d = ProtocolDetector::default();
    assert!(format!("{d:?}").contains("ProtocolDetector"));
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
    assert!(format!("{c:?}").contains("RouterConfig"));
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
    let cloned = ep;
    assert!(format!("{cloned:?}").contains("UnixSocket"));
}
