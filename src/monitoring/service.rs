use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::error::BearDogResult;
use crate::utils::env_utils::ObservabilityConfig;
use super::health::HealthChecker;
use super::types::{
    AlertThresholds, ComponentHealth, HealthStatus, PerformanceMetrics, SystemHealth, SystemMetrics,
};

/// Monitoring service for production deployments
pub struct MonitoringService {
    /// Start time of the monitoring service
    start_time: Instant,
    /// List of registered health checkers
    checkers: Vec<Arc<dyn HealthChecker>>,
    /// Current system metrics
    metrics: Arc<RwLock<SystemMetrics>>,
    /// Observability configuration
    config: ObservabilityConfig,
    /// Alert thresholds for monitoring
    alert_thresholds: AlertThresholds,
}

impl MonitoringService {
    /// Create a new monitoring service
    pub fn new(config: ObservabilityConfig) -> Self {
        Self {
            start_time: Instant::now(),
            checkers: Vec::new(),
            metrics: Arc::new(RwLock::new(Self::default_metrics())),
            config,
            alert_thresholds: AlertThresholds::default(),
        }
    }

    /// Register a health checker
    pub fn register_checker(&mut self, checker: Arc<dyn HealthChecker>) {
        self.checkers.push(checker);
    }

    /// Get system health status
    pub async fn get_health(&self) -> BearDogResult<SystemHealth> {
        let mut components = Vec::new();
        let mut overall_status = HealthStatus::Healthy;

        // Check all registered components
        for checker in &self.checkers {
            let check_start = Instant::now();

            match checker.check_health().await {
                Ok(mut component) => {
                    component.check_duration_ms = check_start.elapsed().as_millis() as u64;

                    // Update overall status based on component status
                    match component.status {
                        HealthStatus::Unhealthy => overall_status = HealthStatus::Unhealthy,
                        HealthStatus::Degraded if overall_status == HealthStatus::Healthy => {
                            overall_status = HealthStatus::Degraded;
                        }
                        _ => {}
                    }

                    components.push(component);
                }
                Err(e) => {
                    overall_status = HealthStatus::Unhealthy;
                    components.push(ComponentHealth {
                        name: checker.component_name().to_string(),
                        status: HealthStatus::Unhealthy,
                        message: Some(format!("Health check failed: {e}")),
                        last_check: Utc::now(),
                        check_duration_ms: check_start.elapsed().as_millis() as u64,
                        metadata: HashMap::new(),
                    });
                }
            }
        }

        Ok(SystemHealth {
            status: overall_status,
            uptime_seconds: self.start_time.elapsed().as_secs() as i64,
            version: env!("CARGO_PKG_VERSION").to_string(),
            components,
            last_updated: Utc::now(),
        })
    }

    /// Get system metrics
    pub async fn get_metrics(&self) -> SystemMetrics {
        self.metrics.read().await.clone()
    }

    /// Update system metrics
    pub async fn update_metrics(&self, metrics: SystemMetrics) {
        let mut current_metrics = self.metrics.write().await;
        *current_metrics = metrics.clone();

        // Check for alerts
        self.check_alerts(&metrics).await;
    }

    /// Start monitoring loop
    pub async fn start_monitoring(&self) -> BearDogResult<()> {
        if !self.config.enable_metrics {
            tracing::info!("Monitoring disabled in configuration");
            return Ok(());
        }

        tracing::info!("Starting monitoring service");

        let metrics = self.metrics.clone();
        let thresholds = self.alert_thresholds.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));

            loop {
                interval.tick().await;

                // Collect system metrics
                let current_metrics = Self::collect_system_metrics().await;

                // Update metrics
                {
                    let mut metrics_guard = metrics.write().await;
                    *metrics_guard = current_metrics.clone();
                }

                // Check thresholds and log warnings
                Self::check_thresholds(&current_metrics, &thresholds).await;
            }
        });

        Ok(())
    }

    /// Collect system performance metrics
    async fn collect_system_metrics() -> SystemMetrics {
        // In a real implementation, you'd use system APIs to get actual metrics
        // For now, we'll provide placeholder implementations

        SystemMetrics {
            timestamp: Utc::now(),
            performance: PerformanceMetrics {
                cpu_usage_percent: Self::get_cpu_usage().await,
                memory_usage_bytes: Self::get_memory_usage().await,
                memory_total_bytes: Self::get_total_memory().await,
                disk_usage_bytes: Self::get_disk_usage().await,
                disk_total_bytes: Self::get_total_disk().await,
                network_rx_bytes: Self::get_network_rx().await,
                network_tx_bytes: Self::get_network_tx().await,
                active_connections: Self::get_active_connections().await,
                request_count: Self::get_request_count().await,
                error_count: Self::get_error_count().await,
                avg_response_time_ms: Self::get_avg_response_time().await,
            },
            custom_metrics: HashMap::new(),
        }
    }

    /// Check alert thresholds
    async fn check_alerts(&self, metrics: &SystemMetrics) {
        let perf = &metrics.performance;
        let thresholds = &self.alert_thresholds;

        if perf.cpu_usage_percent > thresholds.cpu_threshold {
            tracing::warn!("High CPU usage: {:.1}%", perf.cpu_usage_percent);
        }

        let memory_usage_percent =
            (perf.memory_usage_bytes as f64 / perf.memory_total_bytes as f64) * 100.0;
        if memory_usage_percent > thresholds.memory_threshold {
            tracing::warn!("High memory usage: {:.1}%", memory_usage_percent);
        }

        let disk_usage_percent =
            (perf.disk_usage_bytes as f64 / perf.disk_total_bytes as f64) * 100.0;
        if disk_usage_percent > thresholds.disk_threshold {
            tracing::warn!("High disk usage: {:.1}%", disk_usage_percent);
        }

        if perf.avg_response_time_ms > thresholds.response_time_threshold {
            tracing::warn!("High response time: {:.1}ms", perf.avg_response_time_ms);
        }
    }

    /// Check thresholds (static version for spawn)
    async fn check_thresholds(metrics: &SystemMetrics, thresholds: &AlertThresholds) {
        let perf = &metrics.performance;

        if perf.cpu_usage_percent > thresholds.cpu_threshold {
            tracing::warn!("High CPU usage: {:.1}%", perf.cpu_usage_percent);
        }

        let memory_usage_percent =
            (perf.memory_usage_bytes as f64 / perf.memory_total_bytes as f64) * 100.0;
        if memory_usage_percent > thresholds.memory_threshold {
            tracing::warn!("High memory usage: {:.1}%", memory_usage_percent);
        }
    }

    fn default_metrics() -> SystemMetrics {
        SystemMetrics {
            timestamp: Utc::now(),
            performance: PerformanceMetrics {
                cpu_usage_percent: 0.0,
                memory_usage_bytes: 0,
                memory_total_bytes: 0,
                disk_usage_bytes: 0,
                disk_total_bytes: 0,
                network_rx_bytes: 0,
                network_tx_bytes: 0,
                active_connections: 0,
                request_count: 0,
                error_count: 0,
                avg_response_time_ms: 0.0,
            },
            custom_metrics: HashMap::new(),
        }
    }

    // Placeholder system metric collection methods
    // In production, these would integrate with actual system APIs
    async fn get_cpu_usage() -> f64 {
        // Use procfs, sysinfo, or similar crate for real CPU usage
        rand::random::<f64>() * 20.0 // Simulate 0-20% CPU usage
    }

    async fn get_memory_usage() -> u64 {
        // Real implementation would read from /proc/meminfo or similar
        1024 * 1024 * 256 // 256MB
    }

    async fn get_total_memory() -> u64 {
        1024 * 1024 * 1024 * 4 // 4GB
    }

    async fn get_disk_usage() -> u64 {
        1024 * 1024 * 1024 * 10 // 10GB
    }

    async fn get_total_disk() -> u64 {
        1024 * 1024 * 1024 * 100 // 100GB
    }

    async fn get_network_rx() -> u64 {
        1024 * 1024 * 50 // 50MB
    }

    async fn get_network_tx() -> u64 {
        1024 * 1024 * 30 // 30MB
    }

    async fn get_active_connections() -> u32 {
        42 // Placeholder
    }

    async fn get_request_count() -> u64 {
        1000 // Placeholder
    }

    async fn get_error_count() -> u64 {
        5 // Placeholder
    }

    async fn get_avg_response_time() -> f64 {
        150.0 // 150ms average
    }
} 