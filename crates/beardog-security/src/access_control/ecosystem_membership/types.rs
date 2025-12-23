// Types for ecosystem membership access control
//
// This module contains all the data structures and enums used by the
// ecosystem membership access control system.

use beardog_genetics::ecosystem_evolution::{EcosystemMembership, TrustEvolution, EcosystemContext};
use beardog_types::canonical::HealthStatus;
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipConfig {
    /// The trust thresholds value
    pub trust_thresholds: TrustThresholds,
    
    /// Automatic evolution configuration
    /// The auto evolution value
    pub auto_evolution: AutoEvolutionConfig,
    
    /// Legacy system integration
    /// The legacy integration value
    pub legacy_integration: LegacyIntegrationConfig,
    
    /// Health monitoring configuration
    /// The health monitoring value
    pub health_monitoring: HealthMonitoringConfig,
    
    /// Maximum number of memberships to track
    /// Number of max_memberships
    pub max_memberships: usize,
    
    /// How often to clean up expired memberships (in hours)
    /// Number of cleanup_interval_hours
    pub cleanup_interval_hours: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustThresholds {
    /// The core steward value
    pub core_steward: f64,
    
    /// The active contributor value
    pub active_contributor: f64,
    
    /// The learning participant value
    pub learning_participant: f64,
    
    /// The visiting collaborator value
    pub visiting_collaborator: f64,
    
    /// The cautious interaction value
    pub cautious_interaction: f64,
    
    /// The ecosystem protection value
    pub ecosystem_protection: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoEvolutionConfig {
    /// Enable automatic trust evolution
    /// Whether feature is enabled
    pub enabled: bool,
    
    /// Factors that influence evolution
    /// Collection of evolution factors
    pub evolution_factors: Vec<EvolutionFactor>,
    
    /// Number of min_observation_hours
    pub min_observation_hours: u64,
    
    /// Maximum trust change per evolution cycle
    /// The max trust delta value
    pub max_trust_delta: f64,
    
    /// How often to run evolution cycles (hours)
    /// Number of evolution_cycle_hours
    pub evolution_cycle_hours: u64,
}

/// Factor that influences membership evolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionFactor {
    /// Name of the factor
    /// Name of the item
    pub name: String,
    
    /// Weight of this factor in evolution decisions
    /// The weight value
    pub weight: f64,
    
    /// Configuration specific to this factor
    pub config: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyIntegrationConfig {
    /// Enable legacy allowlist integration
    /// Whether allowlist is enabled
    pub allowlist_enabled: bool,
    
    /// Enable legacy blocklist integration
    /// Whether blocklist is enabled
    pub blocklist_enabled: bool,
    
    /// The allowlist trust boost value
    pub allowlist_trust_boost: f64,
    
    /// The blocklist trust penalty value
    pub blocklist_trust_penalty: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMonitoringConfig {
    /// Enable health monitoring
    /// Whether feature is enabled
    pub enabled: bool,
    
    /// Number of check_interval_seconds
    pub check_interval_seconds: u64,
    
    /// The unhealthy threshold value
    pub unhealthy_threshold: f64,
    
    /// Actions to take when unhealthy
    /// Collection of unhealthy actions
    pub unhealthy_actions: Vec<String>,
}

/// Database of ecosystem memberships
#[derive(Debug, Clone)]
pub struct MembershipDatabase {
    /// Map of entity ID to membership entry
    /// Mapping of entries
    pub entries: HashMap<String, MembershipEntry>,
    
    /// Mapping of type index
    pub type_index: HashMap<EcosystemMembership, Vec<String>>,
}

/// Entry in membership database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipEntry {
    pub entity_id: String,
    
    /// Current membership level
    /// The membership value
    pub membership: EcosystemMembership,
    
    /// Current trust level (0.0-1.0)
    /// The trust level value
    pub trust_level: f64,
    
    /// When this membership was established
    /// The established at value
    pub established_at: DateTime<Utc>,
    
    /// When this membership was last updated
    /// The last updated value
    pub last_updated: DateTime<Utc>,
    
    /// Metadata about this membership
    /// The metadata value
    pub metadata: MembershipMetadata,
}

/// Metadata associated with membership
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipMetadata {
    /// Source of this membership
    /// The source value
    pub source: MembershipSource,
    
    /// Evidence supporting this membership level
    pub evidence: Vec<EvidenceItem>,
    
    /// Statistics about interactions
    /// The interaction stats value
    pub interaction_stats: InteractionStatistics,
    
    /// History of membership changes
    /// Collection of history
    pub history: Vec<MembershipHistoryEntry>,
    
    /// Additional custom metadata
    /// Mapping of custom
    pub custom: HashMap<String, String>,
}

/// Source of membership establishment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MembershipSource {
    /// Manually assigned by administrator
    Manual { admin_id: String },
    Manual { admin_id: String },
    Manual { admin_id: String },
    
    /// Automatically determined by system
    Automatic { algorithm: String },
    
    /// Inherited from genetic relationships
    Genetic { parent_entity: String },
    
    /// Migrated from legacy allowlist/blocklist
    Legacy { original_list: String },
    
    /// Established through community consensus
    Community { consensus_score: f64 },
}

/// Evidence supporting membership level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceItem {
    /// Type of evidence
    pub evidence_type: EvidenceType,
    
    /// Strength of this evidence (0.0-1.0)
    /// The strength value
    pub strength: f64,
    
    /// When this evidence was collected
    /// The collected at value
    pub collected_at: DateTime<Utc>,
    
    /// Description of the evidence
    /// The description value
    pub description: String,
    
    /// Source of the evidence
    /// The source value
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of evidence
pub enum EvidenceType {
    /// Positive interactions with the ecosystem
    PositiveInteraction,
    
    /// Contributions to the ecosystem
    Contribution,
    
    /// Endorsements from other members
    Endorsement,
    
    /// Historical behavior patterns
    BehaviorPattern,
    
    /// Technical capabilities demonstrated
    TechnicalCapability,
    
    /// Community standing
    CommunityStanding,
    
    /// Security incident involvement
    SecurityIncident,
    
    /// Policy violations
    PolicyViolation,
    
    /// Genetic relationship evidence
    GeneticRelationship,
}

/// Statistics about entity interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionStatistics {
    /// Total number of interactions
    /// Number of total_interactions
    pub total_interactions: u64,
    
    /// Number of successful interactions
    /// Number of successful_interactions
    pub successful_interactions: u64,
    
    /// Number of failed interactions
    /// Number of failed_interactions
    pub failed_interactions: u64,
    
    /// Average interaction quality score
    /// The average quality value
    pub average_quality: f64,
    
    /// Recent interaction trend
    /// The recent trend value
    pub recent_trend: InteractionTrend,
    
    /// Time of last interaction
    /// Optional last interaction
    pub last_interaction: Option<DateTime<Utc>>,
}

/// Trend in recent interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionTrend {
    /// Currently improving
    Improving,
    /// Represents stable variant
    Stable,
    /// Currently declining
    Declining,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipHistoryEntry {
    /// When this change occurred
    pub timestamp: DateTime<Utc>,
    
    /// Previous membership level
    /// The previous membership value
    pub previous_membership: EcosystemMembership,
    
    /// New membership level
    /// The new membership value
    pub new_membership: EcosystemMembership,
    
    /// Previous trust level
    /// The previous trust value
    pub previous_trust: f64,
    
    /// New trust level
    /// The new trust value
    pub new_trust: f64,
    
    /// The reason value
    pub reason: String,
    
    /// Who or what initiated the change
    /// The initiated by value
    pub initiated_by: String,
}

/// Trust evolution tracker
#[derive(Debug, Clone)]
pub struct TrustEvolutionTracker {
    pub config: TrustComputationConfig,
    
    /// Recent trust change events
    /// Collection of recent events
    pub recent_events: Vec<TrustChangeEvent>,
    
    /// Trust computation engine
    /// The computation engine value
    pub computation_engine: TrustComputationEngine,
}

/// Event representing a change in trust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustChangeEvent {
    /// Entity ID affected
    pub entity_id: String,
    
    /// Previous trust level
    /// The previous trust value
    pub previous_trust: f64,
    
    /// New trust level
    /// The new trust value
    pub new_trust: f64,
    
    /// Factors that influenced this change
    /// Collection of influencing factors
    pub influencing_factors: Vec<String>,
    
    /// When this change occurred
    pub timestamp: DateTime<Utc>,
    
    /// Additional context
    /// Mapping of context
    pub context: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct TrustComputationEngine {
    config: TrustComputationConfig,
    
    /// Active trust algorithms
    algorithms: Vec<TrustAlgorithm>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustComputationConfig {
    /// The base trust value
    pub base_trust: f64,
    
    /// Factors that influence trust computation
    /// Collection of trust factors
    pub trust_factors: Vec<TrustFactor>,
    
    /// The decay factor value
    pub decay_factor: f64,
    
    pub time_window_hours: u64,
}

/// Factor that influences trust computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustFactor {
    /// Name of the factor
    /// Name of the item
    pub name: String,
    
    /// Weight of this factor
    /// The weight value
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustAlgorithm {
    /// Name of the algorithm
    /// Name of the item
    pub name: String,
    
    /// Type of algorithm
    /// The algorithm type value
    pub algorithm_type: TrustAlgorithmType,
    
    pub config: HashMap<String, f64>,
    
    /// Weight of this algorithm in final trust score
    /// The weight value
    pub weight: f64,
}

/// Type of trust algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of trust algorithm
pub enum TrustAlgorithmType {
    /// Simple weighted average
    WeightedAverage,
    
    /// Exponential decay model
    ExponentialDecay,
    
    /// Machine learning based
    MachineLearning { model_type: String },
    MachineLearning { model_type: String },
    MachineLearning { model_type: String },
    
    /// Genetic algorithm based
    Genetic,
    
    /// Bayesian inference
    Bayesian,
    
    /// Custom algorithm
    Custom { implementation: String },
}

/// Access policy engine
#[derive(Debug, Clone)]
pub struct AccessPolicyEngine {
    /// Policies by resource type
    policies: HashMap<String, AccessPolicy>,
    
    /// Policy evaluation engine
    evaluation_engine: PolicyEvaluationEngine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPolicy {
    /// Resource type this policy applies to
    /// The resource type value
    pub resource_type: String,
    
    /// Access rules by membership type
    /// Mapping of rules
    pub rules: HashMap<EcosystemMembership, AccessRule>,
    
    /// The default rule value
    pub default_rule: AccessRule,
    
    /// Metadata about this policy
    /// The metadata value
    pub metadata: PolicyMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessRule {
    /// Whether access is allowed
    /// Whether allowed is enabled
    pub allowed: bool,
    
    /// Conditions that must be met
    /// Collection of conditions
    pub conditions: Vec<AccessCondition>,
    
    /// Optional rate limits
    pub rate_limits: Option<RateLimits>,
    
    /// Time-based restrictions
    pub time_restrictions: Option<TimeRestrictions>,
    
    /// Additional metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessCondition {
    /// Minimum trust level required
    MinTrustLevel(f64),
    
    /// Maximum trust level allowed
    MaxTrustLevel(f64),
    
    /// Specific trust level range
    TrustRange { min: f64, max: f64 },
    TrustRange { min: f64, max: f64 },
    TrustRange { min: f64, max: f64 },
    
    /// Time-based condition
    TimeWindow(TimeWindow),
    
    /// Custom condition
    Custom { condition: String, parameters: HashMap<String, String> },
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimits {
    /// Maximum requests per minute
    /// Number of requests_per_minute
    pub requests_per_minute: u32,
    
    /// Maximum requests per hour
    /// Number of requests_per_hour
    pub requests_per_hour: u32,
    
    /// Maximum requests per day
    /// Number of requests_per_day
    pub requests_per_day: u32,
    
    /// Burst allowance
    /// Number of burst_allowance
    pub burst_allowance: u32,
}

/// Time-based access restrictions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRestrictions {
    /// Allowed time windows
    /// Collection of allowed windows
    pub allowed_windows: Vec<TimeWindow>,
    
    /// Holiday restrictions
    pub holiday_restrictions: Option<HolidayRestrictions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeWindow {
    pub start_time: String,
    
    pub end_time: String,
    
    /// Days of week (0=Sunday, 1=Monday, etc.)
    /// Collection of days of week
    pub days_of_week: Vec<u8>,
}

/// Holiday-based restrictions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolidayRestrictions {
    /// Whether access is restricted on holidays
    pub restrict_on_holidays: bool,
    
    /// List of holiday names to restrict
    pub restricted_holidays: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyMetadata {
    /// When this policy was created
    /// The created at value
    pub created_at: DateTime<Utc>,
    
    /// Who created this policy
    /// The created by value
    pub created_by: String,
    
    /// When this policy was last updated
    /// The last updated value
    pub last_updated: DateTime<Utc>,
    
    /// Who last updated this policy
    /// The updated by value
    pub updated_by: String,
    
    /// Version of this policy
    /// The version value
    pub version: String,
    
    /// Description of this policy
    /// The description value
    pub description: String,
}

/// Policy evaluation engine
#[derive(Debug, Clone)]
pub struct PolicyEvaluationEngine {
    config: EvaluationConfig,
    
    cache: PolicyCache,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationConfig {
    /// Enable caching of evaluation results
    /// Whether enable_caching is enabled
    pub enable_caching: bool,
    
    /// Cache TTL in seconds
    /// Number of cache_ttl_seconds
    pub cache_ttl_seconds: u64,
    
    /// Maximum cache size
    /// Number of max_cache_size
    pub max_cache_size: usize,
    
    /// Enable detailed logging
    /// Whether enable_detailed_logging is enabled
    pub enable_detailed_logging: bool,
}

#[derive(Debug, Clone)]
pub struct PolicyCache {
    /// Cache entries
    entries: HashMap<String, CacheEntry>,
    
    /// Cache statistics
    stats: CacheStats,
}

/// Entry in policy cache
#[derive(Debug, Clone)]
pub struct CacheEntry {
    /// Cached result
    /// Whether result is enabled
    pub result: bool,
    
    /// When this entry expires
    /// The expires at value
    pub expires_at: DateTime<Utc>,
    
    /// Cache key
    /// The key value
    pub key: String,
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Number of cache hits
    /// Number of hits
    pub hits: u64,
    
    /// Number of cache misses
    /// Number of misses
    pub misses: u64,
    
    /// Number of cache evictions
    /// Number of evictions
    pub evictions: u64,
}

/// Integration with genetics system
#[derive(Debug, Clone)]
pub struct GeneticsIntegration {
    config: GeneticsIntegrationConfig,
    
    /// Genetic factors cache
    genetic_factors: HashMap<String, Vec<GeneticFactor>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsIntegrationConfig {
    /// Enable genetics integration
    /// Whether feature is enabled
    pub enabled: bool,
    
    /// Weight of genetic factors in membership decisions
    /// The genetic weight value
    pub genetic_weight: f64,
    
    /// Minimum genetic relationship strength to consider
    /// The min relationship strength value
    pub min_relationship_strength: f64,
    
    /// Maximum genetic influence on trust
    /// The max genetic influence value
    pub max_genetic_influence: f64,
    
    /// Types of genetic relationships to consider
    /// Collection of relationship types
    pub relationship_types: Vec<String>,
}

/// Genetic factor in membership decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticFactor {
    /// Type of genetic relationship
    /// The relationship type value
    pub relationship_type: String,
    
    /// Strength of the relationship
    /// The strength value
    pub strength: f64,
    
    /// Entity this relationship is with
    /// The related entity value
    pub related_entity: String,
    
    /// Trust influence of this relationship
    /// The trust influence value
    pub trust_influence: f64,
}

/// Access decision result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessDecision {
    /// Whether access is granted
    /// Whether granted is enabled
    pub granted: bool,
    
    /// Entity requesting access
    pub entity_id: String,
    
    /// Resource being accessed
    /// The resource value
    pub resource: String,
    
    /// Membership level used in decision
    /// The membership value
    pub membership: EcosystemMembership,
    
    /// Trust level used in decision
    /// The trust level value
    pub trust_level: f64,
    
    /// Collection of reasons
    pub reasons: Vec<String>,
    
    /// Additional context
    /// Mapping of context
    pub context: HashMap<String, String>,
    
    /// When this decision was made
    pub timestamp: DateTime<Utc>,
} 
