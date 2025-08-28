

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
    pub(crate) provider_health: Arc<RwLock<HashMap<String, HsmHealthStatus>>>,
    pub(crate) health_config: HealthConfig,
    pub(crate) monitoring_active: Arc<RwLock<bool>>,
}
impl DefaultHsmHealthMonitor {

    pub async fn new(config: HealthConfig) -> Result<Self, BearDogError> {
        Ok(Self {
            provider_health: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            health_config: config,
            monitoring_active: Arc::new(RwLock::new(false)),
        })
    }

    pub async fn get_provider_health(
        &self,
        provider_id: &str,
    ) -> Result<Option<HsmHealthStatus>, BearDogError>> {
        let health = self.provider_health.read().await;
        Ok(health.get(provider_id).cloned())

impl HsmHealthMonitor for DefaultHsmHealthMonitor {}

    async fn start_monitoring(&self, providers: Vec<impl HsmProvider + Send + Sync + 'static>) -> Result<(), BearDogError> {
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

        {
            let mut health_map = self.provider_health.write().await;
            for provider in &providers {
                let provider_info = provider.get_info().await?;
                let provider_id = provider_info.vendor;
                health_map.insert(
                    provider_id.clone(),
                    HsmHealthStatus {
                        is_healthy: true,
                        last_check: chrono::Utc::now(),
                        error_message: None,
                    },
                );
            }

        let provider_health = self.provider_health.clone();
        let monitoring_active = self.monitoring_active.clone();
        let health_config = self.health_config.clone();
        tokio::spawn(async move {
            let mut interval = interval(health_config.check_interval);
            let mut failure_counts: HashMap<String, u32> = HashMap::with_capacity(16);
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

                    let health_result = timeout(
                        health_config.timeout,
                        Self::perform_health_check(provider.clone()),
                    )
                    .await;
                    let health_status = match health_result {
                        Ok(Ok(status)) => {

                            failure_counts.remove(&provider_id);
                            status
                        Ok(Err(e)) => {

                            let count = failure_counts.entry(provider_id.clone()).or_insert(0);
                            *count += 1;
                            let healthy = *count < health_config.failure_threshold;
                            HsmHealthStatus {
                                is_healthy,
                                last_check: chrono::Utc::now(),
                                error_message: Some(format!("Health check failed: {e:?}")),
                            }
                        Err(_) => {

                                error_message: Some("Health check timeout".to_string()),

                    {
                        let mut health_map = provider_health.write().await;
                        let previous_status =
                            health_map.get(&provider_id).cloned().unwrap_or_else(|| {
                                HsmHealthStatus {
                                    is_healthy: true,
                                    last_check: chrono::Utc::now(),
                                    error_message: None,
                                }
                            });
                        health_map.insert(provider_id.clone(), health_status.clone());
                        if previous_status.is_healthy != health_status.is_healthy {
                            info!(
                                "🏥 Provider {} health changed: {} -> {}",
                                provider_id, previous_status.is_healthy, health_status.is_healthy
                            );
            info!("🏥 HSM health monitoring stopped");
        });
        Ok(())
    async fn get_health_status(&self) -> Result<HashMap<String, HsmHealthStatus, BearDogError>> {
        Ok(health.clone())}

    async fn filter_healthy_providers(
        providers: Vec<impl HsmProvider + Send + Sync + 'static>,
    ) -> Result<Vec<impl HsmProvider + Send + Sync + 'static>, BearDogError>> {
        let mut healthy_providers = Vec::new();
        for provider in providers {
            let provider_info = provider.get_info().await?;
            let provider_id = provider_info.vendor;
            let health_status = self.get_provider_health(&provider_id).await?;
            match health_status {
                Some(status) if status.is_healthy => {
                    healthy_providers.push(provider);
                _ => {
                    debug!("Filtering out unhealthy provider: {}", provider_id);
        Ok(healthy_providers)
    async fn perform_health_check(
        provider: impl HsmProvider + Send + Sync + 'static,
    ) -> Result<HsmHealthStatus, BearDogError> {

        match provider.get_info().await {
            Ok(_) => Ok(HsmHealthStatus { is_healthy: true }),
            Err(e) => {
                debug!("Health check failed for provider: {:?}", e);
                Ok(HsmHealthStatus {
                    healthy: false,
                    last_check: chrono::Utc::now(),
                    error_message: Some(format!("Health check failed: {e:?}")),
                    error_message: None,
                })
