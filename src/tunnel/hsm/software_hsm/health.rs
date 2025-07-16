//! # Software HSM Health Monitoring
//!
//! This module provides health monitoring functionality for the Software HSM.
//! It tracks system status, performance metrics, and provides health assessments.

use super::types::*;
use crate::error::BearDogResult;
use crate::tunnel::hsm::types::*;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

impl SoftwareHealthMonitor {
    /// Create a new software health monitor
    pub async fn new() -> BearDogResult<Self> {
        info!("Creating software health monitor");

        let health_status = Arc::new(RwLock::new(HsmHealthStatus {
            healthy: true,
            last_check: chrono::Utc::now(),
            error_message: None,
            performance_metrics: PerformanceMetrics::default(),
        }));

        let metrics = Arc::new(RwLock::new(PerformanceMetrics {
            operations_per_second: 0.0,
            average_latency_ms: 0.0,
            error_rate: 0.0,
            availability_percentage: 100.0,
        }));

        Ok(Self {
            health_status,
            metrics,
        })
    }

    /// Get current health status
    pub async fn get_health_status(&self) -> BearDogResult<HsmHealthStatus> {
        let status = self.health_status.read().await;
        Ok(status.clone())
    }

    /// Update health status
    pub async fn update_health_status(&self, status: HsmHealthStatus) -> BearDogResult<()> {
        let mut health_status = self.health_status.write().await;
        *health_status = status;

        debug!("Health status updated: healthy={}", health_status.healthy);
        Ok(())
    }

    /// Get performance metrics
    pub async fn get_performance_metrics(&self) -> BearDogResult<PerformanceMetrics> {
        let metrics = self.metrics.read().await;
        Ok(metrics.clone())
    }

    /// Update performance metrics
    pub async fn update_performance_metrics(
        &self,
        metrics: PerformanceMetrics,
    ) -> BearDogResult<()> {
        let mut perf_metrics = self.metrics.write().await;
        *perf_metrics = metrics;

        debug!(
            "Performance metrics updated: {:.2} ops/sec",
            perf_metrics.operations_per_second
        );
        Ok(())
    }

    /// Perform health check
    pub async fn perform_health_check(&self) -> BearDogResult<HsmHealthStatus> {
        info!("Performing health check");

        let mut healthy = true;
        let mut error_message = None;

        // Check key store health
        if self.check_keystore_health().await.is_err() {
            healthy = false;
            error_message = Some("Key store is unhealthy".to_string());
        }

        // Check crypto provider health
        if self.check_crypto_provider_health().await.is_err() {
            healthy = false;
            if error_message.is_none() {
                error_message = Some("Crypto provider is unhealthy".to_string());
            }
        }

        // Check memory protector health
        if self.check_memory_protector_health().await.is_err() {
            healthy = false;
            if error_message.is_none() {
                error_message = Some("Memory protector is unhealthy".to_string());
            }
        }

        // Check audit logger health
        if self.check_audit_logger_health().await.is_err() {
            healthy = false;
            if error_message.is_none() {
                error_message = Some("Audit logger is unhealthy".to_string());
            }
        }

        // Check system resources
        if self.check_system_resources().await.is_err() {
            healthy = false;
            if error_message.is_none() {
                error_message = Some("System resources are unhealthy".to_string());
            }
        }

        let performance_metrics = self.get_performance_metrics().await?;

        let health_status = HsmHealthStatus {
            healthy,
            last_check: chrono::Utc::now(),
            error_message,
            performance_metrics,
        };

        // Update stored health status
        self.update_health_status(health_status.clone()).await?;

        info!("Health check completed: healthy={}", health_status.healthy);
        Ok(health_status)
    }

    /// Check key store health
    async fn check_keystore_health(&self) -> BearDogResult<()> {
        // TODO: Implement actual key store health checks
        Ok(())
    }

    /// Check crypto provider health
    async fn check_crypto_provider_health(&self) -> BearDogResult<()> {
        // TODO: Implement actual crypto provider health checks
        Ok(())
    }

    /// Check memory protector health
    async fn check_memory_protector_health(&self) -> BearDogResult<()> {
        // TODO: Implement actual memory protector health checks
        Ok(())
    }

    /// Check audit logger health
    async fn check_audit_logger_health(&self) -> BearDogResult<()> {
        // TODO: Implement actual audit logger health checks
        Ok(())
    }

    /// Check system resources
    async fn check_system_resources(&self) -> BearDogResult<()> {
        // TODO: Implement actual system resource checks
        Ok(())
    }

    /// Record operation metrics
    pub async fn record_operation(
        &self,
        operation: &str,
        duration: std::time::Duration,
        success: bool,
    ) -> BearDogResult<()> {
        let mut metrics = self.metrics.write().await;

        // Update average latency (simplified calculation)
        let new_latency_ms = duration.as_secs_f64() * 1000.0;
        metrics.average_latency_ms = (metrics.average_latency_ms + new_latency_ms) / 2.0;

        // Update error rate (simplified calculation)
        if !success {
            metrics.error_rate = (metrics.error_rate + 1.0) / 2.0;
        } else {
            metrics.error_rate *= 0.99; // Decay error rate for successes
        }

        // Calculate operations per second (simplified)
        metrics.operations_per_second += 1.0;

        debug!(
            "Recorded operation: {} ({}ms, success: {})",
            operation, new_latency_ms, success
        );

        Ok(())
    }

    /// Update availability percentage
    pub async fn update_availability(&self, availability: f64) -> BearDogResult<()> {
        let mut metrics = self.metrics.write().await;
        metrics.availability_percentage = availability;

        debug!("Updated availability: {:.2}%", availability);
        Ok(())
    }

    /// Get simplified health summary
    pub async fn get_health_summary(&self) -> BearDogResult<SimpleHealthSummary> {
        let health_status = self.health_status.read().await;
        let metrics = self.metrics.read().await;

        Ok(SimpleHealthSummary {
            healthy: health_status.healthy,
            operations_per_second: metrics.operations_per_second,
            average_latency_ms: metrics.average_latency_ms,
            error_rate: metrics.error_rate,
            availability_percentage: metrics.availability_percentage,
            last_check: health_status.last_check,
        })
    }

    /// Reset metrics
    pub async fn reset_metrics(&self) -> BearDogResult<()> {
        let mut metrics = self.metrics.write().await;
        metrics.operations_per_second = 0.0;
        metrics.average_latency_ms = 0.0;
        metrics.error_rate = 0.0;
        metrics.availability_percentage = 100.0;

        info!("Performance metrics reset");
        Ok(())
    }

    /// Set health status
    pub async fn set_health_status(
        &self,
        healthy: bool,
        error_message: Option<String>,
    ) -> BearDogResult<()> {
        let mut health_status = self.health_status.write().await;
        health_status.healthy = healthy;
        health_status.error_message = error_message.clone();
        health_status.last_check = chrono::Utc::now();

        if healthy {
            info!("Health status set to healthy");
        } else {
            warn!("Health status set to unhealthy: {:?}", error_message);
        }

        Ok(())
    }
}

/// Simplified health summary for quick overview
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimpleHealthSummary {
    /// Whether the system is healthy
    pub healthy: bool,
    /// Current operations per second throughput
    pub operations_per_second: f64,
    /// Average latency in milliseconds
    pub average_latency_ms: f64,
    /// Error rate as a percentage (0.0 to 1.0)
    pub error_rate: f64,
    /// System availability as a percentage (0.0 to 100.0)
    pub availability_percentage: f64,
    /// Timestamp of the last health check
    pub last_check: chrono::DateTime<chrono::Utc>,
}
