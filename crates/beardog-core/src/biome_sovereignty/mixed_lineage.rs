// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixedLineageKey {
    pub id: String,
    /// List of partner identities in the key sharing
    /// Collection of partners
    pub partners: Vec<String>,
    /// Cryptographic key material (encrypted)
    /// Collection of key material
    pub key_material: Vec<u8>,
    /// Mapping of trust scores
    pub trust_scores: HashMap<String, f64>,
    /// Key generation timestamp
    /// The created at value
    pub created_at: DateTime<Utc>,
    /// Key expiration timestamp
    /// The expires at value
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Partnership {
    /// Unique partnership identifier
    pub id: String,
    pub partner_id: String,
    /// Partnership relationship type
    /// The relationship type value
    pub relationship_type: PartnershipRelationshipType,
    /// Partnership creation timestamp
    /// The created at value
    pub created_at: DateTime<Utc>,
    /// Partnership expiration timestamp
    /// Optional expires at
    pub expires_at: Option<DateTime<Utc>>,
    /// The trust score value
    pub trust_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnershipRecord {
    /// Partnership identifier
    pub partnership_id: String,
    /// The action value
    pub action: String,
    /// Timestamp of the action
    pub timestamp: DateTime<Utc>,
    /// Additional metadata about the action
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    /// Enable automatic partner discovery
    /// Whether `auto_discovery` is enabled
    pub auto_discovery: bool,
    /// Discovery timeout in seconds
    pub discovery_timeout_secs: u64,
    /// Maximum discovery attempts
    /// Number of `max_discovery_attempts`
    pub max_discovery_attempts: u32,
}

/// Represents the cryptographic identity and sovereignty proof of a Primal entity.
///
/// This component ensures autonomous identity in a mixed human-primal partnership
/// while enabling collaborative operations with human partners.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalKeyComponent {
    pub identity_key: Vec<u8>,
    /// Digital signature proving autonomous decision-making capability
    /// Collection of autonomy signature
    pub autonomy_signature: Vec<u8>,
    /// Cryptographic proof of genesis and legitimate creation
    /// Collection of genesis proof
    pub genesis_proof: Vec<u8>,
    pub identity_hash: Vec<u8>,
    /// Timestamp when this Primal identity was established
    pub creation_timestamp: DateTime<Utc>,
    /// Formal declaration of sovereignty and operational parameters
    /// The sovereignty declaration value
    pub sovereignty_declaration: String,
}

///
/// Contains cryptographic proof of human identity and explicit consent
/// are preserved throughout the collaborative relationship.
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageEvent {
    pub event_id: Uuid,
    /// Timestamp when the genesis event occurred
    pub genesis_timestamp: DateTime<Utc>,
    /// Type of lineage event that occurred
    /// The event type value
    pub event_type: LineageEventType,
    /// List of participants involved in this event
    /// Collection of participants
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
    Genesis,
    /// Partnership establishment event
    PartnershipStarted,
    /// Partnership termination event
    PartnershipEnded,
    Spawning,
    GeneticMixing,
    Evolution,
    /// Sovereignty assertion event
    SovereigntyAssertion,
    /// Corporate interaction event
    CorporateInteraction,
    KeyRotation,
    /// Security incident event
    SecurityIncident,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnershipManager {
    /// Active partnerships indexed by partner ID
    /// Mapping of active partnerships
    pub active_partnerships: HashMap<String, Partnership>,
    /// Collection of partnership history
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
    Temporary,
    Emergency,
    Research,
    /// Commercial partnership with service agreements
    Commercial,
    Family,
    /// Organizational partnership within institutions
    Organizational,
    International,
    /// Anonymous partnership with privacy focus
    Anonymous,
}
