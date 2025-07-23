//! Individual Sovereignty API Models
//!
//! Data structures for empowering individuals through peer-to-peer
//! resource sharing with explicit consent mechanisms.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// PEER-TO-PEER RESOURCE SHARING MODELS
// ============================================================================

/// Request to share resources with a friend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSharingRequest {
    /// Friend's node ID or identity
    pub friend_node_id: String,
    /// Type of resource to share
    pub resource_type: ResourceType,
    /// Amount/limit of resource to share
    pub resource_amount: ResourceAmount,
    /// Duration of sharing arrangement
    pub duration_hours: Option<u32>,
    /// Personal message to friend
    pub personal_message: String,
    /// Consent requirements
    pub consent_requirements: ConsentRequirements,
}

/// Types of resources that can be shared between friends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceType {
    Compute {
        cpu_cores: Option<u32>,
        memory_gb: Option<u32>,
    },
    Storage {
        encrypted: bool,
        backup_only: bool,
    },
    Network {
        bandwidth_mbps: Option<u32>,
    },
}

/// Amount/limits for resource sharing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAmount {
    /// Maximum amount available
    pub maximum: u64,
    /// Current usage
    pub current_usage: u64,
    /// Unit of measurement
    pub unit: String,
}

/// Sharing offer from a friend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharingOffer {
    /// Unique offer identifier
    pub offer_id: String,
    /// Friend offering resources
    pub from_node_id: String,
    /// Friend's display name
    pub from_display_name: String,
    /// Resources being offered
    pub resource_type: ResourceType,
    /// Resource limits
    pub resource_amount: ResourceAmount,
    /// Personal message from friend
    pub personal_message: String,
    /// When offer was made
    pub created_at: String,
    /// When offer expires
    pub expires_at: Option<String>,
    /// Terms and conditions
    pub terms: Vec<String>,
}

/// Active resource sharing arrangement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveShare {
    /// Unique sharing arrangement ID
    pub share_id: String,
    /// Friend you're sharing with
    pub friend_node_id: String,
    /// Friend's display name
    pub friend_display_name: String,
    /// Resource being shared
    pub resource_type: ResourceType,
    /// Usage statistics
    pub usage_stats: ResourceUsageStats,
    /// When sharing started
    pub started_at: String,
    /// When sharing expires
    pub expires_at: Option<String>,
    /// Whether either party can revoke
    pub revocable: bool,
}

/// Resource usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsageStats {
    /// Total allocated
    pub allocated: u64,
    /// Currently in use
    pub current_usage: u64,
    /// Peak usage recorded
    pub peak_usage: u64,
    /// Usage over time
    pub usage_history: Vec<UsageDataPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageDataPoint {
    pub timestamp: String,
    pub usage: u64,
    pub usage_amount: u64,
}

// ============================================================================
// FRIEND-BASED RECOVERY MODELS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardAssignment {
    pub friend_id: String,
    pub friend_display_name: String,
    pub friend_node_id: String,
    pub encrypted_shard: Vec<u8>,
    pub verification_hash: Vec<u8>,
    pub accepted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendRecoveryRequest {
    pub request_id: String,
    pub requester_display_name: String,
    pub recovery_method: RecoveryMethod,
    pub verification_challenge: String,
    pub required_approvals: u32,
    pub current_approvals: u32,
    pub expires_at: String,
    pub personal_message: String,
    pub recovery_friends: Vec<String>,
    pub emergency_contact: Option<String>,
    pub reason: String,
    pub identity_proof: IdentityProof,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryMethod {
    ShamirSecretSharing {
        total_shards: u8,
        required_shards: u8,
    },
    FriendVerification,
    BiometricBackup,
    MultiFactorRecovery,
}

/// Recovery shard distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryShardDistribution {
    /// Unique distribution ID
    pub distribution_id: String,
    /// Shard assignments
    pub shard_assignments: Vec<ShardAssignment>,
    /// Recovery threshold
    pub threshold: u32,
    /// Expiration time
    pub expires_at: Option<String>,
}

// ============================================================================
// IDENTITY MANAGEMENT MODELS
// ============================================================================

/// Self-sovereign identity key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityKey {
    /// Key identifier
    pub key_id: String,
    /// Key purpose
    pub purpose: IdentityKeyPurpose,
    /// Public key data
    pub public_key: String,
    /// Key creation time
    pub created_at: String,
    /// Key expiration
    pub expires_at: Option<String>,
    /// Key status
    pub status: IdentityKeyStatus,
}

/// Purpose of identity key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdentityKeyPurpose {
    /// Primary identity key
    Primary,
    /// Resource sharing authorization
    ResourceSharing,
    /// Emergency recovery
    EmergencyRecovery,
    /// Friend attestation
    FriendAttestation,
    /// Privacy protection
    PrivacyProtection,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IdentityKeyStatus {
    /// Key is active and usable
    Active,
    /// Key is revoked
    Revoked,
    /// Key is expired
    Expired,
    /// Key is suspended temporarily
    Suspended,
}

/// Identity verification claim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityClaim {
    /// Type of claim
    pub claim_type: IdentityClaimType,
    /// Claim data
    pub claim_data: HashMap<String, String>,
    /// Proof of claim
    pub proof: IdentityProof,
    /// Verification requirements
    pub verification_requirements: Vec<String>,
}

/// Types of identity claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdentityClaimType {
    /// Ownership of resource
    ResourceOwnership,
    /// Friend relationship
    FriendRelationship,
    /// Emergency contact
    EmergencyContact,
    /// Identity attestation
    IdentityAttestation,
    /// Custom claim
    Custom(String),
}

/// Proof supporting identity claim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityProof {
    /// Type of proof
    pub proof_type: IdentityProofType,
    /// Proof data
    pub proof_data: String,
    /// Cryptographic signature
    pub signature: Option<String>,
    /// Timestamp
    pub timestamp: String,
}

/// Types of identity proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdentityProofType {
    Ed25519Signature,
    BiometricHash,
    ZeroKnowledgeProof,
    DigitalSignature,
}

// ============================================================================
// CONSENT MANAGEMENT MODELS
// ============================================================================

/// Consent requirements for resource sharing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRequirements {
    /// Explicit consent required
    pub explicit_consent: bool,
    /// Consent expiration
    pub consent_duration_hours: Option<u32>,
    /// Revocation rights
    pub revocable: bool,
    /// Additional consent terms
    pub additional_terms: Vec<String>,
}

/// Active consent record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRecord {
    /// Consent identifier
    pub consent_id: String,
    /// Who gave consent
    pub grantor_id: String,
    /// Who received consent
    pub grantee_id: String,
    /// What consent covers
    pub consent_scope: ConsentScope,
    /// When consent was granted
    pub granted_at: String,
    /// When consent expires
    pub expires_at: Option<String>,
    /// Consent status
    pub status: ConsentStatus,
    /// Revocation rights
    pub revocable: bool,
}

/// Scope of consent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentScope {
    /// Resource types covered
    pub resource_types: Vec<ResourceType>,
    /// Actions permitted
    pub permitted_actions: Vec<String>,
    /// Usage limits
    pub usage_limits: HashMap<String, u64>,
    /// Privacy settings
    pub privacy_level: PrivacyLevel,
}

/// Privacy protection level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrivacyLevel {
    /// Full transparency
    Transparent,
    /// Anonymous usage
    Anonymous,
    /// Zero-knowledge proofs
    ZeroKnowledge,
    /// Maximum privacy
    MaximumPrivacy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConsentStatus {
    /// Consent is active
    Active,
    /// Consent is revoked
    Revoked,
    /// Consent has expired
    Expired,
    /// Consent is suspended
    Suspended,
    /// Consent is pending
    Pending,
}

// ============================================================================
// PRIVACY PROTECTION MODELS
// ============================================================================

/// Privacy status overview
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyStatus {
    /// Overall privacy score
    pub privacy_score: f64,
    /// Active privacy protections
    pub active_protections: Vec<PrivacyProtection>,
    /// Privacy vulnerabilities
    pub vulnerabilities: Vec<PrivacyVulnerability>,
    /// Surveillance detection
    pub surveillance_detection: SurveillanceDetection,
    /// Privacy recommendations
    pub recommendations: Vec<String>,
}

/// Privacy protection mechanism
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyProtection {
    pub protection_type: String,
    pub description: String,
    pub effectiveness_score: f64,
    pub effectiveness: f64,
    pub enabled: bool,
    pub active: bool,
}

/// Privacy vulnerability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyVulnerability {
    pub vulnerability_type: String,
    pub severity: String,
    pub description: String,
    pub remediation_available: bool,
    pub mitigation: String,
}

/// Surveillance detection results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurveillanceDetection {
    pub risk_level: String,
    pub indicators: Vec<String>,
    pub countermeasures: Vec<String>,
    pub last_scan: String,
}
