// SPDX-License-Identifier: AGPL-3.0-or-later

// Modular Advanced Metrics System
//
// This module provides a comprehensive, modular metrics collection and analysis system
// that replaces the large monolithic advanced_metrics.rs file with focused, maintainable components.
//
// ## Architecture
//
// The metrics system is organized into logical domains:
// - **Core**: Main metrics system and storage
// - **Performance**: Performance analysis and monitoring
// - **Security**: Security event tracking and analysis
// - **Ecosystem**: Cross-system interaction metrics
// - **Analytics**: Statistical analysis and reporting
// - **Export**: Metrics export and integration

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;

// Domain-specific metrics modules
/// Trend detection and summarized reporting over recorded events.
pub mod analytics;
/// Minimal metrics core (intervals, enablement).
pub mod core;
/// Cross-primal interaction and integration health metrics.
pub mod ecosystem;
/// Optional export to Prometheus/Grafana and similar systems.
pub mod export;
/// CPU, latency, throughput, and related performance signals.
pub mod performance;
/// Authentication failures, threat level, and compliance-style scores.
pub mod security;

pub use core::{MetricsCore, MetricsCoreConfig};

// Re-export key types
pub use analytics::{AnalyticsConfig, AnalyticsEngine, AnalyticsSummary};
pub use ecosystem::{EcosystemConfig, EcosystemMetrics, EcosystemMonitor};
pub use export::{ExportConfig, ExportEngine};
pub use performance::{PerformanceConfig, PerformanceEngine, PerformanceMetrics};
pub use security::{SecurityMetrics, SecurityMetricsConfig, SecurityMetricsEngine};

/// **UNIFIED ADVANCED METRICS SYSTEM**
///
/// all metrics collection, analysis, and reporting functionality.
#[derive(Debug)]
pub struct UnifiedMetricsSystem {
    /// Core metrics storage and management
    core: Arc<MetricsCore>,

    performance: Arc<PerformanceEngine>,

    /// Security metrics collector
    security: Arc<SecurityMetricsEngine>,

    /// Ecosystem interaction monitor
    ecosystem: Arc<EcosystemMonitor>,

    /// Analytics and reporting engine
    analytics: Arc<AnalyticsEngine>,

    /// Export and integration system
    export: Arc<ExportEngine>,

    /// Real-time event broadcaster
    broadcaster: broadcast::Sender<MetricEvent>,

    /// System configuration (retained for future unified tuning; subsystems clone from `new` input).
    _config: UnifiedMetricsConfig,
}

impl UnifiedMetricsSystem {
    /// Create a new unified metrics system
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when any subsystem (`MetricsCore`, performance, security, ecosystem,
    /// analytics, or export engine) fails to construct.
    pub fn new(config: UnifiedMetricsConfig) -> Result<Self, BearDogError> {
        let (broadcaster, _) = broadcast::channel(config.broadcast_buffer_size);

        let core = Arc::new(MetricsCore::new(config.core.clone())?);
        let performance = Arc::new(PerformanceEngine::new(config.performance.clone())?);
        let security = Arc::new(SecurityMetricsEngine::new(config.security.clone())?);
        let ecosystem = Arc::new(EcosystemMonitor::new(config.ecosystem.clone())?);
        let analytics = Arc::new(AnalyticsEngine::new(config.analytics.clone())?);
        let export = Arc::new(ExportEngine::new(config.export.clone())?);

        Ok(Self {
            core,
            performance,
            security,
            ecosystem,
            analytics,
            export,
            broadcaster,
            _config: config,
        })
    }

    /// Start the unified metrics system
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when any subsystem’s `start` fails.
    pub fn start(&self) -> Result<(), BearDogError> {
        // Start all subsystems
        self.core.start()?;
        self.performance.start()?;
        self.security.start()?;
        self.ecosystem.start()?;
        self.analytics.start()?;
        self.export.start()?;

        tracing::info!("Unified metrics system started successfully");
        Ok(())
    }

    /// Record a metric event
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when a subsystem’s `record_event` fails.
    pub fn record_event(&self, event: MetricEvent) -> Result<(), BearDogError> {
        // Route event to appropriate subsystem
        match &event.category {
            MetricCategory::Performance => self.performance.record_event(&event)?,
            MetricCategory::Security => self.security.record_event(&event)?,
            MetricCategory::Ecosystem => self.ecosystem.record_event(&event)?,
            MetricCategory::Custom => {
                // Custom metrics are handled by the core system
                tracing::debug!("Recording custom metric: {}", event.name);
            }
        }

        // Broadcast event for real-time monitoring
        if self.broadcaster.send(event).is_err() {
            tracing::warn!("Failed to broadcast metric event - no active subscribers");
        }

        Ok(())
    }

    /// Get comprehensive system metrics
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when performance, security, ecosystem, or analytics summaries fail.
    pub fn get_system_metrics(&self) -> Result<SystemMetrics, BearDogError> {
        let performance_metrics = self.performance.get_metrics()?;
        let security_metrics = self.security.get_metrics()?;
        let ecosystem_metrics = self.ecosystem.get_metrics()?;
        let analytics_summary = self.analytics.get_summary()?;

        Ok(SystemMetrics {
            performance: performance_metrics,
            security: security_metrics,
            ecosystem: ecosystem_metrics,
            analytics: analytics_summary,
            timestamp: std::time::SystemTime::now(),
        })
    }
}

/// Unified metrics configuration
#[derive(Debug, Clone)]
pub struct UnifiedMetricsConfig {
    /// Core system configuration
    /// The core value
    pub core: MetricsCoreConfig,

    /// Sampling and retention settings for the performance engine.
    pub performance: PerformanceConfig,

    /// Security metrics configuration
    /// The security value
    pub security: SecurityMetricsConfig,

    /// Ecosystem monitoring configuration
    /// The ecosystem value
    pub ecosystem: EcosystemConfig,

    /// Analytics configuration
    /// The analytics value
    pub analytics: AnalyticsConfig,

    /// Export configuration
    /// The export value
    pub export: ExportConfig,

    /// Broadcast buffer size
    /// Number of `broadcast_buffer_size`
    pub broadcast_buffer_size: usize,
}

impl Default for UnifiedMetricsConfig {
    fn default() -> Self {
        Self {
            core: MetricsCoreConfig::default(),
            performance: PerformanceConfig::default(),
            security: SecurityMetricsConfig::default(),
            ecosystem: EcosystemConfig::default(),
            analytics: AnalyticsConfig::default(),
            export: ExportConfig::default(),
            broadcast_buffer_size: 1000,
        }
    }
}

/// Typed metric observation routed through the unified system and broadcast channel.
#[derive(Debug, Clone)]
pub struct MetricEvent {
    /// Event category
    /// The category value
    pub category: MetricCategory,

    /// Event name
    /// Name of the item
    pub name: String,

    /// Event value
    /// The value value
    pub value: MetricValue,

    /// Event labels/tags
    /// The labels value
    pub labels: std::collections::HashMap<String, String>,

    /// Event timestamp
    pub timestamp: std::time::SystemTime,
}

/// Routes a [`MetricEvent`] to the performance, security, ecosystem, or custom pipeline.
#[derive(Debug, Clone)]
pub enum MetricCategory {
    /// Latency, throughput, resource usage, etc.
    Performance,
    /// Represents security variant
    Security,
    /// Represents ecosystem variant
    Ecosystem,
    /// Represents custom variant
    Custom,
}

/// Metric value types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricValue {
    /// Represents counter variant
    Counter(u64),
    /// Represents gauge variant
    Gauge(f64),
    /// Represents histogram variant
    Histogram(Vec<f64>),
    /// Pre-aggregated sum and observation count (Prometheus-style summary).
    Summary {
        /// Sum of observed values.
        sum: f64,
        /// Number of observations included in `sum`.
        count: u64,
    },
}

/// Comprehensive system metrics
#[derive(Debug, Clone)]
pub struct SystemMetrics {
    /// Engine-reported performance metrics (CPU, latency, errors, etc.).
    pub performance: PerformanceMetrics,
    /// The security value
    pub security: SecurityMetrics,
    /// The ecosystem value
    pub ecosystem: EcosystemMetrics,
    /// The analytics value
    pub analytics: AnalyticsSummary,
    /// Wall-clock time when this aggregate snapshot was assembled.
    pub timestamp: std::time::SystemTime,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // === UnifiedMetricsConfig Tests ===

    #[test]
    fn test_unified_metrics_config_default() {
        let config = UnifiedMetricsConfig::default();
        assert_eq!(config.broadcast_buffer_size, 1000);
    }

    #[test]
    fn test_unified_metrics_config_clone() {
        let config = UnifiedMetricsConfig::default();
        let cloned = config.clone();
        assert_eq!(config.broadcast_buffer_size, cloned.broadcast_buffer_size);
    }

    // === MetricCategory Tests ===

    #[test]
    fn test_metric_category_variants() {
        let perf = MetricCategory::Performance;
        let sec = MetricCategory::Security;
        let eco = MetricCategory::Ecosystem;
        let custom = MetricCategory::Custom;

        assert!(matches!(perf, MetricCategory::Performance));
        assert!(matches!(sec, MetricCategory::Security));
        assert!(matches!(eco, MetricCategory::Ecosystem));
        assert!(matches!(custom, MetricCategory::Custom));
    }

    #[test]
    fn test_metric_category_clone() {
        let category = MetricCategory::Performance;
        let cloned = category;
        assert!(matches!(cloned, MetricCategory::Performance));
    }

    // === MetricValue Tests ===

    #[test]
    fn test_metric_value_counter() {
        let value = MetricValue::Counter(100);
        if let MetricValue::Counter(v) = value {
            assert_eq!(v, 100);
        } else {
            panic!("Expected Counter");
        }
    }

    #[test]
    fn test_metric_value_gauge() {
        let value = MetricValue::Gauge(50.5);
        if let MetricValue::Gauge(v) = value {
            assert!(
                (v - 50.5).abs() < f64::EPSILON,
                "expected gauge ≈ 50.5, got {v}"
            );
        } else {
            panic!("Expected Gauge");
        }
    }

    #[test]
    fn test_metric_value_histogram() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let value = MetricValue::Histogram(values);
        if let MetricValue::Histogram(v) = value {
            assert_eq!(v.len(), 5);
        } else {
            panic!("Expected Histogram");
        }
    }

    #[test]
    fn test_metric_value_summary() {
        let value = MetricValue::Summary {
            sum: 100.0,
            count: 10,
        };
        if let MetricValue::Summary { sum, count } = value {
            assert!(
                (sum - 100.0).abs() < f64::EPSILON,
                "expected summary sum ≈ 100.0, got {sum}"
            );
            assert_eq!(count, 10);
        } else {
            panic!("Expected Summary");
        }
    }

    #[test]
    fn test_metric_value_serialization() {
        let value = MetricValue::Counter(42);
        let serialized = serde_json::to_string(&value).expect("serialize");
        let deserialized: MetricValue = serde_json::from_str(&serialized).expect("deserialize");
        if let MetricValue::Counter(v) = deserialized {
            assert_eq!(v, 42);
        } else {
            panic!("Expected Counter");
        }
    }

    // === MetricEvent Tests ===

    #[test]
    fn test_metric_event_creation() {
        let event = MetricEvent {
            category: MetricCategory::Performance,
            name: "cpu_usage".to_string(),
            value: MetricValue::Gauge(75.5),
            labels: HashMap::new(),
            timestamp: std::time::SystemTime::now(),
        };
        assert_eq!(event.name, "cpu_usage");
        assert!(matches!(event.category, MetricCategory::Performance));
    }

    #[test]
    fn test_metric_event_with_labels() {
        let mut labels = HashMap::new();
        labels.insert("host".to_string(), "server-1".to_string());
        labels.insert("env".to_string(), "production".to_string());

        let event = MetricEvent {
            category: MetricCategory::Security,
            name: "auth_failures".to_string(),
            value: MetricValue::Counter(5),
            labels,
            timestamp: std::time::SystemTime::now(),
        };
        assert_eq!(event.labels.len(), 2);
        assert_eq!(event.labels.get("host"), Some(&"server-1".to_string()));
    }

    #[test]
    fn test_metric_event_clone() {
        let event = MetricEvent {
            category: MetricCategory::Ecosystem,
            name: "test_metric".to_string(),
            value: MetricValue::Counter(1),
            labels: HashMap::new(),
            timestamp: std::time::SystemTime::now(),
        };
        let cloned = event.clone();
        assert_eq!(event.name, cloned.name);
    }

    // === UnifiedMetricsSystem Tests ===

    #[test]
    fn test_unified_metrics_system_creation() {
        let config = UnifiedMetricsConfig::default();
        let result = UnifiedMetricsSystem::new(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unified_metrics_system_start() {
        let config = UnifiedMetricsConfig::default();
        let system = UnifiedMetricsSystem::new(config).expect("system");
        let result = system.start();
        assert!(result.is_ok());
    }

    #[test]
    fn test_unified_metrics_system_record_event() {
        let config = UnifiedMetricsConfig::default();
        let system = UnifiedMetricsSystem::new(config).expect("system");

        let event = MetricEvent {
            category: MetricCategory::Performance,
            name: "request_latency".to_string(),
            value: MetricValue::Gauge(125.0),
            labels: HashMap::new(),
            timestamp: std::time::SystemTime::now(),
        };

        let result = system.record_event(event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unified_metrics_system_record_custom_event() {
        let config = UnifiedMetricsConfig::default();
        let system = UnifiedMetricsSystem::new(config).expect("system");

        let event = MetricEvent {
            category: MetricCategory::Custom,
            name: "custom_metric".to_string(),
            value: MetricValue::Counter(1),
            labels: HashMap::new(),
            timestamp: std::time::SystemTime::now(),
        };

        let result = system.record_event(event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unified_metrics_system_get_metrics() {
        let config = UnifiedMetricsConfig::default();
        let system = UnifiedMetricsSystem::new(config).expect("system");
        system.start().expect("start");

        let result = system.get_system_metrics();
        assert!(result.is_ok());
    }
}
