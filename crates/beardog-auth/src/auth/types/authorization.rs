

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeAuthConfig {

    pub proof_verification_enabled: bool,

    pub max_proof_validity_minutes: u32,

    pub genetic_spawning_enabled: bool,

    pub require_consensus: bool,

    pub consensus_threshold: f64,

    pub max_spawns_per_node: u32,

    pub automated_approval_enabled: bool,
}

impl Default for CrossNodeAuthConfig {
    fn default() -> Self {
        Self {
            proof_verification_enabled: true,
            max_proof_validity_minutes: 60,
            genetic_spawning_enabled: true,
            require_consensus: false,
            consensus_threshold: 0.67,
            max_spawns_per_node: 10,
            automated_approval_enabled: true,
        }
    }
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeAuthorization {

    pub id: String,

    pub requester_node_id: String,

    pub resource_owner_node_id: String,

    pub resource_id: String,

    pub permissions: Vec<ResourcePermission>,

    pub conditions: Vec<AccessCondition>,

    pub created_at: DateTime<Utc>,

    pub expires_at: DateTime<Utc>,

    pub signature: String,

    pub is_active: bool,}

impl CrossNodeAuthorization {

    pub fn is_valid(&self) -> bool {
        self.is_active && Utc::now() < self.expires_at
    }

    pub fn has_permission(&self, permission: &ResourcePermission) -> bool {
        self.permissions.iter().any(|p| p.implies(permission))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResourcePermission {

    Read,

    Write,

    Delete,

    Admin,

    Execute,

    Create,

    Share,

    Backup,

    Restore,

    Audit,

    Replicate,

    Spawn,

    GeneticModify,

    Consensus,

    Compliance,}

impl ResourcePermission {

    pub fn implies(&self, other: &Self) -> bool {
        match (self, other) {
            (ResourcePermission::Admin, _) => true,
            (ResourcePermission::Write, ResourcePermission::Read) => true,
            (ResourcePermission::Delete, ResourcePermission::Write) => true,
            (ResourcePermission::Spawn, ResourcePermission::Create) => true,
            (a, b) => a == b,
        }
    }

    pub fn security_level(&self) -> u8 {
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

    TimeWindow {

        start: DateTime<Utc>,

        end: DateTime<Utc>,
    },

    IpAddress(String),

    RequireMfa,

    MaxUsage(u32),

    RateLimit {
        max_requests: u32,
        window_seconds: u32,
    },

    RequireConsensus {
        threshold: f64,
        nodes: Vec<String>,
    },
}

pub enum AuthMethod {
    Signature,
    Certificate,
    BiometricHash,
    MutualTls,
}

#[derive(Debug, Clone)]
pub struct CrossNodeOperation {

    pub operation_type: OperationType,

    pub target_resource: String,

    pub parameters: HashMap<String, String>,

    pub requester_signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    Create,
    Update,
    Delete,
    Execute,
}

#[derive(Debug, Clone)]
pub struct AuthorizationProof {
    pub authorization_id: String,
    pub operation: CrossNodeOperation,
    pub timestamp: DateTime<Utc>,
    pub proof_signature: String,
}

#[derive(Debug, Clone)]
pub struct ConsensusResult {

    pub approved: bool,

    pub votes: HashMap<String, bool>,

    pub final_score: f64,

    pub participating_nodes: Vec<String>,
}
