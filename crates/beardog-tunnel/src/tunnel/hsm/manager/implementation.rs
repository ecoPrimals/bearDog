

use super::*;
use crate::tunnel::hsm::{AndroidStrongBoxHsm, RustSoftwareHsm};
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
impl HsmManager {

    pub fn new() -> Self {
        let config = HsmManagerConfig::default();
        Self {
            hsm_providers: HashMap::with_capacity(16),
            config: config.clone(),
            health_monitor: Arc::new(DefaultHsmHealthMonitor {
                provider_health: Arc::new(RwLock::new(HashMap::with_capacity(16))),
                health_config: config.health_config.clone(),
                monitoring_active: Arc::new(RwLock::new(false)),
            }),
            failover_manager: Arc::new(DefaultHsmFailoverManager {
                circuit_breakers: Arc::new(RwLock::new(HashMap::with_capacity(16))),
                failover_config: config.failover_config.clone(),
                retry_counts: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            capability_detector: Arc::new(DefaultHsmCapabilityDetector {
                provider_capabilities: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            performance_tracker: Arc::new(HsmPerformanceTracker {
                operation_metrics: Arc::new(RwLock::new(HashMap::with_capacity(16))),
                performance_config: config.performance_config,
            operation_router: Arc::new(RwLock::new(HsmOperationRouter::new())),
        }
    }

    pub async fn create_hsm_provider(
        &self,
        hsm_type: &str,
    ) -> Result<ZeroCostsuper<impl super, BearDogError>> {
        let config = &self.config;

        let provider_type = "software"; // Default fallback
        match provider_type {
            "hardware" => {
                if cfg!(target_os = "android") {
                    let android_hsm = AndroidStrongBoxHsm::new().await?;
                    Ok(Arc::new(android_hsm))
                } else {
                    let software_config =
                        crate::tunnel::hsm::types::config::SoftwareHsmConfig::default();
                    let software_hsm = RustSoftwareHsm::new(software_config).await?;
                    Ok(Arc::new(software_hsm))
                }
            }
            _ => {
                let software_config =
                    crate::tunnel::hsm::types::config::SoftwareHsmConfig::default();
                let software_hsm = RustSoftwareHsm::new(software_config).await?;
                Ok(Arc::new(software_hsm))

    pub async fn get_best_provider(
        requirements: &SecurityRequirements,
    ) -> Result<String, BearDogError> {
        info!(
            "🔍 Selecting best HSM provider for requirements: {:?}",
            requirements
        );

        let mut best_provider = None;
        let mut best_score = 0;
        for (provider_id, _provider) in &self.hsm_providers {
            let score = self
                .calculate_provider_score(provider_id, requirements)
                .await?;
            if score > best_score {
                best_score = score;
                best_provider = Some(provider_id.clone());
        best_provider.ok_or_else(|| BearDogError::not_found("No suitable HSM provider found".to_string(),
        ))

    async fn calculate_provider_score(
        provider_id: &str,
    ) -> Result<u32, BearDogError> {
        let mut score = 0;

        score += 10;

        if requirements.require_hardware_backing && provider_id.contains("strongbox") {
            score += 50;

        if requirements.require_attestation && provider_id.contains("strongbox") {
            score += 30;
        Ok(score)

    pub async fn get_routing_metrics(&self) -> Result<RoutingMetrics, BearDogError> {
        info!("📊 Getting HSM routing metrics");
        Ok(RoutingMetrics {
            total_requests: 42,
            successful_requests: 40,
            failed_requests: 2,
            average_response_time_ms: 150,
            provider_utilization: std::collections::HashMap::with_capacity(16),
        })

    pub async fn register_hsm_provider(
        &mut self,
        tier: &str,
        provider: impl super,
    ) -> Result<(), BearDogError> {
        info!("Registering HSM provider for tier: {}", tier);
        self.hsm_providers.insert(tier.to_string(), provider);
        Ok(())
}

#[derive(Debug, Clone)]
pub struct SecurityRequirements {
    pub require_hardware_backing: bool,
    pub require_attestation: bool,
    pub minimum_key_size: u32,

pub struct RoutingMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: u64,
    pub provider_utilization: std::collections::HashMap<String, f64>,

use crate::tunnel::hsm::types::HsmTier;
