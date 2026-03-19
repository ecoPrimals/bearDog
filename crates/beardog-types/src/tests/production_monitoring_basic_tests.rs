// SPDX-License-Identifier: AGPL-3.0-only

// Production Monitoring Basic Tests
//
// Test suite covering:
// - Production ecosystem configuration and lifecycle
// - Monitoring configuration
// - Alert system functionality
// - System metrics collection

#![allow(clippy::unwrap_used)] // Test code - unwraps are acceptable

use crate::production::{
    health::HealthStatus,
    metrics::CurrentMetrics,
    monitoring::{
        AlertManager, AlertSeverity, AlertStatus, AlertThresholds, MetricType, MonitoringConfig,
        OperationStatus, PerformanceConfig, SystemConfig, SystemMetrics, SystemMetricsCollector,
    },
    EnvironmentLevel, OperationalStatus, PerformanceMetrics, ProductionConfig,
    ProductionCoreConfig, ProductionEcosystem, ProductionEcosystemBuilder, ProductionFlags,
    ProductionState,
};
use chrono::Duration;

// ====================
// Production Ecosystem Tests
// ====================

#[test]
fn test_production_config_default() {
    let config = ProductionConfig::default();
    assert_eq!(config.core.environment_level, EnvironmentLevel::Development);
    assert_eq!(config.core.service_name, "beardog-ecosystem");
    assert!(config.core.flags.enable_advanced_monitoring);
}

#[test]
fn test_production_core_config_custom() {
    let config = ProductionCoreConfig {
        environment_level: EnvironmentLevel::Production,
        service_name: "test-service".to_string(),
        service_version: "2.0.0".to_string(),
        deployment_id: "deploy-123".to_string(),
        region: "us-west-2".to_string(),
        cluster_id: "cluster-prod-1".to_string(),
        node_id: "node-1".to_string(),
        flags: ProductionFlags::default(),
    };

    assert_eq!(config.environment_level, EnvironmentLevel::Production);
    assert_eq!(config.service_name, "test-service");
    assert_eq!(config.region, "us-west-2");
}

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
    assert_eq!(levels[3], EnvironmentLevel::Production);
}

#[test]
fn test_production_flags_default() {
    let flags = ProductionFlags::default();
    assert!(flags.enable_advanced_monitoring);
    assert!(flags.enable_distributed_tracing);
    assert!(flags.enable_performance_profiling);
    assert!(flags.enable_security_auditing);
    assert!(!flags.enable_auto_scaling); // Should be false by default
    assert!(flags.enable_circuit_breakers);
    assert!(flags.enable_rate_limiting);
    assert!(flags.enable_caching);
}

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
    assert_eq!(statuses[1], OperationalStatus::Healthy);
}

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
fn test_production_ecosystem_builder() {
    let ecosystem = ProductionEcosystemBuilder::new()
        .environment_level(EnvironmentLevel::Staging)
        .service("test-service".to_string(), "1.0.0".to_string())
        .deployment(
            "deploy-test".to_string(),
            "us-east-1".to_string(),
            "cluster-1".to_string(),
        )
        .enable_advanced_features()
        .build();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    assert!(ecosystem.is_ok());
    let ecosystem = ecosystem.unwrap();
    assert_eq!(
        ecosystem.config.core.environment_level,
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        EnvironmentLevel::Staging
    );
    assert_eq!(ecosystem.config.core.service_name, "test-service");
    assert_eq!(ecosystem.config.core.region, "us-east-1");
}

#[test]
fn test_production_ecosystem_new() {
    let config = ProductionConfig::default();
    let result = ProductionEcosystem::new(config);
    assert!(result.is_ok());
}

#[test]
fn test_production_ecosystem_initialization() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    let config = ProductionConfig::default();
    let mut ecosystem = ProductionEcosystem::new(config).unwrap();

    let result = ecosystem.initialize();
    assert!(result.is_ok());
    assert_eq!(ecosystem.get_status().status, OperationalStatus::Healthy);
}

#[test]
fn test_production_ecosystem_status_tracking() {
    let config = ProductionConfig::default();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    let mut ecosystem = ProductionEcosystem::new(config).unwrap();

    let initial_status = ecosystem.get_status();
    assert_eq!(initial_status.status, OperationalStatus::Initializing);

    ecosystem.initialize().unwrap();
    assert_eq!(ecosystem.get_status().status, OperationalStatus::Healthy);
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
fn test_production_ecosystem_uptime() {
    let config = ProductionConfig::default();
    let ecosystem = ProductionEcosystem::new(config).unwrap();

    let uptime = ecosystem.uptime();
    // Uptime should be a valid duration (non-negative by type)
    assert!(uptime.as_secs() < u64::MAX);
    assert!(uptime.as_millis() < u128::MAX);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
}

#[test]
fn test_production_ecosystem_health_check() {
    let config = ProductionConfig::default();
    let mut ecosystem = ProductionEcosystem::new(config).unwrap();
    ecosystem.initialize().unwrap();

    let report = ecosystem.health_check();
    assert!(report.is_ok());

    let report = report.unwrap();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert_eq!(report.overall_status, HealthStatus::Healthy);
}

#[test]
fn test_production_ecosystem_shutdown() {
    let config = ProductionConfig::default();
    let mut ecosystem = ProductionEcosystem::new(config).unwrap();
    ecosystem.initialize().unwrap();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    let result = ecosystem.shutdown();
    assert!(result.is_ok());
    assert_eq!(ecosystem.get_status().status, OperationalStatus::Shutdown);
}

#[test]
fn test_production_ecosystem_with_auto_scaling() {
    let mut config = ProductionConfig::default();
    config.core.flags.enable_auto_scaling = true;

    let mut ecosystem = ProductionEcosystem::new(config).unwrap();
    ecosystem.initialize().unwrap();

    // Update metrics with auto-scaling enabled
    let result = ecosystem.update_metrics();
    assert!(result.is_ok());
}

// ====================
// TEST_CATEGORY: integration
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
// Monitoring Configuration Tests
// ====================

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
fn test_monitoring_config_default() {
    let config = MonitoringConfig::default();
    assert_eq!(config.monitoring_interval_seconds, 60);
    assert!(config.enable_alerting);
    assert_eq!(config.alert_retention_count, 1000);
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
#[test]
fn test_performance_config_default() {
    let config = PerformanceConfig::default();
    assert!(config.enable_latency_monitoring);
    assert!(config.enable_throughput_monitoring);
    assert_eq!(config.retention_hours, 24);
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
fn test_system_config_default() {
    let config = SystemConfig::default();
    assert!(config.enable_cpu_monitoring);
    assert!(config.enable_memory_monitoring);
    assert!(config.enable_disk_monitoring);
    assert_eq!(config.system_interval_seconds, 30);
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
#[test]
fn test_system_config_custom_intervals() {
    let config = SystemConfig {
        enable_cpu_monitoring: true,
        enable_memory_monitoring: true,
        enable_disk_monitoring: false,
        system_interval_seconds: 60,
    };

    assert!(config.enable_cpu_monitoring);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert!(!config.enable_disk_monitoring);
    assert_eq!(config.system_interval_seconds, 60);
}

// ====================
// Alert System Tests
// ====================

// TEST_CATEGORY: integration
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
#[test]
fn test_metric_type_variants() {
    let types = [
        MetricType::Counter,
        MetricType::Gauge,
        MetricType::Histogram,
        MetricType::Summary,
    ];

    assert_eq!(types.len(), 4);
    assert_eq!(types[0], MetricType::Counter);
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
fn test_alert_severity_ordering() {
    use std::cmp::Ordering;

    assert_eq!(
        AlertSeverity::Info.cmp(&AlertSeverity::Warning),
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        Ordering::Less
    );
    assert_eq!(
        AlertSeverity::Warning.cmp(&AlertSeverity::Critical),
        Ordering::Less // TEST_CATEGORY: integration
                       // TEST_DOMAIN: types
                       // TEST_PRIORITY: normal
    );
    assert_eq!(
        AlertSeverity::Critical.cmp(&AlertSeverity::Emergency),
        Ordering::Less
    );
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: types
// TEST_PRIORITY: normal

#[test]
fn test_alert_status_variants() {
    let statuses = [
        AlertStatus::Active,
        AlertStatus::Acknowledged,
        AlertStatus::Resolved,
    ];

    assert_eq!(statuses.len(), 3);
    assert_eq!(statuses[0], AlertStatus::Active);
}

#[test]
fn test_operation_status_variants() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    let statuses = [
        OperationStatus::Processing,
        OperationStatus::Completed,
        OperationStatus::Failed,
        OperationStatus::Timeout,
    ];

    assert_eq!(statuses.len(), 4);
    assert_eq!(statuses[1], OperationStatus::Completed);
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: types
// TEST_PRIORITY: normal

#[test]
fn test_alert_manager_creation() {
    let thresholds = AlertThresholds {
        cpu_threshold_percent: 80.0,
        memory_threshold_percent: 85.0,
        disk_threshold_percent: 90.0,
        network_threshold_bps: 100_000_000,
        error_rate_threshold_percent: 5.0,
        response_time_threshold_ms: 1000.0,
    };

    let manager = AlertManager::new(thresholds);
    // Just verify it was created successfully
    let recent_alerts = manager.get_recent_alerts(10);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert!(recent_alerts.is_ok());
}

#[test]
fn test_alert_manager_cpu_threshold() {
    let thresholds = AlertThresholds {
        cpu_threshold_percent: 70.0,
        memory_threshold_percent: 85.0,
        disk_threshold_percent: 90.0,
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        network_threshold_bps: 100_000_000,
        error_rate_threshold_percent: 5.0,
        response_time_threshold_ms: 1000.0,
    };

    let mut manager = AlertManager::new(thresholds);

    // System metrics exceeding CPU threshold
    let system_metrics = SystemMetrics {
        cpu_utilization: 75.0, // Exceeds 70% threshold
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        memory_utilization: 50.0,
        disk_utilization: 40.0,
        network_utilization_bps: 50_000_000,
    };

    let performance_metrics = PerformanceMetrics::default();
    let alerts = manager.check_thresholds(&system_metrics, &performance_metrics);
    assert!(alerts.is_ok());

    let alerts = alerts.unwrap();
    assert!(!alerts.is_empty());
    assert_eq!(alerts[0].severity, AlertSeverity::Warning);
    assert!(alerts[0].message.contains("High CPU utilization"));
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: types
// TEST_PRIORITY: normal

#[test]
fn test_alert_manager_no_threshold_violations() {
    let thresholds = AlertThresholds {
        cpu_threshold_percent: 80.0,
        memory_threshold_percent: 85.0,
        disk_threshold_percent: 90.0,
        network_threshold_bps: 100_000_000,
        error_rate_threshold_percent: 5.0,
        response_time_threshold_ms: 1000.0,
    };

    let mut manager = AlertManager::new(thresholds);

    // System metrics within all thresholds
    let system_metrics = SystemMetrics {
        cpu_utilization: 50.0,
        memory_utilization: 60.0,
        disk_utilization: 70.0,
        network_utilization_bps: 50_000_000,
    };

    let performance_metrics = PerformanceMetrics::default();
    let alerts = manager.check_thresholds(&system_metrics, &performance_metrics);
    assert!(alerts.is_ok());

    let alerts = alerts.unwrap();
    assert!(alerts.is_empty());
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
}

#[test]
fn test_alert_manager_get_recent_alerts() {
    let thresholds = AlertThresholds {
        cpu_threshold_percent: 80.0,
        memory_threshold_percent: 85.0,
        disk_threshold_percent: 90.0,
        network_threshold_bps: 100_000_000,
        error_rate_threshold_percent: 5.0,
        response_time_threshold_ms: 1000.0,
    };

    let manager = AlertManager::new(thresholds);

    // Test with various limits
    for limit in [1, 5, 10, 100] {
        let alerts = manager.get_recent_alerts(limit);
        assert!(alerts.is_ok());
        let alerts = alerts.unwrap();
        assert!(alerts.len() <= limit);
    }
}

// ====================
// System Metrics Tests
// TEST_CATEGORY: integration
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
// ====================

#[test]
fn test_system_metrics_default() {
    let metrics = SystemMetrics::default();
    assert_eq!(metrics.cpu_utilization, 0.0);
    assert_eq!(metrics.memory_utilization, 0.0);
    assert_eq!(metrics.disk_utilization, 0.0);
    assert_eq!(metrics.network_utilization_bps, 0);
}

#[test]
fn test_system_metrics_custom() {
    let metrics = SystemMetrics {
        cpu_utilization: 45.5,
        memory_utilization: 72.3,
        disk_utilization: 35.8,
        network_utilization_bps: 1_500_000,
    };

    assert_eq!(metrics.cpu_utilization, 45.5);
    assert_eq!(metrics.network_utilization_bps, 1_500_000);
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: types
// TEST_PRIORITY: normal

#[test]
fn test_system_metrics_collector_new() {
    let interval = Duration::seconds(60);
    let collector = SystemMetricsCollector::new(interval);

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    let config = collector.get_config();
    assert_eq!(config.monitoring_interval_seconds, 60);
    assert!(config.enable_alerting);
}

#[test]
fn test_system_metrics_collector_start() {
    let interval = Duration::seconds(30);
    let mut collector = SystemMetricsCollector::new(interval);

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    let result = collector.start_collection();
    assert!(result.is_ok());
}

#[test]
fn test_system_metrics_collector_summary() {
    let interval = Duration::seconds(60);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    let collector = SystemMetricsCollector::new(interval);

    let summary = collector.get_summary();
    assert!(summary.is_ok());

    let summary = summary.unwrap();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert!(summary.health_score >= 0.0 && summary.health_score <= 1.0);
    assert_eq!(summary.active_alerts_count, 0);
}

#[test]
fn test_system_metrics_collector_history() {
    let interval = Duration::seconds(60);
    let mut collector = SystemMetricsCollector::new(interval);

    // Initially empty
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert_eq!(collector.get_metrics_history().len(), 0);

    // Add some metrics
    for _ in 0..5 {
        let metrics = CurrentMetrics {
            timestamp: chrono::Utc::now(),
            cpu_usage_percent: 50.0,
            memory_usage_percent: 60.0,
            disk_usage_percent: 40.0,
            network_throughput_bps: 1_000_000,
            active_connections: 100,
            latency_ms: 10.0,
            error_rate_percent: 0.1,
            custom_metrics: std::collections::HashMap::new(),
        };
        collector.add_metrics_to_history(metrics);
    }

    assert_eq!(collector.get_metrics_history().len(), 5);
}

#[test]
fn test_system_metrics_collector_history_retention() {
    let interval = Duration::seconds(60);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    let mut collector = SystemMetricsCollector::new(interval);

    // Add more metrics than retention limit
    let retention_limit = 1000;
    for _ in 0..(retention_limit + 100) {
        let metrics = CurrentMetrics {
            timestamp: chrono::Utc::now(),
            cpu_usage_percent: 50.0,
            memory_usage_percent: 60.0,
            disk_usage_percent: 40.0,
            network_throughput_bps: 1_000_000,
            active_connections: 100,
            latency_ms: 10.0,
            error_rate_percent: 0.1,
            custom_metrics: std::collections::HashMap::new(),
        };
        collector.add_metrics_to_history(metrics);
    }

    // Should be capped at retention limit
    assert_eq!(collector.get_metrics_history().len(), retention_limit);
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
#[test]
fn test_system_metrics_collector_clear_history() {
    let interval = Duration::seconds(60);
    let mut collector = SystemMetricsCollector::new(interval);

    // Add some metrics
    for _ in 0..5 {
        let metrics = CurrentMetrics {
            timestamp: chrono::Utc::now(),
            cpu_usage_percent: 50.0,
            memory_usage_percent: 60.0,
            disk_usage_percent: 40.0,
            network_throughput_bps: 1_000_000,
            active_connections: 100,
            latency_ms: 10.0,
            error_rate_percent: 0.1,
            custom_metrics: std::collections::HashMap::new(),
        };
        collector.add_metrics_to_history(metrics);
    }

    assert_eq!(collector.get_metrics_history().len(), 5);

    collector.clear_metrics_history();
    assert_eq!(collector.get_metrics_history().len(), 0);
}
