//! # HSM Health Monitoring
//!
//! This module provides health monitoring capabilities for HSM providers,
//! including health status tracking and periodic health checks.

use super::config::HealthConfig;
use super::{HsmHealthMonitor, HsmProvider};
use crate::tunnel::hsm::types::{HsmHealthStatus, PerformanceMetrics};
use async_trait::async_trait;
use beardog_errors::BearDogResult;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, timeout};
use tracing::{debug, error, info, warn};

/// Default HSM health monitor
pub struct DefaultHsmHealthMonitor {
    pub(crate) provider_health: Arc<RwLock<HashMap<String, HsmHealthStatus>>>,
    pub(crate) health_config: HealthConfig,
    pub(crate) monitoring_active: Arc<RwLock<bool>>,
}

impl DefaultHsmHealthMonitor {
    /// Create a new HSM health monitor with the specified configuration
    pub async fn new(config: HealthConfig) -> BearDogResult<Self> {
        Ok(Self {
            provider_health: Arc::new(RwLock::new(HashMap::new())),
            health_config: config,
            monitoring_active: Arc::new(RwLock::new(false)),
        })
    }

    /// Get the health status of a specific HSM provider
    pub async fn get_provider_health(
        &self,
        provider_id: &str,
    ) -> BearDogResult<Option<HsmHealthStatus>> {
        let health = self.provider_health.read().await;
        Ok(health.get(provider_id).cloned())
    }
}

#[async_trait]
impl HsmHealthMonitor for DefaultHsmHealthMonitor {
    async fn start_monitoring(&self, providers: Vec<Arc<dyn HsmProvider>>) -> BearDogResult<()> {
        let mut is_active = self.monitoring_active.write().await;
        if *is_active {
            warn!("Health monitoring already active");
            return Ok(());
        }
        *is_active = true;
        drop(is_active);

        info!(
            "🏥 Starting HSM health monitoring for {} providers",
            providers.len()
        );

        // Initialize health status for all providers
        {
            let mut health_map = self.provider_health.write().await;
            for provider in &providers {
                let provider_info = provider.get_info().await?;
                let provider_id = provider_info.vendor;
                health_map.insert(
                    provider_id.clone(),
                    HsmHealthStatus {
                        healthy: true,
                        last_check: chrono::Utc::now(),
                        error_message: None,
                        performance_metrics: PerformanceMetrics::default(),
                    },
                );
            }
        }

        // Start monitoring task
        let provider_health = self.provider_health.clone();
        let monitoring_active = self.monitoring_active.clone();
        let health_config = self.health_config.clone();

        tokio::spawn(async move {
            let mut interval = interval(health_config.check_interval);
            let mut failure_counts: HashMap<String, u32> = HashMap::new();

            loop {
                {
                    let is_active = monitoring_active.read().await;
                    if !*is_active {
                        break;
                    }
                }

                interval.tick().await;

                for provider in &providers {
                    let provider_info = match provider.get_info().await {
                        Ok(info) => info,
                        Err(e) => {
                            error!("Failed to get provider info: {:?}", e);
                            continue;
                        }
                    };
                    let provider_id = provider_info.vendor;

                    // Perform health check with timeout
                    let health_result = timeout(
                        health_config.timeout,
                        Self::perform_health_check(provider.clone()),
                    )
                    .await;

                    let health_status = match health_result {
                        Ok(Ok(status)) => {
                            // Reset failure count on success
                            failure_counts.remove(&provider_id);
                            status
                        }
                        Ok(Err(e)) => {
                            // Increment failure count
                            let count = failure_counts.entry(provider_id.clone()).or_insert(0);
                            *count += 1;

                            let healthy = *count < health_config.failure_threshold;
                            HsmHealthStatus {
                                healthy,
                                last_check: chrono::Utc::now(),
                                error_message: Some(format!("Health check failed: {e:?}")),
                                performance_metrics: PerformanceMetrics::default(),
                            }
                        }
                        Err(_) => {
                            // Timeout
                            let count = failure_counts.entry(provider_id.clone()).or_insert(0);
                            *count += 1;

                            let healthy = *count < health_config.failure_threshold;
                            HsmHealthStatus {
                                healthy,
                                last_check: chrono::Utc::now(),
                                error_message: Some("Health check timeout".to_string()),
                                performance_metrics: PerformanceMetrics::default(),
                            }
                        }
                    };

                    // Update health status
                    {
                        let mut health_map = provider_health.write().await;
                        let previous_status =
                            health_map.get(&provider_id).cloned().unwrap_or_else(|| {
                                HsmHealthStatus {
                                    healthy: true,
                                    last_check: chrono::Utc::now(),
                                    error_message: None,
                                    performance_metrics: PerformanceMetrics::default(),
                                }
                            });
                        health_map.insert(provider_id.clone(), health_status.clone());

                        if previous_status.healthy != health_status.healthy {
                            info!(
                                "🏥 Provider {} health changed: {} -> {}",
                                provider_id, previous_status.healthy, health_status.healthy
                            );
                        }
                    }
                }
            }

            info!("🏥 HSM health monitoring stopped");
        });

        Ok(())
    }

    async fn get_health_status(&self) -> BearDogResult<HashMap<String, HsmHealthStatus>> {
        let health = self.provider_health.read().await;
        Ok(health.clone())
    }

    async fn filter_healthy_providers(
        &self,
        providers: Vec<Arc<dyn HsmProvider>>,
    ) -> BearDogResult<Vec<Arc<dyn HsmProvider>>> {
        let mut healthy_providers = Vec::new();

        for provider in providers {
            let provider_info = provider.get_info().await?;
            let provider_id = provider_info.vendor;

            let health_status = self.get_provider_health(&provider_id).await?;

            match health_status {
                Some(status) if status.healthy => {
                    healthy_providers.push(provider);
                }
                _ => {
                    debug!("Filtering out unhealthy provider: {}", provider_id);
                }
            }
        }

        Ok(healthy_providers)
    }
}

impl DefaultHsmHealthMonitor {
    async fn perform_health_check(
        provider: Arc<dyn HsmProvider>,
    ) -> BearDogResult<HsmHealthStatus> {
        // Simple health check - try to get provider info
        match provider.get_info().await {
            Ok(_) => Ok(HsmHealthStatus {
                healthy: true,
                last_check: chrono::Utc::now(),
                error_message: None,
                performance_metrics: PerformanceMetrics::default(),
            }),
            Err(e) => {
                debug!("Health check failed for provider: {:?}", e);
                Ok(HsmHealthStatus {
                    healthy: false,
                    last_check: chrono::Utc::now(),
                    error_message: Some(format!("Health check failed: {e:?}")),
                    performance_metrics: PerformanceMetrics::default(),
                })
            }
        }
    }
}
