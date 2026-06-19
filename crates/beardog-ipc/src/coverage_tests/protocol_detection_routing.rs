// SPDX-License-Identifier: AGPL-3.0-or-later

//! Protocol detection, routing, capabilities, endpoint.

use crate::isomorphic::IpcEndpoint;
use crate::protocol_router::{Protocol, ProtocolCapabilities, ProtocolDetector, RouterConfig};
use std::path::PathBuf;

// ============================================================================
// Protocol detection comprehensive tests
// ============================================================================

#[test]
fn test_detect_binary_frame() {
    let bytes = vec![0x10, 0x00, 0x00, 0x00, 0x01];
    assert_eq!(
        ProtocolDetector::detect_from_bytes(&bytes),
        Protocol::BinaryFrame
    );
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
    assert_eq!(
        ProtocolDetector::detect_from_bytes(&[0x01, 0x02]),
        Protocol::Unknown
    );
}

// ============================================================================
// Protocol enum comprehensive tests
// ============================================================================

#[test]
fn test_protocol_names() {
    assert_eq!(Protocol::JsonRpc.name(), "json-rpc");
    assert_eq!(Protocol::BinaryFrame.name(), "binary-frame");
    assert_eq!(Protocol::Http.name(), "http");
    assert_eq!(Protocol::Unknown.name(), "unknown");
}

#[test]
fn test_protocol_is_primary() {
    assert!(Protocol::JsonRpc.is_primary());
    assert!(!Protocol::BinaryFrame.is_primary());
    assert!(!Protocol::Http.is_primary());
    assert!(!Protocol::Unknown.is_primary());
}

#[test]
fn test_protocol_display() {
    assert_eq!(format!("{}", Protocol::JsonRpc), "json-rpc");
    assert_eq!(format!("{}", Protocol::BinaryFrame), "binary-frame");
    assert_eq!(format!("{}", Protocol::Http), "http");
    assert_eq!(format!("{}", Protocol::Unknown), "unknown");
}

#[test]
fn test_protocol_eq_and_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(Protocol::BinaryFrame);
    set.insert(Protocol::JsonRpc);
    set.insert(Protocol::BinaryFrame);
    assert_eq!(set.len(), 2);
}

#[test]
fn test_protocol_clone_copy() {
    let p = Protocol::JsonRpc;
    let c = p;
    assert_eq!(p, c);
}

// ============================================================================
// ProtocolDetector tests
// ============================================================================

#[test]
fn test_protocol_detector_new() {
    let d = ProtocolDetector::new();
    assert!(format!("{d:?}").contains("16"));
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
    assert!(c.enable_jsonrpc);
    assert!(c.enable_http);
}

#[test]
fn test_router_config_jsonrpc_only() {
    let c = RouterConfig::jsonrpc_only();
    assert!(c.enable_jsonrpc);
    assert!(!c.enable_http);
}

#[test]
fn test_router_config_supported_protocols_all() {
    let c = RouterConfig::default();
    let protos = c.supported_protocols();
    assert_eq!(protos.len(), 2);
    assert!(protos.contains(&Protocol::JsonRpc));
    assert!(protos.contains(&Protocol::Http));
}

#[test]
fn test_router_config_is_supported() {
    let c = RouterConfig::jsonrpc_only();
    assert!(c.is_supported(Protocol::JsonRpc));
    assert!(!c.is_supported(Protocol::Http));
    assert!(!c.is_supported(Protocol::Unknown));
    assert!(!c.is_supported(Protocol::BinaryFrame));
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
    assert_eq!(c.enable_jsonrpc, c2.enable_jsonrpc);
    assert!(format!("{c:?}").contains("RouterConfig"));
}

// ============================================================================
// ProtocolCapabilities tests
// ============================================================================

#[test]
fn test_capabilities_from_default_config() {
    let config = RouterConfig::default();
    let caps = ProtocolCapabilities::from_config(&config);
    assert!(caps.supported.contains(&"json-rpc".to_string()));
    assert!(caps.supported.contains(&"http".to_string()));
    assert_eq!(caps.recommended, "json-rpc");
    assert!(caps.versions.contains_key("json-rpc"));
    assert!(caps.versions.contains_key("http"));
}

#[test]
fn test_capabilities_from_jsonrpc_only() {
    let config = RouterConfig::jsonrpc_only();
    let caps = ProtocolCapabilities::from_config(&config);
    assert_eq!(caps.supported.len(), 1);
    assert_eq!(caps.recommended, "json-rpc");
}

#[test]
fn test_capabilities_to_json() {
    let config = RouterConfig::default();
    let caps = ProtocolCapabilities::from_config(&config);
    let json = caps.to_json();
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
