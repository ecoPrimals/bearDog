//! Type definitions for cross-node authorization
//!
//! Contains all structs, enums, and type aliases for the authorization module.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::BearDogResult;

/// Configuration for cross-node authorization
///
/// Central configuration for the cross-node authorization system, defining
/// policies for proof verification, genetic spawning, consensus requirements,
/// and workflow automation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeAuthConfig {
    /// Enable proof verification for authorization requests
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
///
/// Represents an authorization grant from one node to another, including
/// permissions, conditions, expiration, and cryptographic validation.
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
    /// Whether the authorization is currently active
    pub is_active: bool,
}

/// Resource permission levels
///
/// Defines the different types of permissions that can be granted
/// for accessing resources across nodes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

/// Access condition modifiers
///
/// Defines conditions that must be met for an authorization to be valid,
/// such as time windows, IP restrictions, and usage limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessCondition {
    /// Time-based access window
    TimeWindow {
        /// Start time of the access window
        start: DateTime<Utc>,
        /// End time of the access window
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
        /// Maximum requests allowed
        max_requests: u32,
        /// Time window in seconds
        window_seconds: u32,
    },
    /// Require consensus from other nodes
    RequireConsensus {
        /// Required consensus threshold (0.0 to 1.0)
        threshold: f64,
        /// List of nodes that must participate
        nodes: Vec<String>,
    },
}

/// Authentication method types
///
/// Defines the different methods that can be used for authentication
/// in cross-node communications.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    /// Digital signature authentication
    Signature,
    /// Certificate-based authentication
    Certificate,
    /// Biometric hash authentication
    BiometricHash,
    /// Mutual TLS authentication
    MutualTls,
}

/// Cross-node operation descriptor
///
/// Describes a specific operation being performed across nodes,
/// including the operation type, target resource, and parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeOperation {
    /// Type of operation being performed
    pub operation_type: OperationType,
    /// Target resource identifier
    pub target_resource: String,
    /// Operation parameters
    pub parameters: HashMap<String, String>,
    /// Signature of the requester
    pub requester_signature: String,
}

/// Operation types for cross-node operations
///
/// Defines the different types of operations that can be performed
/// across nodes in the BearDog network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    /// Read operation
    Read,
    /// Write operation
    Write,
    /// Delete operation
    Delete,
    /// Execute operation
    Execute,
    /// Backup operation
    Backup,
    /// Restore operation
    Restore,
    /// Spawn operation
    Spawn,
    /// Consensus operation
    Consensus,
}

/// Authorization proof structure
///
/// Cryptographic proof that an authorization is valid and can be used
/// for a specific operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationProof {
    /// ID of the authorization being proven
    pub authorization_id: String,
    /// Operation being authorized
    pub operation: CrossNodeOperation,
    /// Timestamp when proof was generated
    pub timestamp: DateTime<Utc>,
    /// Cryptographic signature of the proof
    pub proof_signature: String,
}

/// Cross-node workflow request
///
/// Represents a request to execute a workflow across multiple nodes,
/// including automation checks and escalation conditions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeWorkflowRequest {
    /// Unique request identifier
    pub id: String,
    /// Type of workflow requested
    pub workflow_type: BearDogWorkflowType,
    /// Node ID making the request
    pub requester_node_id: String,
    /// Target nodes for the workflow
    pub target_nodes: Vec<String>,
    /// Required permissions for the workflow
    pub required_permissions: Vec<ResourcePermission>,
    /// Automated checks to perform
    pub automated_checks: Vec<AutomatedCheck>,
    /// Conditions that trigger escalation
    pub escalation_conditions: Vec<EscalationCondition>,
    /// When the request was created
    pub created_at: DateTime<Utc>,
    /// When the request expires
    pub expires_at: DateTime<Utc>,
}

/// BearDog genetic information
///
/// Represents the genetic makeup of a BearDog node, including cryptographic
/// capabilities, security traits, and behavioral characteristics that can
/// be inherited and mutated during spawning.
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
    /// Security clearance level
    pub security_clearance: SecurityClearance,
    /// Node specializations
    pub specializations: Vec<NodeSpecialization>,
}

/// Cryptographic chromosome
///
/// Represents a genetic element that defines cryptographic capabilities
/// and performance characteristics of a BearDog node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoChromosome {
    /// Cryptographic algorithm family
    pub algorithm_family: AlgorithmFamily,
    /// Cryptographic strength in bits
    pub strength_bits: u32,
    /// Compatibility score with other algorithms
    pub compatibility_score: f64,
    /// Performance factor for this algorithm
    pub performance_factor: f64,
    /// Security level (1-10)
    pub security_level: u8,
}

/// Cryptographic algorithm families
///
/// Categorizes different types of cryptographic algorithms that can be
/// part of a BearDog's genetic makeup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlgorithmFamily {
    /// Encryption algorithms
    Encryption(EncryptionFamily),
    /// Digital signature algorithms
    Signing(SigningFamily),
    /// Hash function algorithms
    Hashing(HashingFamily),
    /// Key derivation algorithms
    KeyDerivation(KdfFamily),
    /// Zero-knowledge proof algorithms
    ZeroKnowledge(ZkFamily),
}

/// Encryption algorithm families
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionFamily {
    /// AES encryption
    Aes,
    /// ChaCha encryption
    ChaCha,
    /// Blowfish encryption
    Blowfish,
    /// RSA encryption
    Rsa,
    /// Elliptic Curve Cryptography
    Ecc,
}

/// Digital signature algorithm families
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SigningFamily {
    /// Ed25519 signature algorithm
    Ed25519,
    /// ECDSA signature algorithm
    Ecdsa,
    /// RSA signature algorithm
    Rsa,
    /// Dilithium post-quantum signature
    Dilithium,
}

/// Hash function algorithm families
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HashingFamily {
    /// SHA-2 family
    Sha2,
    /// SHA-3 family
    Sha3,
    /// BLAKE hash function
    Blake,
    /// Argon2 password hashing
    Argon2,
    /// scrypt password hashing
    Scrypt,
}

/// Key derivation function families
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KdfFamily {
    /// PBKDF2 key derivation
    Pbkdf2,
    /// scrypt key derivation
    Scrypt,
    /// Argon2 key derivation
    Argon2,
    /// HKDF key derivation
    Hkdf,
}

/// Zero-knowledge proof algorithm families
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZkFamily {
    /// Bulletproofs
    Bulletproofs,
    /// zk-SNARKs
    Zksnarks,
    /// zk-STARKs
    Zkstarks,
    /// PLONK
    Plonk,
}

/// Capability gene expression
///
/// Represents how a specific capability is expressed in a BearDog's genetics,
/// including inheritance patterns and mutability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityGene {
    /// The capability this gene represents
    pub capability: NodeCapability,
    /// Expression level (0.0 to 1.0)
    pub expression_level: f64,
    /// Whether this gene is dominant in inheritance
    pub dominant: bool,
    /// Whether this gene can mutate
    pub mutable: bool,
    /// Weight in inheritance calculations
    pub inheritance_weight: f64,
}

/// Node capability types
///
/// Defines the various capabilities that a BearDog node can possess,
/// organized by category and specialization.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeCapability {
    // Core capabilities
    /// Provides data storage services
    StorageProvider,
    /// Provides computational services
    ComputeProvider,
    /// Acts as a network relay
    NetworkRelay,
    /// Performs security analysis
    SecurityAnalysis,
    /// Encryption strength (bits)
    EncryptionStrength(u32),

    // Advanced capabilities
    /// Quantum-resistant cryptography
    QuantumResistant,
    /// Multi-party computation support
    MultiPartyComputation,
    /// Homomorphic encryption support
    HomomorphicEncryption,
    /// Zero-knowledge proof support
    ZeroKnowledgeProofs,
    /// Blockchain integration
    BlockchainIntegration,

    // Specialized capabilities
    /// Biometric data processing
    BiometricProcessing,
    /// AI model training
    AiModelTraining,
    /// Distributed consensus participation
    DistributedConsensus,
    /// Cryptographic auditing
    CryptographicAuditing,
    /// Threat detection and analysis
    ThreatDetection,

    // Ecosystem integration
    /// ToadStool compute integration
    ToadStoolCompute,
    /// SongBird service discovery
    SongBirdDiscovery,
    /// NestGate storage integration
    NestGateStorage,
    /// Squirrel plugin support
    SquirrelPlugins,

    // Performance characteristics
    /// High throughput processing
    HighThroughput,
    /// Low latency operations
    LowLatency,
    /// Energy efficient operations
    EnergyEfficient,
    /// Fault tolerance
    FaultTolerant,
    /// Self-healing capabilities
    SelfHealing,

    // Security specializations
    /// Penetration testing
    PenetrationTesting,
    /// Vulnerability scanning
    VulnerabilityScanning,
    /// Incident response
    IncidentResponse,
    /// Forensic analysis
    ForensicAnalysis,
    /// Compliance auditing
    ComplianceAuditing,
}

/// Security traits configuration
///
/// Defines behavioral security characteristics of a BearDog node,
/// including trust levels, paranoia, and operational preferences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityTraits {
    /// Trust threshold for accepting operations (0.0 to 1.0)
    pub trust_threshold: f64,
    /// Paranoia level (1-10 scale)
    pub paranoia_level: u8,
    /// Whether consensus is required for operations
    pub consensus_requirement: bool,
    /// Preference for isolation (0.0 to 1.0)
    pub isolation_preference: f64,
    /// Frequency of security audits
    pub audit_frequency: u32,
}

/// Spawn restriction types
///
/// Defines various restrictions that can be placed on spawning
/// new BearDog instances based on genetics and security policies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpawnRestriction {
    /// Maximum number of concurrent spawns
    MaxConcurrentSpawns(u32),
    /// Required capabilities for spawning
    RequiredCapabilities(Vec<NodeCapability>),
    /// Forbidden capabilities for spawning
    ForbiddenCapabilities(Vec<NodeCapability>),
    /// Minimum trust level required
    MinimumTrustLevel(f64),
    /// Geographic restriction
    GeographicRestriction(String),
    /// Resource limits for spawned instances
    ResourceLimits(ResourceLimits),
}

/// Task types for BearDog operations
///
/// Defines the different types of tasks that can be assigned
/// to BearDog nodes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TaskType {
    /// Data storage tasks
    DataStorage,
    /// Legacy storage alias
    Storage,
    /// Computation tasks
    ComputeTask,
    /// Legacy compute alias
    Compute,
    /// Security analysis tasks
    SecurityAnalysis,
    /// Legacy security alias
    Security,
    /// Network relay tasks
    NetworkRelay,
    /// Legacy network alias
    Network,
    /// Compliance checking tasks
    ComplianceCheck,
    /// Threat hunting tasks
    ThreatHunting,
    /// Backup operation tasks
    BackupOperation,
    /// Disaster recovery tasks
    DisasterRecovery,
}

/// Capability mutation record
///
/// Records a genetic mutation that occurred in a BearDog's capabilities,
/// including the trigger, type, and impact on fitness.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityMutation {
    /// What triggered this mutation
    pub trigger: MutationTrigger,
    /// Type of mutation that occurred
    pub mutation_type: String,
    /// Which capabilities were affected
    pub affected_capabilities: Vec<NodeCapability>,
    /// Impact on fitness score
    pub fitness_impact: f64,
}

/// Mutation trigger types
///
/// Defines the various conditions that can trigger genetic mutations
/// in BearDog capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MutationTrigger {
    /// Environmental stress conditions
    EnvironmentalStress,
    /// Security threat response
    SecurityThreat,
    /// Performance optimization need
    PerformanceOptimization,
    /// Ecosystem integration requirement
    EcosystemIntegration,
    /// User-requested change
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

/// Security clearance levels for nodes
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SecurityClearance {
    Basic,
    Medium,
    High,
    Maximum,
}

/// Node specialization types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeSpecialization {
    GeneralPurpose,
    HighPerformanceCrypto,
    GamingOptimized,
    LowLatencyNetworking,
    ResourceEfficient,
    ServerWorkloads,
    EdgeComputing,
    NetworkOptimized,
    SecurityResponse,
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

impl Default for BearDogGenetics {
    fn default() -> Self {
        Self {
            id: "default-genetics".to_string(),
            crypto_chromosomes: vec![],
            security_traits: SecurityTraits::default(),
            capabilities: vec![],
            spawn_restrictions: vec![],
            generation: 0,
            parent_genetics: None,
            mutations: vec![],
            fitness_score: 0.5,
            security_clearance: SecurityClearance::Basic,
            specializations: vec![NodeSpecialization::GeneralPurpose],
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
    fn generate_proof(
        &self,
        authorization: &CrossNodeAuthorization,
        operation: &CrossNodeOperation,
    ) -> BearDogResult<AuthorizationProof>;
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
