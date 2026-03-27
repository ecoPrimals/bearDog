// SPDX-License-Identifier: AGPL-3.0-only

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
///
/// Provides automatic discovery and classification of Hardware Security Modules
/// across different platforms, vendors, and interfaces.
pub struct UniversalHsmDiscovery {
    /// Map of discovered HSMs by their unique identifier
    discovered_hsms: HashMap<String, DiscoveredHsm>,
    /// Capability detection component
    _capability_detector: capability_detector::CapabilityDetector,
    /// Human entropy classification component
    entropy_classifier: human_entropy_classifier::HumanEntropyClassifier,
    /// Tier assignment manager
    tier_manager: tier_manager::TierManager,
    /// Discovery configuration
    _config: DiscoveryConfig,
}

// Use canonical HSM discovery configuration
pub use beardog_types::canonical::hsm::discovery::HsmDiscoveryConfig as DiscoveryConfig;

// NOTE: This instance had all HSM discovery fields (perfect match with HsmDiscoveryConfig)

/// A discovered HSM with its capabilities
///
/// Represents a fully discovered and characterized HSM device.
#[derive(Debug, Clone)]
pub struct DiscoveredHsm {
    /// HSM vendor name (e.g., "Yubico", "Thales", "AWS")
    pub vendor: String,
    /// HSM model identifier
    pub model: String,
    /// Interface type used to communicate with the HSM
    pub interface_type: HsmInterfaceType,
    /// Connection information for accessing the HSM
    pub connection_info: HsmConnectionInfo,
    /// Comprehensive capabilities of the HSM
    pub capabilities: UniversalHsmCapabilities,
    /// Assigned security tier based on capabilities
    pub assigned_tier: HsmTier,
    /// Whether the HSM supports human entropy collection
    pub supports_human_entropy: bool,
    /// Current health status of the HSM
    pub health_status: HsmHealthStatus,
    /// Timestamp when the HSM was discovered
    pub discovered_at: chrono::DateTime<chrono::Utc>,
}

/// Types of HSM interfaces
///
/// Represents the different communication interfaces through which
/// HSMs can be accessed.
#[derive(Debug, Clone)]
pub enum HsmInterfaceType {
    /// Cloud-based Key Management Service (AWS KMS, GCP KMS, Azure Key Vault)
    CloudKms {
        /// Cloud provider name
        provider: String,
        /// Optional region identifier
        region: Option<String>,
    },
    /// Network-attached HSM (PKCS#11 over network, proprietary protocols)
    NetworkHsm {
        /// Network endpoint address
        endpoint: String,
        /// Network port
        port: u16,
    },
    /// USB-attached HSM (YubiHSM, Solo, etc.)
    UsbHsm {
        /// USB device identifier
        device_id: String,
    },
    /// Software-based HSM implementation
    SoftwareHsm {
        /// Implementation name (e.g., "SoftHSM2", "HashiCorp Vault")
        implementation: String,
    },
    /// Mobile device HSM (Android StrongBox, iOS Secure Enclave)
    MobileHsm {
        /// Platform name (Android, iOS)
        platform: String,
        /// Optional security chip identifier (Titan M, Secure Enclave)
        chip: Option<String>,
    },
    /// Custom API-based HSM
    CustomApi {
        /// API type identifier
        api_type: String,
        /// API endpoint
        endpoint: String,
    },
    /// Trusted Platform Module
    Tpm {
        /// TPM version (1.2, 2.0)
        version: String,
    },
    /// Smart card HSM (PIV, CAC)
    SmartCard {
        /// Card reader identifier
        reader: String,
    },
}

/// HSM connection information
///
/// Contains all necessary details for establishing a connection to an HSM.
#[derive(Debug, Clone)]
pub struct HsmConnectionInfo {
    /// Connection endpoint (URL, IP address, device path)
    pub endpoint: String,
    /// Authentication method for HSM access
    pub auth_method: AuthenticationMethod,
    /// Connection timeout in milliseconds
    pub timeout_ms: u32,
    /// Whether the connection is encrypted
    pub encrypted: bool,
    /// Additional connection parameters
    pub parameters: HashMap<String, String>,
}

/// Authentication methods for HSM access
///
/// Represents the various ways to authenticate to an HSM.
#[derive(Debug, Clone)]
pub enum AuthenticationMethod {
    /// Username and password authentication
    UsernamePassword {
        /// Username for authentication
        username: String,
    },
    /// Certificate-based authentication
    Certificate {
        /// Path to the certificate file
        cert_path: String,
    },
    /// API key authentication
    ApiKey {
        /// API key identifier
        key_id: String,
    },
    /// Hardware token authentication (FIDO2, etc.)
    HardwareToken {
        /// Token identifier
        token_id: String,
    },
    /// Biometric authentication
    Biometric {
        /// Biometric method (fingerprint, face, etc.)
        method: String,
    },
    /// No authentication required
    None,
}

/// Comprehensive HSM capabilities
///
/// Aggregates all capability categories for a discovered HSM.
#[derive(Debug, Clone, Default)]
pub struct UniversalHsmCapabilities {
    /// Key generation capabilities
    pub key_generation: KeyGenerationCapabilities,
    /// Cryptographic operation capabilities
    pub crypto_operations: CryptoOperationCapabilities,
    /// Advanced security features
    pub advanced_features: AdvancedFeatureCapabilities,
    /// Performance characteristics
    pub performance: PerformanceCapabilities,
    /// Security and compliance capabilities
    pub security: SecurityCapabilities,
    /// Human entropy collection capabilities
    pub human_entropy: HumanEntropyCapabilities,
}

/// Key generation capabilities
///
/// Describes what types of keys the HSM can generate.
#[derive(Debug, Clone, Default)]
pub struct KeyGenerationCapabilities {
    /// List of supported key types (Ed25519, P-256, RSA, etc.)
    pub supported_key_types: Vec<String>,
    /// Maximum key sizes by type (in bits)
    pub max_key_sizes: HashMap<String, u32>,
    /// Whether keys are hardware-backed (non-extractable)
    pub hardware_backed: bool,
    /// Whether the HSM has a true hardware RNG
    pub true_rng: bool,
    /// Supported key derivation functions
    pub key_derivation: Vec<String>,
}

/// Cryptographic operation capabilities
///
/// Describes what crypto operations the HSM can perform.
#[derive(Debug, Clone, Default)]
pub struct CryptoOperationCapabilities {
    /// Supported digital signature algorithms
    pub signing_algorithms: Vec<String>,
    /// Supported encryption algorithms
    pub encryption_algorithms: Vec<String>,
    /// Supported hash functions
    pub hash_functions: Vec<String>,
    /// Supported MAC algorithms
    pub mac_algorithms: Vec<String>,
    /// Whether bulk operations are supported
    pub bulk_operations: bool,
    /// Whether streaming operations are supported
    pub streaming: bool,
}

/// Advanced feature capabilities
///
/// Describes advanced security features supported by the HSM.
#[derive(Debug, Clone, Default)]
pub struct AdvancedFeatureCapabilities {
    /// Hardware key attestation support
    pub key_attestation: bool,
    /// User presence verification support
    pub user_presence: bool,
    /// Biometric authentication integration
    pub biometric_integration: bool,
    /// Multi-party computation support
    pub multi_party: bool,
    /// Secure boot verification
    pub secure_boot: bool,
    /// Tamper resistance level
    pub tamper_resistance: TamperResistanceLevel,
}

/// Tamper resistance levels
///
/// Indicates the physical tamper resistance of the HSM.
#[derive(Debug, Clone, Default)]
pub enum TamperResistanceLevel {
    /// No tamper resistance
    #[default]
    None,
    /// Basic tamper evidence (seals, etc.)
    Basic,
    /// Advanced tamper response (zeroization)
    Advanced,
    /// Military-grade tamper resistance (FIPS 140-3 Level 4)
    Military,
}

/// Performance capabilities
///
/// Describes the performance characteristics of the HSM.
#[derive(Debug, Clone, Default)]
pub struct PerformanceCapabilities {
    /// Maximum operations per second
    pub operations_per_second: f64,
    /// Average operation latency in milliseconds
    pub average_latency_ms: f64,
    /// Maximum concurrent operations supported
    pub concurrent_operations: u32,
    /// Memory usage level
    pub memory_usage: MemoryUsageLevel,
}

/// Memory usage levels
///
/// Indicates the memory footprint of HSM operations.
#[derive(Debug, Clone, Default)]
pub enum MemoryUsageLevel {
    /// Low memory usage (< 100MB)
    #[default]
    Low,
    /// Medium memory usage (100MB - 1GB)
    Medium,
    /// High memory usage (> 1GB)
    High,
}

/// Security capabilities
///
/// Describes security and compliance features.
#[derive(Debug, Clone, Default)]
pub struct SecurityCapabilities {
    /// Tamper-evident audit logging
    pub tamper_evident_logs: bool,
    /// Real-time security monitoring
    pub realtime_monitoring: bool,
    /// Compliance certifications (FIPS, CC, etc.)
    pub compliance_reporting: Vec<String>,
}

/// Human entropy capabilities
///
/// Describes the HSM's ability to collect human-sourced entropy.
#[derive(Debug, Clone, Default)]
pub struct HumanEntropyCapabilities {
    /// Ephemeral seed creation from human entropy
    pub ephemeral_seed_creation: bool,
    /// Supported entropy collection methods
    pub collection_methods: Vec<HumanEntropyMethod>,
    /// Entropy quality assessment support
    pub entropy_quality_assessment: bool,
    /// Biometric-based entropy collection
    pub biometric_entropy: bool,
    /// Behavioral-based entropy collection
    pub behavioral_entropy: bool,
    /// Real-time entropy collection
    pub realtime_entropy: bool,
}

/// Methods for collecting human entropy
///
/// Different sources of human-generated randomness.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum HumanEntropyMethod {
    /// Mouse movement patterns
    MouseMovement,
    /// Keyboard typing patterns
    KeystrokeDynamics,
    /// Touch screen interaction patterns
    TouchPatterns,
    /// Voice input variations
    VoicePatterns,
    /// Biometric measurement variations
    BiometricVariations,
    /// Human timing behavior
    BehavioralTiming,
    /// Camera-based random input
    CameraEntropy,
    /// Custom entropy input method
    CustomInput,
}

/// HSM health status
///
/// Current operational status of a discovered HSM.
#[derive(Debug, Clone)]
pub enum HsmHealthStatus {
    /// HSM is operating normally
    Healthy,
    /// HSM has minor issues but is operational
    Warning,
    /// HSM has critical issues, may fail
    Critical,
    /// HSM is not accessible
    Unavailable,
    /// Health status could not be determined
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
            _capability_detector: capability_detector,
            entropy_classifier: entropy_classifier?,
            tier_manager: tier_manager?,
            _config: config,
        })
    }

    /// Discovers all available HSMs using the discovery engine
    ///
    /// Performs parallel discovery across all HSM interfaces:
    /// - PKCS#11 libraries
    /// - Cloud KMS (AWS, GCP, Azure)
    /// - Network HSMs
    /// - USB HSMs (YubiHSM, Solo)
    /// - Software HSMs (BearDog Native, SoftHSM)
    /// - Mobile HSMs (Android StrongBox, iOS Secure Enclave)
    /// - TPMs
    /// - Smart Cards
    ///
    /// # Errors
    /// Returns an error if discovery engine initialization fails.
    /// Individual discoverer failures are logged but don't stop overall discovery.
    pub fn discover_all_hsms(&mut self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("🔍 Starting universal HSM discovery process");

        // Initialize the discovery engine
        let engine = discovery_engine::DiscoveryEngine::new()?;

        // Collect results from all discoverers (failures don't stop discovery)
        let mut all_discovered: Vec<DiscoveredHsm> = Vec::new();

        // 1. PKCS#11 HSMs
        match engine.discover_pkcs11_hsms() {
            Ok(hsms) => {
                info!("🔐 PKCS#11: discovered {} HSMs", hsms.len());
                all_discovered.extend(hsms);
            }
            Err(e) => {
                tracing::warn!("⚠️ PKCS#11 discovery failed: {}", e);
            }
        }

        // 2. Cloud KMS
        match engine.discover_cloud_kms_hsms() {
            Ok(hsms) => {
                info!("☁️ Cloud KMS: discovered {} instances", hsms.len());
                all_discovered.extend(hsms);
            }
            Err(e) => {
                tracing::warn!("⚠️ Cloud KMS discovery failed: {}", e);
            }
        }

        // 3. Network HSMs
        match engine.discover_network_hsms() {
            Ok(hsms) => {
                info!("🌐 Network: discovered {} HSMs", hsms.len());
                all_discovered.extend(hsms);
            }
            Err(e) => {
                tracing::warn!("⚠️ Network HSM discovery failed: {}", e);
            }
        }

        // 4. USB HSMs
        match engine.discover_usb_hsms() {
            Ok(hsms) => {
                info!("🔌 USB: discovered {} HSMs", hsms.len());
                all_discovered.extend(hsms);
            }
            Err(e) => {
                tracing::warn!("⚠️ USB HSM discovery failed: {}", e);
            }
        }

        // 5. Software HSMs (always available - BearDog Native)
        match engine.discover_software_hsms() {
            Ok(hsms) => {
                info!("💻 Software: discovered {} HSMs", hsms.len());
                all_discovered.extend(hsms);
            }
            Err(e) => {
                tracing::warn!("⚠️ Software HSM discovery failed: {}", e);
            }
        }

        // 6. Mobile HSMs (conditional on platform)
        match engine.discover_mobile_hsms() {
            Ok(hsms) => {
                info!("📱 Mobile: discovered {} HSMs", hsms.len());
                all_discovered.extend(hsms);
            }
            Err(e) => {
                tracing::warn!("⚠️ Mobile HSM discovery failed: {}", e);
            }
        }

        // 7. TPMs
        match engine.discover_tpm_hsms() {
            Ok(hsms) => {
                info!("🔐 TPM: discovered {} modules", hsms.len());
                all_discovered.extend(hsms);
            }
            Err(e) => {
                tracing::warn!("⚠️ TPM discovery failed: {}", e);
            }
        }

        // 8. Smart Cards
        match engine.discover_smartcard_hsms() {
            Ok(hsms) => {
                info!("💳 Smart Card: discovered {} cards", hsms.len());
                all_discovered.extend(hsms);
            }
            Err(e) => {
                tracing::warn!("⚠️ Smart Card discovery failed: {}", e);
            }
        }

        // Classify human entropy support for each HSM
        for hsm in &mut all_discovered {
            hsm.supports_human_entropy = self.entropy_classifier.supports_human_entropy(hsm);
            hsm.assigned_tier = self.tier_manager.assign_tier(hsm);
        }

        // Store in internal map (deduplicate by vendor+model+interface)
        for hsm in &all_discovered {
            let key = format!("{}:{}:{:?}", hsm.vendor, hsm.model, hsm.interface_type);
            self.discovered_hsms.insert(key, hsm.clone());
        }

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
    use crate::tunnel::hsm::types::HsmTier;

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

    #[tokio::test]
    async fn test_discover_all_hsms_populates_registry_and_queries() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new(DiscoveryConfig::default()).await?;
        let hsms = discovery.discover_all_hsms()?;
        assert!(
            !hsms.is_empty(),
            "software HSM discoverer should yield at least one HSM"
        );

        let _entropy_supported = discovery.get_human_entropy_hsms();
        let _by_tier = discovery.get_hsms_by_tier(&HsmTier::Software);
        let best = discovery.get_best_hsm_for_operation("sign")?;
        assert!(best.is_some());

        Ok(())
    }
}
