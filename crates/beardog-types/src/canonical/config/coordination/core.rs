//! Core coordination configuration types
//!
//! This module contains the fundamental coordination models and configuration
//! that enable symbiotic coordination patterns.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::{
    consensus::{DecisionProtocol, CollaborationFramework, MutualAccountability},
    transition::{CoordinationTransitionConfig, CoordinationHealthConfig},
    ecosystem::EcosystemIntegrationConfig,
};

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

/// Symbiotic coordination models that replace primary/replica hierarchies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoordinationModel {
    /// Distributed consensus-based coordination
    Distributed {
        consensus_type: ConsensusType,
        decision_thresholds: DecisionThresholds,
    },
    
    /// Rotational leadership based on expertise
    Rotational {
        rotation_criteria: RotationCriteria,
        expertise_mapping: ExpertiseMapping,
    },
    
    /// Contextual authority based on situational expertise
    Contextual {
        context_evaluation: ContextEvaluation,
        authority_delegation: AuthorityDelegation,
    },
    
    /// Collaborative mutual cooperation
    Collaborative {
        decision_protocol: DecisionProtocol,
        collaboration_frameworks: Vec<CollaborationFramework>,
        mutual_accountability: MutualAccountability,
    },
    
    /// Emergent natural leadership
    Emergent {
        emergence_factors: Vec<EmergenceFactor>,
        natural_leadership: NaturalLeadership,
    },
}

/// Consensus mechanisms for distributed coordination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusType {
    /// Simple majority consensus
    Majority,
    /// Requires supermajority (2/3+)
    Supermajority,
    /// Requires unanimous agreement
    Unanimous,
    /// Weighted consensus based on expertise
    WeightedExpertise,
    /// Adaptive consensus that changes based on context
    Adaptive,
}

/// Thresholds for decision making
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionThresholds {
    /// Minimum participation percentage required
    pub min_participation: f64,
    /// Consensus threshold (0.5 = majority, 0.67 = supermajority)
    pub consensus_threshold: f64,
    /// Maximum time allowed for decision making
    pub decision_timeout: Duration,
}

/// Criteria for leadership rotation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationCriteria {
    /// Time-based rotation interval
    pub time_interval: Option<Duration>,
    /// Performance-based rotation triggers
    pub performance_thresholds: Vec<PerformanceThreshold>,
    /// Workload-based rotation triggers
    pub workload_thresholds: WorkloadThresholds,
}

/// Mapping of expertise areas to participants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpertiseMapping {
    /// Map of expertise area to participant expertise entries
    pub expertise_areas: HashMap<String, Vec<ExpertiseEntry>>,
    /// Current expertise assessments
    pub current_assessments: HashMap<String, f64>,
}

/// Individual expertise entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpertiseEntry {
    /// Participant identifier
    pub participant_id: String,
    /// Expertise level (0.0 to 1.0)
    pub expertise_level: f64,
    /// Verification method for expertise
    pub verification_method: String,
    /// Last assessment timestamp
    pub last_assessed: chrono::DateTime<chrono::Utc>,
}

/// Context evaluation for situational authority
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextEvaluation {
    /// Context factors to evaluate
    pub context_factors: Vec<ContextFactor>,
    /// Current context assessment
    pub current_assessment: ContextAssessment,
    /// Evaluation frequency
    pub evaluation_frequency: Duration,
}

/// Factors that influence contextual authority
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContextFactor {
    /// System load and performance
    SystemLoad { threshold: f64 },
    /// Security threat level
    SecurityThreat { level: String },
    /// Time sensitivity of decisions
    TimeSensitivity { urgency: f64 },
    /// Domain expertise requirements
    ExpertiseRequired { domain: String, min_level: f64 },
    /// Resource availability
    ResourceAvailability { resource_type: String, availability: f64 },
}

/// Current context assessment results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextAssessment {
    /// Overall context score
    pub context_score: f64,
    /// Recommended authority level
    pub recommended_authority: String,
    /// Assessment timestamp
    pub assessed_at: chrono::DateTime<chrono::Utc>,
    /// Assessment validity period
    pub valid_until: chrono::DateTime<chrono::Utc>,
}

/// Authority delegation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorityDelegation {
    /// Delegation rules
    pub delegation_rules: Vec<DelegationRule>,
    /// Current active delegations
    pub active_delegations: Vec<Delegation>,
}

/// Rule for authority delegation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationRule {
    /// Rule identifier
    pub rule_id: String,
    /// Conditions that trigger delegation
    pub trigger_conditions: Vec<String>,
    /// Authority level to delegate
    pub authority_level: String,
    /// Duration of delegation
    pub delegation_duration: Duration,
}

/// Active delegation instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delegation {
    /// Delegation identifier
    pub delegation_id: String,
    /// Participant receiving authority
    pub delegate: String,
    /// Authority level delegated
    pub authority_level: String,
    /// Delegation start time
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// Delegation expiry time
    pub expires_at: chrono::DateTime<chrono::Utc>,
    /// Current status
    pub status: DelegationStatus,
}

/// Status of a delegation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DelegationStatus {
    /// Delegation is active
    Active,
    /// Delegation is suspended
    Suspended,
    /// Delegation has expired
    Expired,
    /// Delegation was revoked
    Revoked,
}

/// Factors that influence emergent leadership
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergenceFactor {
    /// Factor name
    pub factor_name: String,
    /// Current factor value
    pub current_value: f64,
    /// Weight in emergence calculation
    pub weight: f64,
    /// Trend over time
    pub trend: String,
}

/// Natural leadership emergence tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NaturalLeadership {
    /// Current natural leaders
    pub current_leaders: Vec<String>,
    /// Leadership emergence scores
    pub emergence_scores: HashMap<String, f64>,
    /// Leadership transition history
    pub transition_history: Vec<String>,
}

/// Performance threshold for rotation triggers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThreshold {
    /// Metric name
    pub metric_name: String,
    /// Threshold value
    pub threshold_value: f64,
    /// Comparison operator (>, <, ==)
    pub comparison: String,
}

/// Workload thresholds for rotation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadThresholds {
    /// Maximum CPU utilization
    pub max_cpu_utilization: f64,
    /// Maximum memory usage
    pub max_memory_usage: f64,
    /// Maximum concurrent requests
    pub max_concurrent_requests: u32,
}

impl Default for CoordinationConfig {
    fn default() -> Self {
        Self {
            coordination_model: CoordinationModel::Collaborative {
                decision_protocol: DecisionProtocol::default(),
                collaboration_frameworks: Vec::new(),
                mutual_accountability: MutualAccountability::default(),
            },
            fallback_models: vec![
                CoordinationModel::Distributed {
                    consensus_type: ConsensusType::Majority,
                    decision_thresholds: DecisionThresholds::default(),
                }
            ],
            transition_config: CoordinationTransitionConfig::default(),
            health_monitoring: CoordinationHealthConfig::default(),
            ecosystem_integration: EcosystemIntegrationConfig::default(),
        }
    }
}

impl Default for DecisionThresholds {
    fn default() -> Self {
        Self {
            min_participation: 0.6,
            consensus_threshold: 0.6,
            decision_timeout: Duration::from_secs(300), // 5 minutes
        }
    }
} 