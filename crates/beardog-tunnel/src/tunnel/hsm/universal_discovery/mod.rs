//! Universal HSM Discovery System
//!
//! This module provides automatic discovery and classification of Hardware Security Modules
//! across different platforms and vendors.

use crate::tunnel::hsm::types::HsmTier;
use beardog_errors::BearDogError;
use std::collections::HashMap;
use tracing::info;

pub mod capability_detector;
pub mod discovery_engine;
pub mod human_entropy_classifier;
pub mod tier_manager;
pub mod universal_adapter;

#[cfg(test)]
mod discovery_tests;

pub use capability_detector::CapabilityDetector;
pub use discovery_engine::DiscoveryEngine;
pub use human_entropy_classifier::HumanEntropyClassifier;
pub use tier_manager::TierManager;
pub use universal_adapter::UniversalAdapter;

/// Universal HSM Discovery Engine
pub struct UniversalHsmDiscovery {
    discovered_hsms: HashMap<String, DiscoveredHsm>,
    capability_detector: capability_detector::CapabilityDetector,
    entropy_classifier: human_entropy_classifier::HumanEntropyClassifier,
    tier_manager: tier_manager::TierManager,
    config: DiscoveryConfig,
}

// Use canonical HSM discovery configuration
pub use beardog_types::canonical::hsm::discovery::HsmDiscoveryConfig as DiscoveryConfig;

// NOTE: This instance had all HSM discovery fields (perfect match with HsmDiscoveryConfig)

/// A discovered HSM with its capabilities
#[derive(Debug, Clone)]
pub struct DiscoveredHsm {
    pub vendor: String,
    pub model: String,
    pub interface_type: HsmInterfaceType,
    pub connection_info: HsmConnectionInfo,
    pub capabilities: UniversalHsmCapabilities,
    pub assigned_tier: HsmTier,
    pub supports_human_entropy: bool,
    pub health_status: HsmHealthStatus,
    pub discovered_at: chrono::DateTime<chrono::Utc>,
}

/// Types of HSM interfaces
#[derive(Debug, Clone)]
pub enum HsmInterfaceType {
    CloudKms {
        provider: String,
        region: Option<String>,
    },
    NetworkHsm {
        endpoint: String,
        port: u16,
    },
    UsbHsm {
        device_id: String,
    },
    SoftwareHsm {
        implementation: String,
    },
    MobileHsm {
        platform: String,
        chip: Option<String>,
    },
    CustomApi {
        api_type: String,
        endpoint: String,
    },
    Tpm {
        version: String,
    },
    SmartCard {
        reader: String,
    },
}

/// HSM connection information
#[derive(Debug, Clone)]
pub struct HsmConnectionInfo {
    pub endpoint: String,
    pub auth_method: AuthenticationMethod,
    pub timeout_ms: u32,
    pub encrypted: bool,
    pub parameters: HashMap<String, String>,
}

/// Authentication methods for HSM access
#[derive(Debug, Clone)]
pub enum AuthenticationMethod {
    UsernamePassword { username: String },
    Certificate { cert_path: String },
    ApiKey { key_id: String },
    HardwareToken { token_id: String },
    Biometric { method: String },
    None,
}

/// Comprehensive HSM capabilities
#[derive(Debug, Clone, Default)]
pub struct UniversalHsmCapabilities {
    pub key_generation: KeyGenerationCapabilities,
    pub crypto_operations: CryptoOperationCapabilities,
    pub advanced_features: AdvancedFeatureCapabilities,
    pub performance: PerformanceCapabilities,
    pub security: SecurityCapabilities,
    pub human_entropy: HumanEntropyCapabilities,
}

/// Key generation capabilities
#[derive(Debug, Clone, Default)]
pub struct KeyGenerationCapabilities {
    pub supported_key_types: Vec<String>,
    pub max_key_sizes: HashMap<String, u32>,
    pub hardware_backed: bool,
    pub true_rng: bool,
    pub key_derivation: Vec<String>,
}

/// Cryptographic operation capabilities
#[derive(Debug, Clone, Default)]
pub struct CryptoOperationCapabilities {
    pub signing_algorithms: Vec<String>,
    pub encryption_algorithms: Vec<String>,
    pub hash_functions: Vec<String>,
    pub mac_algorithms: Vec<String>,
    pub bulk_operations: bool,
    pub streaming: bool,
}

/// Advanced feature capabilities
#[derive(Debug, Clone, Default)]
pub struct AdvancedFeatureCapabilities {
    pub key_attestation: bool,
    pub user_presence: bool,
    pub biometric_integration: bool,
    pub multi_party: bool,
    pub secure_boot: bool,
    pub tamper_resistance: TamperResistanceLevel,
}

/// Tamper resistance levels
#[derive(Debug, Clone, Default)]
pub enum TamperResistanceLevel {
    #[default]
    None,
    Basic,
    Advanced,
    Military,
}

/// Performance capabilities
#[derive(Debug, Clone, Default)]
pub struct PerformanceCapabilities {
    pub operations_per_second: f64,
    pub average_latency_ms: f64,
    pub concurrent_operations: u32,
    pub memory_usage: MemoryUsageLevel,
}

/// Memory usage levels
#[derive(Debug, Clone, Default)]
pub enum MemoryUsageLevel {
    #[default]
    Low,
    Medium,
    High,
}

/// Security capabilities
#[derive(Debug, Clone, Default)]
pub struct SecurityCapabilities {
    pub tamper_evident_logs: bool,
    pub realtime_monitoring: bool,
    pub compliance_reporting: Vec<String>,
}

/// Human entropy capabilities
#[derive(Debug, Clone, Default)]
pub struct HumanEntropyCapabilities {
    pub ephemeral_seed_creation: bool,
    pub collection_methods: Vec<HumanEntropyMethod>,
    pub entropy_quality_assessment: bool,
    pub biometric_entropy: bool,
    pub behavioral_entropy: bool,
    pub realtime_entropy: bool,
}

/// Methods for collecting human entropy
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum HumanEntropyMethod {
    MouseMovement,
    KeystrokeDynamics,
    TouchPatterns,
    VoicePatterns,
    BiometricVariations,
    BehavioralTiming,
    CameraEntropy,
    CustomInput,
}

/// HSM health status
#[derive(Debug, Clone)]
pub enum HsmHealthStatus {
    Healthy,
    Warning,
    Critical,
    Unavailable,
    Unknown,
}

impl UniversalHsmDiscovery {
    /// Creates a new UniversalHsmDiscovery instance
    ///
    /// # Errors
    /// Returns an error if component initialization fails.
    pub async fn new(config: DiscoveryConfig) -> Result<Self, BearDogError> {
        info!("🔍 Initializing Universal HSM Discovery Engine");

        let capability_detector = capability_detector::CapabilityDetector::new();
        let entropy_classifier = human_entropy_classifier::HumanEntropyClassifier::new();
        let tier_manager = tier_manager::TierManager::new();

        Ok(Self {
            discovered_hsms: HashMap::with_capacity(16),
            capability_detector,
            entropy_classifier: entropy_classifier?,
            tier_manager: tier_manager?,
            config,
        })
    }

    /// Discovers all available HSMs
    ///
    /// # Errors
    /// Returns an error if discovery fails.
    pub fn discover_all_hsms(&mut self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("🔍 Starting universal HSM discovery process");

        let all_discovered: Vec<DiscoveredHsm> = Vec::new();

        // Discovery logic would go here
        // For now, returning empty list as placeholder

        info!(
            "✅ Universal HSM discovery complete: {} HSMs found",
            all_discovered.len()
        );

        let human_entropy_count = all_discovered
            .iter()
            .filter(|h| h.supports_human_entropy)
            .count();

        if human_entropy_count > 0 {
            info!(
                "🧠 {} HSMs support human entropy ephemeral seeds (tier elevated)",
                human_entropy_count
            );
        }

        Ok(all_discovered)
    }

    /// Gets HSMs that support human entropy
    pub fn get_human_entropy_hsms(&self) -> Vec<&DiscoveredHsm> {
        self.discovered_hsms
            .values()
            .filter(|hsm| hsm.supports_human_entropy)
            .collect()
    }

    /// Gets HSMs by tier
    pub fn get_hsms_by_tier(&self, tier: &HsmTier) -> Vec<&DiscoveredHsm> {
        self.discovered_hsms
            .values()
            .filter(|hsm| &hsm.assigned_tier == tier)
            .collect()
    }

    /// Gets the best HSM for a specific operation
    ///
    /// # Errors
    /// Returns an error if selection fails.
    pub fn get_best_hsm_for_operation(
        &self,
        operation_type: &str,
    ) -> Result<Option<&DiscoveredHsm>, BearDogError> {
        self.tier_manager.select_best_hsm_for_operation(
            &self.discovered_hsms.values().collect::<Vec<_>>(),
            operation_type,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_discovery_creation() -> Result<(), Box<dyn std::error::Error>> {
        let config = DiscoveryConfig::default();
        let discovery = UniversalHsmDiscovery::new(config).await;
        assert!(discovery.is_ok());
        Ok(())
    }

    #[test]
    fn test_discovery_config_default() -> Result<(), Box<dyn std::error::Error>> {
        let config = DiscoveryConfig::default();
        assert!(config.enable_human_entropy_elevation);
        assert_eq!(config.minimum_entropy_quality, 0.8);
        Ok(())
    }
}
