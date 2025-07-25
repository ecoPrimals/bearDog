//! Universal HSM Discovery System
//!
//! This module provides comprehensive HSM discovery across all platforms
//! and connection types, with intelligent tier-based selection.

use crate::tunnel::hsm::types::{HsmKey, HsmCapabilities, HsmHealthStatus, KeyType, HsmTier};
use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

pub mod capability_detection;
pub mod capability_detector;
pub mod discovery;
pub mod discovery_engine;
pub mod human_entropy_classifier;
pub mod tier_manager;
pub mod universal_adapter;

#[cfg(test)]
pub mod tests;

#[cfg(test)]
pub mod integration_tests;

/// Main Universal HSM Discovery coordinator
#[derive(Debug)]
pub struct UniversalHsmDiscovery {
    discovered_hsms: HashMap<String, DiscoveredHsm>,
    capability_detector: capability_detector::CapabilityDetector,
    entropy_classifier: human_entropy_classifier::HumanEntropyClassifier,
    tier_manager: tier_manager::TierManager,
    discovery_engine: discovery_engine::DiscoveryEngine,
    universal_adapter: universal_adapter::UniversalAdapter,
    config: DiscoveryConfig,
}

/// Discovered HSM with full capability profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredHsm {
    pub hsm_id: String,
    pub vendor: String,
    pub model: String,
    pub version: String,
    pub interface_type: HsmInterfaceType,
    pub connection_info: HsmConnectionInfo,
    pub capabilities: UniversalHsmCapabilities,
    pub assigned_tier: HsmTier,
    pub supports_human_entropy: bool,
    pub health_status: HsmHealthStatus,
    pub discovered_at: chrono::DateTime<chrono::Utc>,
    pub last_health_check: chrono::DateTime<chrono::Utc>,
    pub integration_status: IntegrationStatus,
}

/// Types of HSM interfaces that can be discovered
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HsmInterfaceType {
    // Hardware HSMs
    Pkcs11 {
        library_path: String,
    },
    NetworkHsm {
        endpoint: String,
        protocol: String,
    },
    UsbHsm {
        device_path: String,
    },
    SmartCard {
        reader_name: String,
    },
    Tpm {
        version: String,
    },

    // Cloud HSMs
    AwsKms {
        region: String,
    },
    AzureKeyVault {
        vault_url: String,
    },
    GcpKms {
        project_id: String,
        location: String,
    },

    // Mobile HSMs
    AndroidStrongBox {
        security_level: String,
    },
    IosSecureEnclave {
        enclave_version: String,
    },

    // Software HSMs
    SoftHsm {
        config_path: String,
    },
    OpenSsl {
        engine_path: Option<String>,
    },
    BearDogNative {
        instance_id: String,
    },

    // Platform HSMs
    WindowsCng {
        provider_name: String,
    },
    MacOsKeychain {
        keychain_path: String,
    },

    // Custom/Proprietary
    CustomApi {
        api_endpoint: String,
        api_version: String,
    },
    ProprietaryDriver {
        driver_path: String,
        driver_version: String,
    },
}

/// HSM connection and authentication information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmConnectionInfo {
    pub connection_type: ConnectionType,
    pub authentication: AuthenticationMethod,
    pub endpoint: Option<String>,
    pub port: Option<u16>,
    pub timeout: Duration,
    pub retry_policy: RetryPolicy,
    pub ssl_config: Option<SslConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionType {
    Local,
    Network,
    Cloud,
    Mobile,
    Embedded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationMethod {
    None,
    Pin {
        pin: String,
    },
    Certificate {
        cert_path: String,
        key_path: String,
    },
    Token {
        token: String,
    },
    OAuth {
        client_id: String,
        client_secret: String,
    },
    MutualTls {
        client_cert: String,
        client_key: String,
    },
    ApiKey {
        api_key: String,
    },
    Biometric {
        biometric_type: String,
    },
    SmartCard {
        card_id: String,
    },
}

/// Comprehensive HSM capabilities assessment (renamed to avoid conflict with types::HsmCapabilities)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UniversalHsmCapabilities {
    // Core cryptographic capabilities
    pub key_generation: KeyGenerationCapabilities,
    pub crypto_operations: CryptoOperationCapabilities,
    pub key_management: KeyManagementCapabilities,

    // Advanced features
    pub advanced_features: AdvancedFeatureCapabilities,
    pub performance: PerformanceCapabilities,
    pub security: SecurityCapabilities,

    // Human entropy - critical for tier elevation
    pub human_entropy: HumanEntropyCapabilities,

    // Integration capabilities
    pub api_support: ApiSupportCapabilities,
    pub compliance: ComplianceCapabilities,
}

/// Key generation capabilities
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KeyGenerationCapabilities {
    pub supported_algorithms: Vec<String>,
    pub key_sizes: Vec<u32>,
    pub can_generate_in_hardware: bool,
    pub supports_key_derivation: bool,
    pub supports_secure_key_import: bool,
    pub supports_key_wrapping: bool,
    pub entropy_sources: Vec<String>,
    pub fips_compliant_generation: bool,
}

/// Cryptographic operation capabilities
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CryptoOperationCapabilities {
    pub encryption_algorithms: Vec<String>,
    pub signing_algorithms: Vec<String>,
    pub hashing_algorithms: Vec<String>,
    pub key_agreement_algorithms: Vec<String>,
    pub supports_streaming: bool,
    pub supports_batch_operations: bool,
    pub max_data_size: Option<usize>,
    pub hardware_acceleration: bool,
}

/// Key management and lifecycle capabilities
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KeyManagementCapabilities {
    pub supports_key_backup: bool,
    pub supports_key_recovery: bool,
    pub supports_key_escrow: bool,
    pub supports_key_rotation: bool,
    pub supports_key_versioning: bool,
    pub supports_key_attestation: bool,
    pub key_storage_types: Vec<String>,
    pub max_keys: Option<u32>,
}

/// Advanced HSM features
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdvancedFeatureCapabilities {
    pub supports_secure_boot: bool,
    pub supports_remote_attestation: bool,
    pub supports_secure_channels: bool,
    pub supports_multi_tenancy: bool,
    pub supports_role_based_access: bool,
    pub supports_audit_logging: bool,
    pub supports_clustering: bool,
    pub supports_load_balancing: bool,
}

/// Performance characteristics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceCapabilities {
    pub operations_per_second: HashMap<String, u32>,
    pub latency_ms: HashMap<String, f64>,
    pub throughput_mbps: Option<f64>,
    pub concurrent_operations: u32,
    pub memory_usage_mb: Option<u32>,
    pub power_consumption_watts: Option<f64>,
}

/// Security features and certifications
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityCapabilities {
    pub fips_140_level: Option<u8>,
    pub common_criteria_level: Option<String>,
    pub tamper_resistance: TamperResistance,
    pub secure_key_storage: bool,
    pub side_channel_resistance: bool,
    pub fault_injection_resistance: bool,
    pub certified_algorithms: Vec<String>,
    pub security_certifications: Vec<String>,
}

/// Human entropy capabilities - critical for tier elevation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HumanEntropyCapabilities {
    pub supports_human_entropy: bool,
    pub supports_ephemeral_seeds: bool,
    pub entropy_collection_methods: Vec<EntropyCollectionMethod>,
    pub entropy_quality_assessment: bool,
    pub real_time_entropy_generation: bool,
    pub biometric_entropy_integration: bool,
    pub user_interaction_entropy: bool,
    pub temporal_entropy_collection: bool,
    pub entropy_verification: bool,
    pub ephemeral_seed_lifetime: Option<Duration>,
}

/// Methods for collecting human entropy
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum EntropyCollectionMethod {
    /// Touch screen patterns and gestures
    TouchPatterns { pressure_sensitive: bool },
    /// Device motion and accelerometer data
    DeviceMotion,
    /// Biometric variation patterns
    BiometricVariation,
    /// Keyboard timing patterns
    KeyboardTiming,
    /// Advanced touch patterns with additional sensors
    TouchPatternsAdvanced { pressure_sensitive: bool },
    /// Environmental sensor data (light, proximity, etc.)
    EnvironmentalSensors { ambient_light: bool, proximity: bool },
}

/// API and integration support
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiSupportCapabilities {
    pub pkcs11_support: bool,
    pub jce_support: bool,
    pub cng_support: bool,
    pub openssl_engine: bool,
    pub rest_api: bool,
    pub grpc_api: bool,
    pub graphql_api: bool,
    pub custom_sdks: Vec<String>,
}

/// Compliance and regulatory support
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComplianceCapabilities {
    pub fips_140_certified: bool,
    pub common_criteria_certified: bool,
    pub pci_dss_compliant: bool,
    pub hipaa_compliant: bool,
    pub gdpr_compliant: bool,
    pub sox_compliant: bool,
    pub compliance_certifications: Vec<String>,
    pub audit_trail_support: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum TamperResistance {
    #[default]
    None,
    TamperEvident,
    TamperResistant,
    TamperResponsive,
}

/// HSM security level classification (renamed from HsmTier to avoid conflict)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum HsmSecurityLevel {
    /// Software-only implementations
    Software = 1,
    /// Basic hardware security
    BasicHardware = 2,
    /// Certified hardware security
    CertifiedHardware = 3,
    /// High-security hardware with advanced features  
    HighSecurity = 4,
    /// Premium tier with human entropy ephemeral seed support
    HumanEntropyPremium = 5,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UniversalHsmHealthStatus {
    Healthy,
    Warning { message: String },
    Degraded { reason: String },
    Failed { error: String },
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationStatus {
    Discovered,
    Connecting,
    Connected,
    Authenticated,
    Ready,
    Error { message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SslConfig {
    pub ca_cert_path: Option<String>,
    pub verify_hostname: bool,
    pub min_tls_version: String,
    pub cipher_suites: Vec<String>,
}

/// Discovery configuration
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    pub auto_discovery_enabled: bool,
    pub discovery_interval: Duration,
    pub health_check_interval: Duration,
    pub capability_refresh_interval: Duration,
    pub timeout: Duration,
    pub max_concurrent_discoveries: usize,
    pub tier_elevation_enabled: bool,
    pub human_entropy_priority: bool,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(300), // 5 minutes
            health_check_interval: Duration::from_secs(60), // 1 minute
            capability_refresh_interval: Duration::from_secs(3600), // 1 hour
            timeout: Duration::from_secs(30),
            max_concurrent_discoveries: 10,
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        }
    }
}

impl UniversalHsmDiscovery {
    /// Create new Universal HSM Discovery system
    pub fn new() -> BearDogResult<Self> {
        let config = DiscoveryConfig::default();

        Ok(Self {
            discovered_hsms: HashMap::new(),
            capability_detector: capability_detector::CapabilityDetector::new()?,
            entropy_classifier: human_entropy_classifier::HumanEntropyClassifier::new()?,
            tier_manager: tier_manager::TierManager::new()?,
            discovery_engine: discovery_engine::DiscoveryEngine::new()?,
            universal_adapter: universal_adapter::UniversalAdapter::new()?,
            config,
        })
    }

    /// Discover all available HSMs in the environment
    pub async fn discover_all_hsms(&mut self) -> BearDogResult<Vec<DiscoveredHsm>> {
        info!("🔍 Starting universal HSM discovery");

        // Use discovery engine to find HSMs
        let discovered = self.discovery_engine.discover_hsms(&self.config).await?;

        for mut hsm in discovered {
            // Detect full capabilities
            hsm.capabilities = self
                .capability_detector
                .detect_capabilities(&hsm.interface_type)
                .await?;

            // Classify human entropy support
            hsm.supports_human_entropy = self
                .entropy_classifier
                .classify_human_entropy_support(&hsm.capabilities)
                .await?;

            // Assign tier based on capabilities
            hsm.assigned_tier = self
                .tier_manager
                .assign_tier(&hsm.capabilities, hsm.supports_human_entropy)
                .await?;

            // Store discovered HSM
            self.discovered_hsms.insert(hsm.hsm_id.clone(), hsm);
        }

        let hsm_list: Vec<DiscoveredHsm> = self.discovered_hsms.values().cloned().collect();
        info!("✅ Discovered {} HSMs total", hsm_list.len());

        Ok(hsm_list)
    }

    /// Get HSMs that support human entropy ephemeral seeds
    pub fn get_human_entropy_hsms(&self) -> Vec<&DiscoveredHsm> {
        self.discovered_hsms
            .values()
            .filter(|hsm| hsm.supports_human_entropy)
            .collect()
    }

    /// Get HSMs by tier
    pub fn get_hsms_by_tier(&self, tier: HsmTier) -> Vec<&DiscoveredHsm> {
        self.discovered_hsms
            .values()
            .filter(|hsm| hsm.assigned_tier == tier)
            .collect()
    }

    /// Get the best HSM for a specific operation
    pub async fn get_best_hsm_for_operation(
        &self,
        operation_type: &str,
    ) -> BearDogResult<Option<&DiscoveredHsm>> {
        self.tier_manager
            .select_best_hsm_for_operation(&self.discovered_hsms, operation_type)
            .await
    }

    /// Get universal adapter for HSM integration
    pub fn get_universal_adapter(&self) -> &universal_adapter::UniversalAdapter {
        &self.universal_adapter
    }

    /// Update discovery configuration
    pub fn update_config(&mut self, config: DiscoveryConfig) {
        self.config = config;
    }

    /// Get current HSM statistics
    pub fn get_discovery_stats(&self) -> DiscoveryStats {
        let total_hsms = self.discovered_hsms.len();
        let human_entropy_hsms = self.get_human_entropy_hsms().len();
        let tier_distribution = self.get_tier_distribution();
        let healthy_hsms = self
            .discovered_hsms
            .values()
            .filter(|hsm| matches!(hsm.health_status, HsmHealthStatus::Healthy))
            .count();

        DiscoveryStats {
            total_hsms,
            human_entropy_hsms,
            tier_distribution,
            healthy_hsms,
            last_discovery: chrono::Utc::now(),
        }
    }

    fn get_tier_distribution(&self) -> HashMap<HsmTier, usize> {
        let mut distribution = HashMap::new();
        for hsm in self.discovered_hsms.values() {
            *distribution.entry(hsm.assigned_tier).or_insert(0) += 1;
        }
        distribution
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryStats {
    pub total_hsms: usize,
    pub human_entropy_hsms: usize,
    pub tier_distribution: HashMap<HsmTier, usize>,
    pub healthy_hsms: usize,
    pub last_discovery: chrono::DateTime<chrono::Utc>,
}
