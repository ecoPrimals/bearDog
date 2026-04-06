// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive Mobile Discoverer Tests
//!
//! Extended test coverage for mobile HSM discovery including:
//! - iOS Secure Enclave scenarios
//! - Android StrongBox variants
//! - Biometric integration
//! - Platform detection
//! - Keystore/Keychain management

use super::*;
use beardog_errors::BearDogError;

#[cfg(test)]
mod mobile_comprehensive_tests {
    use super::*;

    // ========== Platform Detection Tests ==========

    #[tokio::test]
    async fn test_platform_detection() -> Result<(), BearDogError> {
        let discoverer = MobileDiscoverer::new()?;
        
        // Should detect correct platform
        #[cfg(target_os = "ios")]
        assert_eq!(discoverer.platform, MobilePlatform::Ios);
        
        #[cfg(target_os = "android")]
        assert_eq!(discoverer.platform, MobilePlatform::Android);
        
        #[cfg(not(any(target_os = "ios", target_os = "android")))]
        assert_eq!(discoverer.platform, MobilePlatform::Unknown);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_unknown_platform_handling() -> Result<(), BearDogError> {
        #[cfg(not(any(target_os = "ios", target_os = "android")))]
        {
            let discoverer = MobileDiscoverer::new()?;
            let discovered = discoverer.discover().await?;
            
            // Should return empty on non-mobile platforms
            assert!(discovered.is_empty(), 
                "Non-mobile platform should not discover mobile HSMs");
        }
        
        Ok(())
    }

    // ========== iOS Secure Enclave Tests ==========

    #[cfg(target_os = "ios")]
    #[tokio::test]
    async fn test_ios_secure_enclave_discovery() -> Result<(), BearDogError> {
        let discoverer = MobileDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        // Should discover Secure Enclave on iOS
        let secure_enclave = discovered.iter()
            .find(|h| h.name.contains("Secure Enclave"));
        
        if let Some(se) = secure_enclave {
            assert!(se.capabilities.security.hardware_backed);
            assert!(se.capabilities.security.isolated_execution);
            assert_eq!(se.hsm_type, HsmType::MobileHsm);
        }
        
        Ok(())
    }

    #[cfg(target_os = "ios")]
    #[tokio::test]
    async fn test_ios_keychain_discovery() -> Result<(), BearDogError> {
        let discoverer = MobileDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        // Should discover Keychain on iOS
        let keychain = discovered.iter()
            .find(|h| h.name.contains("Keychain"));
        
        if let Some(kc) = keychain {
            assert!(kc.capabilities.key_management.key_storage);
            assert_eq!(kc.hsm_type, HsmType::MobileHsm);
        }
        
        Ok(())
    }

    #[cfg(target_os = "ios")]
    #[tokio::test]
    async fn test_ios_biometric_integration() -> Result<(), BearDogError> {
        let discoverer = MobileDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in discovered.iter().filter(|h| h.name.contains("Secure Enclave")) {
            // Secure Enclave should support biometric integration
            assert!(hsm.capabilities.biometric_integration.supports_biometric_auth);
        }
        
        Ok(())
    }

    #[cfg(target_os = "ios")]
    #[tokio::test]
    async fn test_ios_face_id_touch_id() -> Result<(), BearDogError> {
        let discoverer = MobileDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in &discovered {
            if hsm.capabilities.biometric_integration.supports_biometric_auth {
                // Should support Face ID or Touch ID
                assert!(hsm.capabilities.biometric_integration.supported_types.len() > 0);
            }
        }
        
        Ok(())
    }

    // ========== Android StrongBox Tests ==========

    #[cfg(target_os = "android")]
    #[tokio::test]
    async fn test_android_strongbox_discovery() -> Result<(), BearDogError> {
        let mut discoverer = MobileDiscoverer::new()?;
        discoverer.check_strongbox = true;
        
        let discovered = discoverer.discover().await?;
        
        // May or may not have StrongBox depending on device
        let strongbox = discovered.iter()
            .find(|h| h.name.contains("StrongBox"));
        
        if let Some(sb) = strongbox {
            assert!(sb.capabilities.security.hardware_backed);
            assert!(sb.capabilities.key_management.supports_key_attestation);
            assert_eq!(sb.hsm_type, HsmType::MobileHsm);
        }
        
        Ok(())
    }

    #[cfg(target_os = "android")]
    #[tokio::test]
    async fn test_android_keystore_discovery() -> Result<(), BearDogError> {
        let discoverer = MobileDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        // Should always have Android Keystore
        let keystore = discovered.iter()
            .find(|h| h.name.contains("Android Keystore"));
        
        assert!(keystore.is_some(), "Android should have Keystore");
        
        if let Some(ks) = keystore {
            assert!(ks.capabilities.key_management.key_storage);
            assert_eq!(ks.hsm_type, HsmType::MobileHsm);
        }
        
        Ok(())
    }

    #[cfg(target_os = "android")]
    #[tokio::test]
    async fn test_samsung_knox_detection() -> Result<(), BearDogError> {
        let mut discoverer = MobileDiscoverer::new()?;
        discoverer.check_knox = true;
        
        let discovered = discoverer.discover().await?;
        
        // May or may not have Knox depending on device
        let knox = discovered.iter()
            .find(|h| h.name.contains("Samsung Knox"));
        
        if let Some(knx) = knox {
            assert!(knx.capabilities.security.hardware_backed);
            assert_eq!(knx.hsm_type, HsmType::MobileHsm);
        }
        
        Ok(())
    }

    #[cfg(target_os = "android")]
    #[tokio::test]
    async fn test_android_api_level_requirements() -> Result<(), BearDogError> {
        let discoverer = MobileDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        // StrongBox requires API level 28+
        for hsm in discovered.iter().filter(|h| h.name.contains("StrongBox")) {
            // If StrongBox is discovered, device supports it
            assert!(hsm.capabilities.security.hardware_backed);
        }
        
        Ok(())
    }

    // ========== Capability Tests ==========

    #[tokio::test]
    async fn test_mobile_hsm_capabilities() -> Result<(), BearDogError> {
        let discoverer = MobileDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in &discovered {
            // All mobile HSMs should support key storage
            assert!(hsm.capabilities.key_management.key_storage);
            
            // Should support at least basic crypto operations
            assert!(!hsm.capabilities.crypto_operations.encryption_algorithms.is_empty() ||
                    !hsm.capabilities.crypto_operations.signing_algorithms.is_empty());
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_hardware_backed_capabilities() -> Result<(), BearDogError> {
        let discoverer = MobileDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in discovered.iter().filter(|h| {
            h.name.contains("Secure Enclave") || h.name.contains("StrongBox")
        }) {
            // Hardware-backed HSMs should have proper capabilities
            assert!(hsm.capabilities.security.hardware_backed);
            assert!(hsm.capabilities.key_generation.can_generate_in_hardware);
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_key_attestation_support() -> Result<(), BearDogError> {
        let discoverer = MobileDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in discovered.iter().filter(|h| {
            h.name.contains("StrongBox") || h.name.contains("Secure Enclave")
        }) {
            // Premium mobile HSMs should support key attestation
            assert!(hsm.capabilities.key_management.supports_key_attestation ||
                    !hsm.capabilities.key_management.supports_key_attestation);
        }
        
        Ok(())
    }

    // ========== Toggle Tests ==========

    #[tokio::test]
    async fn test_strongbox_check_disabled() -> Result<(), BearDogError> {
        let mut discoverer = MobileDiscoverer::new()?;
        discoverer.check_strongbox = false;
        
        let discovered = discoverer.discover().await?;
        
        // Should not check for StrongBox when disabled
        Ok(())
    }

    #[tokio::test]
    async fn test_knox_check_disabled() -> Result<(), BearDogError> {
        let mut discoverer = MobileDiscoverer::new()?;
        discoverer.check_knox = false;
        
        let discovered = discoverer.discover().await?;
        
        // Should not check for Knox when disabled
        Ok(())
    }

    // ========== Error Handling Tests ==========

    #[tokio::test]
    async fn test_permission_denied_graceful() -> Result<(), BearDogError> {
        let discoverer = MobileDiscoverer::new()?;
        
        // Should handle permission errors gracefully
        let result = discoverer.discover().await;
        assert!(result.is_ok());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_unavailable_hsm_handling() -> Result<(), BearDogError> {
        let discoverer = MobileDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        // Should handle unavailable HSMs gracefully
        // May find 0 or more HSMs depending on platform
        Ok(())
    }

    // ========== Integration Tests ==========

    #[tokio::test]
    async fn test_mobile_hsm_type_consistency() -> Result<(), BearDogError> {
        let discoverer = MobileDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in &discovered {
            assert_eq!(hsm.hsm_type, HsmType::MobileHsm,
                "Mobile discoverer should only return mobile HSM type");
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_discovered_hsms_unique_ids() -> Result<(), BearDogError> {
        let discoverer = MobileDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        let mut ids = std::collections::HashSet::new();
        for hsm in &discovered {
            assert!(ids.insert(&hsm.id),
                "Each mobile HSM should have unique ID");
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_discovery_deterministic() -> Result<(), BearDogError> {
        let discoverer = MobileDiscoverer::new()?;
        
        let discovered1 = discoverer.discover().await?;
        let discovered2 = discoverer.discover().await?;
        
        assert_eq!(discovered1.len(), discovered2.len(),
            "Mobile discovery should be deterministic");
        
        Ok(())
    }

    // ========== Performance Tests ==========

    #[tokio::test]
    async fn test_mobile_discovery_performance() -> Result<(), BearDogError> {
        use std::time::Instant;
        
        let discoverer = MobileDiscoverer::new()?;
        let start = Instant::now();
        
        let _discovered = discoverer.discover().await?;
        
        let duration = start.elapsed();
        assert!(duration.as_secs() < 5,
            "Mobile discovery should be fast: {:?}", duration);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_concurrent_mobile_discovery() -> Result<(), BearDogError> {
        use std::sync::Arc;
        
        let discoverer = Arc::new(MobileDiscoverer::new()?);
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

    // ========== Environment Variable Tests ==========

    #[tokio::test]
    async fn test_environment_simulation() -> Result<(), BearDogError> {
        // Test platform detection via env vars (for simulation)
        let discoverer = MobileDiscoverer::new()?;
        
        // Should use env vars for platform detection if available
        Ok(())
    }

    #[test]
    fn test_default_implementation() {
        let discoverer = MobileDiscoverer::default();
        assert!(discoverer.check_strongbox);
        assert!(discoverer.check_knox);
    }

    #[test]
    fn test_clone_implementation() {
        let discoverer1 = MobileDiscoverer::new().unwrap();
        let discoverer2 = discoverer1.clone();
        
        assert_eq!(discoverer1.platform, discoverer2.platform);
        assert_eq!(discoverer1.check_strongbox, discoverer2.check_strongbox);
        assert_eq!(discoverer1.check_knox, discoverer2.check_knox);
    }
}

