// SPDX-License-Identifier: AGPL-3.0-or-later

//! Ecosystem capabilities implementation
//!
//! Node capability lists are sourced from the [`NodeRegistry`] when available, augmented with
//! policy from [`CrossNodeAuthConfig`]. Discovery heuristics stay aligned with
//! [`beardog_capabilities::CapabilityMetadata`] ids used for runtime advertisement.

use super::types::{CrossNodeAuthEngine, NodeCapability, SpawningMode, VerificationMode};
use beardog_capabilities::CapabilityMetadata;
use beardog_errors::BearDogError;

#[derive(Clone, Copy)]
enum DiscoveredNodeKind {
    Hsm,
    Storage,
    Generic,
}

fn classify_node_id(node_id: &str) -> DiscoveredNodeKind {
    if node_id.starts_with("hsm_") {
        DiscoveredNodeKind::Hsm
    } else if node_id.starts_with("storage_") {
        DiscoveredNodeKind::Storage
    } else {
        DiscoveredNodeKind::Generic
    }
}

impl CrossNodeAuthEngine {
    fn default_ecosystem_capabilities_fallback(&self) -> Vec<NodeCapability> {
        vec![
            NodeCapability::EncryptionStrength(256),
            NodeCapability::SecurityAnalysis,
        ]
    }

    /// Capabilities implied by static security policy (proof verification, consensus, spawning).
    fn security_policy_capabilities(&self) -> Vec<NodeCapability> {
        let mut caps = Vec::new();
        if self.config.consensus_config.required {
            caps.push(NodeCapability::DistributedConsensus);
        }
        if self.config.verification_mode == VerificationMode::Enabled {
            caps.push(NodeCapability::CryptographicAuditing);
        }
        if self.config.spawning_mode == SpawningMode::Enabled {
            caps.push(NodeCapability::ComputeCapable);
        }
        caps
    }

    fn merge_capabilities(
        mut base: Vec<NodeCapability>,
        extra: Vec<NodeCapability>,
    ) -> Vec<NodeCapability> {
        base.extend(extra);
        base.sort_by(|a, b| format!("{a:?}").cmp(&format!("{b:?}")));
        base.dedup_by(|a, b| a == b);
        base
    }

    /// Get ecosystem capabilities for a node
    pub fn get_ecosystem_capabilities(&self, node_id: &str) -> Vec<NodeCapability> {
        match self.node_registry.get_node_info(node_id) {
            Ok(info) => {
                Self::merge_capabilities(info.capabilities, self.security_policy_capabilities())
            }
            Err(_) => self.default_ecosystem_capabilities_fallback(),
        }
    }

    /// [`CapabilityMetadata`] aligned with [`Self::discover_node_capabilities`] for the same `node_id`.
    pub fn ecosystem_capability_metadata(&self, node_id: &str) -> Vec<CapabilityMetadata> {
        match classify_node_id(node_id) {
            DiscoveredNodeKind::Hsm => vec![
                CapabilityMetadata::new("hsm_operations", "1.0").with_interface("HsmOperations"),
                CapabilityMetadata::new("key_generation", "1.0")
                    .with_interface("KeyDerivationProvider"),
                CapabilityMetadata::new("digital_signing", "1.0")
                    .with_interface("LineageSigningProvider"),
            ],
            DiscoveredNodeKind::Storage => vec![
                CapabilityMetadata::new("storage_provider", "1.0")
                    .with_interface("StorageProvider"),
                CapabilityMetadata::new("data_storage", "1.0").with_interface("DataStorage"),
                CapabilityMetadata::new("data_retrieval", "1.0").with_interface("DataRetrieval"),
            ],
            DiscoveredNodeKind::Generic => vec![
                CapabilityMetadata::new("basic_operations", "1.0")
                    .with_interface("BasicOperations"),
                CapabilityMetadata::new("network_communication", "1.0")
                    .with_interface("NetworkCommunication"),
            ],
        }
    }

    /// Discover node capabilities
    ///
    /// # Errors
    ///
    /// Currently always succeeds; the `Result` type is reserved for future discovery failures.
    pub fn discover_node_capabilities(
        &self,
        node_id: &str,
    ) -> Result<Vec<NodeCapability>, BearDogError> {
        let caps = match classify_node_id(node_id) {
            DiscoveredNodeKind::Hsm => vec![
                NodeCapability::HsmOperations,
                NodeCapability::KeyGeneration,
                NodeCapability::DigitalSigning,
            ],
            DiscoveredNodeKind::Storage => vec![
                NodeCapability::StorageProvider,
                NodeCapability::DataStorage,
                NodeCapability::DataRetrieval,
            ],
            DiscoveredNodeKind::Generic => vec![
                NodeCapability::BasicOperations,
                NodeCapability::NetworkCommunication,
            ],
        };
        debug_assert_eq!(
            caps.len(),
            self.ecosystem_capability_metadata(node_id).len()
        );
        Ok(caps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::node_registry::InMemoryNodeRegistry;
    use crate::auth::proof_verifier::PlaceholderProofVerifier;
    use crate::auth::types::NodeInfo;
    use crate::auth::types::authorization::{CrossNodeAuthConfig, SpawningMode, VerificationMode};
    use crate::auth::types::node_registry::NodeRegistry;
    use chrono::Utc;

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
        let capabilities = result.expect("discover_node_capabilities hsm_node_1");
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
        let capabilities = result.expect("discover_node_capabilities storage_node_1");
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
        let capabilities = result.expect("discover_node_capabilities generic_node_1");
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
        let capabilities = result.expect("discover_node_capabilities empty id");
        assert!(
            !capabilities.is_empty(),
            "Should return at least basic capabilities"
        );
    }

    #[test]
    fn test_discover_node_capabilities_prefix_matching() {
        let engine = CrossNodeAuthEngine::default();

        // Test HSM prefix matching
        let hsm_caps = engine
            .discover_node_capabilities("hsm_xyz_123")
            .expect("discover hsm_xyz_123");
        assert!(
            hsm_caps.contains(&NodeCapability::HsmOperations),
            "hsm_ prefix should trigger HSM capabilities"
        );

        // Test storage prefix matching
        let storage_caps = engine
            .discover_node_capabilities("storage_abc_456")
            .expect("discover storage_abc_456");
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

    #[test]
    fn test_get_ecosystem_capabilities_merges_registry_and_policy() {
        let mut registry = InMemoryNodeRegistry::new();
        let node_info = NodeInfo {
            node_id: "registered-node".to_string(),
            address: "127.0.0.1:1".to_string(),
            capabilities: vec![
                NodeCapability::ThreatDetection,
                NodeCapability::ThreatDetection,
            ],
            trust_level: 1.0,
            last_seen: Utc::now(),
            genetics: None,
        };
        registry
            .register_node(node_info)
            .expect("register_node registered-node");

        let mut config = CrossNodeAuthConfig::default();
        config.consensus_config.required = true;
        config.verification_mode = VerificationMode::Disabled;
        config.spawning_mode = SpawningMode::Disabled;

        let engine = CrossNodeAuthEngine::new(
            Box::new(registry),
            Box::new(PlaceholderProofVerifier::new()),
            config,
        );
        let caps = engine.get_ecosystem_capabilities("registered-node");
        assert!(caps.contains(&NodeCapability::DistributedConsensus));
        assert!(caps.contains(&NodeCapability::ThreatDetection));
        assert!(!caps.contains(&NodeCapability::CryptographicAuditing));
        assert!(!caps.contains(&NodeCapability::ComputeCapable));
    }

    #[test]
    fn test_ecosystem_capability_metadata_hsm_storage_generic() {
        let engine = CrossNodeAuthEngine::default();
        let h = engine.ecosystem_capability_metadata("hsm_alpha");
        assert_eq!(h.len(), 3);
        assert!(h.iter().any(|m| m.id == "hsm_operations"));

        let s = engine.ecosystem_capability_metadata("storage_blob_1");
        assert_eq!(s.len(), 3);
        assert!(s.iter().any(|m| m.id == "storage_provider"));

        let g = engine.ecosystem_capability_metadata("other");
        assert_eq!(g.len(), 2);
        assert!(g.iter().any(|m| m.id == "basic_operations"));
    }

    #[test]
    fn test_ecosystem_capability_metadata_hsm_prefix_before_storage_substring() {
        let engine = CrossNodeAuthEngine::default();
        let m = engine.ecosystem_capability_metadata("hsm_storage_like");
        assert!(m.iter().any(|x| x.id == "hsm_operations"));
    }

    #[test]
    fn test_classify_storage_prefix_before_hsm_substring() {
        let engine = CrossNodeAuthEngine::default();
        let caps = engine
            .discover_node_capabilities("storage_hsm_like")
            .expect("discover storage_hsm_like");
        assert!(caps.contains(&NodeCapability::StorageProvider));
    }

    #[test]
    fn test_get_ecosystem_capabilities_merges_full_security_policy() {
        let mut registry = InMemoryNodeRegistry::new();
        let node_info = NodeInfo {
            node_id: "policy-node".to_string(),
            address: "127.0.0.1:2".to_string(),
            capabilities: vec![
                NodeCapability::ThreatDetection,
                NodeCapability::ThreatDetection,
            ],
            trust_level: 0.9,
            last_seen: Utc::now(),
            genetics: None,
        };
        registry
            .register_node(node_info)
            .expect("register_node policy-node");

        let mut config = CrossNodeAuthConfig::default();
        config.consensus_config.required = true;
        config.verification_mode = VerificationMode::Enabled;
        config.spawning_mode = SpawningMode::Enabled;

        let engine = CrossNodeAuthEngine::new(
            Box::new(registry),
            Box::new(PlaceholderProofVerifier::new()),
            config,
        );
        let caps = engine.get_ecosystem_capabilities("policy-node");
        assert!(caps.contains(&NodeCapability::DistributedConsensus));
        assert!(caps.contains(&NodeCapability::CryptographicAuditing));
        assert!(caps.contains(&NodeCapability::ComputeCapable));
        assert!(caps.contains(&NodeCapability::ThreatDetection));
        let threat_count = caps
            .iter()
            .filter(|c| **c == NodeCapability::ThreatDetection)
            .count();
        assert_eq!(
            threat_count, 1,
            "merge should dedupe identical capabilities"
        );
    }

    #[test]
    fn test_security_policy_capabilities_when_consensus_only() {
        let mut registry = InMemoryNodeRegistry::new();
        let node_info = NodeInfo {
            node_id: "consensus-only".to_string(),
            address: "127.0.0.1:3".to_string(),
            capabilities: vec![],
            trust_level: 1.0,
            last_seen: Utc::now(),
            genetics: None,
        };
        registry
            .register_node(node_info)
            .expect("register_node consensus-only");

        let mut config = CrossNodeAuthConfig::default();
        config.consensus_config.required = true;
        config.verification_mode = VerificationMode::Disabled;
        config.spawning_mode = SpawningMode::Disabled;

        let engine = CrossNodeAuthEngine::new(
            Box::new(registry),
            Box::new(PlaceholderProofVerifier::new()),
            config,
        );
        let caps = engine.get_ecosystem_capabilities("consensus-only");
        assert!(caps.contains(&NodeCapability::DistributedConsensus));
        assert!(!caps.contains(&NodeCapability::CryptographicAuditing));
        assert!(!caps.contains(&NodeCapability::ComputeCapable));
    }
}
