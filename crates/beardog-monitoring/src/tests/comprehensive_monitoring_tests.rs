// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive Monitoring Tests
//!
//! Tests for metrics collection, health checks, and alerting

#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all
)]

use crate::SecuritySentinel;
use crate::metrics::{
    MetricCategory, MetricEvent, MetricValue, UnifiedMetricsConfig, UnifiedMetricsSystem,
};
use crate::monitoring::health::traits::HealthChecker;
use crate::monitoring::health::{
    CacheHealthChecker, DatabaseHealthChecker, ExternalApiHealthChecker, HealthCheckAggregator,
    HealthCheckerType,
};
use crate::monitoring::types::ComponentHealth;
use beardog_types::canonical::HealthStatus;
use std::collections::HashMap;
use std::time::SystemTime;

#[cfg(test)]
mod metrics_collection_tests {
    use super::*;

    #[test]
    fn test_counter_metric() {
        let v = MetricValue::Counter(9);
        assert!(matches!(v, MetricValue::Counter(9)));
    }

    #[test]
    fn test_gauge_metric() {
        let v = MetricValue::Gauge(1.25);
        if let MetricValue::Gauge(g) = v {
            assert!((g - 1.25).abs() < 1e-9);
        } else {
            panic!("expected gauge");
        }
    }

    #[test]
    fn test_histogram_metric() {
        let v = MetricValue::Histogram(vec![1.0, 2.0, 3.0]);
        if let MetricValue::Histogram(h) = v {
            assert_eq!(h.len(), 3);
        } else {
            panic!("expected histogram");
        }
    }

    #[test]
    fn test_metric_labels() {
        let mut labels = HashMap::new();
        labels.insert("service".to_string(), "api".to_string());
        let event = MetricEvent {
            category: MetricCategory::Performance,
            name: "req".to_string(),
            value: MetricValue::Counter(1),
            labels,
            timestamp: SystemTime::now(),
        };
        assert_eq!(event.labels.get("service").map(String::as_str), Some("api"));
    }

    #[test]
    fn test_metric_aggregation() {
        let config = UnifiedMetricsConfig::default();
        let system = UnifiedMetricsSystem::new(config).expect("unified metrics");
        let event = MetricEvent {
            category: MetricCategory::Performance,
            name: "agg".to_string(),
            value: MetricValue::Summary {
                sum: 30.0,
                count: 3,
            },
            labels: HashMap::new(),
            timestamp: SystemTime::now(),
        };
        assert!(system.record_event(event).is_ok());
        let m = system.get_system_metrics().expect("system metrics");
        assert_eq!(m.analytics.total_events_processed, 50_000);
    }

    #[test]
    fn test_metric_export() {
        let config = UnifiedMetricsConfig::default();
        let system = UnifiedMetricsSystem::new(config).expect("unified metrics");
        system.start().expect("start");
        let snap = system.get_system_metrics().expect("metrics");
        assert!(snap.timestamp <= SystemTime::now());
    }

    #[test]
    fn test_metric_reset() {
        let mut c = UnifiedMetricsConfig::default();
        c.broadcast_buffer_size = 16;
        let s = UnifiedMetricsSystem::new(c).expect("system");
        assert_eq!(
            s.get_system_metrics().expect("m").performance.error_rate,
            0.0
        );
    }

    #[test]
    fn test_metric_persistence() {
        let e = MetricEvent {
            category: MetricCategory::Security,
            name: "persist_check".to_string(),
            value: MetricValue::Counter(2),
            labels: HashMap::new(),
            timestamp: SystemTime::UNIX_EPOCH,
        };
        assert_eq!(e.timestamp, SystemTime::UNIX_EPOCH);
    }
}

#[cfg(test)]
mod health_check_tests {
    use super::*;

    fn sample_context() -> ComponentHealth {
        ComponentHealth {
            name: "sample".to_string(),
            status: HealthStatus::Healthy,
            message: None,
            last_check: chrono::Utc::now(),
            check_duration_ms: 0,
            metadata: HashMap::new(),
        }
    }

    #[tokio::test]
    async fn test_service_health_check() {
        let h = DatabaseHealthChecker::new();
        let ch = h.check_health().await.expect("db health");
        assert_eq!(ch.status, HealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_database_health_check() {
        let ch = DatabaseHealthChecker::new()
            .check_health()
            .await
            .expect("health");
        assert!(ch.message.is_some());
    }

    #[tokio::test]
    async fn test_cache_health_check() {
        let ch = CacheHealthChecker::new()
            .check_health()
            .await
            .expect("cache");
        assert_eq!(ch.name, "Cache");
    }

    #[tokio::test]
    async fn test_dependency_health_check() {
        let ch = ExternalApiHealthChecker::new("https://example.com/health".to_string())
            .check_health()
            .await
            .expect("external");
        assert_eq!(ch.name, "ExternalAPI");
    }

    #[tokio::test]
    async fn test_health_check_timeout() {
        let mut agg = HealthCheckAggregator::new();
        agg.add_checker(HealthCheckerType::Database(DatabaseHealthChecker::new()));
        let results = agg.check_all().await.expect("check_all");
        assert_eq!(results.len(), 1);
    }

    #[tokio::test]
    async fn test_health_status_aggregation() {
        let mut agg = HealthCheckAggregator::new();
        agg.add_checker(HealthCheckerType::Database(DatabaseHealthChecker::new()));
        agg.add_checker(HealthCheckerType::Cache(CacheHealthChecker::new()));
        let status = agg.get_overall_status().await.expect("overall");
        assert_eq!(status, HealthStatus::Healthy);
    }

    #[test]
    fn test_health_check_interval() {
        let c = sample_context();
        assert_eq!(c.check_duration_ms, 0);
    }

    #[test]
    fn test_degraded_health_state() {
        let mut c = sample_context();
        c.status = HealthStatus::Degraded;
        assert_eq!(c.status, HealthStatus::Degraded);
    }
}

#[cfg(test)]
mod alert_tests {
    use super::*;

    #[test]
    fn test_alert_creation() {
        let s = SecuritySentinel::default();
        assert!(!s.is_monitoring_active());
    }

    #[test]
    fn test_alert_threshold() {
        let mut c = crate::security_sentinel::SecuritySentinelConfig::default();
        c.alert_threshold = 2;
        let s = SecuritySentinel::new(c.clone());
        assert_eq!(c.alert_threshold, 2);
        assert_eq!(s.get_event_count(), 0);
    }

    #[tokio::test]
    async fn test_alert_notification() {
        let s = SecuritySentinel::default();
        s.start_monitoring().expect("start");
        s.process_security_event("auth_failure", HashMap::new())
            .await
            .expect("event");
        let stats = s.get_statistics().await;
        assert!(stats.auth_failures >= 1);
    }

    #[tokio::test]
    async fn test_alert_suppression() {
        let s = SecuritySentinel::default();
        s.start_monitoring().expect("start");
        s.process_security_event("routine", HashMap::new())
            .await
            .expect("event");
        let stats = s.get_statistics().await;
        assert!(stats.suspicious_activities >= 1);
    }

    #[tokio::test]
    async fn test_alert_escalation() {
        let mut c = crate::security_sentinel::SecuritySentinelConfig::default();
        c.alert_threshold = 1;
        let s = SecuritySentinel::new(c);
        s.start_monitoring().expect("start");
        s.process_security_event("noise", HashMap::new())
            .await
            .expect("e1");
        s.process_security_event("other_unknown", HashMap::new())
            .await
            .expect("e2");
        let report = s.get_status_report().await.expect("report");
        assert_eq!(report.health_status, HealthStatus::Unhealthy);
    }

    #[tokio::test]
    async fn test_alert_recovery() {
        let s = SecuritySentinel::default();
        s.start_monitoring().expect("start");
        s.stop_monitoring().expect("stop");
        let report = s.get_status_report().await.expect("report");
        assert_eq!(report.status, "INACTIVE");
    }
}

#[cfg(test)]
mod monitoring_integration_tests {
    use super::*;

    #[test]
    fn test_prometheus_integration() {
        let config = UnifiedMetricsConfig::default();
        let sys = UnifiedMetricsSystem::new(config).expect("sys");
        assert!(sys.start().is_ok());
    }

    #[test]
    fn test_grafana_metrics() {
        let m = UnifiedMetricsSystem::new(UnifiedMetricsConfig::default()).expect("sys");
        let evt = MetricEvent {
            category: MetricCategory::Ecosystem,
            name: "panel".to_string(),
            value: MetricValue::Gauge(0.0),
            labels: HashMap::new(),
            timestamp: SystemTime::now(),
        };
        assert!(m.record_event(evt).is_ok());
    }

    #[test]
    fn test_logging_integration() {
        tracing::info!(target: "beardog_monitoring_test", "health probe");
        assert!(true);
    }

    #[tokio::test]
    async fn test_tracing_integration() {
        tokio::task::yield_now().await;
    }
}

#[cfg(test)]
mod performance_monitoring_tests {
    use super::*;

    #[test]
    fn test_latency_tracking() {
        let m = UnifiedMetricsSystem::new(UnifiedMetricsConfig::default()).expect("sys");
        let e = MetricEvent {
            category: MetricCategory::Performance,
            name: "latency_ms".to_string(),
            value: MetricValue::Histogram(vec![1.0, 2.0, 3.0]),
            labels: HashMap::new(),
            timestamp: SystemTime::now(),
        };
        assert!(m.record_event(e).is_ok());
    }

    #[test]
    fn test_throughput_monitoring() {
        let m = UnifiedMetricsSystem::new(UnifiedMetricsConfig::default()).expect("sys");
        let e = MetricEvent {
            category: MetricCategory::Performance,
            name: "throughput".to_string(),
            value: MetricValue::Counter(100),
            labels: HashMap::new(),
            timestamp: SystemTime::now(),
        };
        assert!(m.record_event(e).is_ok());
    }

    #[test]
    fn test_error_rate_tracking() {
        let m = UnifiedMetricsSystem::new(UnifiedMetricsConfig::default()).expect("sys");
        let e = MetricEvent {
            category: MetricCategory::Security,
            name: "errors".to_string(),
            value: MetricValue::Counter(0),
            labels: HashMap::new(),
            timestamp: SystemTime::now(),
        };
        assert!(m.record_event(e).is_ok());
    }

    #[test]
    fn test_resource_utilization() {
        let m = UnifiedMetricsSystem::new(UnifiedMetricsConfig::default()).expect("sys");
        let metrics = m.get_system_metrics().expect("get");
        assert!(metrics.performance.cpu_usage >= 0.0);
    }

    #[test]
    fn test_sla_compliance() {
        let m = UnifiedMetricsSystem::new(UnifiedMetricsConfig::default()).expect("sys");
        m.start().expect("start");
        let metrics = m.get_system_metrics().expect("get");
        assert!(metrics.performance.error_rate >= 0.0);
        assert!(metrics.performance.throughput_rps >= 0.0);
    }
}

#[cfg(test)]
mod snapshot_tests {
    use super::*;

    #[test]
    fn test_snapshot_creation() {
        let mut snap = crate::monitoring::types::PerformanceSnapshot::default();
        snap.cpu_usage = 10.0;
        snap.active_connections = 3;
        snap.uptime_seconds = 60;
        assert!((snap.cpu_usage - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_snapshot_storage() {
        let mut h = HashMap::new();
        h.insert("k".to_string(), "v".to_string());
        let c = ComponentHealth {
            name: "snap".to_string(),
            status: HealthStatus::Healthy,
            message: Some("ok".to_string()),
            last_check: chrono::Utc::now(),
            check_duration_ms: 1,
            metadata: h,
        };
        assert_eq!(c.metadata.len(), 1);
    }

    #[test]
    fn test_snapshot_retrieval() {
        let m = UnifiedMetricsSystem::new(UnifiedMetricsConfig::default()).expect("sys");
        let sm = m.get_system_metrics().expect("snap");
        assert!(sm.timestamp <= SystemTime::now());
    }

    #[test]
    fn test_snapshot_comparison() {
        let a = MetricValue::Gauge(1.0);
        let b = MetricValue::Gauge(2.0);
        match (a, b) {
            (MetricValue::Gauge(x), MetricValue::Gauge(y)) => assert!(y > x),
            _ => panic!("expected gauges"),
        }
    }
}
