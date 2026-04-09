// SPDX-License-Identifier: AGPL-3.0-or-later

//! PKCS#11 HSM Provider
//!
//! Universal provider implementation for PKCS#11 hardware security modules.

use beardog_errors::BearDogError;
use std::collections::HashMap;
use tracing::debug;

/// PKCS#11 Universal HSM Provider
pub struct Pkcs11UniversalProvider {
    /// HSM capabilities
    capabilities: Option<Pkcs11Capabilities>,
    /// PKCS#11 library path
    library_path: String,
    /// Slot ID
    slot_id: u64,
    /// Provider metadata
    metadata: HashMap<String, String>,
}

/// PKCS#11-specific HSM capabilities
#[derive(Debug, Clone)]
pub struct Pkcs11Capabilities {
    /// Manufacturer ID
    pub manufacturer_id: String,
    /// Model name
    pub model: String,
    /// Serial number
    pub serial_number: String,
    /// Hardware version
    pub hardware_version: String,
    /// Firmware version
    pub firmware_version: String,
}

impl Pkcs11UniversalProvider {
    /// Create a new PKCS#11 HSM provider
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub async fn new(library_path: String, slot_id: u64) -> Result<Self, BearDogError> {
        #[cfg(not(feature = "pkcs11-provider"))]
        {
            debug!(
                library_path = %library_path,
                slot_id,
                "PKCS#11 provider not compiled in (enable `pkcs11-provider` feature and link a PKCS#11 library)"
            );
            Err(BearDogError::requires_capability(
                "pkcs11-provider",
                "PKCS#11 HSM provider requires the pkcs11-provider Cargo feature and a PKCS#11 library",
            ))
        }
        #[cfg(feature = "pkcs11-provider")]
        {
            let provider = Self {
                capabilities: None,
                library_path,
                slot_id,
                metadata: HashMap::with_capacity(16),
            };
            // Phase 2: session and crypto operations require PKCS#11 bindings (pure Rust pkcs11 crate).
            Ok(provider)
        }
    }

    /// Get security level (hardware HSM is level 3)
    pub const fn get_security_level(&self) -> u8 {
        3 // Hardware HSM
    }

    /// Get capabilities
    pub const fn capabilities(&self) -> Option<&Pkcs11Capabilities> {
        self.capabilities.as_ref()
    }

    /// Get the configured PKCS#11 library path
    pub fn library_path(&self) -> &str {
        &self.library_path
    }

    /// Get the configured slot ID
    pub const fn slot_id(&self) -> u64 {
        self.slot_id
    }

    /// Get provider metadata
    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }
}

#[cfg(all(test, feature = "pkcs11-provider"))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pkcs11_provider_creation() -> Result<(), Box<dyn std::error::Error>> {
        let provider =
            Pkcs11UniversalProvider::new("/usr/lib/softhsm/libsofthsm2.so".to_string(), 0).await;
        assert!(provider.is_ok());
        Ok(())
    }

    #[test]
    fn test_security_level() -> Result<(), Box<dyn std::error::Error>> {
        // PKCS#11 HSMs are hardware level (always 3)
        let provider = Pkcs11UniversalProvider {
            capabilities: None,
            library_path: "/usr/lib/softhsm/libsofthsm2.so".to_string(),
            slot_id: 0,
            metadata: HashMap::new(),
        };
        assert_eq!(provider.get_security_level(), 3);
        Ok(())
    }

    // ========================================================================
    // COMPREHENSIVE EDGE CASE TESTS (Week 1 Sprint - PKCS#11)
    // ========================================================================

    #[tokio::test]
    async fn test_provider_creation_various_paths() -> Result<(), Box<dyn std::error::Error>> {
        // Test creation with different library paths
        let paths = vec![
            "/usr/lib/softhsm/libsofthsm2.so",
            "/opt/nfast/toolkits/pkcs11/libcknfast.so",
            "/usr/lib/libeToken.so",
            "/usr/local/lib/opensc-pkcs11.so",
        ];

        for path in paths {
            let provider = Pkcs11UniversalProvider::new(path.to_string(), 0).await;
            assert!(
                provider.is_ok(),
                "Failed to create provider with path: {}",
                path
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_multiple_slot_ids() -> Result<(), Box<dyn std::error::Error>> {
        // Test creation with different slot IDs
        let slots = vec![0, 1, 2, 10, 100, 65535];

        for slot in slots {
            let provider =
                Pkcs11UniversalProvider::new("/usr/lib/softhsm/libsofthsm2.so".to_string(), slot)
                    .await?;

            assert_eq!(provider.slot_id, slot);
        }

        Ok(())
    }

    #[test]
    fn test_capabilities_structure() -> Result<(), Box<dyn std::error::Error>> {
        // Test capabilities structure
        let caps = Pkcs11Capabilities {
            manufacturer_id: "Thales".to_string(),
            model: "nShield Edge".to_string(),
            serial_number: "ABC123456789".to_string(),
            hardware_version: "3.2.1".to_string(),
            firmware_version: "12.50.11".to_string(),
        };

        assert_eq!(caps.manufacturer_id, "Thales");
        assert_eq!(caps.model, "nShield Edge");
        assert_eq!(caps.serial_number, "ABC123456789");
        assert_eq!(caps.hardware_version, "3.2.1");
        assert_eq!(caps.firmware_version, "12.50.11");

        Ok(())
    }

    #[test]
    fn test_capabilities_clone() -> Result<(), Box<dyn std::error::Error>> {
        // Test that capabilities can be cloned
        let caps1 = Pkcs11Capabilities {
            manufacturer_id: "Gemalto".to_string(),
            model: "SafeNet Luna".to_string(),
            serial_number: "XYZ987654321".to_string(),
            hardware_version: "7.0.0".to_string(),
            firmware_version: "7.0.3".to_string(),
        };

        let caps2 = caps1.clone();

        assert_eq!(caps1.manufacturer_id, caps2.manufacturer_id);
        assert_eq!(caps1.model, caps2.model);
        assert_eq!(caps1.serial_number, caps2.serial_number);
        assert_eq!(caps1.hardware_version, caps2.hardware_version);
        assert_eq!(caps1.firmware_version, caps2.firmware_version);

        Ok(())
    }

    #[tokio::test]
    async fn test_provider_with_capabilities() -> Result<(), Box<dyn std::error::Error>> {
        let mut provider =
            Pkcs11UniversalProvider::new("/usr/lib/softhsm/libsofthsm2.so".to_string(), 0).await?;

        // Set capabilities
        provider.capabilities = Some(Pkcs11Capabilities {
            manufacturer_id: "SoftHSM".to_string(),
            model: "SoftHSM v2".to_string(),
            serial_number: "0000000000000000".to_string(),
            hardware_version: "2.0".to_string(),
            firmware_version: "2.6.1".to_string(),
        });

        // Verify capabilities are accessible
        assert!(provider.capabilities().is_some());

        if let Some(caps) = provider.capabilities() {
            assert_eq!(caps.manufacturer_id, "SoftHSM");
            assert_eq!(caps.model, "SoftHSM v2");
        }

        Ok(())
    }

    #[test]
    fn test_metadata_operations() -> Result<(), Box<dyn std::error::Error>> {
        let mut provider = Pkcs11UniversalProvider {
            capabilities: None,
            library_path: "/usr/lib/softhsm/libsofthsm2.so".to_string(),
            slot_id: 0,
            metadata: HashMap::new(),
        };

        // Test metadata insertion and retrieval
        provider
            .metadata
            .insert("slot_description".to_string(), "Test Slot".to_string());
        provider
            .metadata
            .insert("token_label".to_string(), "MyToken".to_string());

        assert_eq!(
            provider.metadata.get("slot_description"),
            Some(&"Test Slot".to_string())
        );
        assert_eq!(
            provider.metadata.get("token_label"),
            Some(&"MyToken".to_string())
        );
        assert_eq!(provider.metadata.get("nonexistent"), None);

        Ok(())
    }

    #[tokio::test]
    async fn test_concurrent_provider_creation() -> Result<(), Box<dyn std::error::Error>> {
        // Test creating multiple providers concurrently
        let mut handles = vec![];

        for i in 0..5 {
            let handle = tokio::spawn(async move {
                Pkcs11UniversalProvider::new("/usr/lib/softhsm/libsofthsm2.so".to_string(), i).await
            });
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
    fn test_all_hsm_manufacturers() -> Result<(), Box<dyn std::error::Error>> {
        // Test capabilities for various HSM manufacturers
        let manufacturers = vec![
            ("Thales", "nShield Edge"),
            ("Gemalto", "SafeNet Luna SA"),
            ("Utimaco", "SecurityServer SE"),
            ("AWS", "CloudHSM"),
            ("YubiHSM", "YubiHSM 2"),
            ("SoftHSM", "SoftHSM v2"),
        ];

        for (manufacturer, model) in manufacturers {
            let caps = Pkcs11Capabilities {
                manufacturer_id: manufacturer.to_string(),
                model: model.to_string(),
                serial_number: "TEST123".to_string(),
                hardware_version: "1.0".to_string(),
                firmware_version: "1.0.0".to_string(),
            };

            assert_eq!(caps.manufacturer_id, manufacturer);
            assert_eq!(caps.model, model);
        }

        Ok(())
    }

    #[test]
    fn test_library_path_variations() -> Result<(), Box<dyn std::error::Error>> {
        // Test various library path formats
        let paths = vec![
            "/usr/lib/softhsm/libsofthsm2.so",
            "/opt/nfast/toolkits/pkcs11/libcknfast.so",
            "/usr/lib/x86_64-linux-gnu/pkcs11/opensc-pkcs11.so",
            "/Library/OpenSC/lib/opensc-pkcs11.so", // macOS
            "C:\\Program Files\\OpenSC\\pkcs11\\opensc-pkcs11.dll", // Windows
        ];

        for path in paths {
            let provider = Pkcs11UniversalProvider {
                capabilities: None,
                library_path: path.to_string(),
                slot_id: 0,
                metadata: HashMap::new(),
            };

            assert_eq!(provider.library_path, path);
            assert_eq!(provider.get_security_level(), 3);
        }

        Ok(())
    }

    #[test]
    fn test_version_string_formats() -> Result<(), Box<dyn std::error::Error>> {
        // Test various version string formats
        let versions = vec![
            ("1.0", "1.0.0"),
            ("2.1", "2.1.5"),
            ("10.5", "10.5.3"),
            ("7.0.0", "7.0.3"),
            ("12.50.11", "12.60.12"),
        ];

        for (hw_ver, fw_ver) in versions {
            let caps = Pkcs11Capabilities {
                manufacturer_id: "Test".to_string(),
                model: "TestHSM".to_string(),
                serial_number: "000000".to_string(),
                hardware_version: hw_ver.to_string(),
                firmware_version: fw_ver.to_string(),
            };

            assert_eq!(caps.hardware_version, hw_ver);
            assert_eq!(caps.firmware_version, fw_ver);
        }

        Ok(())
    }

    #[test]
    fn test_serial_number_formats() -> Result<(), Box<dyn std::error::Error>> {
        // Test various serial number formats
        let serial_numbers = vec![
            "ABC123456789",
            "0000000000000000",
            "XYZ-987-654-321",
            "ABCDEF1234567890",
            "12:34:56:78:90:AB",
        ];

        for serial in serial_numbers {
            let caps = Pkcs11Capabilities {
                manufacturer_id: "Test".to_string(),
                model: "TestHSM".to_string(),
                serial_number: serial.to_string(),
                hardware_version: "1.0".to_string(),
                firmware_version: "1.0.0".to_string(),
            };

            assert_eq!(caps.serial_number, serial);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_slot_id_boundary_values() -> Result<(), Box<dyn std::error::Error>> {
        // Test boundary values for slot IDs
        let slots = vec![0, 1, u64::MAX];

        for slot in slots {
            let provider =
                Pkcs11UniversalProvider::new("/usr/lib/softhsm/libsofthsm2.so".to_string(), slot)
                    .await?;

            assert_eq!(provider.slot_id, slot);
            assert_eq!(provider.get_security_level(), 3);
        }

        Ok(())
    }

    #[test]
    fn test_provider_without_capabilities() -> Result<(), Box<dyn std::error::Error>> {
        let provider = Pkcs11UniversalProvider {
            capabilities: None,
            library_path: "/usr/lib/softhsm/libsofthsm2.so".to_string(),
            slot_id: 0,
            metadata: HashMap::new(),
        };

        // Provider without capabilities should still be valid
        assert!(provider.capabilities().is_none());
        assert_eq!(provider.get_security_level(), 3);

        Ok(())
    }

    #[tokio::test]
    async fn test_multiple_providers_different_slots() -> Result<(), Box<dyn std::error::Error>> {
        // Test multiple providers for different slots
        let provider1 =
            Pkcs11UniversalProvider::new("/usr/lib/softhsm/libsofthsm2.so".to_string(), 0).await?;

        let provider2 =
            Pkcs11UniversalProvider::new("/usr/lib/softhsm/libsofthsm2.so".to_string(), 1).await?;

        let provider3 =
            Pkcs11UniversalProvider::new("/usr/lib/softhsm/libsofthsm2.so".to_string(), 2).await?;

        assert_eq!(provider1.slot_id, 0);
        assert_eq!(provider2.slot_id, 1);
        assert_eq!(provider3.slot_id, 2);

        // All should have hardware security level
        assert_eq!(provider1.get_security_level(), 3);
        assert_eq!(provider2.get_security_level(), 3);
        assert_eq!(provider3.get_security_level(), 3);

        Ok(())
    }

    #[test]
    fn test_capabilities_with_empty_strings() -> Result<(), Box<dyn std::error::Error>> {
        // Test capabilities with empty strings (edge case)
        let caps = Pkcs11Capabilities {
            manufacturer_id: String::new(),
            model: String::new(),
            serial_number: String::new(),
            hardware_version: String::new(),
            firmware_version: String::new(),
        };

        assert_eq!(caps.manufacturer_id, "");
        assert_eq!(caps.model, "");
        assert_eq!(caps.serial_number, "");
        assert_eq!(caps.hardware_version, "");
        assert_eq!(caps.firmware_version, "");

        Ok(())
    }
}
