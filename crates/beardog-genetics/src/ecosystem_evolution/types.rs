// SPDX-License-Identifier: AGPL-3.0-only

//! Core domain types for ecosystem evolution
//!
//! This module defines the fundamental types that represent ecosystem relationships,
//! trust dynamics, coordination models, and symbiosis types. These types replace
//! traditional binary patterns (allowlist/blocklist, trusted/untrusted, primary/replica)
//! with spectrum-based models that better reflect natural ecosystem dynamics.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::support::*;

/// Ecosystem membership spectrum - replaces binary allowlist/blocklist patterns
///
/// This enum represents different levels of participation and trust within an ecosystem,
/// moving beyond simple binary access control to a nuanced spectrum of relationships.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EcosystemMembership {
    /// Trusted ecosystem maintainer with stewardship responsibilities
    CoreSteward {
        /// Areas where this steward has responsibility
        stewardship_areas: Vec<StewardshipArea>,
        /// Trust level (0.8-1.0 range)
        trust_level: f64,
        /// Specific responsibilities assigned
        responsibilities: Vec<String>,
        /// Genetic markers indicating stewardship capabilities
        genetic_markers: Vec<GeneticMarker>,
    },

    /// Regular positive participant in the ecosystem
    ActiveContributor {
        /// Types of contributions this participant makes
        contribution_types: Vec<ContributionType>,
        /// Trust level (0.6-0.9 range)
        trust_level: f64,
        /// Summary of past interactions
        interaction_history: InteractionSummary,
        /// Potential for growth and evolution
        evolutionary_potential: f64,
    },

    /// Growing understanding, learning from the ecosystem
    LearningParticipant {
        /// Current learning trajectory
        learning_path: LearningPath,
        /// Trust level (0.4-0.7 range)
        trust_level: f64,
        /// Mentorship connections established
        mentorship_connections: Vec<MentorConnection>,
        /// Rate of adaptation and learning
        adaptation_rate: f64,
    },

    /// Temporary visitor collaborating on specific scope
    VisitingCollaborator {
        /// Scope of the collaboration
        collaboration_scope: CollaborationScope,
        /// Duration of the visit
        visit_duration: Duration,
        /// Trust level (0.5-0.8 range)
        trust_level: f64,
        /// Terms governing the collaboration
        collaboration_terms: CollaborationTerms,
    },

    /// Requires careful monitoring due to concerning patterns
    CautiousInteraction {
        /// Factors causing concern
        concern_factors: Vec<ConcernFactor>,
        /// Level of monitoring applied
        monitoring_level: MonitoringLevel,
        /// Trust level (0.2-0.5 range)
        trust_level: f64,
        /// Schedule for reviewing status
        review_schedule: ReviewSchedule,
        /// Protocols for healing and improvement
        healing_protocols: Vec<HealingProtocol>,
    },

    /// Temporarily restricted to protect ecosystem health
    EcosystemProtection {
        /// Reason for protection measures
        protection_reason: ProtectionReason,
        /// Level of protection applied
        protection_level: ProtectionLevel,
        /// Trust level (0.0-0.3 range)
        trust_level: f64,
        /// Path for restoration if available
        restoration_path: Option<RestorationPath>,
    },
}

/// Trust evolution dynamics - replaces binary trusted/untrusted states
///
/// Trust is not static but evolves through various states representing the
/// dynamic nature of relationships in an ecosystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrustEvolution {
    /// Trust is actively being built through positive interactions
    Building {
        /// Rate of trust building progress
        progress_rate: f64,
        /// Key milestones in trust building
        milestones: Vec<String>,
        /// Activities contributing to trust building
        building_activities: Vec<String>,
        /// Genetic compatibility factor
        genetic_compatibility: f64,
    },

    /// Trust relationship is stable and flourishing
    Flourishing {
        /// Metrics indicating stability
        stability_metrics: String,
        /// Indicators of flourishing relationship
        flourishing_indicators: Vec<String>,
        /// Mutual benefits of the symbiosis
        symbiotic_benefits: String,
    },

    /// Trust is being questioned due to concerns
    Questioning {
        /// Specific concerns being raised
        concerns: Vec<String>,
        /// Potential path to resolution
        resolution_path: Option<String>,
        /// Protocols for dialogue and clarification
        dialogue_protocols: String,
        /// Genetic markers indicating stress
        genetic_stress_markers: Vec<String>,
    },

    /// Trust is healing from previous damage
    Healing {
        /// Progress in recovery (0.0-1.0)
        recovery_progress: f64,
        /// Actions taken for repair
        repair_actions: Vec<String>,
        /// Timeline for healing process
        healing_timeline: String,
        /// Regenerative processes active
        regenerative_processes: Vec<String>,
    },

    /// Trust is transforming into a new form
    Transforming {
        /// Direction of evolution
        evolution_direction: String,
        /// Catalysts driving transformation
        transformation_catalysts: Vec<String>,
        /// Current stage of metamorphosis
        metamorphosis_stage: String,
    },
}

/// Symbiotic coordination models - replaces primary/replica hierarchical patterns
///
/// Coordination in ecosystems emerges from context and capability rather than
/// fixed hierarchies. This enum represents different coordination models.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoordinationModel {
    /// Collective decision making with no single authority
    Distributed {
        /// Type of consensus mechanism used
        consensus_type: String,
        /// Weights for different participants
        participation_weights: HashMap<String, f64>,
        /// Thresholds for decision making
        decision_thresholds: String,
    },

    /// Leadership rotates based on expertise and context
    Rotational {
        /// Criteria for rotation
        rotation_criteria: String,
        /// Mapping of expertise areas
        expertise_mapping: String,
        /// Schedule for rotation
        rotation_schedule: String,
        /// Current leader if applicable
        current_leader: Option<String>,
    },

    /// Authority follows competency and knowledge
    Contextual {
        /// Mapping of expertise to domains
        expertise_mapping: String,
        /// How context is evaluated
        context_evaluation: String,
        /// Delegation of authority rules
        authority_delegation: String,
        /// Current authority assignments
        current_assignments: HashMap<String, String>,
    },

    /// Shared decision making with mutual cooperation
    Collaborative {
        /// Protocol for collaborative decisions
        decision_protocol: String,
        /// Frameworks guiding collaboration
        collaboration_frameworks: Vec<String>,
        /// Mutual accountability mechanisms
        mutual_accountability: String,
        /// Active collaboration sessions
        active_sessions: Vec<String>,
    },

    /// Leadership emerges naturally from ecosystem dynamics
    Emergent {
        /// Factors enabling emergence
        emergence_factors: Vec<String>,
        /// Natural leadership patterns observed
        natural_leadership: String,
        /// Adaptive hierarchy structures
        adaptive_hierarchy: String,
        /// History of emergence events
        emergence_history: Vec<String>,
    },
}

/// Types of symbiosis - represents relationship dynamics
///
/// Symbiosis types describe the nature of relationships between ecosystem entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SymbiosisType {
    /// Both parties benefit equally from the relationship
    Mutualistic {
        /// Description of mutual benefits
        mutual_benefits: String,
        /// Balance of benefits (0.0-1.0)
        benefit_balance: f64,
        /// Sustainability metrics
        sustainability_metrics: String,
    },

    /// One benefits while the other remains neutral
    Commensal {
        /// Entity that benefits
        beneficiary: String,
        /// Entity that remains neutral
        neutral_party: String,
        /// Impact assessment
        impact_assessment: String,
    },

    /// One party actively helps the other thrive
    Facilitative {
        /// Entity providing facilitation
        facilitator: String,
        /// Entity being facilitated
        beneficiary: String,
        /// Methods of facilitation
        facilitation_methods: Vec<String>,
    },

    /// One provides security and stability to the other
    Protective {
        /// Entity providing protection
        protector: String,
        /// Entity being protected
        protected: String,
        /// Scope of protection
        protection_scope: String,
    },

    /// Healthy competition that drives improvement
    Competitive {
        /// Type of competition
        competition_type: String,
        /// Protocols ensuring fairness
        fairness_protocols: String,
        /// Metrics for improvement
        improvement_metrics: String,
    },
}
