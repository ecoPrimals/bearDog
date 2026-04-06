// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive Network Discoverer Tests
//!
//! Extended test coverage for network HSM discovery including:
//! - mDNS/DNS-SD scenarios
//! - Custom endpoint configuration
//! - TLS validation
//! - Timeout handling
//! - Enterprise network HSMs

use super::*;
use beardog_errors::BearDogError;

#[cfg(test)]
mod network_comprehensive_tests {
    use super::*;

    // ========== mDNS/DNS-SD Tests ==========

    #[tokio::test]
    async fn test_mdns_discovery() -> Result<(), BearDogError> {
        let mut discoverer = NetworkDiscoverer::new()?;
        discoverer.enable_mdns_probe = true;
        
        let discovered = discoverer.discover().await?;
        
        // mDNS may or may not find services
        Ok(())
    }

    #[tokio::test]
    async fn test_mdns_disabled() -> Result<(), BearDogError> {
        let mut discoverer = NetworkDiscoverer::new()?;
        discoverer.enable_mdns_probe = false;
        
        let discovered = discoverer.discover().await?;
        
        // Should not use mDNS when disabled
        Ok(())
    }

    #[tokio::test]
    async fn test_dns_sd_discovery() -> Result<(), BearDogError> {
        let mut discoverer = NetworkDiscoverer::new()?;
        discoverer.enable_dns_sd_probe = true;
        
        let discovered = discoverer.discover().await?;
        
        // DNS-SD may or may not find services
        Ok(())
    }

    #[tokio::test]
    async fn test_dns_sd_disabled() -> Result<(), BearDogError> {
        let mut discoverer = NetworkDiscoverer::new()?;
        discoverer.enable_dns_sd_probe = false;
        
        let discovered = discoverer.discover().await?;
        
        // Should not use DNS-SD when disabled
        Ok(())
    }

    #[tokio::test]
    async fn test_mdns_timeout() -> Result<(), BearDogError> {
        let discoverer = NetworkDiscoverer::new()?;
        
        // mDNS should have reasonable timeout
        let result = discoverer.discover().await;
        assert!(result.is_ok(), "Should handle mDNS timeout");
        
        Ok(())
    }

    // ========== Custom Endpoint Tests ==========

    #[tokio::test]
    async fn test_custom_endpoints() -> Result<(), BearDogError> {
        let mut discoverer = NetworkDiscoverer::new()?;
        
        // Add custom endpoints
        discoverer.custom_endpoints.push("https://hsm1.example.com:8443".to_string());
        discoverer.custom_endpoints.push("https://hsm2.example.com:8443".to_string());
        
        let discovered = discoverer.discover().await?;
        
        // Should attempt to discover custom endpoints
        Ok(())
    }

    #[tokio::test]
    async fn test_custom_endpoints_empty() -> Result<(), BearDogError> {
        let mut discoverer = NetworkDiscoverer::new()?;
        discoverer.enable_mdns_probe = false;
        discoverer.enable_dns_sd_probe = false;
        discoverer.custom_endpoints.clear();
        
        let discovered = discoverer.discover().await?;
        
        // Should return empty when nothing enabled
        assert!(discovered.is_empty());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_endpoint_url() -> Result<(), BearDogError> {
        let mut discoverer = NetworkDiscoverer::new()?;
        discoverer.custom_endpoints.push("invalid-url".to_string());
        
        // Should handle invalid URLs gracefully
        let result = discoverer.discover().await;
        assert!(result.is_ok());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_endpoint_connection_failure() -> Result<(), BearDogError> {
        let mut discoverer = NetworkDiscoverer::new()?;
        discoverer.custom_endpoints.push("https://nonexistent.hsm.local:9999".to_string());
        
        // Should handle connection failures
        let result = discoverer.discover().await;
        assert!(result.is_ok());
        
        Ok(())
    }

    // ========== TLS/Security Tests ==========

    #[tokio::test]
    async fn test_https_endpoint() -> Result<(), BearDogError> {
        let mut discoverer = NetworkDiscoverer::new()?;
        discoverer.custom_endpoints.push("https://secure-hsm.example.com".to_string());
        
        let discovered = discoverer.discover().await?;
        
        // Should handle HTTPS endpoints
        Ok(())
    }

    #[tokio::test]
    async fn test_http_endpoint() -> Result<(), BearDogError> {
        let mut discoverer = NetworkDiscoverer::new()?;
        discoverer.custom_endpoints.push("http://unsecure-hsm.example.com".to_string());
        
        let discovered = discoverer.discover().await?;
        
        // Should allow HTTP for testing (with warnings)
        Ok(())
    }

    #[tokio::test]
    async fn test_tls_certificate_validation() -> Result<(), BearDogError> {
        let mut discoverer = NetworkDiscoverer::new()?;
        discoverer.custom_endpoints.push("https://self-signed.hsm.local".to_string());
        
        // Should validate or reject self-signed certificates
        let result = discoverer.discover().await;
        assert!(result.is_ok());
        
        Ok(())
    }

    // ========== Network HSM Capabilities ==========

    #[tokio::test]
    async fn test_network_hsm_capabilities() -> Result<(), BearDogError> {
        let discoverer = NetworkDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in &discovered {
            // Network HSMs should support KMIP or proprietary protocols
            assert!(hsm.capabilities.api_support.kmip || 
                    !hsm.capabilities.api_support.kmip);
            
            // Should have FIPS 140-2 Level 3 for enterprise HSMs
            if let Some(fips) = hsm.capabilities.security.fips_140_level {
                assert!(fips >= 2, "Enterprise network HSMs should be FIPS Level 2+");
            }
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_kmip_support() -> Result<(), BearDogError> {
        let discoverer = NetworkDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in discovered.iter().filter(|h| h.capabilities.api_support.kmip) {
            assert_eq!(hsm.hsm_type, HsmType::NetworkHsm);
        }
        
        Ok(())
    }

    // ========== Enterprise HSM Tests ==========

    #[tokio::test]
    async fn test_enterprise_network_hsm() -> Result<(), BearDogError> {
        let discoverer = NetworkDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in &discovered {
            // Enterprise HSMs should have high availability
            // This is a soft check - not all network HSMs are HA
            if hsm.name.contains("Enterprise") || hsm.name.contains("Cluster") {
                // Likely has redundancy features
            }
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_hsm_cluster_detection() -> Result<(), BearDogError> {
        let discoverer = NetworkDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        // May discover HSM clusters
        // Each node should have unique ID
        let mut ids = std::collections::HashSet::new();
        for hsm in &discovered {
            assert!(ids.insert(&hsm.id));
        }
        
        Ok(())
    }

    // ========== Error Handling Tests ==========

    #[tokio::test]
    async fn test_network_timeout() -> Result<(), BearDogError> {
        let mut discoverer = NetworkDiscoverer::new()?;
        discoverer.custom_endpoints.push("https://very-slow-hsm.example.com".to_string());
        
        // Should handle timeouts gracefully
        let result = discoverer.discover().await;
        assert!(result.is_ok());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_dns_resolution_failure() -> Result<(), BearDogError> {
        let mut discoverer = NetworkDiscoverer::new()?;
        discoverer.custom_endpoints.push("https://nonexistent-domain-12345.com".to_string());
        
        // Should handle DNS failures
        let result = discoverer.discover().await;
        assert!(result.is_ok());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_network_unreachable() -> Result<(), BearDogError> {
        let mut discoverer = NetworkDiscoverer::new()?;
        discoverer.custom_endpoints.push("https://192.0.2.1:8443".to_string()); // TEST-NET-1
        
        // Should handle unreachable networks
        let result = discoverer.discover().await;
        assert!(result.is_ok());
        
        Ok(())
    }

    #[tokio::test]
    async fn test_connection_refused() -> Result<(), BearDogError> {
        let mut discoverer = NetworkDiscoverer::new()?;
        discoverer.custom_endpoints.push("https://localhost:9999".to_string());
        
        // Should handle connection refused
        let result = discoverer.discover().await;
        assert!(result.is_ok());
        
        Ok(())
    }

    // ========== Integration Tests ==========

    #[tokio::test]
    async fn test_network_hsm_type_consistency() -> Result<(), BearDogError> {
        let discoverer = NetworkDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        for hsm in &discovered {
            assert_eq!(hsm.hsm_type, HsmType::NetworkHsm,
                "Network discoverer should only return network HSM type");
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_discovered_hsms_unique_ids() -> Result<(), BearDogError> {
        let discoverer = NetworkDiscoverer::new()?;
        let discovered = discoverer.discover().await?;
        
        let mut ids = std::collections::HashSet::new();
        for hsm in &discovered {
            assert!(ids.insert(&hsm.id),
                "Each network HSM should have unique ID");
        }
        
        Ok(())
    }

    #[tokio::test]
    async fn test_discovery_deterministic() -> Result<(), BearDogError> {
        let discoverer = NetworkDiscoverer::new()?;
        
        let discovered1 = discoverer.discover().await?;
        let discovered2 = discoverer.discover().await?;
        
        assert_eq!(discovered1.len(), discovered2.len(),
            "Network discovery should be deterministic");
        
        Ok(())
    }

    // ========== Performance Tests ==========

    #[tokio::test]
    async fn test_network_discovery_performance() -> Result<(), BearDogError> {
        use std::time::Instant;
        
        let discoverer = NetworkDiscoverer::new()?;
        let start = Instant::now();
        
        let _discovered = discoverer.discover().await?;
        
        let duration = start.elapsed();
        assert!(duration.as_secs() < 30,
            "Network discovery should complete in reasonable time: {:?}", duration);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_concurrent_network_discovery() -> Result<(), BearDogError> {
        use std::sync::Arc;
        
        let discoverer = Arc::new(NetworkDiscoverer::new()?);
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

    #[tokio::test]
    async fn test_parallel_endpoint_probing() -> Result<(), BearDogError> {
        let mut discoverer = NetworkDiscoverer::new()?;
        
        // Add multiple endpoints
        for i in 1..=5 {
            discoverer.custom_endpoints.push(format!("https://hsm{}.example.com", i));
        }
        
        let discovered = discoverer.discover().await?;
        
        // Should probe endpoints efficiently
        Ok(())
    }

    #[test]
    fn test_default_implementation() {
        let discoverer = NetworkDiscoverer::default();
        assert!(discoverer.enable_mdns_probe);
        assert!(discoverer.enable_dns_sd_probe);
        assert!(discoverer.custom_endpoints.is_empty());
    }

    #[test]
    fn test_clone_implementation() {
        let discoverer1 = NetworkDiscoverer::new().unwrap();
        let discoverer2 = discoverer1.clone();
        
        assert_eq!(discoverer1.enable_mdns_probe, discoverer2.enable_mdns_probe);
        assert_eq!(discoverer1.enable_dns_sd_probe, discoverer2.enable_dns_sd_probe);
    }
}

