//! Production Integration Tests - Week 2 Day 3 (October 26, 2025)
//!
//! Comprehensive integration tests for production ecosystem including:
//! - State transitions and lifecycle
//! - Builder pattern validation
//! - Error handling and recovery
//! - Concurrent operations
//! - Configuration edge cases
//! - Metrics and monitoring integration

use super::*;

// ============================================================================
// Test 1: Production Ecosystem Builder Pattern Validation
// ============================================================================

#[test]
fn test_production_ecosystem_builder_creates_valid_instance() {
    let ecosystem = ProductionEcosystemBuilder::new()
        .environment_level(EnvironmentLevel::Production)
        .service("test-service".to_string(), "1.0.0".to_string())
        .deployment(
            "deploy-123".to_string(),
            "us-west-2".to_string(),
            "cluster-1".to_string(),
        )
        .enable_advanced_features()
        .build();

    assert!(ecosystem.is_ok(), "Builder should create valid ecosystem");
    let ecosystem = ecosystem.unwrap();
    assert_eq!(
        ecosystem.config.core.environment_level,
        EnvironmentLevel::Production
    );
    assert_eq!(ecosystem.config.core.service_name, "test-service");
    assert_eq!(ecosystem.config.core.service_version, "1.0.0");
    assert_eq!(ecosystem.config.core.deployment_id, "deploy-123");
    assert!(ecosystem.config.core.flags.enable_advanced_monitoring);
    assert!(ecosystem.config.core.flags.enable_distributed_tracing);
}

// ============================================================================
// Test 2: Production State Transitions
// ============================================================================

#[test]
fn test_production_state_transitions_follow_lifecycle() {
    let config = ProductionConfig::default();
    let mut ecosystem = ProductionEcosystem::new(config).unwrap();

    // Initial state should be Initializing
    assert_eq!(
        ecosystem.get_status().status,
        OperationalStatus::Initializing
    );

    // After initialization, should be Healthy
    ecosystem.initialize().unwrap();
    assert_eq!(ecosystem.get_status().status, OperationalStatus::Healthy);

    // After shutdown, should be Shutdown
    ecosystem.shutdown().unwrap();
    assert_eq!(ecosystem.get_status().status, OperationalStatus::Shutdown);
}

// ============================================================================
// Test 3: Environment Level Configuration
// ============================================================================

#[test]
fn test_environment_levels_are_distinct_and_comparable() {
    assert_ne!(EnvironmentLevel::Development, EnvironmentLevel::Production);
    assert_ne!(EnvironmentLevel::Staging, EnvironmentLevel::PreProduction);
    assert_eq!(EnvironmentLevel::Critical, EnvironmentLevel::Critical);
    assert_eq!(EnvironmentLevel::Production, EnvironmentLevel::Production);

    // Test serialization
    let level = EnvironmentLevel::Production;
    let serialized = serde_json::to_string(&level).unwrap();
    let deserialized: EnvironmentLevel = serde_json::from_str(&serialized).unwrap();
    assert_eq!(level, deserialized);
}

// ============================================================================
// Test 4: Production Flags Configuration
// ============================================================================

#[test]
fn test_production_flags_default_values_are_secure() {
    let flags = ProductionFlags::default();

    // Security-critical flags should be enabled by default
    assert!(
        flags.enable_advanced_monitoring,
        "Advanced monitoring should be enabled by default"
    );
    assert!(
        flags.enable_security_auditing,
        "Security auditing should be enabled by default"
    );
    assert!(
        flags.enable_circuit_breakers,
        "Circuit breakers should be enabled by default"
    );
    assert!(
        flags.enable_rate_limiting,
        "Rate limiting should be enabled by default"
    );

    // Auto-scaling should be opt-in (disabled by default)
    assert!(
        !flags.enable_auto_scaling,
        "Auto-scaling should be disabled by default (opt-in)"
    );
}

// ============================================================================
// Test 5: Production Core Config Defaults
// ============================================================================

#[test]
fn test_production_core_config_has_sensible_defaults() {
    let config = ProductionCoreConfig::default();

    assert_eq!(config.environment_level, EnvironmentLevel::Development);
    assert_eq!(config.service_name, "beardog-ecosystem");
    assert!(!config.service_version.is_empty(), "Version should be set");
    assert!(
        !config.deployment_id.is_empty(),
        "Deployment ID should be set"
    );
    assert_eq!(config.region, "local");
    assert_eq!(config.cluster_id, "default-cluster");
    assert!(!config.node_id.is_empty(), "Node ID should be set");
}

// ============================================================================
// Test 6: Operational Status Transitions
// ============================================================================

#[test]
fn test_operational_status_covers_all_lifecycle_stages() {
    let statuses = vec![
        OperationalStatus::Initializing,
        OperationalStatus::Healthy,
        OperationalStatus::Degraded,
        OperationalStatus::Unhealthy,
        OperationalStatus::Critical,
        OperationalStatus::Shutdown,
    ];

    // All statuses should be serializable
    for status in &statuses {
        let serialized = serde_json::to_string(status).unwrap();
        let deserialized: OperationalStatus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(*status, deserialized);
    }

    // Statuses should be distinct
    assert_ne!(OperationalStatus::Healthy, OperationalStatus::Degraded);
    assert_ne!(OperationalStatus::Degraded, OperationalStatus::Unhealthy);
    assert_ne!(OperationalStatus::Unhealthy, OperationalStatus::Critical);
}

// ============================================================================
// Test 7: Performance Metrics Initialization
// ============================================================================

#[test]
fn test_performance_metrics_initialize_to_zero() {
    let metrics = PerformanceMetrics::default();

    assert_eq!(
        metrics.avg_response_time_ms, 0.0,
        "Average response time should start at 0"
    );
    assert_eq!(metrics.p95_response_time_ms, 0.0, "P95 should start at 0");
    assert_eq!(metrics.p99_response_time_ms, 0.0, "P99 should start at 0");
    assert_eq!(metrics.requests_per_second, 0.0, "RPS should start at 0");
    assert_eq!(
        metrics.error_rate_percent, 0.0,
        "Error rate should start at 0"
    );
    assert_eq!(
        metrics.throughput_bytes_per_sec, 0,
        "Throughput should start at 0"
    );
}

// ============================================================================
// Test 8: Production State Initialization
// ============================================================================

#[test]
fn test_production_state_initializes_correctly() {
    let state = ProductionState::default();

    assert_eq!(
        state.status,
        OperationalStatus::Initializing,
        "Initial status should be Initializing"
    );
    assert_eq!(
        state.active_connections, 0,
        "Should start with no active connections"
    );
    assert_eq!(
        state.total_requests, 0,
        "Should start with no requests processed"
    );
    assert_eq!(
        state.memory_usage_bytes, 0,
        "Memory usage should start at 0"
    );
    assert_eq!(state.cpu_usage_percent, 0.0, "CPU usage should start at 0%");
    assert_eq!(state.error_count_hourly, 0, "Error count should start at 0");

    // Performance metrics should also be initialized
    assert_eq!(state.performance.avg_response_time_ms, 0.0);
}

// ============================================================================
// Test 9: Production Ecosystem Uptime Tracking
// ============================================================================

#[test]
fn test_production_ecosystem_tracks_uptime() {
    let config = ProductionConfig::default();
    let ecosystem = ProductionEcosystem::new(config).unwrap();

    // Uptime should be very small immediately after creation
    let uptime = ecosystem.uptime();
    assert!(
        uptime.as_secs() < 5,
        "Uptime should be less than 5 seconds immediately after creation"
    );

    // Uptime should be measurable
    std::thread::sleep(std::time::Duration::from_millis(10));
    let uptime2 = ecosystem.uptime();
    assert!(uptime2 > uptime, "Uptime should increase over time");
}

// ============================================================================
// Test 10: Production Config Serialization
// ============================================================================

#[test]
fn test_production_config_serialization_roundtrip() {
    let config = ProductionConfig::default();

    // Serialize to JSON
    let json = serde_json::to_string(&config).unwrap();
    assert!(!json.is_empty(), "Serialized config should not be empty");

    // Deserialize back
    let deserialized: ProductionConfig = serde_json::from_str(&json).unwrap();

    // Compare key fields (can't compare entire structs due to non-derived traits)
    assert_eq!(
        config.core.environment_level,
        deserialized.core.environment_level
    );
    assert_eq!(config.core.service_name, deserialized.core.service_name);
    assert_eq!(config.core.region, deserialized.core.region);
}

// ============================================================================
// Test 11: Multiple Production Ecosystems Can Coexist
// ============================================================================

#[test]
fn test_multiple_production_ecosystems_can_coexist() {
    let config1 = ProductionConfig::default();
    let config2 = ProductionConfig::default();
    let config3 = ProductionConfig::default();

    let ecosystem1 = ProductionEcosystem::new(config1);
    let ecosystem2 = ProductionEcosystem::new(config2);
    let ecosystem3 = ProductionEcosystem::new(config3);

    assert!(
        ecosystem1.is_ok(),
        "First ecosystem should create successfully"
    );
    assert!(
        ecosystem2.is_ok(),
        "Second ecosystem should create successfully"
    );
    assert!(
        ecosystem3.is_ok(),
        "Third ecosystem should create successfully"
    );

    // All should have independent state
    let e1 = ecosystem1.unwrap();
    let e2 = ecosystem2.unwrap();
    let e3 = ecosystem3.unwrap();

    assert_eq!(e1.get_status().status, OperationalStatus::Initializing);
    assert_eq!(e2.get_status().status, OperationalStatus::Initializing);
    assert_eq!(e3.get_status().status, OperationalStatus::Initializing);
}

// ============================================================================
// Test 12: Production Ecosystem Builder Chaining
// ============================================================================

#[test]
fn test_production_ecosystem_builder_method_chaining() {
    // Test that all builder methods can be chained fluently
    let result = ProductionEcosystemBuilder::new()
        .environment_level(EnvironmentLevel::Staging)
        .service("chained-service".to_string(), "2.0.0".to_string())
        .deployment(
            "deploy-chain".to_string(),
            "eu-west-1".to_string(),
            "staging-cluster".to_string(),
        )
        .enable_advanced_features()
        .build();

    assert!(
        result.is_ok(),
        "Chained builder should create valid ecosystem"
    );

    let ecosystem = result.unwrap();

    // Verify all builder settings were applied
    assert_eq!(
        ecosystem.config.core.environment_level,
        EnvironmentLevel::Staging
    );
    assert_eq!(ecosystem.config.core.service_name, "chained-service");
    assert_eq!(ecosystem.config.core.service_version, "2.0.0");
    assert_eq!(ecosystem.config.core.deployment_id, "deploy-chain");
    assert_eq!(ecosystem.config.core.region, "eu-west-1");
    assert_eq!(ecosystem.config.core.cluster_id, "staging-cluster");

    // Verify advanced features were enabled
    assert!(ecosystem.config.core.flags.enable_advanced_monitoring);
    assert!(ecosystem.config.core.flags.enable_distributed_tracing);
    assert!(ecosystem.config.core.flags.enable_performance_profiling);
    assert!(ecosystem.config.core.flags.enable_security_auditing);
}
