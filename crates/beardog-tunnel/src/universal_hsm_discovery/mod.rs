

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


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

#[derive(Debug, Clone)]
    /// Optional port
    pub port: Option<u16>,

    /// The protocol value
    pub protocol: String,

    /// Whether secure is enabled
    pub secure: bool,
}

#[derive(Debug, Clone)]
    TouchPatternsAdvanced { pressure_sensitive: bool },


    BiometricVariation,


    KeyboardTiming,


    DeviceMotion,

    EnvironmentalSensors { sensor_types: Vec<String> },
}

#[derive(Debug, Clone)]
    /// Name of the item
    pub name: String,

    /// The hsm type value
    pub hsm_type: HsmType,

    /// The endpoint value
    pub endpoint: HsmEndpoint,

    /// The capabilities value
    pub capabilities: UniversalHsmCapabilities,

    /// The assigned tier value
    pub assigned_tier: HsmTier,

    /// Whether supports_human_entropy is enabled
    pub supports_human_entropy: bool,

    /// Current status of the health
    pub health_status: HsmHealthStatus,

    /// The discovered at value
    pub discovered_at: chrono::DateTime<chrono::Utc>,

    /// The last health check value
    pub last_health_check: chrono::DateTime<chrono::Utc>,

    /// Current status of the integration
    pub integration_status: IntegrationStatus,
}

#[derive(Arc<RwLock<HashMap<String, DiscoveredHsm>>>,
    /// The tier manager value
    pub tier_manager: Arc<tier_manager::TierManager>,
    /// The entropy classifier value
    pub entropy_classifier: Arc<human_entropy_classifier::HumanEntropyClassifier>,
    pub config: DiscoveryConfig,
}

impl Default for DiscoveryConfig {
    fn default(true,
            enable_pkcs11_discovery: true,
            enable_smartphone_discovery: true,
            discovery_timeout_seconds: 30,
            enable_capability_detection: true,
        }
    }
}
