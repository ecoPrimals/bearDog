//! Type definitions for cross-node authorization
//!
//! Contains all structs, enums, and type aliases for the authorization module.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::BearDogResult;

/// Configuration for cross-node authorization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeAuthConfig {
    /// Enable proof verification
    pub proof_verification_enabled: bool,
    /// Maximum proof validity duration in minutes
    pub max_proof_validity_minutes: u32,
    /// Enable genetic spawning authorization
    pub genetic_spawning_enabled: bool,
    /// Require multi-party consensus for sensitive operations
    pub require_consensus: bool,
    /// Trust threshold for consensus (0.0 to 1.0)
    pub consensus_threshold: f64,
    /// Maximum number of spawned BearDogs per node
    pub max_spawns_per_node: u32,
    /// Enable automated workflow approval
    pub automated_approval_enabled: bool,
}

/// Cross-node authorization structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeAuthorization {
    /// Unique authorization ID
    pub id: String,
    /// Requesting node ID
    pub requester_node_id: String,
    /// Resource owner node ID
    pub resource_owner_node_id: String,
    /// Resource identifier
    pub resource_id: String,
    /// Granted permissions
    pub permissions: Vec<ResourcePermission>,
    /// Access conditions
    pub conditions: Vec<AccessCondition>,
    /// Authorization creation time
    pub created_at: DateTime<Utc>,
    /// Authorization expiration time
    pub expires_at: DateTime<Utc>,
    /// Cryptographic signature
    pub signature: String,
    /// Is the authorization active
    pub is_active: bool,
}

/// Resource permission types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourcePermission {
    /// Read access to the resource
    Read,
    /// Write access to modify the resource
    Write,
    /// Delete permission to remove the resource
    Delete,
    /// Administrative access for full control
    Admin,
    /// Execute permission for running operations
    Execute,
    /// Permission to create new resources
    Create,
    /// Permission to share resources with other nodes
    Share,
    /// Permission to backup the resource
    Backup,
    /// Permission to restore from backup
    Restore,
    /// Permission to audit resource access
    Audit,
    /// Permission to replicate data across nodes
    Replicate,
    /// Permission to spawn new BearDog instances
    Spawn,
    /// Permission to modify genetic algorithms
    GeneticModify,
    /// Permission to participate in consensus
    Consensus,
    /// Permission to perform compliance operations
    Compliance,
}

/// Access conditions for authorization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessCondition {
    /// Time-based access window
    TimeWindow {
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    },
    /// IP address restriction
    IpAddress(String),
    /// Require multi-factor authentication
    RequireMfa,
    /// Maximum usage count
    MaxUsage(u32),
    /// Rate limiting
    RateLimit {
        max_requests: u32,
        window_seconds: u32,
    },
    /// Require consensus from other nodes
    RequireConsensus {
        threshold: f64,
        nodes: Vec<String>,
    },
}

/// Authentication method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    Signature,
    Certificate,
    BiometricHash,
    MutualTls,
}

/// Cross-node operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeOperation {
    pub operation_type: OperationType,
    pub target_resource: String,
    pub parameters: HashMap<String, String>,
    pub requester_signature: String,
}

/// Operation types for cross-node requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    Read,
    Write,
    Delete,
    Execute,
    Backup,
    Restore,
    Spawn,
    Consensus,
}

/// Authorization proof structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationProof {
    pub authorization_id: String,
    pub operation: CrossNodeOperation,
    pub timestamp: DateTime<Utc>,
    pub proof_signature: String,
}

/// Cross-node workflow request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeWorkflowRequest {
    pub id: String,
    pub workflow_type: BearDogWorkflowType,
    pub requester_node_id: String,
    pub target_nodes: Vec<String>,
    pub required_permissions: Vec<ResourcePermission>,
    pub automated_checks: Vec<AutomatedCheck>,
    pub escalation_conditions: Vec<EscalationCondition>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

/// Genetics system for BearDog spawning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearDogGenetics {
    /// Unique genetic ID
    pub id: String,
    /// Cryptographic chromosomes defining capabilities
    pub crypto_chromosomes: Vec<CryptoChromosome>,
    /// Security traits and behaviors
    pub security_traits: SecurityTraits,
    /// Node capabilities enabled by genetics
    pub capabilities: Vec<NodeCapability>,
    /// Spawn restrictions based on genetics
    pub spawn_restrictions: Vec<SpawnRestriction>,
    /// Generation number (0 = original)
    pub generation: u32,
    /// Parent genetics (for spawned instances)
    pub parent_genetics: Option<Vec<String>>,
    /// Mutation history
    pub mutations: Vec<CapabilityMutation>,
    /// Fitness score for genetic selection
    pub fitness_score: f64,
}

/// Cryptographic chromosome defining algorithm families
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoChromosome {
    pub algorithm_family: AlgorithmFamily,
    pub strength_bits: u32,
    pub compatibility_score: f64,
    pub performance_factor: f64,
    pub security_level: u8,
}

/// Algorithm family types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlgorithmFamily {
    Encryption(EncryptionFamily),
    Signing(SigningFamily),
    Hashing(HashingFamily),
    KeyDerivation(KdfFamily),
    ZeroKnowledge(ZkFamily),
}

/// Encryption algorithm families
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionFamily {
    Aes, ChaCha, Blowfish, Rsa, Ecc,
}

/// Digital signature algorithm families
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SigningFamily {
    Ed25519, Ecdsa, Rsa, Dilithium,
}

/// Hashing algorithm families
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HashingFamily {
    Sha2, Sha3, Blake, Argon2, Scrypt,
}

/// Key derivation function families
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KdfFamily {
    Pbkdf2, Scrypt, Argon2, Hkdf,
}

/// Zero-knowledge proof families
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZkFamily {
    Bulletproofs, Zksnarks, Zkstarks, Plonk,
}

/// Capability gene defining specific node abilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityGene {
    pub capability: NodeCapability,
    pub expression_level: f64, // 0.0 to 1.0
    pub dominant: bool,
    pub mutable: bool,
    pub inheritance_weight: f64,
}

/// Node capabilities that can be genetically encoded
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub enum NodeCapability {
    // Core capabilities
    StorageProvider,
    ComputeProvider,
    NetworkRelay,
    SecurityAnalysis,
    EncryptionStrength(u32),
    
    // Advanced capabilities
    QuantumResistant,
    MultiPartyComputation,
    HomomorphicEncryption,
    ZeroKnowledgeProofs,
    BlockchainIntegration,
    
    // Specialized capabilities
    BiometricProcessing,
    AiModelTraining,
    DistributedConsensus,
    CryptographicAuditing,
    ThreatDetection,
    
    // Ecosystem integration
    ToadStoolCompute,
    SongBirdDiscovery,
    NestGateStorage,
    SquirrelPlugins,
    
    // Performance characteristics
    HighThroughput,
    LowLatency,
    EnergyEfficient,
    FaultTolerant,
    SelfHealing,
    
    // Security specializations
    PenetrationTesting,
    VulnerabilityScanning,
    IncidentResponse,
    ForensicAnalysis,
    ComplianceAuditing,
}

/// Security traits and behavioral patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityTraits {
    pub trust_threshold: f64,
    pub paranoia_level: u8, // 1-10 scale
    pub consensus_requirement: bool,
    pub isolation_preference: f64,
    pub audit_frequency: u32,
}

/// Spawn restrictions based on genetic makeup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpawnRestriction {
    MaxConcurrentSpawns(u32),
    RequiredCapabilities(Vec<NodeCapability>),
    ForbiddenCapabilities(Vec<NodeCapability>),
    MinimumTrustLevel(f64),
    GeographicRestriction(String),
    ResourceLimits(ResourceLimits),
}

/// Task types for spawned BearDogs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    DataStorage,
    Storage,
    ComputeTask,
    Compute,
    SecurityAnalysis,
    Security,
    NetworkRelay,
    Network,
    ComplianceCheck,
    ThreatHunting,
    BackupOperation,
    DisasterRecovery,
}

/// Capability mutations during spawning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityMutation {
    pub trigger: MutationTrigger,
    pub mutation_type: String,
    pub affected_capabilities: Vec<NodeCapability>,
    pub fitness_impact: f64,
}

/// Triggers for genetic mutations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MutationTrigger {
    EnvironmentalStress,
    SecurityThreat,
    PerformanceOptimization,
    EcosystemIntegration,
    UserRequirement,
}

/// Spawned BearDog instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnedBearDog {
    pub id: String,
    pub parent_id: String,
    pub genetics: BearDogGenetics,
    pub spawn_purpose: SpawnPurpose,
    pub task_assignment: Vec<TaskType>,
    pub resource_limits: ResourceLimits,
    pub spawn_time: DateTime<Utc>,
    pub expected_lifetime: Option<DateTime<Utc>>,
    pub current_status: SpawnStatus,
    pub performance_metrics: HashMap<String, f64>,
    pub trust_relationships: HashMap<String, f64>,
    pub consensus_participation: bool,
    pub ecosystem_connections: Vec<String>,
}

/// Purpose for spawning new BearDog instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpawnPurpose {
    LoadBalancing,
    SpecializedTask(TaskType),
    EcosystemIntegration(String),
    SecurityResponse,
    EmergencyResponse,
    DisasterRecovery,
    ComplianceRequirement,
    UserRequest,
    GeneticExperiment,
    NetworkExpansion,
    PerformanceOptimization,
}

/// Resource limits for spawned instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_memory_mb: u64,
    pub max_cpu_percent: u8,
    pub max_disk_mb: u64,
    pub max_network_mbps: u32,
    pub max_concurrent_connections: u32,
}

/// Status of spawned BearDog instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpawnStatus {
    Initializing,
    Active,
    Paused,
    Terminated,
    Failed(String),
    Upgrading,
    Hibernating,
}

/// Workflow types for cross-node operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BearDogWorkflowType {
    DataBackup {
        source_node: String,
        backup_nodes: Vec<String>,
        encryption_required: bool,
    },
    ComplianceAudit {
        audit_scope: Vec<String>,
        standards: Vec<String>,
        automated_remediation: bool,
    },
    SecurityIncidentResponse {
        threat_level: u8,
        affected_resources: Vec<String>,
        response_team: Vec<String>,
    },
    GeneticSpawning {
        parent_genetics: Vec<String>,
        spawn_purpose: SpawnPurpose,
        target_capabilities: Vec<NodeCapability>,
    },
}

/// Automated checks for workflow approval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomatedCheck {
    ResourceAvailability,
    SecurityClearance,
    ComplianceValidation,
    TrustVerification,
    CapabilityMatch,
}

/// Conditions that trigger escalation to human approval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EscalationCondition {
    HighRiskOperation,
    ComplianceViolation,
    UnknownNode,
    ResourceExhaustion,
    SecurityThreat,
}

/// Spawn request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnRequest {
    pub parent_genetics: Vec<BearDogGenetics>,
    pub spawn_purpose: SpawnPurpose,
    pub required_capabilities: Vec<NodeCapability>,
    pub resource_limits: ResourceLimits,
    pub target_environment: String,
}

/// Consensus result for multi-party decisions
#[derive(Debug, Clone)]
pub struct ConsensusResult {
    pub approved: bool,
    pub votes: HashMap<String, bool>,
    pub consensus_threshold: f64,
    pub final_score: f64,
    pub participating_nodes: Vec<String>,
}

// Default implementations

impl Default for CrossNodeAuthConfig {
    fn default() -> Self {
        Self {
            proof_verification_enabled: true,
            max_proof_validity_minutes: 60,
            genetic_spawning_enabled: true,
            require_consensus: false,
            consensus_threshold: 0.67,
            max_spawns_per_node: 10,
            automated_approval_enabled: true,
        }
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_mb: 1024,
            max_cpu_percent: 50,
            max_disk_mb: 5120,
            max_network_mbps: 100,
            max_concurrent_connections: 1000,
        }
    }
}

impl Default for SecurityTraits {
    fn default() -> Self {
        Self {
            trust_threshold: 0.5,
            paranoia_level: 5,
            consensus_requirement: false,
            isolation_preference: 0.3,
            audit_frequency: 24, // hours
        }
    }
}

// Utility implementations

impl ResourcePermission {
    /// Check if permission implies another permission
    pub fn implies(&self, other: &Self) -> bool {
        match (self, other) {
            (ResourcePermission::Admin, _) => true,
            (ResourcePermission::Write, ResourcePermission::Read) => true,
            (ResourcePermission::Delete, ResourcePermission::Write) => true,
            (ResourcePermission::Spawn, ResourcePermission::Create) => true,
            (a, b) => a == b,
        }
    }

    /// Get the security level of the permission (higher = more sensitive)
    pub fn security_level(&self) -> u8 {
        match self {
            ResourcePermission::Read => 1,
            ResourcePermission::Audit => 2,
            ResourcePermission::Backup => 3,
            ResourcePermission::Execute => 4,
            ResourcePermission::Create => 5,
            ResourcePermission::Write => 6,
            ResourcePermission::Share => 7,
            ResourcePermission::Replicate => 8,
            ResourcePermission::Restore => 9,
            ResourcePermission::Consensus => 10,
            ResourcePermission::Compliance => 11,
            ResourcePermission::GeneticModify => 12,
            ResourcePermission::Spawn => 13,
            ResourcePermission::Delete => 14,
            ResourcePermission::Admin => 15,
        }
    }
}

impl CrossNodeAuthorization {
    /// Check if authorization is currently valid
    pub fn is_valid(&self) -> bool {
        self.is_active && Utc::now() < self.expires_at
    }

    /// Check if authorization grants specific permission
    pub fn has_permission(&self, permission: &ResourcePermission) -> bool {
        self.permissions.iter().any(|p| p.implies(permission))
    }
}

/// Main cross-node authorization engine
pub struct CrossNodeAuthEngine {
    pub config: CrossNodeAuthConfig,
    pub active_authorizations: HashMap<String, CrossNodeAuthorization>,
    pub spawned_beardogs: HashMap<String, SpawnedBearDog>,
    pub genetics_registry: HashMap<String, BearDogGenetics>,
    pub node_registry: Box<dyn NodeRegistry + Send + Sync>,
    pub proof_verifier: Box<dyn ProofVerifier + Send + Sync>,
    pub workflow_engine: Option<Box<dyn WorkflowEngine + Send + Sync>>,
}

/// Trait for node registry operations
pub trait NodeRegistry: Send + Sync {
    fn get_node_info(&self, node_id: &str) -> BearDogResult<NodeInfo>;
    fn register_node(&mut self, node_info: NodeInfo) -> BearDogResult<()>;
    fn get_trust_level(&self, node_id: &str) -> BearDogResult<f64>;
    fn update_trust_level(&mut self, node_id: &str, trust_level: f64) -> BearDogResult<()>;
}

/// Trait for proof verification
pub trait ProofVerifier: Send + Sync {
    fn verify_authorization_proof(&self, proof: &AuthorizationProof) -> BearDogResult<bool>;
    fn generate_proof(&self, authorization: &CrossNodeAuthorization, operation: &CrossNodeOperation) -> BearDogResult<AuthorizationProof>;
}

/// Trait for workflow engine integration
pub trait WorkflowEngine: Send + Sync {
    fn submit_workflow(&mut self, request: CrossNodeWorkflowRequest) -> BearDogResult<String>;
    fn get_workflow_status(&self, workflow_id: &str) -> BearDogResult<WorkflowStatus>;
}

/// Node information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub id: String,
    pub address: String,
    pub capabilities: Vec<NodeCapability>,
    pub trust_level: f64,
    pub last_seen: DateTime<Utc>,
    pub genetics: Option<BearDogGenetics>,
}

/// Workflow status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowStatus {
    Pending,
    InProgress,
    Completed,
    Failed(String),
    RequiresApproval,
}
