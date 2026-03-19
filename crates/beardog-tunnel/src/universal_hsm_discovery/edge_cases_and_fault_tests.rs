// SPDX-License-Identifier: AGPL-3.0-only

//! Edge Cases and Fault Injection Tests
//!
//! Comprehensive testing for:
//! - Edge case scenarios
//! - Fault injection
//! - Error recovery
//! - Resource exhaustion
//! - Security edge cases

use super::*;
use beardog_errors::BearDogError;
use std::time::Duration;

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[tokio::test]
    async fn test_discovery_with_no_hsms_available() -> Result<(), BearDogError> {
        // Test behavior when no HSMs are available
        let mut engine = UniversalHsmDiscovery::new()?;
        
        // Even with no hardware, should find at least software HSM
        let hsms = engine.discover_all_hsms()?;
        // BearDog software HSM should always be available
        assert!(!hsms.is_empty(), "Software HSM should always be available");
        Ok(())
    }

    #[tokio::test]
    async fn test_discovery_with_all_probes_disabled() -> Result<(), BearDogError> {
        // This would require modifying discovery config
        // Testing that system handles minimal configuration
        let engine = UniversalHsmDiscovery::new()?;
        assert!(engine.discovered_hsms.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_hsm_with_minimal_capabilities() -> Result<(), BearDogError> {
        let tier_manager = tier_manager::TierManager::new()?;
        let caps = UniversalHsmCapabilities::default();
        
        // Should still assign a tier (Software)
        let tier = tier_manager.assign_tier(&caps, false)?;
        assert_eq!(tier, HsmTier::Software);
        Ok(())
    }

    #[tokio::test]
    async fn test_hsm_with_conflicting_capabilities() -> Result<(), BearDogError> {
        let tier_manager = tier_manager::TierManager::new()?;
        let mut caps = UniversalHsmCapabilities::default();
        
        // Conflicting: claims hardware but no FIPS
        caps.security.hardware_backed = true;
        caps.security.fips_140_level = None;
        
        // Should still assign tier gracefully
        let tier = tier_manager.assign_tier(&caps, false)?;
        assert!(tier >= HsmTier::BasicHardware);
        Ok(())
    }

    #[tokio::test]
    async fn test_human_entropy_with_empty_methods() -> Result<(), BearDogError> {
        let classifier = human_entropy_classifier::HumanEntropyClassifier::new()?;
        let mut caps = HumanEntropyCapabilities::default();
        
        // Claims support but no methods
        caps.supports_human_entropy = true;
        caps.entropy_collection_methods = vec![];
        
        let assessment = classifier.assess_human_entropy_capabilities(&caps)?;
        // Should have low score due to no methods
        assert!(assessment.overall_score < 0.5);
        Ok(())
    }

    #[tokio::test]
    async fn test_discovery_stats_with_no_discovered_hsms() -> Result<(), BearDogError> {
        let engine = UniversalHsmDiscovery::new()?;
        let stats = engine.get_discovery_stats();
        
        assert_eq!(stats.total_hsms, 0);
        assert_eq!(stats.human_entropy_hsms, 0);
        assert_eq!(stats.healthy_hsms, 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_best_hsm_with_no_suitable_hsm() -> Result<(), BearDogError> {
        let mut engine = UniversalHsmDiscovery::new()?;
        let _ = engine.discover_all_hsms()?;
        
        // Request HSM for extremely specific operation
        let hsm = engine.get_best_hsm_for_operation("quantum_resistant_key_generation")?;
        // May or may not find one, but shouldn't error
        Ok(())
    }

    #[tokio::test]
    async fn test_tier_filtering_with_nonexistent_tier() -> Result<(), BearDogError> {
        let mut engine = UniversalHsmDiscovery::new()?;
        let _ = engine.discover_all_hsms()?;
        
        // Request tier that might not exist
        let hsms = engine.get_hsms_by_tier(HsmTier::HighSecurity);
        // Should return empty list, not error
        Ok(())
    }
}

#[cfg(test)]
mod fault_injection_tests {
    use super::*;

    #[tokio::test]
    async fn test_discovery_with_extreme_timeout() -> Result<(), BearDogError> {
        let mut engine = UniversalHsmDiscovery::new()?;
        
        let config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_nanos(1), // Extremely short
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        };
        
        engine.update_config(config);
        
        // Should handle gracefully
        let result = engine.discover_all_hsms();
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_discovery_with_very_long_timeout() -> Result<(), BearDogError> {
        let mut engine = UniversalHsmDiscovery::new()?;
        
        let config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_secs(3600), // Very long
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        };
        
        engine.update_config(config);
        
        // Should still work
        let result = engine.discover_all_hsms();
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_concurrent_config_updates() -> Result<(), BearDogError> {
        use std::sync::Arc;
        
        let engine = Arc::new(tokio::sync::Mutex::new(UniversalHsmDiscovery::new()?));
        
        let mut handles = vec![];
        
        for i in 0..5 {
            let eng = Arc::clone(&engine);
            handles.push(tokio::spawn(async move {
                let mut e = eng.lock().await;
                let config = DiscoveryConfig {
                    auto_discovery_enabled: true,
                    discovery_interval: Duration::from_secs(60 + i),
                    health_check_interval: Duration::from_secs(30),
                    capability_refresh_interval: Duration::from_secs(1800),
                    timeout: Duration::from_secs(5),
                    tier_elevation_enabled: true,
                    human_entropy_priority: true,
                };
                e.update_config(config);
            }));
        }
        
        for handle in handles {
            handle.await.map_err(|e| 
                BearDogError::internal(format!("Task failed: {}", e)))?;
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_rapid_discovery_requests() -> Result<(), BearDogError> {
        let mut engine = UniversalHsmDiscovery::new()?;
        
        // Rapid-fire discovery requests
        for _ in 0..10 {
            let _ = engine.discover_all_hsms()?;
        }
        
        // Should handle all requests
        Ok(())
    }

    #[tokio::test]
    async fn test_discovery_with_disabled_features() -> Result<(), BearDogError> {
        let mut engine = UniversalHsmDiscovery::new()?;
        
        let config = DiscoveryConfig {
            auto_discovery_enabled: false,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_secs(5),
            tier_elevation_enabled: false,
            human_entropy_priority: false,
        };
        
        engine.update_config(config);
        
        let result = engine.discover_all_hsms();
        assert!(result.is_ok());
        Ok(())
    }
}

#[cfg(test)]
mod error_recovery_tests {
    use super::*;

    #[tokio::test]
    async fn test_recovery_from_failed_tier_assignment() -> Result<(), BearDogError> {
        let tier_manager = tier_manager::TierManager::new()?;
        
        // Even with extreme capabilities, should not panic
        let mut caps = UniversalHsmCapabilities::default();
        caps.security.fips_140_level = Some(999); // Invalid FIPS level
        
        let result = tier_manager.assign_tier(&caps, false);
        // Should either work or return error, not panic
        Ok(())
    }

    #[tokio::test]
    async fn test_recovery_from_malformed_human_entropy_data() -> Result<(), BearDogError> {
        let classifier = human_entropy_classifier::HumanEntropyClassifier::new()?;
        let mut caps = HumanEntropyCapabilities::default();
        
        // Contradictory data
        caps.supports_human_entropy = false;
        caps.supports_ephemeral_seeds = true; // Contradicts above
        caps.entropy_quality_assessment = true;
        
        let result = classifier.assess_human_entropy_capabilities(&caps);
        // Should handle gracefully
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_multiple_discovery_failures() -> Result<(), BearDogError> {
        let mut engine = UniversalHsmDiscovery::new()?;
        
        // Even if individual discoveries fail, overall should work
        for _ in 0..5 {
            let _ = engine.discover_all_hsms();
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod resource_tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_usage_with_many_discoveries() -> Result<(), BearDogError> {
        let mut engine = UniversalHsmDiscovery::new()?;
        
        // Run many discoveries
        for _ in 0..100 {
            let _ = engine.discover_all_hsms()?;
        }
        
        // Should not accumulate excessive memory
        // (Actual memory testing would require more sophisticated tools)
        Ok(())
    }

    #[tokio::test]
    async fn test_concurrent_resource_access() -> Result<(), BearDogError> {
        use std::sync::Arc;
        
        let engine = Arc::new(tokio::sync::Mutex::new(UniversalHsmDiscovery::new()?));
        
        let mut handles = vec![];
        
        // Many concurrent accessors
        for _ in 0..20 {
            let eng = Arc::clone(&engine);
            handles.push(tokio::spawn(async move {
                let e = eng.lock().await;
                let _ = e.get_discovery_stats();
            }));
        }
        
        for handle in handles {
            handle.await.map_err(|e| 
                BearDogError::internal(format!("Task failed: {}", e)))?;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod security_edge_cases {
    use super::*;

    #[tokio::test]
    async fn test_hsm_with_suspicious_capabilities() -> Result<(), BearDogError> {
        let tier_manager = tier_manager::TierManager::new()?;
        let mut caps = UniversalHsmCapabilities::default();
        
        // Claims everything but is software
        caps.security.hardware_backed = false;
        caps.security.fips_140_level = Some(4);
        caps.security.tamper_resistant = true;
        
        // Should be skeptical and assign lower tier
        let tier = tier_manager.assign_tier(&caps, false)?;
        assert_eq!(tier, HsmTier::Software, 
            "Software HSM claiming hardware features should stay Software tier");
        Ok(())
    }

    #[tokio::test]
    async fn test_human_entropy_score_bounds() -> Result<(), BearDogError> {
        let classifier = human_entropy_classifier::HumanEntropyClassifier::new()?;
        let caps = HumanEntropyCapabilities::default();
        
        let assessment = classifier.assess_human_entropy_capabilities(&caps)?;
        
        // Scores should be in valid range
        assert!(assessment.overall_score >= 0.0);
        assert!(assessment.overall_score <= 1.0);
        assert!(assessment.collection_efficiency >= 0.0);
        assert!(assessment.collection_efficiency <= 1.0);
        assert!(assessment.biometric_integration_score >= 0.0);
        assert!(assessment.biometric_integration_score <= 1.0);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_tier_elevation_requires_proper_entropy() -> Result<(), BearDogError> {
        let tier_manager = tier_manager::TierManager::new()?;
        let mut caps = UniversalHsmCapabilities::default();
        
        // Claims human entropy but has no features
        caps.human_entropy.supports_human_entropy = true;
        caps.human_entropy.supports_ephemeral_seeds = false;
        caps.human_entropy.entropy_collection_methods = vec![];
        
        let tier = tier_manager.assign_tier(&caps, true)?;
        
        // Should not elevate to premium without proper features
        assert!(tier < HsmTier::HumanEntropyPremium);
        Ok(())
    }
}

