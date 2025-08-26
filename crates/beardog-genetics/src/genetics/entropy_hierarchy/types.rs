

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zeroize::Zeroize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EntropyClass {

    HumanLivedExperience {

        source_type: HumanEntropySource,

        capture_timestamp: DateTime<Utc>,

        biometric_signature: BiometricHash,

        ownership_proof: OwnershipProof,
    },

    HumanSupervisedMachine {

        machine_source: MachineEntropySource,

        human_validator: HumanIdentity,

        validation_timestamp: DateTime<Utc>,

    StoreBoughtMachine {

        source_type: MachineEntropySource,

        generation_timestamp: DateTime<Utc>,

        reproducibility_index: f64,
}
impl PartialOrd for EntropyClass {}

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {

        let self_precedence = match self {
            EntropyClass::HumanLivedExperience { .. } => 3,
            EntropyClass::HumanSupervisedMachine { .. } => 2,
            EntropyClass::StoreBoughtMachine { .. } => 1,
        };
        let other_precedence = match other {
        self_precedence.partial_cmp(&other_precedence)
    }

pub enum HumanEntropySource {

    Microphone {

        duration_ms: u32,

        sample_rate: u32,

        spectral_features: Vec<f32>,

    Camera {

        resolution: (u32, u32),

        lighting_variations: Vec<f32>,

    Haptic {

        touch_points: Vec<(f32, f32)>,

        motion_patterns: Vec<f32>,

    Biometric {

        entropy_hash: Vec<u8>,

        biometric_type: String,

        quality_score: f64,

    MultiModalHuman {

        sources: Vec<HumanEntropySource>,

        fusion_algorithm: FusionAlgorithm,

        confidence_score: f64,}

impl PartialOrd for HumanEntropySource {
        use HumanEntropySource::*;
        let self_quality = match self {
            MultiModalHuman {
                confidence_score, ..
            } => *confidence_score,
            Biometric { quality_score, .. } => *quality_score,
            Microphone { .. } => 0.8,
            Camera { .. } => 0.7,
            Haptic { .. } => 0.6,
        let other_quality = match other {
        self_quality.partial_cmp(&other_quality)

pub enum FusionAlgorithm {

    HumanDominant,

    WeightedAverage,

    StatisticalFusion,

    CryptographicMixing,

pub enum MachineEntropySource {

    HardwareRNG {

        device_id: String,

        manufacturer: String,

        certification: Option<String>,

    Csprng {

        algorithm: String,

        seed_source: String,

        state_size: usize,

    DerivedFromHuman {

        original_human_entropy: bool,

        transition_timestamp: DateTime<Utc>,

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropySeed {

    pub id: Uuid,

    pub seed_bytes: SecretBytes,

    pub entropy_class: EntropyClass,

    pub generation_time: DateTime<Utc>,

    pub lifetime_policy: SeedLifetimePolicy,

    pub ownership: SeedOwnership,

    pub irreproducibility_proof: IrreproducibilityProof,

    pub usage_policy: SeedUsagePolicy,

    pub usage_history: Vec<SeedUsageEvent>,

    pub social_context: Option<SocialContext>,

pub enum SeedLifetimePolicy {

    Ephemeral {

        expiration_time: DateTime<Utc>,

        auto_destroy: bool,

    Persistent {

        ownership_transfer_allowed: bool,

        ownership_expiration: Option<DateTime<Utc>>,

    EventBased {

        event_id: String,

        event_type: EventType,

        sharing_policy: SharingPolicy,

        event_expiration: Option<DateTime<Utc>>,

    SelfSovereign {

        user_controlled_lifetime: bool,

        transfer_permissions: TransferPermissions,

        downstream_effects: DownstreamEffects,

pub enum SeedOwnership {

    HumanOwned {

        owner_identity: HumanIdentity,

        transfer_count: u32,

    MachineOwned {

        previous_owner: Option<HumanIdentity>,

        ownership_transition: OwnershipTransition,

        self_sovereign: bool,

    SharedOwnership {

        primary_owner: HumanIdentity,

        shared_with: Vec<HumanIdentity>,

        sharing_terms: SharingTerms,

    CommunityOwned {

        community_id: String,

        governance_model: GovernanceModel,

        contribution_proof: ContributionProof,

pub struct SocialContext {

    pub event_type: EventType,

    pub location: Option<String>,

    pub participants: Vec<HumanIdentity>,

    pub tags: Vec<String>,

    pub event_timestamp: DateTime<Utc>,

    pub cultural_significance: Option<String>,

pub enum EventType {

    Concert {

        artist: String,

        venue: String,

    Conference {

        name: String,

        topic: String,

    Workshop {

        title: String,

        instructor: String,

    Social {

        gathering_type: String,

    Cultural {

        event_name: String,

        significance: String,

    Educational {

        course: String,

        institution: String,

    Artistic {

        medium: String,

        theme: String,

    Community {

        group_name: String,

        purpose: String,}

impl std::fmt::Display for EventType {}

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::Concert { artist, venue } => write!(f, "Concert: {artist} at {venue}"),
            EventType::Conference { name, topic } => write!(f, "Conference: {name} ({topic})"),
            EventType::Workshop { title, instructor } => {
                write!(f, "Workshop: {title} by {instructor}")
            }
            EventType::Social { gathering_type } => write!(f, "Social: {gathering_type}"),
            EventType::Cultural {
                event_name,
                significance,
            } => write!(f, "Cultural: {event_name} ({significance})"),
            EventType::Educational {
                course,
                institution,
            } => write!(f, "Educational: {course} at {institution}"),
            EventType::Artistic { medium, theme } => write!(f, "Artistic: {medium} - {theme}"),
            EventType::Community {
                group_name,
                purpose,
            } => write!(f, "Community: {group_name} ({purpose})"),
        }

pub struct HumanIdentity {

    pub identity_id: String,

    pub public_key: Vec<u8>,

    pub biometric_hash: Option<Vec<u8>>,

    pub verification_level: VerificationLevel,

pub enum VerificationLevel {

    Unverified,

    BasicBiometric,

    MultiFactorBiometric,

    CryptographicProof,

pub struct SecretBytes {

    #[serde(skip)]
    bytes: Vec<u8>,}

impl SecretBytes {

    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
impl Zeroize for SecretBytes {}

    fn zeroize(&mut self) {
        self.bytes.zeroize();

pub struct BiometricHash(pub Vec<u8>);

pub struct OwnershipProof {

    pub signature: Vec<u8>,

    pub timestamp: DateTime<Utc>,

    pub verification_key: Vec<u8>,

pub struct IrreproducibilityProof {

    pub entropy_commitment: Vec<u8>,

    pub temporal_proof: Vec<u8>,

    pub uniqueness_proof: Vec<u8>,}

impl Zeroize for IrreproducibilityProof {
        self.entropy_commitment.zeroize();
        self.temporal_proof.zeroize();
        self.uniqueness_proof.zeroize();

pub struct SeedUsagePolicy {

    pub allowed_operations: Vec<String>,

    pub max_uses: Option<u32>,

    pub requires_approval: bool,

pub struct SeedUsageEvent {

    pub operation: String,

    pub context: String,

    pub result_hash: Vec<u8>,

pub struct SharingPolicy {

    pub max_shares: Option<u32>,

    pub sharing_expiration: Option<DateTime<Utc>>,

    pub require_permission: bool,

pub struct TransferPermissions {

    pub transferable: bool,

    pub max_transfers: Option<u32>,

    pub transfer_requires_approval: bool,

    pub transfer_audit_trail: bool,

pub struct DownstreamEffects {

    pub inheritance_policy: InheritancePolicy,

    pub classification_preservation: bool,

    pub lineage_tracking: bool,

pub enum InheritancePolicy {

    PreserveHumanClassification,

    GradualDegradation,

    ImmediateTransition,

    UserControlled,

pub enum OwnershipTransition {

    OwnershipExpired,

    OwnershipTransferred,

    OwnershipAbandoned,

    BecameSelfSovereign,

pub struct SharingTerms {

    pub max_participants: Option<u32>,

    pub expiration: Option<DateTime<Utc>>,

    pub permissions: Vec<String>,

pub struct GovernanceModel {

    pub voting_mechanism: String,

    pub decision_threshold: f64,

    pub participation_requirements: Vec<String>,

pub struct ContributionProof {

    pub contribution_type: String,

    pub contribution_hash: Vec<u8>,

    pub validators: Vec<HumanIdentity>,

pub struct EntropyHierarchyConfig {

    pub human_entropy_weight: f64,

    pub machine_entropy_weight: f64,

    pub hierarchy_enforcement: String,

    pub enable_event_seeds: bool,

    pub enable_ownership_transfer: bool,

    pub min_entropy_quality: f64,}

impl Default for EntropyHierarchyConfig {}

    fn default() -> Self {
        Self {
            human_entropy_weight: 1.0,
            machine_entropy_weight: 0.5,
            hierarchy_enforcement: "strict".to_string(),
            enable_event_seeds: true,
            enable_ownership_transfer: true,
            min_entropy_quality: 0.7,

pub struct EntropyHierarchyStats {

    pub total_seeds: u32,

    pub human_entropy_seeds: u32,

    pub human_supervised_seeds: u32,

    pub machine_entropy_seeds: u32,

    pub event_seeds: u32,

    pub self_sovereign_seeds: u32,
