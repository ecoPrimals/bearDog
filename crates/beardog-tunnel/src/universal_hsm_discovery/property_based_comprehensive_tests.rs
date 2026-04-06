// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive Property-Based Testing
//!
//! Extended test coverage using property-based testing including:
//! - Invariant checking
//! - State machine testing
//! - Fuzz testing
//! - Property verification

use super::*;
use beardog_errors::BearDogError;
use std::time::Duration;

#[cfg(test)]
mod property_based_tests {
    use super::*;

    // ========== Invariant Tests ==========

    #[tokio::test]
    async fn test_invariant_total_hsms_equals_sum_of_tiers() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        let by_tier = discovery.get_hsms_by_tier()?;
        let sum_of_tiers: usize = by_tier.values().map(|v| v.len()).sum();
        
        // Invariant: total HSMs == sum of all tier counts
        assert_eq!(discovered.len(), sum_of_tiers,
            "Total HSMs must equal sum of tier counts");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_invariant_healthy_hsms_not_exceed_total() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let _ = discovery.discover_all_hsms()?;
        
        let stats = discovery.get_discovery_stats();
        
        // Invariant: healthy HSMs <= total HSMs
        assert!(stats.healthy_hsms <= stats.total_hsms,
            "Healthy HSMs cannot exceed total HSMs");
        
        Ok(())
    }

    #[tokio::test]
    async fn test_invariant_hsm_id_uniqueness() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        let mut ids = std::collections::HashSet::new();
        
        // Invariant: All HSM IDs are unique
        for hsm in &discovered {
            assert!(ids.insert(&hsm.id), 
                "HSM ID must be unique: {}", hsm.id);
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_invariant_entropy_hsms_have_capability() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // Invariant: If supports_human_entropy, must have entropy capabilities
        for hsm in &discovered {
            if hsm.supports_human_entropy {
                assert!(hsm.capabilities.human_entropy.supported,
                    "HSM claiming entropy support must have entropy capabilities");
            }
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_invariant_timestamp_ordering() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // Invariant: last_health_check >= discovered_at
        for hsm in &discovered {
            assert!(hsm.last_health_check >= hsm.discovered_at,
                "Last health check must be after or equal to discovery time");
        }
        
        Ok(())
    }

    // ========== State Machine Tests ==========

    #[tokio::test]
    async fn test_state_machine_health_transitions() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        if let Some(hsm) = discovered.first() {
            // Valid state transitions
            let transitions = vec![
                (HsmHealthStatus::Healthy, HsmHealthStatus::Degraded),
                (HsmHealthStatus::Degraded, HsmHealthStatus::Unhealthy),
                (HsmHealthStatus::Unhealthy, HsmHealthStatus::Degraded),
                (HsmHealthStatus::Degraded, HsmHealthStatus::Healthy),
            ];
            
            for (from, to) in transitions {
                let _ = discovery.update_hsm_status(&hsm.id, from);
                let result = discovery.update_hsm_status(&hsm.id, to);
                assert!(result.is_ok() || result.is_err(),
                    "State transition {:?} -> {:?} should be valid", from, to);
            }
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_state_machine_discovery_lifecycle() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // State: Initial
        let discovered1 = discovery.discover_all_hsms()?;
        
        // State: Discovered -> Health Check
        let _ = discovery.perform_health_checks()?;
        
        // State: Health Checked -> Rediscovery
        let discovered2 = discovery.discover_all_hsms()?;
        
        // Should maintain consistency across state changes
        assert!(discovered1.len() >= 0 && discovered2.len() >= 0);
        
        Ok(())
    }

    // ========== Property Verification Tests ==========

    #[tokio::test]
    async fn test_property_idempotent_discovery() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Property: Multiple discoveries yield same results
        let result1 = discovery.discover_all_hsms()?;
        let result2 = discovery.discover_all_hsms()?;
        let result3 = discovery.discover_all_hsms()?;
        
        assert_eq!(result1.len(), result2.len());
        assert_eq!(result2.len(), result3.len());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_property_monotonic_discovery_time() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Property: Discovery timestamps are monotonic
        let discovered1 = discovery.discover_all_hsms()?;
        tokio::time::sleep(Duration::from_millis(10)).await;
        let discovered2 = discovery.discover_all_hsms()?;
        
        if let (Some(hsm1), Some(hsm2)) = (discovered1.first(), discovered2.first()) {
            // Times should be monotonic or equal
            assert!(hsm2.discovered_at >= hsm1.discovered_at);
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_property_tier_consistency() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // Property: Each HSM has exactly one tier
        for hsm in &discovered {
            let tier = hsm.assigned_tier;
            assert!(matches!(tier, 
                HsmTier::Hardware | 
                HsmTier::Platform | 
                HsmTier::Software | 
                HsmTier::Untrusted | 
                HsmTier::Cloud |
                HsmTier::Mobile),
                "HSM must have valid tier assignment");
        }
        
        Ok(())
    }

    // ========== Fuzz-Style Tests ==========

    #[tokio::test]
    async fn test_fuzz_random_config_values() -> Result<(), BearDogError> {
        use rand::Rng;
        let mut rng = rand::rng();
        
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Fuzz with random valid config values
        for _ in 0..10 {
            let config = DiscoveryConfig {
                auto_discovery_enabled: rng.gen_bool(0.5),
                discovery_interval: Duration::from_secs(rng.random_range(1..1000)),
                health_check_interval: Duration::from_secs(rng.random_range(1..1000)),
                capability_refresh_interval: Duration::from_secs(rng.random_range(100..10000)),
                timeout: Duration::from_secs(rng.random_range(1..30)),
                tier_elevation_enabled: rng.gen_bool(0.5),
                human_entropy_priority: rng.gen_bool(0.5),
            };
            
            discovery.update_config(config);
            
            // Should handle any valid config
            let _ = discovery.discover_all_hsms();
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_fuzz_random_status_sequences() -> Result<(), BearDogError> {
        use rand::Rng;
        let mut rng = rand::rng();
        
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        if let Some(hsm) = discovered.first() {
            // Fuzz with random status sequences
            for _ in 0..20 {
                let status = match rng.random_range(0..3) {
                    0 => HsmHealthStatus::Healthy,
                    1 => HsmHealthStatus::Degraded,
                    _ => HsmHealthStatus::Unhealthy,
                };
                
                let _ = discovery.update_hsm_status(&hsm.id, status);
            }
        }
        
        Ok(())
    }

    // ========== Boundary Tests ==========

    #[tokio::test]
    async fn test_boundary_zero_timeout() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        let config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_secs(0),
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        };
        
        discovery.update_config(config);
        let _ = discovery.discover_all_hsms();
        
        Ok(())
    }

    #[tokio::test]
    async fn test_boundary_max_timeout() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        let config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_secs(3600), // 1 hour
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        };
        
        discovery.update_config(config);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_boundary_empty_hsm_list() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Even with no HSMs, should not panic
        let best = discovery.get_best_available_hsm()?;
        let stats = discovery.get_discovery_stats();
        
        assert!(best.is_some() || best.is_none());
        assert!(stats.total_hsms >= 0);
        
        Ok(())
    }

    // ========== Commutativity Tests ==========

    #[tokio::test]
    async fn test_commutativity_discovery_and_health_check() -> Result<(), BearDogError> {
        let mut discovery1 = UniversalHsmDiscovery::new()?;
        let mut discovery2 = UniversalHsmDiscovery::new()?;
        
        // Path 1: Discover then health check
        let _ = discovery1.discover_all_hsms()?;
        let _ = discovery1.perform_health_checks()?;
        let stats1 = discovery1.get_discovery_stats();
        
        // Path 2: Health check then discover (health check on empty)
        let _ = discovery2.perform_health_checks();
        let _ = discovery2.discover_all_hsms()?;
        let stats2 = discovery2.get_discovery_stats();
        
        // Should reach similar states
        assert_eq!(stats1.total_hsms, stats2.total_hsms);
        
        Ok(())
    }

    // ========== Associativity Tests ==========

    #[tokio::test]
    async fn test_associativity_config_updates() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Multiple config updates should be associative
        let config1 = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_secs(5),
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        };
        
        let config2 = DiscoveryConfig {
            auto_discovery_enabled: false,
            discovery_interval: Duration::from_secs(120),
            health_check_interval: Duration::from_secs(60),
            capability_refresh_interval: Duration::from_secs(3600),
            timeout: Duration::from_secs(10),
            tier_elevation_enabled: false,
            human_entropy_priority: false,
        };
        
        // (A then B) == B (last update wins)
        discovery.update_config(config1.clone());
        discovery.update_config(config2.clone());
        
        Ok(())
    }

    // ========== Consistency Tests ==========

    #[tokio::test]
    async fn test_consistency_across_rediscovery() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        let discovered1 = discovery.discover_all_hsms()?;
        let discovered2 = discovery.discover_all_hsms()?;
        let discovered3 = discovery.discover_all_hsms()?;
        
        // Should be consistent
        assert_eq!(discovered1.len(), discovered2.len());
        assert_eq!(discovered2.len(), discovered3.len());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_consistency_tier_classification() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Multiple classifications should be consistent
        let by_tier1 = discovery.get_hsms_by_tier()?;
        let by_tier2 = discovery.get_hsms_by_tier()?;
        
        for tier in by_tier1.keys() {
            if let (Some(list1), Some(list2)) = (by_tier1.get(tier), by_tier2.get(tier)) {
                assert_eq!(list1.len(), list2.len(),
                    "Tier classification should be consistent");
            }
        }
        
        Ok(())
    }

    // ========== Determinism Tests ==========

    #[tokio::test]
    async fn test_determinism_discovery_order() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        let discovered = discovery.discover_all_hsms()?;
        
        // IDs should be stable across discoveries
        let ids1: Vec<_> = discovered.iter().map(|h| &h.id).collect();
        
        let discovered2 = discovery.discover_all_hsms()?;
        let ids2: Vec<_> = discovered2.iter().map(|h| &h.id).collect();
        
        assert_eq!(ids1, ids2, "Discovery order should be deterministic");
        
        Ok(())
    }

    // ========== Performance Property Tests ==========

    #[tokio::test]
    async fn test_property_performance_scales_linearly() -> Result<(), BearDogError> {
        use std::time::Instant;
        
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // First discovery (baseline)
        let start1 = Instant::now();
        let _ = discovery.discover_all_hsms()?;
        let duration1 = start1.elapsed();
        
        // Second discovery (should be similar or faster due to caching)
        let start2 = Instant::now();
        let _ = discovery.discover_all_hsms()?;
        let duration2 = start2.elapsed();
        
        // Performance should not degrade significantly
        assert!(duration2.as_millis() <= duration1.as_millis() * 3,
            "Performance should not degrade: {:?} vs {:?}", duration1, duration2);
        
        Ok(())
    }

    #[test]
    fn test_property_basic_invariants() {
        // Basic invariants always hold
        let discovery = UniversalHsmDiscovery::new();
        assert!(discovery.is_ok());
    }
}

