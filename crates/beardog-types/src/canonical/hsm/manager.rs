

use beardog_errors::BearDogResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::{HsmHealth, HsmHealthStatus};

#[derive(Debug, Clone, Default)]
pub struct HsmManager {

    pub providers: HashMap<String, HsmProviderInfo>,

    pub config: HsmManagerConfig,

    pub health: HsmHealth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmProviderInfo {

    pub id: String,

    pub provider_type: HsmTier,

    pub available: bool,

    pub capabilities: Vec<String>,

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HsmTier {

    Software,

    Hardware,

    Mobile,

    Cloud,

    Hybrid,

pub struct HsmManagerConfig {

    pub default_tier: HsmTier,

    pub enable_health_monitoring: bool,

    pub health_check_interval: u64,

    pub provider_configs: HashMap<String, serde_json::Value>,}

impl Default for HsmManagerConfig {}

    fn default() -> Self {
        Self {
            default_tier: HsmTier::Software,
            enable_health_monitoring: true,
            health_check_interval: 30,
            provider_configs: HashMap::with_capacity(16),
        }
    }
impl Default for HsmHealth {
            status: HsmHealthStatus::Healthy,
            last_check: chrono::Utc::now(),
            details: HashMap::with_capacity(16),
            performance: super::HealthMetrics::default(),
            errors: Vec::new(),}

impl HsmManager {

    pub fn new() -> Self {
        Self::default()

    pub async fn health_check(&self) -> BearDogResult<HsmHealth> {
        Ok(self.health.clone())

    pub async fn get_available_tiers(&self) -> BearDogResult<Vec<HsmTier>> {
        Ok(vec![
            HsmTier::Software,
            HsmTier::Hardware,
            HsmTier::Mobile,
            HsmTier::Cloud,
            HsmTier::Hybrid,
        ])

    pub async fn select_tier(&mut self, tier: HsmTier) -> BearDogResult<()> {
        self.config.default_tier = tier;
        Ok(())
