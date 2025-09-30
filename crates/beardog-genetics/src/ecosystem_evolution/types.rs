//! # Ecosystem Evolution Types
//!
//! Supporting types and enums for ecosystem evolution genetics.

use super::core::HealthStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Ecosystem context for genetic processing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct EcosystemContext {
    /// Current ecosystem state
    pub ecosystem_state: HashMap<String, String>,
    /// Active relationships
    pub active_relationships: Vec<String>,
    /// Trust metrics
    pub trust_metrics: HashMap<String, f64>,
    /// Collaboration history
    pub collaboration_history: Vec<String>,
}

/// Ecosystem health report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemHealthReport {
    /// Overall ecosystem health status
    pub overall_health: HealthStatus,
    /// Genetic diversity score (0.0-1.0)
    pub genetic_diversity_score: f64,
    /// Adaptation capability score (0.0-1.0)
    pub adaptation_capability: f64,
    /// Symbiotic relationship strength (0.0-1.0)
    pub symbiotic_strength: f64,
    /// Trust network integrity score (0.0-1.0)
    pub trust_network_integrity: f64,
    /// Emergent behaviors observed
    pub emergent_behaviors: Vec<EmergentBehavior>,
    /// Report generation timestamp
    pub timestamp: DateTime<Utc>,
}

/// Emergent behavior in the ecosystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergentBehavior {
    /// Behavior identifier
    pub id: String,
    /// Behavior description
    pub description: String,
    /// Emergence strength (0.0-1.0)
    pub strength: f64,
    /// First observed timestamp
    pub first_observed: DateTime<Utc>,
}

/// Stewardship areas for ecosystem members
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StewardshipArea {
    /// Security and trust management
    Security,
    /// Documentation and knowledge sharing
    Documentation,
    /// Code quality and architecture
    Architecture,
    /// Community building and mentorship
    Community,
    /// Innovation and research
    Innovation,
}

/// Types of contributions to the ecosystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContributionType {
    /// Code contributions
    Code,
    /// Documentation improvements
    Documentation,
    /// Bug reports and testing
    Testing,
    /// Community support and mentorship
    Mentorship,
    /// Research and innovation
    Research,
}

/// Interaction trends in the ecosystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionTrend {
    /// Increasing positive interactions
    Improving,
    /// Stable interaction patterns
    Stable,
    /// Declining interaction quality
    Declining,
    /// Emerging new patterns
    Emerging,
}

/// Binary access patterns (being evolved away from)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinaryAccessPattern {
    /// Traditional whitelist approach
    Whitelist,
    /// Traditional blacklist approach
    Blacklist,
}

/// Binary trust patterns (being evolved away from)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinaryTrust {
    /// Complete trust
    Trusted,
    /// Complete distrust
    Untrusted,
}

/// Hierarchical patterns (being evolved away from)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HierarchicalPattern {
    /// Top-down authority
    TopDown,
    /// Strict hierarchical levels
    Hierarchical,
    /// Role-based access control
    RoleBased,
}


/// Ecosystem membership spectrum - replaces binary whitelist/blacklist patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EcosystemMembership {
    /// Trusted ecosystem maintainer with stewardship responsibilities
    CoreSteward {
        stewardship_areas: Vec<StewardshipArea>,
        trust_level: f64, // 0.8-1.0
        responsibilities: Vec<String>,
    },
    /// Regular positive participant in the ecosystem
    ActiveContributor {
        contribution_types: Vec<ContributionType>,
        trust_level: f64, // 0.6-0.9
        evolutionary_potential: f64,
    },
    /// Growing understanding, learning from the ecosystem
    LearningParticipant {
        trust_level: f64, // 0.4-0.7
        adaptation_rate: f64,
    },
}

/// Trust evolution dynamics - replaces binary trusted/untrusted states
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrustEvolution {
    /// Trust is actively being built through positive interactions
    Building {
        progress_rate: f64,
        milestones: Vec<String>,
        building_activities: Vec<String>,
        genetic_compatibility: f64,
    },
    /// Trust relationship is stable and flourishing
    Flourishing {
        stability_metrics: String,
        flourishing_indicators: Vec<String>,
        symbiotic_benefits: String,
    },
}

/// Symbiotic coordination models - replaces master/slave hierarchical patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoordinationModel {
    /// Collective decision making with no single authority
    Distributed {
        consensus_type: String,
        participation_weights: HashMap<String, f64>,
        decision_thresholds: String,
    },
    /// Leadership rotates based on expertise and context
    Rotational {
        rotation_criteria: String,
        expertise_mapping: String,
        rotation_schedule: String,
        current_leader: Option<String>,
    },
}

/// Types of symbiosis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SymbiosisType {
    /// Both parties benefit equally from the relationship
    Mutualistic {
        mutual_benefits: String,
        benefit_balance: f64,
        sustainability_metrics: String,
    },
    /// One benefits while the other remains neutral
    Commensal {
        beneficiary: String,
        neutral_party: String,
        impact_assessment: String,
    },
}

/// Relationship history for tracking interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct RelationshipHistory {
    /// Collection of interactions
    pub interactions: Vec<String>,
    /// Collection of trust changes
    pub trust_changes: Vec<String>,
    /// Collection of collaboration outcomes
    pub collaboration_outcomes: Vec<String>,
}

impl Default for EmergentBehavior {
    fn default() -> Self {
        Self {
            id: "default".to_string(),
            description: "Default emergent behavior".to_string(),
            strength: 0.0,
            first_observed: Utc::now(),
        }
    }
}

 