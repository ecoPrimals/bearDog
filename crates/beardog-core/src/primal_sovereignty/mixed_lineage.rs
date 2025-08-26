

use beardog_genetics::genetics::entropy_hierarchy::{
    BiometricHash, HumanEntropySource, OwnershipProof,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixedLineageKey {

    pub primal_component: PrimalKeyComponent,

    pub human_component: Option<HumanKeyComponent>,

    pub blending_algorithm: String,

    pub partnership_start: DateTime<Utc>,

    pub partnership_expiration: Option<DateTime<Utc>>,

    pub partnership_terms: HashMap<String, String>,
}

pub struct PrimalKeyComponent {

    pub identity_key: Vec<u8>,

    pub autonomy_signature: Vec<u8>,

    pub genesis_proof: Vec<u8>,

    pub identity_hash: Vec<u8>,

    pub creation_timestamp: DateTime<Utc>,

    pub sovereignty_assertion: String,

pub struct HumanKeyComponent {

    pub partnership_key: Vec<u8>,

    pub human_identity_proof: Vec<u8>,

    pub biometric_hash: BiometricHash,

    pub consent_timestamp: DateTime<Utc>,

    pub granted_permissions: Vec<String>,

    pub entropy_contribution: HumanEntropySource,

    pub partnership_agreement_signature: Vec<u8>,

pub struct LineageEvent {

    pub event_id: String,

    pub timestamp: DateTime<Utc>,

    pub event_type: LineageEventType,

    pub participants: Vec<String>,

    pub event_data: HashMap<String, String>,

    pub event_proof: Vec<u8>,

pub enum LineageEventType {

    Genesis,

    PartnershipStarted,

    PartnershipEnded,

    Spawning,

    GeneticMixing,

    Evolution,
    SovereigntyAssertion,

    CorporateInteraction,

    KeyRotation,

    SecurityIncident,
} 
