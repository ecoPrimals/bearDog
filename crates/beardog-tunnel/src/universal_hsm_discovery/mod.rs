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


/// Universal HSM Discovery System
///
/// This module provides comprehensive HSM discovery across all platforms
/// and connection types, with intelligent tier-based selection.

use beardog_errors::BearDogResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
// CANONICAL IMPORT: use beardog_types::config::UnifiedDiscoveryConfig;
// Re-export canonical types from tunnel::hsm::types
pub use crate::tunnel::hsm::types::capability::{
    AdvancedFeatureCapabilities, ApiSupportCapabilities, ComplianceCapabilities,
    CryptoOperationCapabilities, HsmCapabilities as UniversalHsmCapabilities,
    HumanEntropyCapabilities, KeyGenerationCapabilities, KeyManagementCapabilities,
    PerformanceCapabilities, SecurityCapabilities, TamperResistanceLevel as TamperResistance,
};
pub use crate::tunnel::hsm::types::{status::HsmHealthStatus, tier::HsmTier};
pub mod capability_detection;
pub mod discovery;
pub mod human_entropy_classifier;
pub mod tier_manager;
/// HSM type classification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HsmType {
    /// Physical hardware HSM
    Hardware,
    /// Software-based HSM
    Software,
    /// Cloud-based HSM service
    Cloud,
    /// Smartphone secure element
    Smartphone,
    /// Network-attached HSM
    NetworkHsm,
}
/// HSM endpoint information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmEndpoint {
    /// Connection URL or address
    pub address: String,
    /// Connection port
    pub port: Option<u16>,
    /// Protocol used for connection
    pub protocol: String,
    /// Whether connection uses TLS
    pub secure: bool,
/// Integration status of discovered HSM
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntegrationStatus {
    /// HSM discovered but not yet integrated
    Discovered,
    /// Integration in progress
    Integrating,
    /// Successfully integrated and available
    Available,
    /// Integration failed
    Failed,
    /// HSM is offline or unreachable
    Offline,
/// Discovered HSM information}


pub struct DiscoveredHsm {
    /// Unique HSM identifier
    pub hsm_id: String,
    /// HSM display name
    pub name: String,
    /// HSM type classification
    pub hsm_type: HsmType,
    /// Connection endpoint information
    pub endpoint: HsmEndpoint,
    /// Comprehensive capability assessment
    pub capabilities: UniversalHsmCapabilities,
    /// Assigned tier based on capabilities
    pub assigned_tier: HsmTier,
    /// Whether human entropy is supported
    pub supports_human_entropy: bool,
    /// Current health status
    pub health_status: HsmHealthStatus,
    /// When this HSM was discovered
    pub discovered_at: chrono::DateTime<chrono::Utc>,
    /// Last health check timestamp
    pub last_health_check: chrono::DateTime<chrono::Utc>,
    /// Integration status
    pub integration_status: IntegrationStatus,
/// Entropy collection method for human entropy
pub enum EntropyCollectionMethod {
    /// Basic touch patterns
    TouchPatterns { pressure_sensitive: bool },
    /// Advanced touch patterns with timing
    TouchPatternsAdvanced { pressure_sensitive: bool },
    /// Biometric variation patterns
    BiometricVariation,
    /// Keyboard timing patterns
    KeyboardTiming,
    /// Device motion sensors
    DeviceMotion,
    /// Environmental sensors
    EnvironmentalSensors { sensor_types: Vec<String> },
/// Universal HSM discovery engine
#[derive(Debug)]
pub struct UniversalHsmDiscovery {
    /// Discovered HSMs by ID
    pub discovered_hsms: Arc<RwLock<HashMap<String, DiscoveredHsm>>>,
    /// Tier manager for HSM classification
    pub tier_manager: Arc<tier_manager::TierManager>,
    /// Human entropy classifier
    pub entropy_classifier: Arc<human_entropy_classifier::HumanEntropyClassifier>,
    /// Discovery configuration
    pub config: DiscoveryConfig,
/// Discovery configuration
// MIGRATED: DiscoveryConfig -> use beardog_types::config::UnifiedDiscoveryConfig;


impl Default for DiscoveryConfig {}


    fn default() -> Self {
        Self {
            enable_cloud_discovery: true,
            enable_pkcs11_discovery: true,
            enable_smartphone_discovery: true,
            discovery_timeout_seconds: 30,
            enable_capability_detection: true,
        }
    }
