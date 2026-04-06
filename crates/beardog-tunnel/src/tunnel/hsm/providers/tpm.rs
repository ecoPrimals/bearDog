// SPDX-License-Identifier: AGPL-3.0-or-later

//! TPM (Trusted Platform Module) Provider
//!
//! Universal provider implementation for TPM 2.0 hardware security modules.

use beardog_errors::BearDogError;
use std::collections::HashMap;

/// TPM Universal HSM Provider
#[expect(
    dead_code,
    reason = "pub struct for TPM provider surface; fields used as impl lands"
)]
pub struct TpmUniversalProvider {
    /// HSM capabilities
    capabilities: Option<TpmCapabilities>,
    /// TPM version
    tpm_version: TpmVersion,
    /// Provider metadata
    metadata: HashMap<String, String>,
}

/// TPM version enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TpmVersion {
    /// TPM 1.2 (legacy)
    V1_2,
    /// TPM 2.0
    V2_0,
}

/// TPM-specific HSM capabilities
#[derive(Debug, Clone)]
pub struct TpmCapabilities {
    /// Manufacturer
    pub manufacturer: String,
    /// Vendor string
    pub vendor_string: String,
    /// Firmware version
    pub firmware_version: String,
    /// PCR banks available
    pub pcr_banks: Vec<String>,
}

impl TpmUniversalProvider {
    /// Create a new TPM HSM provider
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub async fn new() -> Result<Self, BearDogError> {
        #[cfg(not(feature = "tpm-provider"))]
        {
            Err(BearDogError::requires_capability(
                "tpm-provider",
                "TPM HSM provider requires the tpm-provider Cargo feature and platform TPM integration",
            ))
        }
        #[cfg(feature = "tpm-provider")]
        {
            let provider = Self {
                capabilities: None,
                tpm_version: TpmVersion::V2_0,
                metadata: HashMap::with_capacity(16),
            };
            // Phase 2: full TPM 2.0 crypto requires tss2 / platform integration (pure Rust tss-esapi).
            Ok(provider)
        }
    }

    /// Get security level (TPM is level 2-3 depending on implementation)
    pub const fn get_security_level(&self) -> u8 {
        match self.tpm_version {
            TpmVersion::V1_2 => 2,
            TpmVersion::V2_0 => 3,
        }
    }

    /// Get TPM version
    pub const fn tpm_version(&self) -> &TpmVersion {
        &self.tpm_version
    }

    /// Get capabilities
    pub const fn capabilities(&self) -> Option<&TpmCapabilities> {
        self.capabilities.as_ref()
    }
}

#[cfg(all(test, feature = "tpm-provider"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tpm_provider_creation() -> Result<(), Box<dyn std::error::Error>> {
        let provider = TpmUniversalProvider::new().await;
        assert!(provider.is_ok());
        Ok(())
    }

    #[test]
    fn test_security_levels() -> Result<(), Box<dyn std::error::Error>> {
        // TPM 2.0 should be level 3, TPM 1.2 should be level 2
        let provider_v2 = TpmUniversalProvider {
            capabilities: None,
            tpm_version: TpmVersion::V2_0,
            metadata: HashMap::new(),
        };
        assert_eq!(provider_v2.get_security_level(), 3);

        let provider_v1 = TpmUniversalProvider {
            capabilities: None,
            tpm_version: TpmVersion::V1_2,
            metadata: HashMap::new(),
        };
        assert_eq!(provider_v1.get_security_level(), 2);

        Ok(())
    }

    #[test]
    fn test_tpm_versions() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(TpmVersion::V2_0, TpmVersion::V2_0);
        assert_eq!(TpmVersion::V1_2, TpmVersion::V1_2);
        assert_ne!(TpmVersion::V1_2, TpmVersion::V2_0);
        Ok(())
    }

    // ========================================================================
    // COMPREHENSIVE EDGE CASE TESTS (Week 1 Sprint - TPM)
    // ========================================================================

    #[tokio::test]
    async fn test_provider_default_version() -> Result<(), Box<dyn std::error::Error>> {
        // Default should be TPM 2.0
        let provider = TpmUniversalProvider::new().await?;
        assert_eq!(provider.tpm_version(), &TpmVersion::V2_0);
        assert_eq!(provider.get_security_level(), 3);
        Ok(())
    }

    #[test]
    fn test_tpm_version_security_levels() -> Result<(), Box<dyn std::error::Error>> {
        // Test security level for both versions
        let versions_and_levels = vec![(TpmVersion::V1_2, 2), (TpmVersion::V2_0, 3)];

        for (version, expected_level) in versions_and_levels {
            let provider = TpmUniversalProvider {
                capabilities: None,
                tpm_version: version.clone(),
                metadata: HashMap::new(),
            };
            assert_eq!(
                provider.get_security_level(),
                expected_level,
                "Failed for version: {:?}",
                version
            );
        }

        Ok(())
    }

    #[test]
    fn test_capabilities_structure() -> Result<(), Box<dyn std::error::Error>> {
        // Test capabilities structure
        let caps = TpmCapabilities {
            manufacturer: "Intel".to_string(),
            vendor_string: "Intel PTT".to_string(),
            firmware_version: "7.85".to_string(),
            pcr_banks: vec!["SHA256".to_string(), "SHA384".to_string()],
        };

        assert_eq!(caps.manufacturer, "Intel");
        assert_eq!(caps.vendor_string, "Intel PTT");
        assert_eq!(caps.firmware_version, "7.85");
        assert_eq!(caps.pcr_banks.len(), 2);
        assert!(caps.pcr_banks.contains(&"SHA256".to_string()));
        assert!(caps.pcr_banks.contains(&"SHA384".to_string()));

        Ok(())
    }

    #[test]
    fn test_capabilities_clone() -> Result<(), Box<dyn std::error::Error>> {
        // Test that capabilities can be cloned
        let caps1 = TpmCapabilities {
            manufacturer: "AMD".to_string(),
            vendor_string: "AMD fTPM".to_string(),
            firmware_version: "3.62".to_string(),
            pcr_banks: vec!["SHA1".to_string(), "SHA256".to_string()],
        };

        let caps2 = caps1.clone();

        assert_eq!(caps1.manufacturer, caps2.manufacturer);
        assert_eq!(caps1.vendor_string, caps2.vendor_string);
        assert_eq!(caps1.firmware_version, caps2.firmware_version);
        assert_eq!(caps1.pcr_banks, caps2.pcr_banks);

        Ok(())
    }

    #[tokio::test]
    async fn test_provider_with_capabilities() -> Result<(), Box<dyn std::error::Error>> {
        let mut provider = TpmUniversalProvider::new().await?;

        // Set capabilities
        provider.capabilities = Some(TpmCapabilities {
            manufacturer: "Infineon".to_string(),
            vendor_string: "Infineon OPTIGA TPM".to_string(),
            firmware_version: "7.85".to_string(),
            pcr_banks: vec!["SHA256".to_string()],
        });

        // Verify capabilities are accessible
        assert!(provider.capabilities().is_some());

        if let Some(caps) = provider.capabilities() {
            assert_eq!(caps.manufacturer, "Infineon");
            assert_eq!(caps.vendor_string, "Infineon OPTIGA TPM");
        }

        Ok(())
    }

    #[test]
    fn test_metadata_operations() -> Result<(), Box<dyn std::error::Error>> {
        let mut provider = TpmUniversalProvider {
            capabilities: None,
            tpm_version: TpmVersion::V2_0,
            metadata: HashMap::new(),
        };

        // Test metadata insertion and retrieval
        provider
            .metadata
            .insert("device_path".to_string(), "/dev/tpm0".to_string());
        provider
            .metadata
            .insert("driver".to_string(), "tpm_tis".to_string());

        assert_eq!(
            provider.metadata.get("device_path"),
            Some(&"/dev/tpm0".to_string())
        );
        assert_eq!(
            provider.metadata.get("driver"),
            Some(&"tpm_tis".to_string())
        );
        assert_eq!(provider.metadata.get("nonexistent"), None);

        Ok(())
    }

    #[tokio::test]
    async fn test_concurrent_provider_creation() -> Result<(), Box<dyn std::error::Error>> {
        // Test creating multiple providers concurrently
        let mut handles = vec![];

        for _ in 0..5 {
            let handle = tokio::spawn(async { TpmUniversalProvider::new().await });
            handles.push(handle);
        }

        // All providers should initialize successfully
        for handle in handles {
            let result = handle.await?;
            assert!(result.is_ok());
        }

        Ok(())
    }

    #[test]
    fn test_all_tpm_manufacturers() -> Result<(), Box<dyn std::error::Error>> {
        // Test capabilities for various TPM manufacturers
        let manufacturers = vec![
            ("Intel", "Intel PTT"),
            ("AMD", "AMD fTPM"),
            ("Infineon", "Infineon OPTIGA TPM"),
            ("STMicroelectronics", "ST33 TPM"),
            ("Nuvoton", "Nuvoton TPM"),
        ];

        for (manufacturer, vendor) in manufacturers {
            let caps = TpmCapabilities {
                manufacturer: manufacturer.to_string(),
                vendor_string: vendor.to_string(),
                firmware_version: "1.0".to_string(),
                pcr_banks: vec!["SHA256".to_string()],
            };

            assert_eq!(caps.manufacturer, manufacturer);
            assert_eq!(caps.vendor_string, vendor);
        }

        Ok(())
    }

    #[test]
    fn test_pcr_banks_variations() -> Result<(), Box<dyn std::error::Error>> {
        // Test various PCR bank configurations
        let pcr_configs = vec![
            vec!["SHA1"],
            vec!["SHA256"],
            vec!["SHA384"],
            vec!["SHA1", "SHA256"],
            vec!["SHA256", "SHA384"],
            vec!["SHA1", "SHA256", "SHA384"],
            vec!["SHA256", "SHA384", "SHA512"],
        ];

        for pcr_banks in pcr_configs {
            let caps = TpmCapabilities {
                manufacturer: "Test".to_string(),
                vendor_string: "TestTPM".to_string(),
                firmware_version: "1.0".to_string(),
                pcr_banks: pcr_banks.iter().map(|s| s.to_string()).collect(),
            };

            assert_eq!(caps.pcr_banks.len(), pcr_banks.len());
        }

        Ok(())
    }

    #[test]
    fn test_firmware_version_formats() -> Result<(), Box<dyn std::error::Error>> {
        // Test various firmware version formats
        let versions = vec!["7.85", "3.62", "1.2.3.4", "2020.05.15", "v1.0"];

        for version in versions {
            let caps = TpmCapabilities {
                manufacturer: "Test".to_string(),
                vendor_string: "TestTPM".to_string(),
                firmware_version: version.to_string(),
                pcr_banks: vec!["SHA256".to_string()],
            };

            assert_eq!(caps.firmware_version, version);
        }

        Ok(())
    }

    #[test]
    fn test_tpm_version_clone() -> Result<(), Box<dyn std::error::Error>> {
        // Test TPM version enum cloning
        let v1 = TpmVersion::V1_2;
        let v2 = v1.clone();
        assert_eq!(v1, v2);

        let v3 = TpmVersion::V2_0;
        let v4 = v3.clone();
        assert_eq!(v3, v4);

        Ok(())
    }

    #[test]
    fn test_provider_without_capabilities() -> Result<(), Box<dyn std::error::Error>> {
        let provider = TpmUniversalProvider {
            capabilities: None,
            tpm_version: TpmVersion::V2_0,
            metadata: HashMap::new(),
        };

        // Provider without capabilities should still be valid
        assert!(provider.capabilities().is_none());
        assert_eq!(provider.get_security_level(), 3);

        Ok(())
    }

    #[test]
    fn test_capabilities_with_empty_pcr_banks() -> Result<(), Box<dyn std::error::Error>> {
        // Test capabilities with no PCR banks (edge case)
        let caps = TpmCapabilities {
            manufacturer: "Test".to_string(),
            vendor_string: "TestTPM".to_string(),
            firmware_version: "1.0".to_string(),
            pcr_banks: Vec::new(),
        };

        assert!(caps.pcr_banks.is_empty());

        Ok(())
    }

    #[test]
    fn test_capabilities_with_many_pcr_banks() -> Result<(), Box<dyn std::error::Error>> {
        // Test capabilities with many PCR banks
        let banks: Vec<String> = (0..10).map(|i| format!("SHA{}", 256 + i * 128)).collect();

        let caps = TpmCapabilities {
            manufacturer: "Test".to_string(),
            vendor_string: "TestTPM".to_string(),
            firmware_version: "1.0".to_string(),
            pcr_banks: banks.clone(),
        };

        assert_eq!(caps.pcr_banks.len(), 10);
        for (i, bank) in caps.pcr_banks.iter().enumerate() {
            assert_eq!(bank, &format!("SHA{}", 256 + i * 128));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests_always {
    use super::*;

    #[tokio::test]
    async fn tpm_new_matches_feature_flag() {
        #[cfg(not(feature = "tpm-provider"))]
        {
            let err = match TpmUniversalProvider::new().await {
                Err(e) => e,
                Ok(_) => panic!("TPM provider should be disabled without feature"),
            };
            let msg = err.to_string();
            assert!(
                msg.contains("TPM")
                    || msg.contains("tpm-provider")
                    || msg.contains("Requires capability"),
                "unexpected error message: {msg}"
            );
        }
        #[cfg(feature = "tpm-provider")]
        {
            let p = TpmUniversalProvider::new()
                .await
                .expect("TPM provider should construct with feature");
            assert_eq!(p.get_security_level(), 3);
            assert_eq!(p.tpm_version(), &TpmVersion::V2_0);
        }
    }

    #[test]
    fn tpm_version_security_levels_via_manual_construct() {
        let v1 = TpmUniversalProvider {
            capabilities: None,
            tpm_version: TpmVersion::V1_2,
            metadata: HashMap::new(),
        };
        assert_eq!(v1.get_security_level(), 2);
        let v2 = TpmUniversalProvider {
            capabilities: None,
            tpm_version: TpmVersion::V2_0,
            metadata: HashMap::new(),
        };
        assert_eq!(v2.get_security_level(), 3);
        assert!(v2.capabilities().is_none());
    }

    #[test]
    fn tpm_capabilities_clone_and_debug() {
        let c = TpmCapabilities {
            manufacturer: "TestMfr".to_string(),
            vendor_string: "Vendor".to_string(),
            firmware_version: "1.0".to_string(),
            pcr_banks: vec!["SHA256".to_string()],
        };
        let c2 = c.clone();
        let s = format!("{c:?}");
        assert!(s.contains("TestMfr"), "{s}");
        assert_eq!(c2.firmware_version, "1.0");
    }
}
