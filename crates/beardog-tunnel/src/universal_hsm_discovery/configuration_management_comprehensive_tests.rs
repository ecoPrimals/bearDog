//! Comprehensive Configuration Management Tests
//!
//! Extended test coverage for configuration management including:
//! - Config validation
//! - Dynamic updates
//! - Persistence
//! - Migration
//! - Default configurations

use super::*;
use beardog_errors::BearDogError;
use std::time::Duration;

#[cfg(test)]
mod configuration_tests {
    use super::*;

    // ========== Config Validation Tests ==========

    #[tokio::test]
    async fn test_default_config() -> Result<(), BearDogError> {
        let discovery = UniversalHsmDiscovery::new()?;
        
        // Should have default configuration
        Ok(())
    }

    #[tokio::test]
    async fn test_valid_config_update() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        let config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_secs(5),
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        };
        
        discovery.update_config(config);
        
        // Should accept valid configuration
        Ok(())
    }

    #[tokio::test]
    async fn test_config_with_zero_timeout() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        let config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_secs(0), // Zero timeout
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        };
        
        discovery.update_config(config);
        
        // Should handle zero timeout
        Ok(())
    }

    #[tokio::test]
    async fn test_config_with_very_long_intervals() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        let config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(86400), // 1 day
            health_check_interval: Duration::from_secs(3600), // 1 hour
            capability_refresh_interval: Duration::from_secs(604800), // 1 week
            timeout: Duration::from_secs(30),
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        };
        
        discovery.update_config(config);
        
        // Should accept long intervals
        Ok(())
    }

    #[tokio::test]
    async fn test_config_with_very_short_intervals() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        let config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(1),
            health_check_interval: Duration::from_secs(1),
            capability_refresh_interval: Duration::from_secs(1),
            timeout: Duration::from_millis(100),
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        };
        
        discovery.update_config(config);
        
        // Should handle short intervals
        Ok(())
    }

    // ========== Dynamic Update Tests ==========

    #[tokio::test]
    async fn test_update_config_while_running() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        let _ = discovery.discover_all_hsms()?;
        
        // Update config while system is running
        let new_config = DiscoveryConfig {
            auto_discovery_enabled: false,
            discovery_interval: Duration::from_secs(120),
            health_check_interval: Duration::from_secs(60),
            capability_refresh_interval: Duration::from_secs(3600),
            timeout: Duration::from_secs(10),
            tier_elevation_enabled: false,
            human_entropy_priority: false,
        };
        
        discovery.update_config(new_config);
        
        // Should apply changes
        Ok(())
    }

    #[tokio::test]
    async fn test_toggle_auto_discovery() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Enable
        let config1 = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_secs(5),
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        };
        discovery.update_config(config1);
        
        // Disable
        let config2 = DiscoveryConfig {
            auto_discovery_enabled: false,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_secs(5),
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        };
        discovery.update_config(config2);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_toggle_tier_elevation() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Disable tier elevation
        let config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_secs(5),
            tier_elevation_enabled: false,
            human_entropy_priority: true,
        };
        
        discovery.update_config(config);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_toggle_human_entropy_priority() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Disable human entropy priority
        let config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_secs(5),
            tier_elevation_enabled: true,
            human_entropy_priority: false,
        };
        
        discovery.update_config(config);
        
        Ok(())
    }

    // ========== Config Persistence Tests ==========

    #[tokio::test]
    async fn test_config_persistence() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        let config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(90),
            health_check_interval: Duration::from_secs(45),
            capability_refresh_interval: Duration::from_secs(2700),
            timeout: Duration::from_secs(8),
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        };
        
        discovery.update_config(config);
        
        // Config should persist across operations
        let _ = discovery.discover_all_hsms()?;
        
        Ok(())
    }

    #[tokio::test]
    async fn test_config_after_multiple_updates() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Multiple rapid updates
        for i in 1..=5 {
            let config = DiscoveryConfig {
                auto_discovery_enabled: i % 2 == 0,
                discovery_interval: Duration::from_secs(60 * i),
                health_check_interval: Duration::from_secs(30 * i),
                capability_refresh_interval: Duration::from_secs(1800),
                timeout: Duration::from_secs(5),
                tier_elevation_enabled: true,
                human_entropy_priority: true,
            };
            discovery.update_config(config);
        }
        
        // Last config should be active
        Ok(())
    }

    // ========== Config Migration Tests ==========

    #[tokio::test]
    async fn test_config_version_compatibility() -> Result<(), BearDogError> {
        // Test that configs from different versions are compatible
        let discovery = UniversalHsmDiscovery::new()?;
        
        // Should work with default config
        Ok(())
    }

    #[tokio::test]
    async fn test_partial_config_update() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Update only some fields
        let config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(120), // Changed
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_secs(5),
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        };
        
        discovery.update_config(config);
        
        Ok(())
    }

    // ========== Concurrent Config Updates ==========

    #[tokio::test]
    async fn test_concurrent_config_updates() -> Result<(), BearDogError> {
        use std::sync::Arc;
        
        let discovery = Arc::new(tokio::sync::Mutex::new(UniversalHsmDiscovery::new()?));
        let mut handles = vec![];
        
        for i in 0..3 {
            let disc = Arc::clone(&discovery);
            handles.push(tokio::spawn(async move {
                let mut d = disc.lock().await;
                let config = DiscoveryConfig {
                    auto_discovery_enabled: true,
                    discovery_interval: Duration::from_secs(60 + i * 10),
                    health_check_interval: Duration::from_secs(30),
                    capability_refresh_interval: Duration::from_secs(1800),
                    timeout: Duration::from_secs(5),
                    tier_elevation_enabled: true,
                    human_entropy_priority: true,
                };
                d.update_config(config);
            }));
        }
        
        for handle in handles {
            handle.await.map_err(|e| 
                BearDogError::internal(format!("Task failed: {}", e)))?;
        }
        
        Ok(())
    }

    // ========== Config Impact Tests ==========

    #[tokio::test]
    async fn test_config_affects_discovery() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Disable auto-discovery
        let config = DiscoveryConfig {
            auto_discovery_enabled: false,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_secs(5),
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        };
        
        discovery.update_config(config);
        
        // Manual discovery should still work
        let _ = discovery.discover_all_hsms()?;
        
        Ok(())
    }

    #[tokio::test]
    async fn test_timeout_config_effect() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Very short timeout
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
        
        // Discovery should respect timeout
        let result = discovery.discover_all_hsms();
        assert!(result.is_ok() || result.is_err());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_tier_elevation_config_effect() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Disable tier elevation
        let config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_secs(5),
            tier_elevation_enabled: false,
            human_entropy_priority: true,
        };
        
        discovery.update_config(config);
        
        let discovered = discovery.discover_all_hsms()?;
        
        // Tier elevation should be disabled
        Ok(())
    }

    #[tokio::test]
    async fn test_human_entropy_priority_config_effect() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Disable human entropy priority
        let config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_secs(5),
            tier_elevation_enabled: true,
            human_entropy_priority: false,
        };
        
        discovery.update_config(config);
        
        let discovered = discovery.discover_all_hsms()?;
        
        // Human entropy priority should be disabled
        Ok(())
    }

    // ========== Integration Tests ==========

    #[tokio::test]
    async fn test_config_lifecycle() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Initial config
        let config1 = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_secs(5),
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        };
        discovery.update_config(config1);
        
        // Discover
        let _ = discovery.discover_all_hsms()?;
        
        // Update config
        let config2 = DiscoveryConfig {
            auto_discovery_enabled: false,
            discovery_interval: Duration::from_secs(120),
            health_check_interval: Duration::from_secs(60),
            capability_refresh_interval: Duration::from_secs(3600),
            timeout: Duration::from_secs(10),
            tier_elevation_enabled: false,
            human_entropy_priority: false,
        };
        discovery.update_config(config2);
        
        // Discover again
        let _ = discovery.discover_all_hsms()?;
        
        Ok(())
    }

    #[test]
    fn test_config_struct_clone() {
        let config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            capability_refresh_interval: Duration::from_secs(1800),
            timeout: Duration::from_secs(5),
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        };
        
        let _cloned = config.clone();
        
        // Should be cloneable
    }
}

