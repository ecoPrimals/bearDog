//! Entropy Hierarchy Types
//!
//! This module defines all the types and data structures for the entropy hierarchy system,
//! including entropy classes, seed policies, ownership models, and social contexts.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
        source_type: HumanEntropySource,
        capture_timestamp: DateTime<Utc>,
        biometric_signature: BiometricHash,
        ownership_proof: OwnershipProof,
    },

    /// MID-TIER: Human-Supervised Machine Entropy
    /// - Machine-generated but human-validated
    /// - Reproducible but authenticated
    HumanSupervisedMachine {
        machine_source: MachineEntropySource,
        human_validator: HumanIdentity,
        validation_timestamp: DateTime<Utc>,
    },

    /// LOWEST TIER: Store-Bought Machine Entropy
    /// - Standard CSPRNG, hardware RNG
    /// - Reproducible and "store-bought compute random"
    /// - Default for all-machine operations
    StoreBoughtMachine {
        source_type: MachineEntropySource,
        generation_timestamp: DateTime<Utc>,
        reproducibility_index: f64, // 0.0-1.0, higher = more reproducible
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
        duration_ms: u32,
        sample_rate: u32,
        spectral_features: Vec<f32>,
    },

    /// Camera entropy from visual variations
    Camera {
        duration_ms: u32,
        resolution: (u32, u32),
        lighting_variations: Vec<f32>,
    },

    /// Haptic entropy from touch and motion
    Haptic {
        duration_ms: u32,
        touch_points: Vec<(f32, f32)>,
        motion_patterns: Vec<f32>,
    },

    /// Biometric entropy (privacy-preserving)
    Biometric {
        entropy_hash: Vec<u8>,
        biometric_type: String,
        quality_score: f64,
    },

    /// Multi-modal fusion of multiple sources
    MultiModalHuman {
        sources: Vec<HumanEntropySource>,
        fusion_algorithm: FusionAlgorithm,
        confidence_score: f64,
    },
}

impl PartialOrd for HumanEntropySource {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use HumanEntropySource::*;
        let self_quality = match self {
            MultiModalHuman { confidence_score, .. } => *confidence_score,
            Biometric { quality_score, .. } => *quality_score,
            Microphone { .. } => 0.8,
            Camera { .. } => 0.7,
            Haptic { .. } => 0.6,
        };

        let other_quality = match other {
            MultiModalHuman { confidence_score, .. } => *confidence_score,
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
    HumanDominant,
    WeightedAverage,
    StatisticalFusion,
    CryptographicMixing,
}

/// Sources of machine entropy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MachineEntropySource {
    /// Hardware random number generators
    HardwareRNG {
        device_id: String,
        manufacturer: String,
        certification: Option<String>,
    },

    /// Cryptographically secure pseudo-random generators
    CSPRNG {
        algorithm: String,
        seed_source: String,
        state_size: usize,
    },

    /// Entropy derived from human sources (after ownership transfer)
    DerivedFromHuman {
        original_human_entropy: bool,
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

/// Policies governing seed lifetime and expiration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SeedLifetimePolicy {
    /// Ephemeral seeds with automatic expiration
    Ephemeral {
        expiration_time: DateTime<Utc>,
        auto_destroy: bool,
    },

    /// Persistent seeds with configurable ownership transfer
    Persistent {
        ownership_transfer_allowed: bool,
        ownership_expiration: Option<DateTime<Utc>>,
    },

    /// Event-based seeds for social/generative purposes
    EventBased {
        event_id: String,
        event_type: EventType,
        sharing_policy: SharingPolicy,
        event_expiration: Option<DateTime<Utc>>,
    },

    /// Self-sovereign seeds with full user control
    SelfSovereign {
        user_controlled_lifetime: bool,
        transfer_permissions: TransferPermissions,
        downstream_effects: DownstreamEffects,
    },
}

/// Ownership models for entropy seeds
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SeedOwnership {
    /// Human-owned with cryptographic proof
    HumanOwned {
        owner_identity: HumanIdentity,
        ownership_proof: OwnershipProof,
        transfer_count: u32,
    },

    /// Machine-owned (after ownership transfer or expiration)
    MachineOwned {
        previous_owner: Option<HumanIdentity>,
        ownership_transition: OwnershipTransition,
        self_sovereign: bool,
    },

    /// Shared ownership for events/social purposes
    SharedOwnership {
        primary_owner: HumanIdentity,
        shared_with: Vec<HumanIdentity>,
        sharing_terms: SharingTerms,
    },

    /// Community-owned for generative purposes
    CommunityOwned {
        community_id: String,
        governance_model: GovernanceModel,
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
    Concert {
        artist: String,
        venue: String,
    },
    Conference {
        name: String,
        topic: String,
    },
    Workshop {
        title: String,
        instructor: String,
    },
    Social {
        gathering_type: String,
    },
    Cultural {
        event_name: String,
        significance: String,
    },
    Educational {
        course: String,
        institution: String,
    },
    Artistic {
        medium: String,
        theme: String,
    },
    Community {
        group_name: String,
        purpose: String,
    },
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::Concert { artist, venue } => write!(f, "Concert: {} at {}", artist, venue),
            EventType::Conference { name, topic } => write!(f, "Conference: {} ({})", name, topic),
            EventType::Workshop { title, instructor } => write!(f, "Workshop: {} by {}", title, instructor),
            EventType::Social { gathering_type } => write!(f, "Social: {}", gathering_type),
            EventType::Cultural { event_name, significance } => write!(f, "Cultural: {} ({})", event_name, significance),
            EventType::Educational { course, institution } => write!(f, "Educational: {} at {}", course, institution),
            EventType::Artistic { medium, theme } => write!(f, "Artistic: {} - {}", medium, theme),
            EventType::Community { group_name, purpose } => write!(f, "Community: {} ({})", group_name, purpose),
        }
    }
}

/// Human identity with cryptographic verification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HumanIdentity {
    pub identity_id: String,
    pub public_key: Vec<u8>,
    pub biometric_hash: Option<Vec<u8>>,
    pub verification_level: VerificationLevel,
}

/// Levels of identity verification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VerificationLevel {
    Unverified,
    BasicBiometric,
    MultiFactorBiometric,
    CryptographicProof,
}

/// Secure container for secret bytes with automatic zeroization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretBytes {
    #[serde(skip)]
    bytes: Vec<u8>,
}

impl SecretBytes {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

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
    pub signature: Vec<u8>,
    pub timestamp: DateTime<Utc>,
    pub verification_key: Vec<u8>,
}

/// Proof that entropy cannot be reproduced
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IrreproducibilityProof {
    pub entropy_commitment: Vec<u8>,
    pub temporal_proof: Vec<u8>,
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
    pub allowed_operations: Vec<String>,
    pub max_uses: Option<u32>,
    pub requires_approval: bool,
}

/// Record of seed usage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeedUsageEvent {
    pub timestamp: DateTime<Utc>,
    pub operation: String,
    pub context: String,
    pub result_hash: Vec<u8>,
}

/// Policy for sharing seeds with others
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharingPolicy {
    pub max_shares: Option<u32>,
    pub sharing_expiration: Option<DateTime<Utc>>,
    pub require_permission: bool,
    pub allowed_operations: Vec<String>,
}

/// Permissions for transferring seed ownership
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferPermissions {
    pub transferable: bool,
    pub max_transfers: Option<u32>,
    pub transfer_requires_approval: bool,
    pub transfer_audit_trail: bool,
}

/// Effects of ownership transfer on derived seeds
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownstreamEffects {
    pub inheritance_policy: InheritancePolicy,
    pub classification_preservation: bool,
    pub lineage_tracking: bool,
}

/// How entropy classification is inherited
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InheritancePolicy {
    PreserveHumanClassification,
    GradualDegradation,
    ImmediateTransition,
    UserControlled,
}

/// How ownership transitions occur
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OwnershipTransition {
    OwnershipExpired,
    OwnershipTransferred,
    OwnershipAbandoned,
    BecameSelfSovereign,
}

/// Terms for shared ownership
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharingTerms {
    pub max_participants: Option<u32>,
    pub expiration: Option<DateTime<Utc>>,
    pub permissions: Vec<String>,
}

/// Community governance model
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GovernanceModel {
    pub voting_mechanism: String,
    pub decision_threshold: f64,
    pub participation_requirements: Vec<String>,
}

/// Proof of contribution to community-owned entropy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContributionProof {
    pub contribution_type: String,
    pub contribution_hash: Vec<u8>,
    pub timestamp: DateTime<Utc>,
    pub validators: Vec<HumanIdentity>,
}

/// Configuration for entropy hierarchy system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntropyHierarchyConfig {
    pub human_entropy_weight: f64,
    pub machine_entropy_weight: f64,
    pub hierarchy_enforcement: String,
    pub enable_event_seeds: bool,
    pub enable_ownership_transfer: bool,
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
    pub total_seeds: u32,
    pub human_entropy_seeds: u32,
    pub human_supervised_seeds: u32,
    pub machine_entropy_seeds: u32,
    pub event_seeds: u32,
    pub self_sovereign_seeds: u32,
} 