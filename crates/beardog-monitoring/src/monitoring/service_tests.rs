// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use crate::types::AlertLevel;

#[tokio::test]
async fn test_monitoring_service_creation() -> Result<(), BearDogError> {
    let config = MonitoringConfig::default();
    let service = MonitoringService::new(config);
    let metrics = service.collect_performance_metrics()?;

    assert!(metrics.cpu_usage_percent >= 0.0);
    assert!(metrics.memory_usage_percent >= 0.0);

    Ok(())
}

#[tokio::test]
async fn test_alert_generation() -> Result<(), BearDogError> {
    let config = Default::default();

    let service = MonitoringService::new(config);
    let metrics = service.collect_performance_metrics()?;

    service.check_alerts(&metrics)?;
    let alerts = service.get_recent_alerts(10).await?;

    let _alert_count = alerts.len();

    Ok(())
}

#[test]
fn test_alert_level_ordering() {
    assert_eq!(AlertLevel::Low, AlertLevel::Low);
    assert_ne!(AlertLevel::Medium, AlertLevel::Critical);
}

#[test]
fn test_monitoring_service_new() {
    let config = MonitoringConfig::default();
    let _service = MonitoringService::new(config);
}

#[test]
fn test_start_monitoring() {
    let service = MonitoringService::new(MonitoringConfig::default());
    let result = service.start_monitoring();
    assert!(result.is_ok());
}

#[test]
fn test_stop_monitoring() {
    let service = MonitoringService::new(MonitoringConfig::default());
    let result = service.stop_monitoring();
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_record_counter() {
    let service = MonitoringService::new(MonitoringConfig::default());
    let result = service.record_counter("test_counter", 42).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_record_gauge() {
    let service = MonitoringService::new(MonitoringConfig::default());
    let result = service.record_gauge("test_gauge", 100.5).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_record_histogram() {
    let service = MonitoringService::new(MonitoringConfig::default());
    let values = vec![1.0, 2.0, 3.0];
    let result = service.record_histogram("test_histogram", values).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_record_timer() {
    let service = MonitoringService::new(MonitoringConfig::default());
    let duration = std::time::Duration::from_millis(100);
    let result = service.record_timer("test_timer", duration).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_take_snapshot() {
    let service = MonitoringService::new(MonitoringConfig::default());
    let result = service.take_snapshot().await;
    assert!(result.is_ok());
}

#[test]
fn test_get_health_status() {
    let service = MonitoringService::new(MonitoringConfig::default());
    let result = service.get_health_status();
    assert!(result.is_ok());
    assert_eq!(result.expect("health status"), HealthStatus::Healthy);
}

#[tokio::test]
async fn test_get_snapshots_empty() {
    let service = MonitoringService::new(MonitoringConfig::default());
    let result = service.get_snapshots().await;
    assert!(result.is_ok());
    assert_eq!(result.expect("snapshots").len(), 0);
}

#[tokio::test]
async fn test_get_latest_snapshot_none() {
    let service = MonitoringService::new(MonitoringConfig::default());
    let result = service.get_latest_snapshot().await;
    assert!(result.is_ok());
    assert!(result.expect("latest snapshot").is_none());
}

#[test]
fn test_get_uptime_seconds() {
    let service = MonitoringService::new(MonitoringConfig::default());
    let uptime = service.get_uptime_seconds();
    assert!(uptime >= 0);
}

#[test]
fn test_collect_performance_metrics() {
    let service = MonitoringService::new(MonitoringConfig::default());
    let result = service.collect_performance_metrics();
    assert!(result.is_ok());
}

#[test]
fn test_check_alerts_normal() {
    let service = MonitoringService::new(MonitoringConfig::default());
    let metrics = SystemPerformanceMetrics {
        cpu_usage_percent: 50.0,
        memory_usage_percent: 60.0,
        memory_total_bytes: 8_589_934_592,
        memory_used_bytes: 5_153_960_755,
        disk_usage_percent: 45.0,
        network_bytes_in: 1_048_576,
        network_bytes_out: 524_288,
        uptime_seconds: 86400,
        active_connections: 10,
    };

    let result = service.check_alerts(&metrics);
    assert!(result.is_ok());
}

#[test]
fn test_check_alerts_cpu_critical() {
    let service = MonitoringService::new(MonitoringConfig::default());
    let metrics = SystemPerformanceMetrics {
        cpu_usage_percent: 95.0,
        memory_usage_percent: 60.0,
        memory_total_bytes: 8_589_934_592,
        memory_used_bytes: 5_153_960_755,
        disk_usage_percent: 45.0,
        network_bytes_in: 1_048_576,
        network_bytes_out: 524_288,
        uptime_seconds: 86400,
        active_connections: 10,
    };

    let result = service.check_alerts(&metrics);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_export_prometheus_metrics() {
    let service = MonitoringService::new(MonitoringConfig::default());
    let result = service.export_prometheus_metrics().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_take_multiple_snapshots() {
    let service = MonitoringService::new(MonitoringConfig::default());

    for _ in 0..3 {
        service.take_snapshot().await.expect("snapshot");
    }

    let snapshots = service.get_snapshots().await.expect("snapshots");
    assert_eq!(snapshots.len(), 3);
}

#[tokio::test]
async fn test_get_latest_snapshot_some() {
    let service = MonitoringService::new(MonitoringConfig::default());
    service.take_snapshot().await.expect("snapshot");

    let result = service.get_latest_snapshot().await;
    assert!(result.is_ok());
    assert!(result.expect("latest snapshot").is_some());
}

#[tokio::test]
async fn test_record_counter_various_values() {
    let service = MonitoringService::new(MonitoringConfig::default());

    for value in &[0, 1, 100, 1000] {
        let result = service.record_counter("test", *value).await;
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_record_gauge_various_values() {
    let service = MonitoringService::new(MonitoringConfig::default());

    for value in &[0.0, 0.5, 1.0, 100.0] {
        let result = service.record_gauge("test", *value).await;
        assert!(result.is_ok());
    }
}

#[test]
fn test_check_alerts_memory_critical() {
    let service = MonitoringService::new(MonitoringConfig::default());
    let metrics = SystemPerformanceMetrics {
        cpu_usage_percent: 50.0,
        memory_usage_percent: 95.0,
        memory_total_bytes: 8_589_934_592,
        memory_used_bytes: 8_153_960_755,
        disk_usage_percent: 45.0,
        network_bytes_in: 1_048_576,
        network_bytes_out: 524_288,
        uptime_seconds: 86400,
        active_connections: 10,
    };

    let result = service.check_alerts(&metrics);
    assert!(result.is_err());
}

#[test]
fn test_check_alerts_disk_critical() {
    let service = MonitoringService::new(MonitoringConfig::default());
    let metrics = SystemPerformanceMetrics {
        cpu_usage_percent: 50.0,
        memory_usage_percent: 60.0,
        memory_total_bytes: 8_589_934_592,
        memory_used_bytes: 5_153_960_755,
        disk_usage_percent: 95.0,
        network_bytes_in: 1_048_576,
        network_bytes_out: 524_288,
        uptime_seconds: 86400,
        active_connections: 10,
    };

    let result = service.check_alerts(&metrics);
    assert!(result.is_err());
}

#[test]
fn monitoring_config_default_clone_roundtrip() {
    let c = MonitoringConfig::default();
    let c2 = c.clone();
    assert_eq!(c.snapshot_interval_seconds, c2.snapshot_interval_seconds);
    assert_eq!(c.max_snapshots, c2.max_snapshots);
}
