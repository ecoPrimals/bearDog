//! Canonical Types Comprehensive Tests
//!
//! This module contains unit tests for canonical types, specifically HealthStatus
//! and ComponentStatus enums. Tests verify instantiation, equality, and cloning behavior.
//!
//! Coverage: HealthStatus (5 tests), ComponentStatus (7 tests)

use beardog_types::canonical::{ComponentStatus, HealthStatus};

// ============================================================================
// HealthStatus Tests
// ============================================================================

/// Tests that HealthStatus::Healthy can be instantiated
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: types
/// TEST_PRIORITY: high
#[test]
fn test_health_healthy() {
    // Given: a Healthy status
    let h = HealthStatus::Healthy;

    // Then: should equal itself
    assert_eq!(h, HealthStatus::Healthy, "Healthy status should match");
}

/// Tests that HealthStatus::Degraded can be instantiated
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: types
/// TEST_PRIORITY: high
#[test]
fn test_health_degraded() {
    // Given: a Degraded status
    let h = HealthStatus::Degraded;

    // Then: should equal itself
    assert_eq!(h, HealthStatus::Degraded, "Degraded status should match");
}

/// Tests that HealthStatus::Unhealthy can be instantiated
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: types
/// TEST_PRIORITY: high
#[test]
fn test_health_unhealthy() {
    // Given: an Unhealthy status
    let h = HealthStatus::Unhealthy;

    // Then: should equal itself
    assert_eq!(h, HealthStatus::Unhealthy, "Unhealthy status should match");
}

/// Tests that different HealthStatus values are not equal
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: types
/// TEST_PRIORITY: normal
#[test]
fn test_health_ne() {
    // Then: different health statuses should not be equal
    assert_ne!(
        HealthStatus::Healthy,
        HealthStatus::Degraded,
        "Different statuses should not match"
    );
}

/// Tests that HealthStatus can be cloned/copied
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: types
/// TEST_PRIORITY: normal
#[test]
fn test_health_clone() {
    // Given: a Healthy status
    let h = HealthStatus::Healthy;

    // When: copying it
    let c = h;

    // Then: should be equal
    assert_eq!(h, c, "Cloned status should match original");
}

// ============================================================================
// ComponentStatus Tests
// ============================================================================

/// Tests that ComponentStatus::Starting can be instantiated
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: types
/// TEST_PRIORITY: high
#[test]
fn test_component_starting() {
    // Given: a Starting status
    let c = ComponentStatus::Starting;

    // Then: should equal itself
    assert_eq!(c, ComponentStatus::Starting, "Starting status should match");
}

/// Tests that ComponentStatus::Running can be instantiated
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: types
/// TEST_PRIORITY: high
#[test]
fn test_component_running() {
    // Given: a Running status
    let c = ComponentStatus::Running;

    // Then: should equal itself
    assert_eq!(c, ComponentStatus::Running, "Running status should match");
}

/// Tests that ComponentStatus::Stopping can be instantiated
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: types
/// TEST_PRIORITY: high
#[test]
fn test_component_stopping() {
    // Given: a Stopping status
    let c = ComponentStatus::Stopping;

    // Then: should equal itself
    assert_eq!(c, ComponentStatus::Stopping, "Stopping status should match");
}

/// Tests that ComponentStatus::Active can be instantiated
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: types
/// TEST_PRIORITY: high
#[test]
fn test_component_active() {
    // Given: an Active status
    let c = ComponentStatus::Active;

    // Then: should equal itself
    assert_eq!(c, ComponentStatus::Active, "Active status should match");
}

/// Tests that ComponentStatus::Inactive can be instantiated
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: types
/// TEST_PRIORITY: high
#[test]
fn test_component_inactive() {
    // Given: an Inactive status
    let c = ComponentStatus::Inactive;

    // Then: should equal itself
    assert_eq!(c, ComponentStatus::Inactive, "Inactive status should match");
}

/// Tests that different ComponentStatus values are not equal
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: types
/// TEST_PRIORITY: normal
#[test]
fn test_component_ne() {
    // Then: different component statuses should not be equal
    assert_ne!(
        ComponentStatus::Running,
        ComponentStatus::Stopping,
        "Different statuses should not match"
    );
}

/// Tests that ComponentStatus can be cloned
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: types
/// TEST_PRIORITY: normal
#[test]
fn test_component_clone() {
    // Given: a Running status
    let c = ComponentStatus::Running;

    // When: cloning it
    let cloned = c.clone();

    // Then: should be equal
    assert_eq!(c, cloned, "Cloned status should match original");
}
