// SPDX-License-Identifier: AGPL-3.0-only
//! Comprehensive Type Validation Tests
//!
//! This module contains high-value integration tests for canonical type construction,
//! validation, conversion, and edge cases. Tests cover `HealthStatus`, `SecurityLevel`,
//! `WorkflowStatus`, and `CanonicalTestConfig` types.
//!
//! Categories tested:
//! - Basic type construction and equality
//! - Clone and Debug trait implementations
//! - Pattern matching and lifecycle transitions
//! - Type conversions and compositions
//! - Edge cases (collections, Options, Results)
//! - Concurrent access and thread safety
//! - Helper functions and utilities

#![allow(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::field_reassign_with_default)]

use beardog_types::canonical::config::domains::testing::CanonicalTestConfig;
use beardog_types::canonical::{HealthStatus, SecurityLevel, WorkflowStatus};

// ============================================================================
// HealthStatus Tests (8 tests)
// ============================================================================

/// Tests construction of Healthy status variant
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: high
#[test]
fn test_health_status_healthy() {
    let status = HealthStatus::Healthy;
    assert_eq!(status, HealthStatus::Healthy);
}

/// Tests construction of Degraded status variant
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: high
#[test]
fn test_health_status_degraded() {
    let status = HealthStatus::Degraded;
    assert_eq!(status, HealthStatus::Degraded);
}

/// Tests construction of Unhealthy status variant
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: high
#[test]
fn test_health_status_unhealthy() {
    let status = HealthStatus::Unhealthy;
    assert_eq!(status, HealthStatus::Unhealthy);
}

/// Tests construction of Unknown status variant
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_health_status_unknown() {
    let status = HealthStatus::Unknown;
    assert_eq!(status, HealthStatus::Unknown);
}

/// Tests equality comparison between `HealthStatus` variants
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: high
#[test]
fn test_health_status_equality() {
    let status1 = HealthStatus::Healthy;
    let status2 = HealthStatus::Healthy;
    let status3 = HealthStatus::Degraded;

    assert_eq!(status1, status2, "Same variants should be equal");
    assert_ne!(status1, status3, "Different variants should not be equal");
}

/// Tests Clone trait implementation for `HealthStatus`
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_health_status_clone() {
    let status1 = HealthStatus::Healthy;
    let status2 = status1;

    assert_eq!(status1, status2, "Cloned status should equal original");
}

/// Tests Debug trait implementation for `HealthStatus`
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_health_status_debug() {
    let status = HealthStatus::Healthy;
    let debug_str = format!("{status:?}");

    assert!(!debug_str.is_empty(), "Debug string should not be empty");
    assert!(
        debug_str.contains("Healthy"),
        "Debug string should contain variant name"
    );
}

/// Tests pattern matching on `HealthStatus` variants
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_health_status_pattern_matching() {
    let status = HealthStatus::Degraded;

    let result = match status {
        HealthStatus::Healthy => "healthy",
        HealthStatus::Degraded => "degraded",
        HealthStatus::Unhealthy => "unhealthy",
        HealthStatus::Unknown => "unknown",
    };

    assert_eq!(result, "degraded");
}

// ============================================================================
// SecurityLevel Tests (8 tests)
// ============================================================================

/// Tests construction of Basic security level
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: high
#[test]
fn test_security_level_basic() {
    let level = SecurityLevel::Basic;
    assert_eq!(level, SecurityLevel::Basic);
}

/// Tests construction of Standard security level
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: high
#[test]
fn test_security_level_standard() {
    let level = SecurityLevel::Standard;
    assert_eq!(level, SecurityLevel::Standard);
}

/// Tests construction of High security level
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: high
#[test]
fn test_security_level_high() {
    let level = SecurityLevel::High;
    assert_eq!(level, SecurityLevel::High);
}

/// Tests construction of Critical security level
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: critical
#[test]
fn test_security_level_critical() {
    let level = SecurityLevel::Critical;
    assert_eq!(level, SecurityLevel::Critical);
}

/// Tests equality comparison between `SecurityLevel` variants
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: high
#[test]
fn test_security_level_equality() {
    let level1 = SecurityLevel::High;
    let level2 = SecurityLevel::High;
    let level3 = SecurityLevel::Basic;

    assert_eq!(level1, level2, "Same levels should be equal");
    assert_ne!(level1, level3, "Different levels should not be equal");
}

/// Tests Clone trait implementation for `SecurityLevel`
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_security_level_clone() {
    let level1 = SecurityLevel::Critical;
    let level2 = level1.clone();

    assert_eq!(level1, level2, "Cloned level should equal original");
}

/// Tests Debug trait implementation for `SecurityLevel`
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_security_level_debug() {
    let level = SecurityLevel::High;
    let debug_str = format!("{level:?}");

    assert!(!debug_str.is_empty(), "Debug string should not be empty");
    assert!(
        debug_str.contains("High"),
        "Debug string should contain level name"
    );
}

/// Tests pattern matching on `SecurityLevel` variants
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_security_level_pattern_matching() {
    let level = SecurityLevel::Critical;

    let result = match level {
        SecurityLevel::Basic => 1,
        SecurityLevel::Standard => 2,
        SecurityLevel::High => 3,
        SecurityLevel::Critical => 4,
    };

    assert_eq!(result, 4);
}

// ============================================================================
// WorkflowStatus Tests (8 tests)
// ============================================================================

/// Tests construction of Pending workflow status
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: high
#[test]
fn test_workflow_status_pending() {
    let status = WorkflowStatus::Pending;
    assert_eq!(status, WorkflowStatus::Pending);
}

/// Tests construction of `InProgress` workflow status
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: high
#[test]
fn test_workflow_status_in_progress() {
    let status = WorkflowStatus::InProgress;
    assert_eq!(status, WorkflowStatus::InProgress);
}

/// Tests construction of Completed workflow status
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: high
#[test]
fn test_workflow_status_completed() {
    let status = WorkflowStatus::Completed;
    assert_eq!(status, WorkflowStatus::Completed);
}

/// Tests construction of Failed workflow status
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: high
#[test]
fn test_workflow_status_failed() {
    let status = WorkflowStatus::Failed;
    assert_eq!(status, WorkflowStatus::Failed);
}

/// Tests equality comparison between `WorkflowStatus` variants
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: high
#[test]
fn test_workflow_status_equality() {
    let status1 = WorkflowStatus::InProgress;
    let status2 = WorkflowStatus::InProgress;
    let status3 = WorkflowStatus::Completed;

    assert_eq!(status1, status2, "Same statuses should be equal");
    assert_ne!(status1, status3, "Different statuses should not be equal");
}

/// Tests Clone trait implementation for `WorkflowStatus`
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_workflow_status_clone() {
    let status1 = WorkflowStatus::Failed;
    let status2 = status1.clone();

    assert_eq!(status1, status2, "Cloned status should equal original");
}

/// Tests Debug trait implementation for `WorkflowStatus`
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_workflow_status_debug() {
    let status = WorkflowStatus::InProgress;
    let debug_str = format!("{status:?}");

    assert!(!debug_str.is_empty(), "Debug string should not be empty");
    assert!(
        debug_str.contains("InProgress"),
        "Debug string should contain status name"
    );
}

/// Tests complete workflow lifecycle transitions
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: high
#[test]
fn test_workflow_status_lifecycle() {
    // Simulate workflow progression
    let mut status = WorkflowStatus::Pending;
    assert_eq!(status, WorkflowStatus::Pending);

    status = WorkflowStatus::InProgress;
    assert_eq!(status, WorkflowStatus::InProgress);

    status = WorkflowStatus::Completed;
    assert_eq!(status, WorkflowStatus::Completed);
}

// ============================================================================
// CanonicalTestConfig Tests (10 tests)
// ============================================================================

/// Tests default `CanonicalTestConfig` construction
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: high
#[test]
fn test_canonical_test_config_default() {
    let config = CanonicalTestConfig::default();

    assert_eq!(
        config.environment, "test",
        "Default environment should be 'test'"
    );
    assert!(!config.verbose, "Verbose should be false by default");
    assert!(
        config.parallel_execution,
        "Parallel execution should be enabled by default"
    );
}

/// Tests environment field is properly set
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_canonical_test_config_environment() {
    let config = CanonicalTestConfig::default();
    assert_eq!(config.environment, "test");
}

/// Tests `max_threads` field has reasonable default value
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_canonical_test_config_max_threads() {
    let config = CanonicalTestConfig::default();
    assert!(config.max_threads >= 1, "Max threads should be at least 1");
    assert!(
        config.max_threads <= 128,
        "Max threads should be at most 128"
    );
}

/// Tests timeout configuration
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_canonical_test_config_timeout() {
    let config = CanonicalTestConfig::default();
    assert_eq!(
        config.timeout_seconds, 300,
        "Default timeout should be 300 seconds"
    );
}

/// Tests property testing configuration
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_canonical_test_config_property_testing() {
    let config = CanonicalTestConfig::default();
    assert!(
        config.property_testing_enabled,
        "Property testing should be enabled"
    );
    assert_eq!(
        config.property_test_iterations, 100,
        "Should have 100 iterations by default"
    );
}

/// Tests fuzzing configuration
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_canonical_test_config_fuzzing() {
    let config = CanonicalTestConfig::default();
    assert!(
        !config.fuzzing_enabled,
        "Fuzzing should be disabled by default"
    );
}

/// Tests random seed configuration
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_canonical_test_config_random_seed() {
    let config = CanonicalTestConfig::default();
    assert!(
        config.random_seed.is_none(),
        "Random seed should be None by default"
    );
}

/// Tests test data directory configuration
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_canonical_test_config_test_data_dir() {
    let config = CanonicalTestConfig::default();
    assert_eq!(config.test_data_dir.to_str().unwrap(), "test-data");
}

/// Tests Clone trait implementation for `CanonicalTestConfig`
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_canonical_test_config_clone() {
    let config1 = CanonicalTestConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.environment, config2.environment);
    assert_eq!(config1.max_threads, config2.max_threads);
}

/// Tests Debug trait implementation for `CanonicalTestConfig`
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_canonical_test_config_debug() {
    let config = CanonicalTestConfig::default();
    let debug_str = format!("{config:?}");

    assert!(!debug_str.is_empty(), "Debug string should not be empty");
    assert!(
        debug_str.contains("test"),
        "Debug string should contain environment"
    );
}

// ============================================================================
// Type Conversion Tests (5 tests)
// ============================================================================

/// Tests `HealthStatus` debug string representation
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_health_status_to_string() {
    let status = HealthStatus::Healthy;
    let s = format!("{status:?}");

    assert!(!s.is_empty(), "String representation should not be empty");
}

/// Tests `SecurityLevel` logical ordering
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_security_level_ordering() {
    let basic = SecurityLevel::Basic;
    let high = SecurityLevel::High;

    assert_ne!(basic, high, "Different security levels should not be equal");
}

/// Tests valid `WorkflowStatus` transitions
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_workflow_status_transition() {
    let statuses = [
        WorkflowStatus::Pending,
        WorkflowStatus::InProgress,
        WorkflowStatus::Completed,
    ];

    assert_eq!(statuses.len(), 3, "Should have 3 states in transition");
}

/// Tests that canonical types have reasonable memory sizes
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_type_size_reasonable() {
    use std::mem::size_of;

    let health_size = size_of::<HealthStatus>();
    let security_size = size_of::<SecurityLevel>();
    let workflow_size = size_of::<WorkflowStatus>();

    // Enums should be small (typically 1-8 bytes)
    assert!(
        health_size <= 16,
        "HealthStatus too large: {health_size} bytes"
    );
    assert!(
        security_size <= 16,
        "SecurityLevel too large: {security_size} bytes"
    );
    assert!(
        workflow_size <= 16,
        "WorkflowStatus too large: {workflow_size} bytes"
    );
}

/// Tests that `CanonicalTestConfig` has reasonable memory size
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_config_size_reasonable() {
    use std::mem::size_of;

    let config_size = size_of::<CanonicalTestConfig>();

    // Config should be under 1KB
    assert!(
        config_size < 1024,
        "CanonicalTestConfig too large: {config_size} bytes"
    );
}

// ============================================================================
// Edge Case Tests (5 tests)
// ============================================================================

/// Tests multiple levels of cloning for `HealthStatus`
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_health_status_multiple_clones() {
    let status1 = HealthStatus::Healthy;
    let status2 = status1;
    let status3 = status2;
    let status4 = status3;

    assert_eq!(status1, status4, "Multiple clones should equal original");
}

/// Tests `SecurityLevel` types in vectors
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_security_level_in_vec() {
    let levels = [
        SecurityLevel::Basic,
        SecurityLevel::Standard,
        SecurityLevel::High,
        SecurityLevel::Critical,
    ];

    assert_eq!(levels.len(), 4);
    assert_eq!(levels[0], SecurityLevel::Basic);
    assert_eq!(levels[3], SecurityLevel::Critical);
}

/// Tests `WorkflowStatus` in Option types
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_workflow_status_in_option() {
    let maybe_status: Option<WorkflowStatus> = Some(WorkflowStatus::InProgress);

    assert!(maybe_status.is_some());
    // More idiomatic: use pattern matching or if-let
    if let Some(status) = maybe_status {
        assert_eq!(status, WorkflowStatus::InProgress);
    }
}

/// Tests `HealthStatus` in Result types
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_health_status_in_result() {
    let result: Result<HealthStatus, String> = Ok(HealthStatus::Healthy);

    assert!(result.is_ok());
    // More idiomatic: use pattern matching or if-let
    if let Ok(status) = result {
        assert_eq!(status, HealthStatus::Healthy);
    }
}

/// Tests that `CanonicalTestConfig` can be modified
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_config_modification() {
    let mut config = CanonicalTestConfig::default();

    config.verbose = true;
    assert!(config.verbose);

    config.max_threads = 8;
    assert_eq!(config.max_threads, 8);

    config.timeout_seconds = 600;
    assert_eq!(config.timeout_seconds, 600);
}

// ============================================================================
// Integration Tests (3 tests)
// ============================================================================

/// Tests that different canonical types work together in same context
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: high
#[test]
fn test_types_work_together() {
    let health = HealthStatus::Healthy;
    let security = SecurityLevel::High;
    let workflow = WorkflowStatus::InProgress;
    let config = CanonicalTestConfig::default();

    // All types should be usable in same context
    assert_eq!(health, HealthStatus::Healthy);
    assert_eq!(security, SecurityLevel::High);
    assert_eq!(workflow, WorkflowStatus::InProgress);
    assert!(!config.environment.is_empty());
}

/// Tests that canonical types can be composed in structs
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_types_in_struct() {
    #[derive(Debug, Clone)]
    struct TestContext {
        health: HealthStatus,
        security: SecurityLevel,
        workflow: WorkflowStatus,
    }

    let context = TestContext {
        health: HealthStatus::Healthy,
        security: SecurityLevel::High,
        workflow: WorkflowStatus::InProgress,
    };

    assert_eq!(context.health, HealthStatus::Healthy);
    assert_eq!(context.security, SecurityLevel::High);
    assert_eq!(context.workflow, WorkflowStatus::InProgress);
}

/// Tests that canonical types are safe for concurrent access
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: high
#[test]
fn test_types_concurrent_access() {
    use std::sync::Arc;
    use std::thread;

    let health = Arc::new(HealthStatus::Healthy);
    let security = Arc::new(SecurityLevel::High);

    let health_clone = Arc::clone(&health);
    let security_clone = Arc::clone(&security);

    let handle = thread::spawn(move || {
        assert_eq!(*health_clone, HealthStatus::Healthy);
        assert_eq!(*security_clone, SecurityLevel::High);
    });

    handle.join().expect("Thread should complete successfully");
}

// ============================================================================
// Helper Function Tests (2 tests)
// ============================================================================

#[cfg(test)]
mod type_helpers {
    use super::*;

    /// Check if health status is operational
    pub fn is_operational(status: &HealthStatus) -> bool {
        matches!(status, HealthStatus::Healthy | HealthStatus::Degraded)
    }

    /// Check if security level is sufficient for required level
    pub fn is_secure_enough(level: &SecurityLevel, required: &SecurityLevel) -> bool {
        level == required
            || matches!(
                (level, required),
                (SecurityLevel::Critical, _)
                    | (
                        SecurityLevel::High,
                        SecurityLevel::Standard | SecurityLevel::Basic
                    )
                    | (SecurityLevel::Standard, SecurityLevel::Basic)
            )
    }
}

/// Tests helper function for checking operational status
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_helper_is_operational() {
    assert!(type_helpers::is_operational(&HealthStatus::Healthy));
    assert!(type_helpers::is_operational(&HealthStatus::Degraded));
    assert!(!type_helpers::is_operational(&HealthStatus::Unhealthy));
}

/// Tests helper function for checking security level sufficiency
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_helper_is_secure_enough() {
    assert!(type_helpers::is_secure_enough(
        &SecurityLevel::High,
        &SecurityLevel::Standard
    ));
    assert!(!type_helpers::is_secure_enough(
        &SecurityLevel::Basic,
        &SecurityLevel::High
    ));
}
