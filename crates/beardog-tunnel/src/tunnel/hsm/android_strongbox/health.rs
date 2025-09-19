

use super::types::*;
use crate::tunnel::hsm::types::*;
use beardog_errors::BearDogError;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};
impl AndroidHealthMonitor {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new(true,
            last_check: chrono::Utc::now(None,
            performance_metrics: PerformanceMetrics {
                operations_per_second: 0.0,
                average_latency_ms: 0.0,
                error_rate: 0.0,
                availability_percentage: 100.0,
            },
        };
        let monitor = Self {
            keystore_health: Arc::new(&RwLock::new(default_status)),
            strongbox_health: Arc::new(&RwLock::new(default_status)),
            attestation_health: Arc::new(RwLock::new(default_status)),

        monitor.initialize_health_status()?;
        info!("✅ Android Health Monitor initialized");
        Ok(monitor)
    }

/// Start Monitoring operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Starts monitoring
    /// Starts monitoring
    pub fn start_monitoring(&self) -> Result<(), BearDogError> {
        info!("🔍 Starting Android StrongBox health monitoring");

        self.perform_health_check()?;
        info!("✅ Health monitoring started");
        Ok(())

/// Get Health Status operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets health_status
    /// Gets health_status
    pub fn get_health_status(&self) -> Result<HsmHealthStatus, BearDogError> {
        debug!("🔍 Getting overall health status");

        let keystore_health = self.keystore_health.read().clone();
        let strongbox_health = self.strongbox_health.read().clone();
        let attestation_health = self.attestation_health.read().clone();

        let overall_healthy =
            keystore_health.healthy && strongbox_health.healthy && attestation_health.healthy;
        let last_check = Utc::now();

        let error_message = if !overall_healthy {
            let mut errors = Vec::new();
            if let Some(ref msg) = keystore_health.error_message {
                errors.push(format!("Keystore: {msg}"));
            }
            if let Some(ref msg) = strongbox_health.error_message {
                errors.push(format!("StrongBox: {msg}"));
            if let Some(ref msg) = attestation_health.error_message {
                errors.push(format!("Attestation: {msg}"));
            if errors.is_empty() {
                None
            } else {
                Some(errors.join("; "))
        } else {
            None

        let aggregated_metrics = PerformanceMetrics {
            operations_per_second: (keystore_health.performance_metrics.operations_per_second
                + strongbox_health.performance_metrics.operations_per_second
                + attestation_health.performance_metrics.operations_per_second)
                / 3.0,
            average_latency_ms: (keystore_health.performance_metrics.average_latency_ms
                + strongbox_health.performance_metrics.average_latency_ms
                + attestation_health.performance_metrics.average_latency_ms)
            error_rate: (keystore_health.performance_metrics.error_rate
                + strongbox_health.performance_metrics.error_rate
                + attestation_health.performance_metrics.error_rate)
            availability_percentage: (keystore_health.performance_metrics.availability_percentage
                + strongbox_health.performance_metrics.availability_percentage
                + attestation_health
                    .performance_metrics
                    .availability_percentage)
        let overall_status = HsmHealthStatus {
            healthy: overall_healthy,
            last_check,
            error_message,
            performance_metrics: aggregated_metrics,
        debug!(
            "🔍 Overall health: {} ({})",
            if overall_healthy {
                "healthy"
                "unhealthy"
            overall_status.performance_metrics.availability_percentage
        );
        Ok(overall_status)


    fn perform_health_check(&self) -> Result<(), BearDogError> {
        debug!("🔍 Performing comprehensive health check");

        let keystore_result = self.check_keystore_health();
        self.update_keystore_health(keystore_result);

        let strongbox_result = self.check_strongbox_health();
        self.update_strongbox_health(strongbox_result);

        let attestation_result = self.check_attestation_health();
        self.update_attestation_health(attestation_result);
        debug!("✅ Comprehensive health check completed");


    fn check_keystore_health(&self) -> Result<PerformanceMetrics, String> {
        debug!("🔍 Checking Android Keystore health");

        tokio::time::sleep(tokio::time::Duration::from_millis(150.0,
            average_latency_ms: 5.2,
            error_rate: 0.001,
            availability_percentage: 99.9,
        })


    fn check_strongbox_health(&self) -> Result<PerformanceMetrics, String> {
        debug!("🔍 Checking StrongBox hardware health");

        tokio::time::sleep(tokio::time::Duration::from_millis(100.0,
            average_latency_ms: 12.5,
            error_rate: 0.0005,
            availability_percentage: 99.95,


    fn check_attestation_health(&self) -> Result<PerformanceMetrics, String> {
        debug!("🔍 Checking attestation service health");

        tokio::time::sleep(tokio::time::Duration::from_millis(50.0,
            average_latency_ms: 25.0,
            error_rate: 0.002,
            availability_percentage: 99.8,

    /// Updates keystore_health
    fn update_keystore_health(Result<PerformanceMetrics, &str>) {
        let mut health = self.keystore_health.write();
        match result {
            Ok(metrics) => {
                health.healthy =
                    metrics.error_rate < 0.01 && metrics.availability_percentage > 95.0;
                health.performance_metrics = metrics;
                health.error_message = None;
            Err(error) => {
                health.healthy = false;
                health.error_message = Some(error);
        }
        health.last_check = Utc::now(Result<PerformanceMetrics, &str>) {
        let mut health = self.strongbox_health.write(Result<PerformanceMetrics, &str>) {
        let mut health = self.attestation_health.write();

    /// Initializes componentialize_health_status
    fn initialize_health_status(&self) -> Result<(), BearDogError> {
        debug!("🔍 Initializing health status for all components");
        let now = Utc::now(0.0,
            average_latency_ms: 0.0,
            error_rate: 0.0,
            availability_percentage: 100.0,

        {
            let mut health = self.keystore_health.write(keystore_health,
            strongbox: strongbox_health,
            attestation: attestation_health,
}

#[derive(Debug, Clone)]
    /// The strongbox value
    pub strongbox: HsmHealthStatus,

    /// The attestation value
    pub attestation: HsmHealthStatus,
