// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(missing_docs)]

//! Core coordination configuration and model enum.

use super::collaboration::{CollaborationFramework, DecisionProtocol, MutualAccountability};
use super::consensus_rotation::{
    ConsensusType, DecisionThresholds, ExpertiseMapping, LeaderInfo, RotationCriteria,
    RotationSchedule,
};
use super::context_authority::{AuthorityDelegation, ContextEvaluation};
use super::ecosystem::EcosystemIntegrationConfig;
use super::supporting::{
    AdaptiveHierarchy, CollaborationSession, EmergenceEvent, EmergenceFactor, NaturalLeadership,
};
use super::transition_health::{CoordinationHealthConfig, CoordinationTransitionConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

/// Evolved coordination configuration that replaces primary/replica patterns
/// with symbiotic coordination models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationConfig {
    /// The coordination model value
    pub coordination_model: CoordinationModel,

    /// Fallback coordination models in case primary fails
    /// Collection of fallback models
    pub fallback_models: Vec<CoordinationModel>,

    pub transition_config: CoordinationTransitionConfig,

    /// The health monitoring value
    pub health_monitoring: CoordinationHealthConfig,

    /// Integration with ecosystem genetics
    /// The ecosystem integration value
    pub ecosystem_integration: EcosystemIntegrationConfig,
}

/// Symbiotic coordination models - replaces primary/replica hierarchical patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoordinationModel {
    /// Collective decision making with no single authority
    Distributed {
        /// Type of consensus mechanism used
        consensus_type: ConsensusType,
        participation_weights: HashMap<String, f64>,
        decision_thresholds: DecisionThresholds,
        consensus_timeout: Duration,
    },

    /// Leadership rotates based on expertise and context
    Rotational {
        rotation_criteria: RotationCriteria,
        /// Mapping of expertise areas to participants
        expertise_mapping: ExpertiseMapping,
        rotation_schedule: RotationSchedule,
        current_leader: Option<LeaderInfo>,
    },

    /// Authority follows competency and knowledge
    Contextual {
        /// Mapping of expertise areas to participants
        expertise_mapping: ExpertiseMapping,
        context_evaluation: ContextEvaluation,
        authority_delegation: AuthorityDelegation,
        /// Current context and authority assignments
        current_assignments: HashMap<Arc<str>, Arc<str>>,
    },

    /// Shared decision making with mutual cooperation
    Collaborative {
        decision_protocol: DecisionProtocol,
        collaboration_frameworks: Vec<CollaborationFramework>,
        mutual_accountability: MutualAccountability,
        /// Active collaboration sessions
        active_sessions: Vec<CollaborationSession>,
    },

    /// Leadership emerges naturally from ecosystem dynamics
    Emergent {
        /// Factors that contribute to leadership emergence
        emergence_factors: Vec<EmergenceFactor>,
        /// Current natural leadership structure
        natural_leadership: NaturalLeadership,
        /// Adaptive hierarchy that changes based on needs
        adaptive_hierarchy: AdaptiveHierarchy,
        /// History of leadership emergence patterns
        emergence_history: Vec<EmergenceEvent>,
    },
}
