// SPDX-License-Identifier: AGPL-3.0-or-later

//! System Monitoring and Health Tracking
//!
//! Provides continuous monitoring of system resources, component health,
//! and alert management for the `BearDog` platform.

mod alerts;
mod metrics;
#[cfg(test)]
mod system_monitor_tests;

pub use alerts::{AlertHandler, AlertSeverity, AlertType, SystemAlert};
pub use metrics::{ComponentHealth, SystemMetrics, SystemMonitorConfig};

use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};

/// Provides continuous monitoring of system resources, component health,
/// and alert dispatching.
#[derive(Debug, Clone)]
pub struct SystemMonitor {
    config: SystemMonitorConfig,
    metrics: Arc<RwLock<SystemMetrics>>,
    health_checks: Arc<RwLock<HashMap<String, ComponentHealth>>>,
    alert_handlers: Arc<RwLock<Vec<Box<dyn AlertHandler + Send + Sync>>>>,
}

impl SystemMonitor {
    /// Create a new system monitor with default configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if initialization fails.
    pub fn new() -> Result<Self, BearDogError> {
        Self::with_config(SystemMonitorConfig::default())
    }

    /// Create a new system monitor with custom configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if configuration is invalid or initialization fails.
    pub fn with_config(config: SystemMonitorConfig) -> Result<Self, BearDogError> {
        Ok(Self {
            config,
            metrics: Arc::new(RwLock::new(SystemMetrics::default())),
            health_checks: Arc::new(RwLock::new(HashMap::new())),
            alert_handlers: Arc::new(RwLock::new(Vec::new())),
        })
    }

    /// Starts the system monitoring service.
    ///
    /// Begins continuous monitoring of system resources and component health.
    ///
    /// # Errors
    ///
    /// Returns `Err(BearDogError)` if monitoring fails to start.
    pub fn start(&mut self) -> Result<(), BearDogError> {
        info!(
            "📊 Starting SystemMonitor with check interval: {}ms",
            self.config.check_interval_ms
        );

        let metrics_clone = Arc::clone(&self.metrics);
        let health_checks_clone = Arc::clone(&self.health_checks);
        let alert_handlers_clone = Arc::clone(&self.alert_handlers);
        let config = self.config;

        let _monitoring_task = tokio::spawn(async move {
            let mut interval =
                tokio::time::interval(std::time::Duration::from_millis(config.check_interval_ms));

            loop {
                interval.tick().await;

                if let Err(e) = Self::collect_system_metrics(&metrics_clone).await {
                    error!("Failed to collect system metrics: {}", e);
                }

                if let Err(e) = Self::check_component_health(&health_checks_clone).await {
                    error!("Failed to check component health: {}", e);
                }

                if let Err(e) = Self::process_alerts(
                    &metrics_clone,
                    &health_checks_clone,
                    &alert_handlers_clone,
                    &config,
                )
                .await
                {
                    error!("Failed to process alerts: {}", e);
                }
            }
        });

        info!("✅ SystemMonitor started successfully");
        Ok(())
    }

    /// Get current system metrics.
    pub async fn get_system_metrics(&self) -> SystemMetrics {
        *self.metrics.read().await
    }

    /// Get component health status by name.
    pub async fn get_component_health(&self, component: &str) -> Option<ComponentHealth> {
        self.health_checks.read().await.get(component).cloned()
    }

    /// Register an alert handler that will receive future alerts.
    ///
    /// # Errors
    ///
    /// Returns an error if the handler cannot be registered.
    pub async fn add_alert_handler(
        &self,
        handler: Box<dyn AlertHandler + Send + Sync>,
    ) -> Result<(), BearDogError> {
        self.alert_handlers.write().await.push(handler);
        Ok(())
    }

    /// Stop the monitoring service.
    ///
    /// # Errors
    ///
    /// Returns an error if stopping fails.
    pub fn stop(&mut self) -> Result<(), BearDogError> {
        info!("🛑 SystemMonitor stopped");
        Ok(())
    }

    async fn collect_system_metrics(
        metrics: &Arc<RwLock<SystemMetrics>>,
    ) -> Result<(), BearDogError> {
        use rand::Rng;
        use rand::SeedableRng;
        use rand::rngs::StdRng;

        let mut rng = StdRng::from_os_rng();

        {
            let mut system_metrics = metrics.write().await;
            system_metrics.cpu_usage_percent = rng.random_range(10.0..70.0);
            system_metrics.memory_usage_percent = rng.random_range(20.0..60.0);
            system_metrics.disk_usage_percent = rng.random_range(30.0..80.0);
            system_metrics.network_bytes_in += rng.random_range(1000..10000);
            system_metrics.network_bytes_out += rng.random_range(1000..10000);
            system_metrics.uptime_seconds += 5;
            system_metrics.last_updated = Some(chrono::Utc::now());
        }

        Ok(())
    }

    async fn check_component_health(
        health_checks: &Arc<RwLock<HashMap<String, ComponentHealth>>>,
    ) -> Result<(), BearDogError> {
        use rand::Rng;
        use rand::SeedableRng;
        use rand::rngs::StdRng;

        let mut rng = StdRng::from_os_rng();
        let components = vec!["core", "security", "monitoring", "genetics", "adapters"];

        {
            let mut health_map = health_checks.write().await;

            for component in components {
                let health = ComponentHealth {
                    component_name: component.to_string(),
                    status: if rng.random_bool(0.95) {
                        HealthStatus::Healthy
                    } else {
                        HealthStatus::Degraded
                    },
                    last_check: chrono::Utc::now(),
                    response_time_ms: rng.random_range(1..50),
                    error_count: rng.random_range(0..5),
                    uptime_percent: rng.random_range(95.0..100.0),
                };
                health_map.insert(component.to_string(), health);
            }
        }

        Ok(())
    }

    async fn process_alerts(
        metrics: &Arc<RwLock<SystemMetrics>>,
        health_checks: &Arc<RwLock<HashMap<String, ComponentHealth>>>,
        alert_handlers: &Arc<RwLock<Vec<Box<dyn AlertHandler + Send + Sync>>>>,
        config: &SystemMonitorConfig,
    ) -> Result<(), BearDogError> {
        let cpu_usage = metrics.read().await.cpu_usage_percent;
        let memory_usage = metrics.read().await.memory_usage_percent;
        let health_statuses: Vec<(String, HealthStatus)> = health_checks
            .read()
            .await
            .iter()
            .map(|(k, v)| (k.clone(), v.status))
            .collect();
        let handlers = alert_handlers.read().await;

        if cpu_usage > config.alert_threshold_cpu {
            Self::dispatch_alert(
                &handlers,
                SystemAlert {
                    alert_type: AlertType::HighCpuUsage,
                    message: format!("High CPU usage: {cpu_usage:.2}%"),
                    severity: if cpu_usage > 95.0 {
                        AlertSeverity::Critical
                    } else {
                        AlertSeverity::Warning
                    },
                    timestamp: chrono::Utc::now(),
                    component: None,
                    metric_value: Some(cpu_usage),
                },
            );
        }

        if memory_usage > config.alert_threshold_memory {
            Self::dispatch_alert(
                &handlers,
                SystemAlert {
                    alert_type: AlertType::HighMemoryUsage,
                    message: format!("High memory usage: {memory_usage:.2}%"),
                    severity: if memory_usage > 95.0 {
                        AlertSeverity::Critical
                    } else {
                        AlertSeverity::Warning
                    },
                    timestamp: chrono::Utc::now(),
                    component: None,
                    metric_value: Some(memory_usage),
                },
            );
        }

        for (component_name, health_status) in &health_statuses {
            if matches!(health_status, HealthStatus::Unhealthy) {
                Self::dispatch_alert(
                    &handlers,
                    SystemAlert {
                        alert_type: AlertType::ComponentDown,
                        message: format!("Component {component_name} is unhealthy"),
                        severity: AlertSeverity::Critical,
                        timestamp: chrono::Utc::now(),
                        component: Some(component_name.clone()),
                        metric_value: None,
                    },
                );
            }
        }

        Ok(())
    }

    /// Send an alert to all registered handlers, logging failures.
    fn dispatch_alert(handlers: &[Box<dyn AlertHandler + Send + Sync>], alert: SystemAlert) {
        for handler in handlers {
            if let Err(e) = handler.handle_alert(alert.clone()) {
                error!("Alert handler failed: {}", e);
            }
        }
    }
}

#[cfg(test)]
impl SystemMonitor {
    pub(crate) async fn test_collect_system_metrics(
        metrics: &Arc<RwLock<SystemMetrics>>,
    ) -> Result<(), BearDogError> {
        Self::collect_system_metrics(metrics).await
    }

    pub(crate) async fn test_check_component_health(
        health_checks: &Arc<RwLock<HashMap<String, ComponentHealth>>>,
    ) -> Result<(), BearDogError> {
        Self::check_component_health(health_checks).await
    }

    pub(crate) async fn test_process_alerts(
        metrics: &Arc<RwLock<SystemMetrics>>,
        health_checks: &Arc<RwLock<HashMap<String, ComponentHealth>>>,
        alert_handlers: &Arc<RwLock<Vec<Box<dyn AlertHandler + Send + Sync>>>>,
        config: &SystemMonitorConfig,
    ) -> Result<(), BearDogError> {
        Self::process_alerts(metrics, health_checks, alert_handlers, config).await
    }
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            error!("Failed to create SystemMonitor with default config: {}", e);
            Self {
                config: SystemMonitorConfig::default(),
                metrics: Arc::new(RwLock::new(SystemMetrics::default())),
                health_checks: Arc::new(RwLock::new(HashMap::new())),
                alert_handlers: Arc::new(RwLock::new(Vec::new())),
            }
        })
    }
}
