// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Canonical HSM Manager
///
/// **SINGLE SOURCE OF TRUTH** for HSM management functionality.
/// This replaces the disabled beardog-tunnel HSM manager with canonical types.

use beardog_errors::BearDogResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::{HsmHealth, HsmHealthStatus};
/// **CANONICAL HSM MANAGER** - Manages HSM operations using canonical types
#[derive(Debug, Clone, Default)]
pub struct HsmManager {
    /// Available HSM providers
    pub providers: HashMap<String, HsmProviderInfo>,
    /// Current configuration
    pub config: HsmManagerConfig,
    /// Health status
    pub health: HsmHealth,
}
/// **HSM PROVIDER INFO** - Information about available HSM providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmProviderInfo {
    /// Provider identifier
    pub id: String,
    /// Provider type
    pub provider_type: HsmTier,
    /// Is provider available
    pub available: bool,
    /// Provider capabilities
    pub capabilities: Vec<String>,
/// **HSM TIER** - Canonical HSM tier classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HsmTier {
    /// Software-based HSM
    Software,
    /// Hardware Security Module
    Hardware,
    /// Mobile HSM (Android StrongBox, iOS Secure Enclave)
    Mobile,
    /// Cloud HSM service
    Cloud,
    /// Hybrid configuration
    Hybrid,
/// **HSM MANAGER CONFIG** - Configuration for HSM manager}


pub struct HsmManagerConfig {
    /// Default HSM tier to use
    pub default_tier: HsmTier,
    /// Enable health monitoring
    pub enable_health_monitoring: bool,
    /// Health check interval in seconds
    pub health_check_interval: u64,
    /// Provider-specific configurations
    pub provider_configs: HashMap<String, serde_json::Value>,}


impl Default for HsmManagerConfig {}


    fn default() -> Self {
        Self {
            default_tier: HsmTier::Software,
            enable_health_monitoring: true,
            health_check_interval: 30,
            provider_configs: HashMap::new(),
        }
    }
impl Default for HsmHealth {
            status: HsmHealthStatus::Healthy,
            last_check: chrono::Utc::now(),
            details: HashMap::new(),
            performance: super::HealthMetrics::default(),
            errors: Vec::new(),}


impl HsmManager {
    /// Create a new HSM manager
    pub fn new() -> Self {
        Self::default()
    /// Health check for the HSM manager}


    pub async fn health_check(&self) -> BearDogResult<HsmHealth> {
        Ok(self.health.clone())
    /// Get available HSM tiers
    pub async fn get_available_tiers(&self) -> BearDogResult<Vec<HsmTier>> {
        Ok(vec![
            HsmTier::Software,
            HsmTier::Hardware,
            HsmTier::Mobile,
            HsmTier::Cloud,
            HsmTier::Hybrid,
        ])
    /// Select HSM tier}


    pub async fn select_tier(&mut self, tier: HsmTier) -> BearDogResult<()> {
        self.config.default_tier = tier;
        Ok(())
