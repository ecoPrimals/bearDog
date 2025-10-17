// Basic type tests for canonical types

use beardog_types::canonical::{ComponentStatus, HealthStatus};

#[test]
fn test_health_status_healthy() {
    let status = HealthStatus::Healthy;
    assert_eq!(status, HealthStatus::Healthy);
}

#[test]
fn test_health_status_degraded() {
    let status = HealthStatus::Degraded;
    assert_eq!(status, HealthStatus::Degraded);
}

#[test]
fn test_health_status_unhealthy() {
    let status = HealthStatus::Unhealthy;
    assert_eq!(status, HealthStatus::Unhealthy);
}

#[test]
fn test_health_status_equality() {
    let s1 = HealthStatus::Healthy;
    let s2 = HealthStatus::Healthy;
    assert_eq!(s1, s2);
}

#[test]
fn test_health_status_inequality() {
    let s1 = HealthStatus::Healthy;
    let s2 = HealthStatus::Degraded;
    assert_ne!(s1, s2);
}

#[test]
fn test_health_status_clone() {
    let status = HealthStatus::Healthy;
    let cloned = status;
    assert_eq!(status, cloned);
}

#[test]
fn test_component_status_starting() {
    let status = ComponentStatus::Starting;
    assert_eq!(status, ComponentStatus::Starting);
}

#[test]
fn test_component_status_running() {
    let status = ComponentStatus::Running;
    assert_eq!(status, ComponentStatus::Running);
}

#[test]
fn test_component_status_stopping() {
    let status = ComponentStatus::Stopping;
    assert_eq!(status, ComponentStatus::Stopping);
}

#[test]
fn test_component_status_active() {
    let status = ComponentStatus::Active;
    assert_eq!(status, ComponentStatus::Active);
}

#[test]
fn test_component_status_inactive() {
    let status = ComponentStatus::Inactive;
    assert_eq!(status, ComponentStatus::Inactive);
}

#[test]
fn test_component_status_equality() {
    let s1 = ComponentStatus::Running;
    let s2 = ComponentStatus::Running;
    assert_eq!(s1, s2);
}

#[test]
fn test_component_status_inequality() {
    let s1 = ComponentStatus::Running;
    let s2 = ComponentStatus::Stopping;
    assert_ne!(s1, s2);
}

#[test]
fn test_component_status_clone() {
    let status = ComponentStatus::Running;
    let cloned = status.clone();
    assert_eq!(status, cloned);
}

#[test]
fn test_health_status_debug() {
    let status = HealthStatus::Healthy;
    let debug_str = format!("{:?}", status);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_component_status_debug() {
    let status = ComponentStatus::Running;
    let debug_str = format!("{:?}", status);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_health_status_in_vec() {
    let statuses = [
        HealthStatus::Healthy,
        HealthStatus::Degraded,
        HealthStatus::Unhealthy,
    ];
    assert_eq!(statuses.len(), 3);
}

#[test]
fn test_component_status_in_vec() {
    let statuses = [
        ComponentStatus::Starting,
        ComponentStatus::Running,
        ComponentStatus::Stopping,
    ];
    assert_eq!(statuses.len(), 3);
}

#[test]
fn test_health_status_in_option() {
    let maybe_status: Option<HealthStatus> = Some(HealthStatus::Healthy);
    assert!(maybe_status.is_some());
}

#[test]
fn test_component_status_in_option() {
    let maybe_status: Option<ComponentStatus> = Some(ComponentStatus::Running);
    assert!(maybe_status.is_some());
}

#[test]
fn test_status_transitions() {
    let mut status = ComponentStatus::Starting;
    status = ComponentStatus::Running;
    assert_eq!(status, ComponentStatus::Running);

    status = ComponentStatus::Stopping;
    assert_eq!(status, ComponentStatus::Stopping);
}

#[test]
fn test_health_degradation() {
    let mut health = HealthStatus::Healthy;
    health = HealthStatus::Degraded;
    assert_eq!(health, HealthStatus::Degraded);

    health = HealthStatus::Unhealthy;
    assert_eq!(health, HealthStatus::Unhealthy);
}
