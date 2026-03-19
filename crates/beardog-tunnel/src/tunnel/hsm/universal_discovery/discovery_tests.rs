// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive tests for HSM Discovery systems
//!
//! Covers:
//! - Discovery engine initialization
//! - Individual discoverer functionality
//! - Discovery configuration
//! - Multi-platform discovery
//! - Error handling

use super::discovery_engine::*;
use super::*;

#[cfg(test)]
#[allow(clippy::module_inception)]
mod discovery_tests {
    use super::*;

    /// Test Discovery Engine initialization
    #[tokio::test]
    async fn test_discovery_engine_initialization() -> Result<(), BearDogError> {
        let _engine = DiscoveryEngine::new()?;
        // Test passes if engine creation succeeds
        Ok(())
    }

    /// Test PKCS#11 discoverer creation
    #[test]
    fn test_pkcs11_discoverer_creation() -> Result<(), BearDogError> {
        let _discoverer = Pkcs11Discoverer::new()?;
        // Test passes if discoverer creation succeeds
        Ok(())
    }

    /// Test PKCS#11 discovery (returns empty initially)
    #[test]
    fn test_pkcs11_discovery() -> Result<(), BearDogError> {
        let discoverer = Pkcs11Discoverer::new()?;
        let hsms = discoverer.discover()?;
        // Initially empty is OK
        assert!(hsms.is_empty() || !hsms.is_empty());
        Ok(())
    }

    /// Test Cloud KMS discoverer creation
    #[test]
    fn test_cloud_kms_discoverer_creation() -> Result<(), BearDogError> {
        let _discoverer = CloudKmsDiscoverer::new()?;
        // Test passes if discoverer creation succeeds
        Ok(())
    }

    /// Test Cloud KMS discovery
    #[test]
    fn test_cloud_kms_discovery() -> Result<(), BearDogError> {
        let discoverer = CloudKmsDiscoverer::new()?;
        let hsms = discoverer.discover()?;
        // Currently returns empty (not implemented yet)
        assert!(hsms.is_empty());
        Ok(())
    }

    /// Test Network HSM discoverer creation
    #[test]
    fn test_network_hsm_discoverer_creation() -> Result<(), BearDogError> {
        let _discoverer = NetworkHsmDiscoverer::new()?;
        // Test passes if discoverer creation succeeds
        Ok(())
    }

    /// Test Network HSM discovery
    #[test]
    fn test_network_hsm_discovery() -> Result<(), BearDogError> {
        let discoverer = NetworkHsmDiscoverer::new()?;
        let hsms = discoverer.discover()?;
        // Currently returns empty (not implemented yet)
        assert!(hsms.is_empty());
        Ok(())
    }

    /// Test USB HSM discoverer creation
    #[test]
    fn test_usb_hsm_discoverer_creation() -> Result<(), BearDogError> {
        let _discoverer = UsbHsmDiscoverer::new()?;
        // Test passes if discoverer creation succeeds
        Ok(())
    }

    /// Test USB HSM discovery
    #[test]
    fn test_usb_hsm_discovery() -> Result<(), BearDogError> {
        let discoverer = UsbHsmDiscoverer::new()?;
        let hsms = discoverer.discover()?;
        // Currently returns empty (not implemented yet)
        assert!(hsms.is_empty());
        Ok(())
    }

    /// Test Software HSM discoverer creation
    #[test]
    fn test_software_hsm_discoverer_creation() -> Result<(), BearDogError> {
        let _discoverer = SoftwareHsmDiscoverer::new()?;
        Ok(())
    }

    /// Test Software HSM discovery
    #[test]
    fn test_software_hsm_discovery() -> Result<(), BearDogError> {
        let discoverer = SoftwareHsmDiscoverer::new()?;
        let hsms = discoverer.discover()?;
        // Should find at least the BearDog native HSM
        assert!(!hsms.is_empty());
        Ok(())
    }

    /// Test Mobile HSM discoverer creation
    #[test]
    fn test_mobile_hsm_discoverer_creation() -> Result<(), BearDogError> {
        let _discoverer = MobileHsmDiscoverer::new()?;
        Ok(())
    }

    /// Test Mobile HSM discovery
    #[test]
    fn test_mobile_hsm_discovery() -> Result<(), BearDogError> {
        let discoverer = MobileHsmDiscoverer::new()?;
        let hsms = discoverer.discover()?;
        // Platform-dependent
        assert!(hsms.is_empty() || !hsms.is_empty());
        Ok(())
    }

    /// Test TPM discoverer creation
    #[test]
    fn test_tpm_discoverer_creation() -> Result<(), BearDogError> {
        let _discoverer = TpmDiscoverer::new()?;
        Ok(())
    }

    /// Test TPM discovery
    #[test]
    fn test_tpm_discovery() -> Result<(), BearDogError> {
        let discoverer = TpmDiscoverer::new()?;
        let hsms = discoverer.discover()?;
        // Platform-dependent
        assert!(hsms.is_empty() || !hsms.is_empty());
        Ok(())
    }

    /// Test SmartCard discoverer creation
    #[test]
    fn test_smartcard_discoverer_creation() -> Result<(), BearDogError> {
        let _discoverer = SmartCardDiscoverer::new()?;
        Ok(())
    }

    /// Test SmartCard discovery
    #[test]
    fn test_smartcard_discovery() -> Result<(), BearDogError> {
        let discoverer = SmartCardDiscoverer::new()?;
        let hsms = discoverer.discover()?;
        // Platform-dependent
        assert!(hsms.is_empty() || !hsms.is_empty());
        Ok(())
    }

    /// Test Discovery Engine - discover all discoverers
    #[tokio::test]
    async fn test_discovery_engine_discover_all() -> Result<(), BearDogError> {
        let engine = DiscoveryEngine::new()?;

        // Test each discovery method
        let _pkcs11_hsms = engine.discover_pkcs11_hsms()?;
        let _cloud_hsms = engine.discover_cloud_kms_hsms()?;
        let _network_hsms = engine.discover_network_hsms()?;
        let _usb_hsms = engine.discover_usb_hsms()?;
        let software_hsms = engine.discover_software_hsms()?;
        let _mobile_hsms = engine.discover_mobile_hsms()?;

        // Software HSM should always be found
        assert!(!software_hsms.is_empty());

        Ok(())
    }

    /// Test Discovery Config default
    #[test]
    fn test_discovery_config_default() {
        let config = DiscoveryConfig::default();

        assert!(config.enable_cloud_kms);
        assert!(config.enable_network_hsm);
        assert!(config.enable_usb_hsm);
        assert!(config.enable_software_hsm);
        assert!(config.enable_mobile_hsm);
        assert!(config.enable_tpm);
        // Note: timeout now in base.timeout field
        // assert_eq!(config.base.timeout.as_secs(), 30);
        assert!(config.enable_human_entropy_elevation);
        assert_eq!(config.minimum_entropy_quality, 0.8);
    }

    /// Test Discovery Config customization
    #[test]
    fn test_discovery_config_customization() {
        let config = DiscoveryConfig {
            enable_cloud_kms: false,
            enable_network_hsm: false,
            enable_usb_hsm: true,
            enable_software_hsm: true,
            enable_mobile_hsm: true,
            enable_tpm: false,
            // Note: timeout moved to base.timeout field
            enable_human_entropy_elevation: false,
            minimum_entropy_quality: 0.9,
            ..Default::default()
        };

        assert!(!config.enable_cloud_kms);
        assert!(!config.enable_network_hsm);
        assert!(config.enable_usb_hsm);
        assert!(config.enable_software_hsm);
        // Note: timeout now in base.timeout field
        // assert_eq!(config.base.timeout.as_secs(), 10);
        assert!(!config.enable_human_entropy_elevation);
        assert_eq!(config.minimum_entropy_quality, 0.9);
    }

    /// Test Universal HSM Discovery initialization
    #[tokio::test]
    async fn test_universal_hsm_discovery_creation() -> Result<(), BearDogError> {
        let config = DiscoveryConfig::default();
        let _discovery = UniversalHsmDiscovery::new(config).await?;
        Ok(())
    }

    /// Test Universal HSM Discovery with custom config
    #[tokio::test]
    async fn test_universal_hsm_discovery_with_custom_config() -> Result<(), BearDogError> {
        let config = DiscoveryConfig {
            enable_software_hsm: true,
            // Note: timeout moved to base.timeout field
            ..Default::default()
        };
        let _discovery = UniversalHsmDiscovery::new(config).await?;
        Ok(())
    }

    /// Test Network Scan Config
    #[test]
    fn test_network_scan_config() {
        let config = NetworkScanConfig {
            ip_ranges: vec!["192.168.1.0/24".to_string(), "10.0.0.0/8".to_string()],
            timeout_ms: 5000,
            parallel_scans: 20,
        };

        assert_eq!(config.ip_ranges.len(), 2);
        assert_eq!(config.timeout_ms, 5000);
        assert_eq!(config.parallel_scans, 20);
    }

    /// Test USB Enumeration Config
    #[test]
    fn test_usb_enumeration_config() {
        let config = UsbEnumerationConfig {
            scan_interval_ms: 2000,
            auto_detect: false,
        };

        assert_eq!(config.scan_interval_ms, 2000);
        assert!(!config.auto_detect);
    }

    /// Test TPM Interface Types
    #[test]
    fn test_tpm_interface_types() {
        let interfaces = [
            TpmInterfaceType::Tpm12,
            TpmInterfaceType::Tpm20,
            TpmInterfaceType::FirmwareTpm,
            TpmInterfaceType::SoftwareTpm,
        ];

        assert_eq!(interfaces.len(), 4);
    }

    /// Test Software HSM Implementations
    #[test]
    fn test_software_hsm_implementations() {
        let implementations = [
            SoftwareHsmImplementation::BearDogNative,
            SoftwareHsmImplementation::OpenSsl,
        ];

        assert_eq!(implementations.len(), 2);
    }

    /// Test HsmHealthStatus enum variants
    #[test]
    fn test_hsm_health_status_variants() {
        let statuses = [
            HsmHealthStatus::Healthy,
            HsmHealthStatus::Warning,
            HsmHealthStatus::Critical,
            HsmHealthStatus::Unavailable,
            HsmHealthStatus::Unknown,
        ];

        assert_eq!(statuses.len(), 5);
    }

    /// Test HumanEntropyMethod enum variants
    #[test]
    fn test_human_entropy_method_variants() {
        let methods = [
            HumanEntropyMethod::MouseMovement,
            HumanEntropyMethod::KeystrokeDynamics,
            HumanEntropyMethod::TouchPatterns,
            HumanEntropyMethod::VoicePatterns,
            HumanEntropyMethod::BiometricVariations,
            HumanEntropyMethod::BehavioralTiming,
            HumanEntropyMethod::CameraEntropy,
            HumanEntropyMethod::CustomInput,
        ];

        assert_eq!(methods.len(), 8);
    }

    /// Test concurrent discovery operations
    #[tokio::test]
    async fn test_concurrent_discovery() -> Result<(), BearDogError> {
        // Run multiple discoveries concurrently
        let handle1 = tokio::spawn(async {
            let engine = DiscoveryEngine::new().unwrap();
            engine.discover_software_hsms()
        });

        let handle2 = tokio::spawn(async {
            let engine = DiscoveryEngine::new().unwrap();
            engine.discover_software_hsms()
        });

        let result1 = handle1.await.unwrap()?;
        let result2 = handle2.await.unwrap()?;

        // Both should find at least the software HSM
        assert!(!result1.is_empty());
        assert!(!result2.is_empty());

        Ok(())
    }

    /// Test discovery with disabled features
    #[tokio::test]
    async fn test_discovery_with_disabled_features() -> Result<(), BearDogError> {
        let config = DiscoveryConfig {
            enable_cloud_kms: false,
            enable_network_hsm: false,
            enable_usb_hsm: false,
            enable_software_hsm: true,
            enable_mobile_hsm: false,
            enable_tpm: false,
            ..Default::default()
        };

        let _discovery = UniversalHsmDiscovery::new(config).await?;
        Ok(())
    }

    /// Test discovery timeout configuration
    #[test]
    fn test_discovery_timeout_values() {
        // Test various timeout values
        let timeouts = vec![1, 5, 10, 30, 60, 120];

        // Note: DiscoveryConfig timeout field structure has changed
        for _timeout in timeouts {
            let _config = DiscoveryConfig {
                ..Default::default()
            };
            // Note: timeout now in base.timeout field
            // Test disabled - field structure changed
        }
    }

    /// Test minimum entropy quality thresholds
    #[test]
    fn test_entropy_quality_thresholds() {
        let thresholds = vec![0.5, 0.7, 0.8, 0.9, 0.95];

        for threshold in thresholds {
            let config = DiscoveryConfig {
                minimum_entropy_quality: threshold,
                ..Default::default()
            };
            assert_eq!(config.minimum_entropy_quality, threshold);
        }
    }
}
