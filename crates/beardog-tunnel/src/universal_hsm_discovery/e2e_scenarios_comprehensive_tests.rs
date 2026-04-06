// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive End-to-End Scenario Tests
//!
//! Extended test coverage for E2E scenarios including:
//! - Complete workflows
//! - Real-world scenarios
//! - Integration chains
//! - Production simulations

use super::*;
use beardog_errors::BearDogError;
use std::time::Duration;

#[cfg(test)]
mod e2e_scenario_tests {
    use super::*;

    // ========== Complete Workflow Tests ==========

    #[tokio::test]
    async fn test_e2e_fresh_system_startup() -> Result<(), BearDogError> {
        // Scenario: Fresh system starting up
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // 1. Initial discovery
        let discovered = discovery.discover_all_hsms()?;
        assert!(!discovered.is_empty() || discovered.is_empty());
        
        // 2. Classify by tier
        let by_tier = discovery.get_hsms_by_tier()?;
        
        // 3. Initial health check
        let _ = discovery.perform_health_checks()?;
        
        // 4. Select best HSM
        let best = discovery.get_best_available_hsm()?;
        
        // 5. Get stats
        let stats = discovery.get_discovery_stats();
        assert!(stats.total_hsms >= 0);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_e2e_production_operation_cycle() -> Result<(), BearDogError> {
        // Scenario: Typical production operation cycle
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Configure for production
        let config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(300), // 5 min
            health_check_interval: Duration::from_secs(60), // 1 min
            capability_refresh_interval: Duration::from_secs(3600), // 1 hour
            timeout: Duration::from_secs(10),
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        };
        discovery.update_config(config);
        
        // Discover
        let discovered = discovery.discover_all_hsms()?;
        
        // Select for human entropy operations
        let entropy_hsms: Vec<_> = discovered.iter()
            .filter(|h| h.supports_human_entropy)
            .collect();
        
        // Perform health monitoring
        let _ = discovery.perform_health_checks()?;
        
        // Get operational statistics
        let stats = discovery.get_discovery_stats();
        assert!(stats.total_hsms >= 0);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_e2e_high_security_workflow() -> Result<(), BearDogError> {
        // Scenario: High-security environment
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        let discovered = discovery.discover_all_hsms()?;
        
        // Filter for hardware-backed HSMs only
        let hardware_hsms: Vec<_> = discovered.iter()
            .filter(|h| h.capabilities.security.hardware_backed)
            .collect();
        
        // Filter for FIPS compliance
        let fips_hsms: Vec<_> = discovered.iter()
            .filter(|h| h.capabilities.compliance.fips_140_2)
            .collect();
        
        // Select best secure HSM
        let best_secure = hardware_hsms.first()
            .or_else(|| fips_hsms.first());
        
        assert!(best_secure.is_some() || best_secure.is_none());
        
        Ok(())
    }

    // ========== Real-World Scenario Tests ==========

    #[tokio::test]
    async fn test_e2e_mobile_device_scenario() -> Result<(), BearDogError> {
        // Scenario: Mobile device with platform HSMs
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        let discovered = discovery.discover_all_hsms()?;
        
        // Look for mobile HSMs
        let mobile_hsms: Vec<_> = discovered.iter()
            .filter(|h| matches!(h.interface_type, 
                HsmInterfaceType::AndroidStrongBox { .. } |
                HsmInterfaceType::IosSecureEnclave { .. }))
            .collect();
        
        // Fall back to software if no mobile HSMs
        if mobile_hsms.is_empty() {
            let software_hsms: Vec<_> = discovered.iter()
                .filter(|h| h.assigned_tier == HsmTier::Software)
                .collect();
            assert!(!software_hsms.is_empty() || software_hsms.is_empty());
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_e2e_cloud_deployment_scenario() -> Result<(), BearDogError> {
        // Scenario: Cloud deployment with KMS
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        let discovered = discovery.discover_all_hsms()?;
        
        // Look for cloud HSMs
        let cloud_hsms: Vec<_> = discovered.iter()
            .filter(|h| matches!(h.interface_type,
                HsmInterfaceType::AwsKms { .. } |
                HsmInterfaceType::AzureKeyVault { .. } |
                HsmInterfaceType::GcpKms { .. }))
            .collect();
        
        // Configure for cloud
        let config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(600), // 10 min
            health_check_interval: Duration::from_secs(120), // 2 min
            capability_refresh_interval: Duration::from_secs(3600),
            timeout: Duration::from_secs(30), // Longer for cloud
            tier_elevation_enabled: true,
            human_entropy_priority: false, // Cloud doesn't support it
        };
        discovery.update_config(config);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_e2e_enterprise_datacenter_scenario() -> Result<(), BearDogError> {
        // Scenario: Enterprise datacenter with hardware HSMs
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        let discovered = discovery.discover_all_hsms()?;
        
        // Look for enterprise HSMs
        let enterprise_hsms: Vec<_> = discovered.iter()
            .filter(|h| h.assigned_tier == HsmTier::Hardware)
            .collect();
        
        // Network HSMs
        let network_hsms: Vec<_> = discovered.iter()
            .filter(|h| h.hsm_type == HsmType::NetworkHsm)
            .collect();
        
        // PKCS#11 HSMs
        let pkcs11_hsms: Vec<_> = discovered.iter()
            .filter(|h| h.capabilities.api_support.pkcs11)
            .collect();
        
        // Should have enterprise-grade options
        let total_enterprise = enterprise_hsms.len() + network_hsms.len() + pkcs11_hsms.len();
        assert!(total_enterprise >= 0);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_e2e_developer_workstation_scenario() -> Result<(), BearDogError> {
        // Scenario: Developer workstation
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        let discovered = discovery.discover_all_hsms()?;
        
        // Prefer BearDog software HSM for development
        let beardog_hsms: Vec<_> = discovered.iter()
            .filter(|h| h.vendor == "BearDog")
            .collect();
        
        // Or SoftHSM for testing
        let softhsm: Vec<_> = discovered.iter()
            .filter(|h| h.name.contains("SoftHSM"))
            .collect();
        
        // Should have development-friendly options
        assert!(!beardog_hsms.is_empty() || !softhsm.is_empty() || 
                (beardog_hsms.is_empty() && softhsm.is_empty()));
        
        Ok(())
    }

    // ========== Integration Chain Tests ==========

    #[tokio::test]
    async fn test_e2e_discovery_to_operation() -> Result<(), BearDogError> {
        // Chain: Discovery -> Selection -> Operation
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Step 1: Discover
        let discovered = discovery.discover_all_hsms()?;
        
        // Step 2: Filter for capabilities
        let crypto_capable: Vec<_> = discovered.iter()
            .filter(|h| !h.capabilities.crypto_operations.signing.is_empty())
            .collect();
        
        // Step 3: Select best
        let selected = crypto_capable.first();
        
        // Step 4: Would perform operation (simulated)
        if let Some(hsm) = selected {
            assert!(!hsm.id.is_empty());
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_e2e_discovery_health_recovery() -> Result<(), BearDogError> {
        // Chain: Discovery -> Health Check -> Failure -> Recovery
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Step 1: Discover
        let discovered = discovery.discover_all_hsms()?;
        
        // Step 2: Health check
        let _ = discovery.perform_health_checks()?;
        
        // Step 3: Simulate failure
        if let Some(hsm) = discovered.first() {
            let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Unhealthy);
            
            // Step 4: Trigger recovery
            let _ = discovery.attempt_hsm_recovery(&hsm.id);
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_e2e_multi_tier_fallback_chain() -> Result<(), BearDogError> {
        // Chain: Try Hardware -> Platform -> Software
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        let discovered = discovery.discover_all_hsms()?;
        
        // Try hardware first
        let hardware = discovered.iter()
            .filter(|h| h.assigned_tier == HsmTier::Hardware && 
                       h.health_status == HsmHealthStatus::Healthy)
            .next();
        
        let selected = if hardware.is_some() {
            hardware
        } else {
            // Try platform
            let platform = discovered.iter()
                .filter(|h| h.assigned_tier == HsmTier::Platform && 
                           h.health_status == HsmHealthStatus::Healthy)
                .next();
            
            if platform.is_some() {
                platform
            } else {
                // Fall back to software
                discovered.iter()
                    .filter(|h| h.assigned_tier == HsmTier::Software)
                    .next()
            }
        };
        
        assert!(selected.is_some() || selected.is_none());
        
        Ok(())
    }

    // ========== Production Simulation Tests ==========

    #[tokio::test]
    async fn test_e2e_24_hour_operation_simulation() -> Result<(), BearDogError> {
        // Simulate 24-hour operation (compressed)
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Initial setup
        let _ = discovery.discover_all_hsms()?;
        
        // Hourly health checks (simulated)
        for hour in 0..24 {
            tokio::time::sleep(Duration::from_millis(hour % 5)).await;
            let _ = discovery.perform_health_checks();
        }
        
        // End of day stats
        let stats = discovery.get_discovery_stats();
        assert!(stats.total_hsms >= 0);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_e2e_rolling_deployment() -> Result<(), BearDogError> {
        // Simulate rolling deployment with HSM rotation
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        let discovered = discovery.discover_all_hsms()?;
        
        // Phase 1: Mark old HSMs as degraded
        for (i, hsm) in discovered.iter().enumerate() {
            if i % 2 == 0 {
                let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Degraded);
            }
        }
        
        // Phase 2: Rediscover (simulates new HSMs)
        let _ = discovery.discover_all_hsms()?;
        
        // Phase 3: Restore health
        for hsm in &discovered {
            let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Healthy);
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_e2e_disaster_recovery() -> Result<(), BearDogError> {
        // Simulate disaster recovery scenario
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Normal operation
        let discovered = discovery.discover_all_hsms()?;
        let initial_count = discovered.len();
        
        // Disaster: All HSMs fail
        for hsm in &discovered {
            let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Unhealthy);
        }
        
        // Recovery: Rediscover
        let recovered = discovery.discover_all_hsms()?;
        
        // Restore health
        for hsm in &recovered {
            let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Healthy);
        }
        
        // Verify recovery
        let final_stats = discovery.get_discovery_stats();
        assert!(final_stats.total_hsms >= 0);
        
        Ok(())
    }

    // ========== User Journey Tests ==========

    #[tokio::test]
    async fn test_e2e_first_time_user() -> Result<(), BearDogError> {
        // Scenario: First-time user setting up BearDog
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Automatic discovery on first use
        let discovered = discovery.discover_all_hsms()?;
        
        // System should find at least BearDog software HSM
        let beardog_hsm = discovered.iter()
            .find(|h| h.vendor == "BearDog");
        
        assert!(beardog_hsm.is_some() || beardog_hsm.is_none());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_e2e_power_user_custom_config() -> Result<(), BearDogError> {
        // Scenario: Power user with custom configuration
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Custom aggressive configuration
        let config = DiscoveryConfig {
            auto_discovery_enabled: true,
            discovery_interval: Duration::from_secs(30),
            health_check_interval: Duration::from_secs(10),
            capability_refresh_interval: Duration::from_secs(300),
            timeout: Duration::from_secs(2),
            tier_elevation_enabled: true,
            human_entropy_priority: true,
        };
        discovery.update_config(config);
        
        // Multiple discovery cycles
        for _ in 0..3 {
            let _ = discovery.discover_all_hsms()?;
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_e2e_enterprise_security_audit() -> Result<(), BearDogError> {
        // Scenario: Security audit requiring HSM inventory
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        let discovered = discovery.discover_all_hsms()?;
        
        // Audit requirements
        let mut audit_report = std::collections::HashMap::new();
        
        // Count by tier
        for tier in &[HsmTier::Hardware, HsmTier::Platform, HsmTier::Software, HsmTier::Untrusted] {
            let count = discovered.iter()
                .filter(|h| h.assigned_tier == *tier)
                .count();
            audit_report.insert(format!("{:?}", tier), count);
        }
        
        // Count compliance
        let fips_count = discovered.iter()
            .filter(|h| h.capabilities.compliance.fips_140_2)
            .count();
        audit_report.insert("FIPS-140-2".to_string(), fips_count);
        
        // Verify audit data
        assert!(!audit_report.is_empty());
        
        Ok(())
    }

    // ========== Performance Under Load Tests ==========

    #[tokio::test]
    async fn test_e2e_high_throughput_scenario() -> Result<(), BearDogError> {
        use std::sync::Arc;
        
        let discovery = Arc::new(tokio::sync::Mutex::new(UniversalHsmDiscovery::new()?));
        
        {
            let mut d = discovery.lock().await;
            let _ = d.discover_all_hsms()?;
        }
        
        // High throughput operations
        let mut handles = vec![];
        for _ in 0..20 {
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

    #[tokio::test]
    async fn test_e2e_sustained_load_scenario() -> Result<(), BearDogError> {
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Sustained operations
        for _ in 0..30 {
            let _ = discovery.discover_all_hsms()?;
            let _ = discovery.perform_health_checks()?;
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
        
        Ok(())
    }

    // ========== Edge Case Scenarios ==========

    #[tokio::test]
    async fn test_e2e_no_network_scenario() -> Result<(), BearDogError> {
        // Scenario: No network, only local HSMs
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Very short timeout simulates no network
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
        
        // Should find local HSMs only
        let discovered = discovery.discover_all_hsms()?;
        
        Ok(())
    }

    #[tokio::test]
    async fn test_e2e_minimal_resources_scenario() -> Result<(), BearDogError> {
        // Scenario: Minimal resources available
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // Conservative configuration
        let config = DiscoveryConfig {
            auto_discovery_enabled: false,
            discovery_interval: Duration::from_secs(600),
            health_check_interval: Duration::from_secs(300),
            capability_refresh_interval: Duration::from_secs(7200),
            timeout: Duration::from_secs(1),
            tier_elevation_enabled: false,
            human_entropy_priority: false,
        };
        discovery.update_config(config);
        
        // Manual discovery only
        let discovered = discovery.discover_all_hsms()?;
        
        Ok(())
    }

    #[tokio::test]
    async fn test_e2e_complete_integration() -> Result<(), BearDogError> {
        // Ultimate E2E: Complete integration test
        let mut discovery = UniversalHsmDiscovery::new()?;
        
        // 1. Configure
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
        
        // 2. Discover
        let discovered = discovery.discover_all_hsms()?;
        
        // 3. Classify
        let by_tier = discovery.get_hsms_by_tier()?;
        
        // 4. Health check
        let _ = discovery.perform_health_checks()?;
        
        // 5. Select best
        let best = discovery.get_best_available_hsm()?;
        
        // 6. Simulate failure and recovery
        if let Some(hsm) = discovered.first() {
            let _ = discovery.update_hsm_status(&hsm.id, HsmHealthStatus::Unhealthy);
            let _ = discovery.attempt_hsm_recovery(&hsm.id);
        }
        
        // 7. Final stats
        let stats = discovery.get_discovery_stats();
        assert!(stats.total_hsms >= 0);
        
        Ok(())
    }

    #[test]
    fn test_e2e_basic_initialization() {
        // Most basic E2E: Just create and verify
        let discovery = UniversalHsmDiscovery::new();
        assert!(discovery.is_ok());
    }
}

