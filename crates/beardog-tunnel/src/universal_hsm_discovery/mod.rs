//! Universal HSM Discovery Module
//!
//! Provides functionality for discovering and classifying Hardware Security Modules
//! across different platforms and providers.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

// Re-export capability types
pub use crate::tunnel::hsm::types::capability::{
    AdvancedFeatureCapabilities, ApiSupportCapabilities, ComplianceCapabilities,
    CryptoOperationCapabilities, HsmCapabilities as UniversalHsmCapabilities,
    HumanEntropyCapabilities, KeyGenerationCapabilities, KeyManagementCapabilities,
    PerformanceCapabilities, SecurityCapabilities, TamperResistanceLevel as TamperResistance,
};
pub use crate::tunnel::hsm::types::{status::HsmHealthStatus, tier::HsmTier};

// Submodules
pub mod capability_detection;
pub mod discovery;
pub mod human_entropy_classifier;
pub mod tier_manager;

#[cfg(test)]
mod comprehensive_core_tests;

#[cfg(test)]
mod edge_cases_and_fault_tests;

#[cfg(test)]
mod capability_detection_comprehensive_tests;

#[cfg(test)]
mod universal_adapter_comprehensive_tests;

#[cfg(test)]
mod health_monitoring_comprehensive_tests;

#[cfg(test)]
mod configuration_management_comprehensive_tests;

#[cfg(test)]
mod workflow_integration_comprehensive_tests;

#[cfg(test)]
mod chaos_engineering_comprehensive_tests;

#[cfg(test)]
mod e2e_scenarios_comprehensive_tests;

#[cfg(test)]
mod property_based_comprehensive_tests;

#[cfg(test)]
mod performance_benchmarks_comprehensive_tests;

/// HSM endpoint information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmEndpoint {
    /// Hostname or address
    pub host: String,
    /// Optional port
    pub port: Option<u16>,
    /// Protocol
    pub protocol: String,
    /// Whether secure connection is required
    pub secure: bool,
}

/// Human entropy methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HumanEntropyMethod {
    /// Touch patterns with advanced pressure sensitivity
    TouchPatternsAdvanced { pressure_sensitive: bool },
    /// Biometric variation
    BiometricVariation,
    /// Keyboard timing
    KeyboardTiming,
    /// Device motion
    DeviceMotion,
    /// Environmental sensors
    EnvironmentalSensors { sensor_types: Vec<String> },
}

/// HSM type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HsmType {
    /// Hardware HSM
    Hardware,
    /// Software HSM
    Software,
    /// Cloud HSM
    Cloud,
    /// Smartphone (iOS/Android)
    Smartphone,
    /// TPM (Trusted Platform Module)
    Tpm,
    /// PKCS#11 provider
    Pkcs11,
}

/// Integration status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntegrationStatus {
    /// Discovered but not configured
    Discovered,
    /// Currently configuring
    Configuring,
    /// Ready for use
    Ready,
    /// Integration failed
    Failed,
    /// Disabled
    Disabled,
}

/// Discovered HSM information
#[derive(Debug, Clone)]
pub struct DiscoveredHsm {
    /// Unique identifier
    pub name: String,
    /// HSM type
    pub hsm_type: HsmType,
    /// Connection endpoint
    pub endpoint: HsmEndpoint,
    /// Detected capabilities
    pub capabilities: UniversalHsmCapabilities,
    /// Assigned tier
    pub assigned_tier: HsmTier,
    /// Whether supports human entropy
    pub supports_human_entropy: bool,
    /// Health status
    pub health_status: HsmHealthStatus,
    /// When discovered
    pub discovered_at: chrono::DateTime<chrono::Utc>,
    /// Last health check time
    pub last_health_check: chrono::DateTime<chrono::Utc>,
    /// Integration status
    pub integration_status: IntegrationStatus,
}

/// Discovery configuration
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    /// Enable cloud HSM discovery
    pub enable_cloud_discovery: bool,
    /// Enable PKCS#11 discovery
    pub enable_pkcs11_discovery: bool,
    /// Enable smartphone discovery
    pub enable_smartphone_discovery: bool,
    /// Discovery timeout in seconds
    pub discovery_timeout_seconds: u64,
    /// Enable capability detection
    pub enable_capability_detection: bool,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            enable_cloud_discovery: true,
            enable_pkcs11_discovery: true,
            enable_smartphone_discovery: true,
            discovery_timeout_seconds: 30,
            enable_capability_detection: true,
        }
    }
}

/// Universal HSM discovery engine
#[derive(Clone)]
pub struct UniversalHsmDiscovery {
    /// Discovered HSMs registry
    discovered_hsms: Arc<RwLock<HashMap<String, DiscoveredHsm>>>,
    /// Tier manager
    tier_manager: Arc<tier_manager::TierManager>,
    /// Entropy classifier
    entropy_classifier: Arc<human_entropy_classifier::HumanEntropyClassifier>,
    /// Discovery configuration
    config: DiscoveryConfig,
}

impl UniversalHsmDiscovery {
    /// Create new discovery engine
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub async fn new(config: DiscoveryConfig) -> Result<Self, BearDogError> {
        info!("Creating Universal HSM Discovery engine");

        Ok(Self {
            discovered_hsms: Arc::new(RwLock::new(HashMap::new())),
            tier_manager: Arc::new(tier_manager::TierManager::new().await?),
            entropy_classifier: Arc::new(
                human_entropy_classifier::HumanEntropyClassifier::new().await?,
            ),
            config,
        })
    }

    /// Discover all available HSMs
    ///
    /// # Errors
    /// Returns an error if discovery fails
    pub async fn discover_all(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("Starting HSM discovery");

        let mut discovered = Vec::new();

        // Cloud HSM discovery
        if self.config.enable_cloud_discovery {
            debug!("Discovering cloud HSMs");
            // TODO: Implement cloud discovery
        }

        // PKCS#11 discovery
        if self.config.enable_pkcs11_discovery {
            debug!("Discovering PKCS#11 HSMs");
            // TODO: Implement PKCS#11 discovery
        }

        // Smartphone discovery
        if self.config.enable_smartphone_discovery {
            debug!("Discovering smartphone HSMs");
            // TODO: Implement smartphone discovery
        }

        info!("Discovered {} HSMs", discovered.len());
        Ok(discovered)
    }

    /// Get discovered HSM by name
    ///
    /// # Errors
    /// Returns an error if HSM not found
    pub async fn get_hsm(&self, name: &str) -> Result<DiscoveredHsm, BearDogError> {
        let hsms = self.discovered_hsms.read().await;
        hsms.get(name)
            .cloned()
            .ok_or_else(|| BearDogError::not_found(format!("HSM not found: {}", name)))
    }

    /// List all discovered HSMs
    ///
    /// # Errors
    /// Returns an error if retrieval fails
    pub async fn list_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        let hsms = self.discovered_hsms.read().await;
        Ok(hsms.values().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_discovery_creation() -> Result<(), BearDogError> {
        let config = DiscoveryConfig::default();
        let discovery = UniversalHsmDiscovery::new(config).await?;
        assert!(discovery.discovered_hsms.read().await.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_default_config() {
        let config = DiscoveryConfig::default();
        assert!(config.enable_cloud_discovery);
        assert!(config.enable_pkcs11_discovery);
        assert!(config.enable_smartphone_discovery);
        assert_eq!(config.discovery_timeout_seconds, 30);
        assert!(config.enable_capability_detection);
    }
}
