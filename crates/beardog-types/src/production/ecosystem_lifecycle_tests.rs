//! Production Ecosystem Lifecycle Tests
//! Created: October 26, 2025
//! Purpose: Week 2 Day 3 - Test ecosystem lifecycle, state transitions, and edge cases

use super::*;

// ============================================================================
// Test 1: ProductionEcosystemBuilder Comprehensive
// ============================================================================

#[test]
fn test_ecosystem_builder_full_configuration() {
    let ecosystem = ProductionEcosystemBuilder::new()
        .environment_level(EnvironmentLevel::Production)
        .service("test-service".to_string(), "1.2.3".to_string())
        .deployment(
            "deploy-123".to_string(),
            "us-west-2".to_string(),
            "prod-cluster".to_string(),
        )
        .enable_advanced_features()
        .build();

    assert!(ecosystem.is_ok(), "Builder should create ecosystem");
    let ecosystem = ecosystem.unwrap();

    assert_eq!(
        ecosystem.config.core.environment_level,
        EnvironmentLevel::Production
    );
    assert_eq!(ecosystem.config.core.service_name, "test-service");
    assert_eq!(ecosystem.config.core.service_version, "1.2.3");
    assert_eq!(ecosystem.config.core.deployment_id, "deploy-123");
    assert_eq!(ecosystem.config.core.region, "us-west-2");
    assert_eq!(ecosystem.config.core.cluster_id, "prod-cluster");
    assert!(ecosystem.config.core.flags.enable_advanced_monitoring);
    assert!(ecosystem.config.core.flags.enable_distributed_tracing);
}

// ============================================================================
// Test 2: State Transitions During Lifecycle
// ============================================================================

#[test]
fn test_ecosystem_state_transitions() {
    let config = ProductionConfig::default();
    let mut ecosystem = ProductionEcosystem::new(config).expect("Should create ecosystem");

    // Initial state should be Initializing
    assert_eq!(
        ecosystem.get_status().status,
        OperationalStatus::Initializing
    );

    // After initialization should be Healthy
    ecosystem.initialize().expect("Should initialize");
    assert_eq!(ecosystem.get_status().status, OperationalStatus::Healthy);

    // After shutdown should be Shutdown
    ecosystem.shutdown().expect("Should shutdown cleanly");
    assert_eq!(ecosystem.get_status().status, OperationalStatus::Shutdown);
}

// ============================================================================
// Test 3: Multiple Initialization Attempts (Should Fail)
// ============================================================================

#[test]
fn test_ecosystem_multiple_initializations() {
    let config = ProductionConfig::default();
    let mut ecosystem = ProductionEcosystem::new(config).expect("Should create ecosystem");

    // First initialization
    ecosystem.initialize().expect("First init should succeed");
    assert_eq!(ecosystem.get_status().status, OperationalStatus::Healthy);

    // Second initialization should fail (metrics already active)
    let result = ecosystem.initialize();
    assert!(
        result.is_err(),
        "Second initialization should fail - system already running"
    );
}

// ============================================================================
// Test 4: Uptime Tracking
// ============================================================================

#[test]
fn test_ecosystem_uptime_tracking() {
    let config = ProductionConfig::default();
    let ecosystem = ProductionEcosystem::new(config).expect("Should create ecosystem");

    let uptime1 = ecosystem.uptime();
    std::thread::sleep(std::time::Duration::from_millis(10));
    let uptime2 = ecosystem.uptime();

    assert!(uptime2 > uptime1, "Uptime should increase");
    assert!(uptime2.as_millis() >= 10, "Uptime should be at least 10ms");
}

// ============================================================================
// Test 5: Health Check State Updates
// ============================================================================

#[test]
fn test_health_check_updates_state() {
    let config = ProductionConfig::default();
    let mut ecosystem = ProductionEcosystem::new(config).expect("Should create ecosystem");

    ecosystem.initialize().expect("Should initialize");

    let _initial_status = ecosystem.get_status().status.clone();
    let health_report = ecosystem.health_check().expect("Health check should work");

    // Status should match health report
    let expected_status = match health_report.overall_status {
        health::HealthStatus::Healthy => OperationalStatus::Healthy,
        health::HealthStatus::Degraded => OperationalStatus::Degraded,
        health::HealthStatus::Unhealthy => OperationalStatus::Unhealthy,
        health::HealthStatus::Critical => OperationalStatus::Critical,
    };

    assert_eq!(ecosystem.get_status().status, expected_status);
}

// ============================================================================
// Test 6: Metrics Update Without Auto-Scaling
// ============================================================================

#[test]
fn test_metrics_update_without_auto_scaling() {
    let mut config = ProductionConfig::default();
    config.core.flags.enable_auto_scaling = false;

    let mut ecosystem = ProductionEcosystem::new(config).expect("Should create ecosystem");
    ecosystem.initialize().expect("Should initialize");

    let result = ecosystem.update_metrics();
    assert!(result.is_ok(), "Metrics update should succeed");
}

// ============================================================================
// Test 7: Metrics Update With Auto-Scaling
// ============================================================================

#[test]
fn test_metrics_update_with_auto_scaling() {
    let mut config = ProductionConfig::default();
    config.core.flags.enable_auto_scaling = true;

    let mut ecosystem = ProductionEcosystem::new(config).expect("Should create ecosystem");
    ecosystem.initialize().expect("Should initialize");

    let result = ecosystem.update_metrics();
    assert!(
        result.is_ok(),
        "Metrics update with auto-scaling should succeed"
    );
}

// ============================================================================
// Test 8: Performance Metrics Default Values
// ============================================================================

#[test]
fn test_performance_metrics_defaults() {
    let metrics = PerformanceMetrics::default();

    assert_eq!(metrics.avg_response_time_ms, 0.0);
    assert_eq!(metrics.p95_response_time_ms, 0.0);
    assert_eq!(metrics.p99_response_time_ms, 0.0);
    assert_eq!(metrics.requests_per_second, 0.0);
    assert_eq!(metrics.error_rate_percent, 0.0);
    assert_eq!(metrics.throughput_bytes_per_sec, 0);
}

// ============================================================================
// Test 9: Production State Counters
// ============================================================================

#[test]
fn test_production_state_counters() {
    let mut state = ProductionState::default();

    assert_eq!(state.active_connections, 0);
    assert_eq!(state.total_requests, 0);
    assert_eq!(state.error_count_hourly, 0);

    // Simulate activity
    state.active_connections = 100;
    state.total_requests = 50000;
    state.error_count_hourly = 5;

    assert_eq!(state.active_connections, 100);
    assert_eq!(state.total_requests, 50000);
    assert_eq!(state.error_count_hourly, 5);
}

// ============================================================================
// Test 10: Builder Default Values
// ============================================================================

#[test]
fn test_builder_default_creates_valid_ecosystem() {
    let ecosystem = ProductionEcosystemBuilder::default()
        .build()
        .expect("Default builder should work");

    assert_eq!(
        ecosystem.config.core.environment_level,
        EnvironmentLevel::Development
    );
    assert_eq!(ecosystem.config.core.service_name, "beardog-ecosystem");
}

// ============================================================================
// Test 11: Shutdown Before Initialize (Should Fail or Handle Gracefully)
// ============================================================================

#[test]
fn test_shutdown_before_initialize() {
    let config = ProductionConfig::default();
    let mut ecosystem = ProductionEcosystem::new(config).expect("Should create ecosystem");

    // Shutdown without initializing - may fail or handle gracefully
    let result = ecosystem.shutdown();

    // System should either handle gracefully or fail predictably
    // We test that it doesn't panic
    if result.is_ok() {
        assert_eq!(ecosystem.get_status().status, OperationalStatus::Shutdown);
    } else {
        // Expected: shutdown requires initialization first
        assert!(result.is_err(), "Shutdown before init may fail (expected)");
    }
}

// ============================================================================
// Test 12: Environment Level Serialization Roundtrip
// ============================================================================

#[test]
fn test_all_environment_levels_serialization() {
    let levels = vec![
        EnvironmentLevel::Development,
        EnvironmentLevel::Staging,
        EnvironmentLevel::PreProduction,
        EnvironmentLevel::Production,
        EnvironmentLevel::Critical,
    ];

    for level in levels {
        let serialized = serde_json::to_string(&level).expect("Should serialize");
        let deserialized: EnvironmentLevel =
            serde_json::from_str(&serialized).expect("Should deserialize");
        assert_eq!(level, deserialized, "Roundtrip should preserve value");
    }
}
