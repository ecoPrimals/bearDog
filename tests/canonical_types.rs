// Canonical types comprehensive tests

use beardog_types::canonical::{ComponentStatus, HealthStatus};

// HealthStatus tests
#[test]
fn test_health_healthy() {
    let h = HealthStatus::Healthy;
    assert_eq!(h, HealthStatus::Healthy);
}

#[test]
fn test_health_degraded() {
    let h = HealthStatus::Degraded;
    assert_eq!(h, HealthStatus::Degraded);
}

#[test]
fn test_health_unhealthy() {
    let h = HealthStatus::Unhealthy;
    assert_eq!(h, HealthStatus::Unhealthy);
}

#[test]
fn test_health_ne() {
    assert_ne!(HealthStatus::Healthy, HealthStatus::Degraded);
}

#[test]
fn test_health_clone() {
    let h = HealthStatus::Healthy;
    let c = h;
    assert_eq!(h, c);
}

// ComponentStatus tests
#[test]
fn test_component_starting() {
    let c = ComponentStatus::Starting;
    assert_eq!(c, ComponentStatus::Starting);
}

#[test]
fn test_component_running() {
    let c = ComponentStatus::Running;
    assert_eq!(c, ComponentStatus::Running);
}

#[test]
fn test_component_stopping() {
    let c = ComponentStatus::Stopping;
    assert_eq!(c, ComponentStatus::Stopping);
}

#[test]
fn test_component_active() {
    let c = ComponentStatus::Active;
    assert_eq!(c, ComponentStatus::Active);
}

#[test]
fn test_component_inactive() {
    let c = ComponentStatus::Inactive;
    assert_eq!(c, ComponentStatus::Inactive);
}

#[test]
fn test_component_ne() {
    assert_ne!(ComponentStatus::Running, ComponentStatus::Stopping);
}

#[test]
fn test_component_clone() {
    let c = ComponentStatus::Running;
    let cloned = c.clone();
    assert_eq!(c, cloned);
}
