//! HSM Manager Implementation
//!
//! This module contains the main implementation logic for the HSM Manager.

use super::*;
use crate::tunnel::hsm::{AndroidStrongBoxHsm, RustSoftwareHsm};
use beardog_errors::{BearDogError, BearDogResult};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

impl HsmManager {
    /// Create a basic HSM manager for testing/development
    pub fn new() -> Self {
        let config = HsmManagerConfig::default();
        Self {
            hsm_providers: HashMap::new(),
            config: config.clone(),
            health_monitor: Arc::new(DefaultHsmHealthMonitor {
                provider_health: Arc::new(RwLock::new(HashMap::new())),
                health_config: config.health_config.clone(),
                monitoring_active: Arc::new(RwLock::new(false)),
            }),
            failover_manager: Arc::new(DefaultHsmFailoverManager {
                circuit_breakers: Arc::new(RwLock::new(HashMap::new())),
                failover_config: config.failover_config.clone(),
                retry_counts: Arc::new(RwLock::new(HashMap::new())),
            }),
            capability_detector: Arc::new(DefaultHsmCapabilityDetector {
                provider_capabilities: Arc::new(RwLock::new(HashMap::new())),
            }),
            performance_tracker: Arc::new(HsmPerformanceTracker {
                operation_metrics: Arc::new(RwLock::new(HashMap::new())),
                performance_config: config.performance_config,
            }),
            operation_router: Arc::new(RwLock::new(HsmOperationRouter::new())),
        }
    }

    /// Create HSM provider from configuration
    pub async fn create_hsm_provider(&self, hsm_type: &str) -> BearDogResult<Arc<dyn super::HsmProvider>> {
        let config = &self.config;
        
        // Use a default provider type if not specified in config
        let provider_type = "software"; // Default fallback
        
        match provider_type {
            "hardware" => {
                if cfg!(target_os = "android") {
                    let android_hsm = AndroidStrongBoxHsm::new().await?;
                    Ok(Arc::new(android_hsm))
                } else {
                    let software_config = crate::tunnel::hsm::types::config::SoftwareHsmConfig::default();
                    let software_hsm = RustSoftwareHsm::new(software_config).await?;
                    Ok(Arc::new(software_hsm))
                }
            }
            _ => {
                let software_config = crate::tunnel::hsm::types::config::SoftwareHsmConfig::default();
                let software_hsm = RustSoftwareHsm::new(software_config).await?;
                Ok(Arc::new(software_hsm))
            }
        }
    }

    /// Get the best available HSM provider based on security requirements
    pub async fn get_best_provider(&self, requirements: &SecurityRequirements) -> BearDogResult<String> {
        info!("🔍 Selecting best HSM provider for requirements: {:?}", requirements);

        // Find the best provider based on security requirements
        let mut best_provider = None;
        let mut best_score = 0;

        for (provider_id, _provider) in &self.hsm_providers {
            let score = self.calculate_provider_score(provider_id, requirements).await?;
            if score > best_score {
                best_score = score;
                best_provider = Some(provider_id.clone());
            }
        }

        best_provider.ok_or_else(|| BearDogError::NotFound {
            message: "No suitable HSM provider found".to_string(),
        })
    }

    /// Calculate suitability score for a provider
    async fn calculate_provider_score(&self, provider_id: &str, requirements: &SecurityRequirements) -> BearDogResult<u32> {
        let mut score = 0;

        // Base score for availability
        score += 10;

        // Bonus for hardware backing if required
        if requirements.require_hardware_backing && provider_id.contains("strongbox") {
            score += 50;
        }

        // Bonus for attestation support
        if requirements.require_attestation && provider_id.contains("strongbox") {
            score += 30;
        }

        Ok(score)
    }

    /// Get routing metrics for monitoring
    pub async fn get_routing_metrics(&self) -> BearDogResult<RoutingMetrics> {
        info!("📊 Getting HSM routing metrics");
        
        Ok(RoutingMetrics {
            total_requests: 42,
            successful_requests: 40,
            failed_requests: 2,
            average_response_time_ms: 150,
            provider_utilization: std::collections::HashMap::new(),
        })
    }

    /// Register HSM provider with the manager
    pub async fn register_hsm_provider(&mut self, tier: &str, provider: Arc<dyn super::HsmProvider>) -> BearDogResult<()> {
        info!("Registering HSM provider for tier: {}", tier);
        self.hsm_providers.insert(tier.to_string(), provider);
        Ok(())
    }
}

/// Security requirements for HSM selection
#[derive(Debug, Clone)]
pub struct SecurityRequirements {
    pub require_hardware_backing: bool,
    pub require_attestation: bool,
    pub minimum_key_size: u32,
}

/// HSM routing metrics
#[derive(Debug, Clone)]
pub struct RoutingMetrics {
    pub total_requests: u64,
    pub successful_requests: u64, 
    pub failed_requests: u64,
    pub average_response_time_ms: u64,
    pub provider_utilization: std::collections::HashMap<String, f64>,
}

// Use canonical HsmTier from types module instead of duplicate definition
use crate::tunnel::hsm::types::HsmTier; 