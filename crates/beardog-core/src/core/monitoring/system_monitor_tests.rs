// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
struct TestAlertHandler {
    received_alerts: std::sync::Arc<std::sync::Mutex<Vec<SystemAlert>>>,
}

impl AlertHandler for TestAlertHandler {
    fn handle_alert(&self, alert: SystemAlert) -> Result<(), BearDogError> {
        self.received_alerts.lock().unwrap().push(alert);
        Ok(())
    }
}

#[derive(Debug)]
struct FailingAlertHandler;

impl AlertHandler for FailingAlertHandler {
    fn handle_alert(&self, _alert: SystemAlert) -> Result<(), BearDogError> {
        Err(BearDogError::internal("handler rejected".to_string()))
    }
}

#[tokio::test]
async fn test_monitoring_system_metrics_default() {
    let metrics = SystemMetrics::default();
    assert_eq!(metrics.cpu_usage_percent, 0.0);
    assert_eq!(metrics.memory_usage_percent, 0.0);
    assert_eq!(metrics.disk_usage_percent, 0.0);
    assert_eq!(metrics.network_bytes_in, 0);
    assert_eq!(metrics.network_bytes_out, 0);
    assert_eq!(metrics.uptime_seconds, 0);
    assert!(metrics.last_updated.is_none());
}

#[tokio::test]
async fn test_monitoring_config_default() {
    let config = SystemMonitorConfig::default();
    assert_eq!(config.check_interval_ms, 5000);
    assert_eq!(config.alert_threshold_cpu, 80.0);
    assert_eq!(config.alert_threshold_memory, 85.0);
    assert_eq!(config.alert_threshold_disk, 90.0);
    assert_eq!(config.max_alert_history, 1000);
}

#[tokio::test]
async fn test_monitoring_with_config() {
    let config = SystemMonitorConfig {
        check_interval_ms: 1000,
        alert_threshold_cpu: 90.0,
        alert_threshold_memory: 95.0,
        alert_threshold_disk: 99.0,
        max_alert_history: 500,
    };
    let monitor = SystemMonitor::with_config(config).expect("valid config");
    let metrics = monitor.get_system_metrics().await;
    assert_eq!(metrics.cpu_usage_percent, 0.0);
}

#[tokio::test]
async fn test_monitoring_get_component_health_none() {
    let monitor = SystemMonitor::new().expect("valid");
    let health = monitor.get_component_health("nonexistent").await;
    assert!(health.is_none());
}

#[tokio::test]
async fn test_monitoring_add_alert_handler() {
    let monitor = SystemMonitor::new().expect("valid");
    let handler = TestAlertHandler {
        received_alerts: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
    };
    let result = monitor.add_alert_handler(Box::new(handler)).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_monitoring_stop() {
    let mut monitor = SystemMonitor::new().expect("valid");
    let result = monitor.stop();
    assert!(result.is_ok());
}

#[test]
fn test_alert_type_variants() {
    let _ = AlertType::HighCpuUsage;
    let _ = AlertType::HighMemoryUsage;
    let _ = AlertType::HighDiskUsage;
    let _ = AlertType::ComponentDown;
    let _ = AlertType::HighErrorRate;
    let _ = AlertType::SlowResponse;
}

#[test]
fn test_alert_severity_variants() {
    assert_eq!(AlertSeverity::Info, AlertSeverity::Info);
    assert_eq!(AlertSeverity::Warning, AlertSeverity::Warning);
    assert_eq!(AlertSeverity::Critical, AlertSeverity::Critical);
    assert_eq!(AlertSeverity::Emergency, AlertSeverity::Emergency);
}

#[test]
fn test_component_health_creation() {
    let health = ComponentHealth {
        component_name: "test".to_string(),
        status: HealthStatus::Healthy,
        last_check: chrono::Utc::now(),
        response_time_ms: 10,
        error_count: 0,
        uptime_percent: 99.9,
    };
    assert_eq!(health.component_name, "test");
    assert_eq!(health.status, HealthStatus::Healthy);
}

#[test]
fn test_system_alert_creation() {
    let alert = SystemAlert {
        alert_type: AlertType::HighCpuUsage,
        message: "test".to_string(),
        severity: AlertSeverity::Warning,
        timestamp: chrono::Utc::now(),
        component: None,
        metric_value: Some(85.0),
    };
    assert_eq!(alert.message, "test");
    assert_eq!(alert.metric_value, Some(85.0));
}

#[test]
fn test_system_monitor_with_config() {
    let config = SystemMonitorConfig {
        check_interval_ms: 1000,
        alert_threshold_cpu: 90.0,
        alert_threshold_memory: 95.0,
        alert_threshold_disk: 95.0,
        max_alert_history: 500,
    };
    let monitor = SystemMonitor::with_config(config).expect("SystemMonitor::with_config in test");
    assert!(std::mem::size_of_val(&monitor) > 0);
}

#[test]
fn test_system_monitor_default() {
    let monitor = SystemMonitor::default();
    assert!(std::mem::size_of_val(&monitor) > 0);
}

#[tokio::test]
async fn test_system_monitor_get_metrics() {
    let monitor = SystemMonitor::new().expect("SystemMonitor::new in test");
    let metrics = monitor.get_system_metrics().await;
    assert!(metrics.cpu_usage_percent >= 0.0);
}

#[tokio::test]
async fn test_start_eventually_updates_metrics_snapshot() {
    let mut monitor = SystemMonitor::with_config(SystemMonitorConfig {
        check_interval_ms: 15,
        ..SystemMonitorConfig::default()
    })
    .expect("monitor");
    monitor.start().expect("start");
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(2);
    let metrics = loop {
        let m = monitor.get_system_metrics().await;
        if m.last_updated.is_some() {
            break m;
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "timed out waiting for background monitor to refresh metrics snapshot"
        );
        tokio::task::yield_now().await;
    };
    assert!(metrics.last_updated.is_some());
    assert!(metrics.cpu_usage_percent >= 0.0);
    let _ = monitor.stop();
}

#[tokio::test]
async fn test_collect_system_metrics_updates_snapshot() {
    let metrics = Arc::new(RwLock::new(SystemMetrics::default()));
    SystemMonitor::test_collect_system_metrics(&metrics)
        .await
        .unwrap();
    let snap = metrics.read().await;
    assert!(snap.last_updated.is_some());
    assert!((10.0..70.0).contains(&snap.cpu_usage_percent));
}

#[tokio::test]
async fn test_check_component_health_populates_map() {
    let health = Arc::new(RwLock::new(HashMap::new()));
    SystemMonitor::test_check_component_health(&health)
        .await
        .unwrap();
    let map = health.read().await;
    assert!(map.contains_key("core"));
    assert!(map.contains_key("genetics"));
}

#[tokio::test]
async fn test_process_alerts_cpu_warning_and_critical_severity() {
    let metrics = Arc::new(RwLock::new(SystemMetrics {
        cpu_usage_percent: 90.0,
        memory_usage_percent: 10.0,
        ..SystemMetrics::default()
    }));
    let health = Arc::new(RwLock::new(HashMap::new()));
    let received = Arc::new(std::sync::Mutex::new(Vec::new()));
    let handlers: Arc<RwLock<Vec<Box<dyn AlertHandler + Send + Sync>>>> =
        Arc::new(RwLock::new(vec![Box::new(TestAlertHandler {
            received_alerts: Arc::clone(&received),
        })]));
    let config = SystemMonitorConfig {
        alert_threshold_cpu: 80.0,
        alert_threshold_memory: 85.0,
        ..SystemMonitorConfig::default()
    };
    SystemMonitor::test_process_alerts(&metrics, &health, &handlers, &config)
        .await
        .unwrap();
    {
        let alerts = received.lock().unwrap();
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].alert_type, AlertType::HighCpuUsage);
        assert_eq!(alerts[0].severity, AlertSeverity::Warning);
    }

    let metrics_crit = Arc::new(RwLock::new(SystemMetrics {
        cpu_usage_percent: 96.0,
        memory_usage_percent: 10.0,
        ..SystemMetrics::default()
    }));
    let received2 = Arc::new(std::sync::Mutex::new(Vec::new()));
    let handlers2: Arc<RwLock<Vec<Box<dyn AlertHandler + Send + Sync>>>> =
        Arc::new(RwLock::new(vec![Box::new(TestAlertHandler {
            received_alerts: Arc::clone(&received2),
        })]));
    SystemMonitor::test_process_alerts(&metrics_crit, &health, &handlers2, &config)
        .await
        .unwrap();
    let alerts2 = received2.lock().unwrap();
    assert_eq!(alerts2[0].severity, AlertSeverity::Critical);
}

#[tokio::test]
async fn test_process_alerts_memory_thresholds() {
    let metrics = Arc::new(RwLock::new(SystemMetrics {
        cpu_usage_percent: 10.0,
        memory_usage_percent: 90.0,
        ..SystemMetrics::default()
    }));
    let health = Arc::new(RwLock::new(HashMap::new()));
    let received = Arc::new(std::sync::Mutex::new(Vec::new()));
    let handlers: Arc<RwLock<Vec<Box<dyn AlertHandler + Send + Sync>>>> =
        Arc::new(RwLock::new(vec![Box::new(TestAlertHandler {
            received_alerts: Arc::clone(&received),
        })]));
    let config = SystemMonitorConfig {
        alert_threshold_cpu: 80.0,
        alert_threshold_memory: 85.0,
        ..SystemMonitorConfig::default()
    };
    SystemMonitor::test_process_alerts(&metrics, &health, &handlers, &config)
        .await
        .unwrap();
    assert_eq!(received.lock().unwrap()[0].severity, AlertSeverity::Warning);

    let metrics_crit = Arc::new(RwLock::new(SystemMetrics {
        cpu_usage_percent: 10.0,
        memory_usage_percent: 96.0,
        ..SystemMetrics::default()
    }));
    let received2 = Arc::new(std::sync::Mutex::new(Vec::new()));
    let handlers2: Arc<RwLock<Vec<Box<dyn AlertHandler + Send + Sync>>>> =
        Arc::new(RwLock::new(vec![Box::new(TestAlertHandler {
            received_alerts: Arc::clone(&received2),
        })]));
    SystemMonitor::test_process_alerts(&metrics_crit, &health, &handlers2, &config)
        .await
        .unwrap();
    assert_eq!(
        received2.lock().unwrap()[0].severity,
        AlertSeverity::Critical
    );
}

#[tokio::test]
async fn test_process_alerts_component_unhealthy() {
    let metrics = Arc::new(RwLock::new(SystemMetrics::default()));
    let mut health_map = HashMap::new();
    health_map.insert(
        "gateway".to_string(),
        ComponentHealth {
            component_name: "gateway".to_string(),
            status: HealthStatus::Unhealthy,
            last_check: chrono::Utc::now(),
            response_time_ms: 5,
            error_count: 1,
            uptime_percent: 10.0,
        },
    );
    let health = Arc::new(RwLock::new(health_map));
    let received = Arc::new(std::sync::Mutex::new(Vec::new()));
    let handlers: Arc<RwLock<Vec<Box<dyn AlertHandler + Send + Sync>>>> =
        Arc::new(RwLock::new(vec![Box::new(TestAlertHandler {
            received_alerts: Arc::clone(&received),
        })]));
    let config = SystemMonitorConfig::default();
    SystemMonitor::test_process_alerts(&metrics, &health, &handlers, &config)
        .await
        .unwrap();
    let alerts = received.lock().unwrap();
    assert_eq!(alerts.len(), 1);
    assert_eq!(alerts[0].alert_type, AlertType::ComponentDown);
    assert_eq!(alerts[0].component.as_deref(), Some("gateway"));
}

#[tokio::test]
async fn test_process_alerts_handler_error_is_tolerated() {
    let metrics = Arc::new(RwLock::new(SystemMetrics {
        cpu_usage_percent: 99.0,
        memory_usage_percent: 10.0,
        ..SystemMetrics::default()
    }));
    let health = Arc::new(RwLock::new(HashMap::new()));
    let handlers: Arc<RwLock<Vec<Box<dyn AlertHandler + Send + Sync>>>> =
        Arc::new(RwLock::new(vec![Box::new(FailingAlertHandler)]));
    let config = SystemMonitorConfig {
        alert_threshold_cpu: 80.0,
        ..SystemMonitorConfig::default()
    };
    let res = SystemMonitor::test_process_alerts(&metrics, &health, &handlers, &config).await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_monitor_start_invokes_background_loop() {
    let mut monitor = SystemMonitor::with_config(SystemMonitorConfig {
        check_interval_ms: 20,
        ..SystemMonitorConfig::default()
    })
    .unwrap();
    monitor.start().unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(80)).await;
    let _ = monitor.stop();
}

#[tokio::test]
async fn test_get_component_health_returns_entry_after_background_check() {
    let mut monitor = SystemMonitor::with_config(SystemMonitorConfig {
        check_interval_ms: 15,
        ..SystemMonitorConfig::default()
    })
    .expect("monitor");
    monitor.start().expect("start");
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(3);
    loop {
        if let Some(h) = monitor.get_component_health("core").await {
            assert_eq!(h.component_name, "core");
            break;
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "timed out waiting for component health"
        );
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    }
    let _ = monitor.stop();
}

#[tokio::test]
async fn test_collect_system_metrics_updates_disk_and_network() {
    let metrics = Arc::new(RwLock::new(SystemMetrics::default()));
    SystemMonitor::test_collect_system_metrics(&metrics)
        .await
        .unwrap();
    let snap = metrics.read().await;
    assert!((30.0..80.0).contains(&snap.disk_usage_percent));
    assert!(snap.network_bytes_in > 0);
    assert!(snap.network_bytes_out > 0);
    assert!(snap.uptime_seconds >= 5);
}

#[tokio::test]
async fn test_system_monitor_clone_shares_metrics_snapshot() {
    let a = SystemMonitor::new().expect("new");
    let b = a.clone();
    let m1 = a.get_system_metrics().await;
    let m2 = b.get_system_metrics().await;
    assert_eq!(m1.cpu_usage_percent, m2.cpu_usage_percent);
}

#[tokio::test]
async fn test_process_alerts_notifies_all_handlers() {
    let r1 = Arc::new(std::sync::Mutex::new(Vec::new()));
    let r2 = Arc::new(std::sync::Mutex::new(Vec::new()));
    let metrics = Arc::new(RwLock::new(SystemMetrics {
        cpu_usage_percent: 99.0,
        memory_usage_percent: 10.0,
        ..SystemMetrics::default()
    }));
    let health = Arc::new(RwLock::new(HashMap::new()));
    let handlers: Arc<RwLock<Vec<Box<dyn AlertHandler + Send + Sync>>>> =
        Arc::new(RwLock::new(vec![
            Box::new(TestAlertHandler {
                received_alerts: Arc::clone(&r1),
            }),
            Box::new(TestAlertHandler {
                received_alerts: Arc::clone(&r2),
            }),
        ]));
    let config = SystemMonitorConfig {
        alert_threshold_cpu: 80.0,
        ..SystemMonitorConfig::default()
    };
    SystemMonitor::test_process_alerts(&metrics, &health, &handlers, &config)
        .await
        .unwrap();
    assert_eq!(r1.lock().unwrap().len(), 1);
    assert_eq!(r2.lock().unwrap().len(), 1);
}

#[test]
fn test_component_health_degraded_status() {
    let h = ComponentHealth {
        component_name: "edge".to_string(),
        status: HealthStatus::Degraded,
        last_check: chrono::Utc::now(),
        response_time_ms: 40,
        error_count: 2,
        uptime_percent: 88.0,
    };
    assert_eq!(h.status, HealthStatus::Degraded);
}
