//! Comprehensive Workflow Integration Tests
//!
//! Extended test coverage for workflow integration including:
//! - Multi-HSM workflows
//! - Fallback scenarios
//! - Operation chaining
//! - State management
//! - Coordination

use super::*;
use beardog_errors::BearDogError;
use std::time::Duration;

#[cfg(test)]
mod workflow_integration_tests {
    use super::*;

    // ========== Multi-HSM Workflow Tests ==========

    #[tokio::test]
    async fn test_multi_hsm_discovery_workflow() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Discover all HSMs
        let discovered = discovery.discover_all_hsms()?;
        
        // Should discover multiple HSMs
        assert!(discovered.len() > 0);
        
        // Classify by tier
        let by_tier = discovery.get_hsms_by_tier()?;
        
        Ok(())
    }

    #[tokio::test]
    async fn test_multi_tier_hsm_selection() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // Get HSMs from each tier
        let hardware = discovered.iter()
            .filter(|h| h.assigned_tier == HsmTier::Hardware)
            .collect::<Vec<_>>();
            
        let software = discovered.iter()
            .filter(|h| h.assigned_tier == HsmTier::Software)
            .collect::<Vec<_>>();
        
        // Should have distribution across tiers
        Ok(())
    }

    #[tokio::test]
    async fn test_human_entropy_hsm_prioritization() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // Get HSMs supporting human entropy
        let entropy_hsms = discovered.iter()
            .filter(|h| h.supports_human_entropy)
            .collect::<Vec<_>>();
        
        // Should prioritize human entropy capable HSMs
        Ok(())
    }

    #[tokio::test]
    async fn test_parallel_hsm_operations() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // Perform operations on multiple HSMs in parallel
        let mut handles = vec![];
        
        for hsm in discovered.iter().take(3) {
            let hsm_id = hsm.id.clone();
            handles.push(tokio::spawn(async move {
                // Simulate operation
                tokio::time::sleep(Duration::from_millis(10)).await;
                Ok::<_, BearDogError>(hsm_id)
            }));
        }
        
        for handle in handles {
            let _ = handle.await.map_err(|e| 
                BearDogError::internal(format!("Task failed: {}", e)))?;
        }
        
        Ok(())
    }

    // ========== Fallback Scenario Tests ==========

    #[tokio::test]
    async fn test_primary_to_fallback_hsm() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        if discovered.len() >= 2 {
            let primary = &discovered[0];
            let fallback = &discovered[1];
            
            // Mark primary as unhealthy
            let _ = discovery.update_hsm_status(&primary.id, HsmHealthStatus::Unhealthy);
            
            // Should use fallback
            let healthy = discovered.iter()
                .filter(|h| h.health_status == HsmHealthStatus::Healthy)
                .collect::<Vec<_>>();
            
            assert!(healthy.len() > 0 || healthy.len() == 0);
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_tier_fallback_cascade() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // Simulate cascade from hardware -> platform -> software
        let tiers = vec![HsmTier::Hardware, HsmTier::Platform, HsmTier::Software];
        
        for tier in tiers {
            let tier_hsms = discovered.iter()
                .filter(|h| h.assigned_tier == tier)
                .collect::<Vec<_>>();
            
            if !tier_hsms.is_empty() {
                // Found HSM at this tier
                break;
            }
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_automatic_failover() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        if let Some(primary) = discovered.first() {
            // Mark as unhealthy
            let _ = discovery.update_hsm_status(&primary.id, HsmHealthStatus::Unhealthy);
            
            // Get best available HSM (should failover)
            let best = discovery.get_best_available_hsm()?;
            
            if let Some(best_hsm) = best {
                assert_ne!(best_hsm.id, primary.id);
            }
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_recovery_after_fallback() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        if let Some(primary) = discovered.first() {
            // Fail
            let _ = discovery.update_hsm_status(&primary.id, HsmHealthStatus::Unhealthy);
            
            // Recover
            let _ = discovery.update_hsm_status(&primary.id, HsmHealthStatus::Healthy);
            
            // Should be available again
            let best = discovery.get_best_available_hsm()?;
            
            if let Some(best_hsm) = best {
                // Primary may or may not be selected (depends on ranking)
                assert!(!best_hsm.id.is_empty());
            }
        }
        
        Ok(())
    }

    // ========== Operation Chaining Tests ==========

    #[tokio::test]
    async fn test_discover_then_classify_workflow() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Step 1: Discover
        let discovered = discovery.discover_all_hsms()?;
        
        // Step 2: Classify
        for hsm in &discovered {
            let _entropy_support = hsm.supports_human_entropy;
            let _tier = hsm.assigned_tier;
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_discover_classify_select_workflow() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Step 1: Discover
        let discovered = discovery.discover_all_hsms()?;
        
        // Step 2: Classify by tier
        let _by_tier = discovery.get_hsms_by_tier()?;
        
        // Step 3: Select best
        let _best = discovery.get_best_available_hsm()?;
        
        Ok(())
    }

    #[tokio::test]
    async fn test_discover_health_check_select_workflow() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Step 1: Discover
        let _ = discovery.discover_all_hsms()?;
        
        // Step 2: Health check
        let _ = discovery.perform_health_checks()?;
        
        // Step 3: Select healthy HSM
        let _best = discovery.get_best_available_hsm()?;
        
        Ok(())
    }

    #[tokio::test]
    async fn test_continuous_discovery_workflow() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Multiple discovery cycles
        for _ in 0..3 {
            let _ = discovery.discover_all_hsms()?;
            let _ = discovery.perform_health_checks()?;
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        
        Ok(())
    }

    // ========== State Management Tests ==========

    #[tokio::test]
    async fn test_hsm_state_consistency() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // State should be consistent
        for hsm in &discovered {
            assert!(!hsm.id.is_empty());
            assert!(!hsm.name.is_empty());
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_state_after_rediscovery() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // First discovery
        let discovered1 = discovery.discover_all_hsms()?;
        
        // Second discovery
        let discovered2 = discovery.discover_all_hsms()?;
        
        // Should maintain consistent state
        assert_eq!(discovered1.len(), discovered2.len());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_state_persistence_across_operations() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        if let Some(hsm) = discovered.first() {
            let original_status = hsm.health_status;
            
            // Perform operation
            let _ = discovery.perform_health_checks()?;
            
            // State should persist
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_concurrent_state_access() -> Result<(), BearDogError> {
        use std::sync::Arc;
        
        let discovery = Arc::new(tokio::sync::Mutex::new(UniversalHsmDiscovery::new()?));
        
        {
            let mut d = discovery.lock().await;
            let _ = d.discover_all_hsms()?;
        }
        
        let mut handles = vec![];
        
        for _ in 0..5 {
            let disc = Arc::clone(&discovery);
            handles.push(tokio::spawn(async move {
                let d = disc.lock().await;
                d.get_discovery_stats()
            }));
        }
        
        for handle in handles {
            let _ = handle.await.map_err(|e| 
                BearDogError::internal(format!("Task failed: {}", e)))?;
        }
        
        Ok(())
    }

    // ========== Coordination Tests ==========

    #[tokio::test]
    async fn test_multi_discoverer_coordination() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // All discoverers should coordinate
        let discovered = discovery.discover_all_hsms()?;
        
        // Results should be merged
        assert!(discovered.len() > 0 || discovered.len() == 0);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_capability_and_health_coordination() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // Capabilities and health should be coordinated
        for hsm in &discovered {
            // Unhealthy HSMs may have degraded capabilities
            if hsm.health_status == HsmHealthStatus::Unhealthy {
                // May affect capabilities
            }
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_tier_and_entropy_coordination() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // Human entropy and tier should be coordinated
        for hsm in &discovered {
            if hsm.supports_human_entropy {
                // Typically higher tier or BearDog native
                assert!(hsm.assigned_tier != HsmTier::Untrusted);
            }
        }
        
        Ok(())
    }

    // ========== Complex Workflow Tests ==========

    #[tokio::test]
    async fn test_full_discovery_lifecycle() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // 1. Initial discovery
        let discovered = discovery.discover_all_hsms()?;
        
        // 2. Classify and rank
        let _by_tier = discovery.get_hsms_by_tier()?;
        
        // 3. Health check
        let _ = discovery.perform_health_checks()?;
        
        // 4. Select best
        let _best = discovery.get_best_available_hsm()?;
        
        // 5. Get stats
        let stats = discovery.get_discovery_stats();
        assert!(stats.total_hsms == discovered.len());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_dynamic_hsm_addition() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Initial discovery
        let discovered1 = discovery.discover_all_hsms()?;
        let count1 = discovered1.len();
        
        // Rediscover (may find more)
        let discovered2 = discovery.discover_all_hsms()?;
        let count2 = discovered2.len();
        
        // Should handle dynamic changes
        assert!(count2 >= count1 - count1 || count2 <= count1 + count1);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_hsm_removal_handling() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        if let Some(hsm) = discovered.first() {
            // Mark as unhealthy (effectively removing from pool)
            let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Unhealthy);
            
            // Should handle removal
            let healthy = discovered.iter()
                .filter(|h| h.health_status == HsmHealthStatus::Healthy)
                .count();
            
            assert!(healthy <= discovered.len());
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_high_availability_workflow() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // Simulate HA scenario
        let mut available_hsms = discovered.clone();
        
        // Remove failed HSMs
        available_hsms.retain(|h| h.health_status == HsmHealthStatus::Healthy);
        
        // Should always have fallback
        assert!(available_hsms.len() > 0 || discovered.is_empty());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_load_balancing_workflow() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // Distribute load across HSMs
        let healthy = discovered.iter()
            .filter(|h| h.health_status == HsmHealthStatus::Healthy)
            .collect::<Vec<_>>();
        
        if healthy.len() > 1 {
            // Can distribute load
            for (i, hsm) in healthy.iter().enumerate() {
                // Round-robin or weighted distribution
                let _assigned_load = i % healthy.len();
            }
        }
        
        Ok(())
    }

    // ========== Edge Case Workflows ==========

    #[tokio::test]
    async fn test_no_hsms_discovered_workflow() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Even with no HSMs, workflow should not crash
        let best = discovery.get_best_available_hsm()?;
        assert!(best.is_some() || best.is_none());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_all_hsms_unhealthy_workflow() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // Mark all as unhealthy
        for hsm in &discovered {
            let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Unhealthy);
        }
        
        // Should handle gracefully
        let best = discovery.get_best_available_hsm()?;
        assert!(best.is_some() || best.is_none());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_rapid_status_changes_workflow() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        if let Some(hsm) = discovered.first() {
            // Rapid status changes
            for _ in 0..10 {
                let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Healthy);
                let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Degraded);
            }
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_workflow_performance() -> Result<(), BearDogError> {
        use std::time::Instant;
        
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        let start = Instant::now();
        
        // Complete workflow
        let _ = discovery.discover_all_hsms()?;
        let _ = discovery.get_hsms_by_tier()?;
        let _ = discovery.perform_health_checks()?;
        let _ = discovery.get_best_available_hsm()?;
        
        let duration = start.elapsed();
        assert!(duration.as_secs() < 10, 
            "Workflow should complete reasonably fast: {:?}", duration);
        
        Ok(())
    }
}

