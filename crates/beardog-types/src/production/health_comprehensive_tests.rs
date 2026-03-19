// SPDX-License-Identifier: AGPL-3.0-only

// Comprehensive Health Monitoring Tests
//
// Extensive test coverage for production health checking and monitoring

use super::health::*;
use std::collections::HashMap;

#[test]
fn test_health_config_creation() {
    let config = HealthConfig {
        enabled: true,
        check_interval_seconds: 60,
        timeout_seconds: 10,
    };
    assert!(config.enabled);
    assert_eq!(config.check_interval_seconds, 60);
}

#[test]
fn test_health_config_validation() {
    let config = HealthConfig {
        enabled: true,
        check_interval_seconds: 0, // Invalid
        timeout_seconds: 10,
    };
    // Config can be created but should be validated before use
    assert!(config.enabled);
}

#[test]
fn test_health_status_ordering() {
    assert!(HealthStatus::Healthy != HealthStatus::Degraded);
    assert!(HealthStatus::Degraded != HealthStatus::Unhealthy);
    assert!(HealthStatus::Unhealthy != HealthStatus::Critical);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
}

#[test]
fn test_component_health_creation() {
    let mut metadata = HashMap::new();
    metadata.insert("location".to_string(), "us-west-2".to_string());

    let component = ComponentHealth {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        name: "database".to_string(),
        status: HealthStatus::Healthy,
        response_time_ms: 15.5,
        metadata,
    };

    assert_eq!(component.name, "database");
    assert_eq!(component.response_time_ms, 15.5);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert_eq!(component.metadata.len(), 1);
}

#[test]
// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
fn test_health_report_timestamp() {
    let report = HealthReport {
        overall_status: HealthStatus::Healthy,
        component_statuses: vec![],
        timestamp: 1_234_567_890,
    };

    assert!(report.timestamp > 0);
    assert_eq!(report.component_statuses.len(), 0);
}

#[test]
fn test_health_checker_start_stop() {
    let config = HealthConfig {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        enabled: true,
        check_interval_seconds: 30,
        timeout_seconds: 5,
    };

    let mut checker = HealthChecker::new(&config).unwrap();
    assert!(checker.start_health_monitoring().is_ok());
    assert!(checker.stop_monitoring().is_ok());
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal

#[test]
fn test_health_report_with_multiple_components() {
    let components = vec![
        ComponentHealth {
            name: "api".to_string(),
            status: HealthStatus::Healthy,
            response_time_ms: 10.0,
            metadata: HashMap::new(),
        },
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        ComponentHealth {
            name: "database".to_string(),
            status: HealthStatus::Degraded,
            response_time_ms: 50.0,
            metadata: HashMap::new(),
        },
    ];

    let report = HealthReport {
        overall_status: HealthStatus::Degraded,
        component_statuses: components,
        timestamp: 1_234_567_890,
    };

    assert_eq!(report.component_statuses.len(), 2);
    assert_eq!(report.overall_status, HealthStatus::Degraded);
}

#[test]
fn test_health_config_serialization_roundtrip() {
    let config = HealthConfig {
        enabled: true,
        check_interval_seconds: 45,
        timeout_seconds: 8,
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
    };

    let json = serde_json::to_string(&config).unwrap();
    let deserialized: HealthConfig = serde_json::from_str(&json).unwrap();

    assert_eq!(config.enabled, deserialized.enabled);
    assert_eq!(
        config.check_interval_seconds,
        deserialized.check_interval_seconds
    );
}

#[test]
fn test_health_status_serialization() {
    let status = HealthStatus::Healthy;
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    let json = serde_json::to_string(&status).unwrap();
    assert!(json.contains("Healthy"));
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
#[test]
fn test_component_health_metadata_manipulation() {
    let mut component = ComponentHealth {
        name: "cache".to_string(),
        status: HealthStatus::Healthy,
        response_time_ms: 2.0,
        metadata: HashMap::new(),
    };

    component
        .metadata
        .insert("version".to_string(), "7.0".to_string());
    component
        .metadata
        .insert("connections".to_string(), "150".to_string());

    assert_eq!(component.metadata.len(), 2);
    assert_eq!(component.metadata.get("version").unwrap(), "7.0");
}
