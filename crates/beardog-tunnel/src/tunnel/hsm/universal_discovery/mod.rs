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


/// # Universal HSM Discovery System
///
/// This module provides automatic discovery and integration of any HSM type,
/// with intelligent capability detection and tier classification.

pub mod capability_detector;
pub mod discovery_engine;
pub mod human_entropy_classifier;
pub mod tier_manager;
pub mod universal_adapter;
use beardog_errors::{BearDogError, BearDogResult};
use std::collections::HashMap;
use tracing::{info, warn};
/// Universal HSM Discovery Engine
/// Automatically discovers and integrates any available HSMs on the system,
/// regardless of vendor, type, or interface.
pub struct UniversalHsmDiscovery {
    /// Discovered HSM instances
    discovered_hsms: HashMap<String, DiscoveredHsm>,
    /// Capability detection engine
    capability_detector: capability_detector::CapabilityDetector,
    /// Human entropy classifier for tier elevation
    entropy_classifier: human_entropy_classifier::HumanEntropyClassifier,
    /// Tier management system
    tier_manager: tier_manager::TierManager,
    /// Discovery configuration
    config: DiscoveryConfig,
}
/// Discovered HSM information
#[derive(Debug, Clone)]
pub struct DiscoveredHsm {
    /// Unique HSM identifier
    pub hsm_id: String,
    /// HSM vendor/manufacturer
    pub vendor: String,
    /// HSM model/type
    pub model: String,
    /// HSM interface type (PKCS#11, KMS, Custom API, etc.)
    pub interface_type: HsmInterfaceType,
    /// Connection information
    pub connection_info: HsmConnectionInfo,
    /// Detected capabilities
    pub capabilities: UniversalHsmCapabilities,
    /// Assigned tier based on capabilities
    pub assigned_tier: HsmTier,
    /// Whether this HSM supports human entropy ephemeral seeds
    pub supports_human_entropy: bool,
    /// HSM health status
    pub health_status: HsmHealthStatus,
    /// Discovery timestamp
    pub discovered_at: chrono::DateTime<chrono::Utc>,
/// HSM Interface Types that can be auto-discovered
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HsmInterfaceType {
    /// PKCS#11 interface (most common)
    Pkcs11 { library_path: String },
    /// Cloud KMS (AWS, Azure, GCP)
    CloudKms { provider: String, region: Option<String> },
    /// Network-attached HSM
    NetworkHsm { endpoint: String, port: u16 },
    /// USB-attached HSM
    UsbHsm { device_id: String },
    /// Software HSM
    SoftwareHsm { implementation: String },
    /// Mobile HSM (Android StrongBox, iOS Secure Enclave)
    MobileHsm { platform: String, chip: Option<String> },
    /// Custom vendor API
    CustomApi { api_type: String, endpoint: String },
    /// TPM (Trusted Platform Module)
    Tpm { version: String },
    /// Smart card interface
    SmartCard { reader: String },
/// HSM Connection Information
pub struct HsmConnectionInfo {
    /// Primary connection endpoint
    pub endpoint: String,
    /// Authentication method
    pub auth_method: AuthenticationMethod,
    /// Connection timeout
    pub timeout_ms: u32,
    /// Whether connection is encrypted
    pub encrypted: bool,
    /// Additional connection parameters
    pub parameters: HashMap<String, String>,
/// Authentication methods supported by discovered HSMs
pub enum AuthenticationMethod {
    /// Username/password authentication
    UsernamePassword { username: String },
    /// Certificate-based authentication
    Certificate { cert_path: String },
    /// API key authentication
    ApiKey { key_id: String },
    /// Hardware token/smart card
    HardwareToken { token_id: String },
    /// Biometric authentication
    Biometric { method: String },
    /// No authentication required
    None,
/// Comprehensive HSM capabilities (discovery-specific)
pub struct DiscoveryHsmCapabilities {
    /// Key generation capabilities
    pub key_generation: KeyGenerationCapabilities,
    /// Cryptographic operations
    pub crypto_operations: CryptoOperationCapabilities,
    /// Advanced features
    pub advanced_features: AdvancedFeatureCapabilities,
    /// Performance characteristics
    pub performance: PerformanceCapabilities,
    /// Security features
    pub security: SecurityCapabilities,
    /// Human entropy capabilities (tier-determining)
    pub human_entropy: HumanEntropyCapabilities,
/// Key generation capabilities
pub struct KeyGenerationCapabilities {
    /// Supported key types
    pub supported_key_types: Vec<String>,
    /// Maximum key sizes
    pub max_key_sizes: HashMap<String, u32>,
    /// Hardware-backed generation
    pub hardware_backed: bool,
    /// True random number generation
    pub true_rng: bool,
    /// Key derivation functions
    pub key_derivation: Vec<String>,
/// Cryptographic operation capabilities
pub struct CryptoOperationCapabilities {
    /// Supported signing algorithms
    pub signing_algorithms: Vec<String>,
    /// Supported encryption algorithms
    pub encryption_algorithms: Vec<String>,
    /// Hash functions
    pub hash_functions: Vec<String>,
    /// MAC algorithms
    pub mac_algorithms: Vec<String>,
    /// Bulk operations support
    pub bulk_operations: bool,
    /// Streaming operations
    pub streaming: bool,
/// Advanced feature capabilities
pub struct AdvancedFeatureCapabilities {
    /// Key attestation support
    pub key_attestation: bool,
    /// User presence validation
    pub user_presence: bool,
    /// Biometric integration
    pub biometric_integration: bool,
    /// Multi-party operations
    pub multi_party: bool,
    /// Secure boot integration
    pub secure_boot: bool,
    /// Hardware tamper resistance
    pub tamper_resistance: TamperResistanceLevel,
/// Performance characteristics
pub struct PerformanceCapabilities {
    /// Operations per second (estimated)
    pub operations_per_second: f64,
    /// Average latency in milliseconds
    pub average_latency_ms: f64,
    /// Concurrent operation support
    pub concurrent_operations: u32,
    /// Memory usage characteristics
    pub memory_usage: MemoryUsageLevel,
/// Security capabilities
/// Human entropy capabilities (determines tier elevation)
pub struct HumanEntropyCapabilities {
    /// Supports human entropy ephemeral seed creation
    pub ephemeral_seed_creation: bool,
    /// Human randomness collection methods
    pub collection_methods: Vec<HumanEntropyMethod>,
    /// Entropy quality assessment
    pub entropy_quality_assessment: bool,
    /// Biometric entropy integration
    pub biometric_entropy: bool,
    /// Behavioral entropy patterns
    pub behavioral_entropy: bool,
    /// Real-time entropy generation
    pub realtime_entropy: bool,
/// Human entropy collection methods
pub enum HumanEntropyMethod {
    /// Mouse movement patterns
    MouseMovement,
    /// Keystroke dynamics
    KeystrokeDynamics,
    /// Touch patterns (mobile)
    TouchPatterns,
    /// Voice patterns
    VoicePatterns,
    /// Biometric variations
    BiometricVariations,
    /// Behavioral timing
    BehavioralTiming,
    /// Camera-based entropy
    CameraEntropy,
    /// Custom human input
    CustomInput,
/// Tamper resistance levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]}


pub enum TamperResistanceLevel {
    /// No tamper resistance
    /// Software-based protection
    Software,
    /// Tamper-evident mechanisms
    TamperEvident,
    /// Tamper-resistant hardware
    TamperResistant,
    /// Tamper-responsive (actively responds to attacks)
    TamperResponsive,
/// Memory usage characteristics
pub enum MemoryUsageLevel {
    /// Very low memory usage (< 1MB)
    VeryLow,
    /// Low memory usage (1-10MB)
    Low,
    /// Moderate memory usage (10-100MB)
    Moderate,
    /// High memory usage (100MB-1GB)
    High,
    /// Very high memory usage (> 1GB)
    VeryHigh,
/// Hardware security levels}


pub enum HardwareSecurityLevel {
    /// Software-only implementation
    /// Trusted Execution Environment
    TrustedEnvironment,
    /// Hardware Security Module
    HardwareModule,
    /// Secure Element
    SecureElement,
    /// StrongBox (highest level)
    StrongBox,
/// Audit capabilities
pub struct AuditCapabilities {
    /// Comprehensive audit logging
    pub comprehensive_logging: bool,
    /// Tamper-evident logs
    pub tamper_evident_logs: bool,
    /// Real-time monitoring
    pub realtime_monitoring: bool,
    /// Compliance reporting
    pub compliance_reporting: Vec<String>,
/// HSM Health Status (discovery-specific, renamed to avoid conflict)}


pub enum DiscoveryHsmHealthStatus {
    /// HSM is healthy and operational
    Healthy,
    /// HSM has minor issues but is operational
    Warning,
    /// HSM has critical issues
    Critical,
    /// HSM is not responding
    Unavailable,
    /// HSM status unknown
    Unknown,
/// Discovery configuration}


// MIGRATED: DiscoveryConfig -> use beardog_types::config::UnifiedDiscoveryConfig;


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
    /// Create a new universal HSM discovery engine
    pub async fn new(config: DiscoveryConfig) -> BearDogResult<Self> {
        info!("🔍 Initializing Universal HSM Discovery Engine");
        let capability_detector = capability_detector::CapabilityDetector::new().await?;
        let entropy_classifier = human_entropy_classifier::HumanEntropyClassifier::new().await?;
        let tier_manager = tier_manager::TierManager::new().await?;
        Ok(Self {
            discovered_hsms: HashMap::new(),
            capability_detector,
            entropy_classifier,
            tier_manager,
            config,
        })
    /// Discover all available HSMs on the system
    pub async fn discover_all_hsms(&mut self) -> BearDogResult<Vec<DiscoveredHsm>> {
        info!("🔍 Starting universal HSM discovery process");
        let mut all_discovered = Vec::new();
        // Discover PKCS#11 HSMs
        if self.config.enable_pkcs11 {
            let pkcs11_hsms = self.discover_pkcs11_hsms().await?;
            all_discovered.extend(pkcs11_hsms);
        // Discover Cloud KMS instances
        if self.config.enable_cloud_kms {
            let cloud_hsms = self.discover_cloud_kms_hsms().await?;
            all_discovered.extend(cloud_hsms);
        // Discover Network HSMs
        if self.config.enable_network_hsm {
            let network_hsms = self.discover_network_hsms().await?;
            all_discovered.extend(network_hsms);
        // Discover USB HSMs
        if self.config.enable_usb_hsm {
            let usb_hsms = self.discover_usb_hsms().await?;
            all_discovered.extend(usb_hsms);
        // Discover Software HSMs
        if self.config.enable_software_hsm {
            let software_hsms = self.discover_software_hsms().await?;
            all_discovered.extend(software_hsms);
        // Discover Mobile HSMs (Android StrongBox, iOS Secure Enclave)
        if self.config.enable_mobile_hsm {
            let mobile_hsms = self.discover_mobile_hsms().await?;
            all_discovered.extend(mobile_hsms);
        // Discover TPMs
        if self.config.enable_tpm {
            let tpm_hsms = self.discover_tpm_hsms().await?;
            all_discovered.extend(tpm_hsms);
        // Detect capabilities for all discovered HSMs
        for hsm in &mut all_discovered {
            hsm.capabilities = self.capability_detector.detect_capabilities(&hsm.interface_type).await?;
            
            // Classify human entropy capabilities
            hsm.supports_human_entropy = self.entropy_classifier
                .classify_human_entropy_support(&hsm.capabilities).await?;
            // Assign tier based on capabilities (with human entropy elevation)
            hsm.assigned_tier = self.tier_manager
                .assign_tier(&hsm.capabilities, hsm.supports_human_entropy).await?;
        // Store discovered HSMs
        for hsm in &all_discovered {
            self.discovered_hsms.insert(hsm.hsm_id.clone(), hsm.clone());
        info!("✅ Universal HSM discovery completed: {} HSMs found", all_discovered.len());
        
        // Log human entropy capable HSMs
        let human_entropy_count = all_discovered.iter()
            .filter(|h| h.supports_human_entropy)
            .count();
        if human_entropy_count > 0 {
            info!("🧠 {} HSMs support human entropy ephemeral seeds (tier elevated)", human_entropy_count);
        Ok(all_discovered)
    /// Get all HSMs that support human entropy ephemeral seeds
    pub fn get_human_entropy_hsms(&self) -> Vec<&DiscoveredHsm> {
        self.discovered_hsms
            .values()
            .filter(|hsm| hsm.supports_human_entropy)
            .collect()
    /// Get HSMs by tier}


    pub fn get_hsms_by_tier(&self, tier: &HsmTier) -> Vec<&DiscoveredHsm> {
            .filter(|hsm| &hsm.assigned_tier == tier)
    /// Get the best HSM for a specific operation type
    pub async fn get_best_hsm_for_operation(&self, operation_type: &str) -> BearDogResult<Option<&DiscoveredHsm>> {
        self.tier_manager.select_best_hsm_for_operation(
            &self.discovered_hsms.values().collect::<Vec<_>>(),
            operation_type
        ).await
/// Import HSM tier type from main types module
use crate::tunnel::hsm::types::HsmTier;
// CANONICAL IMPORT: use beardog_types::config::UnifiedDiscoveryConfig;
/// Re-export discovery components
pub use capability_detector::CapabilityDetector;
pub use discovery_engine::DiscoveryEngine;
pub use human_entropy_classifier::HumanEntropyClassifier;
pub use tier_manager::TierManager;
pub use universal_adapter::UniversalAdapter; 
