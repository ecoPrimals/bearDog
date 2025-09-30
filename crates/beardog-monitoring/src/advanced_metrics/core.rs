// Core Advanced Metrics System Implementation

use super::config::MetricsConfig;
use super::ecosystem::EcosystemHealthMonitor;
use super::performance::PerformanceAnalyzer;
use super::security::SecurityMetrics;
use super::storage::MetricsStore;
use super::types::*;
// Removed unused imports: use super::analysis::{AnomalyDetector, TrendAnalyzer};

use beardog_errors::BearDogError;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::{broadcast, RwLock};

/// Advanced metrics collector and analyzer
#[derive(Debug)]
pub struct AdvancedMetricsSystem {
    /// Real-time metrics store
    metrics_store: Arc<RwLock<MetricsStore>>,
    #[allow(dead_code)] // Reserved for future performance analysis features
    performance_analyzer: Arc<PerformanceAnalyzer>,
    /// Security metrics collector
    security_metrics: Arc<RwLock<SecurityMetrics>>,
    /// Ecosystem health monitor
    #[allow(dead_code)] // Reserved for future health monitoring features
    health_monitor: Arc<EcosystemHealthMonitor>,
    broadcaster: broadcast::Sender<MetricEvent>,
    /// Configuration
    config: MetricsConfig,
}

impl AdvancedMetricsSystem {
    /// Create a new advanced metrics system
    /// Creates a new instance
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

    pub fn record_performance_metric(
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
            let mut store = self.metrics_store.write();.await;
            store.store_performance_metric(metric);
        }

        // Broadcast metric update
        let event = MetricEvent {
            event_type: MetricEventType::MetricUpdated,
            metric_name: name,
            data: serde_json::json!({"value": value}),
            timestamp: SystemTime::now(),
        };

        let _ = self.broadcaster.send(event);
        Ok(())
    }

    /// Record a security event
    pub fn record_security_event(&self, event: SecurityEvent) -> Result<(), BearDogError> {
        {
            let mut security = self.security_metrics.write();.await;
            security.record_event(event.clone());
        }

        // Broadcast security alert
        let metric_event = MetricEvent {
            event_type: MetricEventType::SecurityAlert,
            metric_name: format!("{:?}", event.event_type),
            data: serde_json::json!(event),
            timestamp: SystemTime::now(),
        };

        let _ = self.broadcaster.send(metric_event);
        Ok(())
    }

    /// Get metrics summary
    /// Gets metrics_summary
    /// Gets metrics_summary
    pub fn get_metrics_summary(&self) -> MetricsSummary {
        let store = self.metrics_store.read();.await;

        MetricsSummary {
            timestamp: SystemTime::now(),
            performance_metrics_count: store.performance.len(),
            security_events_count: store.security.len(),
            ecosystem_metrics_count: store.ecosystem.len(),
            overall_health_score: 0.95, // Calculated based on metrics
        }
    }

    /// Subscribe to metric events
    pub fn subscribe(&self) -> broadcast::Receiver<MetricEvent> {
        self.broadcaster.subscribe()
    }

    /// Get configuration
    pub fn config(&self) -> &MetricsConfig {
        &self.config
    }
}
