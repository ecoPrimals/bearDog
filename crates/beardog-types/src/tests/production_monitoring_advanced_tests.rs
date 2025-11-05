// Production Monitoring Advanced Tests
//
// Test suite covering:
// - Performance monitoring and analysis
// - System monitoring and resource tracking
// - Integration scenarios
// - Edge cases and stress testing

#![allow(clippy::unwrap_used)] // Test code - unwraps are acceptable

use crate::production::{
    health::HealthStatus,
    metrics::CurrentMetrics,
    monitoring::{
        AlertManager, AlertThresholds, PerformanceMonitor, ResourceUtilization, SystemConfig,
        SystemMetrics, SystemMetricsCollector, SystemMonitor,
    },
    OperationalStatus, PerformanceMetrics, ProductionConfig, ProductionEcosystem,
};
use chrono::Duration;

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
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            requests_per_second: 100.0,
            error_rate_percent: 0.5,
            throughput_bytes_per_sec: 1_000_000,
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
        };
        monitor.add_performance_data(metrics);
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    // Should be capped at 1000
    assert_eq!(monitor.get_performance_history().len(), 1000);
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: types
// TEST_PRIORITY: normal

#[test]
fn test_performance_monitor_clear_history() {
    let mut monitor = PerformanceMonitor::new();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    // Add data
    for _ in 0..10 {
        monitor.add_performance_data(PerformanceMetrics::default());
    }

    assert_eq!(monitor.get_performance_history().len(), 10);

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
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
// TEST_CATEGORY: integration
// TEST_DOMAIN: types
// TEST_PRIORITY: normal

#[test]
fn test_system_monitor_initialize() {
    let mut monitor = SystemMonitor::new();
    let result = monitor.initialize();
    assert!(result.is_ok());
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
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
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
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

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    let config = monitor.get_config();
    assert!(config.enable_cpu_monitoring);
    assert!(!config.enable_memory_monitoring);
    assert_eq!(config.system_interval_seconds, 60);
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
#[test]
fn test_system_monitor_selective_monitoring() {
    let mut monitor = SystemMonitor::new();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    // Disable some monitoring
    let config = SystemConfig {
        enable_cpu_monitoring: true,
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        enable_memory_monitoring: false,
        enable_disk_monitoring: false,
        system_interval_seconds: 30,
    };

    monitor.update_config(config);

    let overview = monitor.get_overview().unwrap();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    // Only CPU monitoring is enabled
    assert!(overview.resource_utilization.cpu_percent > 0.0);
    assert_eq!(overview.resource_utilization.memory_percent, 0.0);
    assert_eq!(overview.resource_utilization.disk_percent, 0.0);
}

// ====================
// TEST_CATEGORY: integration
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
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
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
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
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
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
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
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

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
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
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            latency_ms: 10.0 + (f64::from(i)) * 0.5,
            error_rate_percent: 0.1,
            custom_metrics: std::collections::HashMap::new(),
        };
        collector.add_metrics_to_history(metrics);
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
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
// TEST_CATEGORY: integration
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
fn test_extreme_metric_values() {
    let metrics = SystemMetrics {
        cpu_utilization: 999.9,
        memory_utilization: 1000.0,
        disk_utilization: 150.0,
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        network_utilization_bps: u64::MAX,
    };

    // Should handle extreme values without panicking
    assert_eq!(metrics.cpu_utilization, 999.9);
    assert_eq!(metrics.network_utilization_bps, u64::MAX);
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
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
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
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
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
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

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
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
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

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

// TEST_CATEGORY: integration
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
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
