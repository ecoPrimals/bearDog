// BearDog Config Validation Tests
// Tests configuration validation and defaults

use beardog_core::BearDogConfig;
use beardog_types::canonical::capabilities::ServiceCapabilityType;
use beardog_types::canonical::HealthStatus;

#[test]
fn test_config_default() {
    // Test default config creation
    let config = BearDogConfig::default();

    // Default should have reasonable values
    assert!(!config.environment.is_empty());
    assert!(!config.version.is_empty());
    assert!(!config.node_id.is_empty());
}

#[test]
fn test_config_environment_default() {
    // Test default environment
    let config = BearDogConfig::default();
    assert_eq!(config.environment, "development");
}

#[test]
fn test_config_version_default() {
    // Test default version
    let config = BearDogConfig::default();
    assert_eq!(config.version, "3.0.0");
}

#[test]
fn test_config_node_id_unique() {
    // Test node_id is unique across instances
    let config1 = BearDogConfig::default();
    let config2 = BearDogConfig::default();

    assert_ne!(config1.node_id, config2.node_id);
}

#[test]
fn test_config_node_id_format() {
    // Test node_id is a valid UUID format
    let config = BearDogConfig::default();

    // UUID should have dashes
    assert!(config.node_id.contains('-'));
    // UUID string length should be 36 chars
    assert_eq!(config.node_id.len(), 36);
}

#[test]
fn test_config_custom_environment() {
    // Test custom environment setting
    let mut config = BearDogConfig::default();
    config.environment = "production".to_string();

    assert_eq!(config.environment, "production");
}

#[test]
fn test_config_custom_version() {
    // Test custom version setting
    let mut config = BearDogConfig::default();
    config.version = "4.0.0".to_string();

    assert_eq!(config.version, "4.0.0");
}

#[test]
fn test_config_clone() {
    // Test config can be cloned
    let config1 = BearDogConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.environment, config2.environment);
    assert_eq!(config1.version, config2.version);
    assert_eq!(config1.node_id, config2.node_id);
}

// Health Status Tests

#[test]
fn test_health_status_healthy() {
    // Test healthy status
    let status = HealthStatus::Healthy;

    assert_eq!(status, HealthStatus::Healthy);
}

#[test]
fn test_health_status_degraded() {
    // Test degraded status
    let status = HealthStatus::Degraded;

    assert_eq!(status, HealthStatus::Degraded);
}

#[test]
fn test_health_status_unhealthy() {
    // Test unhealthy status
    let status = HealthStatus::Unhealthy;

    assert_eq!(status, HealthStatus::Unhealthy);
}

// Service Capability Type Tests

#[test]
fn test_service_capability_key_management() {
    // Test Key Management capability type
    let cap = ServiceCapabilityType::KeyManagement;

    assert_eq!(cap, ServiceCapabilityType::KeyManagement);
}

#[test]
fn test_service_capability_hsm() {
    // Test HSM capability type
    let cap = ServiceCapabilityType::HardwareSecurityModule;

    assert_eq!(cap, ServiceCapabilityType::HardwareSecurityModule);
}

#[test]
fn test_service_capability_database() {
    // Test Database capability type
    let cap = ServiceCapabilityType::DatabaseService;

    assert_eq!(cap, ServiceCapabilityType::DatabaseService);
}

#[test]
fn test_service_capability_authentication() {
    // Test Authentication capability type
    let cap = ServiceCapabilityType::Authentication;

    assert_eq!(cap, ServiceCapabilityType::Authentication);
}

#[test]
fn test_service_capability_cloud_storage() {
    // Test Cloud Storage capability type
    let cap = ServiceCapabilityType::CloudStorage;

    assert_eq!(cap, ServiceCapabilityType::CloudStorage);
}
