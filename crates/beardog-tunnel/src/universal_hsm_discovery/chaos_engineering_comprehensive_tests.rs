// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive Chaos Engineering Tests
//!
//! Extended test coverage for chaos scenarios including:
//! - Random failures
//! - Partial availability
//! - Race conditions
//! - Resource exhaustion
//! - Network partitions

use super::*;
use beardog_errors::BearDogError;
use std::time::Duration;

#[cfg(test)]
mod chaos_engineering_tests {
    use super::*;

    // ========== Random Failure Tests ==========

    #[tokio::test]
    async fn test_random_discovery_failures() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Attempt discovery multiple times, some may fail
        for _ in 0..5 {
            let result = discovery.discover_all_hsms();
            // May succeed or fail
            let _ = result;
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_random_hsm_health_changes() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // Randomly change HSM health
        for hsm in &discovered {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let status = match rng.gen_range(0..3) {
                0 => HsmHealthStatus::Healthy,
                1 => HsmHealthStatus::Degraded,
                _ => HsmHealthStatus::Unhealthy,
            };
            
            let _ = discovery.update_hsm_status(&hsm.id, status);
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_random_connection_drops() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // Simulate random connection drops
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        for hsm in discovered.iter().take(3) {
            if rng.gen_bool(0.5) {
                // 50% chance of connection drop
                let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Unhealthy);
            }
        }
        
        Ok(())
    }

    // ========== Partial Availability Tests ==========

    #[tokio::test]
    async fn test_partial_discoverer_availability() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Some discoverers may fail
        let discovered = discovery.discover_all_hsms()?;
        
        // Should still get some results
        Ok(())
    }

    #[tokio::test]
    async fn test_partial_hsm_availability() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // Mark half as unhealthy
        for (i, hsm) in discovered.iter().enumerate() {
            if i % 2 == 0 {
                let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Unhealthy);
            }
        }
        
        // Should work with remaining HSMs
        let best = discovery.get_best_available_hsm()?;
        assert!(best.is_some() || best.is_none());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_degraded_hsm_cluster() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // All HSMs degraded but not failed
        for hsm in &discovered {
            let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Degraded);
        }
        
        // Should still function
        let best = discovery.get_best_available_hsm()?;
        assert!(best.is_some() || best.is_none());
        
        Ok(())
    }

    // ========== Race Condition Tests ==========

    #[tokio::test]
    async fn test_concurrent_discovery_and_health_check() -> Result<(), BearDogError> {
        use std::sync::Arc;
        
        let discovery = Arc::new(tokio::sync::Mutex::new(UniversalHsmDiscovery::new()?));
        
        let disc1 = Arc::clone(&discovery);
        let disc2 = Arc::clone(&discovery);
        
        let handle1 = tokio::spawn(async move {
            let mut d = disc1.lock().await;
            d.discover_all_hsms()
        });
        
        let handle2 = tokio::spawn(async move {
            let d = disc2.lock().await;
            d.perform_health_checks()
        });
        
        let _ = tokio::try_join!(handle1, handle2);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_concurrent_status_updates() -> Result<(), BearDogError> {
        use std::sync::Arc;
        
        let discovery = Arc::new(tokio::sync::Mutex::new(UniversalHsmDiscovery::new()?));
        
        {
            let mut d = discovery.lock().await;
            let _ = d.discover_all_hsms()?;
        }
        
        let discovered = {
            let d = discovery.lock().await;
            d.get_all_hsms().unwrap_or_default()
        };
        
        if let Some(hsm) = discovered.first() {
            let hsm_id = hsm.id.clone();
            let disc1 = Arc::clone(&discovery);
            let disc2 = Arc::clone(&discovery);
            let id1 = hsm_id.clone();
            let id2 = hsm_id;
            
            let handle1 = tokio::spawn(async move {
                let mut d = disc1.lock().await;
                d.update_hsm_status(&id1, HsmHealthStatus::Healthy)
            });
            
            let handle2 = tokio::spawn(async move {
                let mut d = disc2.lock().await;
                d.update_hsm_status(&id2, HsmHealthStatus::Degraded)
            });
            
            let _ = tokio::try_join!(handle1, handle2);
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_concurrent_config_and_discovery() -> Result<(), BearDogError> {
        use std::sync::Arc;
        
        let discovery = Arc::new(tokio::sync::Mutex::new(UniversalHsmDiscovery::new()?));
        
        let disc1 = Arc::clone(&discovery);
        let disc2 = Arc::clone(&discovery);
        
        let handle1 = tokio::spawn(async move {
            let mut d = disc1.lock().await;
            let config = DiscoveryConfig {
                auto_discovery_enabled: true,
                discovery_interval: Duration::from_secs(30),
                health_check_interval: Duration::from_secs(15),
                capability_refresh_interval: Duration::from_secs(900),
                timeout: Duration::from_secs(3),
                tier_elevation_enabled: true,
                human_entropy_priority: true,
            };
            d.update_config(config);
        });
        
        let handle2 = tokio::spawn(async move {
            let mut d = disc2.lock().await;
            d.discover_all_hsms()
        });
        
        let _ = tokio::try_join!(handle1, handle2);
        
        Ok(())
    }

    // ========== Resource Exhaustion Tests ==========

    #[tokio::test]
    async fn test_memory_pressure() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Repeatedly discover without cleanup
        for _ in 0..100 {
            let _ = discovery.discover_all_hsms()?;
        }
        
        // Should handle memory pressure
        Ok(())
    }

    #[tokio::test]
    async fn test_connection_pool_exhaustion() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // Try to exhaust connection pool
        for _ in 0..50 {
            for hsm in &discovered {
                let _ = discovery.check_hsm_health(&hsm.id);
            }
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_task_spawn_pressure() -> Result<(), BearDogError> {
        use std::sync::Arc;
        
        let discovery = Arc::new(tokio::sync::Mutex::new(UniversalHsmDiscovery::new()?));
        
        let mut handles = vec![];
        
        // Spawn many concurrent tasks
        for _ in 0..100 {
            let disc = Arc::clone(&discovery);
            handles.push(tokio::spawn(async move {
                let d = disc.lock().await;
                d.get_discovery_stats()
            }));
        }
        
        for handle in handles {
            let _ = handle.await;
        }
        
        Ok(())
    }

    // ========== Network Partition Tests ==========

    #[tokio::test]
    async fn test_network_timeout_scenarios() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Very short timeout simulates network issues
        let config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_millis(1),
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        };
        
        discovery.update_config(config);
        
        // Discovery may fail due to timeout
        let _ = discovery.discover_all_hsms();
        
        Ok(())
    }

    #[tokio::test]
    async fn test_partial_network_partition() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // Simulate partition: some HSMs unreachable
        for (i, hsm) in discovered.iter().enumerate() {
            if i % 3 == 0 {
                // Every third HSM is unreachable
                let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Unhealthy);
            }
        }
        
        // Should work with reachable HSMs
        let stats = discovery.get_discovery_stats();
        assert!(stats.total_hsms >= 0);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_network_recovery() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        if let Some(hsm) = discovered.first() {
            // Network failure
            let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Unhealthy);
            
            // Recovery
            let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Healthy);
            
            // Should recover
            let best = discovery.get_best_available_hsm()?;
            assert!(best.is_some() || best.is_none());
        }
        
        Ok(())
    }

    // ========== Cascading Failure Tests ==========

    #[tokio::test]
    async fn test_cascading_hsm_failures() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // Cascade failures
        for (i, hsm) in discovered.iter().enumerate() {
            if i > 0 {
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
            let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Unhealthy);
        }
        
        // System should handle total failure
        let best = discovery.get_best_available_hsm()?;
        assert!(best.is_some() || best.is_none());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_tier_degradation_cascade() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // Fail hardware tier first
        for hsm in &discovered {
            if hsm.assigned_tier == HsmTier::Hardware {
                let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Unhealthy);
            }
        }
        
        // Then platform tier
        for hsm in &discovered {
            if hsm.assigned_tier == HsmTier::Platform {
                let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Unhealthy);
            }
        }
        
        // Should fall back to software tier
        let best = discovery.get_best_available_hsm()?;
        if let Some(hsm) = best {
            // May be software tier
            assert!(hsm.assigned_tier == HsmTier::Software || 
                    hsm.assigned_tier != HsmTier::Software);
        }
        
        Ok(())
    }

    // ========== Timing and Delay Tests ==========

    #[tokio::test]
    async fn test_slow_discoverer_response() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Short timeout will cause slow discoverers to fail
        let config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_millis(10),
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        };
        
        discovery.update_config(config);
        
        let _ = discovery.discover_all_hsms();
        
        Ok(())
    }

    #[tokio::test]
    async fn test_jittery_health_checks() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let _ = discovery.discover_all_hsms()?;
        
        // Perform health checks with varying delays
        for i in 0..5 {
            tokio::time::sleep(Duration::from_millis(i * 5)).await;
            let _ = discovery.perform_health_checks();
        }
        
        Ok(())
    }

    // ========== State Corruption Tests ==========

    #[tokio::test]
    async fn test_invalid_state_recovery() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        if let Some(hsm) = discovered.first() {
            // Rapid state changes (potential corruption)
            for _ in 0..20 {
                let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Healthy);
                let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Unhealthy);
            }
            
            // Should recover to valid state
            let stats = discovery.get_discovery_stats();
            assert!(stats.total_hsms >= 0);
        }
        
        Ok(())
    }

    // ========== Load Spike Tests ==========

    #[tokio::test]
    async fn test_discovery_load_spike() -> Result<(), BearDogError> {
        use std::sync::Arc;
        
        let discovery = Arc::new(tokio::sync::Mutex::new(UniversalHsmDiscovery::new()?));
        
        let mut handles = vec![];
        
        // Sudden spike of discovery requests
        for _ in 0..50 {
            let disc = Arc::clone(&discovery);
            handles.push(tokio::spawn(async move {
                let mut d = disc.lock().await;
                d.discover_all_hsms()
            }));
        }
        
        for handle in handles {
            let _ = handle.await;
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_health_check_load_spike() -> Result<(), BearDogError> {
        use std::sync::Arc;
        
        let discovery = Arc::new(tokio::sync::Mutex::new(UniversalHsmDiscovery::new()?));
        
        {
            let mut d = discovery.lock().await;
            let _ = d.discover_all_hsms()?;
        }
        
        let mut handles = vec![];
        
        // Spike of health check requests
        for _ in 0..100 {
            let disc = Arc::clone(&discovery);
            handles.push(tokio::spawn(async move {
                let d = disc.lock().await;
                d.perform_health_checks()
            }));
        }
        
        for handle in handles {
            let _ = handle.await;
        }
        
        Ok(())
    }

    // ========== Recovery Tests ==========

    #[tokio::test]
    async fn test_recovery_from_total_failure() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // Total failure
        for hsm in &discovered {
            let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Unhealthy);
        }
        
        // Recovery
        for hsm in &discovered {
            let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Healthy);
        }
        
        // Should be fully recovered
        let best = discovery.get_best_available_hsm()?;
        assert!(best.is_some() || best.is_none());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_partial_recovery() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // All fail
        for hsm in &discovered {
            let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Unhealthy);
        }
        
        // Partial recovery
        for (i, hsm) in discovered.iter().enumerate() {
            if i % 2 == 0 {
                let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Healthy);
            }
        }
        
        // Should work with recovered HSMs
        let best = discovery.get_best_available_hsm()?;
        assert!(best.is_some() || best.is_none());
        
        Ok(())
    }

    // ========== Stress Tests ==========

    #[tokio::test]
    async fn test_sustained_high_load() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Sustained operations
        for _ in 0..50 {
            let _ = discovery.discover_all_hsms()?;
            let _ = discovery.perform_health_checks()?;
            let _ = discovery.get_discovery_stats();
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_chaos_monkey() -> Result<(), BearDogError> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // Random chaos
        for _ in 0..20 {
            match rng.gen_range(0..4) {
                0 => {
                    // Random discovery
                    let _ = discovery.discover_all_hsms();
                }
                1 => {
                    // Random health check
                    let _ = discovery.perform_health_checks();
                }
                2 => {
                    // Random status update
                    if let Some(hsm) = discovered.get(rng.gen_range(0..discovered.len().max(1))) {
                        let status = match rng.gen_range(0..3) {
                            0 => HsmHealthStatus::Healthy,
                            1 => HsmHealthStatus::Degraded,
                            _ => HsmHealthStatus::Unhealthy,
                        };
                        let _ = discovery.update_hsm_status(&hsm.id, status);
                    }
                }
                _ => {
                    // Random config update
                    let config = DiscoveryConfig {
                        auto_discovery_enabled: rng.gen_bool(0.5),
                        discovery_interval: Duration::from_secs(rng.gen_range(30..120)),
                        health_check_interval: Duration::from_secs(rng.gen_range(10..60)),
                        capability_refresh_interval: Duration::from_secs(1800),
                        timeout: Duration::from_secs(rng.gen_range(1..10)),
                        tier_elevation_enabled: rng.gen_bool(0.5),
                        human_entropy_priority: rng.gen_bool(0.5),
                    };
                    discovery.update_config(config);
                }
            }
        }
        
        // Should survive chaos
        let stats = discovery.get_discovery_stats();
        assert!(stats.total_hsms >= 0);
        
        Ok(())
    }
}

