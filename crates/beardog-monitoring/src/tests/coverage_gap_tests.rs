// SPDX-License-Identifier: AGPL-3.0-only

//! Coverage gap tests for beardog-monitoring
//!
//! Targets uncovered lines in:
//! - advanced_metrics/analysis.rs (0% → covered)
//! - monitoring/metrics.rs (57% → covered)
//! - monitoring/types.rs (53% → covered)
//! - advanced_metrics/storage.rs (32% → covered)
//! - advanced_metrics/performance.rs (25% → covered)
//! - advanced_metrics/ecosystem.rs (21% → covered)
//! - advanced_metrics/security.rs (62% → covered)
//! - monitoring/health/aggregator.rs (61% → covered)
//! - security_sentinel_example.rs (39% → covered)

use crate::advanced_metrics::analysis::{AnomalyDetector, TrendAnalyzer, TrendDirection};
use crate::advanced_metrics::config::{AnomalyConfig, TrendConfig};
use crate::advanced_metrics::ecosystem::EcosystemHealthMonitor;
use crate::advanced_metrics::performance::PerformanceAnalyzer;
use crate::advanced_metrics::security::SecurityMetrics;
use crate::advanced_metrics::storage::MetricsStore;
use crate::advanced_metrics::types::{
    CustomMetric, EcosystemMetric, InteractionType, MetricDataPoint, MetricStatistics, MetricType,
    PerformanceMetric, SecurityEvent, SecurityEventType, SecurityMetric, SecuritySeverity,
    ThreatLevel,
};
use crate::monitoring::health::aggregator::HealthCheckAggregator;
use crate::monitoring::metrics::{MetricsCollector, PrometheusExporter};
use crate::monitoring::types::{
    AlertSeverity, AlertThresholds, AlertType, ComponentHealth, MetricCollection,
    MetricValue as MonitoringMetricValue, MonitoringAlert, MonitoringConfig, PerformanceSnapshot,
    PrometheusConfig, ResourceUsage, SystemHealth,
};
use beardog_types::canonical::HealthStatus;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

// ═══════════════════════════════════════════════════════════════════
// Helper constructors for complex types
// ═══════════════════════════════════════════════════════════════════

fn make_data_point(value: f64) -> MetricDataPoint {
    MetricDataPoint {
        timestamp: SystemTime::now(),
        value,
        context: None,
    }
}

fn make_performance_metric(name: &str, value: f64) -> PerformanceMetric {
    PerformanceMetric {
        name: name.to_string(),
        value,
        history: Vec::new(),
        stats: MetricStatistics {
            min: value,
            max: value,
            avg: value,
            std_dev: 0.0,
            p95: value,
            p99: value,
            count: 1,
        },
        metric_type: MetricType::Gauge,
        last_updated: SystemTime::now(),
    }
}

fn make_security_metric(event_type: SecurityEventType, count: u64) -> SecurityMetric {
    SecurityMetric {
        event_type,
        count,
        severity_distribution: HashMap::new(),
        recent_events: Vec::new(),
        threat_level: ThreatLevel::Low,
    }
}

fn make_ecosystem_metric(service: &str, latency: f64) -> EcosystemMetric {
    EcosystemMetric {
        service: service.to_string(),
        interaction_type: InteractionType::ApiCall,
        success_rate: 0.99,
        avg_response_time: Duration::from_secs_f64(latency / 1000.0),
        request_volume: 100,
        error_distribution: HashMap::new(),
    }
}

fn make_custom_metric(name: &str, value: f64) -> CustomMetric {
    CustomMetric {
        name: name.to_string(),
        value: crate::advanced_metrics::types::MetricValue::Float(value),
        tags: HashMap::new(),
        timestamp: SystemTime::now(),
    }
}

fn make_security_event() -> SecurityEvent {
    SecurityEvent {
        timestamp: SystemTime::now(),
        event_type: SecurityEventType::AuthenticationFailure,
        severity: SecuritySeverity::Medium,
        description: "Failed login".to_string(),
        source: "test".to_string(),
        context: HashMap::new(),
    }
}

// ═══════════════════════════════════════════════════════════════════
// advanced_metrics/analysis.rs (0% → covered)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn test_anomaly_detector_insufficient_data() {
    let config = AnomalyConfig::default();
    let detector = AnomalyDetector::new(config);

    // Too few data points (default min_data_points = 10)
    let metrics = vec![make_data_point(100.0)];
    let anomalies = detector.detect_anomalies(&metrics);
    assert!(anomalies.is_empty());
}

#[test]
fn test_anomaly_detector_detects_anomaly() {
    let config = AnomalyConfig {
        sensitivity: 0.5,
        min_data_points: 3,
        ..AnomalyConfig::default()
    };
    let detector = AnomalyDetector::new(config);

    let mut metrics: Vec<MetricDataPoint> = (0..5).map(|_| make_data_point(10.0)).collect();
    // Add anomalous value far above threshold
    metrics.push(make_data_point(100.0));

    let anomalies = detector.detect_anomalies(&metrics);
    assert!(!anomalies.is_empty());
    assert!(anomalies[0].score > 0.0);
    assert!(anomalies[0].description.contains("exceeds threshold"));
}

#[test]
fn test_anomaly_detector_no_anomalies() {
    let config = AnomalyConfig {
        sensitivity: 0.5,
        min_data_points: 3,
        ..AnomalyConfig::default()
    };
    let detector = AnomalyDetector::new(config);

    let metrics: Vec<MetricDataPoint> = (0..5).map(|_| make_data_point(10.0)).collect();
    let anomalies = detector.detect_anomalies(&metrics);
    assert!(anomalies.is_empty());
}

#[test]
fn test_trend_analyzer_insufficient_data() {
    let config = TrendConfig::default();
    let analyzer = TrendAnalyzer::new(config);

    let metrics = vec![make_data_point(10.0)];
    let trend = analyzer.analyze_trends(&metrics);
    assert_eq!(trend.direction, TrendDirection::Stable);
    assert_eq!(trend.strength, 0.0);
    assert!(trend.predictions.is_empty());
}

#[test]
fn test_trend_analyzer_increasing_trend() {
    let config = TrendConfig::default();
    let analyzer = TrendAnalyzer::new(config);

    let metrics: Vec<MetricDataPoint> = (0..10).map(|i| make_data_point(i as f64 * 10.0)).collect();

    let trend = analyzer.analyze_trends(&metrics);
    assert_eq!(trend.direction, TrendDirection::Increasing);
    assert!(trend.strength > 0.0);
}

#[test]
fn test_trend_analyzer_decreasing_trend() {
    let config = TrendConfig::default();
    let analyzer = TrendAnalyzer::new(config);

    let metrics: Vec<MetricDataPoint> = (0..10)
        .map(|i| make_data_point((i as f64).mul_add(-10.0, 100.0)))
        .collect();

    let trend = analyzer.analyze_trends(&metrics);
    assert_eq!(trend.direction, TrendDirection::Decreasing);
    assert!(trend.strength > 0.0);
}

#[test]
fn test_trend_analyzer_stable_trend() {
    let config = TrendConfig::default();
    let analyzer = TrendAnalyzer::new(config);

    let metrics: Vec<MetricDataPoint> = (0..5).map(|_| make_data_point(50.0)).collect();

    let trend = analyzer.analyze_trends(&metrics);
    assert_eq!(trend.direction, TrendDirection::Stable);
}

#[test]
fn test_trend_analyzer_more_than_10_points_uses_last_10() {
    let config = TrendConfig::default();
    let analyzer = TrendAnalyzer::new(config);

    let metrics: Vec<MetricDataPoint> = (0..20).map(|i| make_data_point(i as f64 * 5.0)).collect();

    let trend = analyzer.analyze_trends(&metrics);
    assert_eq!(trend.direction, TrendDirection::Increasing);
}

#[test]
fn test_trend_analyzer_exactly_two_data_points() {
    let config = TrendConfig::default();
    let analyzer = TrendAnalyzer::new(config);

    let metrics = vec![make_data_point(10.0), make_data_point(20.0)];
    let trend = analyzer.analyze_trends(&metrics);
    // Needs ≥ 3 points
    assert_eq!(trend.direction, TrendDirection::Stable);
    assert_eq!(trend.strength, 0.0);
}

// ═══════════════════════════════════════════════════════════════════
// advanced_metrics/storage.rs (32% → covered)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn test_metrics_store_new() {
    let store = MetricsStore::new();
    assert_eq!(store.total_metrics(), 0);
    assert!(store.performance.is_empty());
    assert!(store.security.is_empty());
    assert!(store.ecosystem.is_empty());
    assert!(store.custom.is_empty());
}

#[test]
fn test_metrics_store_performance() {
    let mut store = MetricsStore::new();
    let metric = make_performance_metric("cpu_usage", 75.0);

    store.store_performance_metric(metric);
    assert_eq!(store.total_metrics(), 1);

    let retrieved = store.get_performance_metric("cpu_usage");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().value, 75.0);

    assert!(store.get_performance_metric("nonexistent").is_none());
}

#[test]
fn test_metrics_store_security() {
    let mut store = MetricsStore::new();
    let metric = make_security_metric(SecurityEventType::AuthenticationFailure, 5);

    store.store_security_metric("AuthenticationFailure".to_string(), metric);
    assert_eq!(store.total_metrics(), 1);

    let retrieved = store.get_security_metric(&SecurityEventType::AuthenticationFailure);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().count, 5);
}

#[test]
fn test_metrics_store_ecosystem() {
    let mut store = MetricsStore::new();
    let metric = make_ecosystem_metric("songbird", 5.0);

    store.store_ecosystem_metric(metric);
    assert_eq!(store.total_metrics(), 1);

    let retrieved = store.get_ecosystem_metric("songbird");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().success_rate, 0.99);

    assert!(store.get_ecosystem_metric("unknown").is_none());
}

#[test]
fn test_metrics_store_custom() {
    let mut store = MetricsStore::new();
    let metric = make_custom_metric("custom_metric", 42.0);

    store.store_custom_metric(metric);
    assert_eq!(store.total_metrics(), 1);

    let retrieved = store.get_custom_metric("custom_metric");
    assert!(retrieved.is_some());
}

#[test]
fn test_metrics_store_default() {
    let store = MetricsStore::default();
    assert_eq!(store.total_metrics(), 0);
}

// ═══════════════════════════════════════════════════════════════════
// advanced_metrics/performance.rs (25% → covered)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn test_performance_analyzer_new() {
    let analyzer = PerformanceAnalyzer::new();
    let metric = make_performance_metric("test", 50.0);
    assert!(analyzer.analyze_metric(&metric).is_ok());
}

#[test]
fn test_performance_analyzer_default() {
    let analyzer = PerformanceAnalyzer::default();
    let metric = make_performance_metric("default_test", 10.0);
    assert!(analyzer.analyze_metric(&metric).is_ok());
}

// ═══════════════════════════════════════════════════════════════════
// advanced_metrics/security.rs (62% → covered)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn test_security_metrics_new() {
    let mut metrics = SecurityMetrics::new();
    let event = make_security_event();
    metrics.record_event(event);
}

#[test]
fn test_security_metrics_default() {
    let metrics = SecurityMetrics::default();
    let _ = format!("{metrics:?}");
}

// ═══════════════════════════════════════════════════════════════════
// advanced_metrics/ecosystem.rs (21% → covered)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn test_ecosystem_health_monitor_new() {
    let monitor = EcosystemHealthMonitor::new();
    let status = monitor.check_health();
    assert_eq!(
        status.overall_status,
        crate::advanced_metrics::types::HealthStatus::Healthy
    );
    assert_eq!(status.active_alerts, 0);
}

#[test]
fn test_ecosystem_health_monitor_default() {
    let monitor = EcosystemHealthMonitor::default();
    let status = monitor.check_health();
    assert!(status.services.is_empty());
    assert!(status.system_metrics.is_empty());
}

// ═══════════════════════════════════════════════════════════════════
// monitoring/types.rs (53% → covered)
// ═══════════════════════════════════════════════════════════════════

#[test]
fn test_performance_snapshot_default() {
    let snap = PerformanceSnapshot::default();
    assert_eq!(snap.cpu_usage, 0.0);
    assert_eq!(snap.memory_usage, 0.0);
    assert_eq!(snap.memory_total, 0);
    assert_eq!(snap.memory_used, 0);
    assert_eq!(snap.disk_usage, 0.0);
    assert_eq!(snap.disk_total, 0);
    assert_eq!(snap.disk_used, 0);
    assert_eq!(snap.network_rx, 0);
    assert_eq!(snap.network_tx, 0);
    assert_eq!(snap.load_average_1m, 0.0);
    assert_eq!(snap.load_average_5m, 0.0);
    assert_eq!(snap.load_average_15m, 0.0);
    assert_eq!(snap.active_connections, 0);
    assert_eq!(snap.uptime_seconds, 0);
}

#[test]
fn test_resource_usage_default() {
    let usage = ResourceUsage::default();
    assert_eq!(usage.cpu_percent, 0.0);
    assert_eq!(usage.memory_bytes, 0);
    assert_eq!(usage.disk_bytes, 0);
    assert_eq!(usage.network_in_bytes, 0);
    assert_eq!(usage.network_out_bytes, 0);
    assert_eq!(usage.open_file_descriptors, 0);
    assert_eq!(usage.active_threads, 0);
}

#[test]
fn test_alert_thresholds_default() {
    let thresholds = AlertThresholds::default();
    assert_eq!(thresholds.cpu_threshold, 80.0);
    assert_eq!(thresholds.memory_threshold, 85.0);
    assert_eq!(thresholds.disk_threshold, 90.0);
    assert_eq!(thresholds.error_rate_threshold, 5.0);
}

#[test]
fn test_metric_collection_default() {
    let collection = MetricCollection::default();
    assert!(collection.counters.is_empty());
    assert!(collection.gauges.is_empty());
    assert!(collection.histograms.is_empty());
    assert!(collection.timers.is_empty());
}

#[test]
fn test_metric_value_variants() {
    let counter = MonitoringMetricValue::Counter(100);
    let gauge = MonitoringMetricValue::Gauge(std::f64::consts::PI);
    let histogram = MonitoringMetricValue::Histogram(vec![1.0, 2.0, 3.0]);
    let timer = MonitoringMetricValue::Timer(Duration::from_secs(1));

    assert!(matches!(counter, MonitoringMetricValue::Counter(100)));
    assert!(matches!(gauge, MonitoringMetricValue::Gauge(_)));
    assert!(matches!(histogram, MonitoringMetricValue::Histogram(_)));
    assert!(matches!(timer, MonitoringMetricValue::Timer(_)));
}

#[test]
fn test_alert_type_variants() {
    let performance = AlertType::Performance;
    let security = AlertType::Security;
    let resource = AlertType::Resource;
    let error = AlertType::Error;
    let custom = AlertType::Custom("custom".to_string());

    assert_eq!(performance, AlertType::Performance);
    assert_eq!(security, AlertType::Security);
    assert_eq!(resource, AlertType::Resource);
    assert_eq!(error, AlertType::Error);
    assert!(matches!(custom, AlertType::Custom(_)));
}

#[test]
fn test_alert_severity_ordering() {
    assert!(AlertSeverity::Critical > AlertSeverity::High);
    assert!(AlertSeverity::High > AlertSeverity::Medium);
    assert!(AlertSeverity::Medium > AlertSeverity::Low);
}

#[test]
fn test_prometheus_config_default() {
    let config = PrometheusConfig::default();
    assert!(config.enabled);
    assert_eq!(config.endpoint, "/metrics");
    assert_eq!(config.prefix, env!("CARGO_PKG_NAME"));
}

#[test]
fn test_monitoring_config_default() {
    let config = MonitoringConfig::default();
    assert!(config.enabled);
    assert_eq!(config.max_snapshots, 100);
    assert_eq!(config.snapshot_interval_seconds, 60);
}

#[test]
fn test_monitoring_alert() {
    let alert = MonitoringAlert {
        id: "alert-1".to_string(),
        message: "High CPU".to_string(),
        severity: AlertSeverity::High,
        created_at: Utc::now(),
    };
    assert_eq!(alert.id, "alert-1");
    assert_eq!(alert.severity, AlertSeverity::High);
}

#[test]
fn test_component_health() {
    let health = ComponentHealth {
        name: "database".to_string(),
        status: HealthStatus::Healthy,
        message: Some("OK".to_string()),
        last_check: Utc::now(),
        check_duration_ms: 5,
        metadata: HashMap::new(),
    };
    assert_eq!(health.name, "database");
    assert_eq!(health.status, HealthStatus::Healthy);
}

#[test]
fn test_system_health() {
    let health = SystemHealth {
        overall_status: HealthStatus::Healthy,
        components: vec![],
        timestamp: Utc::now(),
    };
    assert_eq!(health.overall_status, HealthStatus::Healthy);
    assert!(health.components.is_empty());
}

// ═══════════════════════════════════════════════════════════════════
// monitoring/metrics.rs (57% → covered)
// ═══════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_metrics_collector_record_all_types() {
    let collector = MetricsCollector::new();

    collector
        .record_counter("requests", 100)
        .await
        .expect("counter");
    collector.record_gauge("cpu", 75.5).await.expect("gauge");
    collector
        .record_histogram("latency", vec![1.0, 2.0, 3.0])
        .await
        .expect("histogram");
    collector
        .record_timer("response", Duration::from_millis(50))
        .await
        .expect("timer");

    assert_eq!(collector.get_metric_count(), 4);

    let all = collector.get_all_metrics().await.expect("get_all");
    assert_eq!(all.len(), 4);
}

#[tokio::test]
async fn test_metrics_collector_clear() {
    let collector = MetricsCollector::new();
    collector.record_counter("a", 1).await.expect("record");
    collector.record_counter("b", 2).await.expect("record");

    collector.clear_metrics().await.expect("clear");
    assert_eq!(collector.get_metric_count(), 0);

    let all = collector.get_all_metrics().await.expect("get_all");
    assert!(all.is_empty());
}

#[tokio::test]
async fn test_metrics_collector_default() {
    let collector = MetricsCollector::default();
    assert_eq!(collector.get_metric_count(), 0);
}

#[tokio::test]
async fn test_prometheus_exporter_export_all_types() {
    let collector = Arc::new(MetricsCollector::new());
    collector.record_counter("reqs", 42).await.expect("record");
    collector.record_gauge("temp", 22.5).await.expect("record");
    collector
        .record_histogram("vals", vec![1.0, 2.0])
        .await
        .expect("record");
    collector
        .record_timer("op", Duration::from_millis(100))
        .await
        .expect("record");

    let config = PrometheusConfig::default();
    let exporter = PrometheusExporter::new(config, collector);

    let output = exporter.export_metrics().await.expect("export");
    assert!(output.contains("beardog_"));
}

#[tokio::test]
async fn test_prometheus_exporter_empty() {
    let collector = Arc::new(MetricsCollector::new());
    let config = PrometheusConfig::default();
    let exporter = PrometheusExporter::new(config, collector);

    let output = exporter.export_metrics().await.expect("export");
    assert!(output.is_empty());
}

#[test]
fn test_prometheus_exporter_health_enabled() {
    let collector = Arc::new(MetricsCollector::new());
    let config = PrometheusConfig {
        enabled: true,
        ..PrometheusConfig::default()
    };
    let exporter = PrometheusExporter::new(config, collector);
    assert_eq!(exporter.get_health_status(), HealthStatus::Healthy);
}

#[test]
fn test_prometheus_exporter_health_disabled() {
    let collector = Arc::new(MetricsCollector::new());
    let config = PrometheusConfig {
        enabled: false,
        ..PrometheusConfig::default()
    };
    let exporter = PrometheusExporter::new(config, collector);
    assert_eq!(exporter.get_health_status(), HealthStatus::Unhealthy);
}

// ═══════════════════════════════════════════════════════════════════
// monitoring/health/aggregator.rs (61% → covered)
// ═══════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_aggregator_default_no_checkers() {
    let aggregator = HealthCheckAggregator::default();
    let results = aggregator.check_all().await.expect("check_all");
    assert!(results.is_empty());

    let status = aggregator.get_overall_status().await.expect("overall");
    assert_eq!(status, HealthStatus::Healthy);
}

#[tokio::test]
async fn test_aggregator_new_is_empty() {
    let aggregator = HealthCheckAggregator::new();
    let results = aggregator.check_all().await.expect("check_all");
    assert!(results.is_empty());
}

// ═══════════════════════════════════════════════════════════════════
// security_sentinel_example.rs (39% → covered)
// ═══════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_demonstrate_security_sentinel() {
    let result = crate::security_sentinel_example::demonstrate_security_sentinel().await;
    assert!(result.is_ok());
}
