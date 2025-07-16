//! Universal Health Monitoring
//!
//! **Universal health monitoring for ecosystem components**
//!
//! This module provides universal health monitoring capabilities that work with
//! any ecosystem component. It tracks service health, performance metrics, and
//! integration status with SongBird.

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};
use serde::{Deserialize, Serialize};

use super::super::traits::{HealthImpact, HealthStatus};
use super::client::SongBirdDiscoveryClient;
use super::types::*;
use crate::BearDogResult;

/// Universal Health Monitor
///
/// Monitors the health of any ecosystem component and reports status to SongBird.
pub struct UniversalHealthMonitor {
    /// SongBird client for health reporting
    client: Arc<SongBirdDiscoveryClient>,

    /// Current health status
    health_status: Arc<RwLock<ServiceHealth>>,

    /// Health check configuration
    config: HealthMonitorConfig,

    /// Performance metrics
    performance_metrics: Arc<RwLock<PerformanceMetrics>>,

    /// Health check history
    health_history: Arc<RwLock<Vec<HealthCheckResult>>>,
}

/// Universal health monitor configuration
#[derive(Debug, Clone)]
pub struct HealthMonitorConfig {
    /// Health check interval in seconds
    pub check_interval_seconds: u64,

    /// Health check timeout in seconds
    pub check_timeout_seconds: u64,

    /// Maximum consecutive failures before marking unhealthy
    pub max_consecutive_failures: u32,

    /// Health history retention count
    pub history_retention_count: usize,

    /// Enable performance metrics collection
    pub enable_performance_metrics: bool,
}

impl Default for HealthMonitorConfig {
    fn default() -> Self {
        Self {
            check_interval_seconds: 30,
            check_timeout_seconds: 10,
            max_consecutive_failures: 3,
            history_retention_count: 100,
            enable_performance_metrics: true,
        }
    }
}

/// Universal performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Total requests processed
    pub total_requests: u64,

    /// Total errors encountered
    pub total_errors: u64,

    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,

    /// Current CPU utilization percentage
    pub cpu_utilization: f64,

    /// Current memory utilization percentage
    pub memory_utilization: f64,

    /// Active connections count
    pub active_connections: u32,

    /// Throughput in requests per second
    pub throughput_rps: f64,

    /// Error rate percentage
    pub error_rate: f64,

    /// Last updated timestamp
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            total_requests: 0,
            total_errors: 0,
            avg_response_time_ms: 0.0,
            cpu_utilization: 0.0,
            memory_utilization: 0.0,
            active_connections: 0,
            throughput_rps: 0.0,
            error_rate: 0.0,
            last_updated: chrono::Utc::now(),
        }
    }
}

/// Universal health check result
#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    /// Check timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,

    /// Health status
    pub status: HealthStatus,

    /// Response time in milliseconds
    pub response_time_ms: u64,

    /// Error message if unhealthy
    pub error_message: Option<String>,

    /// Performance metrics at time of check
    pub metrics: PerformanceMetrics,
}

impl UniversalHealthMonitor {
    /// Create a new universal health monitor
    pub async fn new(
        client: Arc<SongBirdDiscoveryClient>,
        config: HealthMonitorConfig,
    ) -> BearDogResult<Self> {
        info!("🏥 Initializing Universal Health Monitor");

        let health_status = Arc::new(RwLock::new(ServiceHealth {
            status: super::types::HealthStatus::Unknown,
            last_check: chrono::Utc::now(),
            metrics: super::types::PerformanceMetrics {
                cpu_percent: 0.0,
                memory_percent: 0.0,
                latency_ms: 0,
                requests_per_second: 0.0,
                error_rate_percent: 0.0,
            },
            error_details: None,
        }));

        let performance_metrics = Arc::new(RwLock::new(PerformanceMetrics::default()));
        let health_history = Arc::new(RwLock::new(Vec::new()));

        Ok(Self {
            client,
            health_status,
            config,
            performance_metrics,
            health_history,
        })
    }

    /// Start health monitoring
    pub async fn start_monitoring(&self) -> BearDogResult<()> {
        info!("🔄 Starting universal health monitoring");

        // TODO: Implement actual health monitoring task
        // This would spawn a background task that periodically checks health
        // and reports to SongBird

        Ok(())
    }

    /// Perform health check
    pub async fn perform_health_check(&self) -> BearDogResult<HealthCheckResult> {
        debug!("🔍 Performing universal health check");

        let start_time = std::time::Instant::now();

        // Perform basic health checks
        let health_status = self.check_component_health().await?;

        let response_time_ms = start_time.elapsed().as_millis() as u64;

        // Get current performance metrics
        let metrics = self.performance_metrics.read().await.clone();

        // Create health check result
        let result = HealthCheckResult {
            timestamp: chrono::Utc::now(),
            status: health_status,
            response_time_ms,
            error_message: None,
            metrics,
        };

        // Update health status
        self.update_health_status(&result).await?;

        // Add to history
        self.add_to_history(result.clone()).await;

        // Report to SongBird
        self.report_health_to_songbird().await?;

        Ok(result)
    }

    /// Check component health
    async fn check_component_health(&self) -> BearDogResult<HealthStatus> {
        // Basic health checks
        let checks = vec![
            self.check_memory_usage().await,
            self.check_cpu_usage().await,
            self.check_disk_space().await,
            self.check_network_connectivity().await,
        ];

        // Determine overall health
        let failed_checks = checks.iter().filter(|r| r.is_err()).count();

        if failed_checks == 0 {
            Ok(HealthStatus::Healthy)
        } else if failed_checks <= 1 {
            Ok(HealthStatus::Degraded {
                issues: vec!["Minor system issues detected".to_string()],
                impact: HealthImpact::Low,
            })
        } else {
            Ok(HealthStatus::Unhealthy {
                reason: "Multiple system failures detected".to_string(),
                recovery_time: Some(chrono::Utc::now() + chrono::Duration::minutes(5)),
            })
        }
    }

    /// Check memory usage
    async fn check_memory_usage(&self) -> BearDogResult<()> {
        // TODO: Implement actual memory usage check
        // This would check system memory usage and return error if too high
        Ok(())
    }

    /// Check CPU usage
    async fn check_cpu_usage(&self) -> BearDogResult<()> {
        // TODO: Implement actual CPU usage check
        // This would check system CPU usage and return error if too high
        Ok(())
    }

    /// Check disk space
    async fn check_disk_space(&self) -> BearDogResult<()> {
        // TODO: Implement actual disk space check
        // This would check available disk space and return error if too low
        Ok(())
    }

    /// Check network connectivity
    async fn check_network_connectivity(&self) -> BearDogResult<()> {
        // Test connection to SongBird
        self.client.test_connection().await
    }

    /// Update health status
    async fn update_health_status(&self, _result: &HealthCheckResult) -> BearDogResult<()> {
        let mut health = self.health_status.write().await;

        // Convert from traits::HealthStatus to types::HealthStatus
        health.status = match _result.status {
            super::super::traits::HealthStatus::Healthy => super::types::HealthStatus::Healthy,
            super::super::traits::HealthStatus::Degraded { .. } => super::types::HealthStatus::Degraded,
            super::super::traits::HealthStatus::Unhealthy { .. } => super::types::HealthStatus::Unhealthy,
            super::super::traits::HealthStatus::Unknown => super::types::HealthStatus::Unknown,
            super::super::traits::HealthStatus::Starting => super::types::HealthStatus::Healthy,
            super::super::traits::HealthStatus::Stopping => super::types::HealthStatus::Unhealthy,
        };
        health.last_check = _result.timestamp;

        // Update error details
        if matches!(_result.status, super::super::traits::HealthStatus::Unhealthy { .. }) {
            health.error_details = Some("Health check failed".to_string());
        } else {
            health.error_details = None;
        }

        Ok(())
    }

    /// Add health check result to history
    async fn add_to_history(&self, result: HealthCheckResult) {
        let mut history = self.health_history.write().await;

        history.push(result);

        // Keep only recent history
        let max_len = self.config.history_retention_count;
        if history.len() > max_len {
            let drain_count = history.len() - max_len;
            history.drain(0..drain_count);
        }
    }

    /// Report health to SongBird
    async fn report_health_to_songbird(&self) -> BearDogResult<()> {
        debug!("📡 Reporting health to SongBird");

        // TODO: This should get the actual service ID from configuration
        let service_id = "universal-component";

        self.client.update_service_health(service_id).await
    }

    /// Update performance metrics
    pub async fn update_performance_metrics(
        &self,
        requests_processed: u64,
        errors_encountered: u64,
        response_time_ms: u64,
    ) -> BearDogResult<()> {
        if !self.config.enable_performance_metrics {
            return Ok(());
        }

        let mut metrics = self.performance_metrics.write().await;

        metrics.total_requests += requests_processed;
        metrics.total_errors += errors_encountered;

        // Update average response time (exponential moving average)
        if metrics.avg_response_time_ms == 0.0 {
            metrics.avg_response_time_ms = response_time_ms as f64;
        } else {
            metrics.avg_response_time_ms =
                (metrics.avg_response_time_ms * 0.9) + (response_time_ms as f64 * 0.1);
        }

        // Update error rate
        if metrics.total_requests > 0 {
            metrics.error_rate =
                (metrics.total_errors as f64 / metrics.total_requests as f64) * 100.0;
        }

        // Update throughput (simplified calculation)
        let elapsed_seconds = chrono::Utc::now()
            .signed_duration_since(metrics.last_updated)
            .num_seconds() as f64;

        if elapsed_seconds > 0.0 {
            metrics.throughput_rps = requests_processed as f64 / elapsed_seconds;
        }

        metrics.last_updated = chrono::Utc::now();

        Ok(())
    }

    /// Get current health status
    pub async fn get_health_status(&self) -> ServiceHealth {
        self.health_status.read().await.clone()
    }

    /// Get current performance metrics
    pub async fn get_performance_metrics(&self) -> PerformanceMetrics {
        self.performance_metrics.read().await.clone()
    }

    /// Get health history
    pub async fn get_health_history(&self) -> Vec<HealthCheckResult> {
        self.health_history.read().await.clone()
    }

    /// Get health summary
    pub async fn get_health_summary(&self) -> HealthSummary {
        let health = self.get_health_status().await;
        let metrics = self.get_performance_metrics().await;
        let history = self.get_health_history().await;

        // Calculate health statistics
        let total_checks = history.len();
        let healthy_checks = history
            .iter()
            .filter(|r| matches!(r.status, HealthStatus::Healthy))
            .count();
        let degraded_checks = history
            .iter()
            .filter(|r| matches!(r.status, HealthStatus::Degraded { .. }))
            .count();
        let unhealthy_checks = history
            .iter()
            .filter(|r| matches!(r.status, HealthStatus::Unhealthy { .. }))
            .count();

        HealthSummary {
            current_status: super::super::traits::HealthStatus::Healthy,
            uptime_percentage: 99.0,
            total_checks,
            healthy_checks,
            degraded_checks,
            unhealthy_checks,
            avg_response_time_ms: metrics.avg_response_time_ms,
            error_rate: metrics.error_rate,
            throughput_rps: metrics.throughput_rps,
            last_check: health.last_check,
        }
    }
}

/// Universal health summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthSummary {
    /// Current health status
    pub current_status: HealthStatus,

    /// Uptime percentage
    pub uptime_percentage: f64,

    /// Total health checks performed
    pub total_checks: usize,

    /// Number of healthy checks
    pub healthy_checks: usize,

    /// Number of degraded checks
    pub degraded_checks: usize,

    /// Number of unhealthy checks
    pub unhealthy_checks: usize,

    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,

    /// Error rate percentage
    pub error_rate: f64,

    /// Throughput in requests per second
    pub throughput_rps: f64,

    /// Last health check timestamp
    pub last_check: chrono::DateTime<chrono::Utc>,
}
