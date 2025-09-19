// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeAuthConfig {
    /// The verification mode value
    pub verification_mode: VerificationMode,
    pub max_proof_validity_minutes: u32,
    /// The spawning mode value
    pub spawning_mode: SpawningMode,
    pub consensus_config: ConsensusConfig,
    /// Number of max_spawns_per_node
    pub max_spawns_per_node: u32,
    /// The approval mode value
    pub approval_mode: ApprovalMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusConfig {
    /// Whether required is enabled
    pub required: bool,
    /// The threshold value
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeAuthorization {
    pub request_id: String,
    pub requester_node_id: String,
    pub resource_owner_node_id: String,
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
    /// Whether is_active is enabled
    pub is_active: bool,
}

impl CrossNodeAuthorization {
    /// Is Valid operation.
    /// Checks if valid
    /// Checks if valid
    pub fn is_valid(&self) -> bool {
        self.is_active && Utc::now() < self.expires_at
    }

    /// Has Permission operation.
    /// Checks if permission
    /// Checks if permission
    pub fn has_permission(&self, permission: &ResourcePermission) -> bool {
        self.permissions.iter().any(|p| p.implies(permission))
    }
}

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
    /// Implies operation.
    pub fn implies(&self, other: &Self) -> bool {
        match (self, other) {
            (ResourcePermission::Admin, _) => true,
            (ResourcePermission::Write, ResourcePermission::Read) => true,
            (ResourcePermission::Delete, ResourcePermission::Write) => true,
            (ResourcePermission::Spawn, ResourcePermission::Create) => true,
            (a, b) => a == b,
        }
    }

    pub const fn security_level(&self) -> u8 {
        match self {
            ResourcePermission::Read => 1,
            ResourcePermission::Audit => 2,
            ResourcePermission::Backup => 3,
            ResourcePermission::Execute => 4,
            ResourcePermission::Create => 5,
            ResourcePermission::Write => 6,
            ResourcePermission::Share => 7,
            ResourcePermission::Replicate => 8,
            ResourcePermission::Restore => 9,
            ResourcePermission::Consensus => 10,
            ResourcePermission::Compliance => 11,
            ResourcePermission::GeneticModify => 12,
            ResourcePermission::Spawn => 13,
            ResourcePermission::Delete => 14,
            ResourcePermission::Admin => 15,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessCondition {
    /// Represents time window variant
    TimeWindow {
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    },
    IpAddress {
        address: u32,
        window_seconds: u32,
    },
    RequireConsensus {
        threshold: f64,
        nodes: Vec<String>,
    },
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationMode {
    /// Active or enabled state
    Enabled,
    /// Inactive or disabled state
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpawningMode {
    /// Active or enabled state
    Enabled,
    /// Inactive or disabled state
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApprovalMode {
    /// State indicating automated
    Automated,
    /// Represents manual variant
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of operation
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthProof {
    pub proof_id: String,
    /// The operation value
    pub operation: CrossNodeOperation,
    pub timestamp: DateTime<Utc>,
    /// The proof signature value
    pub proof_signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationProof {
    pub authorization_id: String,
    /// The operation value
    pub operation: CrossNodeOperation,
    pub timestamp: DateTime<Utc>,
    /// The proof signature value
    pub proof_signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusResult {
    /// Whether consensus_reached is enabled
    pub consensus_reached: bool,
    /// Mapping of votes
    pub votes: HashMap<String, bool>,
    /// The final score value
    pub final_score: f64,
    /// Collection of participating nodes
    pub participating_nodes: Vec<String>,
}
