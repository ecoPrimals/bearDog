//! Telemetry and monitoring for the BearDog Sovereign Science Framework
//!
//! Provides comprehensive metrics collection, monitoring, and observability
//! for the validation system. Integrates with Prometheus for metrics export
//! and structured logging for detailed observability.

use crate::errors::SovereignScienceError;
use crate::ValidationResults;
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use tracing::{info, warn, error, debug};

/// Telemetry collection framework for validation metrics
#[derive(Debug, Clone)]
pub struct TelemetryFramework {
    /// Unique experiment identifier
    pub experiment_id: String,
    /// Prometheus exporter address
    pub prometheus_addr: Option<SocketAddr>,
    /// Start time of the validation run
    pub start_time: Instant,
}

impl TelemetryFramework {
    /// Initialize the telemetry framework with Prometheus exporter
    ///
    /// # Arguments
    /// * `prometheus_addr` - Optional address to bind Prometheus exporter (e.g., "127.0.0.1:9090")
    ///
    /// # Returns
    /// Initialized telemetry framework
    pub async fn initialize(
        prometheus_addr: Option<SocketAddr>,
    ) -> Result<Self, SovereignScienceError> {
        let experiment_id = format!(
            "BEARDOG-VALIDATION-{}",
            chrono::Utc::now().format("%Y%m%d-%H%M%S")
        );

        // Setup Prometheus exporter if address provided
        if let Some(addr) = prometheus_addr {
            use metrics_exporter_prometheus::PrometheusBuilder;
            
            PrometheusBuilder::new()
                .with_http_listener(addr)
                .install()
                .map_err(|e| {
                    SovereignScienceError::TelemetryError(format!(
                        "Failed to initialize Prometheus exporter: {}",
                        e
                    ))
                })?;

            info!("📊 Prometheus metrics available at http://{}/metrics", addr);
        }

        // Initialize tracing subscriber if not already set
        let _ = tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
            )
            .with_target(false)
            .with_thread_ids(true)
            .with_line_number(true)
            .try_init();

        info!("🔭 Telemetry framework initialized: {}", experiment_id);

        Ok(Self {
            experiment_id,
            prometheus_addr,
            start_time: Instant::now(),
        })
    }

    /// Record a cryptographic operation
    ///
    /// # Arguments
    /// * `op_type` - Type of operation (e.g., "sign", "verify", "encrypt")
    /// * `duration` - Duration of the operation
    /// * `success` - Whether the operation succeeded
    pub fn record_crypto_operation(&self, op_type: &str, duration: Duration, success: bool) {
        if !success {
            warn!("Cryptographic operation failed: {}", op_type);
        }

        debug!(
            op_type = %op_type,
            duration_us = duration.as_micros(),
            success = success,
            "Recorded crypto operation"
        );
    }

    /// Record a performance measurement
    ///
    /// # Arguments
    /// * `metric_name` - Name of the performance metric
    /// * `value` - Measured value
    /// * `unit` - Unit of measurement
    pub fn record_performance(&self, metric_name: &str, value: f64, unit: &str) {
        debug!(
            metric = %metric_name,
            value = value,
            unit = %unit,
            "Recorded performance metric"
        );
    }

    /// Record a security event
    ///
    /// # Arguments
    /// * `event_type` - Type of security event
    /// * `severity` - Severity level (info, warning, critical)
    /// * `details` - Event details
    pub fn record_security_event(&self, event_type: &str, severity: &str, details: &str) {
        match severity {
            "critical" => {
                error!(
                    event_type = %event_type,
                    severity = %severity,
                    details = %details,
                    "CRITICAL SECURITY EVENT"
                );
            }
            "warning" => {
                warn!(
                    event_type = %event_type,
                    severity = %severity,
                    details = %details,
                    "Security warning"
                );
            }
            _ => {
                info!(
                    event_type = %event_type,
                    severity = %severity,
                    details = %details,
                    "Security event"
                );
            }
        }
    }

    /// Record a human dignity check result
    ///
    /// # Arguments
    /// * `check_name` - Name of the dignity check
    /// * `passed` - Whether the check passed
    /// * `score` - Optional score (0.0 to 1.0)
    pub fn record_dignity_check(&self, check_name: &str, passed: bool, score: Option<f64>) {
        if !passed {
            warn!("Human dignity check failed: {}", check_name);
        }

        info!(
            check = %check_name,
            passed = passed,
            score = ?score,
            "Recorded dignity check"
        );
    }

    /// Record validation stage completion
    ///
    /// # Arguments
    /// * `stage_name` - Name of the validation stage
    /// * `success` - Whether the stage succeeded
    /// * `duration` - Duration of the stage
    pub fn record_stage_completion(
        &self,
        stage_name: &str,
        success: bool,
        duration: Duration,
    ) {
        if success {
            info!(
                stage = %stage_name,
                duration_secs = duration.as_secs(),
                "✅ Validation stage completed successfully"
            );
        } else {
            error!(
                stage = %stage_name,
                duration_secs = duration.as_secs(),
                "❌ Validation stage failed"
            );
        }
    }

    /// Record enterprise readiness metric
    ///
    /// # Arguments
    /// * `metric_name` - Name of the readiness metric
    /// * `value` - Metric value
    pub fn record_enterprise_metric(&self, metric_name: &str, value: f64) {
        debug!(
            metric = %metric_name,
            value = value,
            "Recorded enterprise metric"
        );
    }

    /// Record test execution
    ///
    /// # Arguments
    /// * `test_type` - Type of test (unit, integration, e2e, chaos)
    /// * `test_name` - Name of the test
    /// * `passed` - Whether the test passed
    /// * `duration` - Test duration
    pub fn record_test_execution(
        &self,
        test_type: &str,
        test_name: &str,
        passed: bool,
        duration: Duration,
    ) {
        if !passed {
            warn!(
                test_type = %test_type,
                test_name = %test_name,
                duration_ms = duration.as_millis(),
                "Test failed"
            );
        } else {
            debug!(
                test_type = %test_type,
                test_name = %test_name,
                duration_ms = duration.as_millis(),
                "Test passed"
            );
        }
    }

    /// Record final validation results
    ///
    /// # Arguments
    /// * `results` - Validation results to record
    pub async fn record_final_results(&self, results: &ValidationResults) {
        info!("📊 Recording final validation results for {}", results.experiment_id);

        // Log final summary
        let total_duration = self.start_time.elapsed();
        
        info!(
            experiment_id = %results.experiment_id,
            success = results.all_stages_passed,
            duration_secs = total_duration.as_secs(),
            p_value = results.statistical_significance,
            effect_size = results.effect_size,
            confidence_lower = results.confidence_interval.0,
            confidence_upper = results.confidence_interval.1,
            "✅ Final validation results recorded"
        );
    }

    /// Get a snapshot of current metrics (for testing/debugging)
    pub fn get_metrics_summary(&self) -> MetricsSummary {
        let elapsed = self.start_time.elapsed();

        MetricsSummary {
            experiment_id: self.experiment_id.clone(),
            elapsed_seconds: elapsed.as_secs(),
            prometheus_addr: self.prometheus_addr,
        }
    }

    /// Shutdown the telemetry system gracefully
    pub async fn shutdown(&self) {
        let total_duration = self.start_time.elapsed();
        
        info!(
            "🛑 Shutting down telemetry framework: {} (duration: {} seconds)",
            self.experiment_id,
            total_duration.as_secs()
        );

        info!("✅ Telemetry shutdown complete");
    }
}

/// Summary of current telemetry metrics
#[derive(Debug, Clone)]
pub struct MetricsSummary {
    /// Experiment identifier
    pub experiment_id: String,
    /// Elapsed time in seconds
    pub elapsed_seconds: u64,
    /// Prometheus exporter address if configured
    pub prometheus_addr: Option<SocketAddr>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_telemetry_initialization() {
        let telemetry = TelemetryFramework::initialize(None).await.unwrap();

        assert!(telemetry.experiment_id.starts_with("BEARDOG-VALIDATION-"));
        assert!(telemetry.prometheus_addr.is_none());
    }

    #[tokio::test]
    async fn test_crypto_operation_recording() {
        let telemetry = TelemetryFramework::initialize(None).await.unwrap();

        telemetry.record_crypto_operation("test_sign", Duration::from_micros(100), true);
        telemetry.record_crypto_operation("test_verify", Duration::from_micros(50), true);

        // No panics = success
    }

    #[tokio::test]
    async fn test_performance_recording() {
        let telemetry = TelemetryFramework::initialize(None).await.unwrap();

        telemetry.record_performance("test_throughput", 1000.0, "ops/sec");
        telemetry.record_performance("test_latency", 0.5, "ms");

        // No panics = success
    }

    #[tokio::test]
    async fn test_security_event_recording() {
        let telemetry = TelemetryFramework::initialize(None).await.unwrap();

        telemetry.record_security_event("test_event", "info", "Test security event");
        telemetry.record_security_event("test_warning", "warning", "Test warning");

        // No panics = success
    }

    #[tokio::test]
    async fn test_dignity_check_recording() {
        let telemetry = TelemetryFramework::initialize(None).await.unwrap();

        telemetry.record_dignity_check("test_check", true, Some(0.95));
        telemetry.record_dignity_check("test_check_fail", false, Some(0.3));

        // No panics = success
    }

    #[tokio::test]
    async fn test_stage_completion() {
        let telemetry = TelemetryFramework::initialize(None).await.unwrap();

        telemetry.record_stage_completion("test_stage", true, Duration::from_secs(10));

        // No panics = success
    }

    #[tokio::test]
    async fn test_metrics_summary() {
        let telemetry = TelemetryFramework::initialize(None).await.unwrap();

        let summary = telemetry.get_metrics_summary();

        assert!(summary.experiment_id.starts_with("BEARDOG-VALIDATION-"));
        assert!(summary.elapsed_seconds < 5); // Should be very recent
    }

    #[tokio::test]
    async fn test_shutdown() {
        let telemetry = TelemetryFramework::initialize(None).await.unwrap();

        telemetry.shutdown().await;

        // No panics = success
    }
}
