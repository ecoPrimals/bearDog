// SPDX-License-Identifier: AGPL-3.0-only

//! Self Identity Tests
//!
//! Tests for primal self-identity and discovery


#![allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, clippy::field_reassign_with_default, clippy::manual_range_contains, unused_variables, dead_code)]

use crate::ecosystem::self_discovery::{HealthStatus, SelfIdentity};
use beardog_types::canonical::capabilities::ServiceCapabilityType;
use std::collections::HashMap;

#[test]
fn test_self_identity_creation() {
    use beardog_config::domains::network_ports::DEFAULT_API_PORT;
    
    let identity = SelfIdentity {
        id: "beardog-001".to_string(),
        name: "BearDog".to_string(),
        version: "3.0.0".to_string(),
        capabilities: vec![ServiceCapabilityType::Security],
        endpoint: format!("http://localhost:{}", DEFAULT_API_PORT),
        health_status: HealthStatus::Healthy,
        metadata: HashMap::new(),
    };

    assert_eq!(identity.id, "beardog-001");
    assert_eq!(identity.name, "BearDog");
    assert_eq!(identity.version, "3.0.0");
    assert_eq!(identity.capabilities.len(), 1);
}

#[test]
fn test_self_identity_serialization() {
    let identity = SelfIdentity {
        id: "test-id".to_string(),
        name: "TestPrimal".to_string(),
        version: "1.0.0".to_string(),
        capabilities: vec![ServiceCapabilityType::Security],
        endpoint: "http://localhost:9000".to_string(),
        health_status: HealthStatus::Healthy,
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        metadata: HashMap::new(),
    };

    let json = serde_json::to_string(&identity).expect("Serialization should succeed");
    assert!(json.contains("test-id"));
    assert!(json.contains("TestPrimal"));
}

#[test]
fn test_self_identity_deserialization() {
    let json = r#"{
        "id": "deserialized-id",
        "name": "DeserializedPrimal",
        "version": "2.0.0",
        "capabilities": ["Security"],
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        "endpoint": "http://localhost:7000",
        "health_status": "Healthy",
        "metadata": {}
    }"#;

    let identity: SelfIdentity =
        serde_json::from_str(json).expect("Deserialization should succeed");

    assert_eq!(identity.id, "deserialized-id");
    assert_eq!(identity.name, "DeserializedPrimal");
    assert_eq!(identity.version, "2.0.0");
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_self_identity_multiple_capabilities() {
    let capabilities = vec![
        ServiceCapabilityType::Security,
        ServiceCapabilityType::KeyManagement,
        ServiceCapabilityType::Storage,
    ];
    use beardog_config::domains::network_ports::DEFAULT_API_PORT;
    
    let identity = SelfIdentity {
        id: "multi-cap".to_string(),
        name: "MultiCapability".to_string(),
        version: "1.0.0".to_string(),
        capabilities,
        endpoint: format!("http://localhost:{}", DEFAULT_API_PORT),
        health_status: HealthStatus::Healthy,
        metadata: HashMap::new(),
    };

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(identity.capabilities.len(), 3);
}

#[test]
fn test_self_identity_with_metadata() {
    let mut metadata = HashMap::new();
    metadata.insert("region".to_string(), serde_json::json!("us-west-2"));
    metadata.insert("tier".to_string(), serde_json::json!("production"));

    let identity = SelfIdentity {
        id: "metadata-test".to_string(),
        name: "MetadataTest".to_string(),
        version: "1.0.0".to_string(),
        capabilities: vec![],
        endpoint: "http://localhost:8080".to_string(),
        health_status: HealthStatus::Healthy,
        metadata,
    };
 // TEST_CATEGORY: integration
 // TEST_DOMAIN: core
 // TEST_PRIORITY: normal

    assert_eq!(identity.metadata.len(), 2);
    assert!(identity.metadata.contains_key("region"));
    assert!(identity.metadata.contains_key("tier"));
}

#[test]
fn test_self_identity_health_statuses() {
    let statuses = vec![
        HealthStatus::Healthy,
        HealthStatus::Degraded,
        HealthStatus::Unhealthy,
        HealthStatus::Unknown,
    ];

    for status in statuses {
        let identity = SelfIdentity {
            id: format!("status-{status:?}"),
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            name: "StatusTest".to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec![],
            endpoint: "http://localhost:8080".to_string(),
            health_status: status.clone(),
            metadata: HashMap::new(),
        };

        assert_eq!(identity.health_status, status);
    }
}

#[test]
fn test_self_identity_clone() {
    let identity = SelfIdentity {
        id: "clone-test".to_string(),
        name: "CloneTest".to_string(),
        version: "1.0.0".to_string(),
        capabilities: vec![ServiceCapabilityType::Security],
        endpoint: "http://localhost:8080".to_string(),
        health_status: HealthStatus::Healthy,
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        metadata: HashMap::new(),
    };

    let cloned = identity.clone();

    assert_eq!(identity.id, cloned.id);
    assert_eq!(identity.name, cloned.name);
    assert_eq!(identity.version, cloned.version);
    assert_eq!(identity.health_status, cloned.health_status);
}

#[test]
fn test_health_status_equality() {
    assert_eq!(HealthStatus::Healthy, HealthStatus::Healthy);
    assert_eq!(HealthStatus::Degraded, HealthStatus::Degraded);
    assert_ne!(HealthStatus::Healthy, HealthStatus::Degraded);
}
 // TEST_CATEGORY: integration
 // TEST_DOMAIN: core
 // TEST_PRIORITY: normal

#[test]
fn test_self_identity_empty_capabilities() {
    let identity = SelfIdentity {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        id: "no-caps".to_string(),
        name: "NoCaps".to_string(),
        version: "1.0.0".to_string(),
        capabilities: vec![],
        endpoint: "http://localhost:8080".to_string(),
        health_status: HealthStatus::Healthy,
        metadata: HashMap::new(),
    };

    assert!(identity.capabilities.is_empty());
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_self_identity_endpoint_formats() {
    let endpoints = vec![
        "http://localhost:8080",
        "https://api.example.com",
        "grpc://service.local:9090",
        "tcp://192.168.1.1:3000",
    ];

    for endpoint in endpoints {
        let identity = SelfIdentity {
            id: "endpoint-test".to_string(),
            name: "EndpointTest".to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec![],
            endpoint: endpoint.to_string(),
            health_status: HealthStatus::Healthy,
            metadata: HashMap::new(),
        };

        assert_eq!(identity.endpoint, endpoint);
    }
}
