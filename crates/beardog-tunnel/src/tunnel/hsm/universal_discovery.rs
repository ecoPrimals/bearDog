

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod capability_detector;
pub mod discovery_engine;
pub mod human_entropy_classifier;
pub mod tier_manager;
pub mod universal_adapter;
use beardog_errors::BearDogError;
use std::collections::HashMap;
use tracing::{info, warn};

pub struct UniversalHsmDiscovery {

    discovered_hsms: HashMap<String, DiscoveredHsm>,

    capability_detector: capability_detector::CapabilityDetector,

    entropy_classifier: human_entropy_classifier::HumanEntropyClassifier,

    tier_manager: tier_manager::TierManager,

    config: DiscoveryConfig,
}

#[derive(Debug, Clone)]
    /// The vendor value
    pub vendor: String,

    /// The model value
    pub model: String,

    /// The interface type value
    pub interface_type: HsmInterfaceType,

    /// The connection info value
    pub connection_info: HsmConnectionInfo,

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

#[derive(Debug, Clone)]
    CloudKms { provider: String, region: Option<String> },

    NetworkHsm { endpoint: String, port: u16 },

    UsbHsm { device_id: String },

    SoftwareHsm { implementation: String },

    MobileHsm { platform: String, chip: Option<String> },

    CustomApi { api_type: String, endpoint: String },

    Tpm { version: String },

    SmartCard { reader: String },

pub struct HsmConnectionInfo {

    /// The endpoint value
    pub endpoint: String,

    /// The auth method value
    pub auth_method: AuthenticationMethod,

    pub timeout_ms: u32,

    /// Whether encrypted is enabled
    pub encrypted: bool,

    /// Mapping of parameters
    pub parameters: HashMap<String, String>,

pub enum AuthenticationMethod {

    UsernamePassword { username: String },
    UsernamePassword { username: String },
    UsernamePassword { username: String },

    Certificate { cert_path: String },

    ApiKey { key_id: String },

    HardwareToken { token_id: String },

    Biometric { method: String },

    None,

pub struct DiscoveryHsmCapabilities {

    /// The key generation value
    pub key_generation: KeyGenerationCapabilities,

    /// The crypto operations value
    pub crypto_operations: CryptoOperationCapabilities,

    /// The advanced features value
    pub advanced_features: AdvancedFeatureCapabilities,

    pub performance: PerformanceCapabilities,

    /// The security value
    pub security: SecurityCapabilities,

    /// The human entropy value
    pub human_entropy: HumanEntropyCapabilities,

pub struct KeyGenerationCapabilities {

    /// Collection of supported key types
    pub supported_key_types: Vec<String>,

    /// Mapping of max key sizes
    pub max_key_sizes: HashMap<String, u32>,

    /// Whether hardware_backed is enabled
    pub hardware_backed: bool,

    /// Whether true_rng is enabled
    pub true_rng: bool,

    /// Collection of key derivation
    pub key_derivation: Vec<String>,

pub struct CryptoOperationCapabilities {

    /// Collection of signing algorithms
    pub signing_algorithms: Vec<String>,

    /// Collection of encryption algorithms
    pub encryption_algorithms: Vec<String>,

    /// Collection of hash functions
    pub hash_functions: Vec<String>,

    /// Collection of mac algorithms
    pub mac_algorithms: Vec<String>,

    /// Whether bulk_operations is enabled
    pub bulk_operations: bool,

    /// Whether streaming is enabled
    pub streaming: bool,

pub struct AdvancedFeatureCapabilities {

    /// Whether key_attestation is enabled
    pub key_attestation: bool,

    /// Whether user_presence is enabled
    pub user_presence: bool,

    /// Whether biometric_integration is enabled
    pub biometric_integration: bool,

    /// Whether multi_party is enabled
    pub multi_party: bool,

    /// Whether secure_boot is enabled
    pub secure_boot: bool,

    /// The tamper resistance value
    pub tamper_resistance: TamperResistanceLevel,

pub struct PerformanceCapabilities {

    /// The operations per second value
    pub operations_per_second: f64,

    /// The average latency ms value
    pub average_latency_ms: f64,

    /// Number of concurrent_operations
    pub concurrent_operations: u32,

    /// The memory usage value
    pub memory_usage: MemoryUsageLevel,

pub struct HumanEntropyCapabilities {

    /// Whether ephemeral_seed_creation is enabled
    pub ephemeral_seed_creation: bool,

    /// Collection of collection methods
    pub collection_methods: Vec<HumanEntropyMethod>,

    /// Whether entropy_quality_assessment is enabled
    pub entropy_quality_assessment: bool,

    /// Whether biometric_entropy is enabled
    pub biometric_entropy: bool,

    /// Whether behavioral_entropy is enabled
    pub behavioral_entropy: bool,

    pub realtime_entropy: bool,

pub enum HumanEntropyMethod {

    /// Represents mouse movement variant
    MouseMovement,

    /// Represents keystroke dynamics variant
    KeystrokeDynamics,

    /// Represents touch patterns variant
    TouchPatterns,

    /// Represents voice patterns variant
    VoicePatterns,

    /// Represents biometric variations variant
    BiometricVariations,

    /// Currently behavioraltiming
    BehavioralTiming,

    /// Represents camera entropy variant
    CameraEntropy,

    /// Represents custom input variant
    CustomInput,

#[derive(Debug, Clone)]
    pub tamper_evident_logs: bool,

    pub realtime_monitoring: bool,

    /// Collection of compliance reporting
    pub compliance_reporting: Vec<String>,

pub enum DiscoveryHsmHealthStatus {

    /// Represents healthy variant
    Healthy,

    /// Currently warning
    Warning,

    /// Represents critical variant
    Critical,

    /// Represents unavailable variant
    Unavailable,

    /// Unknown or undefined state
    Unknown,

impl Default for DiscoveryConfig {}
impl Default for DiscoveryConfig {}
impl Default for DiscoveryConfig {}

    fn default(true,
            enable_cloud_kms: true,
            enable_network_hsm: true,
            enable_usb_hsm: true,
            enable_software_hsm: true,
            enable_mobile_hsm: true,
            enable_tpm: true,
            discovery_timeout_seconds: 30,
            enable_human_entropy_elevation: true,
            minimum_entropy_quality: 0.8,
        }
    }
impl UniversalHsmDiscovery {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub async fn new(config: DiscoveryConfig) -> Result<Self, BearDogError> {
        info!("🔍 Initializing Universal HSM Discovery Engine");
        let capability_detector = capability_detector::CapabilityDetector::new()?;
        let entropy_classifier = human_entropy_classifier::HumanEntropyClassifier::new()?;
        let tier_manager = tier_manager::TierManager::new()?;
        Ok(Self {
            discovered_hsms: HashMap::with_capacity(16),
            capability_detector,
            entropy_classifier,
            tier_manager,
            config,
        })

/// Discover All Hsms operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn discover_all_hsms(&mut self) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        info!("🔍 Starting universal HSM discovery process");
        let mut all_discovered = Vec::new({} HSMs found", all_discovered.len());

        let human_entropy_count = all_discovered.iter()
            .filter(|h| h.supports_human_entropy)
            .count();
        if human_entropy_count > 0 {
            info!("🧠 {} HSMs support human entropy ephemeral seeds (tier elevated)", human_entropy_count);
        Ok(all_discovered)

/// Get Human Entropy Hsms operation.
    /// Gets human_entropy_hsms
    /// Gets human_entropy_hsms
    pub fn get_human_entropy_hsms(&self) -> Vec<&DiscoveredHsm> {
        self.discovered_hsms
            .values()
            .filter(|hsm| hsm.supports_human_entropy)
            .collect()

/// Get Hsms By Tier operation.
    /// Gets hsms_by_tier
    /// Gets hsms_by_tier
    pub fn get_hsms_by_tier(&self, tier: &HsmTier) -> Vec<&DiscoveredHsm> {
            .filter(|hsm| &hsm.assigned_tier == tier)

/// Get Best Hsm For Operation operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn get_best_hsm_for_operation(&self, operation_type: &str) -> Result<Option<&DiscoveredHsm>, BearDogError>> {
        self.tier_manager.select_best_hsm_for_operation(
            &self.discovered_hsms.values().collect::<Vec<_>>(),
            operation_type
        )

use crate::tunnel::hsm::types::HsmTier;

pub use capability_detector::CapabilityDetector;
pub use discovery_engine::DiscoveryEngine;
pub use human_entropy_classifier::HumanEntropyClassifier;
pub use tier_manager::TierManager;
pub use universal_adapter::UniversalAdapter; 
