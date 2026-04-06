// SPDX-License-Identifier: AGPL-3.0-or-later

//! Supporting types for ecosystem evolution genetics
//!
//! This module contains helper types, enums, and structs used throughout
//! the ecosystem evolution system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ============================================================================
// Supporting Types and Enums
// ============================================================================

/// Area of responsibility a node or operator stewards (security, health, knowledge, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StewardshipArea {
    /// Currently securitymonitoring
    SecurityMonitoring,
    /// Represents resource allocation variant
    ResourceAllocation,
    /// Represents community support variant
    CommunitySupport,
    /// Represents technical maintenance variant
    TechnicalMaintenance,
    /// Represents ecosystem health variant
    EcosystemHealth,
    /// Represents conflict resolution variant
    ConflictResolution,
    /// Currently knowledgesharing
    KnowledgeSharing,
}

/// Kinds of contributions that strengthen the ecosystem and affect trust evolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContributionType {
    /// Represents code contribution variant
    CodeContribution,
    /// Represents documentation improvement variant
    DocumentationImprovement,
    /// Represents community support variant
    CommunitySupport,
    /// Currently bugreporting
    BugReporting,
    /// Represents feature suggestions variant
    FeatureSuggestions,
    /// Currently ecosystemtesting
    EcosystemTesting,
    /// Currently securityauditing
    SecurityAuditing,
    /// Performance tuning or optimization work that benefits shared infrastructure.
    PerformanceOptimization,
}

/// Rolling summary of how two actors have interacted—feeds trust and coordination models.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionSummary {
    /// Number of `total_interactions`
    pub total_interactions: u64,
    /// Number of `positive_interactions`
    pub positive_interactions: u64,
    /// Number of `neutral_interactions`
    pub neutral_interactions: u64,
    /// Number of `concerning_interactions`
    pub concerning_interactions: u64,
    /// The last interaction value
    pub last_interaction: DateTime<Utc>,
    /// The interaction trend value
    pub interaction_trend: InteractionTrend,
    /// The relationship quality value
    pub relationship_quality: f64,
}

/// Direction of change in relationship quality derived from recent interactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionTrend {
    /// Represents improving variant
    Improving,
    /// State indicating stable
    Stable,
    /// Represents declining variant
    Declining,
}

/// Typed tag with expression strength—analogous to a genetic marker in the trust genome.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticMarker {
    /// The marker type value
    pub marker_type: String,
    /// The expression level value
    pub expression_level: f64,
}

/// Suggested progression for an actor to deepen capability or trust within the ecosystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPath {
    /// The current stage value
    pub current_stage: String,
    /// Collection of next milestones
    pub next_milestones: Vec<String>,
    /// Collection of learning resources
    pub learning_resources: Vec<String>,
}

/// Active mentorship edge: who guides whom and how strongly that bond influences trust.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentorConnection {
    /// Stable identifier of the mentoring party.
    pub mentor_id: String,
    /// The mentorship type value
    pub mentorship_type: String,
    /// The connection strength value
    pub connection_strength: f64,
}

/// Declared scope of a collaboration: where partners work together and how success is judged.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationScope {
    /// Collection of areas
    pub areas: Vec<String>,
    /// Collection of limitations
    pub limitations: Vec<String>,
    /// Collection of success metrics
    pub success_metrics: Vec<String>,
}

/// Contract-like obligations and exit criteria for a collaboration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationTerms {
    /// Collection of mutual obligations
    pub mutual_obligations: Vec<String>,
    /// Collection of success criteria
    pub success_criteria: Vec<String>,
    /// Collection of termination conditions
    pub termination_conditions: Vec<String>,
}

/// Reasons an actor or interaction may be flagged for heightened review.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConcernFactor {
    /// Represents unknown entity variant
    UnknownEntity,
    /// Represents previous violations variant
    PreviousViolations,
    /// Represents suspicious behavior variant
    SuspiciousBehavior,
    /// Represents resource overuse variant
    ResourceOveruse,
    /// Represents trust violation variant
    TrustViolation,
}

/// How closely the ecosystem observes an actor after risk signals.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MonitoringLevel {
    /// Represents basic variant
    Basic,
    /// State indicating enhanced
    Enhanced,
    /// Represents intensive variant
    Intensive,
    /// Represents quarantine variant
    Quarantine,
}

/// When the next relationship or trust review occurs and how often reviews repeat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewSchedule {
    /// The next review value
    pub next_review: DateTime<Utc>,
    /// The review frequency value
    pub review_frequency: Duration,
}

/// Named steps and success signals to recover trust or operational health after an incident.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealingProtocol {
    /// Name of the protocol
    pub protocol_name: String,
    /// Collection of healing steps
    pub healing_steps: Vec<String>,
    /// Collection of success indicators
    pub success_indicators: Vec<String>,
}

/// Justification for placing an actor or resource under protective constraints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProtectionReason {
    /// Represents previous concerns variant
    PreviousConcerns,
    /// Represents security threat variant
    SecurityThreat,
    /// Represents resource protection variant
    ResourceProtection,
    /// Represents ecosystem health variant
    EcosystemHealth,
}

/// Strength of safeguards applied (rate limits, isolation) under protection policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProtectionLevel {
    /// Represents minimal variant
    Minimal,
    /// Represents moderate variant
    Moderate,
    /// Represents high variant
    High,
    /// Represents maximum variant
    Maximum,
}

/// Planned sequence to restore normal operation after degradation or quarantine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestorationPath {
    /// Collection of steps
    pub steps: Vec<String>,
    /// Expected duration to complete the restoration plan.
    pub timeline: Duration,
    /// Collection of success criteria
    pub success_criteria: Vec<String>,
}

// ============================================================================
// Ecosystem Context and Health
// ============================================================================

use beardog_types::canonical::HealthStatus;

/// Snapshot of ecosystem vitality: load, open relationships, and recent notable events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemContext {
    /// The current health value
    pub current_health: HealthStatus,
    /// Number of `active_relationships`
    pub active_relationships: usize,
    /// The ecosystem load value
    pub ecosystem_load: f64,
    /// Collection of recent events
    pub recent_events: Vec<String>,
}

/// Aggregated health view across relationships, trust, coordination, and emergent behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemHealthReport {
    /// The overall health value
    pub overall_health: HealthStatus,
    /// The relationship health value
    pub relationship_health: f64,
    /// The trust health value
    pub trust_health: f64,
    /// The coordination health value
    pub coordination_health: f64,
    /// Collection of emergent capabilities
    pub emergent_capabilities: Vec<EmergentBehavior>,
    /// Collection of recommendations
    pub recommendations: Vec<String>,
}

/// Behavior that arises from many actors—positive or risky—used for adaptive policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergentBehavior {
    /// The behavior type value
    pub behavior_type: String,
    /// The emergence strength value
    pub emergence_strength: f64,
    /// Collection of participants
    pub participants: Vec<String>,
    /// Whether beneficial is enabled
    pub beneficial: bool,
}

// ============================================================================
// Input Pattern Types
// ============================================================================

/// Legacy allow/block/simple patterns migrated into richer [`EcosystemMembership`](crate::ecosystem_evolution::EcosystemMembership) models.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinaryAccessPattern {
    /// Represents allowlist variant
    Allowlist {
        /// Collection of allowed entities
        allowed_entities: Vec<String>,
    },
    /// Represents blocklist variant
    Blocklist {
        /// Collection of blocked entities
        blocked_entities: Vec<String>,
    },
    /// Represents simple variant
    Simple {
        /// Whether `is_allowed` is enabled
        is_allowed: bool,
    },
}

/// Coarse trust bit used when migrating from binary trust lists to graduated models.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinaryTrust {
    /// State indicating trusted
    Trusted,
    /// State indicating untrusted
    Untrusted,
}

/// Time series of interactions, trust deltas, and collaboration outcomes between actors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipHistory {
    /// Collection of interactions
    pub interactions: Vec<String>,
    /// Collection of trust changes
    pub trust_changes: Vec<f64>,
    /// Collection of collaboration outcomes
    pub collaboration_outcomes: Vec<String>,
}

/// Topology of dependence between nodes (primary/replica, client/server, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HierarchicalPattern {
    /// Primary-replica database pattern
    PrimaryReplica {
        /// The primary value
        primary: String,
        /// Collection of replicas
        replicas: Vec<String>,
    },
    /// Client-server pattern
    ClientServer {
        /// The server value
        server: String,
        /// Collection of clients
        clients: Vec<String>,
    },
    /// Primary-secondary pattern
    PrimarySecondary {
        /// The primary value
        primary: String,
        /// Collection of secondaries
        secondaries: Vec<String>,
    },
}

// ============================================================================
// Supporting Structs for Genetics Modules
// ============================================================================

/// Rules and triggers that move members between ecosystem roles over time.
#[derive(Debug, Clone, Default)]
pub struct MembershipEvolution {
    /// Collection of evolution rules
    pub evolution_rules: Vec<String>,
    /// Collection of progression paths
    pub progression_paths: Vec<String>,
    /// Collection of transition triggers
    pub transition_triggers: Vec<String>,
}

/// Algorithms and decay that turn raw signals into a scalar or vector trust score.
#[derive(Debug, Clone, Default)]
pub struct TrustComputation {
    /// Collection of trust metrics
    pub trust_metrics: Vec<String>,
    /// Collection of computation algorithms
    pub computation_algorithms: Vec<String>,
    /// The temporal decay value
    pub temporal_decay: f64,
}

/// Valid transitions between membership states, cool-downs, and validation gates.
#[derive(Debug, Clone, Default)]
pub struct MembershipTransitions {
    /// Transition rules
    pub transition_rules: HashMap<String, String>,
    /// Collection of validation criteria
    pub validation_criteria: Vec<String>,
    /// The cool down periods value
    pub cooldown_periods: HashMap<String, u64>,
}

/// How coordination strategies mutate under load, failure, or trust shifts.
#[derive(Debug, Clone, Default)]
pub struct CoordinationEvolution {
    /// Collection of evolution patterns
    pub evolution_patterns: Vec<String>,
    /// Collection of adaptation methods
    pub adaptation_methods: Vec<String>,
    /// The optimization goals value
    pub optimization_goals: HashMap<String, f64>,
}

/// Criteria and rotation for emergent leaders without a fixed centralized appointee.
#[derive(Debug, Clone, Default)]
pub struct LeadershipEmergence {
    /// Collection of emergence criteria
    pub emergence_criteria: Vec<String>,
    /// Collection of rotation policies
    pub rotation_policies: Vec<String>,
    /// Collection of selection algorithms
    pub selection_algorithms: Vec<String>,
}

/// Mediation and prevention patterns when symbiotic partners disagree or compete.
#[derive(Debug, Clone, Default)]
pub struct ConflictResolution {
    /// Collection of resolution strategies
    pub resolution_strategies: Vec<String>,
    /// Collection of mediation protocols
    pub mediation_protocols: Vec<String>,
    /// Collection of prevention measures
    pub prevention_measures: Vec<String>,
}

/// Metrics and calibration used to keep trust scores comparable across nodes.
#[derive(Debug, Clone, Default)]
pub struct TrustMeasurement {
    /// Collection of measurement metrics
    pub measurement_metrics: Vec<String>,
    /// The calibration methods value
    pub calibration_methods: HashMap<String, f64>,
    /// Collection of validation techniques
    pub validation_techniques: Vec<String>,
}

/// Voting, consensus, and aggregation models for group decisions in the ecosystem.
#[derive(Debug, Clone, Default)]
pub struct CollectiveDecisionMaking {
    /// Collection of decision models
    pub decision_models: Vec<String>,
    /// The voting mechanisms value
    pub voting_mechanisms: HashMap<String, String>,
    /// Collection of consensus algorithms
    pub consensus_algorithms: Vec<String>,
}

/// Retention and recall policies so the ecosystem learns without resource hoarding.
#[derive(Debug, Clone, Default)]
pub struct EcosystemMemory {
    /// The retention policies value
    pub retention_policies: HashMap<String, u64>,
    /// Collection of retrieval methods
    pub retrieval_methods: Vec<String>,
    /// The learning integration value
    pub learning_integration: f64,
}

/// Forecasting models that anticipate load, risk, or opportunity from historical signals.
#[derive(Debug, Clone, Default)]
pub struct PredictiveCapabilities {
    /// Collection of prediction models
    pub prediction_models: Vec<String>,
    /// Collection of accuracy metrics
    pub accuracy_metrics: Vec<f64>,
    /// Collection of adaptation mechanisms
    pub adaptation_mechanisms: Vec<String>,
}

/// Detects collective anomalies or beneficial swarm behaviors from telemetry.
#[derive(Debug, Clone, Default)]
pub struct EmergentBehaviorDetection {
    /// Collection of detection algorithms
    pub detection_algorithms: Vec<String>,
    /// Collection of pattern recognition
    pub pattern_recognition: Vec<String>,
    /// The response protocols value
    pub response_protocols: HashMap<String, String>,
}

/// Tunes relationship parameters to maximize quality metrics under constraints.
#[derive(Debug, Clone, Default)]
pub struct RelationshipOptimization {
    /// The optimization algorithms value
    pub optimization_algorithms: HashMap<String, f64>,
    /// Collection of quality metrics
    pub quality_metrics: Vec<String>,
    /// Collection of improvement strategies
    pub improvement_strategies: Vec<String>,
}

/// Weights contextual factors (load, locality, risk) when evaluating interactions.
#[derive(Debug, Clone, Default)]
pub struct ContextAwareness {
    /// Collection of context factors
    pub context_factors: Vec<String>,
    /// The weighting schemes value
    pub weighting_schemes: HashMap<String, f64>,
    /// Collection of adaptation triggers
    pub adaptation_triggers: Vec<String>,
}

/// Protocols to rebuild trust and operational coupling after a breach or drift.
#[derive(Debug, Clone, Default)]
pub struct RelationshipRepair {
    /// Collection of repair protocols
    pub repair_protocols: Vec<String>,
    /// Collection of healing mechanisms
    pub healing_mechanisms: Vec<String>,
    /// Collection of success criteria
    pub success_criteria: Vec<String>,
}

/// Scores a situation before action—feeds contextual decision genetics.
#[derive(Debug, Clone, Default)]
pub struct ContextEvaluationEngine {
    /// Collection of evaluation criteria
    pub evaluation_criteria: Vec<String>,
    /// The scoring algorithms value
    pub scoring_algorithms: HashMap<String, f64>,
    /// Collection of decision factors
    pub decision_factors: Vec<String>,
}

/// Feedback loop that adjusts decision rules as outcomes and trust evolve.
#[derive(Debug, Clone, Default)]
pub struct DecisionAdaptation {
    /// Collection of adaptation rules
    pub adaptation_rules: Vec<String>,
    /// Collection of learning mechanisms
    pub learning_mechanisms: Vec<String>,
    /// The feedback integration value
    pub feedback_integration: f64,
}
