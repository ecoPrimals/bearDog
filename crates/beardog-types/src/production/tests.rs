//! Tests for Production Module
//!
//! Comprehensive tests for production ecosystem.

use super::*;

// ProductionConfig tests
#[test]
fn test_production_config_default() {
    let config = ProductionConfig::default();
    assert_eq!(config.core.environment_level, EnvironmentLevel::Development);
    assert_eq!(config.core.service_name, "beardog-ecosystem");
}

#[test]
fn test_production_config_clone() {
    let config1 = ProductionConfig::default();
    let config2 = config1.clone();
    assert_eq!(config1.core.service_name, config2.core.service_name);
}

// ProductionCoreConfig tests
#[test]
fn test_production_core_config_default() {
    let core = ProductionCoreConfig::default();
    assert_eq!(core.environment_level, EnvironmentLevel::Development);
    assert_eq!(core.service_name, "beardog-ecosystem");
    assert_eq!(core.region, "local");
    assert_eq!(core.cluster_id, "default-cluster");
}

#[test]
fn test_production_core_config_custom() {
    let core = ProductionCoreConfig {
        environment_level: EnvironmentLevel::Production,
        service_name: "test-service".to_string(),
        service_version: "1.0.0".to_string(),
        deployment_id: "deploy-123".to_string(),
        region: "us-west-2".to_string(),
        cluster_id: "prod-cluster".to_string(),
        node_id: "node-01".to_string(),
        flags: ProductionFlags::default(),
    };

    assert_eq!(core.environment_level, EnvironmentLevel::Production);
    assert_eq!(core.service_name, "test-service");
    assert_eq!(core.service_version, "1.0.0");
    assert_eq!(core.region, "us-west-2");
}

// EnvironmentLevel tests
#[test]
fn test_environment_level_variants() {
    let levels = [
        EnvironmentLevel::Development,
        EnvironmentLevel::Staging,
        EnvironmentLevel::PreProduction,
        EnvironmentLevel::Production,
        EnvironmentLevel::Critical,
    ];

    assert_eq!(levels.len(), 5);
}

#[test]
fn test_environment_level_equality() {
    assert_eq!(EnvironmentLevel::Development, EnvironmentLevel::Development);
    assert_ne!(EnvironmentLevel::Development, EnvironmentLevel::Production);
    assert_eq!(EnvironmentLevel::Production, EnvironmentLevel::Production);
}

#[test]
fn test_environment_level_clone() {
    let level1 = EnvironmentLevel::Production;
    let level2 = level1;
    assert_eq!(level1, level2);
}

// ProductionFlags tests
#[test]
fn test_production_flags_default() {
    let flags = ProductionFlags::default();
    assert!(flags.enable_advanced_monitoring);
    assert!(flags.enable_distributed_tracing);
    assert!(flags.enable_performance_profiling);
    assert!(flags.enable_security_auditing);
    assert!(!flags.enable_auto_scaling);
    assert!(flags.enable_circuit_breakers);
    assert!(flags.enable_rate_limiting);
    assert!(flags.enable_caching);
}

#[test]
fn test_production_flags_custom() {
    let flags = ProductionFlags {
        enable_advanced_monitoring: false,
        enable_distributed_tracing: true,
        enable_performance_profiling: false,
        enable_security_auditing: true,
        enable_auto_scaling: false,
        enable_circuit_breakers: true,
        enable_rate_limiting: false,
        enable_caching: true,
    };

    assert!(!flags.enable_advanced_monitoring);
    assert!(flags.enable_distributed_tracing);
    assert!(!flags.enable_performance_profiling);
}

#[test]
fn test_production_flags_all_enabled() {
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
    assert!(flags.enable_auto_scaling);
}

// ProductionState tests
#[test]
fn test_production_state_default() {
    let state = ProductionState::default();
    assert_eq!(state.status, OperationalStatus::Initializing);
    assert_eq!(state.active_connections, 0);
    assert_eq!(state.total_requests, 0);
    assert_eq!(state.memory_usage_bytes, 0);
    assert_eq!(state.cpu_usage_percent, 0.0);
    assert_eq!(state.error_count_hourly, 0);
}

#[test]
fn test_production_state_custom() {
    let state = ProductionState {
        status: OperationalStatus::Healthy,
        active_connections: 100,
        total_requests: 10000,
        memory_usage_bytes: 1024 * 1024 * 512, // 512 MB
        cpu_usage_percent: 45.5,
        error_count_hourly: 5,
        performance: PerformanceMetrics::default(),
    };

    assert_eq!(state.status, OperationalStatus::Healthy);
    assert_eq!(state.active_connections, 100);
    assert_eq!(state.total_requests, 10000);
    assert_eq!(state.cpu_usage_percent, 45.5);
}

#[test]
fn test_production_state_clone() {
    let state1 = ProductionState::default();
    let state2 = state1.clone();
    assert_eq!(state1.status, state2.status);
    assert_eq!(state1.active_connections, state2.active_connections);
}

// OperationalStatus tests
#[test]
fn test_operational_status_variants() {
    let statuses = [
        OperationalStatus::Initializing,
        OperationalStatus::Healthy,
        OperationalStatus::Degraded,
        OperationalStatus::Unhealthy,
        OperationalStatus::Critical,
        OperationalStatus::Shutdown,
    ];

    assert_eq!(statuses.len(), 6);
}

#[test]
fn test_operational_status_equality() {
    assert_eq!(OperationalStatus::Healthy, OperationalStatus::Healthy);
    assert_ne!(OperationalStatus::Healthy, OperationalStatus::Degraded);
    assert_eq!(OperationalStatus::Critical, OperationalStatus::Critical);
}

#[test]
fn test_operational_status_progression() {
    let initializing = OperationalStatus::Initializing;
    let healthy = OperationalStatus::Healthy;
    let degraded = OperationalStatus::Degraded;
    let unhealthy = OperationalStatus::Unhealthy;
    let critical = OperationalStatus::Critical;

    assert_ne!(initializing, healthy);
    assert_ne!(healthy, degraded);
    assert_ne!(degraded, unhealthy);
    assert_ne!(unhealthy, critical);
}

// PerformanceMetrics tests
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
fn test_performance_metrics_custom() {
    let metrics = PerformanceMetrics {
        avg_response_time_ms: 25.5,
        p95_response_time_ms: 50.0,
        p99_response_time_ms: 100.0,
        requests_per_second: 1000.0,
        error_rate_percent: 0.1,
        throughput_bytes_per_sec: 10 * 1024 * 1024, // 10 MB/s
    };

    assert_eq!(metrics.avg_response_time_ms, 25.5);
    assert_eq!(metrics.p95_response_time_ms, 50.0);
    assert_eq!(metrics.requests_per_second, 1000.0);
}

#[test]
fn test_performance_metrics_sla_check() {
    let good_metrics = PerformanceMetrics {
        avg_response_time_ms: 25.0,
        p95_response_time_ms: 45.0,
        p99_response_time_ms: 95.0,
        requests_per_second: 1500.0,
        error_rate_percent: 0.05,
        throughput_bytes_per_sec: 5 * 1024 * 1024,
    };

    // Check SLA compliance
    assert!(good_metrics.p95_response_time_ms < 100.0);
    assert!(good_metrics.p99_response_time_ms < 500.0);
    assert!(good_metrics.error_rate_percent < 0.1);
}

// ProductionEcosystem tests
#[test]
fn test_production_ecosystem_new() {
    let config = ProductionConfig::default();
    let ecosystem = ProductionEcosystem::new(config);
    assert!(ecosystem.is_ok());
}

#[test]
fn test_production_ecosystem_initialize() {
    let config = ProductionConfig::default();
    let mut ecosystem = ProductionEcosystem::new(config).unwrap();
    let result = ecosystem.initialize();
    assert!(result.is_ok());
    assert_eq!(ecosystem.get_status().status, OperationalStatus::Healthy);
}

#[test]
fn test_production_ecosystem_uptime() {
    let config = ProductionConfig::default();
    let ecosystem = ProductionEcosystem::new(config).unwrap();
    let uptime = ecosystem.uptime();
    // Uptime should be a very small value (microseconds to milliseconds)
    assert!(uptime.as_micros() < 1_000_000); // Less than 1 second
}

#[test]
fn test_production_ecosystem_shutdown() {
    let config = ProductionConfig::default();
    let mut ecosystem = ProductionEcosystem::new(config).unwrap();
    ecosystem.initialize().unwrap();

    let result = ecosystem.shutdown();
    assert!(result.is_ok());
    assert_eq!(ecosystem.get_status().status, OperationalStatus::Shutdown);
}

// ProductionEcosystemBuilder tests
#[test]
fn test_builder_default() {
    let builder = ProductionEcosystemBuilder::default();
    let ecosystem = builder.build();
    assert!(ecosystem.is_ok());
}

#[test]
fn test_builder_environment_level() {
    let ecosystem = ProductionEcosystemBuilder::new()
        .environment_level(EnvironmentLevel::Production)
        .build()
        .unwrap();

    assert_eq!(
        ecosystem.config.core.environment_level,
        EnvironmentLevel::Production
    );
}

#[test]
fn test_builder_service() {
    let ecosystem = ProductionEcosystemBuilder::new()
        .service("test-service".to_string(), "1.0.0".to_string())
        .build()
        .unwrap();

    assert_eq!(ecosystem.config.core.service_name, "test-service");
    assert_eq!(ecosystem.config.core.service_version, "1.0.0");
}

#[test]
fn test_builder_deployment() {
    let ecosystem = ProductionEcosystemBuilder::new()
        .deployment(
            "deploy-123".to_string(),
            "us-west-2".to_string(),
            "prod-cluster".to_string(),
        )
        .build()
        .unwrap();

    assert_eq!(ecosystem.config.core.deployment_id, "deploy-123");
    assert_eq!(ecosystem.config.core.region, "us-west-2");
    assert_eq!(ecosystem.config.core.cluster_id, "prod-cluster");
}

#[test]
fn test_builder_enable_advanced_features() {
    let ecosystem = ProductionEcosystemBuilder::new()
        .enable_advanced_features()
        .build()
        .unwrap();

    assert!(ecosystem.config.core.flags.enable_advanced_monitoring);
    assert!(ecosystem.config.core.flags.enable_distributed_tracing);
    assert!(ecosystem.config.core.flags.enable_performance_profiling);
    assert!(ecosystem.config.core.flags.enable_security_auditing);
}

#[test]
fn test_builder_chaining() {
    let ecosystem = ProductionEcosystemBuilder::new()
        .environment_level(EnvironmentLevel::Production)
        .service("api-service".to_string(), "2.0.0".to_string())
        .deployment(
            "prod-001".to_string(),
            "us-east-1".to_string(),
            "cluster-01".to_string(),
        )
        .enable_advanced_features()
        .build()
        .unwrap();

    assert_eq!(
        ecosystem.config.core.environment_level,
        EnvironmentLevel::Production
    );
    assert_eq!(ecosystem.config.core.service_name, "api-service");
    assert_eq!(ecosystem.config.core.region, "us-east-1");
    assert!(ecosystem.config.core.flags.enable_advanced_monitoring);
}

// Serialization tests
#[test]
fn test_production_state_serialization() {
    let state = ProductionState::default();
    let json = serde_json::to_string(&state);
    assert!(json.is_ok());
}

#[test]
fn test_performance_metrics_serialization() {
    let metrics = PerformanceMetrics::default();
    let json = serde_json::to_string(&metrics);
    assert!(json.is_ok());
}

#[test]
fn test_operational_status_serialization() {
    let status = OperationalStatus::Healthy;
    let json = serde_json::to_string(&status);
    assert!(json.is_ok());
}

// Integration tests
#[test]
fn test_full_lifecycle() {
    let config = ProductionConfig::default();
    let mut ecosystem = ProductionEcosystem::new(config).unwrap();

    // Initialize
    ecosystem.initialize().unwrap();
    assert_eq!(ecosystem.get_status().status, OperationalStatus::Healthy);

    // Check uptime (should be very small, just initialized)
    let uptime = ecosystem.uptime();
    assert!(uptime.as_micros() < 1_000_000); // Less than 1 second

    // Shutdown
    ecosystem.shutdown().unwrap();
    assert_eq!(ecosystem.get_status().status, OperationalStatus::Shutdown);
}
