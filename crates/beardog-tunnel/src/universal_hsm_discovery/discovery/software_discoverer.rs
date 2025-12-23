//! Software HSM Discoverer
//!
//! Provides discovery functionality for software-based HSMs
//!
//! This module discovers software implementations of HSM functionality,
//! including:
//! - Local software HSMs (like SoftHSM)
//! - Cryptographic libraries with HSM-like APIs
//! - Built-in BearDog software HSM
//! - System-provided software keystores

use super::super::*;
use beardog_errors::BearDogError;
use chrono::Utc;
use std::path::Path;
use tracing::{debug, info};

/// Software HSM discoverer
#[derive(Debug, Clone)]
pub struct SoftwareDiscoverer {
    /// Whether to discover BearDog software HSM
    enable_beardog_hsm: bool,
    /// Whether to discover SoftHSM
    enable_softhsm: bool,
    /// Whether to discover system keystores
    enable_system_keystores: bool,
}

impl SoftwareDiscoverer {
    /// Create new software discoverer
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            enable_beardog_hsm: true,
            enable_softhsm: true,
            enable_system_keystores: true,
        })
    }

    /// Discover software HSMs
    ///
    /// # Errors
    /// Returns an error if discovery fails
    pub async fn discover(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        info!("🔍 Discovering software-based HSMs");
        
        let mut discovered = Vec::new();

        // Always discover BearDog's built-in software HSM
        if self.enable_beardog_hsm {
            if let Some(beardog_hsm) = self.discover_beardog_software_hsm().await? {
                debug!("✓ Discovered BearDog Software HSM");
                discovered.push(beardog_hsm);
            }
        }

        // Discover SoftHSM if available
        if self.enable_softhsm {
            if let Some(softhsm) = self.discover_softhsm().await? {
                debug!("✓ Discovered SoftHSM");
                discovered.push(softhsm);
            }
        }

        // Discover system keystores
        if self.enable_system_keystores {
            discovered.extend(self.discover_system_keystores().await?);
        }

        info!("✅ Software HSM discovery complete: {} HSMs found", discovered.len());
        Ok(discovered)
    }

    /// Discover BearDog's built-in software HSM
    ///
    /// # Errors
    /// Returns an error if discovery fails
    async fn discover_beardog_software_hsm(&self) -> Result<Option<DiscoveredHsm>, BearDogError> {
        debug!("Discovering BearDog Software HSM");

        // BearDog software HSM is always available
        info!("✓ BearDog Software HSM available");
        Ok(Some(self.create_beardog_software_hsm()))
    }

    /// Discover SoftHSM installation
    ///
    /// # Errors
    /// Returns an error if SoftHSM probe fails
    async fn discover_softhsm(&self) -> Result<Option<DiscoveredHsm>, BearDogError> {
        debug!("Probing for SoftHSM");

        // Common SoftHSM library paths
        let softhsm_paths = [
            "/usr/lib/softhsm/libsofthsm2.so",
            "/usr/local/lib/softhsm/libsofthsm2.so",
            "/usr/lib/x86_64-linux-gnu/softhsm/libsofthsm2.so",
        ];

        for path in &softhsm_paths {
            if Path::new(path).exists() {
                info!("✓ Found SoftHSM at {}", path);
                return Ok(Some(self.create_softhsm_hsm(path.to_string())));
            }
        }

        // Check macOS path
        #[cfg(target_os = "macos")]
        {
            let macos_path = "/usr/local/lib/libsofthsm2.dylib";
            if Path::new(macos_path).exists() {
                info!("✓ Found SoftHSM at {}", macos_path);
                return Ok(Some(self.create_softhsm_hsm(macos_path.to_string())));
            }
        }

        debug!("SoftHSM not found");
        Ok(None)
    }

    /// Discover system-provided keystores
    ///
    /// # Errors
    /// Returns an error if keystore discovery fails
    async fn discover_system_keystores(&self) -> Result<Vec<DiscoveredHsm>, BearDogError> {
        debug!("Discovering system keystores");

        let mut keystores = Vec::new();

        // Linux: Check for GNOME Keyring
        #[cfg(target_os = "linux")]
        {
            if self.check_gnome_keyring()? {
                debug!("✓ Found GNOME Keyring");
                keystores.push(self.create_gnome_keyring_hsm());
            }
        }

        // macOS: Keychain is always available
        #[cfg(target_os = "macos")]
        {
            debug!("✓ macOS Keychain available");
            keystores.push(self.create_macos_keychain_hsm());
        }

        // Windows: Credential Manager / Data Protection API
        #[cfg(target_os = "windows")]
        {
            debug!("✓ Windows Credential Manager available");
            keystores.push(self.create_windows_credential_manager_hsm());
        }

        Ok(keystores)
    }

    /// Check for GNOME Keyring
    ///
    /// # Errors
    /// Returns an error if check fails
    #[cfg(target_os = "linux")]
    fn check_gnome_keyring(&self) -> Result<bool, BearDogError> {
        // Check if D-Bus service is available
        // This is a simplified check - real implementation would query D-Bus
        Ok(std::env::var("DBUS_SESSION_BUS_ADDRESS").is_ok())
    }

    /// Create BearDog Software HSM
    fn create_beardog_software_hsm(&self) -> DiscoveredHsm {
        use beardog_types::constants::domains::network::config;
        let now = Utc::now();
        
        DiscoveredHsm {
            name: "beardog-software-hsm".to_string(),
            hsm_type: HsmType::Software,
            endpoint: HsmEndpoint {
                host: config::default_service_host(),
                port: None,
                protocol: "native".to_string(),
                secure: true,
            },
            capabilities: self.create_software_hsm_capabilities(),
            assigned_tier: HsmTier::Tier3, // Software HSM is typically Tier 3
            supports_human_entropy: true, // BearDog HSM supports human entropy
            health_status: HsmHealthStatus::Healthy,
            discovered_at: now,
            last_health_check: now,
            integration_status: IntegrationStatus::Ready, // Built-in HSM is ready
        }
    }

    /// Create SoftHSM
    fn create_softhsm_hsm(&self, library_path: String) -> DiscoveredHsm {
        let now = Utc::now();
        
        DiscoveredHsm {
            name: "softhsm2".to_string(),
            hsm_type: HsmType::Software,
            endpoint: HsmEndpoint {
                host: library_path.clone(),
                port: None,
                protocol: "pkcs11".to_string(),
                secure: true,
            },
            capabilities: self.create_softhsm_capabilities(),
            assigned_tier: HsmTier::Tier3,
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: now,
            last_health_check: now,
            integration_status: IntegrationStatus::Discovered,
        }
    }

    /// Create GNOME Keyring HSM
    #[cfg(target_os = "linux")]
    fn create_gnome_keyring_hsm(&self) -> DiscoveredHsm {
        let now = Utc::now();
        
        DiscoveredHsm {
            name: "gnome-keyring".to_string(),
            hsm_type: HsmType::Software,
            endpoint: HsmEndpoint {
                host: "dbus://org.gnome.keyring".to_string(),
                port: None,
                protocol: "dbus".to_string(),
                secure: true,
            },
            capabilities: self.create_system_keystore_capabilities(),
            assigned_tier: HsmTier::Tier4, // System keystores are Tier 4
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: now,
            last_health_check: now,
            integration_status: IntegrationStatus::Discovered,
        }
    }

    /// Create macOS Keychain HSM
    #[cfg(target_os = "macos")]
    fn create_macos_keychain_hsm(&self) -> DiscoveredHsm {
        let now = Utc::now();
        
        DiscoveredHsm {
            name: "macos-keychain".to_string(),
            hsm_type: HsmType::Software,
            endpoint: HsmEndpoint {
                host: "system://keychain".to_string(),
                port: None,
                protocol: "keychain".to_string(),
                secure: true,
            },
            capabilities: self.create_system_keystore_capabilities(),
            assigned_tier: HsmTier::Tier4,
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: now,
            last_health_check: now,
            integration_status: IntegrationStatus::Discovered,
        }
    }

    /// Create Windows Credential Manager HSM
    #[cfg(target_os = "windows")]
    fn create_windows_credential_manager_hsm(&self) -> DiscoveredHsm {
        let now = Utc::now();
        
        DiscoveredHsm {
            name: "windows-credential-manager".to_string(),
            hsm_type: HsmType::Software,
            endpoint: HsmEndpoint {
                host: "system://credman".to_string(),
                port: None,
                protocol: "dpapi".to_string(),
                secure: true,
            },
            capabilities: self.create_system_keystore_capabilities(),
            assigned_tier: HsmTier::Tier4,
            supports_human_entropy: false,
            health_status: HsmHealthStatus::Healthy,
            discovered_at: now,
            last_health_check: now,
            integration_status: IntegrationStatus::Discovered,
        }
    }

    /// Create capabilities for BearDog software HSM
    fn create_software_hsm_capabilities(&self) -> UniversalHsmCapabilities {
        use crate::tunnel::hsm::types::capability::*;

        UniversalHsmCapabilities {
            crypto_operations: CryptoOperationCapabilities {
                symmetric_encryption: vec![
                    "AES-128".to_string(),
                    "AES-256".to_string(),
                    "ChaCha20-Poly1305".to_string(),
                ],
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
                hashing: vec![
                    "SHA-256".to_string(),
                    "SHA-384".to_string(),
                    "SHA-512".to_string(),
                    "BLAKE3".to_string(),
                ],
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
                tamper_resistance: TamperResistance::Tier3,
                fips_140_2_level: None,
                common_criteria_eal: None,
                secure_boot: false,
                attestation: false,
            },
            performance: PerformanceCapabilities {
                max_operations_per_second: 10000,
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
                pkcs7: true,
            },
            advanced_features: AdvancedFeatureCapabilities {
                quantum_resistant: false,
                multi_party_computation: false,
                threshold_cryptography: false,
                homomorphic_encryption: false,
            },
            human_entropy: HumanEntropyCapabilities {
                supported: true,
                methods: vec![],
                quality_score: 0.8,
            },
        }
    }

    /// Create capabilities for SoftHSM
    fn create_softhsm_capabilities(&self) -> UniversalHsmCapabilities {
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
                key_rotation: true,
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
                tamper_resistance: TamperResistance::Tier3,
                fips_140_2_level: None,
                common_criteria_eal: None,
                secure_boot: false,
                attestation: false,
            },
            performance: PerformanceCapabilities {
                max_operations_per_second: 5000,
                typical_latency_ms: 2.0,
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

    /// Create capabilities for system keystores
    fn create_system_keystore_capabilities(&self) -> UniversalHsmCapabilities {
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
                tamper_resistance: TamperResistance::Tier4,
                fips_140_2_level: None,
                common_criteria_eal: None,
                secure_boot: false,
                attestation: false,
            },
            performance: PerformanceCapabilities {
                max_operations_per_second: 1000,
                typical_latency_ms: 5.0,
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

impl Default for SoftwareDiscoverer {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            enable_beardog_hsm: true,
            enable_softhsm: true,
            enable_system_keystores: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discoverer_creation() {
        let discoverer = SoftwareDiscoverer::new();
        assert!(discoverer.is_ok());
        
        let disc = discoverer?;
        assert!(disc.enable_beardog_hsm);
        assert!(disc.enable_softhsm);
        assert!(disc.enable_system_keystores);
    }

    #[tokio::test]
    async fn test_software_discovery() {
        let discoverer = SoftwareDiscoverer::new()?;
        let result = discoverer.discover().await;
        assert!(result.is_ok());
        
        let hsms = result?;
        // Should at least find BearDog software HSM
        assert!(hsms.len() >= 1);
        
        // Verify BearDog HSM is present
        let beardog_hsm = hsms.iter().find(|h| h.name == "beardog-software-hsm");
        assert!(beardog_hsm.is_some());
    }

    #[tokio::test]
    async fn test_beardog_hsm_discovery() {
        let discoverer = SoftwareDiscoverer::new()?;
        let beardog_hsm = discoverer.discover_beardog_software_hsm().await?;
        
        assert!(beardog_hsm.is_some());
        let hsm = beardog_hsm?;
        assert_eq!(hsm.name, "beardog-software-hsm");
        assert_eq!(hsm.hsm_type, HsmType::Software);
        assert!(hsm.supports_human_entropy);
        assert_eq!(hsm.integration_status, IntegrationStatus::Ready);
    }

    #[tokio::test]
    async fn test_beardog_hsm_capabilities() {
        let discoverer = SoftwareDiscoverer::new()?;
        let caps = discoverer.create_software_hsm_capabilities();
        
        // Verify BearDog HSM capabilities
        assert!(caps.key_management.key_generation);
        assert!(caps.key_management.key_backup);
        assert!(caps.human_entropy.supported);
        assert!(caps.human_entropy.quality_score > 0.0);
        assert!(caps.performance.max_operations_per_second >= 10000);
    }

    #[tokio::test]
    async fn test_softhsm_capabilities() {
        let discoverer = SoftwareDiscoverer::new()?;
        let caps = discoverer.create_softhsm_capabilities();
        
        // Verify SoftHSM capabilities
        assert!(caps.api_support.pkcs11);
        assert!(caps.key_management.key_generation);
        assert!(!caps.human_entropy.supported);
    }

    // ========== Additional Comprehensive Tests ==========

    #[tokio::test]
    async fn test_beardog_hsm_always_discovered() {
        let discoverer = SoftwareDiscoverer::new()?;
        let hsms = discoverer.discover().await?;
        
        // BearDog HSM should always be available
        let beardog_hsms: Vec<_> = hsms.iter()
            .filter(|h| h.name.contains("BearDog"))
            .collect();
        
        assert!(!beardog_hsms.is_empty(), "BearDog HSM should always be discovered");
    }

    #[tokio::test]
    async fn test_beardog_hsm_disabled_probe() {
        let mut discoverer = SoftwareDiscoverer::new()?;
        discoverer.enable_beardog_hsm = false;
        
        let hsms = discoverer.discover().await?;
        let beardog_count = hsms.iter().filter(|h| h.name.contains("BearDog")).count();
        
        assert_eq!(beardog_count, 0, "Should not discover BearDog when disabled");
    }

    #[tokio::test]
    async fn test_softhsm_disabled_probe() {
        let mut discoverer = SoftwareDiscoverer::new()?;
        discoverer.enable_softhsm = false;
        
        let hsms = discoverer.discover().await?;
        let softhsm_count = hsms.iter().filter(|h| h.name.contains("SoftHSM")).count();
        
        assert_eq!(softhsm_count, 0, "Should not discover SoftHSM when disabled");
    }

    #[tokio::test]
    async fn test_system_keystores_disabled_probe() {
        let mut discoverer = SoftwareDiscoverer::new()?;
        discoverer.enable_system_keystores = false;
        
        let hsms = discoverer.discover().await?;
        let keystore_count = hsms.iter()
            .filter(|h| h.name.contains("Keyring") || h.name.contains("Keychain") || h.name.contains("Credential Manager"))
            .count();
        
        assert_eq!(keystore_count, 0, "Should not discover keystores when disabled");
    }

    #[tokio::test]
    async fn test_all_probes_disabled() {
        let mut discoverer = SoftwareDiscoverer::new()?;
        discoverer.enable_beardog_hsm = false;
        discoverer.enable_softhsm = false;
        discoverer.enable_system_keystores = false;
        
        let hsms = discoverer.discover().await?;
        assert!(hsms.is_empty(), "Should discover nothing when all probes disabled");
    }

    #[tokio::test]
    async fn test_discovered_hsms_have_valid_fields() {
        let discoverer = SoftwareDiscoverer::new()?;
        let hsms = discoverer.discover().await?;
        
        for hsm in &hsms {
            assert!(!hsm.id.is_empty(), "ID should not be empty");
            assert!(!hsm.name.is_empty(), "Name should not be empty");
            assert!(!hsm.vendor.is_empty(), "Vendor should not be empty");
            assert!(hsm.discovered_at <= Utc::now());
        }
    }

    #[tokio::test]
    async fn test_software_hsm_types() {
        let discoverer = SoftwareDiscoverer::new()?;
        let hsms = discoverer.discover().await?;
        
        for hsm in &hsms {
            // All discovered HSMs should be Software type
            assert_eq!(hsm.hsm_type, HsmType::SoftwareHsm, 
                "Software discoverer should only return software HSMs");
        }
    }

    #[tokio::test]
    async fn test_concurrent_software_discovery() {
        use std::sync::Arc;
        let discoverer = Arc::new(SoftwareDiscoverer::new()?);
        
        let mut handles = vec![];
        for _ in 0..5 {
            let disc = Arc::clone(&discoverer);
            handles.push(tokio::spawn(async move {
                disc.discover().await
            }));
        }
        
        for handle in handles {
            let result = handle.await?;
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_discovery_is_deterministic() {
        let discoverer = SoftwareDiscoverer::new()?;
        
        let hsms1 = discoverer.discover().await?;
        let hsms2 = discoverer.discover().await?;
        
        assert_eq!(hsms1.len(), hsms2.len(), "Discovery should return same count");
    }

    #[tokio::test]
    async fn test_beardog_hsm_human_entropy_support() {
        let discoverer = SoftwareDiscoverer::new()?;
        let caps = discoverer.create_software_hsm_capabilities();
        
        assert!(caps.human_entropy.supported, "BearDog HSM should support human entropy");
        assert!(caps.human_entropy.quality_score > 0.0, "Should have quality score");
    }

    #[tokio::test]
    async fn test_system_keystore_capabilities() {
        let discoverer = SoftwareDiscoverer::new()?;
        let caps = discoverer.create_system_keystore_capabilities();
        
        // System keystores should have basic capabilities
        assert!(caps.key_management.key_storage);
        assert!(caps.key_management.key_generation);
    }

    #[test]
    fn test_default_implementation() {
        let discoverer = SoftwareDiscoverer::default();
        assert!(discoverer.enable_beardog_hsm);
        assert!(discoverer.enable_softhsm);
        assert!(discoverer.enable_system_keystores);
    }

    #[test]
    fn test_clone_implementation() {
        let discoverer1 = SoftwareDiscoverer::new()?;
        let discoverer2 = discoverer1.clone();
        
        assert_eq!(discoverer1.enable_beardog_hsm, discoverer2.enable_beardog_hsm);
        assert_eq!(discoverer1.enable_softhsm, discoverer2.enable_softhsm);
    }

    #[tokio::test]
    async fn test_hsm_unique_ids() {
        let discoverer = SoftwareDiscoverer::new()?;
        let hsms = discoverer.discover().await?;
        
        let mut ids = std::collections::HashSet::new();
        for hsm in &hsms {
            assert!(ids.insert(&hsm.id), "Each HSM should have unique ID");
        }
    }

    #[tokio::test]
    async fn test_performance_metrics() {
        use std::time::Instant;
        
        let discoverer = SoftwareDiscoverer::new()?;
        let start = Instant::now();
        
        let _hsms = discoverer.discover().await?;
        
        let duration = start.elapsed();
        assert!(duration.as_secs() < 3, "Software discovery should be fast: {:?}", duration);
    }
}
