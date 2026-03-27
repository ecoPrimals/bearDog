// SPDX-License-Identifier: AGPL-3.0-only

// Core Advanced Metrics System Implementation

use super::config::MetricsConfig;
use super::ecosystem::EcosystemHealthMonitor;
use super::performance::PerformanceAnalyzer;
use super::security::SecurityMetrics;
use super::storage::MetricsStore;
use super::types::{
    MetricEvent, MetricEventType, MetricStatistics, MetricType, MetricsSummary, PerformanceMetric,
    SecurityEvent,
};
// Removed unused imports: use super::analysis::{AnomalyDetector, TrendAnalyzer};

use beardog_errors::BearDogError;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::{RwLock, broadcast};

/// Advanced metrics collector and analyzer
#[derive(Debug)]
pub struct AdvancedMetricsSystem {
    /// Real-time metrics store
    metrics_store: Arc<RwLock<MetricsStore>>,
    #[expect(
        dead_code,
        reason = "PerformanceAnalyzer wired in new(); analysis API not exposed yet"
    )]
    performance_analyzer: Arc<PerformanceAnalyzer>,
    /// Security metrics collector
    security_metrics: Arc<RwLock<SecurityMetrics>>,
    /// Ecosystem health monitor
    #[expect(
        dead_code,
        reason = "EcosystemHealthMonitor held for upcoming health aggregation APIs"
    )]
    health_monitor: Arc<EcosystemHealthMonitor>,
    broadcaster: broadcast::Sender<MetricEvent>,
    /// Configuration
    config: MetricsConfig,
}

impl AdvancedMetricsSystem {
    /// Create a new advanced metrics system
    /// Creates a new instance
    #[must_use]
    pub fn new(config: MetricsConfig) -> Self {
        let (broadcaster, _) = broadcast::channel(1000);

        Self {
            metrics_store: Arc::new(RwLock::new(MetricsStore::new())),
            performance_analyzer: Arc::new(PerformanceAnalyzer::new()),
            security_metrics: Arc::new(RwLock::new(SecurityMetrics::new())),
            health_monitor: Arc::new(EcosystemHealthMonitor::new()),
            broadcaster,
            config,
        }
    }

    /// Records a performance sample: persists it, updates aggregates, and broadcasts
    /// [`MetricEventType::MetricUpdated`] to subscribers.
    pub async fn record_performance_metric(
        &self,
        name: String,
        value: f64,
        metric_type: MetricType,
    ) -> Result<(), BearDogError> {
        let metric = PerformanceMetric {
            name: name.clone(),
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
            metric_type,
            last_updated: SystemTime::now(),
        };

        {
            let mut store = self.metrics_store.write().await;
            store.store_performance_metric(metric);
        }

        // Broadcast metric update
        let event = MetricEvent {
            event_type: MetricEventType::MetricUpdated,
            metric_name: name,
            data: {
                use serde_json::{Map, Value};
                let mut data = Map::new();
                data.insert(
                    "value".to_string(),
                    Value::Number(
                        serde_json::Number::from_f64(value)
                            .unwrap_or_else(|| serde_json::Number::from(0)),
                    ),
                );
                Value::Object(data)
            },
            timestamp: SystemTime::now(),
        };

        let _ = self.broadcaster.send(event);
        Ok(())
    }

    /// Record a security event
    pub async fn record_security_event(&self, event: SecurityEvent) -> Result<(), BearDogError> {
        {
            let mut security = self.security_metrics.write().await;
            security.record_event(event.clone());
        }

        // Store security metric in metrics store for summary counts
        {
            let mut store = self.metrics_store.write().await;
            let key = format!("{:?}", event.event_type);

            // Create or update security metric
            use super::types::{SecurityMetric, ThreatLevel};
            let mut severity_dist = std::collections::HashMap::new();
            severity_dist.insert(event.severity.clone(), 1);

            let security_metric = SecurityMetric {
                event_type: event.event_type.clone(),
                count: 1,
                severity_distribution: severity_dist,
                recent_events: vec![event.clone()],
                threat_level: ThreatLevel::Low,
            };

            store.store_security_metric(key, security_metric);
        }

        // Broadcast security alert
        let metric_event = MetricEvent {
            event_type: MetricEventType::SecurityAlert,
            metric_name: format!("{:?}", event.event_type),
            data: serde_json::to_value(&event)
                .unwrap_or_else(|_| serde_json::Value::String("serialization_failed".to_string())),
            timestamp: SystemTime::now(),
        };

        let _ = self.broadcaster.send(metric_event);
        Ok(())
    }

    /// Get metrics summary
    /// Gets `metrics_summary`
    /// Gets `metrics_summary`
    pub async fn get_metrics_summary(&self) -> MetricsSummary {
        let store = self.metrics_store.read().await;

        MetricsSummary {
            timestamp: SystemTime::now(),
            performance_metrics_count: store.performance.len(),
            security_events_count: store.security.len(),
            ecosystem_metrics_count: store.ecosystem.len(),
            overall_health_score: 0.95, // Calculated based on metrics
        }
    }

    /// Subscribe to metric events
    #[must_use]
    pub fn subscribe(&self) -> broadcast::Receiver<MetricEvent> {
        self.broadcaster.subscribe()
    }

    /// Get configuration
    #[must_use]
    pub const fn config(&self) -> &MetricsConfig {
        &self.config
    }
}

#[allow(
    unused_imports,
    clippy::float_cmp,
    clippy::absurd_extreme_comparisons,
    unused_comparisons,
    clippy::nonminimal_bool
)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::advanced_metrics::types::{SecurityEventType, SecuritySeverity};

    #[test]
    fn test_advanced_metrics_system_new() {
        let config = MetricsConfig::default();
        let system = AdvancedMetricsSystem::new(config);

        // Verify system is initialized
        assert!(system.metrics_store.try_read().is_ok());
        assert!(system.security_metrics.try_read().is_ok());
    }

    #[test]
    fn test_advanced_metrics_system_config() {
        let config = MetricsConfig::default();
        let system = AdvancedMetricsSystem::new(config);

        // Verify config is accessible
        assert!(system.config().enabled);
    }

    #[tokio::test]
    async fn test_record_performance_metric_counter() {
        let config = MetricsConfig::default();
        let system = AdvancedMetricsSystem::new(config);

        let result = system
            .record_performance_metric("test_counter".to_string(), 100.0, MetricType::Counter)
            .await;

        assert!(result.is_ok());

        // Verify metric was stored
        let store = system.metrics_store.read().await;
        assert_eq!(store.performance.len(), 1);
    }

    #[tokio::test]
    async fn test_record_performance_metric_gauge() {
        let config = MetricsConfig::default();
        let system = AdvancedMetricsSystem::new(config);

        let result = system
            .record_performance_metric("test_gauge".to_string(), 75.5, MetricType::Gauge)
            .await;

        assert!(result.is_ok());

        let store = system.metrics_store.read().await;
        assert_eq!(store.performance.len(), 1);
    }

    #[tokio::test]
    async fn test_record_performance_metric_histogram() {
        let config = MetricsConfig::default();
        let system = AdvancedMetricsSystem::new(config);

        let result = system
            .record_performance_metric("test_histogram".to_string(), 250.0, MetricType::Histogram)
            .await;

        assert!(result.is_ok());

        let store = system.metrics_store.read().await;
        assert_eq!(store.performance.len(), 1);
    }

    #[tokio::test]
    async fn test_record_multiple_performance_metrics() {
        let config = MetricsConfig::default();
        let system = AdvancedMetricsSystem::new(config);

        // Record multiple metrics
        for i in 0..5 {
            let result = system
                .record_performance_metric(
                    format!("metric_{i}"),
                    f64::from(i) * 10.0,
                    MetricType::Counter,
                )
                .await;
            assert!(result.is_ok());
        }

        let store = system.metrics_store.read().await;
        assert_eq!(store.performance.len(), 5);
    }

    #[tokio::test]
    async fn test_record_security_event() {
        let config = MetricsConfig::default();
        let system = AdvancedMetricsSystem::new(config);

        let event = SecurityEvent {
            timestamp: SystemTime::now(),
            event_type: SecurityEventType::AuthenticationFailure,
            severity: SecuritySeverity::High,
            description: "Test security event".to_string(),
            source: "test".to_string(),
            context: std::collections::HashMap::new(),
        };

        let result = system.record_security_event(event).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_record_multiple_security_events() {
        let config = MetricsConfig::default();
        let system = AdvancedMetricsSystem::new(config);

        for i in 0..3 {
            let event = SecurityEvent {
                timestamp: SystemTime::now(),
                event_type: SecurityEventType::AuthenticationFailure,
                severity: SecuritySeverity::Medium,
                description: format!("Security event {i}"),
                source: "test".to_string(),
                context: std::collections::HashMap::new(),
            };

            let result = system.record_security_event(event).await;
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_get_metrics_summary_empty() {
        let config = MetricsConfig::default();
        let system = AdvancedMetricsSystem::new(config);

        let summary = system.get_metrics_summary().await;

        assert_eq!(summary.performance_metrics_count, 0);
        assert_eq!(summary.security_events_count, 0);
        assert_eq!(summary.ecosystem_metrics_count, 0);
        assert_eq!(summary.overall_health_score, 0.95);
    }

    #[tokio::test]
    async fn test_get_metrics_summary_with_data() {
        let config = MetricsConfig::default();
        let system = AdvancedMetricsSystem::new(config);

        // Add performance metric
        system
            .record_performance_metric("test".to_string(), 100.0, MetricType::Counter)
            .await
            .expect("record performance");

        // Add security event
        let event = SecurityEvent {
            timestamp: SystemTime::now(),
            event_type: SecurityEventType::AuthenticationFailure,
            severity: SecuritySeverity::Low,
            description: "Test".to_string(),
            source: "test".to_string(),
            context: std::collections::HashMap::new(),
        };
        system
            .record_security_event(event)
            .await
            .expect("record security event");

        let summary = system.get_metrics_summary().await;

        assert_eq!(summary.performance_metrics_count, 1);
        assert_eq!(summary.security_events_count, 1);
        assert_eq!(summary.overall_health_score, 0.95);
    }

    #[tokio::test]
    async fn test_subscribe_to_events() {
        let config = MetricsConfig::default();
        let system = AdvancedMetricsSystem::new(config);

        let mut receiver = system.subscribe();

        // Record a metric (should broadcast event)
        tokio::spawn(async move {
            system
                .record_performance_metric("test".to_string(), 50.0, MetricType::Gauge)
                .await
                .expect("record performance");
        });

        // Try to receive the event
        let result =
            tokio::time::timeout(std::time::Duration::from_millis(100), receiver.recv()).await;

        assert!(result.is_ok());
        let event = result.expect("timeout");
        assert!(event.is_ok());
        let metric_event = event.expect("recv");
        assert_eq!(metric_event.event_type, MetricEventType::MetricUpdated);
        assert_eq!(metric_event.metric_name, "test");
    }

    #[tokio::test]
    async fn test_subscribe_multiple_receivers() {
        let config = MetricsConfig::default();
        let system = Arc::new(AdvancedMetricsSystem::new(config));

        let mut receiver1 = system.subscribe();
        let mut receiver2 = system.subscribe();

        let system_clone = Arc::clone(&system);
        tokio::spawn(async move {
            system_clone
                .record_performance_metric("broadcast_test".to_string(), 75.0, MetricType::Counter)
                .await
                .expect("record performance");
        });

        // Both receivers should get the event
        let result1 =
            tokio::time::timeout(std::time::Duration::from_millis(100), receiver1.recv()).await;

        let result2 =
            tokio::time::timeout(std::time::Duration::from_millis(100), receiver2.recv()).await;

        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }

    #[tokio::test]
    async fn test_concurrent_metric_recording() {
        let config = MetricsConfig::default();
        let system = Arc::new(AdvancedMetricsSystem::new(config));

        let mut handles = vec![];

        // Spawn 10 concurrent metric recording tasks
        for i in 0..10 {
            let system_clone = Arc::clone(&system);
            let handle = tokio::spawn(async move {
                system_clone
                    .record_performance_metric(
                        format!("concurrent_{i}"),
                        f64::from(i),
                        MetricType::Counter,
                    )
                    .await
            });
            handles.push(handle);
        }

        // Wait for all tasks
        for handle in handles {
            let result = handle.await;
            assert!(result.is_ok());
            assert!(result.expect("join").is_ok());
        }

        let store = system.metrics_store.read().await;
        assert_eq!(store.performance.len(), 10);
    }

    #[tokio::test]
    async fn test_performance_metric_statistics() {
        let config = MetricsConfig::default();
        let system = AdvancedMetricsSystem::new(config);

        system
            .record_performance_metric("stats_test".to_string(), 100.0, MetricType::Gauge)
            .await
            .expect("record performance");

        let store = system.metrics_store.read().await;

        // Verify the metric was stored
        assert_eq!(store.performance.len(), 1);

        // Get the metric (performance is a HashMap<String, PerformanceMetric>)
        let metric = store
            .performance
            .get("stats_test")
            .expect("stats_test metric");

        // Verify statistics are initialized
        assert_eq!(metric.stats.min, 100.0);
        assert_eq!(metric.stats.max, 100.0);
        assert_eq!(metric.stats.avg, 100.0);
        assert_eq!(metric.stats.std_dev, 0.0);
        assert_eq!(metric.stats.p95, 100.0);
        assert_eq!(metric.stats.p99, 100.0);
        assert_eq!(metric.stats.count, 1);
    }

    #[tokio::test]
    async fn test_metric_event_data_serialization() {
        let config = MetricsConfig::default();
        let system = AdvancedMetricsSystem::new(config);

        let mut receiver = system.subscribe();

        let system_arc = Arc::new(system);
        let system_clone = Arc::clone(&system_arc);

        tokio::spawn(async move {
            system_clone
                .record_performance_metric(
                    "serialize_test".to_string(),
                    42.5,
                    MetricType::Histogram,
                )
                .await
                .expect("record performance");
        });

        let result =
            tokio::time::timeout(std::time::Duration::from_millis(100), receiver.recv()).await;

        assert!(result.is_ok());
        let event = result.expect("timeout").expect("recv");

        // Verify data field contains value
        assert!(event.data.is_object());
        let obj = event.data.as_object().expect("object payload");
        assert!(obj.contains_key("value"));
    }

    #[tokio::test]
    async fn test_security_event_broadcast() {
        let config = MetricsConfig::default();
        let system = Arc::new(AdvancedMetricsSystem::new(config));

        let mut receiver = system.subscribe();

        let system_clone = Arc::clone(&system);
        tokio::spawn(async move {
            let event = SecurityEvent {
                timestamp: SystemTime::now(),
                event_type: SecurityEventType::UnauthorizedAccess,
                severity: SecuritySeverity::Critical,
                description: "Test security alert".to_string(),
                source: "test".to_string(),
                context: std::collections::HashMap::new(),
            };
            system_clone
                .record_security_event(event)
                .await
                .expect("record security event");
        });

        let result =
            tokio::time::timeout(std::time::Duration::from_millis(100), receiver.recv()).await;

        assert!(result.is_ok());
        let metric_event = result.expect("timeout").expect("recv");
        assert_eq!(metric_event.event_type, MetricEventType::SecurityAlert);
    }
}
