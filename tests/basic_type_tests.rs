//! Basic type tests for canonical types
//!
//! These tests validate fundamental BearDog types including health status
//! and component status enums.

use beardog_types::canonical::{ComponentStatus, HealthStatus};

/// Tests that HealthStatus::Healthy can be instantiated and compared
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: high
#[test]
fn test_health_status_healthy() {
    let status = HealthStatus::Healthy;
    assert_eq!(status, HealthStatus::Healthy);
}

/// Tests that HealthStatus::Degraded can be instantiated and compared
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: high
#[test]
fn test_health_status_degraded() {
    let status = HealthStatus::Degraded;
    assert_eq!(status, HealthStatus::Degraded);
}

/// Tests that HealthStatus::Unhealthy can be instantiated and compared
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: high
#[test]
fn test_health_status_unhealthy() {
    let status = HealthStatus::Unhealthy;
    assert_eq!(status, HealthStatus::Unhealthy);
}

/// Tests that identical HealthStatus values are equal
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: medium
#[test]
fn test_health_status_equality() {
    let s1 = HealthStatus::Healthy;
    let s2 = HealthStatus::Healthy;
    assert_eq!(s1, s2);
}

/// Tests that different HealthStatus values are not equal
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: medium
#[test]
fn test_health_status_inequality() {
    let s1 = HealthStatus::Healthy;
    let s2 = HealthStatus::Degraded;
    assert_ne!(s1, s2);
}

/// Tests that HealthStatus implements Copy trait correctly
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: medium
#[test]
fn test_health_status_clone() {
    let status = HealthStatus::Healthy;
    let cloned = status;
    assert_eq!(status, cloned);
}

/// Tests that ComponentStatus::Starting can be instantiated
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: high
#[test]
fn test_component_status_starting() {
    let status = ComponentStatus::Starting;
    assert_eq!(status, ComponentStatus::Starting);
}

/// Tests that ComponentStatus::Running can be instantiated
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: high
#[test]
fn test_component_status_running() {
    let status = ComponentStatus::Running;
    assert_eq!(status, ComponentStatus::Running);
}

/// Tests that ComponentStatus::Stopping can be instantiated
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: high
#[test]
fn test_component_status_stopping() {
    let status = ComponentStatus::Stopping;
    assert_eq!(status, ComponentStatus::Stopping);
}

/// Tests that ComponentStatus::Active can be instantiated
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: high
#[test]
fn test_component_status_active() {
    let status = ComponentStatus::Active;
    assert_eq!(status, ComponentStatus::Active);
}

/// Tests that ComponentStatus::Inactive can be instantiated
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: high
#[test]
fn test_component_status_inactive() {
    let status = ComponentStatus::Inactive;
    assert_eq!(status, ComponentStatus::Inactive);
}

/// Tests that identical ComponentStatus values are equal
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: medium
#[test]
fn test_component_status_equality() {
    let s1 = ComponentStatus::Running;
    let s2 = ComponentStatus::Running;
    assert_eq!(s1, s2);
}

/// Tests that different ComponentStatus values are not equal
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: medium
#[test]
fn test_component_status_inequality() {
    let s1 = ComponentStatus::Running;
    let s2 = ComponentStatus::Stopping;
    assert_ne!(s1, s2);
}

/// Tests that ComponentStatus implements Clone trait correctly
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: medium
#[test]
fn test_component_status_clone() {
    let status = ComponentStatus::Running;
    let cloned = status.clone();
    assert_eq!(status, cloned);
}

/// Tests that HealthStatus implements Debug trait
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: low
#[test]
fn test_health_status_debug() {
    let status = HealthStatus::Healthy;
    let debug_str = format!("{:?}", status);
    assert!(!debug_str.is_empty());
}

/// Tests that ComponentStatus implements Debug trait
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: low
#[test]
fn test_component_status_debug() {
    let status = ComponentStatus::Running;
    let debug_str = format!("{:?}", status);
    assert!(!debug_str.is_empty());
}

/// Tests that HealthStatus can be stored in collections
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: medium
#[test]
fn test_health_status_in_vec() {
    let statuses = [
        HealthStatus::Healthy,
        HealthStatus::Degraded,
        HealthStatus::Unhealthy,
    ];
    assert_eq!(statuses.len(), 3);
}

/// Tests that ComponentStatus can be stored in collections
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: medium
#[test]
fn test_component_status_in_vec() {
    let statuses = [
        ComponentStatus::Starting,
        ComponentStatus::Running,
        ComponentStatus::Stopping,
    ];
    assert_eq!(statuses.len(), 3);
}

/// Tests that HealthStatus can be used in Option types
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: medium
#[test]
fn test_health_status_in_option() {
    let maybe_status: Option<HealthStatus> = Some(HealthStatus::Healthy);
    assert!(maybe_status.is_some());
}

/// Tests that ComponentStatus can be used in Option types
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: medium
#[test]
fn test_component_status_in_option() {
    let maybe_status: Option<ComponentStatus> = Some(ComponentStatus::Running);
    assert!(maybe_status.is_some());
}

/// Tests component status lifecycle transitions
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: high
#[test]
fn test_status_transitions() {
    // Given: a component starting
    let mut status = ComponentStatus::Starting;

    // When: transitioning to running
    status = ComponentStatus::Running;
    assert_eq!(status, ComponentStatus::Running);

    // Then: can transition to stopping
    status = ComponentStatus::Stopping;
    assert_eq!(status, ComponentStatus::Stopping);
}

/// Tests health status degradation sequence
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: core
/// TEST_PRIORITY: high
#[test]
fn test_health_degradation() {
    // Given: a healthy system
    let mut health = HealthStatus::Healthy;

    // When: health degrades
    health = HealthStatus::Degraded;
    assert_eq!(health, HealthStatus::Degraded);

    // Then: can further degrade to unhealthy
    health = HealthStatus::Unhealthy;
    assert_eq!(health, HealthStatus::Unhealthy);
}
