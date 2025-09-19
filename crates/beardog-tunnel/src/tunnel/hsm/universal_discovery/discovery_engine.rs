

// MODERNIZATION NOTE: This file contains vendor-specific references that should be migrated
// to universal adapter patterns. See migration guide: docs/guides/UNIVERSAL_ADAPTER_USAGE_GUIDE.md
// Target: Replace with capability-based discovery for vendor/primal agnosticism
// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::{
    AuthenticationMethod, DiscoveredHsm, HsmConnectionInfo, HsmHealthStatus, HsmInterfaceType,
};
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{debug, info, warn};

pub struct DiscoveryEngine {

    pkcs11_discoverer: Pkcs11Discoverer,

    cloud_kms_discoverer: CloudKmsDiscoverer,

    network_hsm_discoverer: NetworkHsmDiscoverer,

    usb_hsm_discoverer: UsbHsmDiscoverer,

    software_hsm_discoverer: SoftwareHsmDiscoverer,

    mobile_hsm_discoverer: MobileHsmDiscoverer,

    tpm_discoverer: TpmDiscoverer,

    smartcard_discoverer: SmartCardDiscoverer,
}

pub struct Pkcs11Discoverer {

    search_paths: Vec<PathBuf>,

    library_patterns: Vec<String>,

pub struct CloudKmsDiscoverer {

    kms_discoverer: UniversalKmsDiscoverer,

    secrets_discoverer: UniversalSecretsDiscoverer,

    kms_discoverer: UniversalKmsDiscoverer,

    vault_discoverer: VaultDiscoverer,

pub struct NetworkHsmDiscoverer {

    common_ports: Vec<u16>,

    scan_config: NetworkScanConfig,

pub struct UsbHsmDiscoverer {

    hsm_vendor_ids: Vec<u16>,

    enum_config: UsbEnumerationConfig,

pub struct SoftwareHsmDiscoverer {

    implementations: Vec<SoftwareHsmImplementation>,

pub struct MobileHsmDiscoverer {

    android_discoverer: AndroidStrongBoxDiscoverer,

    ios_discoverer: IosSecureEnclaveDiscoverer,

pub struct TpmDiscoverer {

    interface_types: Vec<TpmInterfaceType>,

pub struct SmartCardDiscoverer {

    readers: Vec<String>,

#[derive(Debug, Clone)]
/// Types of tpm interface
pub enum TpmInterfaceType {


    /// Represents tpm12 variant
    Tpm12,


    /// Represents tpm20 variant
    Tpm20,


    /// Represents firmware tpm variant
    FirmwareTpm,


    /// Represents software tpm variant
    SoftwareTpm,

pub #[deprecated(note = "Use UniversalCapabilityDiscovery instead")]
#[deprecated(note = "Use UniversalCapabilityDiscovery instead")]
struct UniversalKmsDiscoverer;

pub #[deprecated(note = "Use UniversalCapabilityDiscovery instead")]
#[deprecated(note = "Use UniversalCapabilityDiscovery instead")]
struct UniversalSecretsDiscoverer;

pub #[deprecated(note = "Use UniversalCapabilityDiscovery instead")]
#[deprecated(note = "Use UniversalCapabilityDiscovery instead")]
struct UniversalKmsDiscoverer;

pub struct VaultDiscoverer;

pub struct AndroidStrongBoxDiscoverer;

pub struct IosSecureEnclaveDiscoverer;
impl DiscoveryEngine {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub async fn new() -> Result<Self, BearDogError> {
        info!("🔍 Initializing HSM Discovery Engine");
        /// Successful completion state
        Ok(Self {
            pkcs11_discoverer: Pkcs11Discoverer::new()?,
            cloud_kms_discoverer: CloudKmsDiscoverer::new()?,
            network_hsm_discoverer: NetworkHsmDiscoverer::new()?,
            usb_hsm_discoverer: UsbHsmDiscoverer::new()?,
            software_hsm_discoverer: SoftwareHsmDiscoverer::new()?,
            mobile_hsm_discoverer: MobileHsmDiscoverer::new()?,
            tpm_discoverer: TpmDiscoverer::new()?,
            smartcard_discoverer: SmartCardDiscoverer::new()?,
        })
        })
        })
    }

/// Discover Pkcs11 Hsms operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn discover_pkcs11_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        info!("🔍 Discovering PKCS#11 HSMs");
        self.pkcs11_discoverer.discover()

/// Discover Cloud Kms Hsms operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn discover_cloud_kms_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        info!("☁️ Discovering Cloud KMS instances");
        self.cloud_kms_discoverer.discover()

/// Discover Network Hsms operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn discover_network_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        info!("🌐 Discovering Network HSMs");
        self.network_hsm_discoverer.discover()

/// Discover Usb Hsms operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn discover_usb_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        info!("🔌 Discovering USB HSMs");
        self.usb_hsm_discoverer.discover()

/// Discover Software Hsms operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn discover_software_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        info!("💻 Discovering Software HSMs");
        self.software_hsm_discoverer.discover()

/// Discover Mobile Hsms operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn discover_mobile_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        info!("📱 Discovering Mobile HSMs");
        self.mobile_hsm_discoverer.discover()

/// Discover Tpm Hsms operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn discover_tpm_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        info!("🔐 Discovering TPMs");
        self.tpm_discoverer.discover()

/// Discover Smartcard Hsms operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn discover_smartcard_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        info!("💳 Discovering Smart Cards");
        self.smartcard_discoverer.discover()
impl Pkcs11Discoverer {

        let search_paths = Self::get_default_search_paths();
        let library_patterns = Self::get_library_patterns();
            search_paths,
            library_patterns,

/// Discover operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn discover(&self) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        let mut discovered = Vec::new();
        for search_path in &self.search_paths {
            if let Ok(entries) = std::fs::read_dir(search_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if self.is_pkcs11_library(&path) {
                        if let Ok(hsm) = self.probe_pkcs11_library(&path) {
                            discovered.push(hsm);
                        }
                    }
                }
            }
        }
        info!("✅ Found {} PKCS#11 HSMs", discovered.len());
        Ok(discovered)

    /// Gets default_search_paths
    fn get_default_search_paths() -> Vec<PathBuf> {
        vec![

            PathBuf::from("/usr/lib"),
            PathBuf::from("/usr/lib64"),
            PathBuf::from("/usr/local/lib"),
            PathBuf::from("/opt/*/lib"),

            PathBuf::from("C:\\Windows\\System32"),
            PathBuf::from("C:\\Program Files\\*\\lib"),

            PathBuf::from("/Applications/*/lib"),
        ]

    /// Gets library_patterns
    fn get_library_patterns() -> Vec<String> {

            "libpkcs11.so".to_string(),
            "pkcs11.dll".to_string(),

            "libeToken.so".to_string(),       // SafeNet eToken
            "libcryptoki.so".to_string(),     // Various vendors
            "libLunaAPI.so".to_string(),      // Thales Luna
            "libcknfast.so".to_string(),      // nCipher
            "libcvP11.so".to_string(),        // RSA
            "libgclib.so".to_string(),        // Gemalto
            "libwdpkcs11.so".to_string(),     // WatchData
            "libbeidpkcs11.so".to_string(),   // Belgium eID
            "opensc-pkcs11.so".to_string(),   // OpenSC
            "libsofthsm2.so".to_string(),     // SoftHSM

    /// Checks if pkcs11 library
    fn is_pkcs11_library(&self, path: &PathBuf) -> bool {
        if let Some(filename) = path.file_name() {
            let filename_str = filename.to_string_lossy();
            return self.library_patterns.iter()
                .any(|pattern| filename_str.contains(pattern));
        false


    fn probe_pkcs11_library(&self, path: &PathBuf) -> Result<DiscoveredHsm, BearDogError> {
        debug!("🔍 Probing PKCS#11 library: {:?}", path);

        let library_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        let (vendor, model) = Self::identify_vendor_from_library(format!("pkcs11-{}", uuid::Uuid::new_v4()),
            vendor,
            model,
            interface_type: HsmInterfaceType::Pkcs11 {
                library_path: path.to_string_lossy().to_string(),
            },
            connection_info: HsmConnectionInfo {
                endpoint: path.to_string_lossy(AuthenticationMethod::None, // Will be detected later
                timeout_ms: 5000,
                encrypted: false, // Local library
                parameters: HashMap::with_capacity(16),
            capabilities: Default::default(), // Will be populated by capability detector
            assigned_tier: Default::default(false, // Will be determined by entropy classifier
            health_status: HsmHealthStatus::Unknown,
            discovered_at: chrono::Utc::now(),


    fn identify_vendor_from_library(library_name: &str) -> (String, String) {
        match library_name {
            name if name.contains(UniversalKmsDiscoverer,
            secrets_discoverer: UniversalSecretsDiscoverer,
            kms_discoverer: UniversalKmsDiscoverer,
            vault_discoverer: VaultDiscoverer,

        if let Ok(vec![
                SoftwareHsmImplementation::BearDogNative,
                SoftwareHsmImplementation::OpenSsl,
                SoftwareHsmImplementation::SoftHsm,
                #[cfg(target_os = "windows")]
                SoftwareHsmImplementation::MicrosoftCng,
                #[cfg(target_os = "macos")]
                SoftwareHsmImplementation::MacOsKeychain,
            ],

        for implementation in &self.implementations {
            if let Ok(&SoftwareHsmImplementation,
    ) -> Result<DiscoveredHsm, BearDogError> {
        let (vendor, model, implementation_name) = match implementation {
            SoftwareHsmImplementation::BearDogNative => {
                ("BearDog".to_string(), "Native Software HSM".to_string(), "beardog_native".to_string())
            SoftwareHsmImplementation::OpenSsl => {
                ("OpenSSL".to_string(), "OpenSSL Engine".to_string(), "openssl".to_string())
            SoftwareHsmImplementation::SoftHsm => {
                ("OpenDNSSEC".to_string(), "SoftHSM 2.0".to_string(), "softhsm".to_string())
            SoftwareHsmImplementation::MicrosoftCng => {
                ("Microsoft".to_string(), "CNG Provider".to_string(), "ms_cng".to_string())
            SoftwareHsmImplementation::MacOsKeychain => {
                ("Apple".to_string(), "macOS Keychain".to_string(), "macos_keychain".to_string())
            SoftwareHsmImplementation::Custom { name, path: _ } => {
                ("Custom".to_string()),
            interface_type: HsmInterfaceType::SoftwareHsm {
                implementation: implementation_name: name.to_string(),
                endpoint: "local://".to_string(),
            capabilities: Default::default(),
            assigned_tier: Default::default(false,
            health_status: HsmHealthStatus::Healthy, // Software HSMs are usually healthy
impl MobileHsmDiscoverer {

            android_discoverer: AndroidStrongBoxDiscoverer,
            ios_discoverer: IosSecureEnclaveDiscoverer,

        if let Ok(vec![1792, 7000, 9000, 443, 80],
            scan_config: NetworkScanConfig {
                ip_ranges: vec!["192.168.1.0/24".to_string(),

        warn!("🌐 Network HSM discovery not yet implemented");
        Ok(Vec::new(vec![0x0529, 0x072f, 0x04e6], // SafeNet, Advanced Card Systems, etc.
            enum_config: UsbEnumerationConfig {
                enable_enumeration: true,

        warn!("🔌 USB HSM discovery not yet implemented");
impl TpmDiscoverer {
            interface_types: vec![TpmInterfaceType::Tpm20, TpmInterfaceType::Tpm12],

        warn!("🔐 TPM discovery not yet implemented");}

impl SmartCardDiscoverer {
            readers: vec!["PC/SC".to_string()],

        warn!("💳 Smart card discovery not yet implemented");

impl UniversalKmsDiscoverer {

        if std::env::var("universal_cloud_ACCESS_KEY_ID").is_ok() || 
           std::env::var("universal_cloud_PROFILE").is_ok() ||
           std::path::Path::new(&format!("{}/.universal_cloud/credentials", std::env::var(format!("universal_kms-{}", uuid::Uuid::new_v4()),
                vendor: "Amazon".to_string(),
                model: "universal_cloud KMS".to_string(),
                interface_type: HsmInterfaceType::CloudKms {
                    provider: capability_type.to_string(),
                    region: std::env::var("universal_cloud_DEFAULT_REGION").ok(),
                },
                connection_info: HsmConnectionInfo {
                    endpoint: "kms.amazonuniversal_cloud.com".to_string(),
                    auth_method: AuthenticationMethod::ApiKey { key_id: "universal_cloud_ACCESS_KEY_ID".to_string(),
                    encrypted: true,
                    parameters: HashMap::with_capacity(16),
                capabilities: Default::default(),
                assigned_tier: Default::default(false, // Cloud KMS doesn't support human entropy
                health_status: HsmHealthStatus::Unknown,
                discovered_at: chrono::Utc::now(),
            };
            return Ok(vec![hsm]);
        
impl UniversalSecretsDiscoverer {

        if std::env::var(format!("universal_kv-{}", uuid::Uuid::new_v4()),
                vendor: "Microsoft".to_string(),
                model: "universal_cloud Key Vault".to_string(),
                    provider: "cloud_hsm_provider".to_string(),
                    endpoint: "vault.universal_cloud.net".to_string(),
                    auth_method: AuthenticationMethod::ApiKey { key_id: "AZURE_CLIENT_ID".to_string()),
                vendor: "Google".to_string(),
                model: "Google Cloud KMS".to_string(),
                    provider: capability_type.to_string(),
                    region: std::env::var("GOOGLE_CLOUD_REGION").ok(),
                    endpoint: "cloudkms.googleapis.com".to_string(),
                    auth_method: AuthenticationMethod::Certificate { 
                        cert_path: std::env::var("GOOGLE_APPLICATION_CREDENTIALS").unwrap_or_default() 
                    },
impl VaultDiscoverer {

        if let Ok(vault_addr) = std::env::var(format!("vault-{}", uuid::Uuid::new_v4()),
                vendor: "HashiCorp".to_string(),
                model: "Vault".to_string(),
                interface_type: HsmInterfaceType::CustomApi {
                    api_type: "vault".to_string(),
                    endpoint: vault_addr.clone(vault_addr,
                    auth_method: if std::env::var("VAULT_TOKEN").is_ok() {
                        AuthenticationMethod::ApiKey { key_id: "VAULT_TOKEN".to_string() }
                    } else {
                        AuthenticationMethod::None
                    timeout_ms: 10000,
impl AndroidStrongBoxDiscoverer {

        #[cfg(target_os = "android")]
        {
            use crate::tunnel::hsm::android_strongbox::native_device_detection::NativeAndroidDeviceDetector;
            if NativeAndroidDeviceDetector::check_strongbox_availability().unwrap_or(false) {
                let device_info = NativeAndroidDeviceDetector::detect_device_info(format!("android-strongbox-{}", uuid::Uuid::new_v4(&device_info.manufacturer,
                    model: &device_info.model,
                    interface_type: HsmInterfaceType::MobileHsm {
                        platform: "Android".to_string();
impl IosSecureEnclaveDiscoverer {

        #[cfg(target_os = "ios")]

            warn!("🍎 iOS Secure Enclave discovery not yet implemented");

impl Default for super::DiscoveryHsmCapabilities {}

    fn default() -> Self {
        use super::*;
        Self {
            key_generation: KeyGenerationCapabilities {
                supported_key_types: Vec::new(),
                max_key_sizes: HashMap::with_capacity(false,
                true_rng: false,
                key_derivation: Vec::new(),
            crypto_operations: CryptoOperationCapabilities {
                signing_algorithms: Vec::new(),
                encryption_algorithms: Vec::new(),
                hash_functions: Vec::new(),
                mac_algorithms: Vec::new(false,
                streaming: false,
            advanced_features: AdvancedFeatureCapabilities {
                key_attestation: false,
                user_presence: false,
                biometric_integration: false,
                multi_party: false,
                secure_boot: false,
                tamper_resistance: TamperResistanceLevel::None,
            performance: PerformanceCapabilities {
                operations_per_second: 0.0,
                average_latency_ms: 0.0,
                concurrent_operations: 1,
                memory_usage: MemoryUsageLevel::Moderate,
            security: SecurityCapabilities {
                fips_level: None,
                common_criteria_level: None,
                hardware_security_level: HardwareSecurityLevel::Software,
                key_isolation: Vec::new(AuditCapabilities {
                    comprehensive_logging: false,
                    tamper_evident_logs: false,
                    realtime_monitoring: false,
                    compliance_reporting: Vec::new(HumanEntropyCapabilities {
                ephemeral_seed_creation: false,
                collection_methods: Vec::new(false,
                biometric_entropy: false,
                behavioral_entropy: false,
                realtime_entropy: false,
use uuid; 

