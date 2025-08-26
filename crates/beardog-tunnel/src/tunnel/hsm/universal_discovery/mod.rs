

pub mod capability_detector;
pub mod discovery_engine;
pub mod human_entropy_classifier;
pub mod tier_manager;
pub mod universal_adapter;
use beardog_errors::{BearDogError, BearDogResult};
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
pub struct DiscoveredHsm {

    pub hsm_id: String,

    pub vendor: String,

    pub model: String,

    pub interface_type: HsmInterfaceType,

    pub connection_info: HsmConnectionInfo,

    pub capabilities: UniversalHsmCapabilities,

    pub assigned_tier: HsmTier,

    pub supports_human_entropy: bool,

    pub health_status: HsmHealthStatus,

    pub discovered_at: chrono::DateTime<chrono::Utc>,

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HsmInterfaceType {

    Pkcs11 { library_path: String },

    CloudKms { provider: String, region: Option<String> },

    NetworkHsm { endpoint: String, port: u16 },

    UsbHsm { device_id: String },

    SoftwareHsm { implementation: String },

    MobileHsm { platform: String, chip: Option<String> },

    CustomApi { api_type: String, endpoint: String },

    Tpm { version: String },

    SmartCard { reader: String },

pub struct HsmConnectionInfo {

    pub endpoint: String,

    pub auth_method: AuthenticationMethod,

    pub timeout_ms: u32,

    pub encrypted: bool,

    pub parameters: HashMap<String, String>,

pub enum AuthenticationMethod {

    UsernamePassword { username: String },

    Certificate { cert_path: String },

    ApiKey { key_id: String },

    HardwareToken { token_id: String },

    Biometric { method: String },

    None,

pub struct DiscoveryHsmCapabilities {

    pub key_generation: KeyGenerationCapabilities,

    pub crypto_operations: CryptoOperationCapabilities,

    pub advanced_features: AdvancedFeatureCapabilities,

    pub performance: PerformanceCapabilities,

    pub security: SecurityCapabilities,

    pub human_entropy: HumanEntropyCapabilities,

pub struct KeyGenerationCapabilities {

    pub supported_key_types: Vec<String>,

    pub max_key_sizes: HashMap<String, u32>,

    pub hardware_backed: bool,

    pub true_rng: bool,

    pub key_derivation: Vec<String>,

pub struct CryptoOperationCapabilities {

    pub signing_algorithms: Vec<String>,

    pub encryption_algorithms: Vec<String>,

    pub hash_functions: Vec<String>,

    pub mac_algorithms: Vec<String>,

    pub bulk_operations: bool,

    pub streaming: bool,

pub struct AdvancedFeatureCapabilities {

    pub key_attestation: bool,

    pub user_presence: bool,

    pub biometric_integration: bool,

    pub multi_party: bool,

    pub secure_boot: bool,

    pub tamper_resistance: TamperResistanceLevel,

pub struct PerformanceCapabilities {

    pub operations_per_second: f64,

    pub average_latency_ms: f64,

    pub concurrent_operations: u32,

    pub memory_usage: MemoryUsageLevel,

pub struct HumanEntropyCapabilities {

    pub ephemeral_seed_creation: bool,

    pub collection_methods: Vec<HumanEntropyMethod>,

    pub entropy_quality_assessment: bool,

    pub biometric_entropy: bool,

    pub behavioral_entropy: bool,

    pub realtime_entropy: bool,

pub enum HumanEntropyMethod {

    MouseMovement,

    KeystrokeDynamics,

    TouchPatterns,

    VoicePatterns,

    BiometricVariations,

    BehavioralTiming,

    CameraEntropy,

    CustomInput,

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]}

pub enum TamperResistanceLevel {

    Software,

    TamperEvident,

    TamperResistant,

    TamperResponsive,

pub enum MemoryUsageLevel {

    VeryLow,

    Low,

    Moderate,

    High,

    VeryHigh,

pub enum HardwareSecurityLevel {

    TrustedEnvironment,

    HardwareModule,

    SecureElement,

    StrongBox,

pub struct AuditCapabilities {

    pub comprehensive_logging: bool,

    pub tamper_evident_logs: bool,

    pub realtime_monitoring: bool,

    pub compliance_reporting: Vec<String>,

pub enum DiscoveryHsmHealthStatus {

    Healthy,

    Warning,

    Critical,

    Unavailable,

    Unknown,

impl Default for DiscoveryConfig {}

    fn default() -> Self {
        Self {
            enable_pkcs11: true,
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

    pub async fn new(config: DiscoveryConfig) -> BearDogResult<Self> {
        info!("🔍 Initializing Universal HSM Discovery Engine");
        let capability_detector = capability_detector::CapabilityDetector::new().await?;
        let entropy_classifier = human_entropy_classifier::HumanEntropyClassifier::new().await?;
        let tier_manager = tier_manager::TierManager::new().await?;
        Ok(Self {
            discovered_hsms: HashMap::with_capacity(16),
            capability_detector,
            entropy_classifier,
            tier_manager,
            config,
        })

    pub async fn discover_all_hsms(&mut self) -> BearDogResult<Vec<DiscoveredHsm>> {
        info!("🔍 Starting universal HSM discovery process");
        let mut all_discovered = Vec::new();

        if self.config.enable_pkcs11 {
            let pkcs11_hsms = self.discover_pkcs11_hsms().await?;
            all_discovered.extend(pkcs11_hsms);

        if self.config.enable_cloud_kms {
            let cloud_hsms = self.discover_cloud_kms_hsms().await?;
            all_discovered.extend(cloud_hsms);

        if self.config.enable_network_hsm {
            let network_hsms = self.discover_network_hsms().await?;
            all_discovered.extend(network_hsms);

        if self.config.enable_usb_hsm {
            let usb_hsms = self.discover_usb_hsms().await?;
            all_discovered.extend(usb_hsms);

        if self.config.enable_software_hsm {
            let software_hsms = self.discover_software_hsms().await?;
            all_discovered.extend(software_hsms);

        if self.config.enable_mobile_hsm {
            let mobile_hsms = self.discover_mobile_hsms().await?;
            all_discovered.extend(mobile_hsms);

        if self.config.enable_tpm {
            let tpm_hsms = self.discover_tpm_hsms().await?;
            all_discovered.extend(tpm_hsms);

        for hsm in &mut all_discovered {
            hsm.capabilities = self.capability_detector.detect_capabilities(&hsm.interface_type).await?;

            hsm.supports_human_entropy = self.entropy_classifier
                .classify_human_entropy_support(&hsm.capabilities).await?;

            hsm.assigned_tier = self.tier_manager
                .assign_tier(&hsm.capabilities, hsm.supports_human_entropy).await?;

        for hsm in &all_discovered {
            self.discovered_hsms.insert(hsm.hsm_id.clone(), hsm.clone());
        info!("✅ Universal HSM discovery completed: {} HSMs found", all_discovered.len());

        let human_entropy_count = all_discovered.iter()
            .filter(|h| h.supports_human_entropy)
            .count();
        if human_entropy_count > 0 {
            info!("🧠 {} HSMs support human entropy ephemeral seeds (tier elevated)", human_entropy_count);
        Ok(all_discovered)

    pub fn get_human_entropy_hsms(&self) -> Vec<&DiscoveredHsm> {
        self.discovered_hsms
            .values()
            .filter(|hsm| hsm.supports_human_entropy)
            .collect()

    pub fn get_hsms_by_tier(&self, tier: &HsmTier) -> Vec<&DiscoveredHsm> {
            .filter(|hsm| &hsm.assigned_tier == tier)

    pub async fn get_best_hsm_for_operation(&self, operation_type: &str) -> BearDogResult<Option<&DiscoveredHsm>> {
        self.tier_manager.select_best_hsm_for_operation(
            &self.discovered_hsms.values().collect::<Vec<_>>(),
            operation_type
        ).await

use crate::tunnel::hsm::types::HsmTier;

pub use capability_detector::CapabilityDetector;
pub use discovery_engine::DiscoveryEngine;
pub use human_entropy_classifier::HumanEntropyClassifier;
pub use tier_manager::TierManager;
pub use universal_adapter::UniversalAdapter; 
