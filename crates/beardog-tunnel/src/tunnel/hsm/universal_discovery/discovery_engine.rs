

use super::{
    AuthenticationMethod, DiscoveredHsm, HsmConnectionInfo, HsmHealthStatus, HsmInterfaceType,
};
use beardog_errors::{BearDogError, BearDogResult};
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

    aws_discoverer: AwsKmsDiscoverer,

    azure_discoverer: AzureKeyVaultDiscoverer,

    gcp_discoverer: GcpKmsDiscoverer,

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

pub enum TpmInterfaceType {

    Tpm12,

    Tpm20,

    FirmwareTpm,

    SoftwareTpm,

pub struct AwsKmsDiscoverer;

pub struct AzureKeyVaultDiscoverer;

pub struct GcpKmsDiscoverer;

pub struct VaultDiscoverer;

pub struct AndroidStrongBoxDiscoverer;

pub struct IosSecureEnclaveDiscoverer;
impl DiscoveryEngine {

    pub async fn new() -> BearDogResult<Self> {
        info!("🔍 Initializing HSM Discovery Engine");
        Ok(Self {
            pkcs11_discoverer: Pkcs11Discoverer::new().await?,
            cloud_kms_discoverer: CloudKmsDiscoverer::new().await?,
            network_hsm_discoverer: NetworkHsmDiscoverer::new().await?,
            usb_hsm_discoverer: UsbHsmDiscoverer::new().await?,
            software_hsm_discoverer: SoftwareHsmDiscoverer::new().await?,
            mobile_hsm_discoverer: MobileHsmDiscoverer::new().await?,
            tpm_discoverer: TpmDiscoverer::new().await?,
            smartcard_discoverer: SmartCardDiscoverer::new().await?,
        })
    }

    pub async fn discover_pkcs11_hsms(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        info!("🔍 Discovering PKCS#11 HSMs");
        self.pkcs11_discoverer.discover().await

    pub async fn discover_cloud_kms_hsms(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        info!("☁️ Discovering Cloud KMS instances");
        self.cloud_kms_discoverer.discover().await

    pub async fn discover_network_hsms(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        info!("🌐 Discovering Network HSMs");
        self.network_hsm_discoverer.discover().await

    pub async fn discover_usb_hsms(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        info!("🔌 Discovering USB HSMs");
        self.usb_hsm_discoverer.discover().await

    pub async fn discover_software_hsms(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        info!("💻 Discovering Software HSMs");
        self.software_hsm_discoverer.discover().await

    pub async fn discover_mobile_hsms(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        info!("📱 Discovering Mobile HSMs");
        self.mobile_hsm_discoverer.discover().await

    pub async fn discover_tpm_hsms(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        info!("🔐 Discovering TPMs");
        self.tpm_discoverer.discover().await

    pub async fn discover_smartcard_hsms(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        info!("💳 Discovering Smart Cards");
        self.smartcard_discoverer.discover().await
impl Pkcs11Discoverer {

        let search_paths = Self::get_default_search_paths();
        let library_patterns = Self::get_library_patterns();
            search_paths,
            library_patterns,

    pub async fn discover(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        let mut discovered = Vec::new();
        for search_path in &self.search_paths {
            if let Ok(entries) = std::fs::read_dir(search_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if self.is_pkcs11_library(&path) {
                        if let Ok(hsm) = self.probe_pkcs11_library(&path).await {
                            discovered.push(hsm);
                        }
                    }
                }
            }
        }
        info!("✅ Found {} PKCS#11 HSMs", discovered.len());
        Ok(discovered)

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

    fn is_pkcs11_library(&self, path: &PathBuf) -> bool {
        if let Some(filename) = path.file_name() {
            let filename_str = filename.to_string_lossy();
            return self.library_patterns.iter()
                .any(|pattern| filename_str.contains(pattern));
        false

    async fn probe_pkcs11_library(&self, path: &PathBuf) -> BearDogResult<DiscoveredHsm> {
        debug!("🔍 Probing PKCS#11 library: {:?}", path);

        let library_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        let (vendor, model) = Self::identify_vendor_from_library(&library_name);
        Ok(DiscoveredHsm {
            hsm_id: format_args!("pkcs11-{}", uuid::Uuid::new_v4().to_string()),
            vendor,
            model,
            interface_type: HsmInterfaceType::Pkcs11 {
                library_path: path.to_string_lossy().to_string(),
            },
            connection_info: HsmConnectionInfo {
                endpoint: path.to_string_lossy().to_string(),
                auth_method: AuthenticationMethod::None, // Will be detected later
                timeout_ms: 5000,
                encrypted: false, // Local library
                parameters: HashMap::with_capacity(16),
            capabilities: Default::default(), // Will be populated by capability detector
            assigned_tier: Default::default(), // Will be assigned by tier manager
            supports_human_entropy: false, // Will be determined by entropy classifier
            health_status: HsmHealthStatus::Unknown,
            discovered_at: chrono::Utc::now(),

    fn identify_vendor_from_library(library_name: &str) -> (String, String) {
        match library_name {
            name if name.contains("eToken") => ("SafeNet".to_string(), "eToken".to_string()),
            name if name.contains("Luna") => ("Thales".to_string(), "Luna HSM".to_string()),
            name if name.contains("nfast") => ("nCipher".to_string(), "nShield".to_string()),
            name if name.contains("cvP11") => ("RSA".to_string(), "RSA HSM".to_string()),
            name if name.contains("gclib") => ("Gemalto".to_string(), "Gemalto HSM".to_string()),
            name if name.contains("softhsm") => ("OpenDNSSEC".to_string(), "SoftHSM".to_string()),
            name if name.contains("opensc") => ("OpenSC".to_string(), "OpenSC".to_string()),
            _ => ("Unknown".to_string(), "PKCS#11 HSM".to_string()),}

impl CloudKmsDiscoverer {

            aws_discoverer: AwsKmsDiscoverer,
            azure_discoverer: AzureKeyVaultDiscoverer,
            gcp_discoverer: GcpKmsDiscoverer,
            vault_discoverer: VaultDiscoverer,

        if let Ok(aws_hsms) = self.aws_discoverer.discover().await {
            discovered.extend(aws_hsms);

        if let Ok(azure_hsms) = self.azure_discoverer.discover().await {
            discovered.extend(azure_hsms);

        if let Ok(gcp_hsms) = self.gcp_discoverer.discover().await {
            discovered.extend(gcp_hsms);

        if let Ok(vault_hsms) = self.vault_discoverer.discover().await {
            discovered.extend(vault_hsms);
        info!("✅ Found {} Cloud KMS instances", discovered.len());
impl SoftwareHsmDiscoverer {

            implementations: vec![
                SoftwareHsmImplementation::BearDogNative,
                SoftwareHsmImplementation::OpenSsl,
                SoftwareHsmImplementation::SoftHsm,
                #[cfg(target_os = "windows")]
                SoftwareHsmImplementation::MicrosoftCng,
                #[cfg(target_os = "macos")]
                SoftwareHsmImplementation::MacOsKeychain,
            ],

        for implementation in &self.implementations {
            if let Ok(hsm) = self.probe_software_hsm(implementation).await {
                discovered.push(hsm);
        info!("✅ Found {} Software HSMs", discovered.len());

    async fn probe_software_hsm(
        &self,
        implementation: &SoftwareHsmImplementation,
    ) -> BearDogResult<DiscoveredHsm> {
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
                ("Custom".to_string(), name.clone(), name.clone())
        };
            hsm_id: format_args!("software-{}", uuid::Uuid::new_v4().to_string()),
            interface_type: HsmInterfaceType::SoftwareHsm {
                implementation: implementation_name,
                endpoint: "local://".to_string(),
                auth_method: AuthenticationMethod::None,
                timeout_ms: 1000,
                encrypted: false,
            capabilities: Default::default(),
            assigned_tier: Default::default(),
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy, // Software HSMs are usually healthy
impl MobileHsmDiscoverer {

            android_discoverer: AndroidStrongBoxDiscoverer,
            ios_discoverer: IosSecureEnclaveDiscoverer,

        if let Ok(android_hsms) = self.android_discoverer.discover().await {
            discovered.extend(android_hsms);

        if let Ok(ios_hsms) = self.ios_discoverer.discover().await {
            discovered.extend(ios_hsms);
        info!("✅ Found {} Mobile HSMs", discovered.len());

impl NetworkHsmDiscoverer {
            common_ports: vec![1792, 7000, 9000, 443, 80],
            scan_config: NetworkScanConfig {
                ip_ranges: vec!["192.168.1.0/24".to_string()],
                max_concurrent: 10,

        warn!("🌐 Network HSM discovery not yet implemented");
        Ok(Vec::new())}

impl UsbHsmDiscoverer {
            hsm_vendor_ids: vec![0x0529, 0x072f, 0x04e6], // SafeNet, Advanced Card Systems, etc.
            enum_config: UsbEnumerationConfig {
                enable_enumeration: true,

        warn!("🔌 USB HSM discovery not yet implemented");
impl TpmDiscoverer {
            interface_types: vec![TpmInterfaceType::Tpm20, TpmInterfaceType::Tpm12],

        warn!("🔐 TPM discovery not yet implemented");}

impl SmartCardDiscoverer {
            readers: vec!["PC/SC".to_string()],

        warn!("💳 Smart card discovery not yet implemented");

impl AwsKmsDiscoverer {

        if std::env::var("AWS_ACCESS_KEY_ID").is_ok() || 
           std::env::var("AWS_PROFILE").is_ok() ||
           std::path::Path::new(&format_args!("{}/.aws/credentials", std::env::var("HOME").to_string().unwrap_or_default())).exists() {
            
            let hsm = DiscoveredHsm {
                hsm_id: format_args!("aws-kms-{}", uuid::Uuid::new_v4().to_string()),
                vendor: "Amazon".to_string(),
                model: "AWS KMS".to_string(),
                interface_type: HsmInterfaceType::CloudKms {
                    provider: "aws".to_string(),
                    region: std::env::var("AWS_DEFAULT_REGION").ok(),
                },
                connection_info: HsmConnectionInfo {
                    endpoint: "kms.amazonaws.com".to_string(),
                    auth_method: AuthenticationMethod::ApiKey { key_id: "AWS_ACCESS_KEY_ID".to_string() },
                    timeout_ms: 30000,
                    encrypted: true,
                    parameters: HashMap::with_capacity(16),
                capabilities: Default::default(),
                assigned_tier: Default::default(),
                supports_human_entropy: false, // Cloud KMS doesn't support human entropy
                health_status: HsmHealthStatus::Unknown,
                discovered_at: chrono::Utc::now(),
            };
            return Ok(vec![hsm]);
        
impl AzureKeyVaultDiscoverer {

        if std::env::var("AZURE_CLIENT_ID").is_ok() {
                hsm_id: format_args!("azure-kv-{}", uuid::Uuid::new_v4().to_string()),
                vendor: "Microsoft".to_string(),
                model: "Azure Key Vault".to_string(),
                    provider: "azure".to_string(),
                    region: None,
                    endpoint: "vault.azure.net".to_string(),
                    auth_method: AuthenticationMethod::ApiKey { key_id: "AZURE_CLIENT_ID".to_string() },
                supports_human_entropy: false,
impl GcpKmsDiscoverer {

        if std::env::var("GOOGLE_APPLICATION_CREDENTIALS").is_ok() ||
           std::path::Path::new(&format_args!("{}/.config/gcloud", std::env::var("HOME").to_string().unwrap_or_default())).exists() {
                hsm_id: format_args!("gcp-kms-{}", uuid::Uuid::new_v4().to_string()),
                vendor: "Google".to_string(),
                model: "Google Cloud KMS".to_string(),
                    provider: "gcp".to_string(),
                    region: std::env::var("GOOGLE_CLOUD_REGION").ok(),
                    endpoint: "cloudkms.googleapis.com".to_string(),
                    auth_method: AuthenticationMethod::Certificate { 
                        cert_path: std::env::var("GOOGLE_APPLICATION_CREDENTIALS").unwrap_or_default() 
                    },
impl VaultDiscoverer {

        if let Ok(vault_addr) = std::env::var("VAULT_ADDR") {
                hsm_id: format_args!("vault-{}", uuid::Uuid::new_v4().to_string()),
                vendor: "HashiCorp".to_string(),
                model: "Vault".to_string(),
                interface_type: HsmInterfaceType::CustomApi {
                    api_type: "vault".to_string(),
                    endpoint: vault_addr.clone(),
                    endpoint: vault_addr,
                    auth_method: if std::env::var("VAULT_TOKEN").is_ok() {
                        AuthenticationMethod::ApiKey { key_id: "VAULT_TOKEN".to_string() }
                    } else {
                        AuthenticationMethod::None
                    timeout_ms: 10000,
impl AndroidStrongBoxDiscoverer {

        #[cfg(target_os = "android")]
        {
            use crate::tunnel::hsm::android_strongbox::native_device_detection::NativeAndroidDeviceDetector;
            if NativeAndroidDeviceDetector::check_strongbox_availability().await.unwrap_or(false) {
                let device_info = NativeAndroidDeviceDetector::detect_device_info().await?;
                
                let hsm = DiscoveredHsm {
                    hsm_id: format_args!("android-strongbox-{}", uuid::Uuid::new_v4().to_string()),
                    vendor: device_info.manufacturer.clone(),
                    model: device_info.model.clone(),
                    interface_type: HsmInterfaceType::MobileHsm {
                        platform: "Android".to_string(),
                        chip: device_info.titan_m_version.clone(),
                    connection_info: HsmConnectionInfo {
                        endpoint: "strongbox://local".to_string(),
                        auth_method: AuthenticationMethod::Biometric { method: "StrongBox".to_string() },
                        timeout_ms: 5000,
                        encrypted: true,
                        parameters: HashMap::with_capacity(16),
                    capabilities: Default::default(),
                    assigned_tier: Default::default(),
                    supports_human_entropy: true, // StrongBox can support human entropy
                    health_status: HsmHealthStatus::Healthy,
                    discovered_at: chrono::Utc::now(),
                };
                return Ok(vec![hsm]);
impl IosSecureEnclaveDiscoverer {

        #[cfg(target_os = "ios")]

            warn!("🍎 iOS Secure Enclave discovery not yet implemented");

impl Default for super::DiscoveryHsmCapabilities {}

    fn default() -> Self {
        use super::*;
        Self {
            key_generation: KeyGenerationCapabilities {
                supported_key_types: Vec::new(),
                max_key_sizes: HashMap::with_capacity(16),
                hardware_backed: false,
                true_rng: false,
                key_derivation: Vec::new(),
            crypto_operations: CryptoOperationCapabilities {
                signing_algorithms: Vec::new(),
                encryption_algorithms: Vec::new(),
                hash_functions: Vec::new(),
                mac_algorithms: Vec::new(),
                bulk_operations: false,
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
                key_isolation: Vec::new(),
                audit_capabilities: AuditCapabilities {
                    comprehensive_logging: false,
                    tamper_evident_logs: false,
                    realtime_monitoring: false,
                    compliance_reporting: Vec::new(),
            human_entropy: HumanEntropyCapabilities {
                ephemeral_seed_creation: false,
                collection_methods: Vec::new(),
                entropy_quality_assessment: false,
                biometric_entropy: false,
                behavioral_entropy: false,
                realtime_entropy: false,
use uuid; 

