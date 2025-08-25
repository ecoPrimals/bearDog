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


/// # Standalone HSM Discovery
///
/// **BASIC STANDALONE VERSION** for mobile deployment (e.g., Pixel 8)
/// This module provides a minimal HSM discovery system that:
/// - Works standalone for failsafe scenarios
/// - Leverages device hardware security (Android StrongBox, iOS Secure Enclave)
/// - Integrates with ecosystem via songbird universal adapter when available
/// - Falls back gracefully when network unavailable

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn, error};
use serde::{Deserialize, Serialize};
use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::hsm::capabilities::HsmCapabilities;
use super::{DiscoveredHsm, HsmInterfaceType, HsmConnectionInfo, AuthenticationMethod, DiscoveryConfig};
/// **BASIC STANDALONE HSM DISCOVERY**
/// 
/// Minimal HSM discovery focused on:
/// - Mobile hardware security (Android StrongBox, iOS Secure Enclave)
/// - Software fallbacks (`BearDog` Native, OpenSSL)
/// - Ecosystem integration when available
/// - Failsafe operation when isolated
#[derive(Debug)]
pub struct StandaloneHsmDiscovery {
    /// Current device platform
    platform: DevicePlatform,
    /// Available HSMs cache
    discovered_hsms: Arc<RwLock<Vec<StandaloneHsmInfo>>>,
    /// Ecosystem connection status
    ecosystem_status: Arc<RwLock<EcosystemStatus>>,
}
/// **DEVICE PLATFORM DETECTION**
#[derive(Debug, Clone, PartialEq)]
pub enum DevicePlatform {
    /// Android device (focus: StrongBox, KeyStore)
    Android {
        version: String,
        security_level: AndroidSecurityLevel,
    },
    /// iOS device (focus: Secure Enclave)
    Ios {
        secure_enclave: bool,
    /// Linux desktop/server
    Linux {
        kernel_version: String,
        has_tpm: bool,
    /// macOS desktop
    MacOs {
    /// Windows desktop
    Windows {
    /// Unknown/Generic platform
    Generic,
/// **ANDROID SECURITY LEVELS** (for Pixel 8 StrongBox)
pub enum AndroidSecurityLevel {
    /// Hardware-backed security (StrongBox)
    StrongBox,
    /// Trusted Execution Environment
    Tee,
    /// Software only
    Software,
/// **BASIC HSM INFO** (standalone version)
#[derive(Debug, Clone, Serialize, Deserialize)]}


pub struct StandaloneHsmInfo {
    /// HSM identifier
    pub id: String,
    /// HSM type (simplified)
    pub hsm_type: StandaloneHsmType,
    /// Vendor/model info
    pub vendor: String,
}


    pub model: String,
    /// Security tier
    pub security_tier: SecurityTier,
    /// Availability status
    pub available: bool,
    /// Human entropy support
    pub supports_human_entropy: bool,
/// **SIMPLIFIED HSM TYPES** (for standalone)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StandaloneHsmType {
    /// Mobile hardware security
    MobileHardware,
    /// Desktop/server hardware
    DesktopHardware,
    /// Software HSM
    /// Cloud HSM (via ecosystem)
    Cloud,
/// **SECURITY TIERS** (simplified)}


pub enum SecurityTier {
    /// Hardware-backed with tamper resistance
    Hardware,
    /// Trusted execution environment
    Trusted,
    /// Software with memory protection
/// **ECOSYSTEM CONNECTION STATUS**
#[derive(Debug, Clone)]}


pub struct EcosystemStatus {
    /// Connected to songbird universal adapter
    pub songbird_connected: bool,
    /// Connected to toadstool platform
    pub toadstool_connected: bool,
    /// Last successful ecosystem ping
    pub last_ping: Option<chrono::DateTime<chrono::Utc>>,
    /// Available ecosystem capabilities
    pub available_capabilities: Vec<String>,}


impl StandaloneHsmDiscovery {
    /// **CREATE STANDALONE DISCOVERY**
    pub async fn new() -> BearDogResult<Self> {
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
    
    /// **BASIC HSM DISCOVERY** (standalone mode)
    pub async fn discover_basic(&self) -> BearDogResult<Vec<StandaloneHsmInfo>> {
        info!("🔍 Starting basic standalone HSM discovery");
        let mut hsms = Vec::new();
        // 1. Discover platform-specific hardware security
        if let Some(hardware_hsm) = self.discover_platform_hardware().await? {
            hsms.push(hardware_hsm);
        }
        // 2. Discover software fallbacks
        hsms.extend(self.discover_software_fallbacks().await?);
        // 3. Check ecosystem capabilities (if connected)
        if let Ok(ecosystem_hsms) = self.discover_ecosystem_hsms().await {
            hsms.extend(ecosystem_hsms);
        // Cache results
        {
            let mut cache = self.discovered_hsms.write().await;
            *cache = hsms.clone();
        info!("✅ Discovered {} HSMs in standalone mode", hsms.len());
        Ok(hsms)
    /// **DETECT DEVICE PLATFORM**
    async fn detect_platform() -> BearDogResult<DevicePlatform> {
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
    /// **DISCOVER PLATFORM-SPECIFIC HARDWARE**
    async fn discover_platform_hardware(&self) -> BearDogResult<Option<StandaloneHsmInfo>> {
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
    /// **DISCOVER SOFTWARE FALLBACKS**
    async fn discover_software_fallbacks(&self) -> BearDogResult<Vec<StandaloneHsmInfo>> {
        // `BearDog` Native (always available)
        hsms.push(StandaloneHsmInfo {
            id: "beardog-native".to_string(),
            hsm_type: StandaloneHsmType::Software,
            vendor: "`BearDog`".to_string(),
            model: "Native HSM".to_string(),
            security_tier: SecurityTier::Software,
            available: true,
            supports_human_entropy: true, // `BearDog` specializes in human entropy
        });
        // OpenSSL (if available)
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
    /// **DISCOVER ECOSYSTEM HSMs** (via songbird/toadstool)
    async fn discover_ecosystem_hsms(&self) -> BearDogResult<Vec<StandaloneHsmInfo>> {
        let ecosystem_status = self.ecosystem_status.read().await;
        if !ecosystem_status.songbird_connected && !ecosystem_status.toadstool_connected {
            debug!("No ecosystem connection available");
            return Ok(Vec::new());
        // Check for toadstool software HSM capabilities
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
    /// **TRY ECOSYSTEM CONNECTION** (non-blocking)
    pub async fn try_ecosystem_connection(&self) -> BearDogResult<()> {
        debug!("🌐 Attempting ecosystem connection");
        // Try songbird connection (non-blocking)
        let songbird_ok = self.try_songbird_connection().await.is_ok();
        // Try toadstool connection (non-blocking)
        let toadstool_ok = self.try_toadstool_connection().await.is_ok();
        // Update status
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
    /// **CONVERT TO FULL HSM DISCOVERY FORMAT** (for compatibility)
    pub async fn to_discovered_hsms(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        let standalone_hsms = self.discover_basic().await?;
        let mut discovered_hsms = Vec::new();
        for hsm in standalone_hsms {
            discovered_hsms.push(self.convert_to_full_format(hsm).await?);
        Ok(discovered_hsms)
    // === PLATFORM-SPECIFIC DETECTION HELPERS ===
    #[cfg(target_os = "android")]}


    async fn get_android_version() -> BearDogResult<String> {
        // Use Android system properties
        Ok("Android 14".to_string()) // Placeholder - would use JNI calls
    async fn detect_android_security_level() -> BearDogResult<AndroidSecurityLevel> {
        // Check for StrongBox availability (Pixel 8 has it)
        // This would use Android KeyStore API calls via JNI
        Ok(AndroidSecurityLevel::StrongBox) // Placeholder
    #[cfg(any(target_os = "ios", target_os = "macos"))]}


    async fn get_ios_version() -> BearDogResult<String> {
        Ok("iOS 17".to_string()) // Placeholder
    async fn get_macos_version() -> BearDogResult<String> {
        Ok("macOS 14".to_string()) // Placeholder}


    async fn has_secure_enclave() -> BearDogResult<bool> {
        // Check for Secure Enclave availability using safe detection
        // This uses runtime detection rather than unsafe Security Framework FFI
        use std::process::Command;
        let output = Command::new("system_profiler")
            .args(&["SPHardwareDataType"])
            .output();
        match output {
            Ok(output) => {
                let output_str = String::from_utf8_lossy(&output.stdout);
                // Look for T2, M1, M2, or M3 chips which have Secure Enclave
                Ok(output_str.contains("Apple T2") || 
                   output_str.contains("Apple M1") || 
                   output_str.contains("Apple M2") || 
                   output_str.contains("Apple M3"))
            Err(_) => {
                // If we can't detect, assume it's available for safety
                // Better to try and fail gracefully than miss capability
                Ok(true)
    #[cfg(target_os = "linux")]
    async fn get_kernel_version() -> BearDogResult<String> {
        let output = tokio::process::Command::new("uname")
            .args(["-r"])
            .output()
            .await?;
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    #[cfg(target_os = "windows")]}


    async fn get_windows_version() -> BearDogResult<String> {
        let output = Command::new("cmd")
            .args(&["/C", "ver"])
                let version_str = String::from_utf8_lossy(&output.stdout);
                Ok(version_str.trim().to_string())
                // Fallback to reasonable default
                Ok("Windows (version detection failed)".to_string())
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    async fn check_tpm_availability() -> BearDogResult<bool> {
        // Check for TPM device
            tokio::fs::metadata("/dev/tpm0").await.is_ok() || 
            tokio::fs::metadata("/dev/tpmrm0").await.is_ok()
            // Check for TPM using PowerShell command (safe approach)
            use std::process::Command;
            let output = Command::new("powershell")
                .args(&["-Command", "Get-WmiObject -Namespace \"Root\\CIMv2\\Security\\MicrosoftTpm\" -Class Win32_Tpm"])
                .output();
                
            match output {
                Ok(output) => Ok(!output.stdout.is_empty()),
                Err(_) => {
                    // If we can't detect, assume no TPM for safety
                    tracing::debug!("TPM detection failed, assuming no TPM available");
                    Ok(false)}


    async fn check_openssl_available(&self) -> bool {
        tokio::process::Command::new("openssl")
            .args(["version"])
            .await
            .is_ok()
    // === ECOSYSTEM CONNECTION HELPERS ===
    async fn try_songbird_connection(&self) -> BearDogResult<()> {
        // Try to connect to songbird universal adapter
        // This would use the actual universal adapter factory
        debug!("🎼 Attempting songbird connection");
        // Placeholder - would create actual connection
        // let adapter = UniversalAdapterFactory::create_songbird_adapter(
        //     "http://localhost:8080".to_string(),
        //     "api_key".to_string(),
        // ).await?;
        Err(BearDogError::Ecosystem("Songbird connection not implemented".to_string()))}


    async fn try_toadstool_connection(&self) -> BearDogResult<()> {
        // Try to connect to toadstool platform
        debug!("🍄 Attempting toadstool connection");
        // Placeholder - would register with toadstool
        Err(BearDogError::Ecosystem("ToadStool connection not implemented".to_string()))
    /// **CONVERT STANDALONE TO FULL FORMAT**
    async fn convert_to_full_format(&self, standalone: StandaloneHsmInfo) -> BearDogResult<DiscoveredHsm> {
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
                additional_params: std::collections::HashMap::new(),
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
            performance_metrics: std::collections::HashMap::new(),
    fn map_security_tier(&self, tier: SecurityTier) -> super::HsmTier {
        match tier {
            SecurityTier::Hardware => super::HsmTier::Hardware,
            SecurityTier::Trusted => super::HsmTier::Smartphone,
            SecurityTier::Software => super::HsmTier::Software,
/// **STANDALONE HSM FACTORY** (simplified interface)}


pub struct StandaloneHsmFactory;
impl StandaloneHsmFactory {
    /// **CREATE STANDALONE HSM DISCOVERY**}


    pub async fn create() -> BearDogResult<StandaloneHsmDiscovery> {
        StandaloneHsmDiscovery::new().await
    /// **QUICK DISCOVERY** (basic HSM info only)
    pub async fn quick_discover() -> BearDogResult<Vec<StandaloneHsmInfo>> {
        let discovery = Self::create().await?;
        discovery.discover_basic().await
    /// **ECOSYSTEM-AWARE DISCOVERY** (try ecosystem connection first)}


    pub async fn ecosystem_aware_discover() -> BearDogResult<Vec<StandaloneHsmInfo>> {
        // Try ecosystem connection (non-blocking)
        let _ = discovery.try_ecosystem_connection().await;
        // Discover with ecosystem capabilities if available
} 
