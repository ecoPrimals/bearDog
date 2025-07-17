//! Entropy Hierarchy Types
//!
//! This module defines all the types and data structures for the entropy hierarchy system,
//! including entropy classes, seed policies, ownership models, and social contexts.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zeroize::Zeroize;

/// Three-tier entropy hierarchy: Human > Human-Supervised > Store-Bought
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EntropyClass {
    /// HIGHEST TIER: Human-Lived Experience Entropy
    /// - From microphone, camera, haptic sensors
    /// - Irreproducible and uniquely owned
    /// - Cannot be replicated by machines
    HumanLivedExperience {
        /// The type of human entropy source used
        source_type: HumanEntropySource,
        /// Timestamp when the entropy was captured
        capture_timestamp: DateTime<Utc>,
        /// Biometric signature for authentication
        biometric_signature: BiometricHash,
        /// Proof of ownership for this entropy
        ownership_proof: OwnershipProof,
    },

    /// MID-TIER: Human-Supervised Machine Entropy
    /// - Machine-generated but human-validated
    /// - Reproducible but authenticated
    HumanSupervisedMachine {
        /// The machine source that generated the entropy
        machine_source: MachineEntropySource,
        /// Human validator who supervised the generation
        human_validator: HumanIdentity,
        /// Timestamp when the entropy was validated
        validation_timestamp: DateTime<Utc>,
    },

    /// LOWEST TIER: Store-Bought Machine Entropy
    /// - Standard CSPRNG, hardware RNG
    /// - Reproducible and "store-bought compute random"
    /// - Default for all-machine operations
    StoreBoughtMachine {
        /// The type of machine entropy source
        source_type: MachineEntropySource,
        /// Timestamp when the entropy was generated
        generation_timestamp: DateTime<Utc>,
        /// Reproducibility index (0.0-1.0, higher = more reproducible)
        reproducibility_index: f64,
    },
}

impl PartialOrd for EntropyClass {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Define precedence levels: HumanLivedExperience > HumanSupervisedMachine > StoreBoughtMachine
        let self_precedence = match self {
            EntropyClass::HumanLivedExperience { .. } => 3,
            EntropyClass::HumanSupervisedMachine { .. } => 2,
            EntropyClass::StoreBoughtMachine { .. } => 1,
        };

        let other_precedence = match other {
            EntropyClass::HumanLivedExperience { .. } => 3,
            EntropyClass::HumanSupervisedMachine { .. } => 2,
            EntropyClass::StoreBoughtMachine { .. } => 1,
        };

        self_precedence.partial_cmp(&other_precedence)
    }
}

/// Sources of human entropy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HumanEntropySource {
    /// Microphone entropy from ambient sound
    Microphone {
        /// Duration of audio capture in milliseconds
        duration_ms: u32,
        /// Audio sample rate in Hz
        sample_rate: u32,
        /// Spectral features extracted from audio
        spectral_features: Vec<f32>,
    },

    /// Camera entropy from visual variations
    Camera {
        /// Duration of video capture in milliseconds
        duration_ms: u32,
        /// Video resolution (width, height)
        resolution: (u32, u32),
        /// Lighting variation measurements
        lighting_variations: Vec<f32>,
    },

    /// Haptic entropy from touch and motion
    Haptic {
        /// Duration of haptic capture in milliseconds
        duration_ms: u32,
        /// Touch points on the interface
        touch_points: Vec<(f32, f32)>,
        /// Motion patterns detected
        motion_patterns: Vec<f32>,
    },

    /// Biometric entropy (privacy-preserving)
    Biometric {
        /// Entropy hash derived from biometric data
        entropy_hash: Vec<u8>,
        /// Type of biometric data used
        biometric_type: String,
        /// Quality score of the biometric data (0.0-1.0)
        quality_score: f64,
    },

    /// Multi-modal fusion of multiple sources
    MultiModalHuman {
        /// List of human entropy sources being fused
        sources: Vec<HumanEntropySource>,
        /// Algorithm used for fusion
        fusion_algorithm: FusionAlgorithm,
        /// Confidence score of the fusion result (0.0-1.0)
        confidence_score: f64,
    },
}

impl PartialOrd for HumanEntropySource {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use HumanEntropySource::*;
        let self_quality = match self {
            MultiModalHuman {
                confidence_score, ..
            } => *confidence_score,
            Biometric { quality_score, .. } => *quality_score,
            Microphone { .. } => 0.8,
            Camera { .. } => 0.7,
            Haptic { .. } => 0.6,
        };

        let other_quality = match other {
            MultiModalHuman {
                confidence_score, ..
            } => *confidence_score,
            Biometric { quality_score, .. } => *quality_score,
            Microphone { .. } => 0.8,
            Camera { .. } => 0.7,
            Haptic { .. } => 0.6,
        };

        self_quality.partial_cmp(&other_quality)
    }
}

/// Algorithms for fusing multiple entropy sources
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FusionAlgorithm {
    /// Human entropy takes precedence over machine entropy
    HumanDominant,
    /// Weighted average of all entropy sources
    WeightedAverage,
    /// Statistical fusion using entropy properties
    StatisticalFusion,
    /// Cryptographic mixing of entropy sources
    CryptographicMixing,
}

/// Sources of machine entropy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MachineEntropySource {
    /// Hardware random number generators
    HardwareRNG {
        /// Unique identifier for the hardware device
        device_id: String,
        /// Manufacturer of the hardware RNG
        manufacturer: String,
        /// Optional certification level or standard
        certification: Option<String>,
    },

    /// Cryptographically secure pseudo-random generators
    CSPRNG {
        /// Algorithm name used for generation
        algorithm: String,
        /// Source of the initial seed
        seed_source: String,
        /// Size of the internal state in bytes
        state_size: usize,
    },

    /// Entropy derived from human sources (after ownership transfer)
    DerivedFromHuman {
        /// Whether this was originally human entropy
        original_human_entropy: bool,
        /// Timestamp of the ownership transition
        transition_timestamp: DateTime<Utc>,
    },
}

/// An entropy seed with full lifecycle management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropySeed {
    /// Unique identifier for this seed
    pub id: Uuid,

    /// The actual seed bytes (never logged or transmitted)
    pub seed_bytes: SecretBytes,

    /// Entropy classification (preserves hierarchy)
    pub entropy_class: EntropyClass,

    /// Timestamp when seed was generated
    pub generation_time: DateTime<Utc>,

    /// Seed lifetime policy (can be ephemeral, persistent, or transferable)
    pub lifetime_policy: SeedLifetimePolicy,

    /// Current ownership information
    pub ownership: SeedOwnership,

    /// Proof that this seed cannot be reproduced
    pub irreproducibility_proof: IrreproducibilityProof,

    /// Usage restrictions and permissions
    pub usage_policy: SeedUsagePolicy,

    /// Audit trail (what operations used this seed)
    pub usage_history: Vec<SeedUsageEvent>,

    /// Social/event context (for shared seeds)
    pub social_context: Option<SocialContext>,
}

/// Lifecycle policies for entropy seeds
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SeedLifetimePolicy {
    /// Ephemeral seeds with automatic expiration
    Ephemeral {
        /// Time when the seed expires
        expiration_time: DateTime<Utc>,
        /// Whether to automatically destroy the seed on expiration
        auto_destroy: bool,
    },

    /// Persistent seeds with configurable ownership transfer
    Persistent {
        /// Whether ownership can be transferred
        ownership_transfer_allowed: bool,
        /// Optional expiration time for ownership
        ownership_expiration: Option<DateTime<Utc>>,
    },

    /// Event-based seeds for social/generative purposes
    EventBased {
        /// Unique identifier for the event
        event_id: String,
        /// Type of event this seed is associated with
        event_type: EventType,
        /// Sharing policy for this event-based seed
        sharing_policy: SharingPolicy,
        /// Optional expiration time for the event
        event_expiration: Option<DateTime<Utc>>,
    },

    /// Self-sovereign seeds with full user control
    SelfSovereign {
        /// Whether the user controls the seed lifetime
        user_controlled_lifetime: bool,
        /// Permissions for transferring the seed
        transfer_permissions: TransferPermissions,
        /// Effects of downstream usage
        downstream_effects: DownstreamEffects,
    },
}

/// Ownership models for entropy seeds
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SeedOwnership {
    /// Human-owned with cryptographic proof
    HumanOwned {
        /// Identity of the human owner
        owner_identity: HumanIdentity,
        /// Cryptographic proof of ownership
        ownership_proof: OwnershipProof,
        /// Number of times ownership has been transferred
        transfer_count: u32,
    },

    /// Machine-owned (after ownership transfer or expiration)
    MachineOwned {
        /// Previous human owner (if any)
        previous_owner: Option<HumanIdentity>,
        /// Details of the ownership transition
        ownership_transition: OwnershipTransition,
        /// Whether this is self-sovereign ownership
        self_sovereign: bool,
    },

    /// Shared ownership for events/social purposes
    SharedOwnership {
        /// Primary owner of the seed
        primary_owner: HumanIdentity,
        /// List of users this seed is shared with
        shared_with: Vec<HumanIdentity>,
        /// Terms and conditions of sharing
        sharing_terms: SharingTerms,
    },

    /// Community-owned for generative purposes
    CommunityOwned {
        /// Unique identifier for the community
        community_id: String,
        /// Governance model for community decisions
        governance_model: GovernanceModel,
        /// Proof of contribution to the community
        contribution_proof: ContributionProof,
    },
}

/// Social context for event-based seeds
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocialContext {
    /// Event or social setting
    pub event_type: EventType,

    /// Location or venue information
    pub location: Option<String>,

    /// Participants or attendees
    pub participants: Vec<HumanIdentity>,

    /// Tags for categorization
    pub tags: Vec<String>,

    /// Timestamp of social event
    pub event_timestamp: DateTime<Utc>,

    /// Cultural or artistic significance
    pub cultural_significance: Option<String>,
}

/// Types of social events for entropy seed generation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventType {
    /// Concert or musical performance
    Concert {
        /// Name of the performing artist
        artist: String,
        /// Name of the venue
        venue: String,
    },
    /// Conference or professional gathering
    Conference {
        /// Name of the conference
        name: String,
        /// Main topic or theme
        topic: String,
    },
    /// Workshop or training session
    Workshop {
        /// Title of the workshop
        title: String,
        /// Name of the instructor
        instructor: String,
    },
    /// Social gathering
    Social {
        /// Type of social gathering
        gathering_type: String,
    },
    /// Cultural event
    Cultural {
        /// Name of the cultural event
        event_name: String,
        /// Cultural significance description
        significance: String,
    },
    /// Educational event
    Educational {
        /// Course or class name
        course: String,
        /// Educational institution
        institution: String,
    },
    /// Artistic event
    Artistic {
        /// Artistic medium used
        medium: String,
        /// Theme of the artistic work
        theme: String,
    },
    /// Community event
    Community {
        /// Name of the community group
        group_name: String,
        /// Purpose or goal of the community
        purpose: String,
    },
}

impl std::fmt::Display for EventType {
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
    }
}

/// Human identity with cryptographic verification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HumanIdentity {
    /// Unique identifier for the human identity
    pub identity_id: String,
    /// Public key for cryptographic verification
    pub public_key: Vec<u8>,
    /// Optional biometric hash for additional verification
    pub biometric_hash: Option<Vec<u8>>,
    /// Level of verification for this identity
    pub verification_level: VerificationLevel,
}

/// Levels of identity verification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VerificationLevel {
    /// No verification performed
    Unverified,
    /// Basic biometric verification
    BasicBiometric,
    /// Multi-factor biometric verification
    MultiFactorBiometric,
    /// Cryptographic proof of identity
    CryptographicProof,
}

/// Secure container for secret bytes with automatic zeroization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretBytes {
    /// Raw bytes of the secret data
    #[serde(skip)]
    bytes: Vec<u8>,
}

impl SecretBytes {
    /// Create a new SecretBytes container with the given bytes
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    /// Get a reference to the underlying bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Check if the container is empty
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

impl Zeroize for SecretBytes {
    fn zeroize(&mut self) {
        self.bytes.zeroize();
    }
}

/// Hash of biometric data for privacy-preserving verification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BiometricHash(pub Vec<u8>);

/// Cryptographic proof of ownership
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OwnershipProof {
    /// Digital signature proving ownership
    pub signature: Vec<u8>,
    /// Timestamp when the proof was created
    pub timestamp: DateTime<Utc>,
    /// Key used to verify the signature
    pub verification_key: Vec<u8>,
}

/// Proof that entropy cannot be reproduced
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IrreproducibilityProof {
    /// Cryptographic commitment to the entropy value
    pub entropy_commitment: Vec<u8>,
    /// Proof that the entropy was generated at a specific time
    pub temporal_proof: Vec<u8>,
    /// Proof that the entropy is unique and cannot be reproduced
    pub uniqueness_proof: Vec<u8>,
}

impl Zeroize for IrreproducibilityProof {
    fn zeroize(&mut self) {
        self.entropy_commitment.zeroize();
        self.temporal_proof.zeroize();
        self.uniqueness_proof.zeroize();
    }
}

/// Policy governing how seeds can be used
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeedUsagePolicy {
    /// List of operations that are allowed with this seed
    pub allowed_operations: Vec<String>,
    /// Maximum number of times this seed can be used
    pub max_uses: Option<u32>,
    /// Whether seed usage requires approval
    pub requires_approval: bool,
}

/// Record of seed usage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeedUsageEvent {
    /// When the seed was used
    pub timestamp: DateTime<Utc>,
    /// What operation was performed with the seed
    pub operation: String,
    /// Context or reason for the seed usage
    pub context: String,
    /// Hash of the result produced by using the seed
    pub result_hash: Vec<u8>,
}

/// Policy for sharing seeds with others
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharingPolicy {
    /// Maximum number of times this seed can be shared
    pub max_shares: Option<u32>,
    /// When the sharing permissions expire
    pub sharing_expiration: Option<DateTime<Utc>>,
    /// Whether permission is required before sharing
    pub require_permission: bool,
    /// Operations that are allowed when sharing
    pub allowed_operations: Vec<String>,
}

/// Permissions for transferring seed ownership
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferPermissions {
    /// Whether this seed can be transferred to another owner
    pub transferable: bool,
    /// Maximum number of times ownership can be transferred
    pub max_transfers: Option<u32>,
    /// Whether transfers require approval
    pub transfer_requires_approval: bool,
    /// Whether to maintain an audit trail of transfers
    pub transfer_audit_trail: bool,
}

/// Effects of ownership transfer on derived seeds
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownstreamEffects {
    /// How entropy classification is inherited by derived seeds
    pub inheritance_policy: InheritancePolicy,
    /// Whether to preserve the original entropy classification
    pub classification_preservation: bool,
    /// Whether to track the lineage of derived seeds
    pub lineage_tracking: bool,
}

/// How entropy classification is inherited
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InheritancePolicy {
    /// Preserve the original human classification
    PreserveHumanClassification,
    /// Gradually degrade the classification over time
    GradualDegradation,
    /// Immediately transition to machine classification
    ImmediateTransition,
    /// User controls the inheritance behavior
    UserControlled,
}

/// How ownership transitions occur
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OwnershipTransition {
    /// Ownership expired due to time limit
    OwnershipExpired,
    /// Ownership was transferred to another entity
    OwnershipTransferred,
    /// Ownership was abandoned by the owner
    OwnershipAbandoned,
    /// Became self-sovereign ownership
    BecameSelfSovereign,
}

/// Terms for shared ownership
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharingTerms {
    /// Maximum number of participants in shared ownership
    pub max_participants: Option<u32>,
    /// When the shared ownership expires
    pub expiration: Option<DateTime<Utc>>,
    /// Permissions granted to shared owners
    pub permissions: Vec<String>,
}

/// Community governance model
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GovernanceModel {
    /// Mechanism used for voting (e.g., simple majority, consensus)
    pub voting_mechanism: String,
    /// Threshold required for decisions (0.0 to 1.0)
    pub decision_threshold: f64,
    /// Requirements for participation in governance
    pub participation_requirements: Vec<String>,
}

/// Proof of contribution to community-owned entropy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContributionProof {
    /// Type of contribution made to the community
    pub contribution_type: String,
    /// Hash of the contribution for verification
    pub contribution_hash: Vec<u8>,
    /// When the contribution was made
    pub timestamp: DateTime<Utc>,
    /// Validators who verified the contribution
    pub validators: Vec<HumanIdentity>,
}

/// Configuration for entropy hierarchy system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntropyHierarchyConfig {
    /// Weight given to human entropy in calculations
    pub human_entropy_weight: f64,
    /// Weight given to machine entropy in calculations
    pub machine_entropy_weight: f64,
    /// How strictly to enforce the hierarchy (strict, permissive, adaptive)
    pub hierarchy_enforcement: String,
    /// Whether to enable event-based seeds
    pub enable_event_seeds: bool,
    /// Whether to enable ownership transfer
    pub enable_ownership_transfer: bool,
    /// Minimum entropy quality required (0.0 to 1.0)
    pub min_entropy_quality: f64,
}

impl Default for EntropyHierarchyConfig {
    fn default() -> Self {
        Self {
            human_entropy_weight: 1.0,
            machine_entropy_weight: 0.5,
            hierarchy_enforcement: "strict".to_string(),
            enable_event_seeds: true,
            enable_ownership_transfer: true,
            min_entropy_quality: 0.7,
        }
    }
}

/// Statistics about the entropy hierarchy system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntropyHierarchyStats {
    /// Total number of entropy seeds in the system
    pub total_seeds: u32,
    /// Number of human-lived experience entropy seeds
    pub human_entropy_seeds: u32,
    /// Number of human-supervised machine entropy seeds
    pub human_supervised_seeds: u32,
    /// Number of store-bought machine entropy seeds
    pub machine_entropy_seeds: u32,
    /// Number of event-based entropy seeds
    pub event_seeds: u32,
    /// Number of self-sovereign entropy seeds
    pub self_sovereign_seeds: u32,
}
