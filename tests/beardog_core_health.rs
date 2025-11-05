//! BearDog Core Health Check Tests
//!
//! This module contains unit tests for core health checking functionality,
//! including config creation, health status types, and basic error handling.
//!
//! Coverage: Config tests (2 tests), Health status (5 tests), Error types (13 tests)

use beardog_core::BearDogConfig;
use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;

// ============================================================================
// Configuration Tests
// ============================================================================

/// Tests that default configuration can be created
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: high
#[tokio::test]
async fn test_config_default_creation() {
    // When: creating default config
    let config = BearDogConfig::default();

    // Then: should succeed without error
    drop(config);
}

/// Tests that configuration can be cloned
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: normal
#[tokio::test]
async fn test_config_clone() {
    // Given: a default config
    let config1 = BearDogConfig::default();

    // When: cloning it
    let config2 = config1.clone();

    // Then: both configs should be valid
    drop(config1);
    drop(config2);
}

// ============================================================================
// Health Status Tests
// ============================================================================

/// Tests that Healthy status can be created and compared
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: high
#[tokio::test]
async fn test_health_status_healthy() {
    // Given: a Healthy status
    let status = HealthStatus::Healthy;

    // Then: should equal itself
    assert_eq!(status, HealthStatus::Healthy, "Healthy status should match");
}

/// Tests that Degraded status can be created and compared
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: high
#[tokio::test]
async fn test_health_status_degraded() {
    // Given: a Degraded status
    let status = HealthStatus::Degraded;

    // Then: should equal itself
    assert_eq!(
        status,
        HealthStatus::Degraded,
        "Degraded status should match"
    );
}

/// Tests that Unhealthy status can be created and compared
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: high
#[tokio::test]
async fn test_health_status_unhealthy() {
    // Given: an Unhealthy status
    let status = HealthStatus::Unhealthy;

    // Then: should equal itself
    assert_eq!(
        status,
        HealthStatus::Unhealthy,
        "Unhealthy status should match"
    );
}

/// Tests that different health statuses are not equal
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: normal
#[tokio::test]
async fn test_health_status_comparison() {
    // Then: different statuses should not be equal
    assert_ne!(
        HealthStatus::Healthy,
        HealthStatus::Degraded,
        "Healthy != Degraded"
    );
    assert_ne!(
        HealthStatus::Degraded,
        HealthStatus::Unhealthy,
        "Degraded != Unhealthy"
    );

    // Then: same status should be equal
    assert_eq!(
        HealthStatus::Healthy,
        HealthStatus::Healthy,
        "Healthy == Healthy"
    );
}

/// Tests that health status can be cloned/copied
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: normal
#[tokio::test]
async fn test_health_status_clone() {
    // Given: a Healthy status
    let status = HealthStatus::Healthy;

    // When: copying it
    let cloned = status;

    // Then: should be equal
    assert_eq!(status, cloned, "Cloned status should match original");
}

// ============================================================================
// Synchronous Error Type Tests
// ============================================================================

/// Tests system error in sync context
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: high
#[test]
fn test_error_system_sync() {
    // Given: a system error
    let error = BearDogError::system("sync test".to_string());

    // Then: should contain message
    assert!(
        format!("{:?}", error).contains("sync test"),
        "System error should contain message"
    );
}

/// Tests validation error in sync context
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: high
#[test]
fn test_error_validation_sync() {
    // Given: a validation error
    let error = BearDogError::validation("validation test");

    // Then: should contain message
    assert!(
        format!("{:?}", error).contains("validation test"),
        "Validation error should contain message"
    );
}

/// Tests security error in sync context
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: critical
#[test]
fn test_error_security_sync() {
    // Given: a security error
    let error = BearDogError::security("security test".to_string());

    // Then: should contain message
    assert!(
        format!("{:?}", error).contains("security test"),
        "Security error should contain message"
    );
}

/// Tests network error in sync context
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: high
#[test]
fn test_error_network_sync() {
    // Given: a network error
    let error = BearDogError::network("network test".to_string());

    // Then: should contain message
    assert!(
        format!("{:?}", error).contains("network test"),
        "Network error should contain message"
    );
}

/// Tests configuration error in sync context
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: high
#[test]
fn test_error_configuration_sync() {
    // Given: a configuration error
    let error = BearDogError::configuration("config test");

    // Then: should contain message
    assert!(
        format!("{:?}", error).contains("config test"),
        "Configuration error should contain message"
    );
}

/// Tests not_found error in sync context
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: high
#[test]
fn test_error_not_found_sync() {
    // Given: a not found error
    let error = BearDogError::not_found("not found test".to_string());

    // Then: should contain message
    assert!(
        format!("{:?}", error).contains("not found"),
        "NotFound error should contain message"
    );
}

/// Tests unauthorized error in sync context
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: critical
#[test]
fn test_error_unauthorized_sync() {
    // Given: an unauthorized error
    let error = BearDogError::unauthorized("unauthorized test".to_string());

    // Then: should contain message
    assert!(
        format!("{:?}", error).contains("unauthorized"),
        "Unauthorized error should contain message"
    );
}

/// Tests invalid_input error in sync context
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: high
#[test]
fn test_error_invalid_input_sync() {
    // Given: an invalid input error
    let error = BearDogError::invalid_input("invalid input test");

    // Then: should contain message
    assert!(
        format!("{:?}", error).contains("invalid input"),
        "InvalidInput error should contain message"
    );
}

/// Tests unavailable error in sync context
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: high
#[test]
fn test_error_unavailable_sync() {
    // Given: an unavailable error
    let error = BearDogError::unavailable("unavailable test".to_string());

    // Then: should contain message
    assert!(
        format!("{:?}", error).contains("unavailable"),
        "Unavailable error should contain message"
    );
}

/// Tests internal error in sync context
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: high
#[test]
fn test_error_internal_sync() {
    // Given: an internal error
    let error = BearDogError::internal("internal test".to_string());

    // Then: should contain message
    assert!(
        format!("{:?}", error).contains("internal"),
        "Internal error should contain message"
    );
}

/// Tests business error in sync context
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: normal
#[test]
fn test_error_business_sync() {
    // Given: a business error
    let error = BearDogError::business("business test".to_string());

    // Then: should contain message
    assert!(
        format!("{:?}", error).contains("business"),
        "Business error should contain message"
    );
}

/// Tests API error in sync context
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: high
#[test]
fn test_error_api_sync() {
    // Given: an API error
    let error = BearDogError::api("api test".to_string());

    // Then: should contain message
    assert!(
        format!("{:?}", error).contains("api"),
        "API error should contain message"
    );
}

/// Tests workflow error in sync context
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: normal
#[test]
fn test_error_workflow_sync() {
    // Given: a workflow error
    let error = BearDogError::workflow("workflow test".to_string());

    // Then: should contain message
    assert!(
        format!("{:?}", error).contains("workflow"),
        "Workflow error should contain message"
    );
}
