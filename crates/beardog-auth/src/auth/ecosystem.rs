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
    #[must_use]
    pub fn get_ecosystem_capabilities(&self, node_id: &str) -> Vec<NodeCapability> {
        match self.node_registry.get_node_info(node_id) {
            Ok(info) => {
                Self::merge_capabilities(info.capabilities, self.security_policy_capabilities())
            }
            Err(_) => self.default_ecosystem_capabilities_fallback(),
        }
    }

    /// [`CapabilityMetadata`] aligned with [`Self::discover_node_capabilities`] for the same `node_id`.
    #[must_use]
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
#[path = "ecosystem_tests.rs"]
mod tests;
