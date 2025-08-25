// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Individual Sovereignty API Models
///
/// Data structures for empowering individuals through peer-to-peer
/// resource sharing with explicit consent mechanisms.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// ============================================================================
// PEER-TO-PEER RESOURCE SHARING MODELS
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
pub enum ResourceType {
    Compute {
        cpu_cores: Option<u32>,
        memory_gb: Option<u32>,
    },
    Storage {
        encrypted: bool,
        backup_only: bool,
    Network {
        bandwidth_mbps: Option<u32>,
/// Amount/limits for resource sharing
pub struct ResourceAmount {
    /// Maximum amount available
    pub maximum: u64,
    /// Current usage
    pub current_usage: u64,
    /// Unit of measurement
    pub unit: String,
/// Sharing offer from a friend
pub struct SharingOffer {
    /// Unique offer identifier
    pub offer_id: String,
    /// Friend offering resources
    pub from_node_id: String,
    /// Friend's display name
    pub from_display_name: String,
    /// Resources being offered
    /// Resource limits
    /// Personal message from friend
    /// When offer was made
    pub created_at: String,
    /// When offer expires
    pub expires_at: Option<String>,
    /// Terms and conditions
    pub terms: Vec<String>,
/// Active resource sharing arrangement
pub struct ActiveShare {
    /// Unique sharing arrangement ID
    pub share_id: String,
    /// Friend you're sharing with
    pub friend_display_name: String,
    /// Resource being shared
    /// Usage statistics
    pub usage_stats: ResourceUsageStats,
    /// When sharing started
    pub started_at: String,
    /// When sharing expires
    /// Whether either party can revoke
    pub revocable: bool,
/// Resource usage statistics
pub struct ResourceUsageStats {
    /// Total allocated
    pub allocated: u64,
    /// Currently in use
    /// Peak usage recorded
    pub peak_usage: u64,
    /// Usage over time
    pub usage_history: Vec<UsageDataPoint>,
}


pub struct UsageDataPoint {
    pub timestamp: String,
    pub usage: u64,
    pub usage_amount: u64,
// FRIEND-BASED RECOVERY MODELS
pub struct ShardAssignment {
    pub friend_id: String,
    pub encrypted_shard: Vec<u8>,
    pub verification_hash: Vec<u8>,
    pub accepted: bool,
}


pub struct FriendRecoveryRequest {
    pub request_id: String,
    pub requester_display_name: String,
    pub recovery_method: RecoveryMethod,
    pub verification_challenge: String,
    pub required_approvals: u32,
    pub current_approvals: u32,
    pub expires_at: String,
    pub recovery_friends: Vec<String>,
    pub emergency_contact: Option<String>,
    pub reason: String,
    pub identity_proof: IdentityProof,
pub enum RecoveryMethod {
    ShamirSecretSharing {
        total_shards: u8,
        required_shards: u8,
    FriendVerification,
    BiometricBackup,
    MultiFactorRecovery,
/// Recovery shard distribution}


pub struct RecoveryShardDistribution {
    /// Unique distribution ID
    pub distribution_id: String,
    /// Shard assignments
    pub shard_assignments: Vec<ShardAssignment>,
    /// Recovery threshold
    pub threshold: u32,
    /// Expiration time
// IDENTITY MANAGEMENT MODELS
/// Self-sovereign identity key
pub struct IdentityKey {
    /// Key identifier
    pub key_id: String,
    /// Key purpose
    pub purpose: IdentityKeyPurpose,
    /// Public key data
    pub public_key: String,
    /// Key creation time
    /// Key expiration
    /// Key status
    pub status: IdentityKeyStatus,
/// Purpose of identity key
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]}


pub enum IdentityKeyStatus {
    /// Key is active and usable
    Active,
    /// Key is revoked
    Revoked,
    /// Key is expired
    Expired,
    /// Key is suspended temporarily
    Suspended,
/// Identity verification claim
pub struct IdentityClaim {
    /// Type of claim
    pub claim_type: IdentityClaimType,
    /// Claim data
    pub claim_data: HashMap<String, String>,
    /// Proof of claim
    pub proof: IdentityProof,
    /// Verification requirements
    pub verification_requirements: Vec<String>,
/// Types of identity claims
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
/// Proof supporting identity claim}


pub struct IdentityProof {
    /// Type of proof
    pub proof_type: IdentityProofType,
    /// Proof data
    pub proof_data: String,
    /// Cryptographic signature
    pub signature: Option<String>,
    /// Timestamp
/// Types of identity proof
pub enum IdentityProofType {
    Ed25519Signature,
    BiometricHash,
    ZeroKnowledgeProof,
    DigitalSignature,
// CONSENT MANAGEMENT MODELS
/// Consent requirements for resource sharing}


pub struct ConsentRequirements {
    /// Explicit consent required
    pub explicit_consent: bool,
    /// Consent expiration
    pub consent_duration_hours: Option<u32>,
    /// Revocation rights
    /// Additional consent terms
    pub additional_terms: Vec<String>,
/// Active consent record
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
    /// Consent status
    pub status: ConsentStatus,
/// Scope of consent
pub struct ConsentScope {
    /// Resource types covered
    pub resource_types: Vec<ResourceType>,
    /// Actions permitted
    pub permitted_actions: Vec<String>,
    /// Usage limits
    pub usage_limits: HashMap<String, u64>,
    /// Privacy settings
    pub privacy_level: PrivacyLevel,
/// Privacy protection level
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


pub enum ConsentStatus {
    /// Consent is active
    /// Consent is revoked
    /// Consent has expired
    /// Consent is suspended
    /// Consent is pending
    Pending,
// PRIVACY PROTECTION MODELS
/// Privacy status overview}


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
/// Privacy protection mechanism
pub struct PrivacyProtection {
    pub protection_type: String,
    pub description: String,
    pub effectiveness_score: f64,
    pub effectiveness: f64,
    pub enabled: bool,
    pub active: bool,
/// Privacy vulnerability
pub struct PrivacyVulnerability {
    pub vulnerability_type: String,
    pub severity: String,
    pub remediation_available: bool,
    pub mitigation: String,
/// Surveillance detection results
pub struct SurveillanceDetection {
    pub risk_level: String,
    pub indicators: Vec<String>,
    pub countermeasures: Vec<String>,
    pub last_scan: String,
