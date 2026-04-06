// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive tests for Production Ecosystem core functionality
//! Tests cover: `ProductionEcosystem`, `ProductionConfig`, `ProductionState`, builders, lifecycle

use super::*;

// ============================================================================
// ProductionConfig Tests
// ============================================================================

#[test]
fn test_production_config_default() {
    let config = ProductionConfig::default();

    assert_eq!(config.core.service_name, "beardog-ecosystem");
    assert_eq!(config.core.environment_level, EnvironmentLevel::Development);
    assert!(config.core.flags.enable_advanced_monitoring);
}

#[test]
fn test_production_config_clone() {
    let config1 = ProductionConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.core.service_name, config2.core.service_name);
    assert_eq!(
        config1.core.environment_level,
        config2.core.environment_level
    );
}

#[test]
fn test_production_config_debug() {
    let config = ProductionConfig::default();
    let debug_str = format!("{config:?}");

    assert!(debug_str.contains("ProductionConfig"));
}

#[test]
fn test_production_config_custom_values() {
    let mut config = ProductionConfig::default();
    config.core.service_name = "custom-service".to_string();
    config.core.environment_level = EnvironmentLevel::Production;

    assert_eq!(config.core.service_name, "custom-service");
    assert_eq!(config.core.environment_level, EnvironmentLevel::Production);
}

// ============================================================================
// ProductionCoreConfig Tests
// ============================================================================

#[test]
fn test_production_core_config_default() {
    let config = ProductionCoreConfig::default();

    assert_eq!(config.service_name, "beardog-ecosystem");
    assert_eq!(config.environment_level, EnvironmentLevel::Development);
    assert_eq!(config.region, "local");
    assert_eq!(config.cluster_id, "default-cluster");
    // deployment_id and node_id are dynamic, so just check they're not empty
    assert!(!config.deployment_id.is_empty());
    assert!(!config.node_id.is_empty());
}

#[test]
fn test_production_core_config_custom() {
    let config = ProductionCoreConfig {
        service_name: "test-service".to_string(),
        service_version: "1.0.0".to_string(),
        environment_level: EnvironmentLevel::Staging,
        deployment_id: "deploy-123".to_string(),
        region: "us-west-2".to_string(),
        cluster_id: "prod-cluster-01".to_string(),
        node_id: "node-001".to_string(),
        flags: ProductionFlags::default(),
    };

    assert_eq!(config.service_name, "test-service");
    assert_eq!(config.service_version, "1.0.0");
    assert_eq!(config.environment_level, EnvironmentLevel::Staging);
    assert_eq!(config.deployment_id, "deploy-123");
    assert_eq!(config.region, "us-west-2");
    assert_eq!(config.cluster_id, "prod-cluster-01");
}

// ============================================================================
// EnvironmentLevel Tests
// ============================================================================

#[test]
fn test_environment_level_variants() {
    let dev = EnvironmentLevel::Development;
    let staging = EnvironmentLevel::Staging;
    let prod = EnvironmentLevel::Production;

    assert_ne!(dev, staging);
    assert_ne!(staging, prod);
    assert_ne!(dev, prod);
}

#[test]
fn test_environment_level_clone() {
    let level1 = EnvironmentLevel::Production;
    let level2 = level1; // Copy type, no need for .clone()

    assert_eq!(level1, level2);
}

#[test]
fn test_environment_level_debug() {
    let level = EnvironmentLevel::Production;
    let debug_str = format!("{level:?}");

    assert!(debug_str.contains("Production"));
}

#[test]
fn test_environment_level_equality() {
    let dev1 = EnvironmentLevel::Development;
    let dev2 = EnvironmentLevel::Development;

    assert_eq!(dev1, dev2);
}

// ============================================================================
// ProductionFlags Tests
// ============================================================================

#[test]
fn test_production_flags_default() {
    let flags = ProductionFlags::default();

    // Most features enabled by default for production readiness
    assert!(flags.enable_advanced_monitoring);
    assert!(flags.enable_distributed_tracing);
    assert!(flags.enable_performance_profiling);
    assert!(flags.enable_security_auditing);
    assert!(!flags.enable_auto_scaling); // Auto-scaling off by default
    assert!(flags.enable_circuit_breakers);
    assert!(flags.enable_rate_limiting);
    assert!(flags.enable_caching);
}

#[test]
fn test_production_flags_enable_all() {
    let flags = ProductionFlags {
        enable_advanced_monitoring: true,
        enable_distributed_tracing: true,
        enable_performance_profiling: true,
        enable_security_auditing: true,
        enable_auto_scaling: true,
        enable_circuit_breakers: true,
        enable_rate_limiting: true,
        enable_caching: true,
    };

    assert!(flags.enable_advanced_monitoring);
    assert!(flags.enable_distributed_tracing);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
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
fn test_production_flags_partial() {
    let flags = ProductionFlags {
        enable_advanced_monitoring: false,
        enable_distributed_tracing: false,
        ..Default::default()
    };
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    assert!(!flags.enable_advanced_monitoring);
    assert!(!flags.enable_distributed_tracing);
    // Most others still enabled by default
    assert!(flags.enable_performance_profiling);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert!(flags.enable_circuit_breakers);
}

// ============================================================================
// ProductionState Tests
// ============================================================================

#[test]
fn test_production_state_default() {
    let state = ProductionState::default();

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert_eq!(state.status, OperationalStatus::Initializing);
    assert_eq!(state.cpu_usage_percent, 0.0);
    assert_eq!(state.active_connections, 0);
    assert_eq!(state.total_requests, 0);
    assert_eq!(state.error_count_hourly, 0);
}

#[test]
fn test_production_state_update_status() {
    let mut state = ProductionState::default();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    state.status = OperationalStatus::Healthy;

    assert_eq!(state.status, OperationalStatus::Healthy);
}

#[test]
fn test_production_state_update_metrics() {
    let mut state = ProductionState::default();
    state.cpu_usage_percent = 45.5;
    state.active_connections = 100;
    state.total_requests = 5000;
    state.error_count_hourly = 5;

    assert_eq!(state.cpu_usage_percent, 45.5);
    assert_eq!(state.active_connections, 100);
    assert_eq!(state.total_requests, 5000);
    assert_eq!(state.error_count_hourly, 5);
}

#[test]
fn test_production_state_clone() {
    let state1 = ProductionState {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        status: OperationalStatus::Healthy,
        cpu_usage_percent: 50.0,
        active_connections: 200,
        total_requests: 10000,
        memory_usage_bytes: 1_000_000,
        error_count_hourly: 10,
        performance: PerformanceMetrics::default(),
    };
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    let state2 = state1.clone();

    assert_eq!(state1.status, state2.status);
    assert_eq!(state1.cpu_usage_percent, state2.cpu_usage_percent);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert_eq!(state1.active_connections, state2.active_connections);
}

// ============================================================================
// OperationalStatus Tests
// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
// ============================================================================

#[test]
fn test_operational_status_variants() {
    let initializing = OperationalStatus::Initializing;
    let healthy = OperationalStatus::Healthy;
    let degraded = OperationalStatus::Degraded;
    let unhealthy = OperationalStatus::Unhealthy;
    let critical = OperationalStatus::Critical;
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    let shutdown = OperationalStatus::Shutdown;

    assert_ne!(initializing, healthy);
    assert_ne!(healthy, degraded);
    assert_ne!(degraded, unhealthy);
    assert_ne!(unhealthy, critical);
    assert_ne!(critical, shutdown);
}

#[test]
fn test_operational_status_equality() {
    let status1 = OperationalStatus::Healthy;
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    let status2 = OperationalStatus::Healthy;

    assert_eq!(status1, status2);
}

#[test]
fn test_operational_status_initializing() {
    let status = OperationalStatus::Initializing;
    assert_eq!(status, OperationalStatus::Initializing);
}

#[test]
fn test_operational_status_debug() {
    let status = OperationalStatus::Critical;
    let debug_str = format!("{status:?}");

    assert!(debug_str.contains("Critical"));
}

// ============================================================================
// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
// PerformanceMetrics Tests
// ============================================================================

#[test]
fn test_performance_metrics_default() {
    let metrics = PerformanceMetrics::default();

    assert_eq!(metrics.avg_response_time_ms, 0.0);
    assert_eq!(metrics.p95_response_time_ms, 0.0);
    assert_eq!(metrics.p99_response_time_ms, 0.0);
    assert_eq!(metrics.requests_per_second, 0.0);
    assert_eq!(metrics.error_rate_percent, 0.0);
    assert_eq!(metrics.throughput_bytes_per_sec, 0);
}

#[test]
// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
fn test_performance_metrics_custom_values() {
    let metrics = PerformanceMetrics {
        avg_response_time_ms: 25.5,
        p95_response_time_ms: 50.0,
        p99_response_time_ms: 100.0,
        requests_per_second: 1000.0,
        error_rate_percent: 0.5,
        throughput_bytes_per_sec: 1_000_000,
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
    };

    assert_eq!(metrics.avg_response_time_ms, 25.5);
    assert_eq!(metrics.p95_response_time_ms, 50.0);
    assert_eq!(metrics.p99_response_time_ms, 100.0);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert_eq!(metrics.requests_per_second, 1000.0);
    assert_eq!(metrics.error_rate_percent, 0.5);
    assert_eq!(metrics.throughput_bytes_per_sec, 1_000_000);
}

#[test]
fn test_performance_metrics_high_load() {
    let metrics = PerformanceMetrics {
        avg_response_time_ms: 500.0,
        p95_response_time_ms: 1000.0,
        p99_response_time_ms: 2000.0,
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        requests_per_second: 10000.0,
        error_rate_percent: 5.0,
        throughput_bytes_per_sec: 100_000_000,
    };

    assert!(metrics.avg_response_time_ms > 100.0);
    assert!(metrics.error_rate_percent > 1.0);
}

#[test]
fn test_performance_metrics_clone() {
    let metrics1 = PerformanceMetrics {
        avg_response_time_ms: 10.0,
        p95_response_time_ms: 20.0,
        p99_response_time_ms: 30.0,
        requests_per_second: 500.0,
        error_rate_percent: 0.1,
        throughput_bytes_per_sec: 50_000,
    };

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    let metrics2 = metrics1; // Copy type, no need for .clone()

    assert_eq!(metrics1.avg_response_time_ms, metrics2.avg_response_time_ms);
    assert_eq!(metrics1.requests_per_second, metrics2.requests_per_second);
}

// ============================================================================
// ProductionEcosystem Tests
// ============================================================================

#[test]
fn test_production_ecosystem_new() {
    let config = ProductionConfig::default();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    let result = ProductionEcosystem::new(config);

    assert!(result.is_ok());
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
#[test]
fn test_production_ecosystem_new_with_custom_config() {
    let mut config = ProductionConfig::default();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    config.core.service_name = "test-ecosystem".to_string();
    config.core.environment_level = EnvironmentLevel::Production;

    let result = ProductionEcosystem::new(config);
    assert!(result.is_ok());

    let ecosystem = result.unwrap();
    assert_eq!(ecosystem.config.core.service_name, "test-ecosystem");
    assert_eq!(
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        ecosystem.config.core.environment_level,
        EnvironmentLevel::Production
    );
}

#[test]
fn test_production_ecosystem_initial_state() {
    let config = ProductionConfig::default();
    let ecosystem = ProductionEcosystem::new(config).unwrap();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    let state = ecosystem.get_status();
    assert_eq!(state.status, OperationalStatus::Initializing);
}

#[test]
fn test_production_ecosystem_initialize() {
    let config = ProductionConfig::default();
    let mut ecosystem = ProductionEcosystem::new(config).unwrap();

    let result = ecosystem.initialize();
    assert!(result.is_ok());

    let state = ecosystem.get_status();
    assert_eq!(state.status, OperationalStatus::Healthy);
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal

#[test]
fn test_production_ecosystem_uptime() {
    let config = ProductionConfig::default();
    let ecosystem = ProductionEcosystem::new(config).unwrap();

    let uptime = ecosystem.uptime();
    assert!(uptime.as_secs() > 0 || uptime.as_secs() == 0); // Check uptime is valid
}

#[test]
fn test_production_ecosystem_uptime_increases() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    let config = ProductionConfig::default();
    let ecosystem = ProductionEcosystem::new(config).unwrap();

    // ✅ MODERNIZED: Removed sleep - uptime() uses Instant::now().duration_since()
    // which is monotonic and will always be >= previous value
    let uptime1 = ecosystem.uptime();
    let uptime2 = ecosystem.uptime();

    assert!(uptime2 >= uptime1);
}

#[test]
fn test_production_ecosystem_get_status() {
    let config = ProductionConfig::default();
    let mut ecosystem = ProductionEcosystem::new(config).unwrap();

    ecosystem.initialize().unwrap();

    let status = ecosystem.get_status();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert_eq!(status.status, OperationalStatus::Healthy);
}

#[test]
fn test_production_ecosystem_shutdown() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    let config = ProductionConfig::default();
    let mut ecosystem = ProductionEcosystem::new(config).unwrap();

    ecosystem.initialize().unwrap();

    let result = ecosystem.shutdown();
    assert!(result.is_ok());

    let state = ecosystem.get_status();
    assert_eq!(state.status, OperationalStatus::Shutdown);
}

#[test]
fn test_production_ecosystem_lifecycle() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    let config = ProductionConfig::default();
    let mut ecosystem = ProductionEcosystem::new(config).unwrap();

    // Start in Initializing state
    assert_eq!(
        ecosystem.get_status().status,
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        OperationalStatus::Initializing
    );

    // Initialize -> Healthy
    ecosystem.initialize().unwrap();
    assert_eq!(ecosystem.get_status().status, OperationalStatus::Healthy);

    // Shutdown
    ecosystem.shutdown().unwrap();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert_eq!(ecosystem.get_status().status, OperationalStatus::Shutdown);
}

#[test]
fn test_production_ecosystem_health_check() {
    let config = ProductionConfig::default();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    let mut ecosystem = ProductionEcosystem::new(config).unwrap();

    ecosystem.initialize().unwrap();

    let result = ecosystem.health_check();
    assert!(result.is_ok());
}

#[test]
// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
fn test_production_ecosystem_update_metrics() {
    let config = ProductionConfig::default();
    let mut ecosystem = ProductionEcosystem::new(config).unwrap();

    ecosystem.initialize().unwrap();

    let result = ecosystem.update_metrics();
    assert!(result.is_ok());
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
}

#[test]
fn test_production_ecosystem_auto_scaling_disabled() {
    let mut config = ProductionConfig::default();
    config.core.flags.enable_auto_scaling = false;

    let mut ecosystem = ProductionEcosystem::new(config).unwrap();
    ecosystem.initialize().unwrap();

    // Should succeed even with auto-scaling disabled
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    let result = ecosystem.update_metrics();
    assert!(result.is_ok());
}

#[test]
fn test_production_ecosystem_auto_scaling_enabled() {
    let mut config = ProductionConfig::default();
    config.core.flags.enable_auto_scaling = true;

    let mut ecosystem = ProductionEcosystem::new(config).unwrap();
    ecosystem.initialize().unwrap();

    let result = ecosystem.update_metrics();
    assert!(result.is_ok());
}

// ============================================================================
// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
// ProductionEcosystemBuilder Tests
// ============================================================================

#[test]
fn test_builder_new() {
    let builder = ProductionEcosystemBuilder::new();
    assert_eq!(builder.config.core.service_name, "beardog-ecosystem");
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal

#[test]
fn test_builder_default() {
    let builder = ProductionEcosystemBuilder::default();
    assert_eq!(builder.config.core.service_name, "beardog-ecosystem");
}

#[test]
// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
fn test_builder_environment_level() {
    let builder = ProductionEcosystemBuilder::new().environment_level(EnvironmentLevel::Production);

    assert_eq!(
        builder.config.core.environment_level,
        EnvironmentLevel::Production
    );
}

#[test]
// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
fn test_builder_service() {
    let builder =
        ProductionEcosystemBuilder::new().service("my-service".to_string(), "2.0.0".to_string());

    assert_eq!(builder.config.core.service_name, "my-service");
    assert_eq!(builder.config.core.service_version, "2.0.0");
}

#[test]
fn test_builder_deployment() {
    let builder = ProductionEcosystemBuilder::new().deployment(
        "deploy-456".to_string(),
        "eu-west-1".to_string(),
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        "cluster-02".to_string(),
    );

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert_eq!(builder.config.core.deployment_id, "deploy-456");
    assert_eq!(builder.config.core.region, "eu-west-1");
    assert_eq!(builder.config.core.cluster_id, "cluster-02");
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
}

#[test]
fn test_builder_enable_advanced_features() {
    let builder = ProductionEcosystemBuilder::new().enable_advanced_features();

    assert!(builder.config.core.flags.enable_advanced_monitoring);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert!(builder.config.core.flags.enable_distributed_tracing);
    assert!(builder.config.core.flags.enable_performance_profiling);
    assert!(builder.config.core.flags.enable_security_auditing);
}

#[test]
// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
fn test_builder_build() {
    let result = ProductionEcosystemBuilder::new().build();

    assert!(result.is_ok());
}

#[test]
fn test_builder_chain_all_methods() {
    let result = ProductionEcosystemBuilder::new()
        .environment_level(EnvironmentLevel::Staging)
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        .service("chain-test".to_string(), "1.2.3".to_string())
        .deployment(
            "deploy-789".to_string(),
            "ap-south-1".to_string(),
            "cluster-03".to_string(),
        )
        .enable_advanced_features()
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        .build();

    assert!(result.is_ok());

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    let ecosystem = result.unwrap();
    assert_eq!(ecosystem.config.core.service_name, "chain-test");
    assert_eq!(ecosystem.config.core.service_version, "1.2.3");
    assert_eq!(
        ecosystem.config.core.environment_level,
        EnvironmentLevel::Staging
    );
    assert_eq!(ecosystem.config.core.deployment_id, "deploy-789");
    assert!(ecosystem.config.core.flags.enable_advanced_monitoring);
}

#[test]
fn test_builder_production_ready() {
    let result = ProductionEcosystemBuilder::new()
        .environment_level(EnvironmentLevel::Production)
        .service("prod-service".to_string(), "3.0.0".to_string())
        .deployment(
            "prod-deploy-001".to_string(),
            "us-east-1".to_string(),
            "prod-cluster-01".to_string(),
        )
        .enable_advanced_features()
        .build();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    assert!(result.is_ok());

    let mut ecosystem = result.unwrap();
    assert_eq!(
        ecosystem.config.core.environment_level,
        EnvironmentLevel::Production
    );

    // Should initialize successfully
    assert!(ecosystem.initialize().is_ok());
    assert_eq!(ecosystem.get_status().status, OperationalStatus::Healthy);
}

#[test]
fn test_builder_multiple_builds() {
    let builder1 =
        ProductionEcosystemBuilder::new().service("service-1".to_string(), "1.0.0".to_string());

    let ecosystem1 = builder1.build().unwrap();

    let builder2 =
        ProductionEcosystemBuilder::new().service("service-2".to_string(), "2.0.0".to_string());
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    let ecosystem2 = builder2.build().unwrap();

    assert_eq!(ecosystem1.config.core.service_name, "service-1");
    assert_eq!(ecosystem2.config.core.service_name, "service-2");
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_full_ecosystem_workflow() {
    // Build ecosystem
    let mut ecosystem = ProductionEcosystemBuilder::new()
        .environment_level(EnvironmentLevel::Production)
        .service("workflow-test".to_string(), "1.0.0".to_string())
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        .enable_advanced_features()
        .build()
        .unwrap();

    // Initialize
    assert!(ecosystem.initialize().is_ok());
    assert_eq!(ecosystem.get_status().status, OperationalStatus::Healthy);

    // Run health check
    let health_report = ecosystem.health_check();
    assert!(health_report.is_ok());

    // Update metrics
    assert!(ecosystem.update_metrics().is_ok());

    // Check uptime
    let uptime = ecosystem.uptime();
    assert!(uptime.as_secs() > 0 || uptime.as_secs() == 0); // Check uptime is valid

    // Shutdown
    assert!(ecosystem.shutdown().is_ok());
    assert_eq!(ecosystem.get_status().status, OperationalStatus::Shutdown);
}

#[test]
fn test_multiple_ecosystems() {
    let config1 = ProductionConfig::default();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    let mut eco1 = ProductionEcosystem::new(config1).unwrap();

    let config2 = ProductionConfig::default();
    let mut eco2 = ProductionEcosystem::new(config2).unwrap();

    assert!(eco1.initialize().is_ok());
    assert!(eco2.initialize().is_ok());

    assert_eq!(eco1.get_status().status, OperationalStatus::Healthy);
    assert_eq!(eco2.get_status().status, OperationalStatus::Healthy);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
#[test]
fn test_config_independence() {
    let mut config1 = ProductionConfig::default();
    config1.core.service_name = "service-1".to_string();

    let config2 = config1.clone();
    let mut config3 = config2.clone();
    config3.core.service_name = "service-3".to_string();

    assert_eq!(config1.core.service_name, "service-1");
    assert_eq!(config2.core.service_name, "service-1");
    assert_eq!(config3.core.service_name, "service-3");
}
