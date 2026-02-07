//! HSM Discovery Engine
//!
//! This module implements automatic discovery of Hardware Security Modules
//! across different platforms and interfaces.

use super::{
    AuthenticationMethod, DiscoveredHsm, HsmConnectionInfo, HsmHealthStatus, HsmInterfaceType,
    UniversalHsmCapabilities,
};
use crate::tunnel::hsm::types::HsmTier;
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

/// Main discovery engine coordinating all discoverers
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

/// PKCS#11 library discoverer
pub struct Pkcs11Discoverer {
    search_paths: Vec<PathBuf>,
    library_patterns: Vec<String>,
}

/// Cloud KMS discoverer
#[allow(dead_code)] // Fields used in implementation
pub struct CloudKmsDiscoverer {
    enabled_providers: Vec<String>,
}

/// Network HSM discoverer  
pub struct NetworkHsmDiscoverer {
    #[allow(dead_code)] // Future implementation
    common_ports: Vec<u16>,
    #[allow(dead_code)] // Future implementation
    scan_config: NetworkScanConfig,
}

/// USB HSM discoverer
pub struct UsbHsmDiscoverer {
    #[allow(dead_code)] // Future implementation
    hsm_vendor_ids: Vec<u16>,
    #[allow(dead_code)] // Future implementation
    enum_config: UsbEnumerationConfig,
}

/// Software HSM discoverer
pub struct SoftwareHsmDiscoverer {
    implementations: Vec<SoftwareHsmImplementation>,
}

/// Mobile HSM discoverer
pub struct MobileHsmDiscoverer {
    #[allow(dead_code)] // Future implementation
    android_discoverer: AndroidStrongBoxDiscoverer,
    #[allow(dead_code)] // Future implementation
    ios_discoverer: IosSecureEnclaveDiscoverer,
}

/// TPM discoverer
pub struct TpmDiscoverer {
    #[allow(dead_code)] // Future implementation
    interface_types: Vec<TpmInterfaceType>,
}

/// Smart card discoverer
pub struct SmartCardDiscoverer {
    #[allow(dead_code)] // Future implementation
    readers: Vec<String>,
}

/// Network scanning configuration
#[derive(Debug, Clone)]
pub struct NetworkScanConfig {
    pub ip_ranges: Vec<String>,
    pub timeout_ms: u32,
    pub parallel_scans: usize,
}

/// USB enumeration configuration
#[derive(Debug, Clone)]
pub struct UsbEnumerationConfig {
    pub scan_interval_ms: u32,
    pub auto_detect: bool,
}

/// TPM interface types
#[derive(Debug, Clone)]
pub enum TpmInterfaceType {
    Tpm12,
    Tpm20,
    FirmwareTpm,
    SoftwareTpm,
}

/// Software HSM implementations
#[derive(Debug, Clone)]
pub enum SoftwareHsmImplementation {
    BearDogNative,
    OpenSsl,
    SoftHsm,
    MicrosoftCng,
    MacOsKeychain,
    Custom { name: String, path: PathBuf },
}

/// Android StrongBox discoverer
pub struct AndroidStrongBoxDiscoverer;

/// iOS Secure Enclave discoverer
pub struct IosSecureEnclaveDiscoverer;

impl DiscoveryEngine {
    /// Creates a new DiscoveryEngine instance
    ///
    /// # Errors
    /// Returns an error if any discoverer fails to initialize.
    pub fn new() -> Result<Self, BearDogError> {
        info!("🔍 Initializing HSM Discovery Engine");

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
    }

    /// Discovers PKCS#11 HSMs
    pub fn discover_pkcs11_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("🔍 Discovering PKCS#11 HSMs");
        self.pkcs11_discoverer.discover()
    }

    /// Discovers Cloud KMS instances
    pub fn discover_cloud_kms_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("☁️ Discovering Cloud KMS instances");
        self.cloud_kms_discoverer.discover()
    }

    /// Discovers Network HSMs
    pub fn discover_network_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("🌐 Discovering Network HSMs");
        self.network_hsm_discoverer.discover()
    }

    /// Discovers USB HSMs
    pub fn discover_usb_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("🔌 Discovering USB HSMs");
        self.usb_hsm_discoverer.discover()
    }

    /// Discovers Software HSMs
    pub fn discover_software_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("💻 Discovering Software HSMs");
        self.software_hsm_discoverer.discover()
    }

    /// Discovers Mobile HSMs
    pub fn discover_mobile_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("📱 Discovering Mobile HSMs");
        self.mobile_hsm_discoverer.discover()
    }

    /// Discovers TPMs
    pub fn discover_tpm_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("🔐 Discovering TPMs");
        self.tpm_discoverer.discover()
    }

    /// Discovers Smart Cards
    pub fn discover_smartcard_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("💳 Discovering Smart Cards");
        self.smartcard_discoverer.discover()
    }
}

impl Pkcs11Discoverer {
    /// Creates a new PKCS#11 discoverer
    pub fn new() -> Result<Self, BearDogError> {
        let search_paths = Self::get_default_search_paths();
        let library_patterns = Self::get_library_patterns();

        Ok(Self {
            search_paths,
            library_patterns,
        })
    }

    /// Discovers PKCS#11 libraries
    pub fn discover(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
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
    }

    /// Gets default search paths for PKCS#11 libraries
    ///
    /// Paths can be overridden via BEARDOG_HSM_LIBRARY_PATHS environment variable
    /// (colon-separated on Unix, semicolon-separated on Windows)
    fn get_default_search_paths() -> Vec<PathBuf> {
        // Check for environment override first
        if let Ok(env_paths) = std::env::var("BEARDOG_HSM_LIBRARY_PATHS") {
            let separator = if cfg!(windows) { ';' } else { ':' };
            let custom_paths: Vec<PathBuf> = env_paths
                .split(separator)
                .filter(|p| !p.is_empty())
                .map(PathBuf::from)
                .collect();
            
            if !custom_paths.is_empty() {
                tracing::debug!("Using custom HSM library paths from BEARDOG_HSM_LIBRARY_PATHS");
                return custom_paths;
            }
        }
        
        // Platform-appropriate default paths
        vec![
            PathBuf::from("/usr/lib"),
            PathBuf::from("/usr/lib64"),
            PathBuf::from("/usr/local/lib"),
            PathBuf::from("/opt/lib"),
            PathBuf::from("C:\\Windows\\System32"),
            PathBuf::from("C:\\Program Files\\lib"),
            PathBuf::from("/Applications/lib"),
        ]
    }

    /// Gets library name patterns
    fn get_library_patterns() -> Vec<String> {
        vec![
            "libpkcs11.so".to_string(),
            "pkcs11.dll".to_string(),
            "libeToken.so".to_string(),
            "libcryptoki.so".to_string(),
            "libLunaAPI.so".to_string(),
            "libcknfast.so".to_string(),
            "libsofthsm2.so".to_string(),
            "opensc-pkcs11.so".to_string(),
        ]
    }

    /// Checks if a file is a PKCS#11 library
    fn is_pkcs11_library(&self, path: &Path) -> bool {
        if let Some(filename) = path.file_name() {
            let filename_str = filename.to_string_lossy();
            return self
                .library_patterns
                .iter()
                .any(|pattern| filename_str.contains(pattern));
        }
        false
    }

    /// Probes a PKCS#11 library to get HSM info
    fn probe_pkcs11_library(&self, path: &PathBuf) -> Result<DiscoveredHsm, BearDogError> {
        debug!("🔍 Probing PKCS#11 library: {:?}", path);

        let library_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let (vendor, model) = Self::identify_vendor_from_library(&library_name);

        Ok(DiscoveredHsm {
            vendor,
            model,
            interface_type: HsmInterfaceType::CustomApi {
                api_type: "pkcs11".to_string(),
                endpoint: path.to_string_lossy().to_string(),
            },
            connection_info: HsmConnectionInfo {
                endpoint: path.to_string_lossy().to_string(),
                auth_method: AuthenticationMethod::None,
                timeout_ms: 5000,
                encrypted: false,
                parameters: HashMap::new(),
            },
            capabilities: UniversalHsmCapabilities::default(),
            assigned_tier: HsmTier::Software,
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Unknown,
            discovered_at: chrono::Utc::now(),
        })
    }

    /// Identifies vendor from library name
    fn identify_vendor_from_library(library_name: &str) -> (String, String) {
        match library_name {
            name if name.contains("eToken") => ("SafeNet".to_string(), "eToken".to_string()),
            name if name.contains("Luna") => ("Thales".to_string(), "Luna HSM".to_string()),
            name if name.contains("softhsm") => ("OpenDNSSEC".to_string(), "SoftHSM".to_string()),
            name if name.contains("opensc") => ("OpenSC".to_string(), "Smart Card".to_string()),
            _ => ("Unknown".to_string(), "PKCS#11 HSM".to_string()),
        }
    }
}

impl CloudKmsDiscoverer {
    /// Creates a new Cloud KMS discoverer
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            enabled_providers: vec!["aws".to_string(), "azure".to_string(), "gcp".to_string()],
        })
    }

    /// Discovers cloud KMS instances
    pub fn discover(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        warn!("☁️ Cloud KMS discovery not yet implemented");
        Ok(Vec::new())
    }
}

impl NetworkHsmDiscoverer {
    /// Creates a new Network HSM discoverer
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            common_ports: vec![1792, 7000, 9000, 443],
            scan_config: NetworkScanConfig {
                // Example IP ranges for documentation and testing purposes.
                // In production, these would be loaded from network discovery configuration.
                ip_ranges: vec!["192.168.1.0/24".to_string()], // Example: local network
                timeout_ms: 1000,
                parallel_scans: 10,
            },
        })
    }

    /// Discovers network HSMs
    pub fn discover(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        warn!("🌐 Network HSM discovery not yet implemented");
        Ok(Vec::new())
    }
}

impl UsbHsmDiscoverer {
    /// Creates a new USB HSM discoverer
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            hsm_vendor_ids: vec![0x1050, 0x20a0, 0x04e6], // YubiKey, Nitrokey, etc.
            enum_config: UsbEnumerationConfig {
                scan_interval_ms: 1000,
                auto_detect: true,
            },
        })
    }

    /// Discovers USB HSMs
    pub fn discover(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        warn!("🔌 USB HSM discovery not yet implemented");
        Ok(Vec::new())
    }
}

impl SoftwareHsmDiscoverer {
    /// Creates a new Software HSM discoverer
    pub fn new() -> Result<Self, BearDogError> {
        let implementations = vec![
            SoftwareHsmImplementation::BearDogNative,
            SoftwareHsmImplementation::OpenSsl,
            SoftwareHsmImplementation::SoftHsm,
        ];

        #[cfg(target_os = "windows")]
        implementations.push(SoftwareHsmImplementation::MicrosoftCng);

        #[cfg(target_os = "macos")]
        implementations.push(SoftwareHsmImplementation::MacOsKeychain);

        Ok(Self { implementations })
    }

    /// Discovers software HSMs
    pub fn discover(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        let mut discovered = Vec::new();

        for implementation in &self.implementations {
            if let Ok(hsm) = self.probe_software_hsm(implementation) {
                discovered.push(hsm);
            }
        }

        info!("✅ Found {} Software HSMs", discovered.len());
        Ok(discovered)
    }

    /// Probes a software HSM implementation
    fn probe_software_hsm(
        &self,
        implementation: &SoftwareHsmImplementation,
    ) -> Result<DiscoveredHsm, BearDogError> {
        let (vendor, model, implementation_name) = match implementation {
            SoftwareHsmImplementation::BearDogNative => (
                "BearDog".to_string(),
                "Native Software HSM".to_string(),
                "beardog_native".to_string(),
            ),
            SoftwareHsmImplementation::OpenSsl => (
                "OpenSSL".to_string(),
                "OpenSSL Engine".to_string(),
                "openssl".to_string(),
            ),
            SoftwareHsmImplementation::SoftHsm => (
                "OpenDNSSEC".to_string(),
                "SoftHSM 2.0".to_string(),
                "softhsm".to_string(),
            ),
            SoftwareHsmImplementation::MicrosoftCng => (
                "Microsoft".to_string(),
                "CNG Provider".to_string(),
                "ms_cng".to_string(),
            ),
            SoftwareHsmImplementation::MacOsKeychain => (
                "Apple".to_string(),
                "macOS Keychain".to_string(),
                "macos_keychain".to_string(),
            ),
            SoftwareHsmImplementation::Custom { name, path: _ } => {
                ("Custom".to_string(), name.clone(), name.clone())
            }
        };

        Ok(DiscoveredHsm {
            vendor,
            model,
            interface_type: HsmInterfaceType::SoftwareHsm {
                implementation: implementation_name,
            },
            connection_info: HsmConnectionInfo {
                endpoint: "local://".to_string(),
                auth_method: AuthenticationMethod::None,
                timeout_ms: 100,
                encrypted: false,
                parameters: HashMap::new(),
            },
            capabilities: UniversalHsmCapabilities::default(),
            assigned_tier: HsmTier::Software,
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: chrono::Utc::now(),
        })
    }
}

impl MobileHsmDiscoverer {
    /// Creates a new Mobile HSM discoverer
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            android_discoverer: AndroidStrongBoxDiscoverer,
            ios_discoverer: IosSecureEnclaveDiscoverer,
        })
    }

    /// Discovers mobile HSMs
    pub fn discover(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        warn!("📱 Mobile HSM discovery not yet implemented");
        Ok(Vec::new())
    }
}

impl TpmDiscoverer {
    /// Creates a new TPM discoverer
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            interface_types: vec![TpmInterfaceType::Tpm20, TpmInterfaceType::Tpm12],
        })
    }

    /// Discovers TPMs
    pub fn discover(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        warn!("🔐 TPM discovery not yet implemented");
        Ok(Vec::new())
    }
}

impl SmartCardDiscoverer {
    /// Creates a new Smart Card discoverer
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            readers: Vec::new(),
        })
    }

    /// Discovers smart cards
    pub fn discover(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        warn!("💳 Smart Card discovery not yet implemented");
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_discovery_engine_creation() -> Result<(), Box<dyn std::error::Error>> {
        let engine = DiscoveryEngine::new();
        assert!(engine.is_ok());
        Ok(())
    }

    #[test]
    fn test_pkcs11_discoverer() -> Result<(), Box<dyn std::error::Error>> {
        let discoverer = Pkcs11Discoverer::new();
        assert!(discoverer.is_ok());
        Ok(())
    }

    #[test]
    fn test_software_hsm_discovery() -> Result<(), Box<dyn std::error::Error>> {
        let discoverer = SoftwareHsmDiscoverer::new()?;
        let result = discoverer.discover();
        assert!(result.is_ok());
        Ok(())
    }

    // ========================================================================
    // COMPREHENSIVE DISCOVERY ENGINE TESTS (Discovery System Sprint)
    // ========================================================================

    #[tokio::test]
    async fn test_all_discoverers_initialization() -> Result<(), Box<dyn std::error::Error>> {
        // Test that all discoverers can be created
        let pkcs11 = Pkcs11Discoverer::new();
        let cloud_kms = CloudKmsDiscoverer::new();
        let network = NetworkHsmDiscoverer::new();
        let usb = UsbHsmDiscoverer::new();
        let software = SoftwareHsmDiscoverer::new();
        let mobile = MobileHsmDiscoverer::new();
        let tpm = TpmDiscoverer::new();
        let smartcard = SmartCardDiscoverer::new();

        assert!(pkcs11.is_ok());
        assert!(cloud_kms.is_ok());
        assert!(network.is_ok());
        assert!(usb.is_ok());
        assert!(software.is_ok());
        assert!(mobile.is_ok());
        assert!(tpm.is_ok());
        assert!(smartcard.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_discovery_engine_full_scan() -> Result<(), Box<dyn std::error::Error>> {
        let engine = DiscoveryEngine::new()?;

        // Test all discovery methods
        let pkcs11_result = engine.discover_pkcs11_hsms();
        let cloud_result = engine.discover_cloud_kms_hsms();
        let network_result = engine.discover_network_hsms();
        let usb_result = engine.discover_usb_hsms();
        let software_result = engine.discover_software_hsms();
        let mobile_result = engine.discover_mobile_hsms();
        let tpm_result = engine.discover_tpm_hsms();
        let smartcard_result = engine.discover_smartcard_hsms();

        // All methods should return Ok (even if empty)
        assert!(pkcs11_result.is_ok());
        assert!(cloud_result.is_ok());
        assert!(network_result.is_ok());
        assert!(usb_result.is_ok());
        assert!(software_result.is_ok());
        assert!(mobile_result.is_ok());
        assert!(tpm_result.is_ok());
        assert!(smartcard_result.is_ok());

        Ok(())
    }

    #[test]
    fn test_network_scan_config() -> Result<(), Box<dyn std::error::Error>> {
        let config = NetworkScanConfig {
            // Example IP ranges for documentation purposes. In production, load from config.
            ip_ranges: vec![
                "192.168.1.0/24".to_string(), // Example: local network
                "10.0.0.0/8".to_string(),     // Example: private network
            ],
            timeout_ms: 5000,
            parallel_scans: 10,
        };

        assert_eq!(config.ip_ranges.len(), 2);
        assert_eq!(config.timeout_ms, 5000);
        assert_eq!(config.parallel_scans, 10);

        Ok(())
    }

    #[test]
    fn test_usb_enumeration_config() -> Result<(), Box<dyn std::error::Error>> {
        let config = UsbEnumerationConfig {
            scan_interval_ms: 1000,
            auto_detect: true,
        };

        assert_eq!(config.scan_interval_ms, 1000);
        assert!(config.auto_detect);

        Ok(())
    }

    #[test]
    fn test_tpm_interface_types() -> Result<(), Box<dyn std::error::Error>> {
        // Test all TPM interface types
        let types = [
            TpmInterfaceType::Tpm12,
            TpmInterfaceType::Tpm20,
            TpmInterfaceType::FirmwareTpm,
            TpmInterfaceType::SoftwareTpm,
        ];

        assert_eq!(types.len(), 4);

        Ok(())
    }

    #[test]
    fn test_software_hsm_implementations() -> Result<(), Box<dyn std::error::Error>> {
        // Test all software HSM implementation types
        let impls = vec![
            SoftwareHsmImplementation::BearDogNative,
            SoftwareHsmImplementation::OpenSsl,
            SoftwareHsmImplementation::SoftHsm,
            SoftwareHsmImplementation::MicrosoftCng,
            SoftwareHsmImplementation::MacOsKeychain,
            SoftwareHsmImplementation::Custom {
                name: "CustomHSM".to_string(),
                path: PathBuf::from("/opt/custom-hsm"),
            },
        ];

        assert_eq!(impls.len(), 6);

        Ok(())
    }

    #[tokio::test]
    async fn test_concurrent_discovery_engine_creation() -> Result<(), Box<dyn std::error::Error>> {
        // Test creating multiple discovery engines concurrently
        let mut handles = vec![];

        for _ in 0..5 {
            let handle = tokio::spawn(async { DiscoveryEngine::new() });
            handles.push(handle);
        }

        // All should succeed
        for handle in handles {
            let result = handle.await?;
            assert!(result.is_ok());
        }

        Ok(())
    }

    #[test]
    fn test_pkcs11_search_paths() -> Result<(), Box<dyn std::error::Error>> {
        let discoverer = Pkcs11Discoverer::new()?;

        // Verify discoverer has search paths configured
        assert!(!discoverer.search_paths.is_empty());

        Ok(())
    }

    #[test]
    fn test_cloud_kms_providers() -> Result<(), Box<dyn std::error::Error>> {
        let discoverer = CloudKmsDiscoverer::new()?;

        // Verify discoverer has providers configured
        assert!(!discoverer.enabled_providers.is_empty());

        Ok(())
    }

    #[test]
    fn test_network_hsm_ports() -> Result<(), Box<dyn std::error::Error>> {
        let discoverer = NetworkHsmDiscoverer::new()?;

        // Verify discoverer has common ports configured
        assert!(!discoverer.common_ports.is_empty());

        Ok(())
    }

    #[test]
    fn test_usb_hsm_vendor_ids() -> Result<(), Box<dyn std::error::Error>> {
        let discoverer = UsbHsmDiscoverer::new()?;

        // Verify discoverer has vendor IDs configured
        assert!(!discoverer.hsm_vendor_ids.is_empty());

        Ok(())
    }

    #[test]
    fn test_software_hsm_implementations_list() -> Result<(), Box<dyn std::error::Error>> {
        let discoverer = SoftwareHsmDiscoverer::new()?;

        // Verify discoverer has implementations configured
        assert!(!discoverer.implementations.is_empty());

        Ok(())
    }

    #[test]
    fn test_tpm_interface_types_list() -> Result<(), Box<dyn std::error::Error>> {
        let discoverer = TpmDiscoverer::new()?;

        // Verify discoverer has interface types configured
        assert!(!discoverer.interface_types.is_empty());

        Ok(())
    }

    #[test]
    fn test_network_scan_config_clone() -> Result<(), Box<dyn std::error::Error>> {
        const TEST_TIMEOUT_MS: u32 = 3000;
        let config1 = NetworkScanConfig {
            ip_ranges: vec!["192.168.1.0/24".to_string()],
            timeout_ms: TEST_TIMEOUT_MS,
            parallel_scans: 5,
        };

        let config2 = config1.clone();

        assert_eq!(config1.ip_ranges, config2.ip_ranges);
        assert_eq!(config1.timeout_ms, config2.timeout_ms);
        assert_eq!(config1.parallel_scans, config2.parallel_scans);

        Ok(())
    }

    #[test]
    fn test_usb_config_clone() -> Result<(), Box<dyn std::error::Error>> {
        let config1 = UsbEnumerationConfig {
            scan_interval_ms: 2000,
            auto_detect: false,
        };

        let config2 = config1.clone();

        assert_eq!(config1.scan_interval_ms, config2.scan_interval_ms);
        assert_eq!(config1.auto_detect, config2.auto_detect);

        Ok(())
    }

    #[test]
    fn test_custom_software_hsm() -> Result<(), Box<dyn std::error::Error>> {
        let custom = SoftwareHsmImplementation::Custom {
            name: "MyCustomHSM".to_string(),
            path: PathBuf::from("/usr/local/lib/custom-hsm.so"),
        };

        if let SoftwareHsmImplementation::Custom { name, path } = custom {
            assert_eq!(name, "MyCustomHSM");
            assert_eq!(path, PathBuf::from("/usr/local/lib/custom-hsm.so"));
        } else {
            panic!("Expected Custom variant");
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_discovery_engine_repeated_scans() -> Result<(), Box<dyn std::error::Error>> {
        let engine = DiscoveryEngine::new()?;

        // Test that we can run discoveries multiple times
        for _ in 0..3 {
            let software_result = engine.discover_software_hsms();
            assert!(software_result.is_ok());
        }

        Ok(())
    }

    #[test]
    fn test_network_scan_config_with_empty_ranges() -> Result<(), Box<dyn std::error::Error>> {
        // Edge case: empty IP ranges
        let config = NetworkScanConfig {
            ip_ranges: Vec::new(),
            timeout_ms: 5000,
            parallel_scans: 10,
        };

        assert!(config.ip_ranges.is_empty());

        Ok(())
    }

    #[test]
    fn test_network_scan_config_with_many_ranges() -> Result<(), Box<dyn std::error::Error>> {
        // Test with many IP ranges
        let ranges: Vec<String> = (0..10).map(|i| format!("192.168.{}.0/24", i)).collect();

        let config = NetworkScanConfig {
            ip_ranges: ranges.clone(),
            timeout_ms: 5000,
            parallel_scans: 20,
        };

        assert_eq!(config.ip_ranges.len(), 10);

        Ok(())
    }

    #[test]
    fn test_usb_config_variations() -> Result<(), Box<dyn std::error::Error>> {
        // Test various USB configuration combinations
        let configs = vec![
            UsbEnumerationConfig {
                scan_interval_ms: 100,
                auto_detect: true,
            },
            UsbEnumerationConfig {
                scan_interval_ms: 5000,
                auto_detect: false,
            },
            UsbEnumerationConfig {
                scan_interval_ms: 0,
                auto_detect: true,
            },
        ];

        for config in configs {
            // All configurations should be valid
            let _ = config.scan_interval_ms;
            let _ = config.auto_detect;
        }

        Ok(())
    }
}
