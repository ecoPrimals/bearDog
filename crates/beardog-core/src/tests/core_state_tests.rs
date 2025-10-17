//! Core State Management Tests
//!
//! Tests for `BearDog` system state tracking and component management

use crate::core::state::CoreState;
use beardog_types::canonical::{ComponentStatus, HealthStatus};

#[test]
fn test_core_state_default() {
    let state = CoreState::default();

    assert_eq!(state.overall_health, HealthStatus::Healthy);
    assert!(state.components.is_empty());
}

#[test]
fn test_core_state_add_component() {
    let mut state = CoreState::default();

    let component = ComponentStatus::Running;

    state
        .components
        .insert("test-component".to_string(), component);

    assert_eq!(state.components.len(), 1);
    assert!(state.components.contains_key("test-component"));
}

#[test]
fn test_core_state_update_health() {
    let mut state = CoreState::default();

    state.overall_health = HealthStatus::Degraded;
    assert_eq!(state.overall_health, HealthStatus::Degraded);

    state.overall_health = HealthStatus::Unhealthy;
    assert_eq!(state.overall_health, HealthStatus::Unhealthy);
}

#[test]
fn test_core_state_multiple_components() {
    let mut state = CoreState::default();

    let component1 = ComponentStatus::Running;
    let component2 = ComponentStatus::Active;

    state
        .components
        .insert("component1".to_string(), component1);
    state
        .components
        .insert("component2".to_string(), component2);

    assert_eq!(state.components.len(), 2);
}

#[test]
fn test_core_state_uptime_tracking() {
    let state = CoreState::default();

    // Verify start_time is set to now (within reasonable margin)
    let now = std::time::Instant::now();
    let diff = now - state.start_time;

    // Should be within 1 second
    assert!(diff.as_secs() < 1);
}

#[test]
fn test_core_state_remove_component() {
    let mut state = CoreState::default();

    let component = ComponentStatus::Running;

    state
        .components
        .insert("temp-component".to_string(), component);
    assert_eq!(state.components.len(), 1);

    state.components.remove("temp-component");
    assert_eq!(state.components.len(), 0);
}

#[test]
fn test_core_state_clone() {
    let mut state = CoreState::default();
    state.overall_health = HealthStatus::Degraded;

    let cloned = state.clone();

    assert_eq!(state.overall_health, cloned.overall_health);
    assert_eq!(state.components.len(), cloned.components.len());
}

#[test]
fn test_core_state_component_statuses() {
    let mut state = CoreState::default();

    state
        .components
        .insert("starting".to_string(), ComponentStatus::Starting);
    state
        .components
        .insert("running".to_string(), ComponentStatus::Running);
    state
        .components
        .insert("active".to_string(), ComponentStatus::Active);

    assert_eq!(state.components.len(), 3);
    assert!(matches!(
        state.components.get("running"),
        Some(ComponentStatus::Running)
    ));
}

#[test]
fn test_core_state_initial_capacity() {
    let state = CoreState::default();

    // Default capacity should be 10 (as per implementation)
    assert!(state.components.capacity() >= 10);
}

#[test]
fn test_core_state_component_variants() {
    let mut state = CoreState::default();

    // Test all ComponentStatus variants
    state
        .components
        .insert("starting".to_string(), ComponentStatus::Starting);
    state
        .components
        .insert("running".to_string(), ComponentStatus::Running);
    state
        .components
        .insert("stopping".to_string(), ComponentStatus::Stopping);
    state
        .components
        .insert("active".to_string(), ComponentStatus::Active);
    state
        .components
        .insert("inactive".to_string(), ComponentStatus::Inactive);
    state
        .components
        .insert("failed".to_string(), ComponentStatus::Failed);
    state
        .components
        .insert("maintenance".to_string(), ComponentStatus::Maintenance);

    assert_eq!(state.components.len(), 7);
}
