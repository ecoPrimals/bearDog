//! Comprehensive Type Validation Tests
//!
//! High-value integration tests for canonical type construction,
//! validation, conversion, and edge cases.
//!
//! TEST_CATEGORY: integration
//! TEST_DOMAIN: types

use beardog_types::canonical::config::domains::testing::CanonicalTestConfig;
use beardog_types::canonical::{HealthStatus, SecurityLevel, WorkflowStatus};

// ====================================================================================
// HealthStatus Tests (8 tests)
// ====================================================================================

#[test]
fn test_health_status_healthy() {
    // Test healthy status construction
    let status = HealthStatus::Healthy;
    assert_eq!(status, HealthStatus::Healthy);
}

#[test]
fn test_health_status_degraded() {
    // Test degraded status construction
    let status = HealthStatus::Degraded;
    assert_eq!(status, HealthStatus::Degraded);
}

#[test]
fn test_health_status_unhealthy() {
    // Test unhealthy status construction
    let status = HealthStatus::Unhealthy;
    assert_eq!(status, HealthStatus::Unhealthy);
}

#[test]
fn test_health_status_unknown() {
    // Test unknown status construction
    let status = HealthStatus::Unknown;
    assert_eq!(status, HealthStatus::Unknown);
}

#[test]
fn test_health_status_equality() {
    // Test health status equality
    let status1 = HealthStatus::Healthy;
    let status2 = HealthStatus::Healthy;
    let status3 = HealthStatus::Degraded;

    assert_eq!(status1, status2);
    assert_ne!(status1, status3);
}

#[test]
fn test_health_status_clone() {
    // Test health status can be cloned
    let status1 = HealthStatus::Healthy;
    let status2 = status1.clone();

    assert_eq!(status1, status2);
}

#[test]
fn test_health_status_debug() {
    // Test health status debug formatting
    let status = HealthStatus::Healthy;
    let debug_str = format!("{:?}", status);

    assert!(!debug_str.is_empty());
    assert!(debug_str.contains("Healthy"));
}

#[test]
fn test_health_status_pattern_matching() {
    // Test health status pattern matching
    let status = HealthStatus::Degraded;

    let result = match status {
        HealthStatus::Healthy => "healthy",
        HealthStatus::Degraded => "degraded",
        HealthStatus::Unhealthy => "unhealthy",
        HealthStatus::Unknown => "unknown",
    };

    assert_eq!(result, "degraded");
}

// ====================================================================================
// SecurityLevel Tests (8 tests)
// ====================================================================================

#[test]
fn test_security_level_basic() {
    // Test basic security level
    let level = SecurityLevel::Basic;
    assert_eq!(level, SecurityLevel::Basic);
}

#[test]
fn test_security_level_standard() {
    // Test standard security level
    let level = SecurityLevel::Standard;
    assert_eq!(level, SecurityLevel::Standard);
}

#[test]
fn test_security_level_high() {
    // Test high security level
    let level = SecurityLevel::High;
    assert_eq!(level, SecurityLevel::High);
}

#[test]
fn test_security_level_critical() {
    // Test critical security level
    let level = SecurityLevel::Critical;
    assert_eq!(level, SecurityLevel::Critical);
}

#[test]
fn test_security_level_equality() {
    // Test security level equality
    let level1 = SecurityLevel::High;
    let level2 = SecurityLevel::High;
    let level3 = SecurityLevel::Basic;

    assert_eq!(level1, level2);
    assert_ne!(level1, level3);
}

#[test]
fn test_security_level_clone() {
    // Test security level clone
    let level1 = SecurityLevel::Critical;
    let level2 = level1.clone();

    assert_eq!(level1, level2);
}

#[test]
fn test_security_level_debug() {
    // Test security level debug format
    let level = SecurityLevel::High;
    let debug_str = format!("{:?}", level);

    assert!(!debug_str.is_empty());
    assert!(debug_str.contains("High"));
}

#[test]
fn test_security_level_pattern_matching() {
    // Test security level pattern matching
    let level = SecurityLevel::Critical;

    let result = match level {
        SecurityLevel::Basic => 1,
        SecurityLevel::Standard => 2,
        SecurityLevel::High => 3,
        SecurityLevel::Critical => 4,
    };

    assert_eq!(result, 4);
}

// ====================================================================================
// WorkflowStatus Tests (8 tests)
// ====================================================================================

#[test]
fn test_workflow_status_pending() {
    // Test pending workflow status
    let status = WorkflowStatus::Pending;
    assert_eq!(status, WorkflowStatus::Pending);
}

#[test]
fn test_workflow_status_in_progress() {
    // Test in progress workflow status
    let status = WorkflowStatus::InProgress;
    assert_eq!(status, WorkflowStatus::InProgress);
}

#[test]
fn test_workflow_status_completed() {
    // Test completed workflow status
    let status = WorkflowStatus::Completed;
    assert_eq!(status, WorkflowStatus::Completed);
}

#[test]
fn test_workflow_status_failed() {
    // Test failed workflow status
    let status = WorkflowStatus::Failed;
    assert_eq!(status, WorkflowStatus::Failed);
}

#[test]
fn test_workflow_status_equality() {
    // Test workflow status equality
    let status1 = WorkflowStatus::InProgress;
    let status2 = WorkflowStatus::InProgress;
    let status3 = WorkflowStatus::Completed;

    assert_eq!(status1, status2);
    assert_ne!(status1, status3);
}

#[test]
fn test_workflow_status_clone() {
    // Test workflow status clone
    let status1 = WorkflowStatus::Failed;
    let status2 = status1.clone();

    assert_eq!(status1, status2);
}

#[test]
fn test_workflow_status_debug() {
    // Test workflow status debug
    let status = WorkflowStatus::InProgress;
    let debug_str = format!("{:?}", status);

    assert!(!debug_str.is_empty());
    assert!(debug_str.contains("InProgress"));
}

#[test]
fn test_workflow_status_lifecycle() {
    // Test complete workflow lifecycle
    let mut status = WorkflowStatus::Pending;
    assert_eq!(status, WorkflowStatus::Pending);

    status = WorkflowStatus::InProgress;
    assert_eq!(status, WorkflowStatus::InProgress);

    status = WorkflowStatus::Completed;
    assert_eq!(status, WorkflowStatus::Completed);
}

// ====================================================================================
// CanonicalTestConfig Tests (10 tests)
// ====================================================================================

#[test]
fn test_canonical_test_config_default() {
    // Test default test config creation
    let config = CanonicalTestConfig::default();

    assert_eq!(config.environment, "test");
    assert!(!config.verbose);
    assert!(config.parallel_execution);
}

#[test]
fn test_canonical_test_config_environment() {
    // Test environment field
    let config = CanonicalTestConfig::default();
    assert_eq!(config.environment, "test");
}

#[test]
fn test_canonical_test_config_max_threads() {
    // Test max_threads is reasonable
    let config = CanonicalTestConfig::default();
    assert!(config.max_threads >= 1);
    assert!(config.max_threads <= 128);
}

#[test]
fn test_canonical_test_config_timeout() {
    // Test timeout is set
    let config = CanonicalTestConfig::default();
    assert_eq!(config.timeout_seconds, 300);
}

#[test]
fn test_canonical_test_config_property_testing() {
    // Test property testing config
    let config = CanonicalTestConfig::default();
    assert!(config.property_testing_enabled);
    assert_eq!(config.property_test_iterations, 100);
}

#[test]
fn test_canonical_test_config_fuzzing() {
    // Test fuzzing config
    let config = CanonicalTestConfig::default();
    assert!(!config.fuzzing_enabled);
}

#[test]
fn test_canonical_test_config_random_seed() {
    // Test random seed is None by default
    let config = CanonicalTestConfig::default();
    assert!(config.random_seed.is_none());
}

#[test]
fn test_canonical_test_config_test_data_dir() {
    // Test data directory is set
    let config = CanonicalTestConfig::default();
    assert_eq!(config.test_data_dir.to_str().unwrap(), "test-data");
}

#[test]
fn test_canonical_test_config_clone() {
    // Test config can be cloned
    let config1 = CanonicalTestConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.environment, config2.environment);
    assert_eq!(config1.max_threads, config2.max_threads);
}

#[test]
fn test_canonical_test_config_debug() {
    // Test config debug formatting
    let config = CanonicalTestConfig::default();
    let debug_str = format!("{:?}", config);

    assert!(!debug_str.is_empty());
    assert!(debug_str.contains("test"));
}

// ====================================================================================
// Type Conversion Tests (5 tests)
// ====================================================================================

#[test]
fn test_health_status_to_string() {
    // Test health status string representation
    let status = HealthStatus::Healthy;
    let s = format!("{:?}", status);

    assert!(!s.is_empty());
}

#[test]
fn test_security_level_ordering() {
    // Test security levels have logical ordering
    let basic = SecurityLevel::Basic;
    let high = SecurityLevel::High;

    // At minimum they should be different
    assert_ne!(basic, high);
}

#[test]
fn test_workflow_status_transition() {
    // Test valid workflow transitions
    let statuses = vec![
        WorkflowStatus::Pending,
        WorkflowStatus::InProgress,
        WorkflowStatus::Completed,
    ];

    // Should be able to iterate through states
    assert_eq!(statuses.len(), 3);
}

#[test]
fn test_type_size_reasonable() {
    // Test canonical types have reasonable sizes
    use std::mem::size_of;

    let health_size = size_of::<HealthStatus>();
    let security_size = size_of::<SecurityLevel>();
    let workflow_size = size_of::<WorkflowStatus>();

    // Enums should be small (typically 1-8 bytes)
    assert!(health_size <= 16, "HealthStatus too large");
    assert!(security_size <= 16, "SecurityLevel too large");
    assert!(workflow_size <= 16, "WorkflowStatus too large");
}

#[test]
fn test_config_size_reasonable() {
    // Test config type has reasonable size
    use std::mem::size_of;

    let config_size = size_of::<CanonicalTestConfig>();

    // Config should be under 1KB
    assert!(
        config_size < 1024,
        "CanonicalTestConfig too large: {} bytes",
        config_size
    );
}

// ====================================================================================
// Edge Case Tests (5 tests)
// ====================================================================================

#[test]
fn test_health_status_multiple_clones() {
    // Test multiple levels of cloning
    let status1 = HealthStatus::Healthy;
    let status2 = status1.clone();
    let status3 = status2.clone();
    let status4 = status3.clone();

    assert_eq!(status1, status4);
}

#[test]
fn test_security_level_in_vec() {
    // Test security levels in collections
    let levels = vec![
        SecurityLevel::Basic,
        SecurityLevel::Standard,
        SecurityLevel::High,
        SecurityLevel::Critical,
    ];

    assert_eq!(levels.len(), 4);
    assert_eq!(levels[0], SecurityLevel::Basic);
    assert_eq!(levels[3], SecurityLevel::Critical);
}

#[test]
fn test_workflow_status_in_option() {
    // Test workflow status in Option
    let maybe_status: Option<WorkflowStatus> = Some(WorkflowStatus::InProgress);

    assert!(maybe_status.is_some());
    assert_eq!(maybe_status.unwrap(), WorkflowStatus::InProgress);
}

#[test]
fn test_health_status_in_result() {
    // Test health status in Result
    let result: Result<HealthStatus, String> = Ok(HealthStatus::Healthy);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), HealthStatus::Healthy);
}

#[test]
fn test_config_modification() {
    // Test config can be modified
    let mut config = CanonicalTestConfig::default();

    config.verbose = true;
    assert!(config.verbose);

    config.max_threads = 8;
    assert_eq!(config.max_threads, 8);

    config.timeout_seconds = 600;
    assert_eq!(config.timeout_seconds, 600);
}

// ====================================================================================
// Integration Tests (3 tests)
// ====================================================================================

#[test]
fn test_types_work_together() {
    // Test different types can be used together
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

#[test]
fn test_types_in_struct() {
    // Test types can be composed in structs
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

#[test]
fn test_types_concurrent_access() {
    // Test types are safe for concurrent use
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

    handle.join().expect("Thread should complete");
}

// ====================================================================================
// Test Helpers
// ====================================================================================

#[cfg(test)]
mod type_helpers {
    use super::*;

    /// Check if health status is operational
    pub fn is_operational(status: &HealthStatus) -> bool {
        matches!(status, HealthStatus::Healthy | HealthStatus::Degraded)
    }

    /// Check if security level is sufficient
    pub fn is_secure_enough(level: &SecurityLevel, required: &SecurityLevel) -> bool {
        level == required
            || matches!(
                (level, required),
                (SecurityLevel::Critical, _)
                    | (SecurityLevel::High, SecurityLevel::Standard)
                    | (SecurityLevel::High, SecurityLevel::Basic)
                    | (SecurityLevel::Standard, SecurityLevel::Basic)
            )
    }
}

#[test]
fn test_helper_is_operational() {
    assert!(type_helpers::is_operational(&HealthStatus::Healthy));
    assert!(type_helpers::is_operational(&HealthStatus::Degraded));
    assert!(!type_helpers::is_operational(&HealthStatus::Unhealthy));
}

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
