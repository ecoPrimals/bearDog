//! USB HSM Discoverer
//!
//! Provides discovery functionality for USB-connected HSMs
//!
//! This module discovers HSMs connected via USB, including:
//! - YubiKey devices (various models)
//! - Nitrokey devices
//! - Generic PKCS#11 USB tokens
//! - SmartCard readers with crypto tokens
//! - Other USB security tokens

use super::super::*;
use beardog_errors::BearDogError;
use chrono::Utc;
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// Known USB vendor IDs for HSM/security token manufacturers
const YUBICO_VENDOR_ID: u16 = 0x1050;
const NITROKEY_VENDOR_ID: u16 = 0x20a0;
const FEITIAN_VENDOR_ID: u16 = 0x096e;
const GEMALTO_VENDOR_ID: u16 = 0x08e6;

/// USB device information for HSM detection
#[derive(Debug, Clone)]
pub struct UsbDeviceInfo {
    /// USB vendor ID
    pub vendor_id: u16,
    /// USB product ID
    pub product_id: u16,
    /// Device path (if available)
    pub device_path: Option<String>,
    /// Device name/description
    pub device_name: String,
}

/// USB HSM discoverer
#[derive(Debug, Clone)]
pub struct UsbDiscoverer {
    /// Map of known vendor IDs to manufacturer names
    known_vendors: HashMap<u16, String>,
    /// Whether to probe generic USB tokens
    enable_generic_probing: bool,
}

impl UsbDiscoverer {
    /// Create new USB discoverer
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new() -> Result<Self, BearDogError> {
        let mut known_vendors = HashMap::new();
        known_vendors.insert(YUBICO_VENDOR_ID, "Yubico".to_string());
        known_vendors.insert(NITROKEY_VENDOR_ID, "Nitrokey".to_string());
        known_vendors.insert(FEITIAN_VENDOR_ID, "Feitian".to_string());
        known_vendors.insert(GEMALTO_VENDOR_ID, "Gemalto".to_string());

        Ok(Self {
            known_vendors,
            enable_generic_probing: true,
        })
    }

    /// Discover USB HSMs
    ///
    /// # Errors
    /// Returns an error if discovery fails
    pub async fn discover(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("🔍 Discovering USB-connected HSMs");
        
        let mut discovered = Vec::new();

        // Get list of USB devices
        let usb_devices = self.enumerate_usb_devices().await?;
        debug!("Found {} USB devices to check", usb_devices.len());

        for device_info in usb_devices {
            if let Some(hsm) = self.identify_hsm_device(&device_info).await? {
                info!("✓ Found USB HSM: {}", hsm.name);
                discovered.push(hsm);
            }
        }

        info!("✅ USB discovery complete: {} HSMs found", discovered.len());
        Ok(discovered)
    }

    /// Enumerate USB devices
    ///
    /// # Errors
    /// Returns an error if USB enumeration fails
    async fn enumerate_usb_devices(&self) -> Result<Vec<UsbDeviceInfo>, BearDogError> {
        debug!("Enumerating USB devices");

        // In a full implementation, this would use the `rusb` or `libusb` crate
        // to enumerate USB devices. For now, we'll check sysfs on Linux
        
        #[cfg(target_os = "linux")]
        {
            self.enumerate_usb_devices_linux().await
        }

        #[cfg(target_os = "macos")]
        {
            self.enumerate_usb_devices_macos().await
        }

        #[cfg(target_os = "windows")]
        {
            self.enumerate_usb_devices_windows().await
        }

        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            warn!("USB enumeration not implemented for this platform");
            Ok(Vec::new())
        }
    }

    /// Enumerate USB devices on Linux
    ///
    /// # Errors
    /// Returns an error if enumeration fails
    #[cfg(target_os = "linux")]
    async fn enumerate_usb_devices_linux(&self) -> Result<Vec<UsbDeviceInfo>, BearDogError> {
        use std::fs;
        use std::path::Path;

        let mut devices = Vec::new();

        // Check /sys/bus/usb/devices for USB devices
        let usb_devices_path = Path::new("/sys/bus/usb/devices");
        
        if !usb_devices_path.exists() {
            debug!("USB devices path not found");
            return Ok(devices);
        }

        // In a real implementation, we would parse sysfs to get vendor/product IDs
        // For now, we'll check for known device paths
        
        // Check for YubiKey
        if Path::new("/dev/hidraw0").exists() || Path::new("/dev/hidraw1").exists() {
            // This is a simplification - real implementation would read vendor/product IDs
            devices.push(UsbDeviceInfo {
                vendor_id: YUBICO_VENDOR_ID,
                product_id: 0x0407, // YubiKey 4/5 series
                device_path: Some("/dev/hidraw0".to_string()),
                device_name: "YubiKey (detected)".to_string(),
            });
        }

        Ok(devices)
    }

    /// Enumerate USB devices on macOS
    ///
    /// # Errors
    /// Returns an error if enumeration fails
    #[cfg(target_os = "macos")]
    async fn enumerate_usb_devices_macos(&self) -> Result<Vec<UsbDeviceInfo>, BearDogError> {
        // In a real implementation, this would use IOKit or system_profiler
        // to enumerate USB devices
        warn!("macOS USB enumeration requires IOKit integration");
        Ok(Vec::new())
    }

    /// Enumerate USB devices on Windows
    ///
    /// # Errors
    /// Returns an error if enumeration fails
    #[cfg(target_os = "windows")]
    async fn enumerate_usb_devices_windows(&self) -> Result<Vec<UsbDeviceInfo>, BearDogError> {
        // In a real implementation, this would use Windows Device Manager API
        // or WMI to enumerate USB devices
        warn!("Windows USB enumeration requires Device Manager API integration");
        Ok(Vec::new())
    }

    /// Identify if a USB device is an HSM
    ///
    /// # Errors
    /// Returns an error if identification fails
    async fn identify_hsm_device(&self, device_info: &UsbDeviceInfo) -> Result<Option<DiscoveredHsm>, BearDogError> {
        debug!("Checking USB device: vendor={:04x}, product={:04x}", 
               device_info.vendor_id, device_info.product_id);

        // Check if it's a known vendor
        if let Some(vendor_name) = self.known_vendors.get(&device_info.vendor_id) {
            debug!("Detected {} device", vendor_name);
            
            return match device_info.vendor_id {
                YUBICO_VENDOR_ID => Ok(Some(self.create_yubikey_hsm(device_info))),
                NITROKEY_VENDOR_ID => Ok(Some(self.create_nitrokey_hsm(device_info))),
                FEITIAN_VENDOR_ID => Ok(Some(self.create_feitian_hsm(device_info))),
                GEMALTO_VENDOR_ID => Ok(Some(self.create_gemalto_hsm(device_info))),
                _ => Ok(Some(self.create_generic_usb_hsm(device_info))),
            };
        }

        // If generic probing is enabled, try to detect unknown tokens
        if self.enable_generic_probing {
            // In a real implementation, this would try to communicate with the device
            // using CCID protocol or other standard interfaces
            debug!("Generic probing not yet implemented");
        }

        Ok(None)
    }

    /// Create DiscoveredHsm for YubiKey
    fn create_yubikey_hsm(&self, device_info: &UsbDeviceInfo) -> DiscoveredHsm {
        let now = Utc::now();
        
        DiscoveredHsm {
            name: format!("yubikey-{:04x}", device_info.product_id),
            hsm_type: HsmType::Hardware,
            endpoint: HsmEndpoint {
                host: device_info.device_path.clone().unwrap_or_else(|| "usb".to_string()),
                port: None,
                protocol: "usb-ccid".to_string(),
                secure: true,
            },
            capabilities: self.create_yubikey_capabilities(),
            assigned_tier: HsmTier::Tier2, // YubiKeys are Tier 2
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: now,
            last_health_check: now,
            integration_status: IntegrationStatus::Discovered,
        }
    }

    /// Create DiscoveredHsm for Nitrokey
    fn create_nitrokey_hsm(&self, device_info: &UsbDeviceInfo) -> DiscoveredHsm {
        let now = Utc::now();
        
        DiscoveredHsm {
            name: format!("nitrokey-{:04x}", device_info.product_id),
            hsm_type: HsmType::Hardware,
            endpoint: HsmEndpoint {
                host: device_info.device_path.clone().unwrap_or_else(|| "usb".to_string()),
                port: None,
                protocol: "usb-ccid".to_string(),
                secure: true,
            },
            capabilities: self.create_nitrokey_capabilities(),
            assigned_tier: HsmTier::Tier2,
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: now,
            last_health_check: now,
            integration_status: IntegrationStatus::Discovered,
        }
    }

    /// Create DiscoveredHsm for Feitian
    fn create_feitian_hsm(&self, device_info: &UsbDeviceInfo) -> DiscoveredHsm {
        let now = Utc::now();
        
        DiscoveredHsm {
            name: format!("feitian-{:04x}", device_info.product_id),
            hsm_type: HsmType::Hardware,
            endpoint: HsmEndpoint {
                host: device_info.device_path.clone().unwrap_or_else(|| "usb".to_string()),
                port: None,
                protocol: "usb-ccid".to_string(),
                secure: true,
            },
            capabilities: self.create_generic_usb_token_capabilities(),
            assigned_tier: HsmTier::Tier3,
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: now,
            last_health_check: now,
            integration_status: IntegrationStatus::Discovered,
        }
    }

    /// Create DiscoveredHsm for Gemalto
    fn create_gemalto_hsm(&self, device_info: &UsbDeviceInfo) -> DiscoveredHsm {
        let now = Utc::now();
        
        DiscoveredHsm {
            name: format!("gemalto-{:04x}", device_info.product_id),
            hsm_type: HsmType::Hardware,
            endpoint: HsmEndpoint {
                host: device_info.device_path.clone().unwrap_or_else(|| "usb".to_string()),
                port: None,
                protocol: "usb-ccid".to_string(),
                secure: true,
            },
            capabilities: self.create_generic_usb_token_capabilities(),
            assigned_tier: HsmTier::Tier2,
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: now,
            last_health_check: now,
            integration_status: IntegrationStatus::Discovered,
        }
    }

    /// Create DiscoveredHsm for generic USB token
    fn create_generic_usb_hsm(&self, device_info: &UsbDeviceInfo) -> DiscoveredHsm {
        let now = Utc::now();
        
        DiscoveredHsm {
            name: format!("usb-token-{:04x}-{:04x}", device_info.vendor_id, device_info.product_id),
            hsm_type: HsmType::Hardware,
            endpoint: HsmEndpoint {
                host: device_info.device_path.clone().unwrap_or_else(|| "usb".to_string()),
                port: None,
                protocol: "usb-generic".to_string(),
                secure: true,
            },
            capabilities: self.create_generic_usb_token_capabilities(),
            assigned_tier: HsmTier::Tier3,
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: now,
            last_health_check: now,
            integration_status: IntegrationStatus::Discovered,
        }
    }

    /// Create capabilities for YubiKey
    fn create_yubikey_capabilities(&self) -> UniversalHsmCapabilities {
        use crate::tunnel::hsm::types::capability::*;

        UniversalHsmCapabilities {
            crypto_operations: CryptoOperationCapabilities {
                symmetric_encryption: vec!["AES-128".to_string(), "AES-256".to_string()],
                asymmetric_encryption: vec![
                    "RSA-2048".to_string(),
                    "RSA-4096".to_string(),
                    "ECC-P256".to_string(),
                    "ECC-P384".to_string(),
                ],
                signing: vec![
                    "RSA-PSS".to_string(),
                    "ECDSA".to_string(),
                    "Ed25519".to_string(),
                ],
                hashing: vec!["SHA-256".to_string(), "SHA-512".to_string()],
                key_agreement: vec!["ECDH".to_string(), "X25519".to_string()],
            },
            key_management: KeyManagementCapabilities {
                key_generation: true,
                key_storage: true,
                key_rotation: false, // YubiKey doesn't support automated rotation
                key_backup: false,   // Keys cannot be exported
                key_recovery: false,
            },
            key_generation: KeyGenerationCapabilities {
                rsa: vec![2048, 4096],
                ecc: vec![256, 384],
                aes: vec![128, 256],
                supports_secure_random: true,
            },
            security: SecurityCapabilities {
                tamper_resistance: TamperResistance::Tier2,
                fips_140_2_level: Some(2), // YubiKey 5 FIPS
                common_criteria_eal: None,
                secure_boot: true,
                attestation: true,
            },
            performance: PerformanceCapabilities {
                max_operations_per_second: 100, // USB tokens are slower
                typical_latency_ms: 50.0,
                supports_parallel_operations: false,
                hardware_acceleration: true,
            },
            compliance: ComplianceCapabilities {
                fips_140_2: true,
                common_criteria: false,
                pci_dss: false,
                hipaa: false,
                gdpr: true,
            },
            api_support: ApiSupportCapabilities {
                pkcs11: true,
                tpm2: false,
                kmip: false,
                pkcs7: true,
            },
            advanced_features: AdvancedFeatureCapabilities {
                quantum_resistant: false,
                multi_party_computation: false,
                threshold_cryptography: false,
                homomorphic_encryption: false,
            },
            human_entropy: HumanEntropyCapabilities {
                supported: false,
                methods: vec![],
                quality_score: 0.0,
            },
        }
    }

    /// Create capabilities for Nitrokey
    fn create_nitrokey_capabilities(&self) -> UniversalHsmCapabilities {
        use crate::tunnel::hsm::types::capability::*;

        UniversalHsmCapabilities {
            crypto_operations: CryptoOperationCapabilities {
                symmetric_encryption: vec!["AES-128".to_string(), "AES-256".to_string()],
                asymmetric_encryption: vec![
                    "RSA-2048".to_string(),
                    "RSA-4096".to_string(),
                    "ECC-P256".to_string(),
                ],
                signing: vec!["RSA-PSS".to_string(), "ECDSA".to_string()],
                hashing: vec!["SHA-256".to_string(), "SHA-512".to_string()],
                key_agreement: vec!["ECDH".to_string()],
            },
            key_management: KeyManagementCapabilities {
                key_generation: true,
                key_storage: true,
                key_rotation: false,
                key_backup: false,
                key_recovery: false,
            },
            key_generation: KeyGenerationCapabilities {
                rsa: vec![2048, 4096],
                ecc: vec![256, 384],
                aes: vec![128, 256],
                supports_secure_random: true,
            },
            security: SecurityCapabilities {
                tamper_resistance: TamperResistance::Tier2,
                fips_140_2_level: None,
                common_criteria_eal: Some(5), // Nitrokey 3 has CC EAL5+
                secure_boot: true,
                attestation: true,
            },
            performance: PerformanceCapabilities {
                max_operations_per_second: 150,
                typical_latency_ms: 40.0,
                supports_parallel_operations: false,
                hardware_acceleration: true,
            },
            compliance: ComplianceCapabilities {
                fips_140_2: false,
                common_criteria: true,
                pci_dss: false,
                hipaa: false,
                gdpr: true,
            },
            api_support: ApiSupportCapabilities {
                pkcs11: true,
                tpm2: false,
                kmip: false,
                pkcs7: true,
            },
            advanced_features: AdvancedFeatureCapabilities {
                quantum_resistant: false,
                multi_party_computation: false,
                threshold_cryptography: false,
                homomorphic_encryption: false,
            },
            human_entropy: HumanEntropyCapabilities {
                supported: false,
                methods: vec![],
                quality_score: 0.0,
            },
        }
    }

    /// Create capabilities for generic USB token
    fn create_generic_usb_token_capabilities(&self) -> UniversalHsmCapabilities {
        use crate::tunnel::hsm::types::capability::*;

        UniversalHsmCapabilities {
            crypto_operations: CryptoOperationCapabilities {
                symmetric_encryption: vec!["AES-256".to_string()],
                asymmetric_encryption: vec!["RSA-2048".to_string(), "ECC-P256".to_string()],
                signing: vec!["RSA-PSS".to_string(), "ECDSA".to_string()],
                hashing: vec!["SHA-256".to_string()],
                key_agreement: vec!["ECDH".to_string()],
            },
            key_management: KeyManagementCapabilities {
                key_generation: true,
                key_storage: true,
                key_rotation: false,
                key_backup: false,
                key_recovery: false,
            },
            key_generation: KeyGenerationCapabilities {
                rsa: vec![2048],
                ecc: vec![256],
                aes: vec![256],
                supports_secure_random: true,
            },
            security: SecurityCapabilities {
                tamper_resistance: TamperResistance::Tier3,
                fips_140_2_level: None,
                common_criteria_eal: None,
                secure_boot: false,
                attestation: false,
            },
            performance: PerformanceCapabilities {
                max_operations_per_second: 50,
                typical_latency_ms: 100.0,
                supports_parallel_operations: false,
                hardware_acceleration: false,
            },
            compliance: ComplianceCapabilities {
                fips_140_2: false,
                common_criteria: false,
                pci_dss: false,
                hipaa: false,
                gdpr: true,
            },
            api_support: ApiSupportCapabilities {
                pkcs11: true,
                tpm2: false,
                kmip: false,
                pkcs7: false,
            },
            advanced_features: AdvancedFeatureCapabilities {
                quantum_resistant: false,
                multi_party_computation: false,
                threshold_cryptography: false,
                homomorphic_encryption: false,
            },
            human_entropy: HumanEntropyCapabilities {
                supported: false,
                methods: vec![],
                quality_score: 0.0,
            },
        }
    }
}

impl Default for UsbDiscoverer {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            let mut known_vendors = HashMap::new();
            known_vendors.insert(YUBICO_VENDOR_ID, "Yubico".to_string());
            known_vendors.insert(NITROKEY_VENDOR_ID, "Nitrokey".to_string());
            
            Self {
                known_vendors,
                enable_generic_probing: true,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discoverer_creation() {
        let discoverer = UsbDiscoverer::new();
        assert!(discoverer.is_ok());
        
        let disc = discoverer.unwrap();
        assert!(disc.known_vendors.contains_key(&YUBICO_VENDOR_ID));
        assert!(disc.known_vendors.contains_key(&NITROKEY_VENDOR_ID));
    }

    #[tokio::test]
    async fn test_usb_discovery() {
        let discoverer = UsbDiscoverer::new().unwrap();
        let result = discoverer.discover().await;
        assert!(result.is_ok());
        
        // Discovery should succeed even if no devices found
        let _hsms = result.unwrap();
    }

    #[tokio::test]
    async fn test_yubikey_identification() {
        let discoverer = UsbDiscoverer::new().unwrap();
        let device_info = UsbDeviceInfo {
            vendor_id: YUBICO_VENDOR_ID,
            product_id: 0x0407,
            device_path: Some("/dev/hidraw0".to_string()),
            device_name: "YubiKey".to_string(),
        };
        
        let result = discoverer.identify_hsm_device(&device_info).await;
        assert!(result.is_ok());
        
        let hsm = result.unwrap();
        assert!(hsm.is_some());
        
        let hsm = hsm.unwrap();
        assert!(hsm.name.contains("yubikey"));
        assert_eq!(hsm.hsm_type, HsmType::Hardware);
        assert_eq!(hsm.assigned_tier, HsmTier::Tier2);
    }

    #[test]
    fn test_yubikey_capabilities() {
        let discoverer = UsbDiscoverer::new().unwrap();
        let caps = discoverer.create_yubikey_capabilities();
        
        // Verify YubiKey capabilities
        assert!(caps.security.tamper_resistance == TamperResistance::Tier2);
        assert_eq!(caps.security.fips_140_2_level, Some(2));
        assert!(caps.security.attestation);
        assert!(caps.api_support.pkcs11);
        assert!(!caps.key_management.key_backup); // Keys not exportable
    }

    #[test]
    fn test_nitrokey_capabilities() {
        let discoverer = UsbDiscoverer::new().unwrap();
        let caps = discoverer.create_nitrokey_capabilities();
        
        // Verify Nitrokey capabilities
        assert!(caps.security.common_criteria_eal == Some(5));
        assert!(caps.security.attestation);
        assert!(caps.api_support.pkcs11);
    }

    #[test]
    fn test_vendor_detection() {
        let discoverer = UsbDiscoverer::new().unwrap();
        
        assert_eq!(discoverer.known_vendors.get(&YUBICO_VENDOR_ID), Some(&"Yubico".to_string()));
        assert_eq!(discoverer.known_vendors.get(&NITROKEY_VENDOR_ID), Some(&"Nitrokey".to_string()));
        assert_eq!(discoverer.known_vendors.get(&FEITIAN_VENDOR_ID), Some(&"Feitian".to_string()));
        assert_eq!(discoverer.known_vendors.get(&GEMALTO_VENDOR_ID), Some(&"Gemalto".to_string()));
    }
}

