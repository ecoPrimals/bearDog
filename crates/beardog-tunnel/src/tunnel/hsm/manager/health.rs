// SPDX-License-Identifier: AGPL-3.0-or-later

//! HSM Health Monitoring
//!
//! Health monitoring and status management for HSM providers.

use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// HSM health status
#[derive(Debug, Clone)]
pub struct HsmHealthStatus {
    /// Whether the HSM is healthy
    pub is_healthy: bool,
    /// Last check timestamp
    pub last_check: chrono::DateTime<chrono::Utc>,
    /// Error message if unhealthy
    pub error_message: Option<String>,
}

impl HsmHealthStatus {
    /// Create a healthy status
    #[must_use]
    pub fn healthy() -> Self {
        Self {
            is_healthy: true,
            last_check: chrono::Utc::now(),
            error_message: None,
        }
    }
    /// Create an unhealthy status with error
    #[must_use]
    pub fn unhealthy(error: String) -> Self {
        Self {
            is_healthy: false,
            last_check: chrono::Utc::now(),
            error_message: Some(error),
        }
    }
}

/// Health monitor for HSM providers
pub struct HealthMonitor {
    /// Health status cache
    health_cache: Arc<RwLock<HashMap<String, HsmHealthStatus>>>,
    /// Health check interval
    check_interval: Duration,
    /// Running flag
    running: Arc<RwLock<bool>>,
}

impl HealthMonitor {
    /// Create a new health monitor
    #[must_use]
    pub fn new(check_interval: Duration) -> Self {
        Self {
            health_cache: Arc::new(RwLock::new(HashMap::new())),
            check_interval,
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// # Errors
    ///
    /// Returns an error if the Tor-related operation fails.
    /// Start health monitoring
    pub async fn start_monitoring(&self) -> Result<(), BearDogError> {
        let mut running = self.running.write().await;
        if *running {
            return Err(BearDogError::configuration(
                "Health monitoring already running",
            ));
        }
        *running = true;
        drop(running);

        let health_cache = self.health_cache.clone();
        let check_interval = self.check_interval;
        let running_flag = self.running.clone();

        tokio::spawn(async move {
            info!("🏥 HSM health monitoring started");

            // Modern: Use tokio interval instead of manual sleep loop
            let mut interval = tokio::time::interval(check_interval);
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            while *running_flag.read().await {
                interval.tick().await;

                let mut health_map = health_cache.write().await;
                let now = chrono::Utc::now();
                for (provider_id, status) in health_map.iter_mut() {
                    debug!("Health check for provider: {}", provider_id);
                    status.last_check = now;
                }
            }

            info!("🏥 HSM health monitoring stopped");
        });

        Ok(())
    }

    /// Stop health monitoring
    pub async fn stop_monitoring(&self) {
        let mut running = self.running.write().await;
        *running = false;
        info!("🛑 Stopping HSM health monitoring");
    }

    /// Get health status for a provider
    pub async fn get_health_status(&self, provider_id: &str) -> Option<HsmHealthStatus> {
        let cache = self.health_cache.read().await;
        cache.get(provider_id).cloned()
    }

    /// Update health status for a provider
    pub async fn update_health_status(&self, provider_id: String, status: HsmHealthStatus) {
        let mut cache = self.health_cache.write().await;

        if let Some(previous_status) = cache.get(&provider_id)
            && previous_status.is_healthy != status.is_healthy
        {
            if status.is_healthy {
                info!("✅ Provider {} recovered", provider_id);
            } else {
                warn!("❌ Provider {} became unhealthy", provider_id);
            }
        }

        cache.insert(provider_id, status);
    }

    /// Get all health statuses
    pub async fn get_all_health_statuses(&self) -> HashMap<String, HsmHealthStatus> {
        self.health_cache.read().await.clone()
    }

    /// Check if provider is healthy
    pub async fn is_healthy(&self, provider_id: &str) -> bool {
        self.get_health_status(provider_id)
            .await
            .is_some_and(|s| s.is_healthy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_healthy() -> Result<(), Box<dyn std::error::Error>> {
        let status = HsmHealthStatus::healthy();
        assert!(status.is_healthy);
        assert!(status.error_message.is_none());
        Ok(())
    }

    #[test]
    fn test_health_status_unhealthy() -> Result<(), Box<dyn std::error::Error>> {
        let status = HsmHealthStatus::unhealthy("Test error".to_string());
        assert!(!status.is_healthy);
        assert_eq!(status.error_message, Some("Test error".to_string()));
        Ok(())
    }

    #[tokio::test]
    async fn test_health_monitor_creation() -> Result<(), Box<dyn std::error::Error>> {
        let monitor = HealthMonitor::new(Duration::from_secs(30));
        let statuses = monitor.get_all_health_statuses().await;
        assert!(statuses.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_update_health_status() -> Result<(), Box<dyn std::error::Error>> {
        let monitor = HealthMonitor::new(Duration::from_secs(30));

        monitor
            .update_health_status("test-provider".to_string(), HsmHealthStatus::healthy())
            .await;

        let status = monitor.get_health_status("test-provider").await;
        assert!(status.is_some());
        assert!(status.ok_or("status not found")?.is_healthy);
        Ok(())
    }

    #[tokio::test]
    async fn test_is_healthy() -> Result<(), Box<dyn std::error::Error>> {
        let monitor = HealthMonitor::new(Duration::from_secs(30));

        monitor
            .update_health_status("provider-1".to_string(), HsmHealthStatus::healthy())
            .await;

        assert!(monitor.is_healthy("provider-1").await);
        assert!(!monitor.is_healthy("nonexistent").await);
        Ok(())
    }

    #[tokio::test]
    async fn test_health_status_transition() -> Result<(), Box<dyn std::error::Error>> {
        let monitor = HealthMonitor::new(Duration::from_secs(30));

        // Start healthy
        monitor
            .update_health_status("provider-1".to_string(), HsmHealthStatus::healthy())
            .await;

        assert!(monitor.is_healthy("provider-1").await);

        // Become unhealthy
        monitor
            .update_health_status(
                "provider-1".to_string(),
                HsmHealthStatus::unhealthy("Connection lost".to_string()),
            )
            .await;

        assert!(!monitor.is_healthy("provider-1").await);

        let status = monitor
            .get_health_status("provider-1")
            .await
            .ok_or("status not found")?;
        assert_eq!(status.error_message, Some("Connection lost".to_string()));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_all_health_statuses() -> Result<(), Box<dyn std::error::Error>> {
        let monitor = HealthMonitor::new(Duration::from_secs(30));

        monitor
            .update_health_status("p1".to_string(), HsmHealthStatus::healthy())
            .await;
        monitor
            .update_health_status(
                "p2".to_string(),
                HsmHealthStatus::unhealthy("Error".to_string()),
            )
            .await;

        let all_statuses = monitor.get_all_health_statuses().await;
        assert_eq!(all_statuses.len(), 2);
        Ok(())
    }

    #[tokio::test]
    async fn test_start_stop_monitoring() -> Result<(), Box<dyn std::error::Error>> {
        let monitor = HealthMonitor::new(Duration::from_millis(100));

        let result = monitor.start_monitoring().await;
        assert!(result.is_ok());

        // ✅ EVOLVED: Use actual check interval instead of arbitrary duration
        // Modern concurrent pattern - synchronize on semantic duration
        tokio::time::sleep(monitor.check_interval).await;

        monitor.stop_monitoring().await;

        // Verify monitoring stopped (no arbitrary wait needed)
        assert!(
            !*monitor.running.read().await,
            "Monitoring should be stopped"
        );
        Ok(())
    }
}
