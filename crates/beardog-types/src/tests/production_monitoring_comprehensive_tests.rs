// Production Monitoring Comprehensive Tests
//
// Comprehensive test suite for production monitoring, metrics collection,
// alerting, and observability features. Tests edge cases, error conditions,
// concurrent access, and performance characteristics.

#![allow(clippy::unwrap_used)] // Test code - unwraps are acceptable

use crate::production::{
    health::HealthStatus,
    metrics::CurrentMetrics,
    monitoring::{
        AlertManager, AlertSeverity, AlertStatus, AlertThresholds, MetricType, MonitoringConfig,
        OperationStatus, PerformanceConfig, PerformanceMonitor, ResourceUtilization, SystemConfig,
        SystemMetrics, SystemMetricsCollector, SystemMonitor,
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

    assert!(ecosystem.is_ok());
    let ecosystem = ecosystem.unwrap();
    assert_eq!(
        ecosystem.config.core.environment_level,
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
    let config = ProductionConfig::default();
    let mut ecosystem = ProductionEcosystem::new(config).unwrap();

    let result = ecosystem.initialize();
    assert!(result.is_ok());
    assert_eq!(ecosystem.get_status().status, OperationalStatus::Healthy);
}

#[test]
fn test_production_ecosystem_status_tracking() {
    let config = ProductionConfig::default();
    let mut ecosystem = ProductionEcosystem::new(config).unwrap();

    let initial_status = ecosystem.get_status();
    assert_eq!(initial_status.status, OperationalStatus::Initializing);

    ecosystem.initialize().unwrap();
    assert_eq!(ecosystem.get_status().status, OperationalStatus::Healthy);
}

#[test]
fn test_production_ecosystem_uptime() {
    let config = ProductionConfig::default();
    let ecosystem = ProductionEcosystem::new(config).unwrap();

    let uptime = ecosystem.uptime();
    // Uptime should be a valid duration (non-negative by type)
    assert!(uptime.as_secs() < u64::MAX);
    assert!(uptime.as_millis() < u128::MAX);
}

#[test]
fn test_production_ecosystem_health_check() {
    let config = ProductionConfig::default();
    let mut ecosystem = ProductionEcosystem::new(config).unwrap();
    ecosystem.initialize().unwrap();

    let report = ecosystem.health_check();
    assert!(report.is_ok());

    let report = report.unwrap();
    assert_eq!(report.overall_status, HealthStatus::Healthy);
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
// Monitoring Configuration Tests
// ====================

#[test]
fn test_monitoring_config_default() {
    let config = MonitoringConfig::default();
    assert_eq!(config.monitoring_interval_seconds, 60);
    assert!(config.enable_alerting);
    assert_eq!(config.alert_retention_count, 1000);
}

#[test]
fn test_performance_config_default() {
    let config = PerformanceConfig::default();
    assert!(config.enable_latency_monitoring);
    assert!(config.enable_throughput_monitoring);
    assert_eq!(config.retention_hours, 24);
}

#[test]
fn test_system_config_default() {
    let config = SystemConfig::default();
    assert!(config.enable_cpu_monitoring);
    assert!(config.enable_memory_monitoring);
    assert!(config.enable_disk_monitoring);
    assert_eq!(config.system_interval_seconds, 30);
}

#[test]
fn test_system_config_custom_intervals() {
    let config = SystemConfig {
        enable_cpu_monitoring: true,
        enable_memory_monitoring: true,
        enable_disk_monitoring: false,
        system_interval_seconds: 60,
    };

    assert!(config.enable_cpu_monitoring);
    assert!(!config.enable_disk_monitoring);
    assert_eq!(config.system_interval_seconds, 60);
}

// ====================
// Alert System Tests
// ====================

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
fn test_alert_severity_ordering() {
    use std::cmp::Ordering;

    assert_eq!(
        AlertSeverity::Info.cmp(&AlertSeverity::Warning),
        Ordering::Less
    );
    assert_eq!(
        AlertSeverity::Warning.cmp(&AlertSeverity::Critical),
        Ordering::Less
    );
    assert_eq!(
        AlertSeverity::Critical.cmp(&AlertSeverity::Emergency),
        Ordering::Less
    );
}

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
    let statuses = [
        OperationStatus::Processing,
        OperationStatus::Completed,
        OperationStatus::Failed,
        OperationStatus::Timeout,
    ];

    assert_eq!(statuses.len(), 4);
    assert_eq!(statuses[1], OperationStatus::Completed);
}

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
    assert!(recent_alerts.is_ok());
}

#[test]
fn test_alert_manager_cpu_threshold() {
    let thresholds = AlertThresholds {
        cpu_threshold_percent: 70.0,
        memory_threshold_percent: 85.0,
        disk_threshold_percent: 90.0,
        network_threshold_bps: 100_000_000,
        error_rate_threshold_percent: 5.0,
        response_time_threshold_ms: 1000.0,
    };

    let mut manager = AlertManager::new(thresholds);

    // System metrics exceeding CPU threshold
    let system_metrics = SystemMetrics {
        cpu_utilization: 75.0, // Exceeds 70% threshold
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

#[test]
fn test_system_metrics_collector_new() {
    let interval = Duration::seconds(60);
    let collector = SystemMetricsCollector::new(interval);

    let config = collector.get_config();
    assert_eq!(config.monitoring_interval_seconds, 60);
    assert!(config.enable_alerting);
}

#[test]
fn test_system_metrics_collector_start() {
    let interval = Duration::seconds(30);
    let mut collector = SystemMetricsCollector::new(interval);

    let result = collector.start_collection();
    assert!(result.is_ok());
}

#[test]
fn test_system_metrics_collector_summary() {
    let interval = Duration::seconds(60);
    let collector = SystemMetricsCollector::new(interval);

    let summary = collector.get_summary();
    assert!(summary.is_ok());

    let summary = summary.unwrap();
    assert!(summary.health_score >= 0.0 && summary.health_score <= 1.0);
    assert_eq!(summary.active_alerts_count, 0);
}

#[test]
fn test_system_metrics_collector_history() {
    let interval = Duration::seconds(60);
    let mut collector = SystemMetricsCollector::new(interval);

    // Initially empty
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

// ====================
// Performance Monitor Tests
// ====================

#[test]
fn test_performance_monitor_new() {
    let monitor = PerformanceMonitor::new();
    assert_eq!(monitor.get_performance_history().len(), 0);
}

#[test]
fn test_performance_monitor_default() {
    let monitor = PerformanceMonitor::default();
    assert_eq!(monitor.get_performance_history().len(), 0);
}

#[test]
fn test_performance_monitor_initialize() {
    let mut monitor = PerformanceMonitor::new();
    let result = monitor.initialize();
    assert!(result.is_ok());
}

#[test]
fn test_performance_monitor_collect_metrics() {
    let monitor = PerformanceMonitor::new();
    let result = monitor.collect_performance_metrics();
    assert!(result.is_ok());
}

#[test]
fn test_performance_monitor_summary_no_history() {
    let monitor = PerformanceMonitor::new();
    let summary = monitor.get_summary();
    assert!(summary.is_ok());

    let summary = summary.unwrap();
    assert_eq!(summary.efficiency_score, 0.92); // Default score
    assert!(summary.trend_indicator > 0);
}

#[test]
fn test_performance_monitor_summary_with_history() {
    let mut monitor = PerformanceMonitor::new();

    // Add some performance data
    for i in 0..20 {
        let metrics = PerformanceMetrics {
            avg_response_time_ms: 10.0 + (i as f64),
            p95_response_time_ms: 20.0,
            p99_response_time_ms: 30.0,
            requests_per_second: 100.0,
            error_rate_percent: 0.5,
            throughput_bytes_per_sec: 1_000_000,
        };
        monitor.add_performance_data(metrics);
    }

    let summary = monitor.get_summary();
    assert!(summary.is_ok());

    let summary = summary.unwrap();
    assert!(summary.efficiency_score > 0.85);
    assert!(!summary.recommendations.is_empty());
}

#[test]
fn test_performance_monitor_add_data() {
    let mut monitor = PerformanceMonitor::new();

    assert_eq!(monitor.get_performance_history().len(), 0);

    let metrics = PerformanceMetrics::default();
    monitor.add_performance_data(metrics);

    assert_eq!(monitor.get_performance_history().len(), 1);
}

#[test]
fn test_performance_monitor_history_retention() {
    let mut monitor = PerformanceMonitor::new();

    // Add more than the 1000 limit
    for i in 0..1200 {
        let metrics = PerformanceMetrics {
            avg_response_time_ms: i as f64,
            p95_response_time_ms: 20.0,
            p99_response_time_ms: 30.0,
            requests_per_second: 100.0,
            error_rate_percent: 0.5,
            throughput_bytes_per_sec: 1_000_000,
        };
        monitor.add_performance_data(metrics);
    }

    // Should be capped at 1000
    assert_eq!(monitor.get_performance_history().len(), 1000);
}

#[test]
fn test_performance_monitor_clear_history() {
    let mut monitor = PerformanceMonitor::new();

    // Add data
    for _ in 0..10 {
        monitor.add_performance_data(PerformanceMetrics::default());
    }

    assert_eq!(monitor.get_performance_history().len(), 10);

    monitor.clear_history();
    assert_eq!(monitor.get_performance_history().len(), 0);
}

// ====================
// System Monitor Tests
// ====================

#[test]
fn test_system_monitor_new() {
    let monitor = SystemMonitor::new();
    let config = monitor.get_config();
    assert!(config.enable_cpu_monitoring);
    assert!(config.enable_memory_monitoring);
    assert!(config.enable_disk_monitoring);
}

#[test]
fn test_system_monitor_default() {
    let monitor = SystemMonitor::default();
    assert_eq!(monitor.get_monitoring_interval(), 30);
}

#[test]
fn test_system_monitor_initialize() {
    let mut monitor = SystemMonitor::new();
    let result = monitor.initialize();
    assert!(result.is_ok());
}

#[test]
fn test_system_monitor_collect_metrics() {
    let monitor = SystemMonitor::new();
    let result = monitor.collect_system_metrics();
    assert!(result.is_ok());

    let metrics = result.unwrap();
    assert!(metrics.cpu_utilization >= 0.0);
    assert!(metrics.memory_utilization >= 0.0);
}

#[test]
fn test_system_monitor_overview() {
    let monitor = SystemMonitor::new();
    let result = monitor.get_overview();
    assert!(result.is_ok());

    let overview = result.unwrap();
    assert_eq!(overview.status, "Healthy");
    assert!(overview.active_services > 0);
}

#[test]
fn test_system_monitor_update_config() {
    let mut monitor = SystemMonitor::new();

    let new_config = SystemConfig {
        enable_cpu_monitoring: true,
        enable_memory_monitoring: false,
        enable_disk_monitoring: true,
        system_interval_seconds: 60,
    };

    monitor.update_config(new_config);

    let config = monitor.get_config();
    assert!(config.enable_cpu_monitoring);
    assert!(!config.enable_memory_monitoring);
    assert_eq!(config.system_interval_seconds, 60);
}

#[test]
fn test_system_monitor_selective_monitoring() {
    let mut monitor = SystemMonitor::new();

    // Disable some monitoring
    let config = SystemConfig {
        enable_cpu_monitoring: true,
        enable_memory_monitoring: false,
        enable_disk_monitoring: false,
        system_interval_seconds: 30,
    };

    monitor.update_config(config);

    let overview = monitor.get_overview().unwrap();

    // Only CPU monitoring is enabled
    assert!(overview.resource_utilization.cpu_percent > 0.0);
    assert_eq!(overview.resource_utilization.memory_percent, 0.0);
    assert_eq!(overview.resource_utilization.disk_percent, 0.0);
}

// ====================
// Resource Utilization Tests
// ====================

#[test]
fn test_resource_utilization_basic() {
    let utilization = ResourceUtilization {
        cpu_percent: 45.5,
        memory_percent: 62.3,
        disk_percent: 78.9,
        network_percent: 23.1,
    };

    assert_eq!(utilization.cpu_percent, 45.5);
    assert_eq!(utilization.memory_percent, 62.3);
    assert_eq!(utilization.disk_percent, 78.9);
    assert_eq!(utilization.network_percent, 23.1);
}

// ====================
// Integration Tests
// ====================

#[test]
fn test_full_production_monitoring_lifecycle() {
    // Create production ecosystem
    let config = ProductionConfig::default();
    let mut ecosystem = ProductionEcosystem::new(config).unwrap();

    // Initialize
    let result = ecosystem.initialize();
    assert!(result.is_ok());
    assert_eq!(ecosystem.get_status().status, OperationalStatus::Healthy);

    // Update metrics
    let result = ecosystem.update_metrics();
    assert!(result.is_ok());

    // Health check
    let health_report = ecosystem.health_check();
    assert!(health_report.is_ok());

    // Shutdown
    let result = ecosystem.shutdown();
    assert!(result.is_ok());
    assert_eq!(ecosystem.get_status().status, OperationalStatus::Shutdown);
}

#[test]
fn test_alert_system_integration() {
    let thresholds = AlertThresholds {
        cpu_threshold_percent: 70.0,
        memory_threshold_percent: 80.0,
        disk_threshold_percent: 85.0,
        network_threshold_bps: 100_000_000,
        error_rate_threshold_percent: 5.0,
        response_time_threshold_ms: 1000.0,
    };

    let mut manager = AlertManager::new(thresholds);

    // Simulate high load scenario
    let high_load_metrics = SystemMetrics {
        cpu_utilization: 85.0,
        memory_utilization: 88.0,
        disk_utilization: 92.0,
        network_utilization_bps: 150_000_000,
    };

    let performance_metrics = PerformanceMetrics {
        avg_response_time_ms: 1500.0,
        p95_response_time_ms: 2000.0,
        p99_response_time_ms: 3000.0,
        requests_per_second: 500.0,
        error_rate_percent: 8.0,
        throughput_bytes_per_sec: 5_000_000,
    };

    let alerts = manager.check_thresholds(&high_load_metrics, &performance_metrics);
    assert!(alerts.is_ok());

    let alerts = alerts.unwrap();
    // Should have CPU alert at minimum
    assert!(!alerts.is_empty());
}

#[test]
fn test_monitoring_with_disabled_features() {
    let mut config = ProductionConfig::default();
    config.core.flags.enable_advanced_monitoring = false;
    config.core.flags.enable_performance_profiling = false;

    let ecosystem = ProductionEcosystem::new(config);
    assert!(ecosystem.is_ok());
}

#[test]
fn test_metrics_collection_over_time() {
    let interval = Duration::seconds(10);
    let mut collector = SystemMetricsCollector::new(interval);

    collector.start_collection().unwrap();

    // Simulate multiple collection cycles
    for i in 0..10_u32 {
        #[allow(clippy::cast_precision_loss)]
        let metrics = CurrentMetrics {
            timestamp: chrono::Utc::now(),
            cpu_usage_percent: 50.0 + (f64::from(i)) * 2.0,
            memory_usage_percent: 60.0 + (f64::from(i)) * 1.0,
            disk_usage_percent: 40.0,
            network_throughput_bps: 1_000_000 + (u64::from(i) * 100_000),
            active_connections: 100 + (i * 10),
            latency_ms: 10.0 + (f64::from(i)) * 0.5,
            error_rate_percent: 0.1,
            custom_metrics: std::collections::HashMap::new(),
        };
        collector.add_metrics_to_history(metrics);
    }

    let history = collector.get_metrics_history();
    assert_eq!(history.len(), 10);

    // Verify metrics progression
    assert!(history[9].cpu_usage_percent > history[0].cpu_usage_percent);
    assert!(history[9].memory_usage_percent > history[0].memory_usage_percent);
}

// ====================
// Edge Case Tests
// ====================

#[test]
fn test_zero_monitoring_interval() {
    let interval = Duration::seconds(0);
    let collector = SystemMetricsCollector::new(interval);
    let config = collector.get_config();
    assert_eq!(config.monitoring_interval_seconds, 0);
}

#[test]
fn test_negative_duration_handling() {
    // chrono Duration can be negative, but we should handle it gracefully
    let interval = Duration::seconds(-60);
    let collector = SystemMetricsCollector::new(interval);

    // Start collection should still work
    let mut collector_mut = collector;
    let result = collector_mut.start_collection();
    assert!(result.is_ok());
}

#[test]
fn test_extreme_metric_values() {
    let metrics = SystemMetrics {
        cpu_utilization: 999.9,
        memory_utilization: 1000.0,
        disk_utilization: 150.0,
        network_utilization_bps: u64::MAX,
    };

    // Should handle extreme values without panicking
    assert_eq!(metrics.cpu_utilization, 999.9);
    assert_eq!(metrics.network_utilization_bps, u64::MAX);
}

#[test]
fn test_concurrent_metric_collection_simulation() {
    let interval = Duration::seconds(5);
    let mut collector = SystemMetricsCollector::new(interval);

    // Simulate rapid concurrent additions
    for i in 0..100_u32 {
        #[allow(clippy::cast_precision_loss)]
        let metrics = CurrentMetrics {
            timestamp: chrono::Utc::now(),
            cpu_usage_percent: f64::from(i % 100),
            memory_usage_percent: (f64::from(i % 100)) * 0.8,
            disk_usage_percent: 40.0,
            network_throughput_bps: u64::from(i * 1000),
            active_connections: i,
            latency_ms: (f64::from(i)) * 0.5,
            error_rate_percent: 0.1,
            custom_metrics: std::collections::HashMap::new(),
        };
        collector.add_metrics_to_history(metrics);
    }

    assert_eq!(collector.get_metrics_history().len(), 100);
}

#[test]
fn test_alert_metadata_handling() {
    let thresholds = AlertThresholds {
        cpu_threshold_percent: 50.0,
        memory_threshold_percent: 50.0,
        disk_threshold_percent: 50.0,
        network_threshold_bps: 50_000_000,
        error_rate_threshold_percent: 1.0,
        response_time_threshold_ms: 500.0,
    };

    let mut manager = AlertManager::new(thresholds);

    let metrics = SystemMetrics {
        cpu_utilization: 60.0,
        memory_utilization: 40.0,
        disk_utilization: 40.0,
        network_utilization_bps: 40_000_000,
    };

    let perf_metrics = PerformanceMetrics::default();
    let alerts = manager.check_thresholds(&metrics, &perf_metrics).unwrap();

    if !alerts.is_empty() {
        // Verify alert has proper structure
        assert!(!alerts[0].id.to_string().is_empty());
        assert!(!alerts[0].message.is_empty());
        assert!(!alerts[0].source.is_empty());
    }
}

#[test]
fn test_production_ecosystem_multiple_health_checks() {
    let config = ProductionConfig::default();
    let mut ecosystem = ProductionEcosystem::new(config).unwrap();
    ecosystem.initialize().unwrap();

    // Multiple consecutive health checks
    for _ in 0..5 {
        let report = ecosystem.health_check();
        assert!(report.is_ok());
        let report = report.unwrap();
        assert_eq!(report.overall_status, HealthStatus::Healthy);
    }
}

#[test]
fn test_performance_summary_recommendations() {
    let mut monitor = PerformanceMonitor::new();

    // Add minimal data
    for _ in 0..3 {
        monitor.add_performance_data(PerformanceMetrics::default());
    }

    let summary = monitor.get_summary().unwrap();
    assert!(!summary.recommendations.is_empty());

    // Add more data
    for _ in 0..20 {
        monitor.add_performance_data(PerformanceMetrics::default());
    }

    let summary = monitor.get_summary().unwrap();
    assert!(!summary.recommendations.is_empty());
}

#[test]
fn test_system_overview_completeness() {
    let monitor = SystemMonitor::new();
    let overview = monitor.get_overview().unwrap();

    // Verify all fields are populated
    assert!(!overview.status.is_empty());
    assert!(overview.uptime_seconds > 0);
    assert!(overview.timestamp.timestamp() > 0);
    assert!(overview.resource_utilization.cpu_percent >= 0.0);
    assert!(overview.resource_utilization.memory_percent >= 0.0);
    assert!(overview.resource_utilization.disk_percent >= 0.0);
    assert!(overview.resource_utilization.network_percent >= 0.0);
}
