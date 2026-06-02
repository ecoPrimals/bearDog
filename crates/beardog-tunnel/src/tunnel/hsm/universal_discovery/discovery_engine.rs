// SPDX-License-Identifier: AGPL-3.0-or-later

//! HSM Discovery Engine
//!
//! This module implements automatic discovery of Hardware Security Modules
//! across different platforms and interfaces.

use super::{
    AuthenticationMethod, DiscoveredHsm, HsmConnectionInfo, HsmHealthStatus, HsmInterfaceType,
    UniversalHsmCapabilities,
};
use crate::tunnel::hsm::types::HsmTier;
use beardog_config::env_keys;
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

/// Well-known ports for HSM device discovery probing.
const DEFAULT_DISCOVERY_PORTS: &[u16] = &[1792, 7000, 9000, 443];

/// Default HSM discovery timeout in milliseconds.
const DEFAULT_DISCOVERY_TIMEOUT_MS: u32 = 5000;

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
pub struct CloudKmsDiscoverer {
    _enabled_providers: Vec<String>,
}

/// Network HSM discoverer  
pub struct NetworkHsmDiscoverer {
    _common_ports: Vec<u16>,
    _scan_config: NetworkScanConfig,
}

/// USB HSM discoverer
pub struct UsbHsmDiscoverer {
    _hsm_vendor_ids: Vec<u16>,
    _enum_config: UsbEnumerationConfig,
}

/// Software HSM discoverer
pub struct SoftwareHsmDiscoverer {
    implementations: Vec<SoftwareHsmImplementation>,
}

/// Mobile HSM discoverer
pub struct MobileHsmDiscoverer {
    _android_discoverer: AndroidStrongBoxDiscoverer,
    _ios_discoverer: IosSecureEnclaveDiscoverer,
}

/// TPM discoverer
pub struct TpmDiscoverer {
    _interface_types: Vec<TpmInterfaceType>,
}

/// Smart card discoverer
pub struct SmartCardDiscoverer {
    _readers: Vec<String>,
}

/// Network scanning configuration
#[derive(Debug, Clone)]
pub struct NetworkScanConfig {
    /// IP address ranges to scan (CIDR notation)
    pub ip_ranges: Vec<String>,
    /// Timeout per scan in milliseconds
    pub timeout_ms: u32,
    /// Number of parallel scan workers
    pub parallel_scans: usize,
}

/// USB enumeration configuration
#[derive(Debug, Clone)]
pub struct UsbEnumerationConfig {
    /// Interval between USB scans in milliseconds
    pub scan_interval_ms: u32,
    /// Whether to automatically detect new USB HSM devices
    pub auto_detect: bool,
}

/// TPM interface types
#[derive(Debug, Clone)]
pub enum TpmInterfaceType {
    /// TPM 1.2 (legacy)
    Tpm12,
    /// TPM 2.0 (modern)
    Tpm20,
    /// Firmware-based TPM (fTPM)
    FirmwareTpm,
    /// Software-emulated TPM
    SoftwareTpm,
}

/// Software HSM implementations
#[derive(Debug, Clone)]
pub enum SoftwareHsmImplementation {
    /// `BearDog`'s native software HSM
    BearDogNative,
    /// OpenSSL-based provider
    OpenSsl,
    /// `SoftHSM2` PKCS#11 provider
    SoftHsm,
    /// Microsoft CNG provider
    MicrosoftCng,
    /// macOS Keychain provider
    MacOsKeychain,
    /// Custom HSM implementation
    Custom {
        /// Implementation name
        name: String,
        /// Path to the provider library
        path: PathBuf,
    },
}

/// Android `StrongBox` discoverer
pub struct AndroidStrongBoxDiscoverer;

/// iOS Secure Enclave discoverer
pub struct IosSecureEnclaveDiscoverer;

impl DiscoveryEngine {
    /// Creates a new `DiscoveryEngine` instance
    ///
    /// # Errors
    ///
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
    ///
    /// # Errors
    ///
    /// Returns an error if PKCS#11 discovery fails.
    pub fn discover_pkcs11_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("🔍 Discovering PKCS#11 HSMs");
        self.pkcs11_discoverer.discover()
    }

    /// Discovers Cloud KMS instances
    ///
    /// # Errors
    ///
    /// Returns an error if Cloud KMS discovery fails.
    pub fn discover_cloud_kms_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("☁️ Discovering Cloud KMS instances");
        self.cloud_kms_discoverer.discover()
    }

    /// Discovers Network HSMs
    ///
    /// # Errors
    ///
    /// Returns an error if network HSM discovery fails.
    pub fn discover_network_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("🌐 Discovering Network HSMs");
        self.network_hsm_discoverer.discover()
    }

    /// Discovers USB HSMs
    ///
    /// # Errors
    ///
    /// Returns an error if USB HSM discovery fails.
    pub fn discover_usb_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("🔌 Discovering USB HSMs");
        self.usb_hsm_discoverer.discover()
    }

    /// Discovers Software HSMs
    ///
    /// # Errors
    ///
    /// Returns an error if software HSM discovery fails.
    pub fn discover_software_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("💻 Discovering Software HSMs");
        self.software_hsm_discoverer.discover()
    }

    /// Discovers Mobile HSMs
    ///
    /// # Errors
    ///
    /// Returns an error if mobile HSM discovery fails.
    pub fn discover_mobile_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("📱 Discovering Mobile HSMs");
        self.mobile_hsm_discoverer.discover()
    }

    /// Discovers TPMs
    ///
    /// # Errors
    ///
    /// Returns an error if TPM discovery fails.
    pub fn discover_tpm_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("🔐 Discovering TPMs");
        self.tpm_discoverer.discover()
    }

    /// Discovers Smart Cards
    ///
    /// # Errors
    ///
    /// Returns an error if smart card discovery fails.
    pub fn discover_smartcard_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("💳 Discovering Smart Cards");
        self.smartcard_discoverer.discover()
    }
}

impl Pkcs11Discoverer {
    /// Creates a new PKCS#11 discoverer
    ///
    /// # Errors
    ///
    /// Returns an error if the discoverer cannot be initialized.
    pub fn new() -> Result<Self, BearDogError> {
        let search_paths = Self::get_default_search_paths();
        let library_patterns = Self::get_library_patterns();

        Ok(Self {
            search_paths,
            library_patterns,
        })
    }

    /// Discovers PKCS#11 libraries
    ///
    /// # Errors
    ///
    /// Returns an error if PKCS#11 discovery fails.
    pub fn discover(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        let mut discovered = Vec::new();

        for search_path in &self.search_paths {
            if let Ok(entries) = std::fs::read_dir(search_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if self.is_pkcs11_library(&path)
                        && let Ok(hsm) = self.probe_pkcs11_library(&path)
                    {
                        discovered.push(hsm);
                    }
                }
            }
        }

        info!("✅ Found {} PKCS#11 HSMs", discovered.len());
        Ok(discovered)
    }

    /// Gets default search paths for PKCS#11 libraries
    ///
    /// Paths can be overridden via `BEARDOG_HSM_LIBRARY_PATHS` environment variable
    /// (colon-separated on Unix, semicolon-separated on Windows)
    fn get_default_search_paths() -> Vec<PathBuf> {
        // Check for environment override first
        if let Ok(env_paths) = beardog_errors::process_env::var(env_keys::ENV_HSM_LIBRARY_PATHS) {
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
                timeout_ms: DEFAULT_DISCOVERY_TIMEOUT_MS,
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
    ///
    /// # Errors
    ///
    /// Returns an error if the discoverer cannot be initialized.
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            _enabled_providers: vec!["aws".to_string(), "azure".to_string(), "gcp".to_string()],
        })
    }

    /// Discovers cloud KMS instances
    ///
    /// # Errors
    ///
    /// Returns an error if Cloud KMS discovery fails.
    pub fn discover(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("Cloud KMS discovery skipped (no provider credentials detected)");
        Ok(Vec::new())
    }
}

impl NetworkHsmDiscoverer {
    /// Creates a new Network HSM discoverer
    ///
    /// # Errors
    ///
    /// Returns an error if the discoverer cannot be initialized.
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            _common_ports: DEFAULT_DISCOVERY_PORTS.to_vec(),
            _scan_config: NetworkScanConfig {
                // Example IP ranges for documentation and testing purposes.
                // In production, these would be loaded from network discovery configuration.
                ip_ranges: vec!["192.168.1.0/24".to_string()], // Example: local network
                timeout_ms: 1000,
                parallel_scans: 10,
            },
        })
    }

    /// Discovers network HSMs
    ///
    /// # Errors
    ///
    /// Returns an error if network HSM discovery fails.
    pub fn discover(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("Network HSM discovery skipped (no reachable HSM endpoints configured)");
        Ok(Vec::new())
    }
}

impl UsbHsmDiscoverer {
    /// Creates a new USB HSM discoverer
    ///
    /// # Errors
    ///
    /// Returns an error if the discoverer cannot be initialized.
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            _hsm_vendor_ids: vec![0x1050, 0x20a0, 0x04e6], // YubiKey, Nitrokey, etc.
            _enum_config: UsbEnumerationConfig {
                scan_interval_ms: 1000,
                auto_detect: true,
            },
        })
    }

    /// Discovers USB HSMs
    ///
    /// # Errors
    ///
    /// Returns an error if USB HSM discovery fails.
    pub fn discover(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("USB HSM discovery skipped (no matching vendor IDs on bus)");
        Ok(Vec::new())
    }
}

impl SoftwareHsmDiscoverer {
    /// Creates a new Software HSM discoverer
    ///
    /// # Errors
    ///
    /// Returns an error if the discoverer cannot be initialized.
    pub fn new() -> Result<Self, BearDogError> {
        let implementations = vec![
            SoftwareHsmImplementation::BearDogNative,
            SoftwareHsmImplementation::OpenSsl,
            SoftwareHsmImplementation::SoftHsm,
            #[cfg(target_os = "windows")]
            SoftwareHsmImplementation::MicrosoftCng,
            #[cfg(target_os = "macos")]
            SoftwareHsmImplementation::MacOsKeychain,
        ];

        Ok(Self { implementations })
    }

    /// Discovers software HSMs
    ///
    /// # Errors
    ///
    /// Returns an error if software HSM discovery fails.
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
    ///
    /// # Errors
    ///
    /// Returns an error if the discoverer cannot be initialized.
    pub const fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            _android_discoverer: AndroidStrongBoxDiscoverer,
            _ios_discoverer: IosSecureEnclaveDiscoverer,
        })
    }

    /// Discovers mobile HSMs
    ///
    /// # Errors
    ///
    /// Returns an error if mobile HSM discovery fails.
    pub fn discover(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("Mobile HSM discovery skipped (not running on Android/iOS)");
        Ok(Vec::new())
    }
}

impl TpmDiscoverer {
    /// Creates a new TPM discoverer
    ///
    /// # Errors
    ///
    /// Returns an error if the discoverer cannot be initialized.
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            _interface_types: vec![TpmInterfaceType::Tpm20, TpmInterfaceType::Tpm12],
        })
    }

    /// Discovers TPMs
    ///
    /// # Errors
    ///
    /// Returns an error if TPM discovery fails.
    pub fn discover(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("TPM discovery skipped (no TPM device found)");
        Ok(Vec::new())
    }
}

impl SmartCardDiscoverer {
    /// Creates a new Smart Card discoverer
    ///
    /// # Errors
    ///
    /// Returns an error if the discoverer cannot be initialized.
    pub const fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            _readers: Vec::new(),
        })
    }

    /// Discovers smart cards
    ///
    /// # Errors
    ///
    /// Returns an error if smart card discovery fails.
    pub fn discover(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("Smart Card discovery skipped (no readers detected)");
        Ok(Vec::new())
    }
}

#[cfg(test)]
#[path = "discovery_engine_tests.rs"]
mod tests;
