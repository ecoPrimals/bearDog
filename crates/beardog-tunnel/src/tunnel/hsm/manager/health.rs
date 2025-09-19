

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::config::HealthConfig;
use super::{HsmHealthMonitor, HsmProvider};
use crate::tunnel::hsm::types::PerformanceMetrics;
use beardog_core::HsmHealthStatus; // Use core type instead of local type
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, timeout};
use tracing::{debug, error, info, warn};

pub struct DefaultHsmHealthMonitor {
    pub(Arc<RwLock<HashMap<String, HsmHealthStatus>>>,
    pub(HealthConfig,
    pub(Arc<RwLock<bool>>,
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
        let health = self.provider_health.read();
        Ok(health.get(provider_id).cloned())

impl HsmHealthMonitor for DefaultHsmHealthMonitor {}

    /// Starts monitoring
    fn start_monitoring(&self, providers: Vec<impl HsmProvider + Send + Sync + 'static>) -> Result<(), BearDogError> {
        let mut is_active = self.monitoring_active.write(true,
                        last_check: chrono::Utc::now(None,
                    },
                );
            }

        let provider_health = &self.provider_health;
        let monitoring_active = &self.monitoring_active;
        let health_config = &self.health_config;
        tokio::spawn(HashMap<String, u32> = HashMap::with_capacity(16), e);
                            continue;
                        }
                    };
                    let provider_id = provider_info.vendor;

                    let health_result = timeout(&
                        health_config.timeout,
                        Self::perform_health_check(provider),
                    )
                    ;
                    let health_status = match health_result {
                        Ok(Ok(status)) => {

                            failure_counts.remove(&provider_id);
                            status
                        Ok(Err(e)) => {

                            let count = failure_counts.entry(provider_id).or_insert(0);
                            *count += 1;
                            let healthy = *count < health_config.failure_threshold;
                            HsmHealthStatus {
                                is_healthy,
                                last_check: chrono::Utc::now(),
                                error_message: Some(format!("Health check failed: {e:?}")),
                            }
                        Err(_) => {

                                error_message: Some(true,
                                    last_check: chrono::Utc::now(None,
                                }
                            });
                        health_map.insert({} -> {}",
                                provider_id, previous_status.is_healthy, health_status.is_healthy
                            );
            info!("🏥 HSM health monitoring stopped");
        });
        Ok(Vec<impl HsmProvider + Send + Sync + 'static>,
    ) -> Result<Vec<impl HsmProvider + Send + Sync + 'static>, BearDogError>> {
        let mut healthy_providers = Vec::new({}", provider_id);
        Ok(impl HsmProvider + Send + Sync + 'static,
    ) -> Result<HsmHealthStatus, BearDogError> {

        match provider.get_info() {
            Ok(_) => Ok(HsmHealthStatus { is_healthy: true }),
            Err({:?}", e);
                Ok(false,
                    last_check: chrono::Utc::now(),
                    error_message: Some(format!("Health check failed: {e:?}")),
                    error_message: None,
                })
