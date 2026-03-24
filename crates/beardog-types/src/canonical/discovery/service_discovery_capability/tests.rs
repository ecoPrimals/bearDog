// SPDX-License-Identifier: AGPL-3.0-only

//! Service Discovery Capability Tests
//!
//! Tests for service descriptor creation and discovery error display.

use super::*;
use crate::canonical::capabilities::ServiceCapabilityType;
use crate::canonical::types::ids::ServiceInstanceId;
use std::collections::HashMap;

#[test]
fn test_service_descriptor_creation() {
    use beardog_config::domains::network_ports::DEFAULT_API_PORT;

    let descriptor = ServiceDescriptor {
        instance_id: ServiceInstanceId::new("test-123"),
        endpoint: format!("http://localhost:{DEFAULT_API_PORT}"),
        capabilities: vec![ServiceCapabilityType::ServiceMesh],
        metadata: HashMap::new(),
        health: ServiceHealth::Healthy,
        priority: 100,
        protocol: ServiceProtocol::Http,
    };

    assert_eq!(descriptor.instance_id.as_str(), "test-123");
    assert_eq!(descriptor.health, ServiceHealth::Healthy);
}

#[test]
fn test_discovery_error_display() {
    let error = DiscoveryError::ServiceNotFound {
        criteria: "capability=ServiceMesh".to_string(),
    };

    let display = format!("{error}");
    assert!(display.contains("Service not found"));
}
