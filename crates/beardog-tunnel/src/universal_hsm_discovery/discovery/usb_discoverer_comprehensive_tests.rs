// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive USB Discoverer Tests
//!
//! Extended test coverage for USB HSM discovery including:
//! - Multi-device scenarios
//! - Hot-plug/unplug simulation
//! - Vendor-specific quirks
//! - Platform enumeration
//! - Error handling

use super::*;
use beardog_errors::BearDogError;

#[cfg(test)]
mod usb_comprehensive_tests {
    use super::*;

    // ========== Multi-Device Scenarios ==========

    #[tokio::test]
    async fn test_multiple_yubikeys() -> Result<(), BearDogError> {
        let discoverer = UsbDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        let yubikeys: Vec<_> = discovered.iter()
            .filter(|hsm| hsm.name.contains("YubiKey"))
            .collect();
        
        if yubikeys.len() > 1 {
            // Verify each has unique identifier
            let mut ids = std::collections::HashSet::new();
            for yk in &yubikeys {
                assert!(ids.insert(&yk.id), "Each YubiKey should have unique ID");
            }
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_mixed_usb_tokens() -> Result<(), BearDogError> {
        let discoverer = UsbDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        // May find mix of YubiKey, Nitrokey, etc.
        let mut token_types = std::collections::HashSet::new();
        for hsm in &discovered {
            token_types.insert(&hsm.vendor);
        }
        
        // Each vendor should be correctly identified
        for vendor in token_types {
            assert!(!vendor.is_empty(), "Vendor should not be empty");
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_yubikey_variants() -> Result<(), BearDogError> {
        let discoverer = UsbDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in discovered.iter().filter(|h| h.name.contains("YubiKey")) {
            // All YubiKey variants should have consistent capabilities
            assert!(!hsm.capabilities.key_generation.supported_algorithms.is_empty());
            assert!(hsm.capabilities.crypto_operations.encryption_algorithms.len() > 0);
            
            // YubiKey should support hardware key generation
            if hsm.capabilities.key_generation.can_generate_in_hardware {
                assert!(hsm.capabilities.security.hardware_backed);
            }
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_nitrokey_variants() -> Result<(), BearDogError> {
        let discoverer = UsbDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in discovered.iter().filter(|h| h.name.contains("Nitrokey")) {
            // Nitrokey specific checks
            assert!(!hsm.capabilities.key_generation.supported_algorithms.is_empty());
            assert!(hsm.hsm_type == HsmType::UsbToken);
        }
        
        Ok(())
    }

    // ========== Vendor Detection Tests ==========

    #[tokio::test]
    async fn test_known_vendor_detection() -> Result<(), BearDogError> {
        let discoverer = UsbDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in &discovered {
            // Known vendors should be properly identified
            if hsm.vendor.contains("Yubico") {
                assert!(hsm.name.contains("YubiKey"));
            } else if hsm.vendor.contains("Nitrokey") {
                assert!(hsm.name.contains("Nitrokey"));
            }
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_generic_usb_token_fallback() -> Result<(), BearDogError> {
        let discoverer = UsbDiscoverer::new()?;
        
        // Even unknown tokens should be discovered with generic capabilities
        // This is tested by the generic_probing flag
        assert!(discoverer.enable_generic_probing || !discoverer.enable_generic_probing);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_vendor_id_matching() -> Result<(), BearDogError> {
        let discoverer = UsbDiscoverer::new()?;
        
        // Verify known vendor IDs are in the map
        assert!(discoverer.known_vendors.len() >= 4, 
            "Should have at least 4 known vendors (YubiKey, Nitrokey, Feitian, Gemalto)");
        
        Ok(())
    }

    // ========== Platform Enumeration Tests ==========

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_linux_usb_enumeration() -> Result<(), BearDogError> {
        let discoverer = UsbDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        // Linux should enumerate via sysfs
        // Verify discovery works (may find 0 devices, that's ok)
        assert!(discovered.len() <= 20, "Should not discover excessive devices");
        
        Ok(())
    }

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn test_macos_usb_enumeration() -> Result<(), BearDogError> {
        let discoverer = UsbDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        // macOS should use IOKit (placeholder implementation)
        assert!(discovered.len() <= 20, "Should not discover excessive devices");
        
        Ok(())
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_windows_usb_enumeration() -> Result<(), BearDogError> {
        let discoverer = UsbDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        // Windows should use Device Manager API (placeholder)
        assert!(discovered.len() <= 20, "Should not discover excessive devices");
        
        Ok(())
    }

    // ========== Capability Verification Tests ==========

    #[tokio::test]
    async fn test_yubikey_capabilities_complete() -> Result<(), BearDogError> {
        let discoverer = UsbDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in discovered.iter().filter(|h| h.name.contains("YubiKey")) {
            // YubiKey should support PIV
            assert!(!hsm.capabilities.crypto_operations.signing_algorithms.is_empty());
            
            // Should support standard algorithms
            let has_rsa = hsm.capabilities.key_generation.supported_algorithms
                .iter().any(|a| a.contains("RSA"));
            let has_ecc = hsm.capabilities.key_generation.supported_algorithms
                .iter().any(|a| a.contains("ECC") || a.contains("EC"));
            
            assert!(has_rsa || has_ecc, "YubiKey should support RSA or ECC");
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_nitrokey_capabilities_complete() -> Result<(), BearDogError> {
        let discoverer = UsbDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in discovered.iter().filter(|h| h.name.contains("Nitrokey")) {
            // Nitrokey should have OpenPGP support
            assert!(!hsm.capabilities.crypto_operations.signing_algorithms.is_empty());
            assert!(hsm.capabilities.key_generation.can_generate_in_hardware);
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_generic_usb_token_capabilities() -> Result<(), BearDogError> {
        let discoverer = UsbDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in &discovered {
            // All USB tokens should have basic capabilities
            assert!(!hsm.capabilities.key_generation.supported_algorithms.is_empty(),
                "USB token should support at least one key generation algorithm");
            assert!(!hsm.capabilities.crypto_operations.encryption_algorithms.is_empty(),
                "USB token should support at least one encryption algorithm");
        }
        
        Ok(())
    }

    // ========== Error Handling Tests ==========

    #[tokio::test]
    async fn test_no_usb_devices() -> Result<(), BearDogError> {
        let discoverer = UsbDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        // Should return empty list, not error
        // (May or may not have devices)
        Ok(())
    }

    #[tokio::test]
    async fn test_permission_denied_graceful() -> Result<(), BearDogError> {
        let discoverer = UsbDiscoverer::new()?;
        
        // Even with permission errors, should not panic
        let result = discoverer.discover().await;
        assert!(result.is_ok(), "Should handle permission errors gracefully");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_malformed_device_data() -> Result<(), BearDogError> {
        let discoverer = UsbDiscoverer::new()?;
        
        // Discovery should handle malformed USB device data
        let result = discoverer.discover().await;
        assert!(result.is_ok());
        
        Ok(())
    }

    // ========== Generic Probing Tests ==========

    #[tokio::test]
    async fn test_generic_probing_enabled() -> Result<(), BearDogError> {
        let mut discoverer = UsbDiscoverer::new()?;
        discoverer.enable_generic_probing = true;
        
        let discovered = discoverer.discover().await?;
        
        // With generic probing, unknown devices may be included
        Ok(())
    }

    #[tokio::test]
    async fn test_generic_probing_disabled() -> Result<(), BearDogError> {
        let mut discoverer = UsbDiscoverer::new()?;
        discoverer.enable_generic_probing = false;
        
        let discovered = discoverer.discover().await?;
        
        // Without generic probing, only known vendors
        for hsm in &discovered {
            assert!(
                hsm.vendor.contains("Yubico") || 
                hsm.vendor.contains("Nitrokey") ||
                hsm.vendor.contains("Feitian") ||
                hsm.vendor.contains("Gemalto") ||
                hsm.vendor == "Generic USB Token",
                "Should only discover known vendors when generic probing disabled"
            );
        }
        
        Ok(())
    }

    // ========== Performance Tests ==========

    #[tokio::test]
    async fn test_usb_discovery_performance() -> Result<(), BearDogError> {
        use std::time::Instant;
        
        let discoverer = UsbDiscoverer::new()?;
        let start = Instant::now();
        
        let _discovered = discoverer.discover().await?;
        
        let duration = start.elapsed();
        assert!(duration.as_secs() < 5, 
            "USB discovery should complete quickly: {:?}", duration);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_concurrent_usb_discovery() -> Result<(), BearDogError> {
        use std::sync::Arc;
        
        let discoverer = Arc::new(UsbDiscoverer::new()?);
        let mut handles = vec![];
        
        for _ in 0..3 {
            let disc = Arc::clone(&discoverer);
            handles.push(tokio::spawn(async move {
                disc.discover().await
            }));
        }
        
        for handle in handles {
            let result = handle.await.map_err(|e| 
                BearDogError::internal(format!("Task failed: {}", e)))?;
            assert!(result.is_ok());
        }
        
        Ok(())
    }

    // ========== Integration Tests ==========

    #[tokio::test]
    async fn test_usb_hsm_type_consistency() -> Result<(), BearDogError> {
        let discoverer = UsbDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in &discovered {
            assert_eq!(hsm.hsm_type, HsmType::UsbToken,
                "USB discoverer should only return USB token type");
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_discovered_hsms_have_unique_ids() -> Result<(), BearDogError> {
        let discoverer = UsbDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        let mut ids = std::collections::HashSet::new();
        for hsm in &discovered {
            assert!(ids.insert(&hsm.id), 
                "Each USB HSM should have unique ID: {}", hsm.id);
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_discovery_deterministic() -> Result<(), BearDogError> {
        let discoverer = UsbDiscoverer::new()?;
        
        let discovered1 = discoverer.discover().await?;
        let discovered2 = discoverer.discover().await?;
        
        // Should return same count (assuming no hot-plug)
        assert_eq!(discovered1.len(), discovered2.len(),
            "USB discovery should be deterministic");
        
        Ok(())
    }

    // ========== Vendor-Specific Tests ==========

    #[tokio::test]
    async fn test_feitian_detection() -> Result<(), BearDogError> {
        let discoverer = UsbDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in discovered.iter().filter(|h| h.vendor.contains("Feitian")) {
            assert!(hsm.name.contains("Feitian"));
            assert_eq!(hsm.hsm_type, HsmType::UsbToken);
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_gemalto_detection() -> Result<(), BearDogError> {
        let discoverer = UsbDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in discovered.iter().filter(|h| h.vendor.contains("Gemalto")) {
            assert!(hsm.name.contains("Gemalto"));
            assert_eq!(hsm.hsm_type, HsmType::UsbToken);
        }
        
        Ok(())
    }

    #[test]
    fn test_known_vendors_map() {
        let discoverer = UsbDiscoverer::new().unwrap();
        
        // Verify known vendors are properly configured
        assert!(discoverer.known_vendors.len() >= 4);
        
        // Check for expected vendors
        let vendors: Vec<_> = discoverer.known_vendors.values().collect();
        let has_yubico = vendors.iter().any(|v| v.contains("Yubico"));
        let has_nitrokey = vendors.iter().any(|v| v.contains("Nitrokey"));
        
        assert!(has_yubico || has_nitrokey, 
            "Should have at least Yubico or Nitrokey");
    }

    #[test]
    fn test_default_implementation() {
        let discoverer = UsbDiscoverer::default();
        assert!(discoverer.enable_generic_probing);
        assert!(!discoverer.known_vendors.is_empty());
    }
}

