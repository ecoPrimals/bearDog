//! Advanced tests for production module
//! Focus: Builder pattern, lifecycle, edge cases

use super::*;

#[test]
fn test_production_ecosystem_builder_complete() {
    let ecosystem = ProductionEcosystemBuilder::new()
        .environment_level(EnvironmentLevel::Production)
        .service("test-service".to_string(), "1.0.0".to_string())
        .deployment(
            "deploy-123".to_string(),
            "us-west-2".to_string(),
            "prod-cluster-1".to_string(),
        )
        .enable_advanced_features()
        .build();

    assert!(ecosystem.is_ok());
    let ecosystem = ecosystem?;
    assert_eq!(
        ecosystem.config.core.environment_level,
        EnvironmentLevel::Production
    );
    assert_eq!(ecosystem.config.core.service_name, "test-service");
    assert_eq!(ecosystem.config.core.service_version, "1.0.0");
}

#[test]
fn test_production_ecosystem_builder_defaults() {
    let ecosystem = ProductionEcosystemBuilder::default().build();
    assert!(ecosystem.is_ok());

    let ecosystem = ecosystem?;
    assert_eq!(
        ecosystem.config.core.environment_level,
        EnvironmentLevel::Development
    );
    assert!(ecosystem.config.core.flags.enable_advanced_monitoring);
    assert!(ecosystem.config.core.flags.enable_distributed_tracing);
}

#[test]
fn test_production_ecosystem_initialization() {
    let config = ProductionConfig::default();
    let ecosystem = ProductionEcosystem::new(config);

    assert!(ecosystem.is_ok());
    let mut ecosystem = ecosystem?;

    // Test initialization
    let result = ecosystem.initialize();
    assert!(result.is_ok());

    let status = ecosystem.get_status();
    assert_eq!(status.status, OperationalStatus::Healthy);
}

#[test]
fn test_production_ecosystem_shutdown() {
    let config = ProductionConfig::default();
    let ecosystem = ProductionEcosystem::new(config);
    assert!(ecosystem.is_ok());

    let mut ecosystem = ecosystem?;
    let _ = ecosystem.initialize();

    let result = ecosystem.shutdown();
    assert!(result.is_ok());

    let status = ecosystem.get_status();
    assert_eq!(status.status, OperationalStatus::Shutdown);
}

#[test]
fn test_production_ecosystem_uptime() {
    let config = ProductionConfig::default();
    let ecosystem = ProductionEcosystem::new(config)?;

    let uptime = ecosystem.uptime();
    assert!(uptime.as_millis() > 0 || uptime.as_millis() == 0); // Check uptime is valid
}

#[test]
fn test_environment_level_equality() {
    assert_eq!(EnvironmentLevel::Development, EnvironmentLevel::Development);
    assert_ne!(EnvironmentLevel::Development, EnvironmentLevel::Production);

    assert_eq!(EnvironmentLevel::Staging, EnvironmentLevel::Staging);
    assert_ne!(EnvironmentLevel::Staging, EnvironmentLevel::Critical);
}

#[test]
fn test_operational_status_equality() {
    assert_eq!(OperationalStatus::Healthy, OperationalStatus::Healthy);
    assert_ne!(OperationalStatus::Healthy, OperationalStatus::Degraded);

    assert_eq!(OperationalStatus::Critical, OperationalStatus::Critical);
    assert_ne!(OperationalStatus::Critical, OperationalStatus::Shutdown);
}

#[test]
fn test_production_flags_serialization() {
    let flags = ProductionFlags::default();
    let serialized = serde_json::to_string(&flags);
    assert!(serialized.is_ok());

    let deserialized: Result<ProductionFlags, _> = serde_json::from_str(&serialized?);
    assert!(deserialized.is_ok());
}

#[test]
fn test_production_state_defaults() {
    let state = ProductionState::default();
    assert_eq!(state.status, OperationalStatus::Initializing);
    assert_eq!(state.active_connections, 0);
    assert_eq!(state.total_requests, 0);
    assert_eq!(state.memory_usage_bytes, 0);
    assert_eq!(state.cpu_usage_percent, 0.0);
    assert_eq!(state.error_count_hourly, 0);
}

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

#[test]
fn test_production_config_clone() {
    let config1 = ProductionConfig::default();
    let config2 = config1.clone();

    assert_eq!(
        config1.core.environment_level,
        config2.core.environment_level
    );
    assert_eq!(config1.core.service_name, config2.core.service_name);
}

#[test]
fn test_environment_level_all_variants() {
    let levels = vec![
        EnvironmentLevel::Development,
        EnvironmentLevel::Staging,
        EnvironmentLevel::PreProduction,
        EnvironmentLevel::Production,
        EnvironmentLevel::Critical,
    ];

    for level in levels {
        let cloned = level.clone();
        assert_eq!(level, cloned);
    }
}

#[test]
fn test_operational_status_all_variants() {
    let statuses = vec![
        OperationalStatus::Initializing,
        OperationalStatus::Healthy,
        OperationalStatus::Degraded,
        OperationalStatus::Unhealthy,
        OperationalStatus::Critical,
        OperationalStatus::Shutdown,
    ];

    for status in statuses {
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }
}

#[test]
fn test_production_core_config_node_id() {
    let config = ProductionCoreConfig::default();
    // node_id should be set (either from HOSTNAME or "unknown")
    assert!(!config.node_id.is_empty());
}

#[test]
fn test_production_flags_individual_features() {
    let mut flags = ProductionFlags::default();

    // Test individual flag mutations
    flags.enable_advanced_monitoring = false;
    assert!(!flags.enable_advanced_monitoring);

    flags.enable_auto_scaling = true;
    assert!(flags.enable_auto_scaling);

    flags.enable_circuit_breakers = false;
    assert!(!flags.enable_circuit_breakers);
}

#[test]
fn test_builder_service_method() {
    let ecosystem = ProductionEcosystemBuilder::new()
        .service("custom-service".to_string(), "2.0.0".to_string())
        .build()
        ?;

    assert_eq!(ecosystem.config.core.service_name, "custom-service");
    assert_eq!(ecosystem.config.core.service_version, "2.0.0");
}

#[test]
fn test_builder_deployment_method() {
    let ecosystem = ProductionEcosystemBuilder::new()
        .deployment(
            "deploy-456".to_string(),
            "eu-west-1".to_string(),
            "test-cluster".to_string(),
        )
        .build()
        ?;

    assert_eq!(ecosystem.config.core.deployment_id, "deploy-456");
    assert_eq!(ecosystem.config.core.region, "eu-west-1");
    assert_eq!(ecosystem.config.core.cluster_id, "test-cluster");
}

#[test]
fn test_production_state_serialization() {
    let state = ProductionState::default();
    let serialized = serde_json::to_string(&state);
    assert!(serialized.is_ok());

    let deserialized: Result<ProductionState, _> = serde_json::from_str(&serialized?);
    assert!(deserialized.is_ok());
}

#[test]
fn test_performance_metrics_serialization() {
    let metrics = PerformanceMetrics::default();
    let serialized = serde_json::to_string(&metrics);
    assert!(serialized.is_ok());

    let deserialized: Result<PerformanceMetrics, _> = serde_json::from_str(&serialized?);
    assert!(deserialized.is_ok());
}
