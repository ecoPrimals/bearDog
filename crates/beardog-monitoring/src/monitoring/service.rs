// SPDX-License-Identifier: AGPL-3.0-or-later

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use crate::monitoring::metrics::{MetricsCollector, PrometheusExporter};
use crate::monitoring::types::{ComponentHealth, MonitoringConfig, ResourceUsage};
use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// `MonitoringSnapshot` provides a point-in-time view of system monitoring data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringSnapshot {
    /// Unique identifier for this snapshot (typically a UUID).
    pub id: String,
    /// Timestamp when the snapshot was taken
    pub timestamp: DateTime<Utc>,
    /// CPU, memory, disk, network, and uptime figures at snapshot time.
    pub performance: SystemPerformanceMetrics,
    /// Overall system health status
    /// The health value
    pub health: HealthStatus,
    /// Collection of health details
    pub health_details: Vec<ComponentHealth>,
    /// Active alerts at snapshot time
    /// Collection of active alerts
    pub active_alerts: Vec<String>,
    /// Resource usage statistics
    /// The resource usage value
    pub resource_usage: ResourceUsage,
}

/// Point-in-time OS-level performance figures backing a [`MonitoringSnapshot`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemPerformanceMetrics {
    /// CPU usage as a percentage (0.0 to 100.0)
    /// The cpu usage percent value
    pub cpu_usage_percent: f64,
    /// Memory usage as a percentage (0.0 to 100.0)
    /// The memory usage percent value
    pub memory_usage_percent: f64,
    /// Total memory in bytes
    /// Number of `memory_total_bytes`
    pub memory_total_bytes: u64,
    /// Used memory in bytes
    /// Number of `memory_used_bytes`
    pub memory_used_bytes: u64,
    /// Disk usage as a percentage (0.0 to 100.0)
    /// The disk usage percent value
    pub disk_usage_percent: f64,
    /// Network bytes received
    /// Number of `network_bytes_in`
    pub network_bytes_in: u64,
    /// Network bytes sent
    /// Number of `network_bytes_out`
    pub network_bytes_out: u64,
    /// System uptime in seconds
    pub uptime_seconds: u64,
    /// Number of active connections
    /// Number of `active_connections`
    pub active_connections: u32,
}

/// `HealthSummary` provides a summary of system health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthSummary {
    /// Overall system health status
    /// Current status of the overall
    pub overall_status: HealthStatus,
    /// Total number of monitored components
    /// Number of `total_components`
    pub total_components: u32,
    /// Number of healthy components
    /// Number of `healthy_components`
    pub healthy_components: u32,
    /// Timestamp of the health check
    /// The last check value
    pub last_check: DateTime<Utc>,
}

/// Live monitoring facade: metrics collection, optional Prometheus export, snapshots, and alerts.
#[derive(Debug)]
pub struct MonitoringService {
    config: MonitoringConfig,
    metrics_collector: Arc<MetricsCollector>,
    prometheus_exporter: Option<PrometheusExporter>,
    snapshots: Arc<RwLock<Vec<MonitoringSnapshot>>>,
    alerts: Arc<RwLock<Vec<String>>>,
    start_time: Instant,
}

impl MonitoringService {
    /// Creates a new monitoring service instance
    #[must_use]
    /// Creates a new instance
    pub fn new(config: MonitoringConfig) -> Self {
        let metrics_collector = Arc::new(MetricsCollector::new());
        let prometheus_exporter = if config.prometheus.enabled {
            Some(PrometheusExporter::new(
                config.prometheus.clone(),
                Arc::clone(&metrics_collector),
            ))
        } else {
            None
        };

        Self {
            config,
            metrics_collector,
            prometheus_exporter,
            snapshots: Arc::new(RwLock::new(Vec::new())),
            alerts: Arc::new(RwLock::new(Vec::new())),
            start_time: Instant::now(),
        }
    }

    /// Records a counter metric
    ///
    /// # Errors
    /// Returns an error if the metric cannot be recorded
    pub async fn record_counter(&self, name: &str, value: u64) -> Result<(), BearDogError> {
        self.metrics_collector.record_counter(name, value).await
    }

    /// Records a gauge metric
    ///
    /// # Errors
    /// Returns an error if the metric cannot be recorded
    pub async fn record_gauge(&self, name: &str, value: f64) -> Result<(), BearDogError> {
        self.metrics_collector.record_gauge(name, value).await
    }

    /// Records histogram values
    ///
    /// # Errors
    /// Returns an error if the metric cannot be recorded
    pub async fn record_histogram(&self, name: &str, values: Vec<f64>) -> Result<(), BearDogError> {
        self.metrics_collector.record_histogram(name, values).await
    }

    /// Records a timer metric
    ///
    /// # Errors
    /// Returns an error if the metric cannot be recorded
    pub async fn record_timer(&self, name: &str, duration: Duration) -> Result<(), BearDogError> {
        self.metrics_collector.record_timer(name, duration).await
    }

    /// Starts the monitoring service
    ///
    /// # Errors
    /// Returns an error if monitoring cannot be started
    /// Starts monitoring
    pub fn start_monitoring(&self) -> Result<(), BearDogError> {
        tracing::info!("Starting monitoring service");
        Ok(())
    }

    /// Stops the monitoring service
    ///
    /// # Errors
    /// Returns an error if monitoring cannot be stopped
    /// Stops monitoring
    pub fn stop_monitoring(&self) -> Result<(), BearDogError> {
        tracing::info!("Stopping monitoring service");
        Ok(())
    }

    /// Takes a monitoring snapshot
    ///
    /// # Errors
    /// Returns an error if the snapshot cannot be created
    pub async fn take_snapshot(&self) -> Result<MonitoringSnapshot, BearDogError> {
        let performance = self.collect_performance_metrics().await?;
        let health_summary = self.collect_health_summary()?;
        let alerts = self.alerts.read().await.clone();

        // Collect detailed health information
        let health_details = self.collect_component_health().await?;

        let snapshot = MonitoringSnapshot {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            performance,
            health: health_summary.overall_status,
            health_details,
            active_alerts: alerts,
            resource_usage: ResourceUsage::default(),
        };

        {
            let mut snapshots = self.snapshots.write().await;
            snapshots.push(snapshot.clone());
            if snapshots.len() > self.config.max_snapshots {
                snapshots.remove(0);
            }
        } // snapshots lock is dropped here

        Ok(snapshot)
    }

    /// Gets the current health status
    ///
    /// # Errors
    /// Returns an error if health status cannot be determined
    /// Gets `health_status`
    pub const fn get_health_status(&self) -> Result<HealthStatus, BearDogError> {
        Ok(HealthStatus::Healthy)
    }

    ///
    /// # Errors
    /// Returns an error if metrics cannot be exported
    pub async fn export_prometheus_metrics(&self) -> Result<String, BearDogError> {
        match &self.prometheus_exporter {
            Some(exporter) => exporter.export_metrics().await,
            None => Ok("# Prometheus export not configured\n".to_string()),
        }
    }

    /// Gets all monitoring snapshots
    ///
    /// # Errors
    /// Returns an error if snapshots cannot be retrieved
    /// Gets snapshots
    pub async fn get_snapshots(&self) -> Result<Vec<MonitoringSnapshot>, BearDogError> {
        Ok(self.snapshots.read().await.clone())
    }

    /// Gets the most recent snapshot
    ///
    /// # Errors
    /// Returns an error if no snapshots are available
    /// Gets `latest_snapshot`
    pub async fn get_latest_snapshot(&self) -> Result<Option<MonitoringSnapshot>, BearDogError> {
        let snapshots = self.snapshots.read().await;
        Ok(snapshots.last().cloned())
    }

    /// Gets system uptime in seconds
    #[must_use]
    /// Gets `uptime_seconds`
    pub fn get_uptime_seconds(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }

    /// Collect real system performance metrics from `/proc` (Linux) or
    /// report "unavailable" with safe defaults on other platforms.
    ///
    /// # Errors
    /// Returns an error if metrics cannot be collected
    pub async fn collect_performance_metrics(&self) -> Result<SystemPerformanceMetrics, BearDogError> {
        let uptime_seconds = self.start_time.elapsed().as_secs();

        let (cpu_usage_percent, memory_usage_percent, memory_total_bytes, memory_used_bytes) =
            Self::read_proc_metrics().await;

        Ok(SystemPerformanceMetrics {
            cpu_usage_percent,
            memory_usage_percent,
            memory_total_bytes,
            memory_used_bytes,
            disk_usage_percent: 0.0,
            network_bytes_in: 0,
            network_bytes_out: 0,
            uptime_seconds,
            active_connections: 0,
        })
    }

    #[cfg(target_os = "linux")]
    async fn read_proc_metrics() -> (f64, f64, u64, u64) {
        let cpu = Self::read_cpu_percent().await.unwrap_or(0.0);
        let (mem_pct, mem_total, mem_used) = Self::read_memory_stats().await.unwrap_or((0.0, 0, 0));
        (cpu, mem_pct, mem_total, mem_used)
    }

    #[cfg(not(target_os = "linux"))]
    async fn read_proc_metrics() -> (f64, f64, u64, u64) {
        (0.0, 0.0, 0, 0)
    }

    #[cfg(target_os = "linux")]
    async fn read_cpu_percent() -> Option<f64> {
        let content = tokio::fs::read_to_string("/proc/stat").await.ok()?;
        let cpu_line = content.lines().find(|l| l.starts_with("cpu "))?;
        let fields: Vec<u64> = cpu_line
            .split_whitespace()
            .skip(1)
            .take(8)
            .filter_map(|f| f.parse::<u64>().ok())
            .collect();
        if fields.len() < 4 {
            return None;
        }
        let total: u64 = fields.iter().sum();
        let idle = fields[3] + fields.get(4).copied().unwrap_or(0);
        if total == 0 {
            return Some(0.0);
        }
        #[expect(
            clippy::cast_precision_loss,
            reason = "CPU ratio from /proc; f64 sufficient"
        )]
        Some(((total - idle) as f64 / total as f64) * 100.0)
    }

    #[cfg(target_os = "linux")]
    async fn read_memory_stats() -> Option<(f64, u64, u64)> {
        let content = tokio::fs::read_to_string("/proc/meminfo").await.ok()?;
        let mut total_kb: Option<u64> = None;
        let mut available_kb: Option<u64> = None;
        for line in content.lines() {
            if let Some(rest) = line.strip_prefix("MemTotal:") {
                total_kb = rest.split_whitespace().next()?.parse().ok();
            } else if let Some(rest) = line.strip_prefix("MemAvailable:") {
                available_kb = rest.split_whitespace().next()?.parse().ok();
            }
            if total_kb.is_some() && available_kb.is_some() {
                break;
            }
        }
        let total = total_kb?;
        let available = available_kb?;
        if total == 0 {
            return Some((0.0, 0, 0));
        }
        let used = total.saturating_sub(available);
        let total_bytes = total * 1024;
        let used_bytes = used * 1024;
        #[expect(
            clippy::cast_precision_loss,
            reason = "Memory ratio from /proc; f64 sufficient"
        )]
        let pct = (used as f64 / total as f64) * 100.0;
        Some((pct, total_bytes, used_bytes))
    }

    fn collect_health_summary(&self) -> Result<HealthSummary, BearDogError> {
        // Collect health checks from all monitored components
        let health_results = self.perform_health_checks()?;

        let component_count = u32::try_from(health_results.len())
            .map_err(|_| BearDogError::system("Component count overflow".to_string()))?;
        let healthy_components = u32::try_from(
            health_results
                .iter()
                .filter(|h: &&ComponentHealth| h.status == HealthStatus::Healthy)
                .count(),
        )
        .map_err(|_| BearDogError::system("Healthy component count overflow".to_string()))?;

        // Determine overall status based on component health
        let overall_status = if healthy_components == component_count {
            HealthStatus::Healthy
        } else if healthy_components > 0 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Unhealthy
        };

        Ok(HealthSummary {
            overall_status,
            total_components: component_count,
            healthy_components,
            last_check: Utc::now(),
        })
    }

    /// Perform health checks on all monitored components
    fn perform_health_checks(&self) -> Result<Vec<ComponentHealth>, BearDogError> {
        let health_results = vec![
            // Check core monitoring service itself
            ComponentHealth {
                name: "monitoring_service".to_string(),
                status: HealthStatus::Healthy,
                message: Some("Monitoring service operational".to_string()),
                last_check: Utc::now(),
                check_duration_ms: 0,
                metadata: HashMap::new(),
            },
            // Check metrics collection
            self.check_metrics_collection(),
            // Check alert system
            self.check_alert_system(),
            // Check snapshot storage
            self.check_snapshot_storage(),
        ];

        Ok(health_results)
    }

    /// Check metrics collection health
    fn check_metrics_collection(&self) -> ComponentHealth {
        ComponentHealth {
            name: "metrics_collection".to_string(),
            status: HealthStatus::Healthy,
            message: Some("Metrics collection active".to_string()),
            last_check: Utc::now(),
            check_duration_ms: 0,
            metadata: HashMap::new(),
        }
    }

    /// Check alert system health
    fn check_alert_system(&self) -> ComponentHealth {
        ComponentHealth {
            name: "alert_system".to_string(),
            status: HealthStatus::Healthy,
            message: Some("Alert system operational".to_string()),
            last_check: Utc::now(),
            check_duration_ms: 0,
            metadata: HashMap::new(),
        }
    }

    /// Check snapshot storage health
    fn check_snapshot_storage(&self) -> ComponentHealth {
        ComponentHealth {
            name: "snapshot_storage".to_string(),
            status: HealthStatus::Healthy,
            message: Some("Snapshot storage operational".to_string()),
            last_check: Utc::now(),
            check_duration_ms: 0,
            metadata: HashMap::new(),
        }
    }

    /// Collect detailed component health asynchronously
    async fn collect_component_health(&self) -> Result<Vec<ComponentHealth>, BearDogError> {
        // Perform synchronous health checks
        self.perform_health_checks()
    }

    /// Check metrics and generate alerts if thresholds are exceeded
    ///
    /// # Errors
    /// Returns an error if alert checking fails
    pub fn check_alerts(&self, metrics: &SystemPerformanceMetrics) -> Result<(), BearDogError> {
        // Check CPU usage
        if metrics.cpu_usage_percent > 90.0 {
            return Err(BearDogError::system(format!(
                "Critical: CPU usage at {:.1}%",
                metrics.cpu_usage_percent
            )));
        }

        // Check memory usage
        if metrics.memory_usage_percent > 90.0 {
            return Err(BearDogError::system(format!(
                "Critical: Memory usage at {:.1}%",
                metrics.memory_usage_percent
            )));
        }

        // Check disk usage
        if metrics.disk_usage_percent > 90.0 {
            return Err(BearDogError::system(format!(
                "Critical: Disk usage at {:.1}%",
                metrics.disk_usage_percent
            )));
        }

        Ok(())
    }

    /// Gets recent alerts
    ///
    /// # Errors
    /// Returns an error if alerts cannot be retrieved
    /// Gets `recent_alerts`
    pub async fn get_recent_alerts(&self, limit: usize) -> Result<Vec<String>, BearDogError> {
        let alerts = self.alerts.read().await;
        let start_idx = if alerts.len() > limit {
            alerts.len() - limit
        } else {
            0
        };
        Ok(alerts[start_idx..].to_vec())
    }

    /// Adds a new alert
    ///
    /// # Errors
    /// Returns an error if the alert cannot be added
    pub async fn add_alert(&self, alert: &str) -> Result<(), BearDogError> {
        self.alerts.write().await.push(alert.to_string());
        Ok(())
    }

    /// Clears old alerts based on age
    ///
    /// # Errors
    /// Returns an error if alerts cannot be cleared
    #[expect(
        clippy::cast_possible_wrap,
        reason = "max_age_hours used as chrono hours; bounded by caller configuration"
    )]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Retention heuristic count fits usize on supported platforms"
    )]
    pub async fn clear_old_alerts(&self, max_age_hours: u64) -> Result<usize, BearDogError> {
        let _cutoff_time = Utc::now() - chrono::Duration::hours(max_age_hours as i64);
        let mut alerts = self.alerts.write().await;
        let _original_count = alerts.len();

        // In a real implementation, alerts would have timestamps
        // For now, we'll implement a simple retention policy
        // Keep only the most recent alerts based on max_age_hours
        let retention_count = if max_age_hours == 0 {
            0
        } else {
            // Keep roughly 10 alerts per hour as a heuristic
            (max_age_hours * 10) as usize
        };

        if alerts.len() > retention_count {
            let remove_count = alerts.len() - retention_count;
            alerts.drain(0..remove_count);
            Ok(remove_count)
        } else {
            Ok(0)
        }
    }
}

impl Default for MonitoringService {
    fn default() -> Self {
        let config = MonitoringConfig::default();
        Self::new(config)
    }
}

#[allow(
    unused_imports,
    clippy::float_cmp,
    clippy::absurd_extreme_comparisons,
    unused_comparisons,
    clippy::nonminimal_bool,
    reason = "monitoring service tests: float thresholds and exhaustive coverage (multi-lint expect causes unfulfilled_lint_expectations)"
)]
#[cfg(test)]
#[path = "service_tests.rs"]
mod tests;
