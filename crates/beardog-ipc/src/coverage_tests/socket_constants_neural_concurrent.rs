// SPDX-License-Identifier: AGPL-3.0-only

//! Constants, neural discovery path, concurrent protocol/router checks.

use crate::protocol_router::{Protocol, ProtocolCapabilities, ProtocolDetector, RouterConfig};
use crate::types::Capability;

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
