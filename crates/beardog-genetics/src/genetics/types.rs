//! Type definitions and data structures for the genetics engine
//!
//! Contains all structs, enums, and type aliases for genetic operations.

use async_trait::async_trait;
use beardog_errors::BearDogResult;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// Import genetics types from auth module (which were in cross_node_auth)
use beardog_auth::auth::{BearDogGenetics, SpawnPurpose};

/// Configuration for genetics operations
///
/// Controls how the genetic algorithms behave when creating new nodes or evolving
/// existing ones. These parameters balance genetic diversity with security stability.
///
/// # Fields
///
/// - `base_mutation_rate`: Base probability of mutations (0.0-1.0)
/// - `max_genetic_diversity`: Maximum allowed genetic variance from parents (0.0-1.0)  
/// - `min_security_threshold`: Minimum security level required for offspring (0.0-1.0)
/// - `capability_inheritance_weight`: How strongly offspring inherit parent capabilities (0.0-1.0)
/// - `trait_blending_factor`: How much parent traits are blended vs. dominant inheritance (0.0-1.0)
/// - `enable_directed_evolution`: Whether to allow purpose-specific genetic optimization
///
/// # Security Considerations
///
/// Higher mutation rates increase diversity but may reduce security stability.
/// Lower inheritance weights allow more innovation but may lose proven security traits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsConfig {
    /// Base probability of genetic mutations occurring during reproduction
    pub base_mutation_rate: f64,
    /// Maximum genetic diversity allowed to prevent excessive drift from secure baselines
    pub max_genetic_diversity: f64,
    /// Minimum security threshold that all offspring must meet
    pub min_security_threshold: f64,
    /// Weight given to inheriting parent capabilities vs. generating new ones
    pub capability_inheritance_weight: f64,
    /// Factor for blending parent traits vs. dominant inheritance patterns
    pub trait_blending_factor: f64,
    /// Enable directed evolution for specific spawn purposes
    pub enable_directed_evolution: bool,
}

impl Default for GeneticsConfig {
    fn default() -> Self {
        Self {
            base_mutation_rate: 0.05,
            max_genetic_diversity: 0.8,
            min_security_threshold: 0.7,
            capability_inheritance_weight: 0.8,
            trait_blending_factor: 0.6,
            enable_directed_evolution: true,
        }
    }
}

/// Storage trait for node genetics
///
/// Provides persistence and retrieval of genetic information for BearDog nodes.
/// Implementations should ensure data integrity and atomic operations.
#[async_trait]
pub trait GeneticsStore: Send + Sync {
    /// Store genetics for a specific node
    ///
    /// # Arguments
    /// - `node_id`: Unique identifier for the node
    /// - `genetics`: Complete genetic profile to store
    ///
    /// # Security
    /// This operation should be atomic and include integrity verification.
    async fn store_genetics(&self, node_id: &str, genetics: &BearDogGenetics) -> BearDogResult<()>;

    /// Load genetics for a specific node
    ///
    /// # Arguments
    /// - `node_id`: Unique identifier for the node
    ///
    /// # Returns
    /// - `Some(genetics)` if the node exists
    /// - `None` if the node is not found
    async fn load_genetics(&self, node_id: &str) -> BearDogResult<Option<BearDogGenetics>>;

    /// List all genetics in the store
    ///
    /// # Returns
    /// Vector of tuples containing (node_id, genetics) for all stored nodes.
    /// This is primarily used for genetic analysis and lineage tracking.
    async fn list_all_genetics(&self) -> BearDogResult<Vec<(String, BearDogGenetics)>>;

    /// Delete genetics for a specific node
    ///
    /// # Arguments
    /// - `node_id`: Unique identifier for the node to delete
    ///
    /// # Security
    /// This is a destructive operation that should be logged for audit purposes.
    async fn delete_genetics(&self, node_id: &str) -> BearDogResult<()>;
}

/// In-memory implementation of genetics storage
///
/// Provides thread-safe storage for genetics data in memory. This implementation
/// is suitable for development and testing, but persistent storage should be used
/// in production environments.
pub struct InMemoryGeneticsStore {
    /// Thread-safe storage for genetics data indexed by node ID
    genetics: Arc<RwLock<HashMap<String, BearDogGenetics>>>,
}

impl Default for InMemoryGeneticsStore {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryGeneticsStore {
    /// Create a new in-memory genetics store
    pub fn new() -> Self {
        Self {
            genetics: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl GeneticsStore for InMemoryGeneticsStore {
    async fn store_genetics(&self, node_id: &str, genetics: &BearDogGenetics) -> BearDogResult<()> {
        let mut store = self.genetics.write().await;
        store.insert(node_id.to_string(), genetics.clone());
        Ok(())
    }

    async fn load_genetics(&self, node_id: &str) -> BearDogResult<Option<BearDogGenetics>> {
        let store = self.genetics.read().await;
        Ok(store.get(node_id).cloned())
    }

    async fn list_all_genetics(&self) -> BearDogResult<Vec<(String, BearDogGenetics)>> {
        let store = self.genetics.read().await;
        Ok(store
            .iter()
            .map(|(id, genetics)| (id.clone(), genetics.clone()))
            .collect())
    }

    async fn delete_genetics(&self, node_id: &str) -> BearDogResult<()> {
        let mut store = self.genetics.write().await;
        store.remove(node_id);
        Ok(())
    }
}

// Note: TaskType ToString implementation moved to beardog_auth crate to avoid orphan rule violations
// Use Display trait or format! macro instead for string conversion

/// Multi-party workflow types for genetic spawning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BearDogWorkflowType {
    /// Automated consensus between trusted nodes
    AutomatedConsensus {
        /// List of participating node IDs
        participating_nodes: Vec<String>,
        /// Consensus threshold (0.0-1.0) - fraction of nodes that must agree
        consensus_threshold: f64,
        /// Maximum time to wait for consensus
        max_decision_time: Duration,
    },
    /// Human approval required
    HumanApprovalRequired {
        /// Required approver roles (e.g., "security_officer", "compliance_lead")
        approver_roles: Vec<String>,
        /// Minimum number of approvals needed
        min_approvals: u32,
        /// Timeout for human response
        approval_timeout: Duration,
    },
    /// Hybrid approval (automated + human oversight)
    HybridApproval {
        /// Automated checks that must pass first
        automated_checks: Vec<AutomatedCheck>,
        /// Whether human oversight is required after automated checks
        human_oversight: bool,
        /// Conditions that trigger escalation to humans
        escalation_conditions: Vec<EscalationCondition>,
    },
}

/// Automated security checks for spawning workflows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomatedCheck {
    /// Minimum trust score required
    TrustScore {
        /// Minimum trust score that must be met
        min_score: f64,
    },
    /// Resource availability check
    ResourceAvailability {
        /// Minimum resource requirements that must be available
        min_resources: ResourceLimits,
    },
    /// Compliance validation
    ComplianceValidation {
        /// List of compliance standards that must be met
        required_standards: Vec<String>,
    },
    /// Threat assessment
    ThreatAssessment {
        /// Maximum acceptable risk level
        max_risk_level: f64,
    },
    /// Geographic compliance
    GeographicCompliance {
        /// List of jurisdictions where spawning is allowed
        allowed_jurisdictions: Vec<String>,
    },
    /// Temporal window restrictions
    TemporalWindow {
        /// List of allowed hours for spawning (0-23)
        allowed_hours: Vec<u8>,
    },
}

/// Conditions that escalate to human review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EscalationCondition {
    /// High resource usage
    HighResourceUsage {
        /// Resource usage threshold that triggers escalation
        threshold: f64,
    },
    /// Unusual genetic patterns
    UnusualGeneticPattern {
        /// Deviation threshold for genetic patterns
        deviation_threshold: f64,
    },
    /// Multiple failed automated checks
    MultipleFailures {
        /// Maximum number of failures before escalation
        max_failures: u32,
    },
    /// Spawning outside normal hours
    OffHoursSpawn,
    /// Cross-jurisdictional spawning
    CrossBorderSpawn,
}

/// Resource constraints for spawned nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum CPU percentage (0.0-100.0)
    pub max_cpu_percent: f64,
    /// Maximum memory in MB
    pub max_memory_mb: u64,
    /// Maximum storage in GB
    pub max_storage_gb: u64,
    /// Maximum network bandwidth in Mbps
    pub max_network_mbps: u64,
    /// Geographic restrictions
    pub allowed_jurisdictions: Vec<String>,
    /// Temporal spawning windows
    pub temporal_windows: Vec<TimeWindow>,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_cpu_percent: 80.0,
            max_memory_mb: 4096,
            max_storage_gb: 100,
            max_network_mbps: 1000,
            allowed_jurisdictions: vec!["US".to_string(), "EU".to_string()],
            temporal_windows: vec![],
        }
    }
}

/// Time window for allowed spawning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeWindow {
    /// Start hour (0-23)
    pub start_hour: u8,
    /// End hour (0-23)
    pub end_hour: u8,
    /// Days of week (0=Sunday, 6=Saturday)
    pub days_of_week: Vec<u8>,
    /// Timezone (e.g., "UTC", "America/New_York")
    pub timezone: String,
}

/// Spawn request for genetic reproduction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnRequest {
    /// Unique ID for this spawn request
    pub request_id: String,
    /// ID of the requesting parent node
    pub requesting_parent: String,
    /// IDs of additional co-parent nodes
    pub co_parents: Vec<String>,
    /// Purpose of the spawn (affects genetic optimization)
    pub purpose: SpawnPurpose,
    /// Resource requirements for the child
    pub resource_requirements: ResourceLimits,
    /// Workflow type for approval
    pub workflow_type: BearDogWorkflowType,
    /// When the request was created
    pub created_at: DateTime<Utc>,
    /// Expiration time for the request
    pub expires_at: DateTime<Utc>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Result of spawn request processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnResult {
    /// Request ID this result corresponds to
    pub request_id: String,
    /// Whether the spawn was approved
    pub approved: bool,
    /// Generated genetics for the child (if approved)
    pub child_genetics: Option<BearDogGenetics>,
    /// ID assigned to the child node (if approved)
    pub child_node_id: Option<String>,
    /// Approval/rejection reason
    pub decision_reason: String,
    /// Nodes that participated in the decision
    pub decision_participants: Vec<String>,
    /// When the decision was made
    pub decided_at: DateTime<Utc>,
    /// Audit trail of the decision process
    pub decision_audit_trail: Vec<DecisionAuditEntry>,
}

/// Audit entry for spawn decision process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionAuditEntry {
    /// Timestamp of this audit event
    pub timestamp: DateTime<Utc>,
    /// Node or user that performed the action
    pub actor: String,
    /// Type of action performed
    pub action: String,
    /// Result or outcome of the action
    pub result: String,
    /// Additional context or metadata
    pub context: HashMap<String, String>,
}

/// Genetic recombination parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecombinationParams {
    /// How to combine crypto chromosomes
    pub chromosome_strategy: ChromosomeRecombinationStrategy,
    /// How to blend security traits
    pub trait_blending: TraitBlendingStrategy,
    /// How to merge capabilities
    pub capability_merging: CapabilityMergingStrategy,
    /// Mutation rate for the recombination
    pub mutation_rate: f64,
    /// Whether to apply directed evolution
    pub directed_evolution: bool,
}

/// Strategy for combining crypto chromosomes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChromosomeRecombinationStrategy {
    /// Take the dominant (highest strength) chromosome from parents
    DominantSelection,
    /// Genetic crossover between parents
    Crossover,
    /// Average chromosome properties
    Averaging,
    /// Weighted average of parent chromosomes
    WeightedAverage {
        /// Weights for each parent chromosome
        weights: Vec<f64>,
    },
    /// Combine flags using bitwise OR
    BitwiseUnion,
    /// Custom blending with specified parameters
    CustomBlend {
        /// Factor controlling dominance in blending
        dominance_factor: f64,
    },
}

/// Strategy for blending security traits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TraitBlendingStrategy {
    /// Simple average of parent traits
    Average,
    /// Weighted average with specified weights
    WeightedAverage {
        /// Weights for each parent trait
        weights: Vec<f64>,
    },
    /// Select dominant traits based on random selection
    Dominant,
    /// Select best traits from each parent
    Selective,
    /// Take traits from the most paranoid parent
    MostParanoid,
    /// Take traits from the most cooperative parent  
    MostCooperative,
    /// Custom blending with specified factor
    CustomBlend {
        /// Factor controlling trait blending
        blending_factor: f64,
    },
}

/// Strategy for merging capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CapabilityMergingStrategy {
    /// Union of all parent capabilities
    Union,
    /// Intersection of parent capabilities (only common ones)
    Intersection,
    /// Select capabilities based on random selection and fitness
    Selective,
    /// Weighted combination of capabilities
    WeightedCombination {
        /// Weights for each capability
        weights: Vec<f64>,
    },
    /// Best capabilities from each parent
    BestOfBreed,
}

/// Cryptographic lineage verification data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticLineage {
    /// Child node ID
    pub child_node_id: String,
    /// Parent node IDs
    pub parent_node_ids: Vec<String>,
    /// Generation number (0 for genesis nodes)
    pub generation: u32,
    /// Cryptographic proof of lineage
    pub lineage_proof: LineageProof,
    /// When the child was spawned
    pub spawn_timestamp: DateTime<Utc>,
    /// Genetic diversity score
    pub diversity_score: f64,
}

/// Cryptographic proof of genetic lineage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageProof {
    /// Ed25519 signatures from each parent
    pub parent_signatures: Vec<ParentSignature>,
    /// Hash of the child genetics
    pub child_genetics_hash: Vec<u8>,
    /// Hash chain linking to parent genetics
    pub parent_genetics_hashes: Vec<Vec<u8>>,
    /// Witness signatures (if required by workflow)
    pub witness_signatures: Vec<WitnessSignature>,
}

/// Parent signature in lineage proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParentSignature {
    /// Parent node ID
    pub parent_node_id: String,
    /// Ed25519 signature over spawn data
    pub signature: Vec<u8>,
    /// Public key used for verification
    pub public_key: Vec<u8>,
}

/// Witness signature for spawn verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessSignature {
    /// Witness node ID or human approver ID
    pub witness_id: String,
    /// Type of witness (node, human, etc.)
    pub witness_type: WitnessType,
    /// Signature over the spawn decision
    pub signature: Vec<u8>,
    /// Public key for verification
    pub public_key: Vec<u8>,
}

/// Type of witness for spawn verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WitnessType {
    /// Another BearDog node
    Node,
    /// Human approver
    Human,
    /// External system
    External {
        /// Type of external system providing witness
        system_type: String,
    },
}
