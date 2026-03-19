// SPDX-License-Identifier: AGPL-3.0-only

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_genetics::genetics::entropy_hierarchy::BiometricHash;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Mixed lineage key combining primal and human components
pub struct MixedLineageKey {
    /// The primal component value
    pub primal_component: PrimalKeyComponent,
    /// Optional human component
    pub human_component: Option<HumanKeyComponent>,
    /// The blending algorithm value
    pub blending_algorithm: String,
    /// The partnership start value
    pub partnership_start: DateTime<Utc>,
    /// Optional partnership expiration
    pub partnership_expiration: Option<DateTime<Utc>>,
    /// Mapping of partnership terms
    pub partnership_terms: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalKeyComponent {
    pub primal_id: String,
    pub genesis_timestamp: DateTime<Utc>,
    /// Collection of genesis proof
    pub genesis_proof: Vec<u8>,
    /// Collection of device binding
    pub device_binding: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanKeyComponent {
    /// Collection of partnership key
    pub partnership_key: Vec<u8>,
    /// The biometric hash value
    pub biometric_hash: BiometricHash,
    pub consent_timestamp: DateTime<Utc>,
    /// Collection of granted permissions
    pub granted_permissions: Vec<String>,
}

impl Default for MixedLineageKey {
    fn default() -> Self {
        Self {
            primal_component: PrimalKeyComponent::default(),
            human_component: None,
            blending_algorithm: "default_blend".to_string(),
            partnership_start: Utc::now(),
            partnership_expiration: None,
            partnership_terms: HashMap::new(),
        }
    }
}

impl Default for PrimalKeyComponent {
    fn default() -> Self {
        Self {
            primal_id: "default_primal".to_string(),
            genesis_timestamp: Utc::now(),
            genesis_proof: vec![],
            device_binding: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageEvent {
    pub timestamp: DateTime<Utc>,
    /// The event type value
    pub event_type: LineageEventType,
    /// Collection of participants
    pub participants: Vec<String>,
    /// Collection of event data
    pub event_data: Vec<(String, String)>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// Types of events that can occur in a primal's lineage
/// Types of lineage event
pub enum LineageEventType {
    /// Represents genesis variant
    Genesis,
    /// State indicating partnershipstarted
    PartnershipStarted,
    /// State indicating partnershipended
    PartnershipEnded,
    /// Currently spawning
    Spawning,


    /// Currently geneticmixing
    GeneticMixing,


    /// Represents evolution variant
    Evolution,
    /// Represents sovereignty assertion variant
    SovereigntyAssertion,
    /// Represents corporate interaction variant
    CorporateInteraction,
    /// Represents key rotation variant
    KeyRotation,
    /// Represents security incident variant
    SecurityIncident,
}
