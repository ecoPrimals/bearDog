//! # HSM Health Monitoring Module
//!
//! This module provides health monitoring functionality for HSM providers, including
//! automated health checks, status tracking, and provider filtering based on health.

use super::{
    HsmHealthMonitor, HsmProvider, HsmHealthStatus, HsmInfo, HsmTier, SoftwareHsmType,
    KeyStorageType, MemoryProtectionLevel, TamperResistanceLevel, PerformanceMetrics,
};
use crate::error::{BearDogError, BearDogResult};
use crate::tunnel::hsm::config::HealthConfig;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, timeout};
use tracing::info;

/// Default HSM health monitor implementation
pub struct DefaultHsmHealthMonitor {
    provider_health: Arc<RwLock<HashMap<String, HsmHealthStatus>>>,
    health_config: HealthConfig,
    monitoring_active: Arc<RwLock<bool>>,
}

impl DefaultHsmHealthMonitor {
    pub async fn new(config: HealthConfig) -> BearDogResult<Self> {
        Ok(Self {
            provider_health: Arc::new(RwLock::new(HashMap::new())),
            health_config: config,
            monitoring_active: Arc::new(RwLock::new(false)),
        })
    }

    pub async fn get_provider_health(
        &self,
        provider_id: &str,
    ) -> BearDogResult<Option<HsmHealthStatus>> {
        let health_map = self.provider_health.read().await;
        Ok(health_map.get(provider_id).cloned())
    }
}

#[async_trait]
impl HsmHealthMonitor for DefaultHsmHealthMonitor {
    async fn start_monitoring(&self, providers: Vec<Arc<dyn HsmProvider>>) -> BearDogResult<()> {
        info!(
            "🏥 Starting health monitoring for {} providers",
            providers.len()
        );

        let mut monitoring_active = self.monitoring_active.write().await;
        *monitoring_active = true;

        // Start health check tasks for each provider
        for provider in providers {
            let provider_clone = provider.clone();
            let health_map = self.provider_health.clone();
            let config = self.health_config.clone();
            let monitoring_active = self.monitoring_active.clone();

            tokio::spawn(async move {
                let mut check_interval = interval(config.check_interval);

                loop {
                    check_interval.tick().await;

                    // Check if monitoring is still active
                    {
                        let active = monitoring_active.read().await;
                        if !*active {
                            break;
                        }
                    }

                    // Perform health check
                    let health_result =
                        timeout(config.timeout, provider_clone.health_check()).await;

                    let health_status = match health_result {
                        Ok(Ok(status)) => status,
                        Ok(Err(error)) => HsmHealthStatus {
                            healthy: false,
                            last_check: chrono::Utc::now(),
                            error_message: Some(error.to_string()),
                            performance_metrics: PerformanceMetrics::default(),
                        },
                        Err(_) => HsmHealthStatus {
                            healthy: false,
                            last_check: chrono::Utc::now(),
                            error_message: Some("Health check timed out".to_string()),
                            performance_metrics: PerformanceMetrics::default(),
                        },
                    };

                    // Update health status
                    {
                        let provider_info =
                            provider_clone.get_info().await.unwrap_or_else(|_| HsmInfo {
                                hsm_type: HsmTier::SoftwareHsm {
                                    implementation: SoftwareHsmType::RustSoftwareHsm,
                                    key_storage: KeyStorageType::Memory,
                                    encryption_at_rest: false,
                                    memory_protection: MemoryProtectionLevel::None,
                                },
                                vendor: "unknown".to_string(),
                                model: "unknown".to_string(),
                                version: "unknown".to_string(),
                                capabilities: vec![],
                                supported_algorithms: vec![],
                                max_key_size: None,
                                certification: None,
                                tamper_resistance: TamperResistanceLevel::None,
                            });

                        let provider_id =
                            format!("{}_{}", provider_info.vendor, provider_info.model);

                        let mut health_map = health_map.write().await;
                        health_map.insert(provider_id, health_status);
                    }
                }
            });
        }

        Ok(())
    }

    async fn get_health_status(&self) -> BearDogResult<HashMap<String, HsmHealthStatus>> {
        let health_map = self.provider_health.read().await;
        Ok(health_map.clone())
    }

    async fn filter_healthy_providers(
        &self,
        providers: Vec<Arc<dyn HsmProvider>>,
    ) -> BearDogResult<Vec<Arc<dyn HsmProvider>>> {
        let mut healthy_providers = Vec::new();

        for provider in providers {
            let provider_info = provider.get_info().await?;
            let provider_id = format!("{}_{}", provider_info.vendor, provider_info.model);

            let health_map = self.provider_health.read().await;
            if let Some(health_status) = health_map.get(&provider_id) {
                if health_status.healthy {
                    healthy_providers.push(provider);
                }
            } else {
                // If we don't have health info, assume healthy
                healthy_providers.push(provider);
            }
        }

        Ok(healthy_providers)
    }
} 