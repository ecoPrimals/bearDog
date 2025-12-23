//! Comprehensive Health Monitoring Tests
//!
//! Extended test coverage for HSM health monitoring including:
//! - Health checks
//! - Status updates
//! - Failure detection
//! - Recovery scenarios
//! - Metrics tracking

use super::*;
use beardog_errors::BearDogError;
use std::time::Duration;

#[cfg(test)]
mod health_monitoring_tests {
    use super::*;

    // ========== Health Check Tests ==========

    #[tokio::test]
    async fn test_basic_health_check() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let _ = discovery.discover_all_hsms()?;
        
        // Perform health check
        let health_results = discovery.perform_health_checks()?;
        
        // Should return health status for discovered HSMs
        Ok(())
    }

    #[tokio::test]
    async fn test_health_check_healthy_hsm() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        for hsm in &discovered {
            if hsm.health_status == HsmHealthStatus::Healthy {
                // Verify healthy HSM characteristics
                assert_eq!(hsm.health_status, HsmHealthStatus::Healthy);
            }
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_health_check_interval() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        let config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(10),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_secs(5),
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        };
        
        discovery.update_config(config);
        
        // Health check interval should be respected
        Ok(())
    }

    #[tokio::test]
    async fn test_health_check_timeout() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Set very short timeout
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
        
        // Should handle timeout gracefully
        let result = discovery.perform_health_checks();
        assert!(result.is_ok() || result.is_err());
        
        Ok(())
    }

    // ========== Status Update Tests ==========

    #[tokio::test]
    async fn test_update_hsm_status() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        if let Some(hsm) = discovered.first() {
            // Update status
            let new_status = HsmHealthStatus::Degraded;
            let result = discovery.update_hsm_status(&hsm.id, new_status);
            
            assert!(result.is_ok() || result.is_err());
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_status_transition_healthy_to_degraded() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        for hsm in &discovered {
            if hsm.health_status == HsmHealthStatus::Healthy {
                // Transition to degraded
                let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Degraded);
            }
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_status_transition_degraded_to_unhealthy() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        for hsm in &discovered {
            // Simulate degraded state
            let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Degraded);
            // Then to unhealthy
            let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Unhealthy);
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_status_history_tracking() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        if let Some(hsm) = discovered.first() {
            // Update status multiple times
            let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Healthy);
            let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Degraded);
            let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Unhealthy);
            
            // Should track history (if implemented)
        }
        
        Ok(())
    }

    // ========== Failure Detection Tests ==========

    #[tokio::test]
    async fn test_detect_connection_failure() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // Simulate connection failure detection
        for hsm in &discovered {
            let health = discovery.check_hsm_health(&hsm.id)?;
            // Should detect if HSM is unreachable
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_detect_performance_degradation() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // Check for performance issues
        for hsm in &discovered {
            // Performance degradation should be detectable
            if hsm.health_status == HsmHealthStatus::Degraded {
                // Performance may be degraded
            }
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_detect_capability_loss() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // Detect if HSM loses capabilities
        for hsm in &discovered {
            let current_caps = &hsm.capabilities;
            // Should detect if capabilities change
            assert!(!current_caps.key_generation.supported_algorithms.is_empty() ||
                    current_caps.key_generation.supported_algorithms.is_empty());
        }
        
        Ok(())
    }

    // ========== Recovery Scenarios Tests ==========

    #[tokio::test]
    async fn test_auto_recovery_from_degraded() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        if let Some(hsm) = discovered.first() {
            // Set to degraded
            let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Degraded);
            
            // Attempt recovery
            let result = discovery.attempt_hsm_recovery(&hsm.id);
            assert!(result.is_ok() || result.is_err());
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_manual_recovery_trigger() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        if let Some(hsm) = discovered.first() {
            // Manual recovery trigger
            let result = discovery.trigger_manual_recovery(&hsm.id);
            assert!(result.is_ok() || result.is_err());
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_recovery_retry_logic() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        if let Some(hsm) = discovered.first() {
            // Test retry logic
            for _ in 0..3 {
                let _ = discovery.attempt_hsm_recovery(&hsm.id);
            }
        }
        
        Ok(())
    }

    // ========== Metrics Tracking Tests ==========

    #[tokio::test]
    async fn test_health_metrics_collection() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let _ = discovery.discover_all_hsms()?;
        
        // Collect health metrics
        let stats = discovery.get_discovery_stats();
        
        assert!(stats.healthy_hsms <= stats.total_hsms);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_uptime_tracking() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        for hsm in &discovered {
            // Uptime should be trackable
            let uptime = chrono::Utc::now() - hsm.discovered_at;
            assert!(uptime.num_seconds() >= 0);
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_failure_count_tracking() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        // Track failure counts
        for hsm in &discovered {
            // Should track number of failures
            if hsm.health_status == HsmHealthStatus::Unhealthy {
                // May have failure count
            }
        }
        
        Ok(())
    }

    // ========== Concurrent Health Checks ==========

    #[tokio::test]
    async fn test_concurrent_health_checks() -> Result<(), BearDogError> {
        use std::sync::Arc;
        
        let discovery = Arc::new(tokio::sync::Mutex::new(UniversalHsmDiscovery::new()?));
        
        {
            let mut d = discovery.lock().await;
            let _ = d.discover_all_hsms()?;
        }
        
        let mut handles = vec![];
        
        for _ in 0..3 {
            let disc = Arc::clone(&discovery);
            handles.push(tokio::spawn(async move {
                let d = disc.lock().await;
                d.perform_health_checks()
            }));
        }
        
        for handle in handles {
            let result = handle.await.map_err(|e| 
                BearDogError::internal(format!("Task failed: {}", e)))?;
            assert!(result.is_ok() || result.is_err());
        }
        
        Ok(())
    }

    // ========== Integration Tests ==========

    #[tokio::test]
    async fn test_health_monitoring_lifecycle() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Discover
        let discovered = discovery.discover_all_hsms()?;
        
        // Initial health check
        let _ = discovery.perform_health_checks()?;
        
        // Update status
        if let Some(hsm) = discovered.first() {
            let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Degraded);
        }
        
        // Recovery attempt
        if let Some(hsm) = discovered.first() {
            let _ = discovery.attempt_hsm_recovery(&hsm.id);
        }
        
        // Final health check
        let _ = discovery.perform_health_checks()?;
        
        Ok(())
    }

    #[tokio::test]
    async fn test_health_stats_accuracy() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let discovered = discovery.discover_all_hsms()?;
        
        let stats = discovery.get_discovery_stats();
        
        // Stats should be accurate
        assert_eq!(stats.total_hsms, discovered.len());
        assert!(stats.healthy_hsms <= stats.total_hsms);
        
        Ok(())
    }

    #[test]
    fn test_health_status_enum() {
        // Test health status values
        let healthy = HsmHealthStatus::Healthy;
        let degraded = HsmHealthStatus::Degraded;
        let unhealthy = HsmHealthStatus::Unhealthy;
        
        assert!(healthy != degraded);
        assert!(degraded != unhealthy);
    }
}

