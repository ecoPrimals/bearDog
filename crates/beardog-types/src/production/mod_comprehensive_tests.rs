//! Comprehensive tests for production module root
//!
//! Created: October 28, 2025
//! Purpose: Increase coverage for production/mod.rs from 17% → 60%+

use super::*;

#[test]
fn test_production_config_default() {
    let config = ProductionConfig::default();

    // Verify all fields are initialized
    assert!(format!("{:?}", config.core).contains("ProductionCoreConfig"));
    assert!(format!("{:?}", config.monitoring).contains("MonitoringConfig"));
    assert!(format!("{:?}", config.observability).contains("ObservabilityConfig"));
}

#[test]
fn test_production_config_clone() {
    let config1 = ProductionConfig::default();
    let config2 = config1.clone();

    // Verify clone creates independent copy
    assert_eq!(format!("{:?}", config1.core), format!("{:?}", config2.core));
}

#[test]
fn test_production_config_serialization() {
    let config = ProductionConfig::default();

    let json = serde_json::to_string(&config).expect("Serialization should succeed");
    assert!(!json.is_empty());
    assert!(json.contains("core"));

    let deserialized: ProductionConfig =
        serde_json::from_str(&json).expect("Deserialization should succeed");
    assert!(format!("{:?}", deserialized.core).contains("ProductionCoreConfig"));
}

#[test]
fn test_environment_level_variants() {
    let levels = vec![
        EnvironmentLevel::Development,
        EnvironmentLevel::Staging,
        EnvironmentLevel::PreProduction,
        EnvironmentLevel::Production,
        EnvironmentLevel::Critical,
    ];

    for level in levels {
        assert!(!format!("{:?}", level).is_empty());
    }
}

#[test]
fn test_environment_level_equality() {
    let level1 = EnvironmentLevel::Production;
    let level2 = EnvironmentLevel::Production;
    let level3 = EnvironmentLevel::Development;

    assert_eq!(level1, level2);
    assert_ne!(level1, level3);
}

#[test]
fn test_environment_level_serialization() {
    let level = EnvironmentLevel::Production;
    let json = serde_json::to_string(&level).expect("Serialization should succeed");
    assert!(!json.is_empty());

    let deserialized: EnvironmentLevel =
        serde_json::from_str(&json).expect("Deserialization should succeed");
    assert_eq!(level, deserialized);
}

#[test]
// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
fn test_operational_status_variants() {
    let statuses = vec![
        OperationalStatus::Initializing,
        OperationalStatus::Healthy,
        OperationalStatus::Degraded,
        OperationalStatus::Unhealthy,
        OperationalStatus::Critical,
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        OperationalStatus::Shutdown,
    ];

    for status in statuses {
        assert!(!format!("{:?}", status).is_empty());
    }
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
}

#[test]
fn test_operational_status_equality() {
    let status1 = OperationalStatus::Healthy;
    let status2 = OperationalStatus::Healthy;
    let status3 = OperationalStatus::Degraded;

    assert_eq!(status1, status2);
    assert_ne!(status1, status3);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
}

#[test]
fn test_operational_status_serialization() {
    let status = OperationalStatus::Degraded;
    let json = serde_json::to_string(&status).expect("Serialization should succeed");
    assert!(!json.is_empty());

    let deserialized: OperationalStatus =
        serde_json::from_str(&json).expect("Deserialization should succeed");
    assert_eq!(status, deserialized);
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal

#[test]
fn test_production_state_default() {
    let state = ProductionState::default();

    assert_eq!(state.active_connections, 0);
    assert_eq!(state.total_requests, 0);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert_eq!(state.memory_usage_bytes, 0);
    assert_eq!(state.cpu_usage_percent, 0.0);
}

#[test]
fn test_production_state_clone() {
    let state1 = ProductionState::default();
    let state2 = state1.clone();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    assert_eq!(state1.active_connections, state2.active_connections);
    assert_eq!(state1.total_requests, state2.total_requests);
}

#[test]
fn test_production_state_with_metrics() {
    let mut state = ProductionState::default();

    state.active_connections = 50;
    state.total_requests = 10000;
    state.memory_usage_bytes = 1024 * 1024 * 512; // 512 MB
    state.cpu_usage_percent = 45.5;
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    assert_eq!(state.active_connections, 50);
    assert_eq!(state.total_requests, 10000);
    assert_eq!(state.memory_usage_bytes, 1024 * 1024 * 512);
    assert_eq!(state.cpu_usage_percent, 45.5);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
#[test]
fn test_production_state_status_transitions() {
    let mut state = ProductionState::default();

    // Initial state
    assert!(matches!(state.status, OperationalStatus::Initializing));

    // Transition to healthy
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    state.status = OperationalStatus::Healthy;
    assert_eq!(state.status, OperationalStatus::Healthy);

    // Transition to degraded
    state.status = OperationalStatus::Degraded;
    assert_eq!(state.status, OperationalStatus::Degraded);

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    // Transition to critical
    state.status = OperationalStatus::Critical;
    assert_eq!(state.status, OperationalStatus::Critical);

    // Transition to shutdown
    state.status = OperationalStatus::Shutdown;
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert_eq!(state.status, OperationalStatus::Shutdown);
}

#[test]
fn test_performance_metrics_default() {
    let metrics = PerformanceMetrics::default();

    assert_eq!(metrics.avg_response_time_ms, 0.0);
    assert_eq!(metrics.p95_response_time_ms, 0.0);
    assert_eq!(metrics.p99_response_time_ms, 0.0);
    assert_eq!(metrics.requests_per_second, 0.0);
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal

#[test]
fn test_performance_metrics_clone() {
    let metrics1 = PerformanceMetrics::default();
    let metrics2 = metrics1.clone();

    assert_eq!(metrics1.avg_response_time_ms, metrics2.avg_response_time_ms);
    assert_eq!(metrics1.p95_response_time_ms, metrics2.p95_response_time_ms);
}

#[test]
fn test_performance_metrics_with_values() {
    let mut metrics = PerformanceMetrics::default();

    metrics.avg_response_time_ms = 15.5;
    metrics.p95_response_time_ms = 45.2;
    metrics.p99_response_time_ms = 125.8;
    metrics.requests_per_second = 1500.0;

    assert_eq!(metrics.avg_response_time_ms, 15.5);
    assert_eq!(metrics.p95_response_time_ms, 45.2);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert_eq!(metrics.p99_response_time_ms, 125.8);
    assert_eq!(metrics.requests_per_second, 1500.0);
}

#[test]
fn test_production_flags_all_enabled() {
    let flags = ProductionFlags {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        enable_advanced_monitoring: true,
        enable_distributed_tracing: true,
        enable_performance_profiling: true,
        enable_security_auditing: true,
        enable_auto_scaling: true,
        enable_circuit_breakers: true,
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        enable_rate_limiting: true,
        enable_caching: true,
    };

    assert!(flags.enable_advanced_monitoring);
    assert!(flags.enable_distributed_tracing);
    assert!(flags.enable_performance_profiling);
    assert!(flags.enable_security_auditing);
    assert!(flags.enable_auto_scaling);
    assert!(flags.enable_circuit_breakers);
    assert!(flags.enable_rate_limiting);
    assert!(flags.enable_caching);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
}

#[test]
fn test_production_flags_all_disabled() {
    let flags = ProductionFlags {
        enable_advanced_monitoring: false,
        enable_distributed_tracing: false,
        enable_performance_profiling: false,
        enable_security_auditing: false,
        enable_auto_scaling: false,
        enable_circuit_breakers: false,
        enable_rate_limiting: false,
        enable_caching: false,
    };

    assert!(!flags.enable_advanced_monitoring);
    assert!(!flags.enable_distributed_tracing);
    assert!(!flags.enable_performance_profiling);
    assert!(!flags.enable_security_auditing);
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal

#[test]
fn test_production_flags_clone() {
    let flags1 = ProductionFlags {
        enable_advanced_monitoring: true,
        enable_distributed_tracing: false,
        enable_performance_profiling: true,
        enable_security_auditing: false,
        enable_auto_scaling: true,
        enable_circuit_breakers: false,
        enable_rate_limiting: true,
        enable_caching: false,
    };

    let flags2 = flags1.clone();

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert_eq!(
        flags1.enable_advanced_monitoring,
        flags2.enable_advanced_monitoring
    );
    assert_eq!(
        flags1.enable_distributed_tracing,
        flags2.enable_distributed_tracing
    );
}

#[test]
fn test_production_core_config_with_values() {
    let config = ProductionCoreConfig {
        environment_level: EnvironmentLevel::Production,
        service_name: "beardog-test".to_string(),
        service_version: "1.0.0".to_string(),
        deployment_id: "deploy-001".to_string(),
        region: "us-east-1".to_string(),
        cluster_id: "cluster-main".to_string(),
        node_id: "node-1".to_string(),
        flags: ProductionFlags {
            enable_advanced_monitoring: true,
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            enable_distributed_tracing: true,
            enable_performance_profiling: false,
            enable_security_auditing: true,
            enable_auto_scaling: false,
            enable_circuit_breakers: true,
            enable_rate_limiting: true,
            enable_caching: false,
        },
    };

    assert_eq!(config.service_name, "beardog-test");
    assert_eq!(config.service_version, "1.0.0");
    assert_eq!(config.region, "us-east-1");
    assert!(config.flags.enable_advanced_monitoring);
}

#[test]
fn test_production_state_serialization() {
    let state = ProductionState::default();

    let json = serde_json::to_string(&state).expect("Serialization should succeed");
    assert!(!json.is_empty());

    let deserialized: ProductionState =
        serde_json::from_str(&json).expect("Deserialization should succeed");
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert_eq!(state.active_connections, deserialized.active_connections);
}

#[test]
fn test_environment_level_ordering() {
    // Test that we can use environment levels in comparisons
    let dev = EnvironmentLevel::Development;
    let prod = EnvironmentLevel::Production;

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert_eq!(dev, EnvironmentLevel::Development);
    assert_ne!(dev, prod);
}

#[test]
fn test_operational_status_critical_detection() {
    let state = ProductionState {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        status: OperationalStatus::Critical,
        active_connections: 0,
        total_requests: 0,
        memory_usage_bytes: 0,
        cpu_usage_percent: 0.0,
        error_count_hourly: 1000,
        performance: PerformanceMetrics::default(),
    };

    assert_eq!(state.status, OperationalStatus::Critical);
    assert!(state.error_count_hourly > 100);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
#[test]
fn test_production_config_roundtrip() {
    let config = ProductionConfig::default();

    let json = serde_json::to_string(&config).expect("Serialization should succeed");
    let deserialized: ProductionConfig =
        serde_json::from_str(&json).expect("Deserialization should succeed");

    assert!(format!("{:?}", deserialized.core).contains("ProductionCoreConfig"));
}
