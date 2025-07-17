//! Core authorization types for cross-node authentication and permissions
//!
//! This module contains the fundamental types for cross-node authorization,
//! including authorization grants, permissions, conditions, and operations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for cross-node authorization
///
/// Central configuration for the cross-node authorization system, defining
/// policies for proof verification, genetic spawning, consensus requirements,
/// and workflow automation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeAuthConfig {
    /// Enable proof verification for authorization requests
    pub proof_verification_enabled: bool,
    /// Maximum proof validity duration in minutes
    pub max_proof_validity_minutes: u32,
    /// Enable genetic spawning authorization
    pub genetic_spawning_enabled: bool,
    /// Require multi-party consensus for sensitive operations
    pub require_consensus: bool,
    /// Trust threshold for consensus (0.0 to 1.0)
    pub consensus_threshold: f64,
    /// Maximum number of spawned BearDogs per node
    pub max_spawns_per_node: u32,
    /// Enable automated workflow approval
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

/// Cross-node authorization structure
///
/// Represents an authorization grant from one node to another, including
/// permissions, conditions, expiration, and cryptographic validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeAuthorization {
    /// Unique authorization ID
    pub id: String,
    /// Requesting node ID
    pub requester_node_id: String,
    /// Resource owner node ID
    pub resource_owner_node_id: String,
    /// Resource identifier
    pub resource_id: String,
    /// Granted permissions
    pub permissions: Vec<ResourcePermission>,
    /// Access conditions
    pub conditions: Vec<AccessCondition>,
    /// Authorization creation time
    pub created_at: DateTime<Utc>,
    /// Authorization expiration time
    pub expires_at: DateTime<Utc>,
    /// Cryptographic signature
    pub signature: String,
    /// Whether the authorization is currently active
    pub is_active: bool,
}

impl CrossNodeAuthorization {
    /// Check if authorization is currently valid
    pub fn is_valid(&self) -> bool {
        self.is_active && Utc::now() < self.expires_at
    }

    /// Check if authorization grants specific permission
    pub fn has_permission(&self, permission: &ResourcePermission) -> bool {
        self.permissions.iter().any(|p| p.implies(permission))
    }
}

/// Resource permission levels
///
/// Defines the different types of permissions that can be granted
/// for accessing resources across nodes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResourcePermission {
    /// Read access to the resource
    Read,
    /// Write access to modify the resource
    Write,
    /// Delete permission to remove the resource
    Delete,
    /// Administrative access for full control
    Admin,
    /// Execute permission for running operations
    Execute,
    /// Permission to create new resources
    Create,
    /// Permission to share resources with other nodes
    Share,
    /// Permission to backup the resource
    Backup,
    /// Permission to restore from backup
    Restore,
    /// Permission to audit resource access
    Audit,
    /// Permission to replicate data across nodes
    Replicate,
    /// Permission to spawn new BearDog instances
    Spawn,
    /// Permission to modify genetic algorithms
    GeneticModify,
    /// Permission to participate in consensus
    Consensus,
    /// Permission to perform compliance operations
    Compliance,
}

impl ResourcePermission {
    /// Check if permission implies another permission
    pub fn implies(&self, other: &Self) -> bool {
        match (self, other) {
            (ResourcePermission::Admin, _) => true,
            (ResourcePermission::Write, ResourcePermission::Read) => true,
            (ResourcePermission::Delete, ResourcePermission::Write) => true,
            (ResourcePermission::Spawn, ResourcePermission::Create) => true,
            (a, b) => a == b,
        }
    }

    /// Get the security level of the permission (higher = more sensitive)
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

/// Access condition modifiers
///
/// Defines conditions that must be met for an authorization to be valid,
/// such as time windows, IP restrictions, and usage limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessCondition {
    /// Time-based access window
    TimeWindow {
        /// Start time of the access window
        start: DateTime<Utc>,
        /// End time of the access window
        end: DateTime<Utc>,
    },
    /// IP address restriction
    IpAddress(String),
    /// Require multi-factor authentication
    RequireMfa,
    /// Maximum usage count
    MaxUsage(u32),
    /// Rate limiting
    RateLimit {
        /// Maximum requests allowed
        max_requests: u32,
        /// Time window in seconds
        window_seconds: u32,
    },
    /// Require consensus from other nodes
    RequireConsensus {
        /// Required consensus threshold (0.0 to 1.0)
        threshold: f64,
        /// List of nodes that must participate
        nodes: Vec<String>,
    },
}

/// Authentication method types
///
/// Defines the different methods that can be used for authentication
/// in cross-node communications.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    /// Digital signature authentication
    Signature,
    /// Certificate-based authentication
    Certificate,
    /// Biometric hash authentication
    BiometricHash,
    /// Mutual TLS authentication
    MutualTls,
}

/// Cross-node operation descriptor
///
/// Describes a specific operation being performed across nodes,
/// including the operation type, target resource, and parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeOperation {
    /// Type of operation being performed
    pub operation_type: OperationType,
    /// Target resource identifier
    pub target_resource: String,
    /// Operation parameters
    pub parameters: HashMap<String, String>,
    /// Signature of the requester
    pub requester_signature: String,
}

/// Operation types for cross-node operations
///
/// Defines the different types of operations that can be performed
/// across nodes in the BearDog network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    /// Read operation
    Read,
    /// Write operation
    Write,
    /// Delete operation
    Delete,
    /// Execute operation
    Execute,
    /// Backup operation
    Backup,
    /// Restore operation
    Restore,
    /// Spawn operation
    Spawn,
    /// Consensus operation
    Consensus,
}

/// Authorization proof structure
///
/// Cryptographic proof that an authorization is valid and can be used
/// for a specific operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationProof {
    /// ID of the authorization being proven
    pub authorization_id: String,
    /// Operation being authorized
    pub operation: CrossNodeOperation,
    /// Timestamp when proof was generated
    pub timestamp: DateTime<Utc>,
    /// Cryptographic signature of the proof
    pub proof_signature: String,
}

/// Consensus result for multi-party decisions
#[derive(Debug, Clone)]
pub struct ConsensusResult {
    /// Whether the consensus was approved
    pub approved: bool,
    /// Individual votes from participating nodes
    pub votes: HashMap<String, bool>,
    /// Minimum threshold required for consensus
    pub consensus_threshold: f64,
    /// Final calculated consensus score
    pub final_score: f64,
    /// List of nodes that participated in the consensus
    pub participating_nodes: Vec<String>,
}
