

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn, error};
use serde::{Deserialize, Serialize};
use beardog_errors::BearDogError;
use beardog_types::canonical::hsm::capabilities::HsmCapabilities;
use super::{DiscoveredHsm, HsmInterfaceType, HsmConnectionInfo, AuthenticationMethod, DiscoveryConfig};

#[derive(Debug)]
pub struct StandaloneHsmDiscovery {

    platform: DevicePlatform,

    discovered_hsms: Arc<RwLock<Vec<StandaloneHsmInfo>>>,

    ecosystem_status: Arc<RwLock<EcosystemStatus>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DevicePlatform {

    Android {
        version: String,
        security_level: AndroidSecurityLevel,
    },

    Ios {
        secure_enclave: bool,

    Linux {
        kernel_version: String,
        has_tpm: bool,

    MacOs {

    Windows {

    Generic,

pub enum AndroidSecurityLevel {

    StrongBox,

    Tee,

    Software,

#[derive(Debug, Clone, Serialize, Deserialize)]}

pub struct StandaloneHsmInfo {

    pub id: String,

    pub hsm_type: StandaloneHsmType,

    pub vendor: String,
}

    pub model: String,

    pub security_tier: SecurityTier,

    pub available: bool,

    pub supports_human_entropy: bool,

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StandaloneHsmType {

    MobileHardware,

    DesktopHardware,

    Cloud,

pub enum SecurityTier {

    Hardware,

    Trusted,

#[derive(Debug, Clone)]}

pub struct EcosystemStatus {

    pub songbird_connected: bool,

    pub toadstool_connected: bool,

    pub last_ping: Option<chrono::DateTime<chrono::Utc>>,

    pub available_capabilities: Vec<String>,}

impl StandaloneHsmDiscovery {

    pub async fn new() -> Result<Self, BearDogError> {
        info!("🔍 Initializing Standalone HSM Discovery");
        
        let platform = Self::detect_platform().await?;
        info!("📱 Detected platform: {:?}", platform);
        Ok(Self {
            platform,
            discovered_hsms: Arc::new(RwLock::new(Vec::new())),
            ecosystem_status: Arc::new(RwLock::new(EcosystemStatus {
                songbird_connected: false,
                toadstool_connected: false,
                last_ping: None,
                available_capabilities: Vec::new(),
            })),
        })
    }

    pub async fn discover_basic(&self) -> Result<Vec<StandaloneHsmInfo>, BearDogError>> {
        info!("🔍 Starting basic standalone HSM discovery");
        let mut hsms = Vec::new();

        if let Some(hardware_hsm) = self.discover_platform_hardware().await? {
            hsms.push(hardware_hsm);
        }

        hsms.extend(self.discover_software_fallbacks().await?);

        if let Ok(ecosystem_hsms) = self.discover_ecosystem_hsms().await {
            hsms.extend(ecosystem_hsms);

        {
            let mut cache = self.discovered_hsms.write().await;
            *cache = hsms.clone();
        info!("✅ Discovered {} HSMs in standalone mode", hsms.len());
        Ok(hsms)

    async fn detect_platform() -> Result<DevicePlatform, BearDogError> {
        #[cfg(target_os = "android")]
            let version = Self::get_android_version().await?;
            let security_level = Self::detect_android_security_level().await?;
            Ok(DevicePlatform::Android { version, security_level })
        #[cfg(target_os = "ios")]
            let version = Self::get_ios_version().await?;
            let secure_enclave = Self::has_secure_enclave().await?;
            Ok(DevicePlatform::Ios { version, secure_enclave })
        #[cfg(target_os = "linux")]
            let kernel_version = Self::get_kernel_version().await?;
            let has_tpm = Self::check_tpm_availability().await?;
            Ok(DevicePlatform::Linux { kernel_version, has_tpm })
        #[cfg(target_os = "macos")]
            let version = Self::get_macos_version().await?;
            Ok(DevicePlatform::MacOs { version, secure_enclave })
        #[cfg(target_os = "windows")]
            let version = Self::get_windows_version().await?;
            Ok(DevicePlatform::Windows { version, has_tpm })
        #[cfg(not(any(target_os = "android", target_os = "ios", target_os = "linux", target_os = "macos", target_os = "windows")))]
            warn!("Unknown platform, using generic detection");
            Ok(DevicePlatform::Generic)

    async fn discover_platform_hardware(&self) -> Result<Option<StandaloneHsmInfo>, BearDogError>> {
        match &self.platform {
            DevicePlatform::Android { security_level, .. } => {
                if *security_level == AndroidSecurityLevel::StrongBox {
                    info!("📱 Found Android StrongBox (Pixel 8 hardware security)");
                    Ok(Some(StandaloneHsmInfo {
                        id: "android-strongbox".to_string(),
                        hsm_type: StandaloneHsmType::MobileHardware,
                        vendor: "Google".to_string(),
                        model: "Android StrongBox".to_string(),
                        security_tier: SecurityTier::Hardware,
                        available: true,
                        supports_human_entropy: true, // Pixel 8 supports biometric entropy
                    }))
                } else {
                    debug!("Android TEE/Software security detected");
                        id: "android-keystore".to_string(),
                        vendor: "Android".to_string(),
                        model: "KeyStore".to_string(),
                        security_tier: SecurityTier::Trusted,
                        supports_human_entropy: false,
                }
            },
            
            DevicePlatform::Ios { secure_enclave: true, .. } => {
                info!("🍎 Found iOS Secure Enclave");
                Ok(Some(StandaloneHsmInfo {
                    id: "ios-secure-enclave".to_string(),
                    hsm_type: StandaloneHsmType::MobileHardware,
                    vendor: "Apple".to_string(),
                    model: "Secure Enclave".to_string(),
                    security_tier: SecurityTier::Hardware,
                    available: true,
                    supports_human_entropy: true, // Touch ID/Face ID entropy
                }))
            DevicePlatform::Linux { has_tpm: true, .. } | 
            DevicePlatform::Windows { has_tpm: true, .. } => {
                info!("🔐 Found TPM hardware");
                    id: "tpm-hardware".to_string(),
                    hsm_type: StandaloneHsmType::DesktopHardware,
                    vendor: "TPM".to_string(),
                    model: "TPM 2.0".to_string(),
                    supports_human_entropy: false,
            DevicePlatform::MacOs { secure_enclave: true, .. } => {
                info!("🍎 Found macOS Secure Enclave");
                    id: "macos-secure-enclave".to_string(),
                    model: "macOS Secure Enclave".to_string(),
                    supports_human_entropy: true,
            _ => {
                debug!("No platform-specific hardware security found");
                Ok(None)
            }

    async fn discover_software_fallbacks(&self) -> Result<Vec<StandaloneHsmInfo>, BearDogError>> {

        hsms.push(StandaloneHsmInfo {
            id: "beardog-native".to_string(),
            hsm_type: StandaloneHsmType::Software,
            vendor: "`BearDog`".to_string(),
            model: "Native HSM".to_string(),
            security_tier: SecurityTier::Software,
            available: true,
            supports_human_entropy: true, // `BearDog` specializes in human entropy
        });

        if self.check_openssl_available().await {
            hsms.push(StandaloneHsmInfo {
                id: "openssl-software".to_string(),
                hsm_type: StandaloneHsmType::Software,
                vendor: "OpenSSL".to_string(),
                model: "Software Crypto".to_string(),
                security_tier: SecurityTier::Software,
                available: true,
                supports_human_entropy: false,
            });

    async fn discover_ecosystem_hsms(&self) -> Result<Vec<StandaloneHsmInfo>, BearDogError>> {
        let ecosystem_status = self.ecosystem_status.read().await;
        if !ecosystem_status.songbird_connected && !ecosystem_status.toadstool_connected {
            debug!("No ecosystem connection available");
            return Ok(Vec::new());

        if ecosystem_status.toadstool_connected {
            if ecosystem_status.available_capabilities.contains(&"software_hsm".to_string()) {
                hsms.push(StandaloneHsmInfo {
                    id: "toadstool-software-hsm".to_string(),
                    hsm_type: StandaloneHsmType::Software,
                    vendor: "ToadStool".to_string(),
                    model: "Platform Software HSM".to_string(),
                    security_tier: SecurityTier::Software,
                });
            if ecosystem_status.available_capabilities.contains(&"cloud_hsm".to_string()) {
                    id: "toadstool-cloud-hsm".to_string(),
                    hsm_type: StandaloneHsmType::Cloud,
                    model: "Cloud HSM Substrate".to_string(),
                    security_tier: SecurityTier::Trusted,
        info!("🌐 Found {} ecosystem HSMs", hsms.len());

    pub async fn try_ecosystem_connection(&self) -> Result<(), BearDogError> {
        debug!("🌐 Attempting ecosystem connection");

        let songbird_ok = self.try_songbird_connection().await.is_ok();

        let toadstool_ok = self.try_toadstool_connection().await.is_ok();

            let mut status = self.ecosystem_status.write().await;
            status.songbird_connected = songbird_ok;
            status.toadstool_connected = toadstool_ok;
            status.last_ping = Some(chrono::Utc::now());
            if songbird_ok || toadstool_ok {
                info!("✅ Ecosystem connection established (songbird: {}, toadstool: {})", 
                      songbird_ok, toadstool_ok);
            } else {
                debug!("❌ No ecosystem connection available (standalone mode)");
        Ok(())

    pub async fn to_discovered_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError>> {
        let standalone_hsms = self.discover_basic().await?;
        let mut discovered_hsms = Vec::new();
        for hsm in standalone_hsms {
            discovered_hsms.push(self.convert_to_full_format(hsm).await?);
        Ok(discovered_hsms)

    #[cfg(target_os = "android")]}

    async fn get_android_version() -> Result<String, BearDogError> {

        Ok("Android 14".to_string()) // Placeholder - would use JNI calls
    async fn detect_android_security_level() -> Result<AndroidSecurityLevel, BearDogError> {

        Ok(AndroidSecurityLevel::StrongBox) // Placeholder
    #[cfg(any(target_os = "ios", target_os = "macos"))]}

    async fn get_ios_version() -> Result<String, BearDogError> {
        Ok("iOS 17".to_string()) // Placeholder
    async fn get_macos_version() -> Result<String, BearDogError> {
        Ok("macOS 14".to_string()) // Placeholder}

    async fn has_secure_enclave() -> Result<bool, BearDogError> {

        use std::process::Command;
        let output = Command::new("system_profiler")
            .args(&["SPHardwareDataType"])
            .output();
        match output {
            Ok(output) => {
                let output_str = String::from_utf8_lossy(&output.stdout);

                Ok(output_str.contains("Apple T2") || 
                   output_str.contains("Apple M1") || 
                   output_str.contains("Apple M2") || 
                   output_str.contains("Apple M3"))
            Err(_) => {

                Ok(true)
    #[cfg(target_os = "linux")]
    async fn get_kernel_version() -> Result<String, BearDogError> {
        let output = tokio::process::Command::new("uname")
            .args(["-r"])
            .output()
            .await?;
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    #[cfg(target_os = "windows")]}

    async fn get_windows_version() -> Result<String, BearDogError> {
        let output = Command::new("cmd")
            .args(&["/C", "ver"])
                let version_str = String::from_utf8_lossy(&output.stdout);
                Ok(version_str.trim().to_string())

                Ok("Windows (version detection failed)".to_string())
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    async fn check_tpm_availability() -> Result<bool, BearDogError> {

            tokio::fs::metadata("/dev/tpm0").await.is_ok() || 
            tokio::fs::metadata("/dev/tpmrm0").await.is_ok()

            use std::process::Command;
            let output = Command::new("powershell")
                .args(&["-Command", "Get-WmiObject -Namespace \"Root\\CIMv2\\Security\\MicrosoftTpm\" -Class Win32_Tpm"])
                .output();
                
            match output {
                Ok(output) => Ok(!output.stdout.is_empty()),
                Err(_) => {

                    tracing::debug!("TPM detection failed, assuming no TPM available");
                    Ok(false)}

    async fn check_openssl_available(&self) -> bool {
        tokio::process::Command::new("openssl")
            .args(["version"])
            .await
            .is_ok()

    async fn try_songbird_connection(&self) -> Result<(), BearDogError> {

        debug!("🎼 Attempting songbird connection");

        Err(BearDogError::Ecosystem("Songbird connection not implemented".to_string()))}

    async fn try_toadstool_connection(&self) -> Result<(), BearDogError> {

        debug!("🍄 Attempting toadstool connection");

        Err(BearDogError::Ecosystem("ToadStool connection not implemented".to_string()))

    async fn convert_to_full_format(&self, standalone: StandaloneHsmInfo) -> Result<DiscoveredHsm, BearDogError> {
        let interface_type = match standalone.hsm_type {
            StandaloneHsmType::MobileHardware => {
                if standalone.id.contains("android") {
                    HsmInterfaceType::AndroidStrongBox {
                        security_level: "StrongBox".to_string(),
                    }
                    HsmInterfaceType::IosSecureEnclave {
                        enclave_version: "1.0".to_string(),
            StandaloneHsmType::DesktopHardware => {
                HsmInterfaceType::Tpm {
                    version: "2.0".to_string(),
            StandaloneHsmType::Software => {
                if standalone.id.contains("beardog") {
                    HsmInterfaceType::`BearDog`Native {
                        instance_id: standalone.id.clone(),
                    HsmInterfaceType::OpenSsl {
                        engine_path: None,
            StandaloneHsmType::Cloud => {
                HsmInterfaceType::CustomApi {
                    api_endpoint: "toadstool-cloud".to_string(),
                    api_version: "1.0".to_string(),
        };
        Ok(DiscoveredHsm {
            id: standalone.id,
            vendor: standalone.vendor,
            model: standalone.model,
            version: "1.0".to_string(),
            interface_type,
            connection_info: HsmConnectionInfo {
                connection_type: super::ConnectionType::Local,
                authentication: AuthenticationMethod::None,
                endpoint: None,
                port: None,
                additional_params: std::collections::HashMap::with_capacity(16),
            capabilities: self.create_basic_capabilities(&standalone).await,
            tier: self.map_security_tier(standalone.security_tier),
            supports_human_entropy: standalone.supports_human_entropy,
    async fn create_basic_capabilities(&self, hsm: &StandaloneHsmInfo) -> HsmCapabilities {
        use beardog_types::canonical::hsm::capabilities::*;
        HsmCapabilities {
            vendor: hsm.vendor.clone(),
            model: hsm.model.clone(),
            firmware_version: "1.0".to_string(),
            supported_key_types: vec!["AES".to_string(), "RSA".to_string(), "ECC".to_string()],
            supported_algorithms: vec!["AES-GCM".to_string(), "RSA-PSS".to_string(), "ECDSA".to_string()],
            key_generation: KeyGenerationCapabilities {
                hardware_generation: hsm.security_tier == SecurityTier::Hardware,
                supported_key_sizes: vec![256, 2048],
                generation_speed: Some(100),
            key_management: KeyManagementCapabilities {
                max_keys: Some(1000),
                backup_recovery: false,
                key_migration: false,
                key_versioning: false,
                lifecycle_management: true,
            advanced_features: AdvancedFeatureCapabilities {
                hardware_attestation: hsm.security_tier == SecurityTier::Hardware,
                secure_boot: false,
                tamper_resistance: hsm.security_tier == SecurityTier::Hardware,
                true_random_generation: true,
                physical_security_level: match hsm.security_tier {
                    SecurityTier::Hardware => "FIPS-140-2-Level-3".to_string(),
                    SecurityTier::Trusted => "FIPS-140-2-Level-2".to_string(),
                    SecurityTier::Software => "FIPS-140-2-Level-1".to_string(),
                },
            api_support: ApiSupportCapabilities {
                pkcs11: false,
                jce: hsm.id.contains("android"),
                capi: false,
                cng: false,
                native_api: true,
                rest_api: false,
            security: SecurityCapabilities {
                fips_level: Some(match hsm.security_tier {
                    SecurityTier::Hardware => 3,
                    SecurityTier::Trusted => 2,
                    SecurityTier::Software => 1,
                }),
                common_criteria: None,
                certifications: vec!["Internal".to_string()],
                auth_methods: if hsm.supports_human_entropy {
                    vec!["Biometric".to_string(), "PIN".to_string()]
                    vec!["PIN".to_string()]
            performance_metrics: std::collections::HashMap::with_capacity(16),
    fn map_security_tier(&self, tier: SecurityTier) -> super::HsmTier {
        match tier {
            SecurityTier::Hardware => super::HsmTier::Hardware,
            SecurityTier::Trusted => super::HsmTier::Smartphone,
            SecurityTier::Software => super::HsmTier::Software,

pub struct StandaloneHsmFactory;
impl StandaloneHsmFactory {

    pub async fn create() -> Result<StandaloneHsmDiscovery, BearDogError> {
        StandaloneHsmDiscovery::new().await

    pub async fn quick_discover() -> Result<Vec<StandaloneHsmInfo>, BearDogError>> {
        let discovery = Self::create().await?;
        discovery.discover_basic().await

    pub async fn ecosystem_aware_discover() -> Result<Vec<StandaloneHsmInfo>, BearDogError>> {

        let _ = discovery.try_ecosystem_connection().await;

} 
