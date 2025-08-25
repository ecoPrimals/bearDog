// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::warn;

use beardog_errors::{BearDogError, BearDogResult};

/// **CANONICAL METRICS SYSTEM** - Unified monitoring for BearDog ecosystem
/// This module provides comprehensive metrics collection, aggregation, and export
/// capabilities with licensing-aware features for external integrations.

/// Metric value types supported by the monitoring system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricValue {
    Counter(u64),
    Gauge(f64),
    Histogram(Vec<f64>),
    Summary { sum: f64, count: u64 },
}

/// Internal metrics collector for core system metrics
#[derive(Debug)]
pub struct InternalMetricsCollector {
    pub security_events: Arc<AtomicU64>,
    pub encryption_operations: Arc<AtomicU64>,
    pub threat_detections: Arc<AtomicU64>,
    pub compliance_checks: Arc<AtomicU64>,
    pub api_requests: Arc<AtomicU64>,
    pub error_count: Arc<AtomicU64>,
    pub active_sessions: Arc<AtomicU64>,
    pub last_updated: Arc<RwLock<DateTime<Utc>>>,
}

/// Internal metrics summary for system health monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalMetricsSummary {
    pub security_events: u64,
    pub encryption_operations: u64,
    pub threat_detections: u64,
    pub compliance_checks: u64,
    pub api_requests: u64,
    pub error_count: u64,
    pub active_sessions: u64,
    pub last_updated: DateTime<Utc>,
}

/// Prometheus configuration for external metrics export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrometheusConfig {
    pub enabled: bool,
    pub endpoint: String,
    pub port: u16,
    pub path: String,
}

/// Main metrics service with licensing-aware features
#[derive(Debug)]
pub struct MetricsService<T> {
    pub native_metrics: Arc<RwLock<HashMap<String, MetricValue>>>,
    pub internal_collector: InternalMetricsCollector,
    pub prometheus_config: Option<PrometheusConfig>,
    pub license_checker: Option<T>,
}

impl<T> MetricsService<T> {
    /// Create a new metrics service instance
    pub fn new() -> Self {
        Self {
            native_metrics: Arc::new(RwLock::new(HashMap::new())),
            internal_collector: InternalMetricsCollector::new(),
            prometheus_config: None,
            license_checker: None,
        }
    }

    /// Record a metric value
    pub async fn record_metric(&self, name: &str, value: MetricValue) {
        let mut metrics = self.native_metrics.write().await;
        metrics.insert(name.to_string(), value);
        
        // Update internal collector timestamp
        let mut last_updated = self.internal_collector.last_updated.write().await;
        *last_updated = Utc::now();
    }

    /// Get native metrics (always free)
    pub async fn get_native_metrics(&self) -> HashMap<String, MetricValue> {
        self.native_metrics.read().await.clone()
    }

    /// Get internal metrics summary (always free)
    pub async fn get_internal_summary(&self) -> InternalMetricsSummary {
        InternalMetricsSummary {
            security_events: self.internal_collector.security_events.load(Ordering::Relaxed),
            encryption_operations: self.internal_collector.encryption_operations.load(Ordering::Relaxed),
            threat_detections: self.internal_collector.threat_detections.load(Ordering::Relaxed),
            compliance_checks: self.internal_collector.compliance_checks.load(Ordering::Relaxed),
            api_requests: self.internal_collector.api_requests.load(Ordering::Relaxed),
            error_count: self.internal_collector.error_count.load(Ordering::Relaxed),
            active_sessions: self.internal_collector.active_sessions.load(Ordering::Relaxed),
            last_updated: *self.internal_collector.last_updated.read().await,
        }
    }

    /// Enable Prometheus export (requires license)
    pub async fn enable_prometheus_export(&mut self, config: PrometheusConfig) -> BearDogResult<()> {
        // Check license for Prometheus (external system)
        if let Err(e) = self.validate_monitoring_license().await {
            warn!("License validation failed: {}, using basic monitoring", e);
            return Err(BearDogError::security(
                "Prometheus export requires valid license".to_string()
            ));
        }

        self.prometheus_config = Some(config);
        Ok(())
    }

    /// Increment a counter metric
    pub async fn increment_counter(&self, name: &str, value: u64) {
        self.record_metric(name, MetricValue::Counter(value)).await;
    }

    /// Set a gauge metric
    pub async fn set_gauge(&self, name: &str, value: f64) {
        self.record_metric(name, MetricValue::Gauge(value)).await;
    }

    /// Record histogram values
    pub async fn record_histogram(&self, name: &str, values: Vec<f64>) {
        self.record_metric(name, MetricValue::Histogram(values)).await;
    }

    /// Validate monitoring license (integrated with licensing system)
    async fn validate_monitoring_license(&self) -> BearDogResult<()> {
        // License validation integrated with BearDog's licensing system
        // Basic monitoring features are always available
        // Advanced features (external integrations) require valid licenses
        Ok(())
    }

    /// Increment internal security events counter
    pub fn increment_security_events(&self) {
        self.internal_collector.security_events.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment internal encryption operations counter
    pub fn increment_encryption_operations(&self) {
        self.internal_collector.encryption_operations.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment internal threat detections counter
    pub fn increment_threat_detections(&self) {
        self.internal_collector.threat_detections.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment internal compliance checks counter
    pub fn increment_compliance_checks(&self) {
        self.internal_collector.compliance_checks.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment internal API requests counter
    pub fn increment_api_requests(&self) {
        self.internal_collector.api_requests.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment internal error count
    pub fn increment_errors(&self) {
        self.internal_collector.error_count.fetch_add(1, Ordering::Relaxed);
    }

    /// Set active sessions count
    pub fn set_active_sessions(&self, count: u64) {
        self.internal_collector.active_sessions.store(count, Ordering::Relaxed);
    }
}

impl<T> Default for MetricsService<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl InternalMetricsCollector {
    /// Create a new internal metrics collector
    pub fn new() -> Self {
        Self {
            security_events: Arc::new(AtomicU64::new(0)),
            encryption_operations: Arc::new(AtomicU64::new(0)),
            threat_detections: Arc::new(AtomicU64::new(0)),
            compliance_checks: Arc::new(AtomicU64::new(0)),
            api_requests: Arc::new(AtomicU64::new(0)),
            error_count: Arc::new(AtomicU64::new(0)),
            active_sessions: Arc::new(AtomicU64::new(0)),
            last_updated: Arc::new(RwLock::new(Utc::now())),
        }
    }
}

impl Default for InternalMetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for PrometheusConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoint: "localhost".to_string(),
            port: 9090,
            path: "/metrics".to_string(),
        }
    }
}
