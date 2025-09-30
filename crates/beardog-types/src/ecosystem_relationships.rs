//! Ecosystem Relationship Evolution
//! 
//! Implementation of spectrum-based relationship patterns that replace binary
//! access control with biological ecosystem modeling.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Ecosystem membership spectrum replacing binary whitelist/blacklist
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EcosystemMembership { /// Trusted ecosystem maintainer with full stewardship rights
    CoreSteward {
        /// Areas of stewardship responsibility
        stewardship_areas: Vec<String>,
        /// Trust level (0.0-1.0)
        trust_level: f64 },
    /// Regular positive participant with established contributions
    ActiveContributor { /// Types of contributions made
        contribution_types: Vec<String>,
        /// Contribution quality score (0.0-1.0)
        contribution_score: f64 },
    /// Growing participant with learning path
    LearningParticipant { /// Current learning path and progress
        learning_path: LearningPath,
        /// Metrics tracking learning progress
        progress_metrics: ProgressMetrics },
    /// Temporary partnership with specific collaboration scope
    VisitingCollaborator { /// Defined scope of collaboration
        collaboration_scope: CollaborationScope,
        /// Duration of the visit/collaboration
        visit_duration: std::time::Duration },
    /// Requires careful monitoring due to concern factors
    CautiousInteraction { /// Factors causing concern
        concern_factors: Vec<ConcernFactor>,
        /// Level of monitoring required
        monitoring_level: MonitoringLevel },
    /// Temporarily restricted with restoration path
    EcosystemProtection { /// Level of protection/restriction applied
        protection_level: ProtectionLevel,
        /// Optional path to restore full membership
        restoration_path: Option<RestorationStrategy> },
}

/// Trust evolution states representing dynamic relationship development
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrustEvolution { /// Building trust through consistent positive interactions
    Building {
        /// Rate of trust progress (0.0-1.0)
        progress_rate: f64,
        /// Milestones achieved in trust building
        milestone_achievements: Vec<String>,
        /// Indicators showing trust confidence
        confidence_indicators: Vec<TrustIndicator> },
    /// Flourishing in mutual benefit and stable relationship
    Flourishing { /// Metrics showing relationship stability
        stability_metrics: StabilityProfile,
        /// Indicators of mutual benefit
        mutual_benefit_indicators: Vec<BenefitMetric> },
    /// Questioning phase requiring clarification and resolution
    Questioning { /// Types of concerns being questioned
        concern_factors: Vec<ConcernType>,
        /// Areas needing clarification
        clarification_needed: Vec<String>,
        /// Optional path for resolution
        resolution_path: Option<ResolutionStrategy> },
    /// Healing from trust damage with active repair
    Healing { /// Progress in recovery (0.0-1.0)
        recovery_progress: f64,
        /// Milestones for rebuilding trust
        rebuilding_milestones: Vec<Milestone>,
        /// Actions being taken to repair trust
        trust_repair_actions: Vec<RepairAction> },
    /// Evolving into new relationship patterns
    Evolving { /// Direction of evolution
        evolution_direction: EvolutionPath,
        /// Strategies for adaptation
        adaptation_strategies: Vec<AdaptationStrategy> },
}

/// Coordination models for ecosystem governance
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CoordinationModel { /// Distributed decision making with consensus
    Distributed(ConsensusType),
    /// Rotating leadership based on expertise
    Rotational(RotationCriteria),
    /// Collaborative decision making
    Collaborative(DecisionProtocol),
    /// Emergent leadership based on natural selection
    Emergent(EmergenceFactors) }

/// Types of symbiotic relationships in the ecosystem
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SymbiosisType { /// Both parties benefit from the relationship
    Mutualistic,
    /// One benefits while the other is neutral
    Commensal,
    /// One party facilitates the other's success
    Facilitative,
    /// One party provides protection to the other
    Protective,
    /// Healthy competition that drives improvement
    Competitive }

// Supporting types for the ecosystem relationships

/// Learning path for participants growing within the ecosystem
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LearningPath { /// Current stage in the learning journey
    pub current_stage: String,
    /// Next milestones to achieve
    pub next_milestones: Vec<String>,
    /// Percentage completion of the learning path
    pub completion_percentage: f64 }

/// Metrics tracking participant progress and contributions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgressMetrics { /// Skills successfully acquired
    pub skills_acquired: Vec<String>,
    /// Number of contributions made to the ecosystem
    pub contributions_made: u32,
    /// Community feedback score (0.0-1.0)
    pub community_feedback_score: f64 }

/// Scope defining the boundaries of collaboration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollaborationScope { /// Operations that are allowed within this collaboration
    pub allowed_operations: Vec<String>,
    /// Resource usage limits
    pub resource_limits: HashMap<String, u64>,
    /// Boundaries of interaction
    pub interaction_boundaries: Vec<String> }

/// Factors that may cause concern in ecosystem interactions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConcernFactor { /// Security-related incident occurred
    SecurityIncident,
    /// Policy violation detected
    PolicyViolation,
    /// Resource abuse identified
    ResourceAbuse,
    /// Community complaint received
    CommunityComplaint,
    /// Technical misuse detected
    TechnicalMisuse }

/// Levels of monitoring applied to ecosystem participants
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonitoringLevel { /// Light monitoring with basic checks
    Light,
    /// Standard monitoring with regular checks
    Standard,
    /// Enhanced monitoring with frequent checks
    Enhanced,
    /// Intensive monitoring with continuous observation
    Intensive }

/// Levels of protection applied to the ecosystem
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtectionLevel { /// Read-only access granted
    ReadOnlyAccess,
    /// Limited interaction allowed
    LimitedInteraction,
    /// Quarantine mode activated
    QuarantineMode,
    /// Temporary ban implemented
    TemporaryBan }

/// Strategy for restoring full ecosystem membership
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RestorationStrategy { /// Actions required for restoration
    pub required_actions: Vec<String>,
    /// Timeline for restoration in weeks
    pub timeline_weeks: u32,
    /// Criteria for successful restoration
    pub success_criteria: Vec<String> }

/// Indicator of trust level and trend
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrustIndicator { /// Type of trust indicator
    pub indicator_type: String,
    /// Current value of the indicator
    pub value: f64,
    /// Trend direction of the indicator
    pub trend: TrendDirection }

/// Profile showing relationship stability metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StabilityProfile { /// Overall consistency score
    pub consistency_score: f64,
    /// Various reliability metrics
    pub reliability_metrics: HashMap<String, f64>,
    /// Index of predictability
    pub predictability_index: f64 }

/// Metric showing mutual benefit in relationships
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BenefitMetric { /// Name of the benefit metric
    pub metric_name: String,
    /// Value of the benefit
    pub value: f64,
    /// Who benefits from this metric
    pub beneficiary: String }

/// Types of concerns that may arise in questioning phase
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConcernType { /// Performance-related concerns
    Performance,
    /// Security-related concerns
    Security,
    /// Reliability concerns
    Reliability,
    /// Communication issues
    Communication,
    /// Resource usage concerns
    ResourceUsage }

/// Strategy for resolving concerns and issues
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolutionStrategy { /// Approach to resolution
    pub approach: String,
    /// Steps to take for resolution
    pub steps: Vec<String>,
    /// Expected duration for resolution
    pub expected_duration: std::time::Duration }

/// Milestone in trust building or repair process
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Milestone { /// Name of the milestone
    pub name: String,
    /// Description of what the milestone represents
    pub description: String,
    /// Criteria for milestone completion
    pub completion_criteria: Vec<String> }

/// Action taken to repair damaged trust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepairAction { /// Type of repair action
    pub action_type: String,
    /// Description of the action
    pub description: String,
    /// Party responsible for the action
    pub responsible_party: String }

/// Path of evolution for relationships
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvolutionPath { /// Starting state of evolution
    pub from_state: String,
    /// Target state of evolution
    pub to_state: String,
    /// Factors driving the transition
    pub transition_factors: Vec<String> }

/// Strategy for adapting to ecosystem changes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdaptationStrategy { /// Name of the adaptation strategy
    pub strategy_name: String,
    /// Steps for implementation
    pub implementation_steps: Vec<String>,
    /// Metrics for measuring success
    pub success_metrics: Vec<String> }

/// Direction of trend for trust indicators
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrendDirection { /// Trend is improving
    Improving,
    /// Trend is stable
    Stable,
    /// Trend is declining
    Declining }

/// Types of consensus mechanisms
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsensusType { /// Unanimous agreement required
    Unanimous,
    /// Simple majority required
    Majority,
    /// Super majority required
    SuperMajority,
    /// Weighted voting system
    WeightedVoting }

/// Criteria for rotating leadership
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RotationCriteria { /// Duration of each rotation period
    pub rotation_period: std::time::Duration,
    /// Required expertise for leadership
    pub expertise_requirements: Vec<String>,
    /// Method for selecting leaders
    pub selection_method: String }

/// Mapping of expertise domains to experts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpertiseMapping { /// Domain experts by area
    pub domain_experts: HashMap<String, Vec<String>>,
    /// Authority levels by person
    pub authority_levels: HashMap<String, u8> }

/// Protocol for collaborative decision making
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionProtocol { /// Method for making decisions
    pub decision_method: String,
    /// Required participants in decisions
    pub required_participants: Vec<String>,
    /// Method for resolving conflicts
    pub conflict_resolution: String }

/// Factors that enable emergent leadership
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmergenceFactors { /// Criteria for natural selection of leaders
    pub natural_selection_criteria: Vec<String>,
    /// Patterns of leadership emergence
    pub leadership_emergence_patterns: Vec<String> }

// Implementation methods for the main types
impl EcosystemMembership {
    /// Check if this membership allows a specific operation
    pub fn allows_operation(&self, operation: &str) -> bool {
        match self {
            Self::CoreSteward { .. } => true, // Stewards can do everything
            Self::ActiveContributor { contribution_types, .. } => {
                contribution_types.iter().any(|t| t == operation) || operation == "read
            }
            Self::LearningParticipant { .. } => matches!(operation,   read" |"learn "),
            Self::VisitingCollaborator { collaboration_scope, .. } => {
                collaboration_scope.allowed_operations.contains(&operation.to_string())
            }
            Self::CautiousInteraction { .. } => operation ==    read"",
            Self::EcosystemProtection { .. } => false,
        }
    }

    /// Get the trust level for this membership type
    pub fn trust_level(&self) -> f64 {
        match self {
            Self::CoreSteward { trust_level, .. } => *trust_level,
            Self::ActiveContributor { contribution_score, .. } => contribution_score * contribution_score, // Squared for emphasis
            Self::LearningParticipant { progress_metrics, .. } => progress_metrics.community_feedback_score * 0.6,
            Self::VisitingCollaborator { .. } => 0.5, // Neutral trust for visitors
            Self::CautiousInteraction { .. } => 0.2, // Low trust due to concerns
            Self::EcosystemProtection { .. } => 0.0, // No trust during protection
        }
    }
}

impl TrustEvolution {
    /// Calculate the overall trust score for this evolution state
    pub fn trust_score(&self) -> f64 {
        match self {
            Self::Building { progress_rate, .. } => 0.3 + (progress_rate * 0.4),
            Self::Flourishing { stability_metrics, .. } => 0.7 + (stability_metrics.consistency_score * 0.3),
            Self::Questioning { .. } => 0.4, // Moderate trust during questioning
            Self::Healing { recovery_progress, .. } => 0.2 + (recovery_progress * 0.5),
            Self::Evolving { .. } => 0.6, // Good trust during positive evolution
        }
    }
} 