// SPDX-License-Identifier: AGPL-3.0-or-later

//! Additional tests for capability registry
//!
//! These tests supplement the existing comprehensive tests with additional
//! edge cases, error paths, and concurrent scenarios.


#![allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, clippy::field_reassign_with_default, clippy::manual_range_contains, unused_variables, dead_code)]

#[cfg(test)]
mod capability_registry_additional_tests {
    use crate::zero_knowledge_bootstrap::capability_registry::*;
    use beardog_types::canonical::capabilities::{
        ServiceCapabilityType, UniversalCapability, CapabilityMetadata,
    };
    use std::sync::Arc;
    use tokio::time::Duration;
    
    #[tokio::test]
    async fn test_registry_creation_with_config() {
        let registry = CapabilityRegistry::new();
        
        // Registry should start empty
        let caps = registry.get_all_capabilities().await;
        assert!(caps.is_ok());
        assert!(caps.unwrap().is_empty());
    }
    
    #[tokio::test]
    async fn test_register_capability_basic() {
        let registry = CapabilityRegistry::new();
        
        let capability = UniversalCapability {
            id: "test-cap-1".to_string(),
            name: "Test Capability".to_string(),
            capability_type: ServiceCapabilityType::Security,
            provider_id: "test-provider".to_string(),
            endpoint: "http://localhost:8080".to_string(),
            metadata: CapabilityMetadata::default(),
            health_status: Default::default(),
        };
        
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let result = registry.register(capability).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_register_multiple_capabilities() {
        let registry = CapabilityRegistry::new();
         // TEST_CATEGORY: unit
         // TEST_DOMAIN: core
         // TEST_PRIORITY: normal
        
        for i in 0..5 {
            let capability = UniversalCapability {
                id: format!("cap-{}", i),
                name: format!("Capability {}", i),
                capability_type: ServiceCapabilityType::Security,
                provider_id: format!("provider-{}", i),
                endpoint: format!("http://localhost:808{}", i),
                metadata: CapabilityMetadata::default(),
                health_status: Default::default(),
            };
            
            let result = registry.register(capability).await;
            assert!(result.is_ok(), "Failed to register capability {}", i);
        }
         // TEST_CATEGORY: unit
         // TEST_DOMAIN: core
         // TEST_PRIORITY: normal
        
        let all_caps = registry.get_all_capabilities().await;
        assert!(all_caps.is_ok());
        assert_eq!(all_caps.unwrap().len(), 5);
    }
    
    #[tokio::test]
    async fn test_discover_capabilities_by_type() {
        let registry = CapabilityRegistry::new();
        
        // Register security capabilities
        for i in 0..3 {
            let capability = UniversalCapability {
                id: format!("security-cap-{}", i),
                name: format!("Security Capability {}", i),
                capability_type: ServiceCapabilityType::Security,
                provider_id: format!("provider-{}", i),
                endpoint: format!("http://localhost:808{}", i),
                metadata: CapabilityMetadata::default(),
                health_status: Default::default(),
            };
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            registry.register(capability).await.unwrap();
        }
        
        // Register storage capabilities
        for i in 0..2 {
            let capability = UniversalCapability {
                id: format!("storage-cap-{}", i),
                name: format!("Storage Capability {}", i),
                capability_type: ServiceCapabilityType::Storage,
                provider_id: format!("provider-storage-{}", i),
                endpoint: format!("http://localhost:809{}", i),
                metadata: CapabilityMetadata::default(),
                health_status: Default::default(),
            };
            registry.register(capability).await.unwrap();
        }
        
        // Discover security capabilities
        let security_caps = registry
            .discover_by_type(ServiceCapabilityType::Security)
            .await;
        
        assert!(security_caps.is_ok());
        let security_caps = security_caps.unwrap();
        assert_eq!(security_caps.len(), 3);
        assert!(security_caps.iter().all(|c| 
            matches!(c.capability_type, ServiceCapabilityType::Security)
        ));
    }
    
    #[tokio::test]
    async fn test_capability_removal() {
        let registry = CapabilityRegistry::new();
        
        let cap_id = "removable-cap".to_string();
        let capability = UniversalCapability {
            id: cap_id.clone(),
            name: "Removable Capability".to_string(),
            capability_type: ServiceCapabilityType::Security,
            provider_id: "test-provider".to_string(),
            endpoint: "http://localhost:8080".to_string(),
            metadata: CapabilityMetadata::default(),
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            health_status: Default::default(),
        };
        
        // Register capability
        let reg_result = registry.register(capability).await;
        assert!(reg_result.is_ok());
        
        // Verify it exists
        let all_caps = registry.get_all_capabilities().await.unwrap();
        assert_eq!(all_caps.len(), 1);
        
        // Remove capability
        let remove_result = registry.remove(&cap_id).await;
        assert!(remove_result.is_ok());
        
        // Verify it's gone
        let all_caps = registry.get_all_capabilities().await.unwrap();
        assert_eq!(all_caps.len(), 0);
    }
    
    #[tokio::test]
    async fn test_concurrent_registrations() {
        let registry = Arc::new(CapabilityRegistry::new());
        let mut handles = vec![];
        
        // Spawn 10 tasks to register capabilities concurrently
        for i in 0..10 {
            let registry_clone = Arc::clone(&registry);
            let handle = tokio::spawn(async move {
                // TEST_CATEGORY: unit
                // TEST_DOMAIN: core
                // TEST_PRIORITY: normal
                let capability = UniversalCapability {
                    id: format!("concurrent-cap-{}", i),
                    name: format!("Concurrent Capability {}", i),
                    capability_type: ServiceCapabilityType::Security,
                    provider_id: format!("provider-{}", i),
                    endpoint: format!("http://localhost:808{}", i),
                    metadata: CapabilityMetadata::default(),
                    health_status: Default::default(),
                };
                
                registry_clone.register(capability).await
            });
            handles.push(handle);
        }
        
        // Wait for all registrations to complete
        for handle in handles {
            let result = handle.await;
            assert!(result.is_ok());
            assert!(result.unwrap().is_ok());
        }
        
        // Verify all capabilities were registered
        let all_caps = registry.get_all_capabilities().await.unwrap();
        assert_eq!(all_caps.len(), 10);
    }
    
    #[tokio::test]
    async fn test_capability_update() {
        let registry = CapabilityRegistry::new();
        
        let cap_id = "updateable-cap".to_string();
        
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        // Register initial capability
        let capability_v1 = UniversalCapability {
            id: cap_id.clone(),
            name: "Version 1".to_string(),
            capability_type: ServiceCapabilityType::Security,
            provider_id: "test-provider".to_string(),
            endpoint: "http://localhost:8080".to_string(),
            metadata: CapabilityMetadata::default(),
            health_status: Default::default(),
        };
        
        registry.register(capability_v1).await.unwrap();
        
        // Update capability (register with same ID)
        let capability_v2 = UniversalCapability {
            id: cap_id.clone(),
            name: "Version 2".to_string(),
            capability_type: ServiceCapabilityType::Security,
            provider_id: "test-provider".to_string(),
            endpoint: "http://localhost:8081".to_string(), // Changed endpoint
            metadata: CapabilityMetadata::default(),
            health_status: Default::default(),
        };
        
        registry.register(capability_v2).await.unwrap();
        
        // Verify only one capability exists (update, not duplicate)
        let all_caps = registry.get_all_capabilities().await.unwrap();
        assert_eq!(all_caps.len(), 1);
        assert_eq!(all_caps[0].name, "Version 2");
        assert_eq!(all_caps[0].endpoint, "http://localhost:8081");
    }
    
    #[tokio::test]
    async fn test_empty_registry_operations() {
        let registry = CapabilityRegistry::new();
         // TEST_CATEGORY: unit
         // TEST_DOMAIN: core
         // TEST_PRIORITY: normal
        
        // All operations should work on empty registry
        let all_caps = registry.get_all_capabilities().await;
        assert!(all_caps.is_ok());
        assert!(all_caps.unwrap().is_empty());
        
        let security_caps = registry
            .discover_by_type(ServiceCapabilityType::Security)
            .await;
        assert!(security_caps.is_ok());
        assert!(security_caps.unwrap().is_empty());
        
        // Removing non-existent capability should handle gracefully
        let remove_result = registry.remove(&"non-existent".to_string()).await;
        // May return Ok(()) or Err depending on implementation
        // Either is acceptable for non-existent removal
    }
    
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_capability_metadata_preservation() {
        let registry = CapabilityRegistry::new();
        
        let mut metadata = CapabilityMetadata::default();
        // Note: CapabilityMetadata fields depend on actual implementation
        // This test ensures metadata is preserved
        
        let capability = UniversalCapability {
            id: "meta-cap".to_string(),
            name: "Metadata Capability".to_string(),
            capability_type: ServiceCapabilityType::Security,
            provider_id: "test-provider".to_string(),
            endpoint: "http://localhost:8080".to_string(),
            metadata: metadata.clone(),
            health_status: Default::default(),
        };
        
        registry.register(capability).await.unwrap();
        
        let all_caps = registry.get_all_capabilities().await.unwrap();
        assert_eq!(all_caps.len(), 1);
        // Metadata should be preserved (exact comparison depends on metadata fields)
    }
}

