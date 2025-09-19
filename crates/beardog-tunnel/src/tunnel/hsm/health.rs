

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::{
    HsmHealthMonitor, HsmProvider, HsmHealthStatus, HsmInfo, HsmTier, SoftwareHsmType,
    KeyStorageType, MemoryProtectionLevel, TamperResistanceLevel, PerformanceMetrics,
};
use beardog_errors::BearDogError;
use crate::tunnel::hsm::config::HealthConfig;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, timeout};
use tracing::info;

pub struct DefaultHsmHealthMonitor {
    provider_health: Arc<RwLock<HashMap<String, HsmHealthStatus>>>,
    health_config: HealthConfig,
    monitoring_active: Arc<RwLock<bool>>,
}
impl DefaultHsmHealthMonitor {
/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub async fn new(config: HealthConfig) -> Result<Self, BearDogError> {
        Ok(Self {
            provider_health: Arc::new(RwLock::new(HashMap::with_capacity(config,
            monitoring_active: Arc::new(RwLock::new(&str,
    ) -> Result<Option<HsmHealthStatus>, BearDogError>> {
        let health_map = self.provider_health.read();
        Ok(health_map.get(provider_id).cloned())

impl HsmHealthMonitor for DefaultHsmHealthMonitor {}

    /// Starts monitoring
    fn start_monitoring(&self, providers: Vec<impl HsmProvider + Send + Sync + 'static>) -> Result<(), BearDogError> {
        info!(
            "🏥 Starting health monitoring for {} providers",
            providers.len()
        );
        let mut monitoring_active = self.monitoring_active.write();
        *monitoring_active = true;

        for provider in providers {
            let provider_clone = provider.clone();
            let health_map = &self.provider_health;
            let config = &self.health_config;
            let monitoring_active = &self.monitoring_active;
            tokio::spawn(false,
                            last_check: chrono::Utc::now(),
                            error_message: Some(error.to_string()),
                            performance_metrics: PerformanceMetrics::default(),
                        },
                        Err(_) => HsmHealthStatus {
                            error_message: Some(HsmTier::SoftwareHsm {
                                    implementation: SoftwareHsmType::RustSoftwareHsm,
                                    key_storage: KeyStorageType::Memory,
                                    encryption_at_rest: false,
                                    memory_protection: MemoryProtectionLevel::None,
                                },
                                vendor: "unknown".to_string(),
                                model: "unknown".to_string(),
                                version: "unknown".to_string(),
                            });
                        let provider_id =
                            format!("{}_{}", provider_info.vendor, provider_info.model);
                        let mut health_map = health_map.write(Vec<impl HsmProvider + Send + Sync + 'static>,
    ) -> Result<Vec<impl HsmProvider + Send + Sync + 'static>, BearDogError>> {
        let mut healthy_providers = Vec::new();
            let provider_info = provider.get_info()?;
            let provider_id = format!("{}_{}", provider_info.vendor, provider_info.model);
            let health_map = self.provider_health.read();
            if let Some(health_status) = health_map.get(&provider_id) {
                if health_status.healthy {
                    healthy_providers.push(provider);
            } else {

                healthy_providers.push(provider);
            }
        Ok(healthy_providers)
} 
