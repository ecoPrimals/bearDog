//! Mobile HSM Discoverer
//!
//! Provides discovery functionality for mobile platform HSMs
//!
//! This module discovers mobile-specific HSMs:
//! - iOS Secure Enclave (on iOS devices)
//! - Android StrongBox HSM (hardware security module, official API: KeyMaster)
//! - Android Keystore (on Android devices)
//! - Samsung Knox (on Samsung devices)

use super::super::*;
use beardog_errors::BearDogError;
use chrono::Utc;
use std::env;
use std::path::Path;
use tracing::{debug, info, warn};

/// Mobile platform types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MobilePlatform {
    /// iOS/iPadOS/watchOS
    Ios,
    /// Android
    Android,
    /// Unknown mobile platform
    Unknown,
}

/// Mobile HSM type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MobileHsmType {
    /// iOS Secure Enclave
    IosSecureEnclave,
    /// Android StrongBox HSM (official Android API: KeyMaster)
    AndroidStrongBox,
    /// Android Keystore (software or hardware-backed)
    AndroidKeystore,
    /// Samsung Knox
    SamsungKnox,
}

/// Mobile HSM discoverer
#[derive(Debug, Clone)]
pub struct MobileDiscoverer {
    /// Detected platform
    platform: MobilePlatform,
    /// Whether to check for StrongBox on Android
    check_strongbox: bool,
    /// Whether to check for Samsung Knox
    check_knox: bool,
}

impl MobileDiscoverer {
    /// Create new mobile discoverer
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new() -> Result<Self, BearDogError> {
        let platform = Self::detect_platform();
        
        Ok(Self {
            platform,
            check_strongbox: true,
            check_knox: true,
        })
    }

    /// Detect current mobile platform
    fn detect_platform() -> MobilePlatform {
        #[cfg(target_os = "ios")]
        {
            return MobilePlatform::Ios;
        }

        #[cfg(target_os = "android")]
        {
            return MobilePlatform::Android;
        }

        // Check environment hints for cross-compilation or testing
        if let Ok(platform) = env::var("MOBILE_PLATFORM") {
            match platform.to_lowercase().as_str() {
                "ios" => return MobilePlatform::Ios,
                "android" => return MobilePlatform::Android,
                _ => {}
            }
        }

        MobilePlatform::Unknown
    }

    /// Discover mobile HSMs
    ///
    /// # Errors
    /// Returns an error if discovery fails
    pub async fn discover(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("🔍 Discovering mobile HSMs (platform: {:?})", self.platform);
        
        let mut discovered = Vec::new();

        match self.platform {
            MobilePlatform::Ios => {
                discovered.extend(self.discover_ios_hsms().await?);
            }
            MobilePlatform::Android => {
                discovered.extend(self.discover_android_hsms().await?);
            }
            MobilePlatform::Unknown => {
                debug!("Not running on mobile platform, skipping mobile HSM discovery");
            }
        }

        info!("✅ Mobile HSM discovery complete: {} HSMs found", discovered.len());
        Ok(discovered)
    }

    /// Discover iOS HSMs
    ///
    /// # Errors
    /// Returns an error if discovery fails
    async fn discover_ios_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        let mut hsms = Vec::new();

        // Check for Secure Enclave
        if let Some(secure_enclave) = self.detect_ios_secure_enclave().await? {
            info!("✓ Found iOS Secure Enclave");
            hsms.push(secure_enclave);
        }

        // iOS Keychain is always available
        hsms.push(self.create_ios_keychain_hsm());

        Ok(hsms)
    }

    /// Detect iOS Secure Enclave
    ///
    /// # Errors
    /// Returns an error if detection fails
    #[cfg(target_os = "ios")]
    async fn detect_ios_secure_enclave(&self) -> Result<Option<DiscoveredHsm>, BearDogError> {
        // In a full implementation, this would check:
        // 1. Device capability using Security framework
        // 2. kSecAttrTokenIDSecureEnclave availability
        // 3. Device model (A7+ chip required)
        
        // For now, assume Secure Enclave is available on iOS
        Ok(Some(self.create_ios_secure_enclave_hsm()))
    }

    /// Detect iOS Secure Enclave (non-iOS platforms)
    ///
    /// # Errors
    /// Returns an error if detection fails
    #[cfg(not(target_os = "ios"))]
    async fn detect_ios_secure_enclave(&self) -> Result<Option<DiscoveredHsm>, BearDogError> {
        // Check for simulator or test environment
        if env::var("IOS_SECURE_ENCLAVE_AVAILABLE").is_ok() {
            Ok(Some(self.create_ios_secure_enclave_hsm()))
        } else {
            Ok(None)
        }
    }

    /// Create iOS Secure Enclave HSM
    fn create_ios_secure_enclave_hsm(&self) -> DiscoveredHsm {
        let now = Utc::now();
        
        DiscoveredHsm {
            name: "ios-secure-enclave".to_string(),
            hsm_type: HsmType::Hardware,
            endpoint: HsmEndpoint {
                host: "local".to_string(),
                port: None,
                protocol: "ios-security-framework".to_string(),
                secure: true,
            },
            capabilities: self.create_ios_secure_enclave_capabilities(),
            assigned_tier: HsmTier::Tier1, // Secure Enclave is Tier 1
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: now,
            last_health_check: now,
            integration_status: IntegrationStatus::Discovered,
        }
    }

    /// Create iOS Keychain HSM
    fn create_ios_keychain_hsm(&self) -> DiscoveredHsm {
        let now = Utc::now();
        
        DiscoveredHsm {
            name: "ios-keychain".to_string(),
            hsm_type: HsmType::Software,
            endpoint: HsmEndpoint {
                host: "local".to_string(),
                port: None,
                protocol: "ios-keychain".to_string(),
                secure: true,
            },
            capabilities: self.create_ios_keychain_capabilities(),
            assigned_tier: HsmTier::Tier3, // Software keychain is Tier 3
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: now,
            last_health_check: now,
            integration_status: IntegrationStatus::Discovered,
        }
    }

    /// Discover Android HSMs
    ///
    /// # Errors
    /// Returns an error if discovery fails
    async fn discover_android_hsms(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        let mut hsms = Vec::new();

        // Check for StrongBox HSM (Android KeyMaster API)
        if self.check_strongbox {
            if let Some(strongbox) = self.detect_android_strongbox().await? {
                info!("✓ Found Android StrongBox HSM");
                hsms.push(strongbox);
            }
        }

        // Check for Samsung Knox
        if self.check_knox {
            if let Some(knox) = self.detect_samsung_knox().await? {
                info!("✓ Found Samsung Knox");
                hsms.push(knox);
            }
        }

        // Android Keystore is always available
        hsms.push(self.create_android_keystore_hsm());

        Ok(hsms)
    }

    /// Detect Android StrongBox
    ///
    /// # Errors
    /// Returns an error if detection fails
    #[cfg(target_os = "android")]
    async fn detect_android_strongbox(&self) -> Result<Option<DiscoveredHsm>, BearDogError> {
        // In a full implementation, this would check:
        // 1. PackageManager.FEATURE_STRONGBOX_KEYSTORE
        // 2. KeyInfo.isInsideSecureHardware() && KeyInfo.getSecurityLevel() == SECURITY_LEVEL_STRONGBOX
        // 3. Android API level >= 28
        
        // For now, check environment
        if env::var("ANDROID_STRONGBOX_AVAILABLE").is_ok() {
            Ok(Some(self.create_android_strongbox_hsm()))
        } else {
            Ok(None)
        }
    }

    /// Detect Android StrongBox (non-Android platforms)
    ///
    /// # Errors
    /// Returns an error if detection fails
    #[cfg(not(target_os = "android"))]
    async fn detect_android_strongbox(&self) -> Result<Option<DiscoveredHsm>, BearDogError> {
        if env::var("ANDROID_STRONGBOX_AVAILABLE").is_ok() {
            Ok(Some(self.create_android_strongbox_hsm()))
        } else {
            Ok(None)
        }
    }

    /// Detect Samsung Knox
    ///
    /// # Errors
    /// Returns an error if detection fails
    #[cfg(target_os = "android")]
    async fn detect_samsung_knox(&self) -> Result<Option<DiscoveredHsm>, BearDogError> {
        // In a full implementation, this would check:
        // 1. com.samsung.android.knox.container.KnoxContainerManager
        // 2. Build.MANUFACTURER == "samsung"
        // 3. Knox SDK availability
        
        if env::var("SAMSUNG_KNOX_AVAILABLE").is_ok() {
            Ok(Some(self.create_samsung_knox_hsm()))
        } else {
            Ok(None)
        }
    }

    /// Detect Samsung Knox (non-Android platforms)
    ///
    /// # Errors
    /// Returns an error if detection fails
    #[cfg(not(target_os = "android"))]
    async fn detect_samsung_knox(&self) -> Result<Option<DiscoveredHsm>, BearDogError> {
        if env::var("SAMSUNG_KNOX_AVAILABLE").is_ok() {
            Ok(Some(self.create_samsung_knox_hsm()))
        } else {
            Ok(None)
        }
    }

    /// Create Android StrongBox HSM
    fn create_android_strongbox_hsm(&self) -> DiscoveredHsm {
        let now = Utc::now();
        
        DiscoveredHsm {
            name: "android-strongbox".to_string(),
            hsm_type: HsmType::Hardware,
            endpoint: HsmEndpoint {
                host: "local".to_string(),
                port: None,
                protocol: "android-keystore".to_string(),
                secure: true,
            },
            capabilities: self.create_android_strongbox_capabilities(),
            assigned_tier: HsmTier::Tier1, // StrongBox is Tier 1
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: now,
            last_health_check: now,
            integration_status: IntegrationStatus::Discovered,
        }
    }

    /// Create Android Keystore HSM
    fn create_android_keystore_hsm(&self) -> DiscoveredHsm {
        let now = Utc::now();
        
        DiscoveredHsm {
            name: "android-keystore".to_string(),
            hsm_type: HsmType::Software, // May be hardware-backed, but we classify as software
            endpoint: HsmEndpoint {
                host: "local".to_string(),
                port: None,
                protocol: "android-keystore".to_string(),
                secure: true,
            },
            capabilities: self.create_android_keystore_capabilities(),
            assigned_tier: HsmTier::Tier2, // Tier 2 (may be hardware-backed)
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: now,
            last_health_check: now,
            integration_status: IntegrationStatus::Discovered,
        }
    }

    /// Create Samsung Knox HSM
    fn create_samsung_knox_hsm(&self) -> DiscoveredHsm {
        let now = Utc::now();
        
        DiscoveredHsm {
            name: "samsung-knox".to_string(),
            hsm_type: HsmType::Hardware,
            endpoint: HsmEndpoint {
                host: "local".to_string(),
                port: None,
                protocol: "samsung-knox".to_string(),
                secure: true,
            },
            capabilities: self.create_samsung_knox_capabilities(),
            assigned_tier: HsmTier::Tier1, // Knox is Tier 1
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: now,
            last_health_check: now,
            integration_status: IntegrationStatus::Discovered,
        }
    }

    /// Create iOS Secure Enclave capabilities
    fn create_ios_secure_enclave_capabilities(&self) -> UniversalHsmCapabilities {
        use crate::tunnel::hsm::types::capability::*;

        UniversalHsmCapabilities {
            crypto_operations: CryptoOperationCapabilities {
                symmetric_encryption: vec!["AES-256".to_string()],
                asymmetric_encryption: vec!["ECC-P256".to_string()],
                signing: vec!["ECDSA".to_string()],
                hashing: vec!["SHA-256".to_string()],
                key_agreement: vec!["ECDH".to_string()],
            },
            key_management: KeyManagementCapabilities {
                key_generation: true,
                key_storage: true,
                key_rotation: false,
                key_backup: false, // Keys cannot leave Secure Enclave
                key_recovery: false,
            },
            key_generation: KeyGenerationCapabilities {
                rsa: vec![],
                ecc: vec![256],
                aes: vec![256],
                supports_secure_random: true,
            },
            security: SecurityCapabilities {
                tamper_resistance: TamperResistance::Tier1,
                fips_140_2_level: Some(2), // Secure Enclave is FIPS 140-2 Level 2 (some Level 3)
                common_criteria_eal: None,
                secure_boot: true,
                attestation: true,
            },
            performance: PerformanceCapabilities {
                max_operations_per_second: 1000,
                typical_latency_ms: 10.0,
                supports_parallel_operations: false,
                hardware_acceleration: true,
            },
            compliance: ComplianceCapabilities {
                fips_140_2: true,
                common_criteria: false,
                pci_dss: false,
                hipaa: true,
                gdpr: true,
            },
            api_support: ApiSupportCapabilities {
                pkcs11: false,
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

    /// Create iOS Keychain capabilities
    fn create_ios_keychain_capabilities(&self) -> UniversalHsmCapabilities {
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
                key_backup: true, // iCloud Keychain backup
                key_recovery: true,
            },
            key_generation: KeyGenerationCapabilities {
                rsa: vec![2048],
                ecc: vec![256],
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
                pkcs11: false,
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

    /// Create Android StrongBox capabilities
    fn create_android_strongbox_capabilities(&self) -> UniversalHsmCapabilities {
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
                key_backup: false, // Keys cannot be exported
                key_recovery: false,
            },
            key_generation: KeyGenerationCapabilities {
                rsa: vec![2048, 4096],
                ecc: vec![256],
                aes: vec![128, 256],
                supports_secure_random: true,
            },
            security: SecurityCapabilities {
                tamper_resistance: TamperResistance::Tier1,
                fips_140_2_level: Some(1), // StrongBox certification varies
                common_criteria_eal: None,
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
                common_criteria: false,
                pci_dss: false,
                hipaa: false,
                gdpr: true,
            },
            api_support: ApiSupportCapabilities {
                pkcs11: false,
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

    /// Create Android Keystore capabilities
    fn create_android_keystore_capabilities(&self) -> UniversalHsmCapabilities {
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
                fips_140_2_level: None,
                common_criteria_eal: None,
                secure_boot: true,
                attestation: true,
            },
            performance: PerformanceCapabilities {
                max_operations_per_second: 2000,
                typical_latency_ms: 5.0,
                supports_parallel_operations: true,
                hardware_acceleration: true,
            },
            compliance: ComplianceCapabilities {
                fips_140_2: false,
                common_criteria: false,
                pci_dss: false,
                hipaa: false,
                gdpr: true,
            },
            api_support: ApiSupportCapabilities {
                pkcs11: false,
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

    /// Create Samsung Knox capabilities
    fn create_samsung_knox_capabilities(&self) -> UniversalHsmCapabilities {
        use crate::tunnel::hsm::types::capability::*;

        UniversalHsmCapabilities {
            crypto_operations: CryptoOperationCapabilities {
                symmetric_encryption: vec!["AES-256".to_string()],
                asymmetric_encryption: vec!["RSA-2048".to_string(), "ECC-P256".to_string()],
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
                rsa: vec![2048],
                ecc: vec![256],
                aes: vec![256],
                supports_secure_random: true,
            },
            security: SecurityCapabilities {
                tamper_resistance: TamperResistance::Tier1,
                fips_140_2_level: Some(1),
                common_criteria_eal: Some(5), // Knox has CC EAL5+
                secure_boot: true,
                attestation: true,
            },
            performance: PerformanceCapabilities {
                max_operations_per_second: 1000,
                typical_latency_ms: 15.0,
                supports_parallel_operations: false,
                hardware_acceleration: true,
            },
            compliance: ComplianceCapabilities {
                fips_140_2: true,
                common_criteria: true,
                pci_dss: false,
                hipaa: true,
                gdpr: true,
            },
            api_support: ApiSupportCapabilities {
                pkcs11: false,
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

impl Default for MobileDiscoverer {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            let platform = Self::detect_platform();
            Self {
                platform,
                check_strongbox: true,
                check_knox: true,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discoverer_creation() {
        let discoverer = MobileDiscoverer::new();
        assert!(discoverer.is_ok());
    }

    #[tokio::test]
    async fn test_mobile_discovery() {
        let discoverer = MobileDiscoverer::new().unwrap();
        let result = discoverer.discover().await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_platform_detection() {
        let platform = MobileDiscoverer::detect_platform();
        // Platform detection should not fail
        assert!(matches!(platform, MobilePlatform::Ios | MobilePlatform::Android | MobilePlatform::Unknown));
    }

    #[test]
    fn test_ios_secure_enclave_capabilities() {
        let discoverer = MobileDiscoverer::new().unwrap();
        let caps = discoverer.create_ios_secure_enclave_capabilities();
        
        assert_eq!(caps.security.fips_140_2_level, Some(2));
        assert!(caps.security.tamper_resistance == TamperResistance::Tier1);
        assert!(caps.security.attestation);
        assert!(!caps.key_management.key_backup); // Keys cannot leave enclave
    }

    #[test]
    fn test_android_strongbox_capabilities() {
        let discoverer = MobileDiscoverer::new().unwrap();
        let caps = discoverer.create_android_strongbox_capabilities();
        
        assert!(caps.security.tamper_resistance == TamperResistance::Tier1);
        assert!(caps.security.attestation);
        assert!(!caps.key_management.key_backup);
    }

    #[test]
    fn test_samsung_knox_capabilities() {
        let discoverer = MobileDiscoverer::new().unwrap();
        let caps = discoverer.create_samsung_knox_capabilities();
        
        assert_eq!(caps.security.common_criteria_eal, Some(5));
        assert!(caps.compliance.common_criteria);
    }

    #[test]
    fn test_mobile_hsm_types() {
        assert_eq!(MobileHsmType::IosSecureEnclave, MobileHsmType::IosSecureEnclave);
        assert_ne!(MobileHsmType::IosSecureEnclave, MobileHsmType::AndroidStrongBox);
    }
}

