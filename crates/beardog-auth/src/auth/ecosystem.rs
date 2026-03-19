// SPDX-License-Identifier: AGPL-3.0-only

//! Ecosystem capabilities implementation

use super::types::*;
use beardog_errors::BearDogError;

impl CrossNodeAuthEngine {
    /// Get ecosystem capabilities for a node
    pub fn get_ecosystem_capabilities(&self, _node_id: &str) -> Vec<NodeCapability> {
        // Stub implementation - would query actual node capabilities
        vec![
            NodeCapability::EncryptionStrength(256),
            NodeCapability::SecurityAnalysis,
        ]
    }

    /// Discover node capabilities
    pub fn discover_node_capabilities(
        &self,
        node_id: &str,
    ) -> Result<Vec<NodeCapability>, BearDogError> {
        // Stub implementation based on node ID prefix
        let capabilities = if node_id.starts_with("hsm_") {
            vec![
                NodeCapability::HsmOperations,
                NodeCapability::KeyGeneration,
                NodeCapability::DigitalSigning,
            ]
        } else if node_id.starts_with("storage_") {
            vec![
                NodeCapability::StorageProvider,
                NodeCapability::DataStorage,
                NodeCapability::DataRetrieval,
            ]
        } else {
            vec![
                NodeCapability::BasicOperations,
                NodeCapability::NetworkCommunication,
            ]
        };
        Ok(capabilities)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ecosystem_capabilities_returns_default_capabilities() {
        let engine = CrossNodeAuthEngine::default();
        let capabilities = engine.get_ecosystem_capabilities("test-node");

        assert_eq!(
            capabilities.len(),
            2,
            "Should return 2 default capabilities"
        );
        assert!(capabilities.contains(&NodeCapability::EncryptionStrength(256)));
        assert!(capabilities.contains(&NodeCapability::SecurityAnalysis));
    }

    #[test]
    fn test_get_ecosystem_capabilities_consistent() {
        let engine = CrossNodeAuthEngine::default();

        let caps1 = engine.get_ecosystem_capabilities("node1");
        let caps2 = engine.get_ecosystem_capabilities("node2");

        assert_eq!(
            caps1, caps2,
            "Should return consistent capabilities regardless of node ID"
        );
    }

    #[test]
    fn test_discover_node_capabilities_hsm_node() {
        let engine = CrossNodeAuthEngine::default();
        let result = engine.discover_node_capabilities("hsm_node_1");

        assert!(result.is_ok(), "HSM node discovery should succeed");
        let capabilities = result.unwrap();
        assert_eq!(capabilities.len(), 3, "HSM node should have 3 capabilities");
        assert!(capabilities.contains(&NodeCapability::HsmOperations));
        assert!(capabilities.contains(&NodeCapability::KeyGeneration));
        assert!(capabilities.contains(&NodeCapability::DigitalSigning));
    }

    #[test]
    fn test_discover_node_capabilities_storage_node() {
        let engine = CrossNodeAuthEngine::default();
        let result = engine.discover_node_capabilities("storage_node_1");

        assert!(result.is_ok(), "Storage node discovery should succeed");
        let capabilities = result.unwrap();
        assert_eq!(
            capabilities.len(),
            3,
            "Storage node should have 3 capabilities"
        );
        assert!(capabilities.contains(&NodeCapability::StorageProvider));
        assert!(capabilities.contains(&NodeCapability::DataStorage));
        assert!(capabilities.contains(&NodeCapability::DataRetrieval));
    }

    #[test]
    fn test_discover_node_capabilities_generic_node() {
        let engine = CrossNodeAuthEngine::default();
        let result = engine.discover_node_capabilities("generic_node_1");

        assert!(result.is_ok(), "Generic node discovery should succeed");
        let capabilities = result.unwrap();
        assert_eq!(
            capabilities.len(),
            2,
            "Generic node should have 2 capabilities"
        );
        assert!(capabilities.contains(&NodeCapability::BasicOperations));
        assert!(capabilities.contains(&NodeCapability::NetworkCommunication));
    }

    #[test]
    fn test_discover_node_capabilities_empty_node_id() {
        let engine = CrossNodeAuthEngine::default();
        let result = engine.discover_node_capabilities("");

        assert!(result.is_ok(), "Empty node ID should default to generic");
        let capabilities = result.unwrap();
        assert!(
            !capabilities.is_empty(),
            "Should return at least basic capabilities"
        );
    }

    #[test]
    fn test_discover_node_capabilities_prefix_matching() {
        let engine = CrossNodeAuthEngine::default();

        // Test HSM prefix matching
        let hsm_caps = engine.discover_node_capabilities("hsm_xyz_123").unwrap();
        assert!(
            hsm_caps.contains(&NodeCapability::HsmOperations),
            "hsm_ prefix should trigger HSM capabilities"
        );

        // Test storage prefix matching
        let storage_caps = engine
            .discover_node_capabilities("storage_abc_456")
            .unwrap();
        assert!(
            storage_caps.contains(&NodeCapability::StorageProvider),
            "storage_ prefix should trigger storage capabilities"
        );
    }

    #[test]
    fn test_ecosystem_capabilities_not_empty() {
        let engine = CrossNodeAuthEngine::default();
        let capabilities = engine.get_ecosystem_capabilities("any-node");

        assert!(
            !capabilities.is_empty(),
            "Ecosystem capabilities should never be empty"
        );
    }
}
