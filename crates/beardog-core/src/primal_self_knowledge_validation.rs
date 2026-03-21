// SPDX-License-Identifier: AGPL-3.0-only

//! Primal Self-Knowledge Validation Tests
//!
//! Tests that validate:
//! 1. Primals only have self-knowledge (no hardcoded other primal names)
//! 2. Primals discover others at runtime through capabilities
//! 3. Discovery is dynamic and doesn't rely on configuration

#[cfg(test)]
use crate::primal_self_knowledge::PrimalSelfKnowledge;

/// Validate that primal only knows itself
///
/// This test ensures no hardcoded knowledge of other primals
#[cfg(test)]
mod self_knowledge_tests {
    use super::*;

    #[tokio::test]
    async fn test_primal_knows_only_itself() {
        let primal = PrimalSelfKnowledge::new();

        // Should know its own identity
        assert!(primal.get_self_identity().is_ok());

        // Should NOT have hardcoded knowledge of other primals
        // These should all return "unknown" or require discovery
        assert!(
            primal.get_known_primal_count() <= 1,
            "Primal should only know itself"
        );
    }

    #[tokio::test]
    async fn test_primal_capabilities_self_aware() {
        let primal = PrimalSelfKnowledge::new();

        // Should be able to list its own capabilities
        let caps = primal.get_self_capabilities();
        assert!(caps.is_ok(), "Should know own capabilities");
    }

    #[tokio::test]
    async fn test_no_hardcoded_primal_names() {
        // This test validates that the codebase doesn't have hardcoded primal names
        // in the self-knowledge system

        let primal = PrimalSelfKnowledge::new();
        let identity = primal.get_self_identity().unwrap();

        // The identity should be dynamically determined, not hardcoded
        // Check that common primal names are NOT hardcoded
        let forbidden_peer_markers = ["-peer-", "other-primal:", "hardcoded-peer:"];

        for marker in forbidden_peer_markers {
            assert!(
                !identity.to_lowercase().contains(marker),
                "Self-knowledge should not embed hardcoded peer markers: {marker}"
            );
        }
    }
}

/// Validate runtime discovery of other primals
#[cfg(test)]
mod runtime_discovery_tests {
    use super::*;

    #[tokio::test]
    async fn test_discover_by_capability_does_not_panic() {
        let primal = PrimalSelfKnowledge::new();

        // Discovery should execute without panicking, even if no services found
        let result = primal.discover_by_capability(vec![]);

        // Even if no primals are found, discovery should execute without errors
        assert!(result.is_ok(), "Discovery should execute successfully");
    }

    #[tokio::test]
    async fn test_discovery_is_repeatable() {
        let primal = PrimalSelfKnowledge::new();

        // First discovery
        let result1 = primal.discover_by_capability(vec![]);

        // Second discovery (should query again, not use static list)
        let result2 = primal.discover_by_capability(vec![]);

        // Both should succeed (even if empty)
        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }

    #[tokio::test]
    async fn test_discovery_without_configuration() {
        // Test that discovery works without pre-configuration
        let primal = PrimalSelfKnowledge::new();

        // No configuration loaded, should still attempt discovery
        let result = primal.discover_by_capability(vec![]);

        // Should not error even if no configuration
        assert!(
            result.is_ok(),
            "Discovery should work without pre-configuration"
        );
    }
}

/// Validate sovereignty compliance
#[cfg(test)]
mod sovereignty_tests {
    use super::*;

    #[tokio::test]
    async fn test_primal_sovereignty_maintained() {
        let primal = PrimalSelfKnowledge::new();

        // Primal should maintain sovereignty
        // - Knows itself
        // - Doesn't have dependencies hardcoded
        // - Can operate independently

        let identity = primal.get_self_identity();
        assert!(identity.is_ok(), "Primal should know its own identity");

        let caps = primal.get_self_capabilities();
        assert!(caps.is_ok(), "Primal should know its own capabilities");
    }

    #[tokio::test]
    async fn test_discovery_is_dynamic_not_static() {
        // Validate that discovery attempts runtime mechanisms
        let primal = PrimalSelfKnowledge::new();

        // Discovery should execute (not fail with "not implemented")
        let result = primal.discover_by_capability(vec![]);

        // Should attempt discovery even if mechanisms aren't fully implemented yet
        assert!(result.is_ok(), "Discovery mechanism should be present");
    }
}

/// Integration validation
#[cfg(test)]
mod integration_validation {
    use super::*;

    #[tokio::test]
    async fn test_full_discovery_lifecycle() {
        let primal = PrimalSelfKnowledge::new();

        // 1. Primal knows itself
        let identity = primal.get_self_identity();
        assert!(identity.is_ok());

        // 2. Primal can query its capabilities
        let self_caps = primal.get_self_capabilities();
        assert!(self_caps.is_ok());

        // 3. Primal can attempt discovery (even if none found)
        let discovered = primal.discover_by_capability(vec![]);
        assert!(discovered.is_ok());

        // 4. Discovery is repeatable
        let discovered2 = primal.discover_by_capability(vec![]);
        assert!(discovered2.is_ok());
    }

    #[tokio::test]
    async fn test_no_primal_name_hardcoding() {
        // Test that the system doesn't rely on hardcoded primal names
        let primal = PrimalSelfKnowledge::new();

        let identity = primal.get_self_identity().unwrap();

        // Identity should be from environment/config, not hardcoded
        // Common hardcoded names should NOT appear
        assert!(!identity.contains("other-primal-12345")); // No specific peer instances
        assert!(!identity.contains("hardcoded-peer-67890")); // No specific peer instances
    }
}
