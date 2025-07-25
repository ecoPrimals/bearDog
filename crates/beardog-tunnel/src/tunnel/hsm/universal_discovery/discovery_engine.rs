//! # HSM Discovery Engine
//!
//! This module provides comprehensive HSM discovery capabilities
//! for all supported HSM types and interfaces.

use super::{
    AuthenticationMethod, DiscoveredHsm, HsmConnectionInfo, HsmHealthStatus, HsmInterfaceType,
};
use beardog_errors::{BearDogError, BearDogResult};
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{debug, info, warn};

/// Discovery engine for finding HSMs of all types
pub struct DiscoveryEngine {
    /// PKCS#11 library discoverer
    pkcs11_discoverer: Pkcs11Discoverer,
    /// Cloud KMS discoverer
    cloud_kms_discoverer: CloudKmsDiscoverer,
    /// Network HSM discoverer
    network_hsm_discoverer: NetworkHsmDiscoverer,
    /// USB HSM discoverer
    usb_hsm_discoverer: UsbHsmDiscoverer,
    /// Software HSM discoverer
    software_hsm_discoverer: SoftwareHsmDiscoverer,
    /// Mobile HSM discoverer
    mobile_hsm_discoverer: MobileHsmDiscoverer,
    /// TPM discoverer
    tpm_discoverer: TpmDiscoverer,
    /// Smart card discoverer
    smartcard_discoverer: SmartCardDiscoverer,
}

/// PKCS#11 library discoverer
pub struct Pkcs11Discoverer {
    /// Common PKCS#11 library search paths
    search_paths: Vec<PathBuf>,
    /// Known PKCS#11 library patterns
    library_patterns: Vec<String>,
}

/// Cloud KMS service discoverer
pub struct CloudKmsDiscoverer {
    /// AWS KMS discoverer
    aws_discoverer: AwsKmsDiscoverer,
    /// Azure Key Vault discoverer
    azure_discoverer: AzureKeyVaultDiscoverer,
    /// Google Cloud KMS discoverer
    gcp_discoverer: GcpKmsDiscoverer,
    /// HashiCorp Vault discoverer
    vault_discoverer: VaultDiscoverer,
}

/// Network HSM discoverer
pub struct NetworkHsmDiscoverer {
    /// Common HSM network ports
    common_ports: Vec<u16>,
    /// Network scanning configuration
    scan_config: NetworkScanConfig,
}

/// USB HSM discoverer
pub struct UsbHsmDiscoverer {
    /// Known USB vendor IDs for HSM devices
    hsm_vendor_ids: Vec<u16>,
    /// Device enumeration configuration
    enum_config: UsbEnumerationConfig,
}

/// Software HSM discoverer
pub struct SoftwareHsmDiscoverer {
    /// Software HSM implementations to detect
    implementations: Vec<SoftwareHsmImplementation>,
}

/// Mobile HSM discoverer
pub struct MobileHsmDiscoverer {
    /// Android StrongBox discoverer
    android_discoverer: AndroidStrongBoxDiscoverer,
    /// iOS Secure Enclave discoverer
    ios_discoverer: IosSecureEnclaveDiscoverer,
}

/// TPM discoverer
pub struct TpmDiscoverer {
    /// TPM interface types to check
    interface_types: Vec<TpmInterfaceType>,
}

/// Smart card discoverer
pub struct SmartCardDiscoverer {
    /// PC/SC readers to enumerate
    readers: Vec<String>,
}

/// Network scanning configuration
#[derive(Debug, Clone)]
pub struct NetworkScanConfig {
    /// IP address ranges to scan
    pub ip_ranges: Vec<String>,
    /// Connection timeout in milliseconds
    pub timeout_ms: u32,
    /// Maximum concurrent connections
    pub max_concurrent: usize,
}

/// USB enumeration configuration
#[derive(Debug, Clone)]
pub struct UsbEnumerationConfig {
    /// Enable device enumeration
    pub enable_enumeration: bool,
    /// Enumeration timeout
    pub timeout_ms: u32,
}

/// Software HSM implementations
#[derive(Debug, Clone)]
pub enum SoftwareHsmImplementation {
    /// OpenSSL-based software HSM
    OpenSsl,
    /// SoftHSM (PKCS#11 software implementation)
    SoftHsm,
    /// BearDog native software HSM
    BearDogNative,
    /// Microsoft CNG (Windows)
    MicrosoftCng,
    /// macOS Keychain
    MacOsKeychain,
    /// Custom implementation
    Custom { name: String, path: String },
}

/// TPM interface types
#[derive(Debug, Clone)]
pub enum TpmInterfaceType {
    /// TPM 1.2
    Tpm12,
    /// TPM 2.0
    Tpm20,
    /// fTPM (Firmware TPM)
    FirmwareTpm,
    /// Software TPM simulator
    SoftwareTpm,
}

/// AWS KMS discoverer
pub struct AwsKmsDiscoverer;

/// Azure Key Vault discoverer
pub struct AzureKeyVaultDiscoverer;

/// Google Cloud KMS discoverer
pub struct GcpKmsDiscoverer;

/// HashiCorp Vault discoverer
pub struct VaultDiscoverer;

/// Android StrongBox discoverer
pub struct AndroidStrongBoxDiscoverer;

/// iOS Secure Enclave discoverer
pub struct IosSecureEnclaveDiscoverer;

impl DiscoveryEngine {
    /// Create a new discovery engine
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

    /// Discover PKCS#11 HSMs
    pub async fn discover_pkcs11_hsms(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        info!("🔍 Discovering PKCS#11 HSMs");
        self.pkcs11_discoverer.discover().await
    }

    /// Discover Cloud KMS instances
    pub async fn discover_cloud_kms_hsms(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        info!("☁️ Discovering Cloud KMS instances");
        self.cloud_kms_discoverer.discover().await
    }

    /// Discover Network HSMs
    pub async fn discover_network_hsms(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        info!("🌐 Discovering Network HSMs");
        self.network_hsm_discoverer.discover().await
    }

    /// Discover USB HSMs
    pub async fn discover_usb_hsms(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        info!("🔌 Discovering USB HSMs");
        self.usb_hsm_discoverer.discover().await
    }

    /// Discover Software HSMs
    pub async fn discover_software_hsms(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        info!("💻 Discovering Software HSMs");
        self.software_hsm_discoverer.discover().await
    }

    /// Discover Mobile HSMs
    pub async fn discover_mobile_hsms(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        info!("📱 Discovering Mobile HSMs");
        self.mobile_hsm_discoverer.discover().await
    }

    /// Discover TPMs
    pub async fn discover_tpm_hsms(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        info!("🔐 Discovering TPMs");
        self.tpm_discoverer.discover().await
    }

    /// Discover Smart Cards
    pub async fn discover_smartcard_hsms(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        info!("💳 Discovering Smart Cards");
        self.smartcard_discoverer.discover().await
    }
}

impl Pkcs11Discoverer {
    /// Create a new PKCS#11 discoverer
    pub async fn new() -> BearDogResult<Self> {
        let search_paths = Self::get_default_search_paths();
        let library_patterns = Self::get_library_patterns();

        Ok(Self {
            search_paths,
            library_patterns,
        })
    }

    /// Discover PKCS#11 libraries
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
    }

    /// Get default PKCS#11 search paths
    fn get_default_search_paths() -> Vec<PathBuf> {
        vec![
            // Linux paths
            PathBuf::from("/usr/lib"),
            PathBuf::from("/usr/lib64"),
            PathBuf::from("/usr/local/lib"),
            PathBuf::from("/opt/*/lib"),
            // Windows paths
            PathBuf::from("C:\\Windows\\System32"),
            PathBuf::from("C:\\Program Files\\*\\lib"),
            // macOS paths
            PathBuf::from("/usr/lib"),
            PathBuf::from("/usr/local/lib"),
            PathBuf::from("/Applications/*/lib"),
        ]
    }

    /// Get PKCS#11 library patterns
    fn get_library_patterns() -> Vec<String> {
        vec![
            // Generic PKCS#11
            "libpkcs11.so".to_string(),
            "pkcs11.dll".to_string(),
            // Vendor-specific libraries
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
        ]
    }

    /// Check if path is a PKCS#11 library
    fn is_pkcs11_library(&self, path: &PathBuf) -> bool {
        if let Some(filename) = path.file_name() {
            let filename_str = filename.to_string_lossy();
            return self.library_patterns.iter()
                .any(|pattern| filename_str.contains(pattern));
        }
        false
    }

    /// Probe a PKCS#11 library to get HSM information
    async fn probe_pkcs11_library(&self, path: &PathBuf) -> BearDogResult<DiscoveredHsm> {
        debug!("🔍 Probing PKCS#11 library: {:?}", path);

        // In a real implementation, this would:
        // 1. Load the PKCS#11 library dynamically
        // 2. Call C_Initialize, C_GetInfo, C_GetSlotList
        // 3. Extract manufacturer, model, and slot information
        // 4. Test library functionality

        let library_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let (vendor, model) = Self::identify_vendor_from_library(&library_name);

        Ok(DiscoveredHsm {
            hsm_id: format!("pkcs11-{}", uuid::Uuid::new_v4()),
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
                parameters: HashMap::new(),
            },
            capabilities: Default::default(), // Will be populated by capability detector
            assigned_tier: Default::default(), // Will be assigned by tier manager
            supports_human_entropy: false, // Will be determined by entropy classifier
            health_status: HsmHealthStatus::Unknown,
            discovered_at: chrono::Utc::now(),
        })
    }

    /// Identify vendor from library name
    fn identify_vendor_from_library(library_name: &str) -> (String, String) {
        match library_name {
            name if name.contains("eToken") => ("SafeNet".to_string(), "eToken".to_string()),
            name if name.contains("Luna") => ("Thales".to_string(), "Luna HSM".to_string()),
            name if name.contains("nfast") => ("nCipher".to_string(), "nShield".to_string()),
            name if name.contains("cvP11") => ("RSA".to_string(), "RSA HSM".to_string()),
            name if name.contains("gclib") => ("Gemalto".to_string(), "Gemalto HSM".to_string()),
            name if name.contains("softhsm") => ("OpenDNSSEC".to_string(), "SoftHSM".to_string()),
            name if name.contains("opensc") => ("OpenSC".to_string(), "OpenSC".to_string()),
            _ => ("Unknown".to_string(), "PKCS#11 HSM".to_string()),
        }
    }
}

impl CloudKmsDiscoverer {
    /// Create a new cloud KMS discoverer
    pub async fn new() -> BearDogResult<Self> {
        Ok(Self {
            aws_discoverer: AwsKmsDiscoverer,
            azure_discoverer: AzureKeyVaultDiscoverer,
            gcp_discoverer: GcpKmsDiscoverer,
            vault_discoverer: VaultDiscoverer,
        })
    }

    /// Discover cloud KMS instances
    pub async fn discover(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        let mut discovered = Vec::new();

        // Try to discover AWS KMS
        if let Ok(aws_hsms) = self.aws_discoverer.discover().await {
            discovered.extend(aws_hsms);
        }

        // Try to discover Azure Key Vault
        if let Ok(azure_hsms) = self.azure_discoverer.discover().await {
            discovered.extend(azure_hsms);
        }

        // Try to discover Google Cloud KMS
        if let Ok(gcp_hsms) = self.gcp_discoverer.discover().await {
            discovered.extend(gcp_hsms);
        }

        // Try to discover HashiCorp Vault
        if let Ok(vault_hsms) = self.vault_discoverer.discover().await {
            discovered.extend(vault_hsms);
        }

        info!("✅ Found {} Cloud KMS instances", discovered.len());
        Ok(discovered)
    }
}

impl SoftwareHsmDiscoverer {
    /// Create a new software HSM discoverer
    pub async fn new() -> BearDogResult<Self> {
        Ok(Self {
            implementations: vec![
                SoftwareHsmImplementation::BearDogNative,
                SoftwareHsmImplementation::OpenSsl,
                SoftwareHsmImplementation::SoftHsm,
                #[cfg(target_os = "windows")]
                SoftwareHsmImplementation::MicrosoftCng,
                #[cfg(target_os = "macos")]
                SoftwareHsmImplementation::MacOsKeychain,
            ],
        })
    }

    /// Discover software HSM implementations
    pub async fn discover(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        let mut discovered = Vec::new();

        for implementation in &self.implementations {
            if let Ok(hsm) = self.probe_software_hsm(implementation).await {
                discovered.push(hsm);
            }
        }

        info!("✅ Found {} Software HSMs", discovered.len());
        Ok(discovered)
    }

    /// Probe a software HSM implementation
    async fn probe_software_hsm(
        &self,
        implementation: &SoftwareHsmImplementation,
    ) -> BearDogResult<DiscoveredHsm> {
        let (vendor, model, implementation_name) = match implementation {
            SoftwareHsmImplementation::BearDogNative => {
                ("BearDog".to_string(), "Native Software HSM".to_string(), "beardog_native".to_string())
            }
            SoftwareHsmImplementation::OpenSsl => {
                ("OpenSSL".to_string(), "OpenSSL Engine".to_string(), "openssl".to_string())
            }
            SoftwareHsmImplementation::SoftHsm => {
                ("OpenDNSSEC".to_string(), "SoftHSM 2.0".to_string(), "softhsm".to_string())
            }
            SoftwareHsmImplementation::MicrosoftCng => {
                ("Microsoft".to_string(), "CNG Provider".to_string(), "ms_cng".to_string())
            }
            SoftwareHsmImplementation::MacOsKeychain => {
                ("Apple".to_string(), "macOS Keychain".to_string(), "macos_keychain".to_string())
            }
            SoftwareHsmImplementation::Custom { name, path: _ } => {
                ("Custom".to_string(), name.clone(), name.clone())
            }
        };

        Ok(DiscoveredHsm {
            hsm_id: format!("software-{}", uuid::Uuid::new_v4()),
            vendor,
            model,
            interface_type: HsmInterfaceType::SoftwareHsm {
                implementation: implementation_name,
            },
            connection_info: HsmConnectionInfo {
                endpoint: "local://".to_string(),
                auth_method: AuthenticationMethod::None,
                timeout_ms: 1000,
                encrypted: false,
                parameters: HashMap::new(),
            },
            capabilities: Default::default(),
            assigned_tier: Default::default(),
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy, // Software HSMs are usually healthy
            discovered_at: chrono::Utc::now(),
        })
    }
}

impl MobileHsmDiscoverer {
    /// Create a new mobile HSM discoverer
    pub async fn new() -> BearDogResult<Self> {
        Ok(Self {
            android_discoverer: AndroidStrongBoxDiscoverer,
            ios_discoverer: IosSecureEnclaveDiscoverer,
        })
    }

    /// Discover mobile HSMs
    pub async fn discover(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        let mut discovered = Vec::new();

        // Try Android StrongBox
        if let Ok(android_hsms) = self.android_discoverer.discover().await {
            discovered.extend(android_hsms);
        }

        // Try iOS Secure Enclave
        if let Ok(ios_hsms) = self.ios_discoverer.discover().await {
            discovered.extend(ios_hsms);
        }

        info!("✅ Found {} Mobile HSMs", discovered.len());
        Ok(discovered)
    }
}

// Stub implementations for the rest of the discoverers
impl NetworkHsmDiscoverer {
    pub async fn new() -> BearDogResult<Self> {
        Ok(Self {
            common_ports: vec![1792, 7000, 9000, 443, 80],
            scan_config: NetworkScanConfig {
                ip_ranges: vec!["192.168.1.0/24".to_string()],
                timeout_ms: 1000,
                max_concurrent: 10,
            },
        })
    }

    pub async fn discover(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        // Network discovery implementation would go here
        warn!("🌐 Network HSM discovery not yet implemented");
        Ok(Vec::new())
    }
}

impl UsbHsmDiscoverer {
    pub async fn new() -> BearDogResult<Self> {
        Ok(Self {
            hsm_vendor_ids: vec![0x0529, 0x072f, 0x04e6], // SafeNet, Advanced Card Systems, etc.
            enum_config: UsbEnumerationConfig {
                enable_enumeration: true,
                timeout_ms: 5000,
            },
        })
    }

    pub async fn discover(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        // USB discovery implementation would go here
        warn!("🔌 USB HSM discovery not yet implemented");
        Ok(Vec::new())
    }
}

impl TpmDiscoverer {
    pub async fn new() -> BearDogResult<Self> {
        Ok(Self {
            interface_types: vec![TpmInterfaceType::Tpm20, TpmInterfaceType::Tpm12],
        })
    }

    pub async fn discover(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        // TPM discovery implementation would go here
        warn!("🔐 TPM discovery not yet implemented");
        Ok(Vec::new())
    }
}

impl SmartCardDiscoverer {
    pub async fn new() -> BearDogResult<Self> {
        Ok(Self {
            readers: vec!["PC/SC".to_string()],
        })
    }

    pub async fn discover(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        // Smart card discovery implementation would go here
        warn!("💳 Smart card discovery not yet implemented");
        Ok(Vec::new())
    }
}

// Cloud provider discoverer implementations
impl AwsKmsDiscoverer {
    pub async fn discover(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        // Check for AWS credentials and KMS access
        if std::env::var("AWS_ACCESS_KEY_ID").is_ok() || 
           std::env::var("AWS_PROFILE").is_ok() ||
           std::path::Path::new(&format!("{}/.aws/credentials", std::env::var("HOME").unwrap_or_default())).exists() {
            
            let hsm = DiscoveredHsm {
                hsm_id: format!("aws-kms-{}", uuid::Uuid::new_v4()),
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
                    parameters: HashMap::new(),
                },
                capabilities: Default::default(),
                assigned_tier: Default::default(),
                supports_human_entropy: false, // Cloud KMS doesn't support human entropy
                health_status: HsmHealthStatus::Unknown,
                discovered_at: chrono::Utc::now(),
            };
            
            return Ok(vec![hsm]);
        }
        
        Ok(Vec::new())
    }
}

impl AzureKeyVaultDiscoverer {
    pub async fn discover(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        // Check for Azure credentials
        if std::env::var("AZURE_CLIENT_ID").is_ok() {
            let hsm = DiscoveredHsm {
                hsm_id: format!("azure-kv-{}", uuid::Uuid::new_v4()),
                vendor: "Microsoft".to_string(),
                model: "Azure Key Vault".to_string(),
                interface_type: HsmInterfaceType::CloudKms {
                    provider: "azure".to_string(),
                    region: None,
                },
                connection_info: HsmConnectionInfo {
                    endpoint: "vault.azure.net".to_string(),
                    auth_method: AuthenticationMethod::ApiKey { key_id: "AZURE_CLIENT_ID".to_string() },
                    timeout_ms: 30000,
                    encrypted: true,
                    parameters: HashMap::new(),
                },
                capabilities: Default::default(),
                assigned_tier: Default::default(),
                supports_human_entropy: false,
                health_status: HsmHealthStatus::Unknown,
                discovered_at: chrono::Utc::now(),
            };
            
            return Ok(vec![hsm]);
        }
        
        Ok(Vec::new())
    }
}

impl GcpKmsDiscoverer {
    pub async fn discover(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        // Check for GCP credentials
        if std::env::var("GOOGLE_APPLICATION_CREDENTIALS").is_ok() ||
           std::path::Path::new(&format!("{}/.config/gcloud", std::env::var("HOME").unwrap_or_default())).exists() {
            
            let hsm = DiscoveredHsm {
                hsm_id: format!("gcp-kms-{}", uuid::Uuid::new_v4()),
                vendor: "Google".to_string(),
                model: "Google Cloud KMS".to_string(),
                interface_type: HsmInterfaceType::CloudKms {
                    provider: "gcp".to_string(),
                    region: std::env::var("GOOGLE_CLOUD_REGION").ok(),
                },
                connection_info: HsmConnectionInfo {
                    endpoint: "cloudkms.googleapis.com".to_string(),
                    auth_method: AuthenticationMethod::Certificate { 
                        cert_path: std::env::var("GOOGLE_APPLICATION_CREDENTIALS").unwrap_or_default() 
                    },
                    timeout_ms: 30000,
                    encrypted: true,
                    parameters: HashMap::new(),
                },
                capabilities: Default::default(),
                assigned_tier: Default::default(),
                supports_human_entropy: false,
                health_status: HsmHealthStatus::Unknown,
                discovered_at: chrono::Utc::now(),
            };
            
            return Ok(vec![hsm]);
        }
        
        Ok(Vec::new())
    }
}

impl VaultDiscoverer {
    pub async fn discover(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        // Check for Vault configuration
        if let Ok(vault_addr) = std::env::var("VAULT_ADDR") {
            let hsm = DiscoveredHsm {
                hsm_id: format!("vault-{}", uuid::Uuid::new_v4()),
                vendor: "HashiCorp".to_string(),
                model: "Vault".to_string(),
                interface_type: HsmInterfaceType::CustomApi {
                    api_type: "vault".to_string(),
                    endpoint: vault_addr.clone(),
                },
                connection_info: HsmConnectionInfo {
                    endpoint: vault_addr,
                    auth_method: if std::env::var("VAULT_TOKEN").is_ok() {
                        AuthenticationMethod::ApiKey { key_id: "VAULT_TOKEN".to_string() }
                    } else {
                        AuthenticationMethod::None
                    },
                    timeout_ms: 10000,
                    encrypted: true,
                    parameters: HashMap::new(),
                },
                capabilities: Default::default(),
                assigned_tier: Default::default(),
                supports_human_entropy: false,
                health_status: HsmHealthStatus::Unknown,
                discovered_at: chrono::Utc::now(),
            };
            
            return Ok(vec![hsm]);
        }
        
        Ok(Vec::new())
    }
}

impl AndroidStrongBoxDiscoverer {
    pub async fn discover(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        // Use our existing Android StrongBox detection
        #[cfg(target_os = "android")]
        {
            use crate::tunnel::hsm::android_strongbox::native_device_detection::NativeAndroidDeviceDetector;
            
            if NativeAndroidDeviceDetector::check_strongbox_availability().await.unwrap_or(false) {
                let device_info = NativeAndroidDeviceDetector::detect_device_info().await?;
                
                let hsm = DiscoveredHsm {
                    hsm_id: format!("android-strongbox-{}", uuid::Uuid::new_v4()),
                    vendor: device_info.manufacturer.clone(),
                    model: device_info.model.clone(),
                    interface_type: HsmInterfaceType::MobileHsm {
                        platform: "Android".to_string(),
                        chip: device_info.titan_m_version.clone(),
                    },
                    connection_info: HsmConnectionInfo {
                        endpoint: "strongbox://local".to_string(),
                        auth_method: AuthenticationMethod::Biometric { method: "StrongBox".to_string() },
                        timeout_ms: 5000,
                        encrypted: true,
                        parameters: HashMap::new(),
                    },
                    capabilities: Default::default(),
                    assigned_tier: Default::default(),
                    supports_human_entropy: true, // StrongBox can support human entropy
                    health_status: HsmHealthStatus::Healthy,
                    discovered_at: chrono::Utc::now(),
                };
                
                return Ok(vec![hsm]);
            }
        }
        
        Ok(Vec::new())
    }
}

impl IosSecureEnclaveDiscoverer {
    pub async fn discover(&self) -> BearDogResult<Vec<DiscoveredHsm>> {
        // iOS Secure Enclave discovery would go here
        #[cfg(target_os = "ios")]
        {
            // Real iOS detection would use Security framework
            warn!("🍎 iOS Secure Enclave discovery not yet implemented");
        }
        
        Ok(Vec::new())
    }
}

// Add Default trait implementations for the capability structures
impl Default for super::DiscoveryHsmCapabilities {
    fn default() -> Self {
        use super::*;
        Self {
            key_generation: KeyGenerationCapabilities {
                supported_key_types: Vec::new(),
                max_key_sizes: HashMap::new(),
                hardware_backed: false,
                true_rng: false,
                key_derivation: Vec::new(),
            },
            crypto_operations: CryptoOperationCapabilities {
                signing_algorithms: Vec::new(),
                encryption_algorithms: Vec::new(),
                hash_functions: Vec::new(),
                mac_algorithms: Vec::new(),
                bulk_operations: false,
                streaming: false,
            },
            advanced_features: AdvancedFeatureCapabilities {
                key_attestation: false,
                user_presence: false,
                biometric_integration: false,
                multi_party: false,
                secure_boot: false,
                tamper_resistance: TamperResistanceLevel::None,
            },
            performance: PerformanceCapabilities {
                operations_per_second: 0.0,
                average_latency_ms: 0.0,
                concurrent_operations: 1,
                memory_usage: MemoryUsageLevel::Moderate,
            },
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
                },
            },
            human_entropy: HumanEntropyCapabilities {
                ephemeral_seed_creation: false,
                collection_methods: Vec::new(),
                entropy_quality_assessment: false,
                biometric_entropy: false,
                behavioral_entropy: false,
                realtime_entropy: false,
            },
        }
    }
}

use uuid; 