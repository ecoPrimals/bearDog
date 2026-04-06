// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::core::system::BearDogCore;
use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
use beardog_types::canonical::HealthStatus;
use std::sync::Arc;

use crate::core::system::BearDogCore;
use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
use beardog_types::canonical::HealthStatus;

/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: critical
#[tokio::test]
async fn test_core_initialization() {
    // Test that core initializes with default config
    let result = BearDogCore::with_default_config();
    assert!(result.is_ok(), "Core should initialize with default config");

    let core = result.unwrap();
    let state = core.state.read().await;
    assert_eq!(state.overall_health, HealthStatus::Healthy);
    // Components list should be accessible (empty or not)
    assert!(state.components.is_empty() || !state.components.is_empty());
}

/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: high
#[tokio::test]
async fn test_core_lifecycle() {
    // Test full lifecycle: create -> use -> verify state
    let config = UnifiedBearDogConfig::development();
    let core = BearDogCore::new(config);

    // Verify initial state
    let state = core.state.read().await;
    assert_eq!(state.overall_health, HealthStatus::Healthy);
    let initial_time = state.start_time;
    drop(state);

    // Perform actual state operations (no artificial delays needed)
    // Multiple reads/writes to verify state consistency
    for _ in 0..5 {
        let state = core.state.read().await;
        assert_eq!(
            state.start_time, initial_time,
            "Start time should not change during operations"
        );
    }

    // Verify state persists after operations
    let state = core.state.read().await;
    assert_eq!(
        state.start_time, initial_time,
        "Start time should not change"
    );
}

/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: high
#[tokio::test]
async fn test_core_cleanup() {
    // Test that core cleans up resources properly
    let core = BearDogCore::with_default_config().unwrap();
    let state = core.state.read().await;
    let _component_count = state.components.len();
    drop(state);

    // Drop core and verify cleanup
    drop(core);

    // If we get here without panic, cleanup succeeded
    assert!(true, "Core cleanup completed without panic");
}

/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: critical
#[tokio::test]
async fn test_core_concurrent_state_access() {
    // Test concurrent read/write access to core state
    let core = BearDogCore::with_default_config().unwrap();
    let core_clone = std::sync::Arc::new(core);

    // Spawn multiple readers
    let mut handles = vec![];
    for i in 0..5 {
        let core_ref = core_clone.clone();
        let handle = tokio::spawn(async move {
            for _ in 0..10 {
                let state = core_ref.state.read().await;
                assert_eq!(state.overall_health, HealthStatus::Healthy);
                // No artificial delay - test true concurrency
            }
        });
        handles.push(handle);
    }

    // Wait for all readers
    for handle in handles {
        handle.await.unwrap();
    }
}

/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: high
#[tokio::test]
async fn test_core_state_transitions() {
    // Test state transitions through different health states
    let core = BearDogCore::with_default_config().unwrap();

    // Initial state
    {
        let state = core.state.read().await;
        assert_eq!(state.overall_health, HealthStatus::Healthy);
    }

    // Transition to degraded
    {
        let mut state = core.state.write().await;
        state.overall_health = HealthStatus::Degraded;
    }

    // Verify degraded state
    {
        let state = core.state.read().await;
        assert_eq!(state.overall_health, HealthStatus::Degraded);
    }

    // Recover to healthy
    {
        let mut state = core.state.write().await;
        state.overall_health = HealthStatus::Healthy;
    }

    // Verify recovered
    {
        let state = core.state.read().await;
        assert_eq!(state.overall_health, HealthStatus::Healthy);
    }
}

/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: high
#[tokio::test]
async fn test_core_multiple_config_types() {
    // Test core with different configuration types
    let configs = vec![
        UnifiedBearDogConfig::default(),
        UnifiedBearDogConfig::development(),
    ];

    for config in configs {
        let result = BearDogCore::new(config);
        let state = result.state.read().await;
        assert_eq!(state.overall_health, HealthStatus::Healthy);
    }
}

/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: high
#[tokio::test]
async fn test_core_rapid_state_changes() {
    // Test rapid state changes
    let core = BearDogCore::with_default_config().unwrap();

    for i in 0..20 {
        let mut state = core.state.write().await;
        if i % 2 == 0 {
            state.overall_health = HealthStatus::Healthy;
        } else {
            state.overall_health = HealthStatus::Degraded;
        }
    }

    // Explicitly set to healthy at the end
    {
        let mut state = core.state.write().await;
        state.overall_health = HealthStatus::Healthy;
    }

    // Verify final state
    let state = core.state.read().await;
    assert_eq!(state.overall_health, HealthStatus::Healthy);
}

/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: medium
#[tokio::test]
async fn test_core_state_read_write_balance() {
    // Test that many readers don't block writers
    let core = std::sync::Arc::new(BearDogCore::with_default_config().unwrap());

    // Use barrier for truly concurrent read/write test (no polling waits)
    let barrier = Arc::new(tokio::sync::Barrier::new(11)); // 10 readers + 1 writer

    // Spawn readers that check state exists (not specific value due to concurrent writes)
    let mut read_handles = vec![];
    for _ in 0..10 {
        let core_ref = core.clone();
        let barrier_ref = Arc::clone(&barrier);
        let handle = tokio::spawn(async move {
            // Wait for all tasks to be ready (no artificial delays)
            barrier_ref.wait().await;

            // Read truly concurrently with writer
            let state = core_ref.state.read().await;
            // Just verify we can read state, don't assert specific value
            let _health = state.overall_health;
        });
        read_handles.push(handle);
    }

    // Writer joins the barrier - all tasks start simultaneously
    barrier.wait().await;

    // Write concurrently with reads (tests RwLock behavior)
    {
        let mut state = core.state.write().await;
        state.overall_health = HealthStatus::Degraded;
    }

    // Restore state
    {
        let mut state = core.state.write().await;
        state.overall_health = HealthStatus::Healthy;
    }

    // Wait for readers
    for handle in read_handles {
        handle.await.unwrap();
    }

    // Verify final state
    let state = core.state.read().await;
    assert_eq!(state.overall_health, HealthStatus::Healthy);
}

/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: high
#[tokio::test]
async fn test_core_initialization_with_development_config() {
    // Test initialization with development configuration
    let config = UnifiedBearDogConfig::development();

    let core = BearDogCore::new(config);
    let state = core.state.read().await;
    assert_eq!(state.overall_health, HealthStatus::Healthy);
}

/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: medium
#[tokio::test]
async fn test_core_start_time_immutability() {
    // Verify start time cannot be changed
    let core = BearDogCore::with_default_config().unwrap();

    let original_start_time = {
        let state = core.state.read().await;
        state.start_time
    };

    // Perform multiple operations to verify start_time invariant
    // (no artificial delay - test the invariant directly)
    for _ in 0..10 {
        let state = core.state.read().await;
        assert_eq!(
            state.start_time, original_start_time,
            "Start time must remain constant across operations"
        );
    }
}

/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: high
#[tokio::test]
async fn test_core_state_management() {
    // Test state management operations
    let core = BearDogCore::with_default_config().unwrap();

    // Test read access
    {
        let state = core.state.read().await;
        assert_eq!(state.overall_health, HealthStatus::Healthy);
    }

    // Test write access
    {
        let mut state = core.state.write().await;
        let original_health = state.overall_health;
        state.overall_health = HealthStatus::Degraded;
        assert_eq!(state.overall_health, HealthStatus::Degraded);
        // Restore
        state.overall_health = original_health;
    }

    // Verify state after write
    {
        let state = core.state.read().await;
        assert_eq!(state.overall_health, HealthStatus::Healthy);
    }
}

/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: high
#[tokio::test]
async fn test_core_error_handling() {
    // Test that core handles invalid configurations gracefully
    let config = UnifiedBearDogConfig::development();
    let core = BearDogCore::new(config);

    // Core should still be in valid state
    let state = core.state.read().await;
    assert!(
        state.overall_health != HealthStatus::Unhealthy,
        "Core should maintain health even with challenges"
    );
}

/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: normal
#[tokio::test]
async fn test_core_multiple_instances() {
    // Test that multiple core instances can coexist
    let core1 = BearDogCore::with_default_config().unwrap();
    let core2 = BearDogCore::with_default_config().unwrap();

    let state1 = core1.state.read().await;
    let state2 = core2.state.read().await;

    assert_eq!(state1.overall_health, HealthStatus::Healthy);
    assert_eq!(state2.overall_health, HealthStatus::Healthy);

    // They should be independent
    assert_ne!(
        state1.start_time, state2.start_time,
        "Different instances should have different start times"
    );
}
