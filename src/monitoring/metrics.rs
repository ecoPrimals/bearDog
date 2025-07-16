use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::error::{BearDogError, BearDogResult};
use crate::licensing::LicenseManager;
use super::types::{InternalMetricsSummary, MetricValue, PrometheusConfig};

/// Metrics collection and export system
///
/// **LICENSING CLARITY**:
/// - Native Rust metrics collection: FREE (AGPL)
/// - Prometheus export: REQUIRES LICENSE (external system)
/// - Internal monitoring: FREE (AGPL)
pub struct MetricsService {
    /// Native Rust metrics (always free)
    native_metrics: Arc<RwLock<HashMap<String, MetricValue>>>,
    /// License manager for external integrations
    license_manager: Arc<LicenseManager>,
    /// Prometheus exporter (if licensed)
    prometheus_exporter: Option<PrometheusExporter>,
    /// Internal metrics collection (always enabled)
    internal_collector: InternalMetricsCollector,
}

/// Native Rust metrics collection (100% free under AGPL)
#[derive(Debug, Clone)]
pub struct InternalMetricsCollector {
    /// Counter for security-related events
    pub security_events: Arc<AtomicU64>,
    /// Counter for encryption operations performed
    pub encryption_operations: Arc<AtomicU64>,
    /// Counter for threat detections
    pub threat_detections: Arc<AtomicU64>,
    /// Counter for compliance checks performed
    pub compliance_checks: Arc<AtomicU64>,
    /// Counter for API requests processed
    pub api_requests: Arc<AtomicU64>,
    /// Counter for errors encountered
    pub error_count: Arc<AtomicU64>,
    /// Counter for currently active sessions
    pub active_sessions: Arc<AtomicU64>,
    /// Timestamp of when metrics were last updated
    pub last_updated: Arc<RwLock<DateTime<Utc>>>,
}

/// Prometheus exporter (requires license for external Prometheus systems)
pub struct PrometheusExporter {
    /// Whether Prometheus export is enabled
    enabled: bool,
    /// Endpoint URL for Prometheus metrics export
    endpoint: String,
    /// Port number for Prometheus metrics server
    port: u16,
}

impl MetricsService {
    /// Create new metrics service
    pub fn new(license_manager: Arc<LicenseManager>) -> Self {
        Self {
            native_metrics: Arc::new(RwLock::new(HashMap::new())),
            license_manager,
            prometheus_exporter: None,
            internal_collector: InternalMetricsCollector::new(),
        }
    }

    /// Record metric (always free - native Rust)
    pub async fn record_metric(&self, name: &str, value: MetricValue) {
        let mut metrics = self.native_metrics.write().await;
        metrics.insert(name.to_string(), value);

        // Update internal collector timestamp
        *self.internal_collector.last_updated.write().await = Utc::now();
    }

    /// Get native metrics (always free)
    pub async fn get_native_metrics(&self) -> HashMap<String, MetricValue> {
        self.native_metrics.read().await.clone()
    }

    /// Get internal metrics summary (always free)
    pub async fn get_internal_summary(&self) -> InternalMetricsSummary {
        InternalMetricsSummary {
            security_events: self
                .internal_collector
                .security_events
                .load(Ordering::Relaxed),
            encryption_operations: self
                .internal_collector
                .encryption_operations
                .load(Ordering::Relaxed),
            threat_detections: self
                .internal_collector
                .threat_detections
                .load(Ordering::Relaxed),
            compliance_checks: self
                .internal_collector
                .compliance_checks
                .load(Ordering::Relaxed),
            api_requests: self.internal_collector.api_requests.load(Ordering::Relaxed),
            error_count: self.internal_collector.error_count.load(Ordering::Relaxed),
            active_sessions: self
                .internal_collector
                .active_sessions
                .load(Ordering::Relaxed),
            last_updated: *self.internal_collector.last_updated.read().await,
        }
    }

    /// Enable Prometheus export (requires license)
    pub async fn enable_prometheus_export(
        &mut self,
        config: PrometheusConfig,
    ) -> BearDogResult<()> {
        // Check license for Prometheus (external system)
        if !self
            .license_manager
            .verify_external_function_access("prometheus")?
        {
            return Err(BearDogError::Configuration {
                message: "Prometheus export requires a BearDog license. Native Rust metrics are always free. Contact sales for enterprise Prometheus integration.".to_string()
            });
        }

        self.prometheus_exporter = Some(PrometheusExporter {
            enabled: true,
            endpoint: config.endpoint,
            port: config.port,
        });

        tracing::info!("✅ Prometheus export enabled (licensed feature)");
        Ok(())
    }

    /// Export metrics to Prometheus (licensed feature)
    pub async fn export_to_prometheus(&self) -> BearDogResult<String> {
        if let Some(exporter) = &self.prometheus_exporter {
            if !exporter.enabled {
                return Err(BearDogError::Configuration {
                    message: "Prometheus export is not enabled".to_string(),
                });
            }

            // Generate Prometheus format
            let summary = self.get_internal_summary().await;
            let prometheus_output = format!(
                "# HELP beardog_security_events_total Total security events processed\n\
                 # TYPE beardog_security_events_total counter\n\
                 beardog_security_events_total {}\n\
                 # HELP beardog_encryption_operations_total Total encryption operations\n\
                 # TYPE beardog_encryption_operations_total counter\n\
                 beardog_encryption_operations_total {}\n\
                 # HELP beardog_threat_detections_total Total threats detected\n\
                 # TYPE beardog_threat_detections_total counter\n\
                 beardog_threat_detections_total {}\n\
                 # HELP beardog_compliance_checks_total Total compliance checks\n\
                 # TYPE beardog_compliance_checks_total counter\n\
                 beardog_compliance_checks_total {}\n\
                 # HELP beardog_api_requests_total Total API requests\n\
                 # TYPE beardog_api_requests_total counter\n\
                 beardog_api_requests_total {}\n\
                 # HELP beardog_errors_total Total errors\n\
                 # TYPE beardog_errors_total counter\n\
                 beardog_errors_total {}\n\
                 # HELP beardog_active_sessions Current active sessions\n\
                 # TYPE beardog_active_sessions gauge\n\
                 beardog_active_sessions {}\n",
                summary.security_events,
                summary.encryption_operations,
                summary.threat_detections,
                summary.compliance_checks,
                summary.api_requests,
                summary.error_count,
                summary.active_sessions
            );

            Ok(prometheus_output)
        } else {
            Err(BearDogError::Configuration {
                message: "Prometheus export not configured. Use native metrics (free) or obtain a license for Prometheus integration.".to_string()
            })
        }
    }

    /// Increment security event counter (always free)
    pub fn increment_security_events(&self) {
        self.internal_collector
            .security_events
            .fetch_add(1, Ordering::Relaxed);
    }

    /// Increment encryption operations (always free)
    pub fn increment_encryption_operations(&self) {
        self.internal_collector
            .encryption_operations
            .fetch_add(1, Ordering::Relaxed);
    }

    /// Record threat detection (always free)
    pub fn record_threat_detection(&self) {
        self.internal_collector
            .threat_detections
            .fetch_add(1, Ordering::Relaxed);
    }

    /// Record compliance check (always free)
    pub fn record_compliance_check(&self) {
        self.internal_collector
            .compliance_checks
            .fetch_add(1, Ordering::Relaxed);
    }

    /// Record API request (always free)
    pub fn record_api_request(&self) {
        self.internal_collector
            .api_requests
            .fetch_add(1, Ordering::Relaxed);
    }

    /// Record error (always free)
    pub fn record_error(&self) {
        self.internal_collector
            .error_count
            .fetch_add(1, Ordering::Relaxed);
    }

    /// Update active sessions (always free)
    pub fn update_active_sessions(&self, count: u64) {
        self.internal_collector
            .active_sessions
            .store(count, Ordering::Relaxed);
    }
}

impl Default for InternalMetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl InternalMetricsCollector {
    /// Create a new internal metrics collector with default values
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