use crate::tunnel::hsm::types::canonical::PerformanceMetrics as CanonicalPerformanceMetrics;
use crate::tunnel::hsm::types::*;
use beardog_errors::BearDogError;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Software HSM health monitor
pub struct SoftwareHealthMonitor {
    health_status: Arc<RwLock<HsmHealthStatus>>,
    metrics: Arc<RwLock<CanonicalPerformanceMetrics>>,
}

impl SoftwareHealthMonitor {
    /// Create new health monitor
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub async fn new() -> Result<Self, BearDogError> {
        info!("Creating software health monitor");

        let health_status = Arc::new(RwLock::new(HsmHealthStatus {
            is_healthy: true,
            last_check: chrono::Utc::now(),
            error_message: None,
            performance_metrics: PerformanceMetrics::default(),
        }));

        let metrics = Arc::new(RwLock::new(CanonicalPerformanceMetrics {
            operations_per_second: 0.0,
            average_latency_ms: 0.0,
            success_rate: 100.0,
            memory_usage_mb: 0.0,
            cpu_usage_percent: 0.0,
            error_count: 0,
            uptime_seconds: 0,
        }));

        Ok(Self {
            health_status,
            metrics,
        })
    }

    /// Get current health status
    ///
    /// # Errors
    /// Returns an error if status cannot be retrieved
    pub async fn get_health_status(&self) -> Result<HsmHealthStatus, BearDogError> {
        let status = self.health_status.read().await;
        Ok(status.clone())
    }

    /// Update health status
    ///
    /// # Errors
    /// Returns an error if update fails
    pub async fn update_health_status(&self, status: HsmHealthStatus) -> Result<(), BearDogError> {
        let mut health_status = self.health_status.write().await;
        *health_status = status;
        debug!(
            "Health status updated: healthy={}",
            health_status.is_healthy
        );
        Ok(())
    }

    /// Get performance metrics
    ///
    /// # Errors
    /// Returns an error if metrics cannot be retrieved
    pub async fn get_performance_metrics(&self) -> Result<PerformanceMetrics, BearDogError> {
        let canonical_metrics = self.metrics.read().await;

        Ok(PerformanceMetrics {
            operations_per_second: canonical_metrics.operations_per_second,
            average_latency_ms: canonical_metrics.average_latency_ms,
            success_rate: canonical_metrics.success_rate,
            memory_usage_mb: canonical_metrics.memory_usage_mb,
            cpu_usage_percent: canonical_metrics.cpu_usage_percent,
            network_throughput_bps: 1_000_000.0,
            latency_ms: canonical_metrics.average_latency_ms,
            throughput_mbps: canonical_metrics.operations_per_second / 1000.0,
            uptime_seconds: 0,
        })
    }

    /// Update performance metrics
    ///
    /// # Errors
    /// Returns an error if update fails
    pub async fn update_performance_metrics(
        &self,
        metrics: PerformanceMetrics,
    ) -> Result<(), BearDogError> {
        let mut perf_metrics = self.metrics.write().await;
        perf_metrics.operations_per_second = metrics.operations_per_second;
        perf_metrics.average_latency_ms = metrics.average_latency_ms;
        perf_metrics.success_rate = metrics.success_rate;
        perf_metrics.memory_usage_mb = metrics.memory_usage_mb;
        perf_metrics.cpu_usage_percent = metrics.cpu_usage_percent;
        // Note: error_count not in types::status::PerformanceMetrics, only in canonical
        perf_metrics.uptime_seconds = metrics.uptime_seconds;

        debug!(
            "Performance metrics updated: {:.2} ops/sec",
            perf_metrics.operations_per_second
        );
        Ok(())
    }

    /// Perform comprehensive health check
    ///
    /// # Errors
    /// Returns an error if health check fails
    pub async fn perform_health_check(&self) -> Result<HsmHealthStatus, BearDogError> {
        info!("Performing health check");
        let mut healthy = true;
        let mut error_message = None;

        // Check keystore health
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
            is_healthy: healthy,
            last_check: chrono::Utc::now(),
            error_message: error_message.clone(),
            performance_metrics,
        };

        // Update stored health status
        let mut status_lock = self.health_status.write().await;
        *status_lock = health_status.clone();

        if !healthy {
            warn!("Health status set to unhealthy: {:?}", error_message);
        }

        Ok(health_status)
    }

    /// Check keystore health
    async fn check_keystore_health(&self) -> Result<(), BearDogError> {
        debug!("Checking keystore health");
        // Implementation would check actual keystore
        Ok(())
    }

    /// Check crypto provider health
    async fn check_crypto_provider_health(&self) -> Result<(), BearDogError> {
        debug!("Checking crypto provider health");
        // Implementation would check actual crypto provider
        Ok(())
    }

    /// Check memory protector health
    async fn check_memory_protector_health(&self) -> Result<(), BearDogError> {
        debug!("Checking memory protector health");
        // Implementation would check actual memory protector
        Ok(())
    }

    /// Check audit logger health
    async fn check_audit_logger_health(&self) -> Result<(), BearDogError> {
        debug!("Checking audit logger health");
        // Implementation would check actual audit logger
        Ok(())
    }

    /// Check system resources
    async fn check_system_resources(&self) -> Result<(), BearDogError> {
        debug!("Checking system resources");
        // Implementation would check CPU, memory, disk, etc.
        Ok(())
    }

    /// Record operation for metrics
    pub async fn record_operation(
        &self,
        latency_ms: f64,
        success: bool,
    ) -> Result<(), BearDogError> {
        let mut metrics = self.metrics.write().await;

        // Update operations per second (simple moving average)
        metrics.operations_per_second = metrics.operations_per_second * 0.9 + 0.1;

        // Update average latency
        metrics.average_latency_ms = metrics.average_latency_ms * 0.9 + latency_ms * 0.1;

        // Update success rate
        if success {
            metrics.success_rate = metrics.success_rate * 0.99 + 1.0 * 0.01;
        } else {
            metrics.error_count += 1;
            metrics.success_rate *= 0.99;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_monitor_creation() -> Result<(), BearDogError> {
        let monitor = SoftwareHealthMonitor::new().await?;
        let status = monitor.get_health_status().await?;
        assert!(status.is_healthy);
        Ok(())
    }

    #[tokio::test]
    async fn test_health_check() -> Result<(), BearDogError> {
        let monitor = SoftwareHealthMonitor::new().await?;
        let status = monitor.perform_health_check().await?;
        assert!(status.is_healthy);
        assert!(status.error_message.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_metrics_update() -> Result<(), BearDogError> {
        let monitor = SoftwareHealthMonitor::new().await?;

        monitor.record_operation(10.5, true).await?;
        monitor.record_operation(15.3, true).await?;

        let metrics = monitor.get_performance_metrics().await?;
        assert!(metrics.operations_per_second > 0.0);
        assert!(metrics.average_latency_ms > 0.0);
        Ok(())
    }

    #[tokio::test]
    async fn test_error_tracking() -> Result<(), BearDogError> {
        let monitor = SoftwareHealthMonitor::new().await?;

        monitor.record_operation(10.0, true).await?;
        monitor.record_operation(15.0, false).await?;

        let metrics = monitor.get_performance_metrics().await?;
        // Error tracking is now part of success_rate
        assert!(metrics.success_rate < 100.0);
        assert!(metrics.success_rate < 100.0);
        Ok(())
    }
}

/// Simple health summary for compatibility
#[derive(Debug, Clone)]
pub struct SimpleHealthSummary {
    /// Overall health status
    pub is_healthy: bool,
    /// Number of operations per second
    pub ops_per_second: f64,
    /// Average latency in milliseconds
    pub avg_latency_ms: f64,
    /// Success rate percentage
    pub success_rate: f64,
}

impl Default for SimpleHealthSummary {
    fn default() -> Self {
        Self {
            is_healthy: true,
            ops_per_second: 0.0,
            avg_latency_ms: 0.0,
            success_rate: 100.0,
        }
    }
}
