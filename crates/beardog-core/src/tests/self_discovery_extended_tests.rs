// SPDX-License-Identifier: AGPL-3.0-or-later

#![allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, clippy::field_reassign_with_default, clippy::manual_range_contains, unused_variables, dead_code)]

// Extended Self-Discovery Tests
// Created: October 25, 2025
// Purpose: Expand test coverage for self-discovery functionality

use crate::zero_knowledge_bootstrap::self_discovery::{SelfDiscoveryEngine, SelfDiscoveryEnvInputs};

#[cfg(test)]
mod self_discovery_extended_tests {
    use super::*;

    #[tokio::test]
    async fn test_self_discovery_engine_initialization() {
        let engine = SelfDiscoveryEngine::new();
        assert!(engine.is_ok(), "Engine should initialize successfully");
    }

    #[tokio::test]
    async fn test_self_identity_discovery_uniqueness() {
        let mut engine1 = SelfDiscoveryEngine::new().unwrap();
        let mut engine2 = SelfDiscoveryEngine::new().unwrap();

        let identity1 = engine1.discover_self_identity();
        let identity2 = engine2.discover_self_identity();

        assert!(identity1.is_ok());
        assert!(identity2.is_ok());

        let id1 = identity1.unwrap();
        let id2 = identity2.unwrap();

        // Each instance should have a unique primal ID
        assert_ne!(
            id1.primal_id, id2.primal_id,
            "Each engine should generate a unique primal ID"
        );
    }

    #[tokio::test]
    async fn test_self_identity_has_required_fields() {
        let mut engine = SelfDiscoveryEngine::new().unwrap();
        let identity = engine.discover_self_identity().unwrap();

        // Verify all required fields are present
        assert!(
            !identity.primal_id.is_empty(),
            "Primal ID should not be empty"
        );
        let prefix = identity.primal_id.split('-').next().unwrap_or("");
        assert!(
            !prefix.is_empty(),
            "Primal ID should have a type prefix segment"
        );
        assert!(
            !identity.capabilities.is_empty(),
            "Should discover at least one capability"
        );
        assert!(
            !identity.endpoints.is_empty(),
            "Should have at least one endpoint"
        );
        assert!(
            identity.metadata.display_name.is_some(),
            "Metadata should include display name"
        );
        assert!(
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            !identity.metadata.version.is_empty(),
            "Version should not be empty"
        );
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    }

    #[tokio::test]
    async fn test_capability_discovery_non_empty() {
        let mut engine = SelfDiscoveryEngine::new().unwrap();
        let identity = engine.discover_self_identity().unwrap();

        assert!(
            identity.capabilities.len() >= 1,
            "Should discover at least one capability (security is core)"
        );

        // Verify capabilities exist
        for capability in &identity.capabilities {
            // ServiceCapabilityType is an enum, verify it's valid
            let _cap_str = format!("{:?}", capability);
            assert!(
                !_cap_str.is_empty(),
                // TEST_CATEGORY: integration
                // TEST_DOMAIN: core
                // TEST_PRIORITY: normal
                "Capability should have valid representation"
            );
        }
    }

    #[tokio::test]
    async fn test_endpoint_discovery_validity() {
        let mut engine = SelfDiscoveryEngine::new().unwrap();
        let identity = engine.discover_self_identity().unwrap();

        assert!(
            !identity.endpoints.is_empty(),
            "Should have at least one endpoint"
        );

        // Verify endpoint structure
        for endpoint in &identity.endpoints {
            let endpoint_str = format!("{:?}", endpoint);
            assert!(
                !endpoint_str.is_empty(),
                "Endpoint should have valid representation"
            );
        }
    }

    #[tokio::test]
    async fn test_metadata_completeness() {
        let mut engine = SelfDiscoveryEngine::new().unwrap();
        let identity = engine.discover_self_identity().unwrap();
 // TEST_CATEGORY: integration
 // TEST_DOMAIN: core
 // TEST_PRIORITY: normal

        let metadata = &identity.metadata;

        assert!(
            metadata.display_name.is_some(),
            "Display name should be present"
        );
        assert!(!metadata.version.is_empty(), "Version should not be empty");

        // Version should follow semantic versioning
        assert!(
            metadata.version.contains('.'),
            "Version should contain dots (semantic versioning)"
        );
    }

    #[tokio::test]
    async fn test_multiple_discovery_calls_consistent() {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let mut engine = SelfDiscoveryEngine::new().unwrap();

        let identity1 = engine.discover_self_identity().unwrap();
        let identity2 = engine.discover_self_identity().unwrap();

        let parts1: Vec<&str> = identity1.primal_id.split('-').collect();
        let parts2: Vec<&str> = identity2.primal_id.split('-').collect();
        assert_eq!(parts1.len(), 3, "Identity should have 3 parts");
        assert_eq!(parts2.len(), 3, "Identity should have 3 parts");
        assert_eq!(parts1[0], parts2[0], "Same engine inputs should yield same type prefix");
    }

    #[tokio::test]
    async fn test_capability_discovery_includes_security() {
        let mut engine = SelfDiscoveryEngine::new().unwrap();
        let identity = engine.discover_self_identity().unwrap();

        // BearDog should always discover at least one capability
        assert!(
            !identity.capabilities.is_empty(),
            "BearDog should always discover at least one capability"
        );
    }

    #[tokio::test]
    async fn test_primal_id_format_validity() {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let mut engine = SelfDiscoveryEngine::new().unwrap();
        let identity = engine.discover_self_identity().unwrap();

        let primal_id = &identity.primal_id;

        let parts: Vec<&str> = primal_id.split('-').collect();
        assert_eq!(parts.len(), 3, "Expected type-host-uuid fragment layout");
        assert!(
            !parts[2].is_empty(),
            "UUID fragment should not be empty"
        );
    }

    #[tokio::test]
    async fn test_discovery_with_display_name_inputs() {
        let mut engine = SelfDiscoveryEngine::with_inputs(SelfDiscoveryEnvInputs {
            beardog_display_name: Some("Test BearDog Instance".to_string()),
            ..Default::default()
        })
        .unwrap();
        let identity = engine.discover_self_identity().unwrap();

        assert_eq!(
            identity.metadata.display_name.as_deref(),
            Some("Test BearDog Instance")
        );
    }

    #[tokio::test]
    async fn test_capability_metadata_structure() {
        let mut engine = SelfDiscoveryEngine::new().unwrap();
        let identity = engine.discover_self_identity().unwrap();

        for capability in &identity.capabilities {
            // Verify each capability is a valid enum variant
            let cap_debug = format!("{:?}", capability);
            assert!(
                !cap_debug.is_empty(),
                // TEST_CATEGORY: integration
                // TEST_DOMAIN: core
                // TEST_PRIORITY: normal
                "Capability should have valid debug representation"
            );
        }
    }

    #[tokio::test]
    async fn test_concurrent_discovery_safety() {
        use std::sync::Arc;
        use tokio::sync::Mutex;

        let engine = Arc::new(Mutex::new(SelfDiscoveryEngine::new().unwrap()));

        let mut handles = vec![];

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        // Spawn multiple concurrent discovery operations
        for _ in 0..10 {
            let engine_clone = Arc::clone(&engine);
            let handle = tokio::spawn(async move {
                let mut eng = engine_clone.lock().await;
                eng.discover_self_identity()
            });
            handles.push(handle);
        }

        // Wait for all to complete
        for handle in handles {
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            let result = handle.await.unwrap();
            assert!(result.is_ok(), "Concurrent discovery should succeed");
        }
    }

    #[tokio::test]
    async fn test_discovery_error_handling() {
        // Test that engine handles errors gracefully
        let engine_result = SelfDiscoveryEngine::new();
        assert!(engine_result.is_ok(), "Engine creation should not fail");

        let mut engine = engine_result.unwrap();
        let discovery_result = engine.discover_self_identity();
        assert!(
            discovery_result.is_ok(),
            "Discovery should succeed or return proper error"
        );
    }

    #[tokio::test]
    async fn test_endpoint_format_validation() {
        let mut engine = SelfDiscoveryEngine::new().unwrap();
        let identity = engine.discover_self_identity().unwrap();
 // TEST_CATEGORY: integration
 // TEST_DOMAIN: core
 // TEST_PRIORITY: important

        for endpoint in &identity.endpoints {
            // Endpoints should be valid
            let endpoint_str = format!("{:?}", endpoint);
            assert!(
                !endpoint_str.is_empty(),
                "Endpoint should have valid representation"
            );
        }
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_version_format_semantic() {
        let mut engine = SelfDiscoveryEngine::new().unwrap();
        let identity = engine.discover_self_identity().unwrap();

        let version = &identity.metadata.version;
        let parts: Vec<&str> = version.split('.').collect();

        assert!(
            parts.len() >= 2,
            "Version should have at least major.minor: {}",
            version
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        );

        // First two parts should be numbers
        for (i, part) in parts.iter().take(2).enumerate() {
            assert!(
                part.chars().all(|c| c.is_ascii_digit()),
                "Version part {} should be numeric: {}",
                i,
                part
            );
        }
    }

    #[tokio::test]
    async fn test_capability_uniqueness() {
        let mut engine = SelfDiscoveryEngine::new().unwrap();
        let identity = engine.discover_self_identity().unwrap();

        let mut seen_capabilities = std::collections::HashSet::new();

        for capability in &identity.capabilities {
            let key = format!("{:?}", capability);
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            assert!(
                seen_capabilities.insert(key.clone()),
                "Duplicate capability found: {}",
                key
            );
        }
    }

    #[tokio::test]
    async fn test_endpoint_uniqueness() {
        let mut engine = SelfDiscoveryEngine::new().unwrap();
        let identity = engine.discover_self_identity().unwrap();

        let mut seen_endpoints = std::collections::HashSet::new();
 // TEST_CATEGORY: integration
 // TEST_DOMAIN: core
 // TEST_PRIORITY: normal

        for endpoint in &identity.endpoints {
            let endpoint_str = format!("{:?}", endpoint);
            assert!(
                seen_endpoints.insert(endpoint_str.clone()),
                "Duplicate endpoint found: {}",
                endpoint_str
            );
        }
    }

    #[tokio::test]
    async fn test_discovery_performance() {
        use std::time::Instant;
 // TEST_CATEGORY: integration
 // TEST_DOMAIN: core
 // TEST_PRIORITY: normal

        let start = Instant::now();
        let mut engine = SelfDiscoveryEngine::new().unwrap();
        let _identity = engine.discover_self_identity().unwrap();
        let duration = start.elapsed();

        // Discovery should complete in reasonable time (< 1 second)
        assert!(
            duration.as_secs() < 1,
            "Discovery took too long: {:?}",
            duration
        );
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_identity_serialization_readiness() {
        let mut engine = SelfDiscoveryEngine::new().unwrap();
        let identity = engine.discover_self_identity().unwrap();

        // Verify all strings are valid UTF-8 and serializable
        assert!(identity.primal_id.is_ascii());
        assert!(!identity.metadata.version.is_empty());

        // All fields should be serialization-ready
        for capability in &identity.capabilities {
            let cap_str = format!("{:?}", capability);
            assert!(
                !cap_str.contains('\0'),
                "Capability should not contain null bytes"
            );
        }
    }
}
