//! Ecosystem Registration Tests
//!
//! Tests for service registration and ecosystem integration

use crate::ecosystem::service_registration::EcosystemRegistration;
use beardog_types::canonical::HealthStatus;
use std::collections::HashMap;

#[test]
fn test_ecosystem_registration_creation() {
    let mut endpoints = HashMap::new();
    endpoints.insert("http".to_string(), "http://localhost:8080".to_string());

    let registration = EcosystemRegistration {
        service_id: "test-service-123".to_string(),
        service_name: "Test Service".to_string(),
        version: "1.0.0".to_string(),
        endpoints,
        capabilities: vec!["compute".to_string(), "storage".to_string()],
        health_status: HealthStatus::Healthy,
    };

    assert_eq!(registration.service_id, "test-service-123");
    assert_eq!(registration.service_name, "Test Service");
    assert_eq!(registration.version, "1.0.0");
    assert_eq!(registration.capabilities.len(), 2);
}

#[test]
fn test_ecosystem_registration_serialization() {
    let mut endpoints = HashMap::new();
    endpoints.insert("http".to_string(), "http://localhost:8080".to_string());
 // TEST_CATEGORY: integration
 // TEST_DOMAIN: core
 // TEST_PRIORITY: normal

    let registration = EcosystemRegistration {
        service_id: "test-123".to_string(),
        service_name: "Test".to_string(),
        version: "1.0.0".to_string(),
        endpoints,
        capabilities: vec!["test".to_string()],
        health_status: HealthStatus::Healthy,
    };

    let json = serde_json::to_string(&registration).expect("Serialization should succeed");
    assert!(json.contains("test-123"));
    assert!(json.contains("Test"));
    assert!(json.contains("1.0.0"));
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_ecosystem_registration_deserialization() {
    let json = r#"{
        "service_id": "test-456",
        "service_name": "Deserialized Service",
        "version": "2.1.0",
        "endpoints": {"http": "http://localhost:9000"},
        "capabilities": ["compute", "analytics"],
        "health_status": "Healthy"
    }"#;

    let registration: EcosystemRegistration =
        serde_json::from_str(json).expect("Deserialization should succeed");

    assert_eq!(registration.service_id, "test-456");
    assert_eq!(registration.service_name, "Deserialized Service");
    assert_eq!(registration.version, "2.1.0");
    assert_eq!(registration.capabilities.len(), 2);
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
}

#[test]
fn test_ecosystem_registration_multiple_endpoints() {
    let mut endpoints = HashMap::new();
    endpoints.insert("http".to_string(), "http://localhost:8080".to_string());
    endpoints.insert("https".to_string(), "https://localhost:8443".to_string());
    endpoints.insert("grpc".to_string(), "grpc://localhost:9090".to_string());

    let registration = EcosystemRegistration {
        service_id: "multi-endpoint".to_string(),
        service_name: "Multi Endpoint Service".to_string(),
        version: "1.0.0".to_string(),
        endpoints,
        capabilities: vec!["multi-protocol".to_string()],
        health_status: HealthStatus::Healthy,
    };
 // TEST_CATEGORY: integration
 // TEST_DOMAIN: core
 // TEST_PRIORITY: normal

    assert_eq!(registration.endpoints.len(), 3);
    assert!(registration.endpoints.contains_key("http"));
    assert!(registration.endpoints.contains_key("https"));
    assert!(registration.endpoints.contains_key("grpc"));
}

#[test]
fn test_ecosystem_registration_no_capabilities() {
    let registration = EcosystemRegistration {
        service_id: "no-caps".to_string(),
        service_name: "No Capabilities".to_string(),
        version: "1.0.0".to_string(),
        endpoints: HashMap::new(),
        capabilities: vec![],
        health_status: HealthStatus::Healthy,
    };

    assert!(registration.capabilities.is_empty());
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
}

#[test]
fn test_ecosystem_registration_health_statuses() {
    let statuses = vec![
        HealthStatus::Healthy,
        HealthStatus::Degraded,
        HealthStatus::Unhealthy,
        HealthStatus::Unknown,
    ];

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    for status in statuses {
        let registration = EcosystemRegistration {
            service_id: format!("service-{status:?}"),
            service_name: "Test".to_string(),
            version: "1.0.0".to_string(),
            endpoints: HashMap::new(),
            capabilities: vec![],
            health_status: status,
        };

        assert_eq!(registration.health_status, status);
    }
}

#[test]
fn test_ecosystem_registration_clone() {
    let mut endpoints = HashMap::new();
    endpoints.insert("http".to_string(), "http://localhost:8080".to_string());

    let registration = EcosystemRegistration {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        service_id: "clone-test".to_string(),
        service_name: "Clone Test".to_string(),
        version: "1.0.0".to_string(),
        endpoints,
        capabilities: vec!["test".to_string()],
        health_status: HealthStatus::Healthy,
    };

    let cloned = registration.clone();

    assert_eq!(registration.service_id, cloned.service_id);
    assert_eq!(registration.service_name, cloned.service_name);
    assert_eq!(registration.version, cloned.version);
    assert_eq!(registration.capabilities, cloned.capabilities);
}

#[test]
fn test_ecosystem_registration_version_formats() {
    let versions = vec!["1.0.0", "2.1.3", "0.0.1-alpha", "1.0.0-beta.1"];
 // TEST_CATEGORY: integration
 // TEST_DOMAIN: core
 // TEST_PRIORITY: normal

    for version in versions {
        let registration = EcosystemRegistration {
            service_id: "version-test".to_string(),
            service_name: "Version Test".to_string(),
            version: version.to_string(),
            endpoints: HashMap::new(),
            capabilities: vec![],
            health_status: HealthStatus::Healthy,
        };

        assert_eq!(registration.version, version);
    }
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_ecosystem_registration_capability_management() {
    let capabilities = vec![
        "compute".to_string(),
        "storage".to_string(),
        "networking".to_string(),
        "security".to_string(),
    ];

    let registration = EcosystemRegistration {
        service_id: "capability-test".to_string(),
        service_name: "Capability Test".to_string(),
        version: "1.0.0".to_string(),
        endpoints: HashMap::new(),
        capabilities,
        health_status: HealthStatus::Healthy,
    };

    assert_eq!(registration.capabilities.len(), 4);
    assert!(registration.capabilities.contains(&"compute".to_string()));
    assert!(registration.capabilities.contains(&"security".to_string()));
}
