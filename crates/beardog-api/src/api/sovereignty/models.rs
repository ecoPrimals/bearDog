

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSharingRequest {

    pub friend_node_id: String,

    pub resource_type: ResourceType,

    pub resource_amount: ResourceAmount,

    pub duration_hours: Option<u32>,

    pub personal_message: String,

    pub consent_requirements: ConsentRequirements,
}

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

pub struct ResourceAmount {

    pub maximum: u64,

    pub current_usage: u64,

    pub unit: String,

pub struct SharingOffer {

    pub offer_id: String,

    pub from_node_id: String,

    pub from_display_name: String,

    pub created_at: String,

    pub expires_at: Option<String>,

    pub terms: Vec<String>,

pub struct ActiveShare {

    pub share_id: String,

    pub friend_display_name: String,

    pub usage_stats: ResourceUsageStats,

    pub started_at: String,

    pub revocable: bool,

pub struct ResourceUsageStats {

    pub allocated: u64,

    pub peak_usage: u64,

    pub usage_history: Vec<UsageDataPoint>,
}

pub struct UsageDataPoint {
    pub timestamp: String,
    pub usage: u64,
    pub usage_amount: u64,

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

pub struct RecoveryShardDistribution {

    pub distribution_id: String,

    pub shard_assignments: Vec<ShardAssignment>,

    pub threshold: u32,

pub struct IdentityKey {

    pub key_id: String,

    pub purpose: IdentityKeyPurpose,

    pub public_key: String,

    pub status: IdentityKeyStatus,

pub enum IdentityKeyPurpose {

    Primary,

    ResourceSharing,

    EmergencyRecovery,

    FriendAttestation,

    PrivacyProtection,
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]}

pub enum IdentityKeyStatus {

    Active,

    Revoked,

    Expired,

    Suspended,

pub struct IdentityClaim {

    pub claim_type: IdentityClaimType,

    pub claim_data: HashMap<String, String>,

    pub proof: IdentityProof,

    pub verification_requirements: Vec<String>,

pub enum IdentityClaimType {

    ResourceOwnership,

    FriendRelationship,

    EmergencyContact,

    IdentityAttestation,

    Custom(String),

pub struct IdentityProof {

    pub proof_type: IdentityProofType,

    pub proof_data: String,

    pub signature: Option<String>,

pub enum IdentityProofType {
    Ed25519Signature,
    BiometricHash,
    ZeroKnowledgeProof,
    DigitalSignature,

pub struct ConsentRequirements {

    pub explicit_consent: bool,

    pub consent_duration_hours: Option<u32>,

    pub additional_terms: Vec<String>,

pub struct ConsentRecord {

    pub consent_id: String,

    pub grantor_id: String,

    pub grantee_id: String,

    pub consent_scope: ConsentScope,

    pub granted_at: String,

    pub status: ConsentStatus,

pub struct ConsentScope {

    pub resource_types: Vec<ResourceType>,

    pub permitted_actions: Vec<String>,

    pub usage_limits: HashMap<String, u64>,

    pub privacy_level: PrivacyLevel,

pub enum PrivacyLevel {

    Transparent,

    Anonymous,

    ZeroKnowledge,

    MaximumPrivacy,
}

pub enum ConsentStatus {

    Pending,

pub struct PrivacyStatus {

    pub privacy_score: f64,

    pub active_protections: Vec<PrivacyProtection>,

    pub vulnerabilities: Vec<PrivacyVulnerability>,

    pub surveillance_detection: SurveillanceDetection,

    pub recommendations: Vec<String>,

pub struct PrivacyProtection {
    pub protection_type: String,
    pub description: String,
    pub effectiveness_score: f64,
    pub effectiveness: f64,
    pub enabled: bool,
    pub active: bool,

pub struct PrivacyVulnerability {
    pub vulnerability_type: String,
    pub severity: String,
    pub remediation_available: bool,
    pub mitigation: String,

pub struct SurveillanceDetection {
    pub risk_level: String,
    pub indicators: Vec<String>,
    pub countermeasures: Vec<String>,
    pub last_scan: String,
