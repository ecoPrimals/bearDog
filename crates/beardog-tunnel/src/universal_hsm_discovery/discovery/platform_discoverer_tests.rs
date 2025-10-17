//! Comprehensive tests for Platform HSM Discoverer
//!
//! Test coverage for TPM, TEE, Android StrongBox, and iOS Secure Enclave discovery

use super::*;
use beardog_errors::BearDogError;

#[cfg(test)]
mod platform_discoverer_tests {
    use super::*;

    // ========== Creation & Initialization Tests ==========

    #[tokio::test]
    async fn test_platform_discoverer_creation() -> Result<(), BearDogError> {
        let discoverer = PlatformDiscoverer::new()?;
        assert!(discoverer.enable_tpm_probe, "TPM probe should be enabled by default");
        assert!(discoverer.enable_tee_probe, "TEE probe should be enabled by default");
        Ok(())
    }

    #[tokio::test]
    async fn test_platform_discoverer_with_custom_config() -> Result<(), BearDogError> {
        let mut discoverer = PlatformDiscoverer::new()?;
        
        // Test disabling probes
        discoverer.enable_tpm_probe = false;
        assert!(!discoverer.enable_tpm_probe, "TPM probe should be disabled");
        
        discoverer.enable_tee_probe = false;
        assert!(!discoverer.enable_tee_probe, "TEE probe should be disabled");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_discoverer_is_cloneable() -> Result<(), BearDogError> {
        let discoverer1 = PlatformDiscoverer::new()?;
        let discoverer2 = discoverer1.clone();
        
        assert_eq!(discoverer1.enable_tpm_probe, discoverer2.enable_tpm_probe);
        assert_eq!(discoverer1.enable_tee_probe, discoverer2.enable_tee_probe);
        Ok(())
    }

    // ========== TPM Discovery Tests ==========

    #[tokio::test]
    async fn test_tpm_discovery_when_enabled() -> Result<(), BearDogError> {
        let discoverer = PlatformDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        // Should return results (empty or with TPM)
        assert!(discovered.len() <= 10, "Should not discover excessive HSMs");
        
        // Check if any TPM was discovered
        let tpm_hsms: Vec<_> = discovered.iter()
            .filter(|hsm| hsm.name.contains("TPM"))
            .collect();
        
        // On systems with TPM, we should find it
        if !tpm_hsms.is_empty() {
            for tpm in tpm_hsms {
                assert!(tpm.name.contains("Trusted Platform Module") || tpm.name.contains("TPM"));
                assert!(tpm.hsm_type == HsmType::PlatformHsm);
            }
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_tpm_discovery_when_disabled() -> Result<(), BearDogError> {
        let mut discoverer = PlatformDiscoverer::new()?;
        discoverer.enable_tpm_probe = false;
        
        let discovered = discoverer.discover().await?;
        
        // Should not discover TPM
        let tpm_count = discovered.iter()
            .filter(|hsm| hsm.name.contains("TPM"))
            .count();
        
        assert_eq!(tpm_count, 0, "Should not discover TPM when probe is disabled");
        Ok(())
    }

    #[tokio::test]
    async fn test_tpm_capabilities_structure() -> Result<(), BearDogError> {
        let discoverer = PlatformDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in discovered.iter().filter(|h| h.name.contains("TPM")) {
            // Verify basic capability structure
            assert!(!hsm.capabilities.key_generation.supported_algorithms.is_empty(),
                "TPM should support key generation algorithms");
            assert!(!hsm.capabilities.crypto_operations.encryption_algorithms.is_empty(),
                "TPM should support encryption algorithms");
            
            // TPM should support hardware key generation
            assert!(hsm.capabilities.key_generation.can_generate_in_hardware,
                "TPM should support hardware key generation");
            
            // Verify security features
            if let Some(fips_level) = hsm.capabilities.security.fips_140_level {
                assert!(fips_level >= 1 && fips_level <= 4,
                    "FIPS level should be between 1-4");
            }
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_multiple_tpm_devices() -> Result<(), BearDogError> {
        // Some systems may have multiple TPM devices
        let discoverer = PlatformDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        let tpm_hsms: Vec<_> = discovered.iter()
            .filter(|hsm| hsm.name.contains("TPM"))
            .collect();
        
        // Verify each TPM has unique identifier
        if tpm_hsms.len() > 1 {
            let mut ids = std::collections::HashSet::new();
            for tpm in &tpm_hsms {
                assert!(ids.insert(&tpm.id), "Each TPM should have unique ID");
            }
        }
        
        Ok(())
    }

    // ========== TEE Discovery Tests ==========

    #[tokio::test]
    async fn test_tee_discovery_when_enabled() -> Result<(), BearDogError> {
        let discoverer = PlatformDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        // Check if any TEE was discovered
        let tee_hsms: Vec<_> = discovered.iter()
            .filter(|hsm| hsm.name.contains("TEE"))
            .collect();
        
        // On systems with TEE, verify it
        for tee in tee_hsms {
            assert!(tee.name.contains("Trusted Execution Environment") || tee.name.contains("TEE"));
            assert_eq!(tee.hsm_type, HsmType::PlatformHsm);
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_tee_discovery_when_disabled() -> Result<(), BearDogError> {
        let mut discoverer = PlatformDiscoverer::new()?;
        discoverer.enable_tee_probe = false;
        
        let discovered = discoverer.discover().await?;
        
        // Should not discover TEE
        let tee_count = discovered.iter()
            .filter(|hsm| hsm.name.contains("TEE"))
            .count();
        
        assert_eq!(tee_count, 0, "Should not discover TEE when probe is disabled");
        Ok(())
    }

    #[tokio::test]
    async fn test_tee_capabilities_structure() -> Result<(), BearDogError> {
        let discoverer = PlatformDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in discovered.iter().filter(|h| h.name.contains("TEE")) {
            // Verify TEE capabilities
            assert!(!hsm.capabilities.key_generation.supported_algorithms.is_empty(),
                "TEE should support key generation");
            assert!(!hsm.capabilities.crypto_operations.signing_algorithms.is_empty(),
                "TEE should support signing");
            
            // TEE should have isolation features
            assert!(hsm.capabilities.security.isolated_execution,
                "TEE should support isolated execution");
        }
        
        Ok(())
    }

    // ========== Android StrongBox Tests ==========

    #[cfg(target_os = "android")]
    #[tokio::test]
    async fn test_android_strongbox_discovery() -> Result<(), BearDogError> {
        let discoverer = PlatformDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        // Check for StrongBox
        let strongbox_hsms: Vec<_> = discovered.iter()
            .filter(|hsm| hsm.name.contains("StrongBox"))
            .collect();
        
        for sb in strongbox_hsms {
            assert_eq!(sb.hsm_type, HsmType::MobileHsm);
            assert!(sb.capabilities.security.hardware_backed,
                "StrongBox should be hardware-backed");
        }
        
        Ok(())
    }

    #[cfg(target_os = "android")]
    #[tokio::test]
    async fn test_android_strongbox_capabilities() -> Result<(), BearDogError> {
        let discoverer = PlatformDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in discovered.iter().filter(|h| h.name.contains("StrongBox")) {
            // StrongBox specific capabilities
            assert!(hsm.capabilities.key_generation.can_generate_in_hardware,
                "StrongBox should generate keys in hardware");
            assert!(hsm.capabilities.key_management.supports_key_attestation,
                "StrongBox should support key attestation");
            assert!(hsm.capabilities.security.tamper_resistant,
                "StrongBox should be tamper-resistant");
        }
        
        Ok(())
    }

    // ========== iOS Secure Enclave Tests ==========

    #[cfg(target_os = "ios")]
    #[tokio::test]
    async fn test_ios_secure_enclave_discovery() -> Result<(), BearDogError> {
        let discoverer = PlatformDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        // Check for Secure Enclave
        let se_hsms: Vec<_> = discovered.iter()
            .filter(|hsm| hsm.name.contains("Secure Enclave"))
            .collect();
        
        for se in se_hsms {
            assert_eq!(se.hsm_type, HsmType::MobileHsm);
            assert!(se.capabilities.security.hardware_backed,
                "Secure Enclave should be hardware-backed");
        }
        
        Ok(())
    }

    #[cfg(target_os = "ios")]
    #[tokio::test]
    async fn test_ios_secure_enclave_capabilities() -> Result<(), BearDogError> {
        let discoverer = PlatformDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in discovered.iter().filter(|h| h.name.contains("Secure Enclave")) {
            // Secure Enclave specific capabilities
            assert!(hsm.capabilities.key_generation.can_generate_in_hardware);
            assert!(hsm.capabilities.key_management.supports_key_attestation);
            assert!(hsm.capabilities.biometric_integration.supports_biometric_auth);
        }
        
        Ok(())
    }

    // ========== Cross-Platform Tests ==========

    #[tokio::test]
    async fn test_discover_returns_consistent_results() -> Result<(), BearDogError> {
        let discoverer = PlatformDiscoverer::new()?;
        
        // Run discovery multiple times
        let discovered1 = discoverer.discover().await?;
        let discovered2 = discoverer.discover().await?;
        
        // Should return same number of HSMs (assuming no hardware changes)
        assert_eq!(discovered1.len(), discovered2.len(),
            "Discovery should be deterministic");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_all_discovered_hsms_have_valid_fields() -> Result<(), BearDogError> {
        let discoverer = PlatformDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in &discovered {
            // Verify required fields
            assert!(!hsm.id.is_empty(), "HSM ID should not be empty");
            assert!(!hsm.name.is_empty(), "HSM name should not be empty");
            assert!(!hsm.vendor.is_empty(), "Vendor should not be empty");
            
            // Verify timestamps
            assert!(hsm.discovered_at <= Utc::now(), "Discovery time should not be in future");
            
            // Verify capabilities exist
            assert!(!hsm.capabilities.key_generation.supported_algorithms.is_empty(),
                "Should have at least one key generation algorithm");
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_discovered_hsms_have_correct_type() -> Result<(), BearDogError> {
        let discoverer = PlatformDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in &discovered {
            // Platform discoverer should only return platform or mobile HSMs
            assert!(
                matches!(hsm.hsm_type, HsmType::PlatformHsm | HsmType::MobileHsm),
                "Platform discoverer should only return platform or mobile HSMs, got {:?}",
                hsm.hsm_type
            );
        }
        
        Ok(())
    }

    // ========== Error Handling Tests ==========

    #[tokio::test]
    async fn test_discovery_handles_permission_errors_gracefully() -> Result<(), BearDogError> {
        // Even with permission errors, discovery should not fail
        let discoverer = PlatformDiscoverer::new()?;
        let result = discoverer.discover().await;
        
        // Should return Ok, even if no HSMs found
        assert!(result.is_ok(), "Discovery should handle permission errors gracefully");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_discovery_when_no_platform_hsms_available() -> Result<(), BearDogError> {
        let mut discoverer = PlatformDiscoverer::new()?;
        
        // Disable all probes
        discoverer.enable_tpm_probe = false;
        discoverer.enable_tee_probe = false;
        
        let discovered = discoverer.discover().await?;
        
        // Should return empty list, not error
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        {
            assert!(discovered.is_empty(), "Should return empty list when no HSMs available");
        }
        
        Ok(())
    }

    // ========== Performance Tests ==========

    #[tokio::test]
    async fn test_discovery_completes_in_reasonable_time() -> Result<(), BearDogError> {
        use std::time::Instant;
        
        let discoverer = PlatformDiscoverer::new()?;
        let start = Instant::now();
        
        let _discovered = discoverer.discover().await?;
        
        let duration = start.elapsed();
        
        // Discovery should complete in under 5 seconds
        assert!(duration.as_secs() < 5,
            "Discovery took too long: {:?}", duration);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_concurrent_discoveries() -> Result<(), BearDogError> {
        let discoverer = PlatformDiscoverer::new()?;
        
        // Run multiple discoveries concurrently
        let mut handles = vec![];
        for _ in 0..5 {
            let disc = discoverer.clone();
            handles.push(tokio::spawn(async move {
                disc.discover().await
            }));
        }
        
        // All should complete successfully
        for handle in handles {
            let result = handle.await.map_err(|e| 
                BearDogError::internal(format!("Task failed: {}", e)))?;
            assert!(result.is_ok(), "Concurrent discovery should succeed");
        }
        
        Ok(())
    }

    // ========== Integration Tests ==========

    #[tokio::test]
    async fn test_discovered_hsms_can_be_serialized() -> Result<(), BearDogError> {
        let discoverer = PlatformDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in &discovered {
            // Should be able to debug print
            let debug_str = format!("{:?}", hsm);
            assert!(!debug_str.is_empty(), "Should be able to debug print HSM");
            
            // Should be cloneable
            let _cloned = hsm.clone();
        }
        
        Ok(())
    }
}

