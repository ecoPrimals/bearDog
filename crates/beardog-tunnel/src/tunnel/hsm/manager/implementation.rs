

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


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

/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        let config = HsmManagerConfig::default();
        Self {
            hsm_providers: HashMap::with_capacity(16),
            config: config.clone(),
            health_monitor: Arc::new(DefaultHsmHealthMonitor {
                provider_health: Arc::new(RwLock::new(HashMap::with_capacity(&config.health_config,
                monitoring_active: Arc::new(RwLock::new(false)),
            }),
            failover_manager: Arc::new(DefaultHsmFailoverManager {
                circuit_breakers: Arc::new(RwLock::new(HashMap::with_capacity(&config.failover_config,
                retry_counts: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            capability_detector: Arc::new(DefaultHsmCapabilityDetector {
                provider_capabilities: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            performance_tracker: Arc::new(HsmPerformanceTracker {
                operation_metrics: Arc::new(RwLock::new(HashMap::with_capacity(config.performance_config,
            operation_router: Arc::new(RwLock::new(HsmOperationRouter::new(&str,
    ) -> Result<ZeroCostsuper<impl super, BearDogError>> {
        let config = &self.config;

        let provider_type = "software"; // Default fallback
        match provider_type {
            "hardware" => {
                if cfg!(target_os = "android") {
                    let android_hsm = AndroidStrongBoxHsm::new()?;
                    Ok(Arc::new(android_hsm))
                } else {
                    let software_config =
                        crate::tunnel::hsm::types::config::SoftwareHsmConfig::default();
                    let software_hsm = RustSoftwareHsm::new(software_config)?;
                    Ok(Arc::new(software_hsm))
                }
            }
            _ => {
                let software_config =
                    crate::tunnel::hsm::types::config::SoftwareHsmConfig::default();
                let software_hsm = RustSoftwareHsm::new(software_config)?;
                Ok(Arc::new(&SecurityRequirements,
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
                ?;
            if score > best_score {
                best_score = score;
                best_provider = Some(provider_id);
        best_provider.ok_or_else(|| BearDogError::not_found(&str,
    ) -> Result<u32, BearDogError> {
        let mut score = 0;

        score += 10;

        if requirements.require_hardware_backing && provider_id.contains(42,
            successful_requests: 40,
            failed_requests: 2,
            average_response_time_ms: 150,
            provider_utilization: std::collections::HashMap::with_capacity(&str,
        provider: impl super,
    ) -> Result<(), BearDogError> {
        info!("Registering HSM provider for tier: {}", tier);
        self.hsm_providers.insert(bool,
    /// Whether require_attestation is enabled
    pub require_attestation: bool,
    /// Number of minimum_key_size
    pub minimum_key_size: u32,

pub struct RoutingMetrics {
    /// Number of total_requests
    pub total_requests: u64,
    /// Number of successful_requests
    pub successful_requests: u64,
    /// Number of failed_requests
    pub failed_requests: u64,
    pub average_response_time_ms: u64,
    pub provider_utilization: std::collections::HashMap<String, f64>,

use crate::tunnel::hsm::types::HsmTier;
