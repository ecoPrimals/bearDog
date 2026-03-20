// SPDX-License-Identifier: AGPL-3.0-only

//! Errors, capabilities, service info, discovery query coverage tests.

use crate::error::{IpcError, IpcResult};
use crate::isomorphic::IpcEndpoint;
use crate::protocol_router::{Protocol, ProtocolCapabilities, ProtocolDetector, RouterConfig};
use crate::registry_client::{
    JsonRpcError, JsonRpcRequest, JsonRpcResponse, PrimalInfo, PrimalRegistryClient,
};
use crate::types::{Capability, DiscoveryQuery, ServiceInfo};
use std::collections::HashMap;

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
