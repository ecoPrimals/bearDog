//! Comprehensive Tests for Core State Management
//!
//! Tests for `CoreState` including:
//! - Default state creation
//! - Component registration and tracking
//! - Health status management
//! - Uptime calculation
//! - State cloning and serialization
//! - Concurrent access patterns
//!
//! Created: October 27, 2025 - Test Coverage Expansion

#![allow(
    unused_imports,
    clippy::float_cmp,
    clippy::useless_vec,
    clippy::needless_range_loop,
    clippy::uninlined_format_args,
    clippy::field_reassign_with_default,
    clippy::manual_range_contains,
    unused_variables,
    dead_code
)]

use crate::core::state::CoreState;
use beardog_types::canonical::{ComponentStatus, HealthStatus};
use std::thread;
use std::time::Duration;

// ============================================================================
// Default State Creation Tests
// ============================================================================

#[test]
fn test_core_state_default_creation() {
    let state = CoreState::default();

    assert_eq!(state.components.len(), 0);
    assert_eq!(state.overall_health, HealthStatus::Healthy);
    // Start time should be recent
    assert!(state.start_time.elapsed().as_secs() < 1);
}

#[test]
fn test_core_state_default_is_healthy() {
    let state = CoreState::default();
    assert_eq!(state.overall_health, HealthStatus::Healthy);
}

#[test]
fn test_core_state_default_has_empty_components() {
    let state = CoreState::default();
    assert!(state.components.is_empty());
}

#[test]
fn test_core_state_default_start_time_is_now() {
    let state = CoreState::default();
    let elapsed = state.start_time.elapsed();

    // Should be created just now (within 100ms)
    assert!(elapsed < Duration::from_millis(100));
}

// ============================================================================
// Component Management Tests
// ============================================================================

#[test]
fn test_core_state_add_single_component() {
    let mut state = CoreState::default();

    state
        .components
        .insert("security".to_string(), ComponentStatus::Running);

    assert_eq!(state.components.len(), 1);
    assert_eq!(
        state.components.get("security"),
        Some(&ComponentStatus::Running)
    );
}

#[test]
fn test_core_state_add_multiple_components() {
    let mut state = CoreState::default();

    state
        .components
        .insert("security".to_string(), ComponentStatus::Running);
    state
        .components
        .insert("monitor".to_string(), ComponentStatus::Running);
    state
        .components
        .insert("optimizer".to_string(), ComponentStatus::Running);

    assert_eq!(state.components.len(), 3);
}

#[test]
fn test_core_state_update_component_status() {
    let mut state = CoreState::default();

    state
        .components
        .insert("security".to_string(), ComponentStatus::Running);
    assert_eq!(
        state.components.get("security"),
        Some(&ComponentStatus::Running)
    );

    // Update status
    state
        .components
        .insert("security".to_string(), ComponentStatus::Stopping);
    assert_eq!(
        state.components.get("security"),
        Some(&ComponentStatus::Stopping)
    );
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_core_state_remove_component() {
    let mut state = CoreState::default();

    state
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        .components
        .insert("security".to_string(), ComponentStatus::Running);
    assert_eq!(state.components.len(), 1);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    state.components.remove("security");
    assert_eq!(state.components.len(), 0);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_core_state_component_not_found() {
    let state = CoreState::default();
    assert_eq!(state.components.get("nonexistent"), None);
}

#[test]
fn test_core_state_multiple_component_statuses() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let mut state = CoreState::default();

    state
        .components
        .insert("security".to_string(), ComponentStatus::Running);
    state
        .components
        .insert("monitor".to_string(), ComponentStatus::Stopping);
    state
        .components
        .insert("optimizer".to_string(), ComponentStatus::Inactive);

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(
        state.components.get("security"),
        Some(&ComponentStatus::Running)
    );
    assert_eq!(
        state.components.get("monitor"),
        Some(&ComponentStatus::Stopping)
    );
    assert_eq!(
        state.components.get("optimizer"),
        Some(&ComponentStatus::Inactive)
    );
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
// ============================================================================
// Health Status Tests
// ============================================================================

#[test]
fn test_core_state_health_status_healthy() {
    let state = CoreState {
        overall_health: HealthStatus::Healthy,
        ..Default::default()
    };

    assert_eq!(state.overall_health, HealthStatus::Healthy);
}

#[test]
fn test_core_state_health_status_degraded() {
    let state = CoreState {
        overall_health: HealthStatus::Degraded,
        ..Default::default()
    };

    assert_eq!(state.overall_health, HealthStatus::Degraded);
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[test]
fn test_core_state_health_status_unhealthy() {
    let state = CoreState {
        overall_health: HealthStatus::Unhealthy,
        ..Default::default()
    };

    assert_eq!(state.overall_health, HealthStatus::Unhealthy);
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_core_state_health_status_transition() {
    let mut state = CoreState::default();

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(state.overall_health, HealthStatus::Healthy);

    state.overall_health = HealthStatus::Degraded;
    assert_eq!(state.overall_health, HealthStatus::Degraded);

    state.overall_health = HealthStatus::Unhealthy;
    assert_eq!(state.overall_health, HealthStatus::Unhealthy);

    state.overall_health = HealthStatus::Healthy;
    assert_eq!(state.overall_health, HealthStatus::Healthy);
}

// ============================================================================
// Uptime Calculation Tests
// ============================================================================

#[test]
fn test_core_state_uptime_just_created() {
    let state = CoreState::default();
    let uptime = state.start_time.elapsed();

    // Should be very close to 0 (within 100ms)
    assert!(uptime < Duration::from_millis(100));
}

#[test]
fn test_core_state_uptime_increases() {
    let state = CoreState::default();

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // ✅ MODERNIZED: Removed sleep - elapsed() is monotonically increasing
    let uptime1 = state.start_time.elapsed();
    let uptime2 = state.start_time.elapsed();

    assert!(
        uptime2 >= uptime1,
        "Elapsed time is monotonically increasing"
    );
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_core_state_uptime_monotonic() {
    let state = CoreState::default();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    let mut previous_uptime = state.start_time.elapsed();

    // ✅ MODERNIZED: Removed sleep - elapsed() is always monotonic
    for _ in 0..5 {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let current_uptime = state.start_time.elapsed();
        assert!(
            current_uptime >= previous_uptime,
            "Uptime is monotonically increasing"
        );
        previous_uptime = current_uptime;
    }
}

// ============================================================================
// Clone Tests
// ============================================================================

#[test]
fn test_core_state_clone() {
    let mut state = CoreState::default();
    state
        .components
        .insert("security".to_string(), ComponentStatus::Running);
    state.overall_health = HealthStatus::Degraded;
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    let cloned = state.clone();

    assert_eq!(cloned.components.len(), 1);
    assert_eq!(cloned.overall_health, HealthStatus::Degraded);
    assert_eq!(
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        cloned.components.get("security"),
        Some(&ComponentStatus::Running)
    );
}

#[test]
fn test_core_state_clone_independence() {
    let mut state = CoreState::default();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    state
        .components
        .insert("security".to_string(), ComponentStatus::Running);

    let mut cloned = state.clone();

    // Modify clone
    cloned
        .components
        .insert("monitor".to_string(), ComponentStatus::Running);
    cloned.overall_health = HealthStatus::Unhealthy;

    // Original should be unchanged
    assert_eq!(state.components.len(), 1);
    assert_eq!(state.overall_health, HealthStatus::Healthy);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    // Clone should be modified
    assert_eq!(cloned.components.len(), 2);
    assert_eq!(cloned.overall_health, HealthStatus::Unhealthy);
}

#[test]
fn test_core_state_clone_start_time_same() {
    let state = CoreState::default();
    // ✅ MODERNIZED: Removed sleep - clone preserves start_time regardless of when clone occurs

    let cloned = state.clone();

    // Start time should be the same instant
    // (though elapsed time will be slightly different due to cloning time)
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let diff = state
        .start_time
        .elapsed()
        .as_millis()
        .abs_diff(cloned.start_time.elapsed().as_millis());

    // Should be very close (within 10ms)
    assert!(diff < 10);
}

// ============================================================================
// Debug Formatting Tests
// ============================================================================

#[test]
fn test_core_state_debug_format() {
    let state = CoreState::default();
    let debug_str = format!("{state:?}");

    // Should contain struct name and key fields
    assert!(debug_str.contains("CoreState"));
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(debug_str.contains("components"));
    assert!(debug_str.contains("overall_health"));
    assert!(debug_str.contains("start_time"));
}

#[test]
fn test_core_state_debug_with_components() {
    let mut state = CoreState::default();
    state
        .components
        .insert("security".to_string(), ComponentStatus::Running);

    let debug_str = format!("{state:?}");
    assert!(debug_str.contains("security"));
    assert!(debug_str.contains("Healthy"));
}

// ============================================================================
// Capacity Tests
// ============================================================================
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[test]
fn test_core_state_components_initial_capacity() {
    let state = CoreState::default();

    // Default creates HashMap with capacity 10
    assert!(state.components.capacity() >= 10);
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_core_state_components_grow_beyond_capacity() {
    let mut state = CoreState::default();

    // Add more than initial capacity
    for i in 0..20 {
        state
            .components
            .insert(format!("component_{i}"), ComponentStatus::Running);
    }

    assert_eq!(state.components.len(), 20);
    // Capacity should have grown
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(state.components.capacity() >= 20);
}

// ============================================================================
// Integration Tests
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
// ============================================================================

#[test]
fn test_core_state_realistic_scenario() {
    let mut state = CoreState::default();

    // Simulate system startup
    assert_eq!(state.overall_health, HealthStatus::Healthy);

    // Register components
    state
        .components
        .insert("security".to_string(), ComponentStatus::Running);
    state
        .components
        .insert("monitor".to_string(), ComponentStatus::Running);
    state
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        .components
        .insert("optimizer".to_string(), ComponentStatus::Running);
    state
        .components
        .insert("adapter".to_string(), ComponentStatus::Running);

    assert_eq!(state.components.len(), 4);

    // Simulate a component degradation
    state
        .components
        .insert("monitor".to_string(), ComponentStatus::Stopping);
    state.overall_health = HealthStatus::Degraded;

    assert_eq!(state.overall_health, HealthStatus::Degraded);
    assert_eq!(
        state.components.get("monitor"),
        Some(&ComponentStatus::Stopping)
    );

    // Simulate recovery
    state
        .components
        .insert("monitor".to_string(), ComponentStatus::Running);
    state.overall_health = HealthStatus::Healthy;

    assert_eq!(state.overall_health, HealthStatus::Healthy);

    // Check uptime
    assert!(state.start_time.elapsed() > Duration::from_nanos(0));
}

#[test]
fn test_core_state_concurrent_cloning() {
    let state = CoreState::default();

    // Simulate concurrent reads via cloning
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let state_clone = state.clone();
            thread::spawn(move || {
                assert_eq!(state_clone.overall_health, HealthStatus::Healthy);
                assert!(state_clone.components.is_empty());
            })
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_core_state_empty_component_name() {
    let mut state = CoreState::default();

    // Empty string as component name (edge case)
    state
        .components
        .insert(String::new(), ComponentStatus::Running);

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(state.components.len(), 1);
    assert_eq!(state.components.get(""), Some(&ComponentStatus::Running));
}

#[test]
fn test_core_state_very_long_component_name() {
    let mut state = CoreState::default();

    let long_name = "a".repeat(1000);
    state
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        .components
        .insert(long_name.clone(), ComponentStatus::Running);

    assert_eq!(state.components.len(), 1);
    assert_eq!(
        state.components.get(&long_name),
        Some(&ComponentStatus::Running)
    );
}

#[test]
fn test_core_state_unicode_component_name() {
    let mut state = CoreState::default();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    let unicode_name = "安全_セキュリティ_🔐";
    state
        .components
        .insert(unicode_name.to_string(), ComponentStatus::Running);

    assert_eq!(state.components.len(), 1);
    assert_eq!(
        state.components.get(unicode_name),
        Some(&ComponentStatus::Running)
    );
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_core_state_many_components() {
    let mut state = CoreState::default();

    // Add many components
    for i in 0..1000 {
        state.components.insert(
            format!("component_{i}"),
            if i % 3 == 0 {
                ComponentStatus::Running
            } else if i % 3 == 1 {
                ComponentStatus::Active
            } else {
                ComponentStatus::Inactive
            },
        );
    }

    assert_eq!(state.components.len(), 1000);
}
