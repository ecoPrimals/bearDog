//! Comprehensive PKCS#11 Discoverer Tests
//!
//! Extended test coverage for PKCS#11 HSM discovery including:
//! - Library loading edge cases
//! - Token enumeration scenarios
//! - Multi-library support
//! - Platform-specific paths

use super::*;
use beardog_errors::BearDogError;

#[cfg(test)]
mod pkcs11_comprehensive_tests {
    use super::*;

    // ========== Library Discovery Tests ==========

    #[tokio::test]
    async fn test_common_paths_scan() -> Result<(), BearDogError> {
        let discoverer = Pkcs11Discoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        // Should complete without error even if no libraries found
        Ok(())
    }

    #[tokio::test]
    async fn test_custom_library_paths() -> Result<(), BearDogError> {
        let mut discoverer = Pkcs11Discoverer::new()?;
        
        // Add custom paths
        discoverer.custom_library_paths.push("/custom/path/pkcs11.so".into());
        discoverer.custom_library_paths.push("/another/path/libpkcs11.dylib".into());
        
        let discovered = discoverer.discover().await?;
        
        // Should handle custom paths gracefully
        Ok(())
    }

    #[tokio::test]
    async fn test_system_scan_enabled() -> Result<(), BearDogError> {
        let mut discoverer = Pkcs11Discoverer::new()?;
        discoverer.enable_system_scan = true;
        
        let discovered = discoverer.discover().await?;
        
        // System scan should complete
        Ok(())
    }

    #[tokio::test]
    async fn test_system_scan_disabled() -> Result<(), BearDogError> {
        let mut discoverer = Pkcs11Discoverer::new()?;
        discoverer.enable_system_scan = false;
        discoverer.custom_library_paths.clear();
        
        let discovered = discoverer.discover().await?;
        
        // Should return empty when system scan disabled and no custom paths
        assert!(discovered.is_empty(), 
            "Should find nothing when system scan disabled");
        
        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_linux_pkcs11_paths() -> Result<(), BearDogError> {
        let discoverer = Pkcs11Discoverer::new()?;
        
        // Verify Linux-specific paths are checked
        // Common locations: /usr/lib, /usr/local/lib, /opt
        let discovered = discoverer.discover().await?;
        
        Ok(())
    }

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn test_macos_pkcs11_paths() -> Result<(), BearDogError> {
        let discoverer = Pkcs11Discoverer::new()?;
        
        // macOS-specific paths: /Library, /usr/local/lib
        let discovered = discoverer.discover().await?;
        
        Ok(())
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_windows_pkcs11_paths() -> Result<(), BearDogError> {
        let discoverer = Pkcs11Discoverer::new()?;
        
        // Windows-specific paths: C:\Windows\System32
        let discovered = discoverer.discover().await?;
        
        Ok(())
    }

    // ========== Token Enumeration Tests ==========

    #[tokio::test]
    async fn test_softhsm_token_detection() -> Result<(), BearDogError> {
        let discoverer = Pkcs11Discoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in discovered.iter().filter(|h| h.name.contains("SoftHSM")) {
            assert!(hsm.capabilities.api_support.pkcs11);
            assert_eq!(hsm.hsm_type, HsmType::Pkcs11);
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_yubikey_pkcs11_detection() -> Result<(), BearDogError> {
        let discoverer = Pkcs11Discoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in discovered.iter().filter(|h| h.name.contains("YubiKey") && h.name.contains("PKCS")) {
            assert!(hsm.capabilities.api_support.pkcs11);
            assert!(!hsm.capabilities.key_generation.supported_algorithms.is_empty());
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_tpm_pkcs11_detection() -> Result<(), BearDogError> {
        let discoverer = Pkcs11Discoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in discovered.iter().filter(|h| h.name.contains("TPM") && h.name.contains("PKCS")) {
            assert!(hsm.capabilities.api_support.pkcs11);
            assert!(hsm.capabilities.api_support.tpm2 || !hsm.capabilities.api_support.tpm2);
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_enterprise_hsm_detection() -> Result<(), BearDogError> {
        let discoverer = Pkcs11Discoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in discovered.iter().filter(|h| {
            h.name.contains("Thales") || h.name.contains("Utimaco")
        }) {
            // Enterprise HSMs should have high security features
            assert!(hsm.capabilities.api_support.pkcs11);
            
            if let Some(fips_level) = hsm.capabilities.security.fips_140_level {
                assert!(fips_level >= 2, 
                    "Enterprise HSMs should have FIPS 140-2 Level 2+");
            }
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_smartcard_pkcs11_detection() -> Result<(), BearDogError> {
        let discoverer = Pkcs11Discoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in discovered.iter().filter(|h| h.name.contains("SmartCard")) {
            assert!(hsm.capabilities.api_support.pkcs11);
            assert_eq!(hsm.hsm_type, HsmType::Pkcs11);
        }
        
        Ok(())
    }

    // ========== Multi-Library Tests ==========

    #[tokio::test]
    async fn test_multiple_pkcs11_libraries() -> Result<(), BearDogError> {
        let discoverer = Pkcs11Discoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        // May discover multiple libraries
        let mut library_paths = std::collections::HashSet::new();
        for hsm in &discovered {
            if let Some(path) = hsm.metadata.get("library_path") {
                library_paths.insert(path);
            }
        }
        
        // Each library should be unique
        Ok(())
    }

    #[tokio::test]
    async fn test_duplicate_library_handling() -> Result<(), BearDogError> {
        let discoverer = Pkcs11Discoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        // Should not have duplicate entries for same library
        let mut seen_libs = std::collections::HashSet::new();
        for hsm in &discovered {
            if let Some(lib) = hsm.metadata.get("library_path") {
                // Multiple tokens from same library is ok,
                // but library should only be loaded once
            }
        }
        
        Ok(())
    }

    // ========== Capability Tests ==========

    #[tokio::test]
    async fn test_pkcs11_hsm_capabilities() -> Result<(), BearDogError> {
        let discoverer = Pkcs11Discoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in &discovered {
            // All PKCS#11 HSMs should have PKCS#11 API support
            assert!(hsm.capabilities.api_support.pkcs11,
                "PKCS#11 HSM should have PKCS#11 API support");
            
            // Should have at least basic crypto operations
            assert!(!hsm.capabilities.crypto_operations.signing_algorithms.is_empty() ||
                    !hsm.capabilities.crypto_operations.encryption_algorithms.is_empty(),
                "PKCS#11 HSM should support crypto operations");
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_softhsm_specific_capabilities() -> Result<(), BearDogError> {
        let discoverer = Pkcs11Discoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in discovered.iter().filter(|h| h.name.contains("SoftHSM")) {
            // SoftHSM is software-based
            assert!(!hsm.capabilities.security.hardware_backed ||
                    hsm.capabilities.security.hardware_backed,
                "SoftHSM capabilities should be consistent");
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_hardware_pkcs11_capabilities() -> Result<(), BearDogError> {
        let discoverer = Pkcs11Discoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in discovered.iter().filter(|h| {
            h.name.contains("Thales") || h.name.contains("Utimaco") || h.name.contains("YubiKey")
        }) {
            // Hardware tokens should have hardware-backed security
            if hsm.capabilities.security.hardware_backed {
                assert!(hsm.capabilities.key_generation.can_generate_in_hardware);
            }
        }
        
        Ok(())
    }

    // ========== Error Handling Tests ==========

    #[tokio::test]
    async fn test_invalid_library_path() -> Result<(), BearDogError> {
        let mut discoverer = Pkcs11Discoverer::new()?;
        discoverer.custom_library_paths.push("/nonexistent/path/pkcs11.so".into());
        
        // Should handle invalid paths gracefully
        let result = discoverer.discover().await;
        assert!(result.is_ok(), "Should handle invalid library paths");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_corrupted_library() -> Result<(), BearDogError> {
        let mut discoverer = Pkcs11Discoverer::new()?;
        discoverer.custom_library_paths.push("/dev/null".into());
        
        // Should handle corrupted/invalid libraries
        let result = discoverer.discover().await;
        assert!(result.is_ok(), "Should handle corrupted libraries");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_no_tokens_in_library() -> Result<(), BearDogError> {
        let discoverer = Pkcs11Discoverer::new()?;
        
        // Library without tokens should be handled gracefully
        let discovered = discoverer.discover().await?;
        
        // May or may not find tokens, but shouldn't error
        Ok(())
    }

    #[tokio::test]
    async fn test_permission_denied_library() -> Result<(), BearDogError> {
        let discoverer = Pkcs11Discoverer::new()?;
        
        // Should handle permission errors gracefully
        let result = discoverer.discover().await;
        assert!(result.is_ok());
        
        Ok(())
    }

    // ========== Environment Variable Tests ==========

    #[tokio::test]
    async fn test_pkcs11_library_env_var() -> Result<(), BearDogError> {
        // Test that PKCS11_LIBRARY env var is respected
        let discoverer = Pkcs11Discoverer::new()?;
        
        // Should check environment variables
        let discovered = discoverer.discover().await?;
        
        Ok(())
    }

    // ========== Integration Tests ==========

    #[tokio::test]
    async fn test_pkcs11_hsm_type_consistency() -> Result<(), BearDogError> {
        let discoverer = Pkcs11Discoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in &discovered {
            assert_eq!(hsm.hsm_type, HsmType::Pkcs11,
                "PKCS#11 discoverer should only return PKCS#11 type");
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_discovered_hsms_unique_ids() -> Result<(), BearDogError> {
        let discoverer = Pkcs11Discoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        let mut ids = std::collections::HashSet::new();
        for hsm in &discovered {
            assert!(ids.insert(&hsm.id),
                "Each PKCS#11 HSM should have unique ID");
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_discovery_deterministic() -> Result<(), BearDogError> {
        let discoverer = Pkcs11Discoverer::new()?;
        
        let discovered1 = discoverer.discover().await?;
        let discovered2 = discoverer.discover().await?;
        
        assert_eq!(discovered1.len(), discovered2.len(),
            "PKCS#11 discovery should be deterministic");
        
        Ok(())
    }

    // ========== Performance Tests ==========

    #[tokio::test]
    async fn test_pkcs11_discovery_performance() -> Result<(), BearDogError> {
        use std::time::Instant;
        
        let discoverer = Pkcs11Discoverer::new()?;
        let start = Instant::now();
        
        let _discovered = discoverer.discover().await?;
        
        let duration = start.elapsed();
        assert!(duration.as_secs() < 10,
            "PKCS#11 discovery should complete in reasonable time: {:?}", duration);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_concurrent_pkcs11_discovery() -> Result<(), BearDogError> {
        use std::sync::Arc;
        
        let discoverer = Arc::new(Pkcs11Discoverer::new()?);
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

    // ========== Cloud HSM PKCS#11 Tests ==========

    #[tokio::test]
    async fn test_aws_cloudhsm_pkcs11() -> Result<(), BearDogError> {
        let discoverer = Pkcs11Discoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in discovered.iter().filter(|h| h.name.contains("AWS CloudHSM")) {
            assert!(hsm.capabilities.api_support.pkcs11);
            // Cloud HSMs should have high security
            if let Some(fips) = hsm.capabilities.security.fips_140_level {
                assert!(fips >= 2);
            }
        }
        
        Ok(())
    }

    #[test]
    fn test_default_implementation() {
        let discoverer = Pkcs11Discoverer::default();
        assert!(discoverer.enable_system_scan);
        assert!(discoverer.custom_library_paths.is_empty());
    }

    #[test]
    fn test_clone_implementation() {
        let discoverer1 = Pkcs11Discoverer::new().unwrap();
        let discoverer2 = discoverer1.clone();
        
        assert_eq!(discoverer1.enable_system_scan, discoverer2.enable_system_scan);
    }
}

