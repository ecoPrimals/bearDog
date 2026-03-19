// SPDX-License-Identifier: AGPL-3.0-only



use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::{HsmHealth, HsmHealthStatus};

#[derive(HashMap<String, HsmProviderInfo>,

    /// Config
    pub config: HsmManagerConfig,

    /// Health
    /// The health value
    pub health: HsmHealth,
}

#[derive(Debug, Clone)]
    /// Provider Type
    pub provider_type: HsmTier,

    /// Available
    /// Whether available is enabled
    pub available: bool,

    /// Capabilities
    /// Collection of capabilities
    pub capabilities: Vec<String>,

#[derive(Debug, Clone)]
    /// Enable Health Monitoring
    /// Whether enable_health_monitoring is enabled
    pub enable_health_monitoring: bool,

    /// Health Check Interval
    /// Number of health_check_interval
    pub health_check_interval: u64,

    /// Provider Configs
    pub provider_configs: HashMap<String, serde_json::Value>,}

impl Default for HsmManagerConfig {}

    fn default(HsmTier::Software,
            enable_health_monitoring: true,
            health_check_interval: 30,
            provider_configs: HashMap::with_capacity(HsmHealthStatus::Healthy,
            last_check: chrono::Utc::now(),
            details: HashMap::with_capacity(16),
            performance: super::HealthMetrics::default(),
            errors: Vec::new(),}

impl HsmManager {

/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()

/// Health Check operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn health_check(&self) -> impl std::future::Future<Output = Result<HsmHealth, BearDogError>> + Send {
        async move {
        Ok(self.health)

/// Get Available Tiers operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets available_tiers
    /// Gets available_tiers
    pub fn get_available_tiers(&self) -> Result<Vec<HsmTier>, BearDogError>> {
        Ok(vec![
            HsmTier::Software,
            HsmTier::Hardware,
            HsmTier::Mobile,
            HsmTier::Cloud,
            HsmTier::Hybrid,
        ])

/// Select Tier operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn select_tier(&mut self, tier: HsmTier) -> Result<(), BearDogError> {
        self.config.default_tier = tier;
        Ok(())
