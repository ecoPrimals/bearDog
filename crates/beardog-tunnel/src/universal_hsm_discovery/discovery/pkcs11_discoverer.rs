// SPDX-License-Identifier: AGPL-3.0-only

//! PKCS#11 HSM Discoverer
//!
//! Provides discovery functionality for PKCS#11-based HSMs
//!
//! This module discovers HSMs accessible via PKCS#11 libraries, including:
//! - Hardware HSMs (Thales, Utimaco, etc.)
//! - SoftHSM instances
//! - SmartCard tokens
//! - TPM via PKCS#11 provider
//! - Other PKCS#11-compatible devices

use super::super::*;
use beardog_errors::BearDogError;
use chrono::Utc;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

/// Known PKCS#11 library locations by platform
const PKCS11_COMMON_PATHS: &[&str] = &[
    // SoftHSM
    "/usr/lib/softhsm/libsofthsm2.so",
    "/usr/lib/x86_64-linux-gnu/softhsm/libsofthsm2.so",
    "/usr/local/lib/softhsm/libsofthsm2.so",
    "/opt/homebrew/lib/softhsm/libsofthsm2.so",
    
    // OpenSC (smartcard)
    "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so",
    "/usr/lib/opensc-pkcs11.so",
    "/usr/local/lib/opensc-pkcs11.so",
    
    // TPM PKCS#11
    "/usr/lib/x86_64-linux-gnu/libtpm2_pkcs11.so",
    "/usr/lib/libtpm2_pkcs11.so",
    
    // YubiKey
    "/usr/lib/x86_64-linux-gnu/libykcs11.so",
    "/usr/local/lib/libykcs11.so",
    
    // Thales/nCipher
    "/opt/nfast/toolkits/pkcs11/libcknfast.so",
    
    // Utimaco
    "/opt/utimaco/lib/libcs_pkcs11_R2.so",
    
    // AWS CloudHSM
    "/opt/cloudhsm/lib/libcloudhsm_pkcs11.so",
];

/// PKCS#11 token information
#[derive(Debug, Clone)]
pub struct Pkcs11TokenInfo {
    /// Library path
    pub library_path: PathBuf,
    /// Token label
    pub label: String,
    /// Manufacturer ID
    pub manufacturer_id: String,
    /// Model
    pub model: String,
    /// Serial number
    pub serial_number: String,
    /// Slot ID
    pub slot_id: u64,
}

/// PKCS#11 HSM discoverer
#[derive(Debug, Clone)]
pub struct Pkcs11Discoverer {
    /// Additional library paths to check
    custom_library_paths: Vec<PathBuf>,
    /// Whether to scan system library directories
    enable_system_scan: bool,
}

impl Pkcs11Discoverer {
    /// Create new PKCS#11 discoverer
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            custom_library_paths: Vec::new(),
            enable_system_scan: true,
        })
    }

    /// Add custom library path to scan
    pub fn add_library_path(&mut self, path: PathBuf) {
        self.custom_library_paths.push(path);
    }

    /// Discover PKCS#11 HSMs
    ///
    /// # Errors
    /// Returns an error if discovery fails
    pub async fn discover(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("🔍 Discovering PKCS#11 HSMs");
        
        let mut discovered = Vec::new();
        let mut found_libraries = HashSet::new();

        // Discover PKCS#11 libraries
        let library_paths = self.find_pkcs11_libraries().await?;
        debug!("Found {} potential PKCS#11 libraries", library_paths.len());

        for lib_path in library_paths {
            // Skip duplicates
            if !found_libraries.insert(lib_path.clone()) {
                continue;
            }

            // Try to enumerate tokens from this library
            match self.enumerate_tokens(&lib_path).await {
                Ok(tokens) => {
                    for token_info in tokens {
                        if let Some(hsm) = self.create_hsm_from_token(&token_info) {
                            info!("✓ Found PKCS#11 HSM: {}", hsm.name);
                            discovered.push(hsm);
                        }
                    }
                }
                Err(e) => {
                    debug!("Failed to enumerate tokens from {:?}: {}", lib_path, e);
                }
            }
        }

        info!("✅ PKCS#11 discovery complete: {} HSMs found", discovered.len());
        Ok(discovered)
    }

    /// Find PKCS#11 libraries on the system
    ///
    /// # Errors
    /// Returns an error if library search fails
    async fn find_pkcs11_libraries(&self) -> Result<Vec<PathBuf>, BearDogError> {
        let mut libraries = Vec::new();

        // Check common paths
        for path_str in PKCS11_COMMON_PATHS {
            let path = Path::new(path_str);
            if path.exists() {
                debug!("Found PKCS#11 library: {:?}", path);
                libraries.push(path.to_path_buf());
            }
        }

        // Check custom paths
        for path in &self.custom_library_paths {
            if path.exists() {
                debug!("Found custom PKCS#11 library: {:?}", path);
                libraries.push(path.clone());
            }
        }

        // Check environment variable
        if let Ok(p11_lib) = std::env::var("PKCS11_MODULE") {
            let path = PathBuf::from(p11_lib);
            if path.exists() {
                debug!("Found PKCS#11 library from env: {:?}", path);
                libraries.push(path);
            }
        }

        // Platform-specific discovery
        #[cfg(target_os = "macos")]
        {
            libraries.extend(self.find_macos_pkcs11_libraries().await?);
        }

        #[cfg(target_os = "windows")]
        {
            libraries.extend(self.find_windows_pkcs11_libraries().await?);
        }

        Ok(libraries)
    }

    /// Find PKCS#11 libraries on macOS
    ///
    /// # Errors
    /// Returns an error if discovery fails
    #[cfg(target_os = "macos")]
    async fn find_macos_pkcs11_libraries(&self) -> Result<Vec<PathBuf>, BearDogError> {
        let mut libraries = Vec::new();

        // Check Homebrew locations
        let homebrew_paths = [
            "/opt/homebrew/lib",
            "/usr/local/lib",
        ];

        for base_path in &homebrew_paths {
            if let Ok(entries) = std::fs::read_dir(base_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if name.contains("pkcs11") && name.ends_with(".dylib") {
                            debug!("Found macOS PKCS#11 library: {:?}", path);
                            libraries.push(path);
                        }
                    }
                }
            }
        }

        Ok(libraries)
    }

    /// Find PKCS#11 libraries on Windows
    ///
    /// # Errors
    /// Returns an error if discovery fails
    #[cfg(target_os = "windows")]
    async fn find_windows_pkcs11_libraries(&self) -> Result<Vec<PathBuf>, BearDogError> {
        let mut libraries = Vec::new();

        // Check common Windows locations
        let windows_paths = [
            r"C:\Program Files\OpenSC Project\OpenSC\pkcs11",
            r"C:\Program Files (x86)\OpenSC Project\OpenSC\pkcs11",
            r"C:\Windows\System32",
        ];

        for base_path in &windows_paths {
            if let Ok(entries) = std::fs::read_dir(base_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if name.contains("pkcs11") && name.ends_with(".dll") {
                            debug!("Found Windows PKCS#11 library: {:?}", path);
                            libraries.push(path);
                        }
                    }
                }
            }
        }

        Ok(libraries)
    }

    /// Enumerate tokens from a PKCS#11 library
    ///
    /// # Errors
    /// Returns an error if enumeration fails
    async fn enumerate_tokens(&self, library_path: &Path) -> Result<Vec<Pkcs11TokenInfo>, BearDogError> {
        debug!("Enumerating tokens from {:?}", library_path);

        // In a full implementation, this would:
        // 1. Load the PKCS#11 library using dlopen/LoadLibrary
        // 2. Call C_Initialize()
        // 3. Call C_GetSlotList() to get slots
        // 4. Call C_GetTokenInfo() for each slot
        // 5. Extract token information
        // 6. Call C_Finalize()
        //
        // For now, we'll create synthetic token info for known libraries

        let mut tokens = Vec::new();

        // Detect library type from path
        let lib_name = library_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        if lib_name.contains("softhsm") {
            tokens.push(Pkcs11TokenInfo {
                library_path: library_path.to_path_buf(),
                label: "SoftHSM Token".to_string(),
                manufacturer_id: "SoftHSM Project".to_string(),
                model: "SoftHSM v2".to_string(),
                serial_number: "0000000000000000".to_string(),
                slot_id: 0,
            });
        } else if lib_name.contains("opensc") {
            tokens.push(Pkcs11TokenInfo {
                library_path: library_path.to_path_buf(),
                label: "SmartCard Token".to_string(),
                manufacturer_id: "OpenSC Project".to_string(),
                model: "PKCS#11".to_string(),
                serial_number: "0000000000000001".to_string(),
                slot_id: 0,
            });
        } else if lib_name.contains("tpm2") {
            tokens.push(Pkcs11TokenInfo {
                library_path: library_path.to_path_buf(),
                label: "TPM PKCS#11 Token".to_string(),
                manufacturer_id: "TPM2 Software".to_string(),
                model: "TPM 2.0".to_string(),
                serial_number: "0000000000000002".to_string(),
                slot_id: 0,
            });
        } else if lib_name.contains("ykcs11") {
            tokens.push(Pkcs11TokenInfo {
                library_path: library_path.to_path_buf(),
                label: "YubiKey PIV".to_string(),
                manufacturer_id: "Yubico".to_string(),
                model: "YubiKey 5".to_string(),
                serial_number: "0000000000000003".to_string(),
                slot_id: 0,
            });
        } else if lib_name.contains("cknfast") {
            tokens.push(Pkcs11TokenInfo {
                library_path: library_path.to_path_buf(),
                label: "Thales HSM".to_string(),
                manufacturer_id: "Thales".to_string(),
                model: "nShield".to_string(),
                serial_number: "0000000000000004".to_string(),
                slot_id: 0,
            });
        } else if lib_name.contains("cloudhsm") {
            tokens.push(Pkcs11TokenInfo {
                library_path: library_path.to_path_buf(),
                label: "AWS CloudHSM".to_string(),
                manufacturer_id: "Amazon Web Services".to_string(),
                model: "CloudHSM".to_string(),
                serial_number: "0000000000000005".to_string(),
                slot_id: 0,
            });
        } else {
            // Generic PKCS#11 token
            tokens.push(Pkcs11TokenInfo {
                library_path: library_path.to_path_buf(),
                label: "Generic PKCS#11 Token".to_string(),
                manufacturer_id: "Unknown".to_string(),
                model: "PKCS#11".to_string(),
                serial_number: "0000000000000099".to_string(),
                slot_id: 0,
            });
        }

        Ok(tokens)
    }

    /// Create DiscoveredHsm from PKCS#11 token info
    fn create_hsm_from_token(&self, token_info: &Pkcs11TokenInfo) -> Option<DiscoveredHsm> {
        let now = Utc::now();

        // Determine HSM type and tier based on manufacturer
        let (hsm_type, tier, capabilities) = self.classify_token(token_info);

        Some(DiscoveredHsm {
            name: format!("pkcs11-{}-{}", 
                token_info.manufacturer_id.to_lowercase().replace(' ', "-"),
                token_info.slot_id),
            hsm_type,
            endpoint: HsmEndpoint {
                host: token_info.library_path.to_string_lossy().to_string(),
                port: Some(token_info.slot_id as u16),
                protocol: "pkcs11".to_string(),
                secure: true,
            },
            capabilities,
            assigned_tier: tier,
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: now,
            last_health_check: now,
            integration_status: IntegrationStatus::Discovered,
        })
    }

    /// Classify token and determine capabilities
    fn classify_token(&self, token_info: &Pkcs11TokenInfo) -> (HsmType, HsmTier, UniversalHsmCapabilities) {
        use crate::tunnel::hsm::types::capability::*;

        // Classify based on manufacturer
        match token_info.manufacturer_id.to_lowercase().as_str() {
            s if s.contains("thales") || s.contains("ncipher") => {
                (HsmType::Hardware, HsmTier::Tier1, self.create_enterprise_hsm_capabilities())
            }
            s if s.contains("utimaco") => {
                (HsmType::Hardware, HsmTier::Tier1, self.create_enterprise_hsm_capabilities())
            }
            s if s.contains("amazon") || s.contains("cloudhsm") => {
                (HsmType::Cloud, HsmTier::Tier1, self.create_cloud_hsm_capabilities())
            }
            s if s.contains("yubico") => {
                (HsmType::Hardware, HsmTier::Tier2, self.create_yubikey_pkcs11_capabilities())
            }
            s if s.contains("tpm") => {
                (HsmType::Hardware, HsmTier::Tier2, self.create_tpm_pkcs11_capabilities())
            }
            s if s.contains("softhsm") => {
                (HsmType::Software, HsmTier::Tier3, self.create_softhsm_capabilities())
            }
            s if s.contains("opensc") => {
                (HsmType::Hardware, HsmTier::Tier3, self.create_smartcard_capabilities())
            }
            _ => {
                (HsmType::Software, HsmTier::Tier4, self.create_generic_pkcs11_capabilities())
            }
        }
    }

    /// Create capabilities for enterprise HSMs
    fn create_enterprise_hsm_capabilities(&self) -> UniversalHsmCapabilities {
        use crate::tunnel::hsm::types::capability::*;

        UniversalHsmCapabilities {
            crypto_operations: CryptoOperationCapabilities {
                symmetric_encryption: vec![
                    "AES-128".to_string(),
                    "AES-256".to_string(),
                    "3DES".to_string(),
                ],
                asymmetric_encryption: vec![
                    "RSA-2048".to_string(),
                    "RSA-4096".to_string(),
                    "ECC-P256".to_string(),
                    "ECC-P384".to_string(),
                    "ECC-P521".to_string(),
                ],
                signing: vec![
                    "RSA-PSS".to_string(),
                    "ECDSA".to_string(),
                    "Ed25519".to_string(),
                ],
                hashing: vec!["SHA-256".to_string(), "SHA-384".to_string(), "SHA-512".to_string()],
                key_agreement: vec!["ECDH".to_string(), "X25519".to_string()],
            },
            key_management: KeyManagementCapabilities {
                key_generation: true,
                key_storage: true,
                key_rotation: true,
                key_backup: true,
                key_recovery: true,
            },
            key_generation: KeyGenerationCapabilities {
                rsa: vec![2048, 3072, 4096],
                ecc: vec![256, 384, 521],
                aes: vec![128, 192, 256],
                supports_secure_random: true,
            },
            security: SecurityCapabilities {
                tamper_resistance: TamperResistance::Tier1,
                fips_140_2_level: Some(3),
                common_criteria_eal: Some(4),
                secure_boot: true,
                attestation: true,
            },
            performance: PerformanceCapabilities {
                max_operations_per_second: 10000,
                typical_latency_ms: 5.0,
                supports_parallel_operations: true,
                hardware_acceleration: true,
            },
            compliance: ComplianceCapabilities {
                fips_140_2: true,
                common_criteria: true,
                pci_dss: true,
                hipaa: true,
                gdpr: true,
            },
            api_support: ApiSupportCapabilities {
                pkcs11: true,
                tpm2: false,
                kmip: true,
                pkcs7: true,
            },
            advanced_features: AdvancedFeatureCapabilities {
                quantum_resistant: true,
                multi_party_computation: true,
                threshold_cryptography: true,
                homomorphic_encryption: false,
            },
            human_entropy: HumanEntropyCapabilities {
                supported: false,
                methods: vec![],
                quality_score: 0.0,
            },
        }
    }

    /// Create capabilities for cloud HSMs
    fn create_cloud_hsm_capabilities(&self) -> UniversalHsmCapabilities {
        use crate::tunnel::hsm::types::capability::*;

        let mut caps = self.create_enterprise_hsm_capabilities();
        caps.performance.max_operations_per_second = 25000;
        caps.performance.typical_latency_ms = 10.0;
        caps.security.fips_140_2_level = Some(3);
        caps
    }

    /// Create capabilities for YubiKey via PKCS#11
    fn create_yubikey_pkcs11_capabilities(&self) -> UniversalHsmCapabilities {
        use crate::tunnel::hsm::types::capability::*;

        UniversalHsmCapabilities {
            crypto_operations: CryptoOperationCapabilities {
                symmetric_encryption: vec!["AES-128".to_string(), "AES-256".to_string()],
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
                aes: vec![128, 256],
                supports_secure_random: true,
            },
            security: SecurityCapabilities {
                tamper_resistance: TamperResistance::Tier2,
                fips_140_2_level: Some(2),
                common_criteria_eal: None,
                secure_boot: true,
                attestation: true,
            },
            performance: PerformanceCapabilities {
                max_operations_per_second: 100,
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

    /// Create capabilities for TPM via PKCS#11
    fn create_tpm_pkcs11_capabilities(&self) -> UniversalHsmCapabilities {
        use crate::tunnel::hsm::types::capability::*;

        UniversalHsmCapabilities {
            crypto_operations: CryptoOperationCapabilities {
                symmetric_encryption: vec!["AES-128".to_string(), "AES-256".to_string()],
                asymmetric_encryption: vec!["RSA-2048".to_string(), "ECC-P256".to_string()],
                signing: vec!["RSA-PSS".to_string(), "ECDSA".to_string()],
                hashing: vec!["SHA-256".to_string(), "SHA-384".to_string()],
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
                aes: vec![128, 256],
                supports_secure_random: true,
            },
            security: SecurityCapabilities {
                tamper_resistance: TamperResistance::Tier2,
                fips_140_2_level: Some(2),
                common_criteria_eal: Some(4),
                secure_boot: true,
                attestation: true,
            },
            performance: PerformanceCapabilities {
                max_operations_per_second: 500,
                typical_latency_ms: 20.0,
                supports_parallel_operations: false,
                hardware_acceleration: true,
            },
            compliance: ComplianceCapabilities {
                fips_140_2: true,
                common_criteria: true,
                pci_dss: false,
                hipaa: false,
                gdpr: true,
            },
            api_support: ApiSupportCapabilities {
                pkcs11: true,
                tpm2: true,
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

    /// Create capabilities for SoftHSM
    fn create_softhsm_capabilities(&self) -> UniversalHsmCapabilities {
        use crate::tunnel::hsm::types::capability::*;

        UniversalHsmCapabilities {
            crypto_operations: CryptoOperationCapabilities {
                symmetric_encryption: vec!["AES-128".to_string(), "AES-256".to_string()],
                asymmetric_encryption: vec!["RSA-2048".to_string(), "RSA-4096".to_string(), "ECC-P256".to_string()],
                signing: vec!["RSA-PSS".to_string(), "ECDSA".to_string()],
                hashing: vec!["SHA-256".to_string(), "SHA-512".to_string()],
                key_agreement: vec!["ECDH".to_string()],
            },
            key_management: KeyManagementCapabilities {
                key_generation: true,
                key_storage: true,
                key_rotation: false,
                key_backup: true,
                key_recovery: true,
            },
            key_generation: KeyGenerationCapabilities {
                rsa: vec![2048, 4096],
                ecc: vec![256, 384],
                aes: vec![128, 256],
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
                max_operations_per_second: 5000,
                typical_latency_ms: 1.0,
                supports_parallel_operations: true,
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

    /// Create capabilities for SmartCard tokens
    fn create_smartcard_capabilities(&self) -> UniversalHsmCapabilities {
        use crate::tunnel::hsm::types::capability::*;

        UniversalHsmCapabilities {
            crypto_operations: CryptoOperationCapabilities {
                symmetric_encryption: vec!["AES-128".to_string()],
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
                aes: vec![128],
                supports_secure_random: true,
            },
            security: SecurityCapabilities {
                tamper_resistance: TamperResistance::Tier3,
                fips_140_2_level: None,
                common_criteria_eal: Some(4),
                secure_boot: false,
                attestation: false,
            },
            performance: PerformanceCapabilities {
                max_operations_per_second: 50,
                typical_latency_ms: 100.0,
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

    /// Create capabilities for generic PKCS#11 tokens
    fn create_generic_pkcs11_capabilities(&self) -> UniversalHsmCapabilities {
        use crate::tunnel::hsm::types::capability::*;

        UniversalHsmCapabilities {
            crypto_operations: CryptoOperationCapabilities {
                symmetric_encryption: vec!["AES-256".to_string()],
                asymmetric_encryption: vec!["RSA-2048".to_string()],
                signing: vec!["RSA-PSS".to_string()],
                hashing: vec!["SHA-256".to_string()],
                key_agreement: vec![],
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
                ecc: vec![],
                aes: vec![256],
                supports_secure_random: true,
            },
            security: SecurityCapabilities {
                tamper_resistance: TamperResistance::Tier4,
                fips_140_2_level: None,
                common_criteria_eal: None,
                secure_boot: false,
                attestation: false,
            },
            performance: PerformanceCapabilities {
                max_operations_per_second: 100,
                typical_latency_ms: 50.0,
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

impl Default for Pkcs11Discoverer {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            custom_library_paths: Vec::new(),
            enable_system_scan: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discoverer_creation() {
        let discoverer = Pkcs11Discoverer::new();
        assert!(discoverer.is_ok());
    }

    #[tokio::test]
    async fn test_pkcs11_discovery() {
        let discoverer = Pkcs11Discoverer::new()?;
        let result = discoverer.discover().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_library_discovery() {
        let discoverer = Pkcs11Discoverer::new()?;
        let libraries = discoverer.find_pkcs11_libraries().await;
        assert!(libraries.is_ok());
    }

    #[test]
    fn test_token_classification() {
        let discoverer = Pkcs11Discoverer::new()?;

        // Test Thales classification
        let thales_token = Pkcs11TokenInfo {
            library_path: PathBuf::from("/opt/nfast/lib/libcknfast.so"),
            label: "Thales Token".to_string(),
            manufacturer_id: "Thales".to_string(),
            model: "nShield".to_string(),
            serial_number: "123456".to_string(),
            slot_id: 0,
        };
        let (hsm_type, tier, _) = discoverer.classify_token(&thales_token);
        assert_eq!(hsm_type, HsmType::Hardware);
        assert_eq!(tier, HsmTier::Tier1);

        // Test SoftHSM classification
        let softhsm_token = Pkcs11TokenInfo {
            library_path: PathBuf::from("/usr/lib/softhsm/libsofthsm2.so"),
            label: "SoftHSM Token".to_string(),
            manufacturer_id: "SoftHSM Project".to_string(),
            model: "SoftHSM v2".to_string(),
            serial_number: "789012".to_string(),
            slot_id: 0,
        };
        let (hsm_type, tier, _) = discoverer.classify_token(&softhsm_token);
        assert_eq!(hsm_type, HsmType::Software);
        assert_eq!(tier, HsmTier::Tier3);
    }

    #[test]
    fn test_enterprise_hsm_capabilities() {
        let discoverer = Pkcs11Discoverer::new()?;
        let caps = discoverer.create_enterprise_hsm_capabilities();
        
        assert!(caps.security.fips_140_2_level == Some(3));
        assert!(caps.security.tamper_resistance == TamperResistance::Tier1);
        assert!(caps.key_management.key_backup);
        assert!(caps.advanced_features.quantum_resistant);
        assert!(caps.api_support.pkcs11);
    }

    #[test]
    fn test_custom_library_paths() {
        let mut discoverer = Pkcs11Discoverer::new()?;
        discoverer.add_library_path(PathBuf::from("/custom/path/lib.so"));
        assert_eq!(discoverer.custom_library_paths.len(), 1);
    }
}

