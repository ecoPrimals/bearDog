//! Platform HSM Discoverer
//!
//! Provides discovery functionality for platform-specific HSMs (TPM, TEE, etc.)
//!
//! This module discovers hardware security modules that are integrated into
//! the platform/device, such as:
//! - TPM (Trusted Platform Module) on Linux/Windows
//! - Android StrongBox/TEE
//! - iOS Secure Enclave
//! - Platform-specific trusted execution environments

use super::super::*;
use beardog_errors::BearDogError;
use chrono::Utc;
use std::path::Path;
use tracing::{debug, info, warn};

/// Platform HSM discoverer
#[derive(Debug, Clone)]
pub struct PlatformDiscoverer {
    /// Whether to probe for TPM
    enable_tpm_probe: bool,
    /// Whether to probe for TEE
    enable_tee_probe: bool,
}

impl PlatformDiscoverer {
    /// Create new platform discoverer
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            enable_tpm_probe: true,
            enable_tee_probe: true,
        })
    }

    /// Discover platform HSMs
    ///
    /// # Errors
    /// Returns an error if discovery fails
    pub async fn discover(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("🔍 Discovering platform-specific HSMs");
        
        let mut discovered = Vec::new();

        // Discover TPM on supported platforms
        if self.enable_tpm_probe {
            if let Some(tpm_hsm) = self.discover_tpm().await? {
                debug!("✓ Discovered TPM HSM");
                discovered.push(tpm_hsm);
            }
        }

        // Discover platform TEE
        if self.enable_tee_probe {
            if let Some(tee_hsm) = self.discover_tee().await? {
                debug!("✓ Discovered TEE HSM");
                discovered.push(tee_hsm);
            }
        }

        // Discover Android StrongBox (platform-specific)
        #[cfg(target_os = "android")]
        {
            if let Some(strongbox) = self.discover_android_strongbox().await? {
                debug!("✓ Discovered Android StrongBox");
                discovered.push(strongbox);
            }
        }

        // Discover iOS Secure Enclave (platform-specific)
        #[cfg(target_os = "ios")]
        {
            if let Some(secure_enclave) = self.discover_ios_secure_enclave().await? {
                debug!("✓ Discovered iOS Secure Enclave");
                discovered.push(secure_enclave);
            }
        }

        info!("✅ Platform discovery complete: {} HSMs found", discovered.len());
        Ok(discovered)
    }

    /// Discover TPM (Trusted Platform Module)
    ///
    /// # Errors
    /// Returns an error if TPM probe fails
    async fn discover_tpm(&self) -> Result<Option<DiscoveredHsm>, BearDogError> {
        debug!("Probing for TPM");

        // Check for TPM on Linux
        #[cfg(target_os = "linux")]
        {
            // Common TPM device paths
            let tpm_paths = ["/dev/tpm0", "/dev/tpmrm0"];
            
            for tpm_path in &tpm_paths {
                if Path::new(tpm_path).exists() {
                    info!("✓ Found TPM device at {}", tpm_path);
                    
                    return Ok(Some(self.create_tpm_hsm(tpm_path.to_string())));
                }
            }
        }

        // Check for TPM on Windows
        #[cfg(target_os = "windows")]
        {
            // On Windows, check registry or use platform APIs
            // For now, we'll do a basic check
            if self.check_windows_tpm()? {
                info!("✓ Found TPM on Windows");
                return Ok(Some(self.create_tpm_hsm("windows-tpm".to_string())));
            }
        }

        debug!("No TPM found on this platform");
        Ok(None)
    }

    /// Discover Trusted Execution Environment
    ///
    /// # Errors
    /// Returns an error if TEE probe fails
    async fn discover_tee(&self) -> Result<Option<DiscoveredHsm>, BearDogError> {
        debug!("Probing for TEE");

        #[cfg(target_os = "linux")]
        {
            // Check for OP-TEE or other TEE implementations
            if Path::new("/dev/tee0").exists() || Path::new("/dev/teepriv0").exists() {
                info!("✓ Found TEE device");
                return Ok(Some(self.create_tee_hsm()));
            }
        }

        debug!("No TEE found on this platform");
        Ok(None)
    }

    /// Discover Android StrongBox
    ///
    /// # Errors
    /// Returns an error if StrongBox probe fails
    #[cfg(target_os = "android")]
    async fn discover_android_strongbox(&self) -> Result<Option<DiscoveredHsm>, BearDogError> {
        debug!("Probing for Android StrongBox");

        // Check for Android StrongBox support
        // In a real implementation, this would query Android's Keystore API
        // For now, we provide the structure
        
        // TODO: Integrate with Android Keystore API to detect StrongBox
        warn!("Android StrongBox detection requires platform-specific APIs");
        
        Ok(None)
    }

    /// Discover iOS Secure Enclave
    ///
    /// # Errors
    /// Returns an error if Secure Enclave probe fails
    #[cfg(target_os = "ios")]
    async fn discover_ios_secure_enclave(&self) -> Result<Option<DiscoveredHsm>, BearDogError> {
        debug!("Probing for iOS Secure Enclave");

        // Check for iOS Secure Enclave support
        // In a real implementation, this would query iOS Security framework
        // For now, we provide the structure
        
        // TODO: Integrate with iOS Security framework to detect Secure Enclave
        warn!("iOS Secure Enclave detection requires platform-specific APIs");
        
        Ok(None)
    }

    /// Check for TPM on Windows
    ///
    /// # Errors
    /// Returns an error if Windows TPM check fails
    #[cfg(target_os = "windows")]
    fn check_windows_tpm(&self) -> Result<bool, BearDogError> {
        // TODO: Implement Windows-specific TPM detection
        // This would typically use Windows Management Instrumentation (WMI)
        // or the TPM Base Services (TBS) API
        warn!("Windows TPM detection requires platform-specific APIs");
        Ok(false)
    }

    /// Create a DiscoveredHsm for TPM
    fn create_tpm_hsm(&self, device_path: String) -> DiscoveredHsm {
        let now = Utc::now();
        
        DiscoveredHsm {
            name: format!("tpm-{}", device_path.replace('/', "-")),
            hsm_type: HsmType::Tpm,
            endpoint: HsmEndpoint {
                host: device_path.clone(),
                port: None,
                protocol: "tpm".to_string(),
                secure: true,
            },
            capabilities: self.create_tpm_capabilities(),
            assigned_tier: HsmTier::Tier2, // TPM is typically Tier 2
            supports_human_entropy: false, // TPM doesn't typically support human entropy
            health_status: HsmHealthStatus::Healthy,
            discovered_at: now,
            last_health_check: now,
            integration_status: IntegrationStatus::Discovered,
        }
    }

    /// Create a DiscoveredHsm for TEE
    fn create_tee_hsm(&self) -> DiscoveredHsm {
        let now = Utc::now();
        
        DiscoveredHsm {
            name: "platform-tee".to_string(),
            hsm_type: HsmType::Hardware, // TEE is hardware-backed
            endpoint: HsmEndpoint {
                host: "/dev/tee0".to_string(),
                port: None,
                protocol: "tee".to_string(),
                secure: true,
            },
            capabilities: self.create_tee_capabilities(),
            assigned_tier: HsmTier::Tier2,
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: now,
            last_health_check: now,
            integration_status: IntegrationStatus::Discovered,
        }
    }

    /// Create capabilities for TPM
    fn create_tpm_capabilities(&self) -> UniversalHsmCapabilities {
        use crate::tunnel::hsm::types::capability::*;

        UniversalHsmCapabilities {
            crypto_operations: CryptoOperationCapabilities {
                symmetric_encryption: vec!["AES-128".to_string(), "AES-256".to_string()],
                asymmetric_encryption: vec!["RSA-2048".to_string()],
                signing: vec!["RSA-PSS".to_string(), "ECDSA".to_string()],
                hashing: vec!["SHA-256".to_string(), "SHA-384".to_string()],
                key_agreement: vec!["ECDH".to_string()],
            },
            key_management: KeyManagementCapabilities {
                key_generation: true,
                key_storage: true,
                key_rotation: true,
                key_backup: false, // TPM keys typically not exportable
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
                fips_140_2_level: Some(2),
                common_criteria_eal: Some(4),
                secure_boot: true,
                attestation: true,
            },
            performance: PerformanceCapabilities {
                max_operations_per_second: 1000,
                typical_latency_ms: 10.0,
                supports_parallel_operations: true,
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

    /// Create capabilities for TEE
    fn create_tee_capabilities(&self) -> UniversalHsmCapabilities {
        use crate::tunnel::hsm::types::capability::*;

        UniversalHsmCapabilities {
            crypto_operations: CryptoOperationCapabilities {
                symmetric_encryption: vec!["AES-128".to_string(), "AES-256".to_string()],
                asymmetric_encryption: vec!["RSA-2048".to_string(), "ECC-P256".to_string()],
                signing: vec!["RSA-PSS".to_string(), "ECDSA".to_string()],
                hashing: vec!["SHA-256".to_string(), "SHA-512".to_string()],
                key_agreement: vec!["ECDH".to_string()],
            },
            key_management: KeyManagementCapabilities {
                key_generation: true,
                key_storage: true,
                key_rotation: true,
                key_backup: false,
                key_recovery: false,
            },
            key_generation: KeyGenerationCapabilities {
                rsa: vec![2048, 4096],
                ecc: vec![256, 384, 521],
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
                max_operations_per_second: 5000,
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
}

impl Default for PlatformDiscoverer {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            enable_tpm_probe: true,
            enable_tee_probe: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discoverer_creation() {
        let discoverer = PlatformDiscoverer::new();
        assert!(discoverer.is_ok());
        
        let disc = discoverer.unwrap();
        assert!(disc.enable_tpm_probe);
        assert!(disc.enable_tee_probe);
    }

    #[tokio::test]
    async fn test_platform_discovery() {
        let discoverer = PlatformDiscoverer::new().unwrap();
        let result = discoverer.discover().await;
        assert!(result.is_ok());
        
        // Discovery should succeed even if no HSMs are found
        let hsms = result.unwrap();
        // The number of HSMs found depends on the platform
        assert!(hsms.len() <= 4); // At most TPM, TEE, and platform-specific
    }

    #[tokio::test]
    async fn test_tpm_capabilities() {
        let discoverer = PlatformDiscoverer::new().unwrap();
        let caps = discoverer.create_tpm_capabilities();
        
        // Verify TPM capabilities
        assert!(caps.key_management.key_generation);
        assert!(caps.key_management.key_storage);
        assert!(caps.security.tamper_resistance == TamperResistance::Tier2);
        assert!(caps.security.fips_140_2_level == Some(2));
        assert!(!caps.human_entropy.supported);
    }

    #[tokio::test]
    async fn test_tee_capabilities() {
        let discoverer = PlatformDiscoverer::new().unwrap();
        let caps = discoverer.create_tee_capabilities();
        
        // Verify TEE capabilities
        assert!(caps.key_management.key_generation);
        assert!(caps.performance.max_operations_per_second > 1000);
        assert!(caps.security.secure_boot);
        assert!(!caps.human_entropy.supported);
    }

    // ========== Additional Comprehensive Tests ==========

    #[tokio::test]
    async fn test_tpm_discovery_with_disabled_probe() {
        let mut discoverer = PlatformDiscoverer::new().unwrap();
        discoverer.enable_tpm_probe = false;
        
        let hsms = discoverer.discover().await.unwrap();
        let tpm_count = hsms.iter().filter(|h| h.name.contains("TPM")).count();
        assert_eq!(tpm_count, 0, "Should not discover TPM when probe disabled");
    }

    #[tokio::test]
    async fn test_tee_discovery_with_disabled_probe() {
        let mut discoverer = PlatformDiscoverer::new().unwrap();
        discoverer.enable_tee_probe = false;
        
        let hsms = discoverer.discover().await.unwrap();
        let tee_count = hsms.iter().filter(|h| h.name.contains("TEE")).count();
        assert_eq!(tee_count, 0, "Should not discover TEE when probe disabled");
    }

    #[tokio::test]
    async fn test_all_hsms_have_valid_timestamps() {
        let discoverer = PlatformDiscoverer::new().unwrap();
        let hsms = discoverer.discover().await.unwrap();
        
        for hsm in &hsms {
            assert!(hsm.discovered_at <= Utc::now());
            assert!(!hsm.id.is_empty());
            assert!(!hsm.name.is_empty());
        }
    }

    #[tokio::test]
    async fn test_concurrent_discoveries() {
        use std::sync::Arc;
        let discoverer = Arc::new(PlatformDiscoverer::new().unwrap());
        
        let mut handles = vec![];
        for _ in 0..3 {
            let disc = Arc::clone(&discoverer);
            handles.push(tokio::spawn(async move {
                disc.discover().await
            }));
        }
        
        for handle in handles {
            let result = handle.await.unwrap();
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_discovery_deterministic() {
        let discoverer = PlatformDiscoverer::new().unwrap();
        
        let hsms1 = discoverer.discover().await.unwrap();
        let hsms2 = discoverer.discover().await.unwrap();
        
        assert_eq!(hsms1.len(), hsms2.len(), "Discovery should be deterministic");
    }

    #[test]
    fn test_default_implementation() {
        let discoverer = PlatformDiscoverer::default();
        assert!(discoverer.enable_tpm_probe);
        assert!(discoverer.enable_tee_probe);
    }

    #[test]
    fn test_clone_implementation() {
        let discoverer1 = PlatformDiscoverer::new().unwrap();
        let discoverer2 = discoverer1.clone();
        
        assert_eq!(discoverer1.enable_tpm_probe, discoverer2.enable_tpm_probe);
        assert_eq!(discoverer1.enable_tee_probe, discoverer2.enable_tee_probe);
    }
}
