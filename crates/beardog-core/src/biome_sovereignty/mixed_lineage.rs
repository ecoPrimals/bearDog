// SPDX-License-Identifier: AGPL-3.0-only

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Cryptographic key with mixed human-primal lineage
///
/// Represents a shared key derived from combined human and primal entropy,
/// supporting mixed lineage partnerships and collaborative operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixedLineageKey {
    /// Unique key identifier
    pub id: String,
    /// Partner identities participating in the key sharing
    pub partners: Vec<String>,
    /// Encrypted cryptographic key material
    pub key_material: Vec<u8>,
    /// Trust scores for each partner in the key agreement
    pub trust_scores: HashMap<String, f64>,
    /// Timestamp when the key was generated
    pub created_at: DateTime<Utc>,
    /// Timestamp when the key expires
    pub expires_at: DateTime<Utc>,
}

/// Active partnership between biomes or entities
///
/// Represents an established relationship with defined trust levels,
/// duration limits, and partnership type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Partnership {
    /// Unique partnership identifier
    pub id: String,
    /// Identifier of the partner entity
    pub partner_id: String,
    /// Type of partnership relationship
    pub relationship_type: PartnershipRelationshipType,
    /// Timestamp when partnership was established
    pub created_at: DateTime<Utc>,
    /// Optional expiration timestamp for temporary partnerships
    pub expires_at: Option<DateTime<Utc>>,
    /// Current trust score for this partnership (0.0-1.0)
    pub trust_score: f64,
}

/// Historical record of partnership actions and events
///
/// Tracks the history of activities within a partnership for auditing,
/// trust evaluation, and lineage verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnershipRecord {
    /// Identifier of the partnership this record belongs to
    pub partnership_id: String,
    /// Action or event that occurred (e.g., "created", "renewed", "terminated")
    pub action: String,
    /// Timestamp when the action occurred
    pub timestamp: DateTime<Utc>,
    /// Additional contextual metadata about the action
    pub metadata: HashMap<String, String>,
}

/// Configuration for partner discovery and negotiation
///
// Use canonical biome discovery configuration
pub use beardog_types::canonical::biome::discovery::BiomeDiscoveryConfig as DiscoveryConfig;

// NOTE: Original fields mapped to canonical:
// - auto_discovery → auto_discovery_partners
// - discovery_timeout_secs → base.timeout
// - max_discovery_attempts → base.max_attempts

/// Represents the cryptographic identity and sovereignty proof of a Primal entity.
///
/// This component ensures autonomous identity in a mixed human-primal partnership
/// while enabling collaborative operations with human partners.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalKeyComponent {
    /// Cryptographic identity key for the Primal entity
    pub identity_key: Vec<u8>,
    /// Digital signature proving autonomous decision-making capability
    pub autonomy_signature: Vec<u8>,
    /// Cryptographic proof of genesis and legitimate creation
    pub genesis_proof: Vec<u8>,
    /// Hash of the Primal's identity for verification
    pub identity_hash: Vec<u8>,
    /// Timestamp when this Primal identity was established
    pub creation_timestamp: DateTime<Utc>,
    /// Formal declaration of sovereignty and operational parameters
    pub sovereignty_declaration: String,
}

/// Human component of a mixed lineage key
///
/// Contains cryptographic proof of human identity and explicit consent,
/// ensuring human dignity and sovereignty are preserved throughout
/// the collaborative relationship.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanKeyComponent {
    /// Collection of human public key
    pub human_public_key: Vec<u8>,
    /// Cryptographic proof of human identity (privacy-preserving)
    pub human_identity_proof: Vec<u8>,
    /// The biometric hash value
    pub biometric_hash: beardog_genetics::BiometricHash,
    /// Timestamp of explicit consent to partnership
    pub consent_timestamp: DateTime<Utc>,
    /// List of permissions explicitly granted by the human
    /// Collection of granted permissions
    pub granted_permissions: Vec<String>,
    /// The entropy contribution value
    pub entropy_contribution: beardog_genetics::HumanEntropySource,
    /// Digital signature on partnership agreement
    /// Collection of partnership agreement signature
    pub partnership_agreement_signature: Vec<u8>,
}

/// Record of a significant lineage event
///
/// Captures important events in the life cycle of a biome's lineage,
/// including creation, evolution, partnerships, and sovereignty assertions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageEvent {
    /// Unique event identifier
    pub event_id: Uuid,
    /// Timestamp when the event occurred
    pub genesis_timestamp: DateTime<Utc>,
    /// Type of lineage event
    pub event_type: LineageEventType,
    /// Participants involved in this event
    pub participants: Vec<String>,
    /// Additional event data and metadata
    /// Mapping of event data
    pub event_data: HashMap<String, String>,
    /// Cryptographic proof of event authenticity
    /// Collection of event proof
    pub event_proof: Vec<u8>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// Types of lineage event
pub enum LineageEventType {
    /// Initial creation of a biome or lineage
    Genesis,
    /// Partnership establishment event
    PartnershipStarted,
    /// Partnership termination event
    PartnershipEnded,
    /// Creation of offspring or derivative lineage
    Spawning,
    /// Genetic crossover and mixing between lineages
    GeneticMixing,
    /// Evolutionary adaptation and trait selection
    Evolution,
    /// Sovereignty assertion event
    SovereigntyAssertion,
    /// Corporate interaction event
    CorporateInteraction,
    /// Cryptographic key rotation for security
    KeyRotation,
    /// Security incident event
    SecurityIncident,
}

/// Manages partnerships between biomes and entities
///
/// Coordinates partnership lifecycle, trust evaluation, and discovery
/// of potential partnership candidates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnershipManager {
    /// Currently active partnerships indexed by partner ID
    pub active_partnerships: HashMap<String, Partnership>,
    /// Historical record of all partnership events and changes
    pub partnership_history: Vec<PartnershipRecord>,
    /// Trust calculation algorithm configuration
    /// The trust algorithm value
    pub trust_algorithm: String,
    /// Maximum number of simultaneous partnerships
    /// Number of `max_partnerships`
    pub max_partnerships: usize,
    /// Default partnership duration in seconds
    /// Number of `default_duration`
    pub default_duration: u64,
    /// Partnership discovery and negotiation settings
    pub discovery_config: DiscoveryConfig,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// Types of partnership relationship
pub enum PartnershipRelationshipType {
    /// Equal partnership with shared responsibilities
    Equal,
    /// Primary-secondary hierarchy with defined roles
    Hierarchical,
    /// Temporary partnership with limited duration
    Temporary,
    /// Emergency partnership for crisis response
    Emergency,
    /// Research collaboration and experimentation
    Research,
    /// Commercial partnership with service agreements
    Commercial,
    /// Family-based lineage relationship
    Family,
    /// Organizational partnership within institutions
    Organizational,
    /// International cross-border partnership
    International,
    /// Anonymous partnership with privacy focus
    Anonymous,
}
