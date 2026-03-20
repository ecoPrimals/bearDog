// SPDX-License-Identifier: AGPL-3.0-only

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Policy bundle governing cross-node proof checks, spawning, and optional quorum approval.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeAuthConfig {
    /// Whether incoming proofs must be validated before honoring cross-node calls.
    pub verification_mode: VerificationMode,
    /// Maximum age of a cryptographic proof before it is rejected as stale.
    pub max_proof_validity_minutes: u32,
    /// Whether child primals may be spawned automatically or must remain disabled.
    pub spawning_mode: SpawningMode,
    /// Parameters for optional multi-node agreement on sensitive operations.
    pub consensus_config: ConsensusConfig,
    /// Number of `max_spawns_per_node`
    pub max_spawns_per_node: u32,
    /// The approval mode value
    pub approval_mode: ApprovalMode,
}

/// Tunable quorum rules applied when [`CrossNodeAuthConfig`] enables consensus-gated actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfig {
    /// When true, participating nodes must affirm high-risk operations before they proceed.
    pub required: bool,
    /// Fraction of votes (0.0–1.0) required to consider consensus satisfied.
    pub threshold: f64,
}

impl Default for CrossNodeAuthConfig {
    fn default() -> Self {
        Self {
            verification_mode: VerificationMode::Enabled,
            max_proof_validity_minutes: 60,
            spawning_mode: SpawningMode::Enabled,
            consensus_config: ConsensusConfig {
                required: false,
                threshold: 0.67,
            },
            max_spawns_per_node: 10,
            approval_mode: ApprovalMode::Automated,
        }
    }
}

/// Signed grant describing who may act on a remote resource and under which constraints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeAuthorization {
    /// Correlates logs, proofs, and workflow steps for this authorization request.
    pub request_id: String,
    /// Node initiating the cross-node operation.
    pub requester_node_id: String,
    /// Node that owns the target resource being accessed.
    pub resource_owner_node_id: String,
    /// Opaque resource identifier interpreted by the owner’s policy engine.
    pub resource_id: String,
    /// Collection of permissions
    pub permissions: Vec<ResourcePermission>,
    /// Collection of conditions
    pub conditions: Vec<AccessCondition>,
    /// The created at value
    pub created_at: DateTime<Utc>,
    /// The expires at value
    pub expires_at: DateTime<Utc>,
    /// The signature value
    pub signature: String,
    /// Whether `is_active` is enabled
    pub is_active: bool,
}

impl CrossNodeAuthorization {
    /// Is Valid operation.
    /// Checks if valid
    /// Checks if valid
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.is_active && Utc::now() < self.expires_at
    }

    /// Has Permission operation.
    /// Checks if permission
    /// Checks if permission
    #[must_use]
    pub fn has_permission(&self, permission: &ResourcePermission) -> bool {
        self.permissions.iter().any(|p| p.implies(permission))
    }
}

/// Fine-grained verbs that may appear on a [`CrossNodeAuthorization::permissions`] list.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResourcePermission {
    /// Represents read variant
    Read,

    /// Represents write variant
    Write,

    /// Represents delete variant
    Delete,

    /// Represents admin variant
    Admin,

    /// Represents execute variant
    Execute,

    /// Represents create variant
    Create,

    /// Represents share variant
    Share,

    /// Represents backup variant
    Backup,

    /// Represents restore variant
    Restore,

    /// Represents audit variant
    Audit,

    /// Represents replicate variant
    Replicate,

    /// Represents spawn variant
    Spawn,

    /// Represents genetic modify variant
    GeneticModify,

    /// Represents consensus variant
    Consensus,

    /// Represents compliance variant
    Compliance,
}

impl ResourcePermission {
    /// Returns true if possessing `self` is sufficient to satisfy a requirement for `other`.
    #[must_use]
    pub fn implies(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Admin, _) => true,
            (Self::Write, Self::Read) => true,
            (Self::Delete, Self::Write) => true,
            (Self::Spawn, Self::Create) => true,
            (a, b) => a == b,
        }
    }

    /// Relative sensitivity rank used for ordering and UI; higher means more privileged.
    #[must_use]
    pub const fn security_level(&self) -> u8 {
        match self {
            Self::Read => 1,
            Self::Audit => 2,
            Self::Backup => 3,
            Self::Execute => 4,
            Self::Create => 5,
            Self::Write => 6,
            Self::Share => 7,
            Self::Replicate => 8,
            Self::Restore => 9,
            Self::Consensus => 10,
            Self::Compliance => 11,
            Self::GeneticModify => 12,
            Self::Spawn => 13,
            Self::Delete => 14,
            Self::Admin => 15,
        }
    }
}

/// Additional predicates that must hold before an authorization is considered valid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessCondition {
    /// Access is only valid between `start` and `end` (inclusive/exclusive per enforcement layer).
    TimeWindow {
        /// Inclusive lower bound of the validity window.
        start: DateTime<Utc>,
        /// Exclusive or inclusive upper bound of the validity window.
        end: DateTime<Utc>,
    },
    /// Restricts callers to a specific IPv4 address observed within `window_seconds`.
    IpAddress {
        /// IPv4 address in host-endian `u32` form (e.g. `0x7F000001` for `127.0.0.1`).
        address: u32,
        /// Sliding window, in seconds, for recent IP verification.
        window_seconds: u32,
    },
    /// Requires a quorum of listed nodes to approve before the grant activates.
    RequireConsensus {
        /// Fraction of `nodes` that must approve (interpreted by the consensus engine).
        threshold: f64,
        /// Participant node identifiers eligible to vote.
        nodes: Vec<String>,
    },
}

/// Supported strong-authentication mechanisms when binding identities to authorizations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    /// Represents signature variant
    Signature,
    /// Represents certificate variant
    Certificate,
    /// Represents biometric hash variant
    BiometricHash,
    /// Represents mutual tls variant
    MutualTls,
}

/// Toggles whether cryptographic proof verification runs on the hot path.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerificationMode {
    /// Active or enabled state
    Enabled,
    /// Inactive or disabled state
    Disabled,
}

/// Enables or disables automatic primal spawning flows tied to authorization decisions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpawningMode {
    /// Active or enabled state
    Enabled,
    /// Inactive or disabled state
    Disabled,
}

/// Determines whether sensitive operations proceed automatically or await human approval.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApprovalMode {
    /// State indicating automated
    Automated,
    /// Represents manual variant
    Manual,
}

/// Verb applied to `target_resource` inside a [`CrossNodeOperation`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    /// Represents read variant
    Read,
    /// Represents write variant
    Write,
    /// Represents execute variant
    Execute,
    /// Represents delete variant
    Delete,
}

/// Concrete action a requester intends to perform against `target_resource`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeOperation {
    /// The operation type value
    pub operation_type: OperationType,
    /// The target resource value
    pub target_resource: String,
    /// Mapping of parameters
    pub parameters: HashMap<String, String>,
    /// The requester signature value
    pub requester_signature: String,
}

impl Default for CrossNodeOperation {
    fn default() -> Self {
        Self {
            operation_type: OperationType::Read,
            target_resource: String::new(),
            parameters: HashMap::new(),
            requester_signature: String::new(),
        }
    }
}

/// Proof object bound to a [`CrossNodeOperation`] prior to full authorization issuance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthProof {
    /// Identifier linking this proof to audit logs and replay caches.
    pub proof_id: String,
    /// The operation value
    pub operation: CrossNodeOperation,
    /// Time at which the proof was produced (used for freshness checks).
    pub timestamp: DateTime<Utc>,
    /// The proof signature value
    pub proof_signature: String,
}

/// Evidence that a specific [`CrossNodeAuthorization`] authorized `operation` at `timestamp`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationProof {
    /// Matches [`CrossNodeAuthorization::request_id`] or another stable authorization key.
    pub authorization_id: String,
    /// The operation value
    pub operation: CrossNodeOperation,
    /// Time at which this proof was issued; verifiers compare against max age policy.
    pub timestamp: DateTime<Utc>,
    /// The proof signature value
    pub proof_signature: String,
}

/// Outcome of a quorum vote among participating nodes for a gated operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusResult {
    /// Whether `consensus_reached` is enabled
    pub consensus_reached: bool,
    /// Mapping of votes
    pub votes: HashMap<String, bool>,
    /// The final score value
    pub final_score: f64,
    /// Collection of participating nodes
    pub participating_nodes: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ResourcePermission tests
    #[test]
    fn test_resource_permission_variants() {
        let permissions = [
            ResourcePermission::Read,
            ResourcePermission::Write,
            ResourcePermission::Delete,
            ResourcePermission::Admin,
            ResourcePermission::Execute,
        ];

        assert_eq!(permissions.len(), 5);
    }

    #[test]
    fn test_permission_implies_admin_all() {
        let admin = ResourcePermission::Admin;
        assert!(admin.implies(&ResourcePermission::Read));
        assert!(admin.implies(&ResourcePermission::Write));
        assert!(admin.implies(&ResourcePermission::Delete));
    }

    #[test]
    fn test_permission_security_levels() {
        assert_eq!(ResourcePermission::Read.security_level(), 1);
        assert_eq!(ResourcePermission::Admin.security_level(), 15);
        assert!(
            ResourcePermission::Admin.security_level() > ResourcePermission::Read.security_level()
        );
    }

    // AccessCondition tests
    #[test]
    fn test_access_condition_time_window() {
        let now = Utc::now();
        let condition = AccessCondition::TimeWindow {
            start: now,
            end: now + chrono::Duration::hours(24),
        };

        match condition {
            AccessCondition::TimeWindow { start, end } => {
                assert!(end > start);
            }
            _ => panic!("Expected TimeWindow variant"),
        }
    }

    #[test]
    fn test_access_condition_ip_address() {
        let condition = AccessCondition::IpAddress {
            address: 0x7F000001, // 127.0.0.1
            window_seconds: 3600,
        };

        match condition {
            AccessCondition::IpAddress {
                address,
                window_seconds,
            } => {
                assert_eq!(address, 0x7F000001);
                assert_eq!(window_seconds, 3600);
            }
            _ => panic!("Expected IpAddress variant"),
        }
    }

    #[test]
    fn test_access_condition_require_consensus() {
        let condition = AccessCondition::RequireConsensus {
            threshold: 0.75,
            nodes: vec!["node1".to_string(), "node2".to_string()],
        };

        match condition {
            AccessCondition::RequireConsensus { threshold, nodes } => {
                assert_eq!(threshold, 0.75);
                assert_eq!(nodes.len(), 2);
            }
            _ => panic!("Expected RequireConsensus variant"),
        }
    }

    // AuthMethod tests
    #[test]
    fn test_auth_method_variants() {
        let methods = [
            AuthMethod::Signature,
            AuthMethod::Certificate,
            AuthMethod::BiometricHash,
            AuthMethod::MutualTls,
        ];

        assert_eq!(methods.len(), 4);
    }

    // VerificationMode tests
    #[test]
    fn test_verification_mode_enabled() {
        let mode = VerificationMode::Enabled;
        assert!(matches!(mode, VerificationMode::Enabled));
    }

    #[test]
    fn test_verification_mode_disabled() {
        let mode = VerificationMode::Disabled;
        assert!(matches!(mode, VerificationMode::Disabled));
    }

    // SpawningMode tests
    #[test]
    fn test_spawning_mode_enabled() {
        let mode = SpawningMode::Enabled;
        assert!(matches!(mode, SpawningMode::Enabled));
    }

    #[test]
    fn test_spawning_mode_disabled() {
        let mode = SpawningMode::Disabled;
        assert!(matches!(mode, SpawningMode::Disabled));
    }

    // ApprovalMode tests
    #[test]
    fn test_approval_mode_automated() {
        let mode = ApprovalMode::Automated;
        assert!(matches!(mode, ApprovalMode::Automated));
    }

    #[test]
    fn test_approval_mode_manual() {
        let mode = ApprovalMode::Manual;
        assert!(matches!(mode, ApprovalMode::Manual));
    }

    // OperationType tests
    #[test]
    fn test_operation_type_variants() {
        let types = [
            OperationType::Read,
            OperationType::Write,
            OperationType::Execute,
            OperationType::Delete,
        ];

        assert_eq!(types.len(), 4);
    }

    // CrossNodeOperation tests
    #[test]
    fn test_cross_node_operation_default() {
        let op = CrossNodeOperation::default();
        assert!(matches!(op.operation_type, OperationType::Read));
        assert!(op.target_resource.is_empty());
        assert!(op.parameters.is_empty());
        assert!(op.requester_signature.is_empty());
    }

    #[test]
    fn test_cross_node_operation_creation() {
        let mut params = HashMap::new();
        params.insert("key1".to_string(), "value1".to_string());

        let op = CrossNodeOperation {
            operation_type: OperationType::Write,
            target_resource: "resource-1".to_string(),
            parameters: params.clone(),
            requester_signature: "sig-data".to_string(),
        };

        assert!(matches!(op.operation_type, OperationType::Write));
        assert_eq!(op.target_resource, "resource-1");
        assert_eq!(op.parameters.len(), 1);
        assert!(!op.requester_signature.is_empty());
    }

    // AuthProof tests
    #[test]
    fn test_auth_proof_creation() {
        let proof = AuthProof {
            proof_id: "proof-001".to_string(),
            operation: CrossNodeOperation::default(),
            timestamp: Utc::now(),
            proof_signature: "signature".to_string(),
        };

        assert_eq!(proof.proof_id, "proof-001");
        assert!(!proof.proof_signature.is_empty());
    }

    // AuthorizationProof tests
    #[test]
    fn test_authorization_proof_creation() {
        let proof = AuthorizationProof {
            authorization_id: "auth-001".to_string(),
            operation: CrossNodeOperation::default(),
            timestamp: Utc::now(),
            proof_signature: "signature-data".to_string(),
        };

        assert_eq!(proof.authorization_id, "auth-001");
        assert!(!proof.proof_signature.is_empty());
    }

    // ConsensusResult tests
    #[test]
    fn test_consensus_result_reached() {
        let mut votes = HashMap::new();
        votes.insert("node1".to_string(), true);
        votes.insert("node2".to_string(), true);
        votes.insert("node3".to_string(), false);

        let result = ConsensusResult {
            consensus_reached: true,
            votes: votes.clone(),
            final_score: 0.67,
            participating_nodes: vec![
                "node1".to_string(),
                "node2".to_string(),
                "node3".to_string(),
            ],
        };

        assert!(result.consensus_reached);
        assert_eq!(result.votes.len(), 3);
        assert_eq!(result.participating_nodes.len(), 3);
        assert!(result.final_score > 0.5);
    }

    #[test]
    fn test_consensus_result_not_reached() {
        let mut votes = HashMap::new();
        votes.insert("node1".to_string(), false);
        votes.insert("node2".to_string(), false);

        let result = ConsensusResult {
            consensus_reached: false,
            votes,
            final_score: 0.0,
            participating_nodes: vec!["node1".to_string(), "node2".to_string()],
        };

        assert!(!result.consensus_reached);
        assert_eq!(result.final_score, 0.0);
    }

    #[test]
    fn test_consensus_result_serialization() {
        let result = ConsensusResult {
            consensus_reached: true,
            votes: HashMap::new(),
            final_score: 1.0,
            participating_nodes: vec![],
        };

        let json = serde_json::to_string(&result);
        assert!(json.is_ok(), "Should be able to serialize consensus result");
    }
}
