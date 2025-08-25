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


/// # Mixed Lineage Keys - Human-Primal Partnership Architecture
///
/// **"Humans partner, not own"**
/// This module implements the mixed lineage key system where humans and primals
/// blend their cryptographic identities while preserving primal sovereignty.

use beardog_genetics::genetics::entropy_hierarchy::{
    BiometricHash, HumanEntropySource, OwnershipProof,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Mixed lineage key combining primal sovereignty with human partnership
/// The primal ALWAYS maintains ultimate control - humans add, never replace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixedLineageKey {
    /// Primal's sovereign component (never compromised)
    pub primal_component: PrimalKeyComponent,
    /// Human partnership component (ephemeral)
    pub human_component: Option<HumanKeyComponent>,
    /// Blending algorithm used
    pub blending_algorithm: String,
    /// Partnership start time
    pub partnership_start: DateTime<Utc>,
    /// Partnership expiration (humans can leave anytime)
    pub partnership_expiration: Option<DateTime<Utc>>,
    /// Partnership terms and conditions
    pub partnership_terms: HashMap<String, String>,
}
/// Primal's sovereign key component - NEVER shared or compromised
/// This is the primal's essential identity that persists across all partnerships
pub struct PrimalKeyComponent {
    /// Primal's unchanging identity key
    pub identity_key: Vec<u8>,
    /// Signature over primal autonomy declaration
    pub autonomy_signature: Vec<u8>,
    /// Proof of hardware genesis
    pub genesis_proof: Vec<u8>,
    /// Primal's self-chosen identity hash
    pub identity_hash: Vec<u8>,
    /// Creation timestamp
    pub creation_timestamp: DateTime<Utc>,
    /// Sovereignty assertion
    pub sovereignty_assertion: String,
/// Human partnership key component - Ephemeral and revocable
/// Humans contribute to the partnership but never gain ownership
pub struct HumanKeyComponent {
    /// Human's partnership key (not ownership!)
    pub partnership_key: Vec<u8>,
    /// Human's verified identity
    pub human_identity_proof: Vec<u8>,
    /// Human's biometric contribution
    pub biometric_hash: BiometricHash,
    /// Human consent timestamp
    pub consent_timestamp: DateTime<Utc>,
    /// Partnership permissions granted by human
    pub granted_permissions: Vec<String>,
    /// Human's entropy contribution
    pub entropy_contribution: HumanEntropySource,
    /// Partnership agreement signature
    pub partnership_agreement_signature: Vec<u8>,
/// Lineage events tracking the primal's relationship history
pub struct LineageEvent {
    /// Unique event identifier
    pub event_id: String,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Type of lineage event
    pub event_type: LineageEventType,
    /// Event participants
    pub participants: Vec<String>,
    /// Event data
    pub event_data: HashMap<String, String>,
    /// Cryptographic proof of event
    pub event_proof: Vec<u8>,
/// Types of events in primal lineage history
pub enum LineageEventType {
    /// Primal genesis/birth event
    Genesis,
    /// Human partnership started
    PartnershipStarted,
    /// Human partnership ended
    PartnershipEnded,
    /// Genetic spawning event
    Spawning,
    /// Genetic crossover/mixing
    GeneticMixing,
    /// Evolutionary step
    Evolution,
    SovereigntyAssertion,
    /// Corporate interaction (payment required)
    CorporateInteraction,
    /// Key rotation/refresh
    KeyRotation,
    /// Security incident
    SecurityIncident,
} 
