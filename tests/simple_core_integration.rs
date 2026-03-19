// Simple Core Integration Tests
// Migrated from tests_NEEDS_FIXING_BACKUP/simple_core_tests.rs
// Date: October 10, 2025

use beardog_core::BearDogConfig;
use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;

#[test]
fn test_core_config_creation() {
    // Test basic config creation
    let config = BearDogConfig::default();

    assert!(!config.node_id.is_empty(), "Node ID should be generated");
    assert!(!config.environment.is_empty(), "Environment should be set");
    assert!(!config.version.is_empty(), "Version should be set");
}

#[test]
fn test_core_config_node_id_uniqueness() {
    // Test that each config gets a unique node ID
    let config1 = BearDogConfig::default();
    let config2 = BearDogConfig::default();

    assert_ne!(
        config1.node_id, config2.node_id,
        "Each config should have a unique node ID"
    );
}

#[test]
fn test_core_config_clone() {
    // Test config cloning works correctly
    let config1 = BearDogConfig::default();
    let config2 = config1.clone();

    assert_eq!(
        config1.node_id, config2.node_id,
        "Cloned config should have same node ID"
    );
    assert_eq!(
        config1.environment, config2.environment,
        "Cloned config should have same environment"
    );
    assert_eq!(
        config1.version, config2.version,
        "Cloned config should have same version"
    );
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_health_status_variants() {
    // Test all health status variants exist and can be created
    let healthy = HealthStatus::Healthy;
    let degraded = HealthStatus::Degraded;
    let unhealthy = HealthStatus::Unhealthy;
    let unknown = HealthStatus::Unknown;
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    assert_eq!(healthy, HealthStatus::Healthy);
    assert_eq!(degraded, HealthStatus::Degraded);
    assert_eq!(unhealthy, HealthStatus::Unhealthy);
    assert_eq!(unknown, HealthStatus::Unknown);
}

#[test]
fn test_health_status_equality() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Test health status equality comparison
    let status1 = HealthStatus::Healthy;
    let status2 = HealthStatus::Healthy;
    let status3 = HealthStatus::Degraded;

    assert_eq!(status1, status2, "Same health statuses should be equal");
    assert_ne!(
        status1, status3,
        "Different health statuses should not be equal"
    );
}

#[test]
fn test_config_environment_values() {
    // Test different environment configurations
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    // Modern idiomatic pattern: Use struct initialization instead of mutation
    let config = BearDogConfig {
        environment: "development".to_string(),
        ..Default::default()
    };
    assert_eq!(config.environment, "development");

    let config = BearDogConfig {
        environment: "staging".to_string(),
        ..Default::default()
    };
    assert_eq!(config.environment, "staging");

    let config = BearDogConfig {
        environment: "production".to_string(),
        ..Default::default()
    };
    assert_eq!(config.environment, "production");
}

#[test]
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_config_version_format() {
    // Test version string format
    let config = BearDogConfig::default();

    assert!(!config.version.is_empty(), "Version should not be empty");
    // Version should contain digits
    assert!(
        config.version.chars().any(char::is_numeric),
        "Version should contain numbers"
    );
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[test]
fn test_error_creation() {
    // Test BearDogError creation
    let error = BearDogError::validation("Test validation error");

    // Error should be created successfully
    assert!(format!("{error:?}").contains("validation"));
}

#[test]
fn test_error_system_creation() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Test system error creation
    let error = BearDogError::system("System error".to_string());

    // Error should be created successfully
    assert!(format!("{error:?}").contains("System"));
}

#[test]
fn test_config_serialization_support() {
    // Test that config supports clone (prerequisite for serialization)
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: important
    let config1 = BearDogConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.node_id, config2.node_id);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: important
#[test]
fn test_health_status_clone() {
    // Test health status can be cloned
    let status1 = HealthStatus::Healthy;
    let status2 = status1;

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(status1, status2);
}

#[test]
fn test_config_custom_values() {
    // Test setting custom config values
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let mut config = BearDogConfig::default();

    let custom_node_id = "custom-node-123".to_string();
    config.node_id = custom_node_id.clone();

    assert_eq!(config.node_id, custom_node_id);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_multiple_configs_independence() {
    // Test that multiple configs are independent
    let mut config1 = BearDogConfig::default();
    let mut config2 = BearDogConfig::default();

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    config1.environment = "dev".to_string();
    config2.environment = "prod".to_string();

    assert_eq!(config1.environment, "dev");
    assert_eq!(config2.environment, "prod");
}

#[test]
fn test_health_status_debug() {
    // Test health status debug formatting
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let status = HealthStatus::Healthy;
    let debug_str = format!("{status:?}");

    assert!(debug_str.contains("Healthy"));
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: important
#[test]
fn test_error_context_preservation() {
    // Test that errors preserve context
    let error = BearDogError::validation("Field validation failed");
    let error_str = format!("{error:?}");

    assert!(error_str.contains("validation") || error_str.contains("Field"));
}
