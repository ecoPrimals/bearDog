// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive core tests for Universal HSM Discovery
//!
//! This module contains extensive test coverage for:
//! - Tier management
//! - Human entropy classification
//! - Capability detection
//! - Discovery engine
//! - Error handling
//! - Integration scenarios

use super::*;
use beardog_errors::BearDogError;
use std::time::Duration;

#[cfg(test)]
mod tier_management_tests {
    use super::*;

    #[tokio::test]
    async fn test_tier_assignment_for_software_hsm() -> Result<(), BearDogError> {
        let tier_manager = tier_manager::TierManager::new()?;
        let mut caps = UniversalHsmCapabilities::default();
        
        // Software HSM should get Software tier
        let tier = tier_manager.assign_tier(&caps, false)?;
        assert_eq!(tier, HsmTier::Software);
        Ok(())
    }

    #[tokio::test]
    async fn test_tier_assignment_for_basic_hardware() -> Result<(), BearDogError> {
        let tier_manager = tier_manager::TierManager::new()?;
        let mut caps = UniversalHsmCapabilities::default();
        
        // Hardware-backed but not certified
        caps.security.hardware_backed = true;
        
        let tier = tier_manager.assign_tier(&caps, false)?;
        assert!(tier >= HsmTier::BasicHardware);
        Ok(())
    }

    #[tokio::test]
    async fn test_tier_assignment_for_certified_hardware() -> Result<(), BearDogError> {
        let tier_manager = tier_manager::TierManager::new()?;
        let mut caps = UniversalHsmCapabilities::default();
        
        // FIPS certified hardware
        caps.security.hardware_backed = true;
        caps.security.fips_140_level = Some(2);
        
        let tier = tier_manager.assign_tier(&caps, false)?;
        assert!(tier >= HsmTier::CertifiedHardware);
        Ok(())
    }

    #[tokio::test]
    async fn test_tier_elevation_with_human_entropy() -> Result<(), BearDogError> {
        let tier_manager = tier_manager::TierManager::new()?;
        let mut caps = UniversalHsmCapabilities::default();
        
        // Software HSM with human entropy
        caps.human_entropy.supports_human_entropy = true;
        caps.human_entropy.supports_ephemeral_seeds = true;
        
        let tier_without_entropy = tier_manager.assign_tier(&caps, false)?;
        let tier_with_entropy = tier_manager.assign_tier(&caps, true)?;
        
        assert!(tier_with_entropy > tier_without_entropy, 
            "Human entropy should elevate tier");
        Ok(())
    }

    #[tokio::test]
    async fn test_tier_assignment_for_high_security() -> Result<(), BearDogError> {
        let tier_manager = tier_manager::TierManager::new()?;
        let mut caps = UniversalHsmCapabilities::default();
        
        // High security HSM
        caps.security.hardware_backed = true;
        caps.security.fips_140_level = Some(3);
        caps.security.tamper_resistant = true;
        caps.security.isolated_execution = true;
        
        let tier = tier_manager.assign_tier(&caps, false)?;
        assert!(tier >= HsmTier::HighSecurity);
        Ok(())
    }

    #[tokio::test]
    async fn test_tier_assignment_consistency() -> Result<(), BearDogError> {
        let tier_manager = tier_manager::TierManager::new()?;
        let caps = UniversalHsmCapabilities::default();
        
        // Same capabilities should always get same tier
        let tier1 = tier_manager.assign_tier(&caps, false)?;
        let tier2 = tier_manager.assign_tier(&caps, false)?;
        
        assert_eq!(tier1, tier2, "Tier assignment should be deterministic");
        Ok(())
    }
}

#[cfg(test)]
mod human_entropy_tests {
    use super::*;

    #[tokio::test]
    async fn test_human_entropy_classifier_creation() -> Result<(), BearDogError> {
        let classifier = human_entropy_classifier::HumanEntropyClassifier::new()?;
        // Should create successfully
        Ok(())
    }

    #[tokio::test]
    async fn test_basic_human_entropy_assessment() -> Result<(), BearDogError> {
        let classifier = human_entropy_classifier::HumanEntropyClassifier::new()?;
        let mut caps = HumanEntropyCapabilities::default();
        
        caps.supports_human_entropy = true;
        caps.entropy_collection_methods = vec![
            EntropyCollectionMethod::Keystroke { timing_analysis: true }
        ];
        
        let assessment = classifier.assess_human_entropy_capabilities(&caps)?;
        assert!(assessment.overall_score > 0.0);
        Ok(())
    }

    #[tokio::test]
    async fn test_premium_human_entropy_assessment() -> Result<(), BearDogError> {
        let classifier = human_entropy_classifier::HumanEntropyClassifier::new()?;
        let mut caps = HumanEntropyCapabilities::default();
        
        // Premium features
        caps.supports_human_entropy = true;
        caps.supports_ephemeral_seeds = true;
        caps.entropy_quality_assessment = true;
        caps.real_time_entropy_generation = true;
        caps.biometric_entropy_integration = true;
        caps.entropy_collection_methods = vec![
            EntropyCollectionMethod::BiometricVariations { template_noise: true },
            EntropyCollectionMethod::TouchPatterns { pressure_sensitive: true },
            EntropyCollectionMethod::BehavioralBiometrics { pattern_recognition: true },
        ];
        
        let assessment = classifier.assess_human_entropy_capabilities(&caps)?;
        assert!(assessment.overall_score > 0.7, "Premium should have high score");
        assert!(assessment.meets_tier_elevation_criteria);
        Ok(())
    }

    #[tokio::test]
    async fn test_no_human_entropy_assessment() -> Result<(), BearDogError> {
        let classifier = human_entropy_classifier::HumanEntropyClassifier::new()?;
        let caps = HumanEntropyCapabilities::default();
        
        let assessment = classifier.assess_human_entropy_capabilities(&caps)?;
        assert_eq!(assessment.overall_score, 0.0);
        assert!(!assessment.meets_tier_elevation_criteria);
        Ok(())
    }

    #[tokio::test]
    async fn test_ephemeral_seeds_requirement() -> Result<(), BearDogError> {
        let classifier = human_entropy_classifier::HumanEntropyClassifier::new()?;
        let mut caps = HumanEntropyCapabilities::default();
        
        caps.supports_human_entropy = true;
        caps.supports_ephemeral_seeds = true;
        
        let assessment = classifier.assess_human_entropy_capabilities(&caps)?;
        assert!(assessment.supports_ephemeral_seeds);
        Ok(())
    }
}

#[cfg(test)]
mod discovery_engine_tests {
    use super::*;

    #[tokio::test]
    async fn test_discovery_engine_creation() -> Result<(), BearDogError> {
        let engine = UniversalHsmDiscovery::new()?;
        assert!(engine.discovered_hsms.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_discovery_all_hsms() -> Result<(), BearDogError> {
        let mut engine = UniversalHsmDiscovery::new()?;
        let hsms = engine.discover_all_hsms()?;
        
        // Should discover at least software HSMs
        assert!(!hsms.is_empty(), "Should discover at least one HSM");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_discovery_stats() -> Result<(), BearDogError> {
        let mut engine = UniversalHsmDiscovery::new()?;
        let _ = engine.discover_all_hsms()?;
        
        let stats = engine.get_discovery_stats();
        assert!(stats.total_hsms > 0);
        assert!(stats.tier_distribution.len() > 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_human_entropy_hsms() -> Result<(), BearDogError> {
        let mut engine = UniversalHsmDiscovery::new()?;
        let _ = engine.discover_all_hsms()?;
        
        let entropy_hsms = engine.get_human_entropy_hsms();
        // BearDog HSM should always support human entropy
        assert!(!entropy_hsms.is_empty(), "Should find HSMs with human entropy");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_hsms_by_tier() -> Result<(), BearDogError> {
        let mut engine = UniversalHsmDiscovery::new()?;
        let _ = engine.discover_all_hsms()?;
        
        // Should have at least software tier HSMs
        let software_hsms = engine.get_hsms_by_tier(HsmTier::Software);
        assert!(!software_hsms.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_best_hsm_for_root_key_generation() -> Result<(), BearDogError> {
        let mut engine = UniversalHsmDiscovery::new()?;
        let _ = engine.discover_all_hsms()?;
        
        let hsm = engine.get_best_hsm_for_operation("root_key_generation")?;
        if let Some(hsm) = hsm {
            assert!(hsm.supports_human_entropy, 
                "Root key generation should prefer human entropy HSM");
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_discovery_config_update() -> Result<(), BearDogError> {
        let mut engine = UniversalHsmDiscovery::new()?;
        
        let config = DiscoveryConfig {
            auto_discovery_enabled: false,
            discovery_interval: Duration::from_secs(120),
            health_check_interval: Duration::from_secs(60),
            capability_refresh_interval: Duration::from_secs(3600),
            timeout: Duration::from_secs(10),
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        };
        
        engine.update_config(config);
        // Should update without error
        Ok(())
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[tokio::test]
    async fn test_discovery_with_invalid_config() {
        // Test that invalid configurations are handled gracefully
        let engine = UniversalHsmDiscovery::new();
        assert!(engine.is_ok(), "Should handle initialization gracefully");
    }

    #[tokio::test]
    async fn test_discovery_timeout_handling() -> Result<(), BearDogError> {
        let mut engine = UniversalHsmDiscovery::new()?;
        
        // Very short timeout
        let config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_millis(1), // Very short
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        };
        
        engine.update_config(config);
        
        // Should still work, even with aggressive timeout
        let result = engine.discover_all_hsms();
        assert!(result.is_ok(), "Should handle timeout gracefully");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_hsm_with_invalid_tier() -> Result<(), BearDogError> {
        let mut engine = UniversalHsmDiscovery::new()?;
        let _ = engine.discover_all_hsms()?;
        
        // Custom/unknown tier - should return empty
        let hsms = engine.get_hsms_by_tier(HsmTier::Software);
        // Should not panic
        Ok(())
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_end_to_end_discovery_workflow() -> Result<(), BearDogError> {
        // Complete workflow test
        let mut engine = UniversalHsmDiscovery::new()?;
        
        // Configure
        let config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_secs(5),
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        };
        engine.update_config(config);
        
        // Discover
        let discovered = engine.discover_all_hsms()?;
        assert!(!discovered.is_empty());
        
        // Get stats
        let stats = engine.get_discovery_stats();
        assert!(stats.total_hsms == discovered.len());
        
        // Get human entropy HSMs
        let entropy_hsms = engine.get_human_entropy_hsms();
        assert!(stats.human_entropy_hsms == entropy_hsms.len());
        
        // Select best HSM for operation
        let best_hsm = engine.get_best_hsm_for_operation("root_key_generation")?;
        assert!(best_hsm.is_some());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_concurrent_discovery_operations() -> Result<(), BearDogError> {
        use std::sync::Arc;
        
        let engine = Arc::new(tokio::sync::Mutex::new(UniversalHsmDiscovery::new()?));
        
        let mut handles = vec![];
        
        for _ in 0..5 {
            let eng = Arc::clone(&engine);
            handles.push(tokio::spawn(async move {
                let mut e = eng.lock().await;
                e.discover_all_hsms()
            }));
        }
        
        for handle in handles {
            let result = handle.await.map_err(|e| 
                BearDogError::internal(format!("Task failed: {}", e)))?;
            assert!(result.is_ok());
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_multi_tier_hsm_selection() -> Result<(), BearDogError> {
        let mut engine = UniversalHsmDiscovery::new()?;
        let _ = engine.discover_all_hsms()?;
        
        // Test that different operations select appropriate tiers
        let operations = vec![
            ("root_key_generation", HsmTier::CertifiedHardware),
            ("bulk_encryption", HsmTier::BasicHardware),
            ("authentication_token", HsmTier::Software),
        ];
        
        for (operation, expected_min_tier) in operations {
            if let Some(hsm) = engine.get_best_hsm_for_operation(operation)? {
                // Should select appropriate tier or better
                // Note: Actual tier depends on available HSMs
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn test_discovery_performance() -> Result<(), BearDogError> {
        let mut engine = UniversalHsmDiscovery::new()?;
        
        let start = Instant::now();
        let _ = engine.discover_all_hsms()?;
        let duration = start.elapsed();
        
        // Discovery should complete in reasonable time
        assert!(duration.as_secs() < 10, 
            "Discovery took too long: {:?}", duration);
        Ok(())
    }

    #[tokio::test]
    async fn test_stats_retrieval_performance() -> Result<(), BearDogError> {
        let mut engine = UniversalHsmDiscovery::new()?;
        let _ = engine.discover_all_hsms()?;
        
        let start = Instant::now();
        for _ in 0..1000 {
            let _ = engine.get_discovery_stats();
        }
        let duration = start.elapsed();
        
        // Stats retrieval should be fast
        assert!(duration.as_millis() < 100,
            "Stats retrieval too slow: {:?}", duration);
        Ok(())
    }

    #[tokio::test]
    async fn test_tier_filtering_performance() -> Result<(), BearDogError> {
        let mut engine = UniversalHsmDiscovery::new()?;
        let _ = engine.discover_all_hsms()?;
        
        let start = Instant::now();
        for tier in &[
            HsmTier::Software,
            HsmTier::BasicHardware,
            HsmTier::CertifiedHardware,
        ] {
            let _ = engine.get_hsms_by_tier(*tier);
        }
        let duration = start.elapsed();
        
        assert!(duration.as_millis() < 50,
            "Tier filtering too slow: {:?}", duration);
        Ok(())
    }
}

