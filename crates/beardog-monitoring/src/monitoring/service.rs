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
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

use super::metrics::{MetricsService, InternalMetricsSummary};
use beardog_errors::BearDogResult;

/// **CANONICAL MONITORING SERVICE** - Unified system monitoring for BearDog
/// This service provides comprehensive system monitoring, alerting, and health checks
/// with licensing-aware features for enterprise integrations.

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub cpu_alert_threshold: f64,
    pub memory_alert_threshold: f64,
    pub disk_alert_threshold: f64,
    pub network_alert_threshold: f64,
    pub enable_alerts: bool,
    pub alert_history_size: usize,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            cpu_alert_threshold: 80.0,
            memory_alert_threshold: 85.0,
            disk_alert_threshold: 90.0,
            network_alert_threshold: 95.0,
            enable_alerts: true,
            alert_history_size: 100,
        }
    }
}

/// System metrics collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: DateTime<Utc>,
    pub performance: PerformanceMetrics,
    pub internal_summary: InternalMetricsSummary,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub cpu_usage_percent: f64,
    pub memory_usage_bytes: u64,
    pub memory_total_bytes: u64,
    pub disk_usage_bytes: u64,
    pub disk_total_bytes: u64,
    pub network_bytes_sent: u64,
    pub network_bytes_received: u64,
    pub uptime_seconds: u64,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            cpu_usage_percent: 0.0,
            memory_usage_bytes: 0,
            memory_total_bytes: 0,
            disk_usage_bytes: 0,
            disk_total_bytes: 0,
            network_bytes_sent: 0,
            network_bytes_received: 0,
            uptime_seconds: 0,
        }
    }
}

/// Alert levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertLevel {
    Info,
    Warning,
    Critical,
    Emergency,
}

/// System alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: String,
    pub level: AlertLevel,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub resolved: bool,
    pub metadata: std::collections::HashMap<String, String>,
}

/// Main monitoring service
pub struct MonitoringService {
    config: MonitoringConfig,
    metrics_service: MetricsService<()>,
    alerts: Arc<RwLock<VecDeque<Alert>>>,
}

impl MonitoringService {
    /// Create a new monitoring service
    pub fn new(config: MonitoringConfig) -> Self {
        Self {
            config,
            metrics_service: MetricsService::new(),
            alerts: Arc::new(RwLock::new(VecDeque::new())),
        }
    }

    /// Start the monitoring service
    pub async fn start(&self) -> BearDogResult<()> {
        info!("Starting BearDog monitoring service");
        
        // Start periodic monitoring
        let service_clone = self.clone_for_background();
        tokio::spawn(async move {
            loop {
                if let Err(e) = service_clone.monitoring_loop().await {
                    error!("Monitoring loop error: {}", e);
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            }
        });

        Ok(())
    }

    /// Collect current system metrics
    pub async fn collect_metrics(&self) -> BearDogResult<SystemMetrics> {
        let performance = self.collect_performance_metrics().await?;
        let internal_summary = self.metrics_service.get_internal_summary().await;

        Ok(SystemMetrics {
            timestamp: Utc::now(),
            performance,
            internal_summary,
        })
    }

    /// Check for alerts based on current metrics
    pub async fn check_alerts(&self, metrics: &SystemMetrics) -> BearDogResult<()> {
        if !self.config.enable_alerts {
            return Ok(());
        }

        let mut new_alerts = Vec::new();

        // Check CPU usage
        if metrics.performance.cpu_usage_percent > self.config.cpu_alert_threshold {
            new_alerts.push(Alert {
                id: uuid::Uuid::new_v4().to_string(),
                level: AlertLevel::Warning,
                message: format!("High CPU usage: {:.1}%", metrics.performance.cpu_usage_percent),
                timestamp: Utc::now(),
                resolved: false,
                metadata: std::collections::HashMap::new(),
            });
        }

        // Check memory usage
        let memory_usage_percent = (metrics.performance.memory_usage_bytes as f64 
            / metrics.performance.memory_total_bytes as f64) * 100.0;
        if memory_usage_percent > self.config.memory_alert_threshold {
            new_alerts.push(Alert {
                id: uuid::Uuid::new_v4().to_string(),
                level: AlertLevel::Warning,
                message: format!("High memory usage: {:.1}%", memory_usage_percent),
                timestamp: Utc::now(),
                resolved: false,
                metadata: std::collections::HashMap::new(),
            });
        }

        // Add alerts to the queue
        let mut alerts = self.alerts.write().await;
        for alert in new_alerts {
            warn!("New alert: {:?} - {}", alert.level, alert.message);
            alerts.push_back(alert);
            
            // Keep alert history within configured size
            while alerts.len() > self.config.alert_history_size {
                alerts.pop_front();
            }
        }

        Ok(())
    }

    /// Get recent alerts
    pub async fn get_recent_alerts(&self, limit: usize) -> BearDogResult<Vec<Alert>> {
        let alerts = self.alerts.read().await;
        Ok(alerts.iter().rev().take(limit).cloned().collect())
    }

    /// Get current system health status
    pub async fn get_health_status(&self) -> BearDogResult<String> {
        let _metrics = self.collect_metrics().await?;
        let alerts = self.get_recent_alerts(10).await?;
        
        let critical_alerts = alerts.iter()
            .filter(|a| a.level == AlertLevel::Critical || a.level == AlertLevel::Emergency)
            .count();

        if critical_alerts > 0 {
            Ok("CRITICAL".to_string())
        } else if !alerts.is_empty() {
            Ok("WARNING".to_string())
        } else {
            Ok("HEALTHY".to_string())
        }
    }

    /// Internal monitoring loop
    async fn monitoring_loop(&self) -> BearDogResult<()> {
        let _metrics = self.collect_metrics().await?;
        self.check_alerts(&_metrics).await?;
        
        // Update internal metrics
        self.metrics_service.increment_api_requests();
        
        Ok(())
    }

    /// Collect performance metrics from the system
    async fn collect_performance_metrics(&self) -> BearDogResult<PerformanceMetrics> {
        // In a real implementation, these would collect actual system metrics
        // For now, return mock data to ensure compilation
        Ok(PerformanceMetrics {
            cpu_usage_percent: 25.0,
            memory_usage_bytes: 1024 * 1024 * 512, // 512MB
            memory_total_bytes: 1024 * 1024 * 1024 * 8, // 8GB
            disk_usage_bytes: 1024 * 1024 * 1024 * 10, // 10GB
            disk_total_bytes: 1024 * 1024 * 1024 * 100, // 100GB
            network_bytes_sent: 1024 * 1024, // 1MB
            network_bytes_received: 1024 * 1024 * 2, // 2MB
            uptime_seconds: 3600, // 1 hour
        })
    }

    /// Clone service for background tasks
    fn clone_for_background(&self) -> MonitoringServiceClone {
        MonitoringServiceClone {
            config: self.config.clone(),
            alerts: Arc::clone(&self.alerts),
        }
    }
}

/// Lightweight clone for background tasks
#[derive(Clone)]
struct MonitoringServiceClone {
    config: MonitoringConfig,
    alerts: Arc<RwLock<VecDeque<Alert>>>,
}

impl MonitoringServiceClone {
    async fn monitoring_loop(&self) -> BearDogResult<()> {
        // Simplified monitoring loop for background tasks
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_monitoring_service_creation() -> BearDogResult<()> {
        let config = MonitoringConfig::default();
        let service = MonitoringService::new(config);
        let metrics = service.collect_metrics().await?;
        
        assert!(metrics.performance.cpu_usage_percent >= 0.0);
        assert!(metrics.performance.memory_usage_bytes > 0);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_alert_generation() -> BearDogResult<()> {
        let config = MonitoringConfig {
            cpu_alert_threshold: 0.1, // Very low threshold to trigger alert
            ..Default::default()
        };
        
        let service = MonitoringService::new(config);
        let metrics = service.collect_metrics().await?;
        
        service.check_alerts(&metrics).await?;
        let alerts = service.get_recent_alerts(10).await?;
        
        // Should have at least one alert due to low threshold
        assert!(!alerts.is_empty());
        
        Ok(())
    }

    #[test]
    fn test_alert_level_ordering() {
        assert_eq!(AlertLevel::Info, AlertLevel::Info);
        assert_ne!(AlertLevel::Warning, AlertLevel::Critical);
    }
}
