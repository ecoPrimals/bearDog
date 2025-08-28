

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HsmType {

    Hardware,

    Software,

    Cloud,

    Smartphone,

    NetworkHsm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmEndpoint {

    pub address: String,

    pub port: Option<u16>,

    pub protocol: String,

    pub secure: bool,

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntegrationStatus {

    Discovered,

    Integrating,

    Available,

    Failed,

    Offline,

pub struct DiscoveredHsm {

    pub hsm_id: String,

    pub name: String,

    pub hsm_type: HsmType,

    pub endpoint: HsmEndpoint,

    pub capabilities: UniversalHsmCapabilities,

    pub assigned_tier: HsmTier,

    pub supports_human_entropy: bool,

    pub health_status: HsmHealthStatus,

    pub discovered_at: chrono::DateTime<chrono::Utc>,

    pub last_health_check: chrono::DateTime<chrono::Utc>,

    pub integration_status: IntegrationStatus,

pub enum EntropyCollectionMethod {

    TouchPatterns { pressure_sensitive: bool },

    TouchPatternsAdvanced { pressure_sensitive: bool },

    BiometricVariation,

    KeyboardTiming,

    DeviceMotion,

    EnvironmentalSensors { sensor_types: Vec<String> },

#[derive(Debug)]
pub struct UniversalHsmDiscovery {

    pub discovered_hsms: Arc<RwLock<HashMap<String, DiscoveredHsm>>>,

    pub tier_manager: Arc<tier_manager::TierManager>,

    pub entropy_classifier: Arc<human_entropy_classifier::HumanEntropyClassifier>,

    pub config: DiscoveryConfig,

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
