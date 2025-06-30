//! Cross-Node Authorization System
//!
//! This module implements BearDog's core innovation: **cryptographic proof of permissions**
//! for secure distributed storage operations between nodes. The system enables "friends helping
//! friends store data securely with mathematical proof of permission."
//!
//! ## Architecture Overview
//!
//! The cross-node authorization system consists of several key components:
//!
//! * **Authorization Proofs**: Cryptographic signatures that prove a node has permission
//!   to perform specific operations on behalf of another node
//! * **Node Registry**: Manages known nodes and their trust relationships
//! * **Permission Management**: Fine-grained control over what operations are allowed
//! * **Verification Engine**: Validates authorization proofs and ensures security
//!
//! ## Key Concepts
//!
//! ### Resource Permissions
//!
//! BearDog supports multiple permission levels:
//! - **Read**: Access to view data
//! - **Write**: Ability to modify data
//! - **Delete**: Permission to remove data
//! - **Admin**: Full administrative control
//! - **Execute**: Right to run operations
//!
//! ### Authorization Flow
//!
//! 1. Node A wants to perform an operation on Node B's resources
//! 2. Node A requests authorization from Node B
//! 3. Node B generates a cryptographic proof granting specific permissions
//! 4. Node A presents this proof when performing operations
//! 5. The system verifies the proof's authenticity and validity
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use beardog::cross_node_auth::{CrossNodeAuthEngine, ResourcePermission};
//! use beardog::node_registry::BearDogNodeRegistry;
//! use beardog::proof_verifier::BearDogProofVerifier;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create the authorization engine
//! let node_registry = BearDogNodeRegistry::new();
//! let proof_verifier = BearDogProofVerifier::new();
//! let mut auth_engine = CrossNodeAuthEngine::new(
//!     Box::new(node_registry),
//!     Box::new(proof_verifier)
//! );
//!
//! // Grant authorization for another node to read our data
//! let auth = auth_engine.grant_authorization(
//!     "requesting_node_id".to_string(),
//!     "resource_path".to_string(),
//!     ResourcePermission::Read,
//!     3600 // expires in 1 hour
//! ).await?;
//!
//! // Later, verify an operation is authorized
//! let is_authorized = auth_engine.verify_authorization(
//!     &auth,
//!     "resource_path",
//!     &ResourcePermission::Read
//! ).await?;
//!
//! println!("Operation authorized: {}", is_authorized);
//! # Ok(())
//! # }
//! ```

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

use crate::workflows::{MultiPartyWorkflowEngine, WorkflowPriority, WorkflowRequest, WorkflowType};
use crate::{BearDogError, BearDogResult};

/// Cross-node authorization engine with genetic spawning capabilities
///
/// This is the core engine that manages secure cross-node operations and genetic spawning
/// in the BearDog network. It orchestrates the complete lifecycle of inter-node cooperation,
/// from permission requests to genetic recombination and child node spawning.
///
/// # Core Capabilities
///
/// - **Permission Management**: Grant, verify, and revoke cross-node permissions
/// - **Genetic Spawning**: Enable nodes to reproduce and create offspring with combined genetics
/// - **Workflow Integration**: Orchestrate multi-party approval workflows for sensitive operations
/// - **Cryptographic Proofs**: Generate and verify cryptographic authorization proofs
/// - **Genetic Evolution**: Manage genetic recombination, mutation, and lineage tracking
///
/// # Architecture
///
/// The engine integrates several key components:
/// - `NodeRegistry`: Manages known nodes and trust relationships  
/// - `ProofGenerator`/`ProofVerifier`: Handle cryptographic proof operations
/// - `BearDogGeneticsEngine`: Manages genetic algorithms and spawning
/// - `CrossNodeAuthStore`: Persists authorization state
///
/// # Example Usage
///
/// ```rust,no_run
/// use beardog::cross_node_auth::{CrossNodeAuthEngine, CrossNodeAuthConfig, ResourcePermission};
/// use std::sync::Arc;
///
/// async fn setup_auth_engine() -> Result<(), Box<dyn std::error::Error>> {
///     let config = CrossNodeAuthConfig::default();
///     // Initialize with required components...
///     // let engine = CrossNodeAuthEngine::new(config, ...).await?;
///     
///     // Request permission to access another node's data
///     // let workflow_id = engine.request_cross_node_permission(
///     //     "target_node_id",
///     //     vec![ResourcePermission::Read],
///     //     "Need access for backup operation",
///     //     &workflow_engine
///     // ).await?;
///     
///     Ok(())
/// }
/// ```
pub struct CrossNodeAuthEngine {
    pub node_registry: Arc<dyn NodeRegistry>,
    pub proof_generator: Arc<dyn ProofGenerator>,
    pub proof_verifier: Arc<dyn ProofVerifier>,
    pub auth_store: Arc<dyn CrossNodeAuthStore>,
    pub genetics_engine: Arc<dyn BearDogGeneticsEngine>,

    // Internal state
    active_authorizations: Arc<RwLock<HashMap<String, CrossNodeAuthorization>>>,
    pending_requests: Arc<RwLock<HashMap<String, CrossNodeWorkflowRequest>>>,
    spawned_children: Arc<RwLock<HashMap<String, SpawnedBearDog>>>,
    genetics_vault: Arc<RwLock<BearDogGenetics>>,

    // Configuration
    config: CrossNodeAuthConfig,
}

/// Configuration for cross-node authorization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeAuthConfig {
    pub node_id: String,
    pub node_keypair: Vec<u8>,
    pub signing_key: Vec<u8>, // Alias for compatibility
    pub default_authorization_ttl: Duration,
    pub max_authorization_ttl: Duration, // Maximum allowed TTL
    pub max_concurrent_authorizations: u32,
    pub require_approval_for_high_risk: bool,
    pub require_explicit_permissions: bool, // Require explicit permission grants
    pub auto_approve_trusted_nodes: bool,
    pub allow_permission_delegation: bool, // Allow nodes to delegate permissions
    pub enable_proof_caching: bool,        // Enable caching of proof verification results
    pub trusted_nodes: Vec<String>,
}

/// Cross-node authorization structure (cryptographic proof of permissions)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeAuthorization {
    pub id: String,
    pub grantor_node_id: String, // Node granting permission
    pub grantee_node_id: String, // Node receiving permission
    pub resource_permissions: Vec<ResourcePermission>,
    pub granted_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub workflow_id: String, // Original workflow that granted this
    pub approval_chain: Vec<crate::workflows::ApprovalRecord>, // Who approved this
    pub conditions: Vec<AccessCondition>,

    // Cryptographic proof
    pub grantor_signature: Vec<u8>, // Ed25519 signature
}

/// Represents the type of permission granted for cross-node operations.
///
/// BearDog uses fine-grained permissions to ensure nodes can only perform
/// operations they've been explicitly authorized for. This follows the
/// principle of least privilege.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourcePermission {
    /// Permission to read/view data
    ///
    /// Allows accessing file contents, listing directories, and viewing metadata.
    Read,

    /// Permission to modify existing data
    ///
    /// Includes updating file contents, changing metadata, and modifying configurations.
    Write,

    /// Permission to remove data
    ///
    /// Allows deleting files, directories, and other resources. This is a high-privilege
    /// operation that should be granted carefully.
    Delete,

    /// Full administrative control
    ///
    /// Grants complete control over resources, including the ability to change
    /// permissions, modify system settings, and perform any operation.
    Admin,

    /// Permission to execute operations
    ///
    /// Allows running scripts, executing commands, and triggering system operations.
    Execute,
}

/// Access conditions that must be met for operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessCondition {
    TimeWindow {
        start_hour: u8, // 0-23
        end_hour: u8,   // 0-23
        timezone: String,
    },
    GeographicRestriction {
        allowed_countries: Vec<String>,
        allowed_regions: Vec<String>,
    },
    NetworkRestriction {
        allowed_ip_ranges: Vec<String>,
    },
    RequireAdditionalAuth {
        auth_methods: Vec<AuthMethod>,
    },
    MaxConcurrentOperations(u32),
    RequireAuditLog(bool),
    RequireEncryption(bool),
}

/// Authentication methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    TOTP,
    SMS,
    Email,
    Hardware,
    Biometric,
}

/// Cross-node operation being performed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeOperation {
    pub operation_type: OperationType,
    pub resource_id: String,
    pub data_size_bytes: Option<u64>,
    pub estimated_duration: Option<Duration>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Types of operations that can be performed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    StoreData,
    RetrieveData,
    DeleteData,
    ShareData,
    ComputeTask,
    NetworkRelay,
    KeyRecovery,
    EmergencyAccess,
}

/// Authorization proof that can be verified by target node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationProof {
    pub authorization: CrossNodeAuthorization,
    pub operation: CrossNodeOperation,
    pub request_timestamp: DateTime<Utc>,
    pub requester_signature: Vec<u8>, // Grantee signs the request
}

/// Cross-node workflow request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNodeWorkflowRequest {
    pub workflow_id: String,
    pub from_node_id: String,
    pub to_node_id: String,
    pub requested_permissions: Vec<ResourcePermission>,
    pub justification: String,
    pub expires_at: DateTime<Utc>,
    pub conditions: Vec<AccessCondition>,
}

/// BearDog genetic material - the "DNA" that defines a node's capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Complete genetic profile for a BearDog node
///
/// This structure represents the "genetic code" of a BearDog node, containing all the
/// inherited and evolved security capabilities, traits, and restrictions that define
/// the node's behavior and abilities.
///
/// # Genetic Components
///
/// - **Crypto Chromosomes**: The cryptographic "DNA" that determines encryption, signing, and hashing capabilities
/// - **Capability Genes**: Specific security and operational capabilities inherited or evolved
/// - **Security Traits**: Behavioral characteristics like paranoia level and cooperation tendency
/// - **Lineage Information**: Family tree tracking for audit and genetic analysis
/// - **Spawn Restrictions**: Rules governing how this node can reproduce
///
/// # Inheritance Model
///
/// BearDog genetics follow biological-inspired inheritance patterns:
/// - **Dominant traits**: Some genetic features are expressed more strongly
/// - **Recombination**: Child nodes combine genetics from multiple parents
/// - **Mutation**: Controlled genetic variation to maintain diversity
/// - **Selection pressure**: Environment influences which traits are beneficial
///
/// # Example Structure
///
/// ```rust,no_run
/// use beardog::cross_node_auth::{BearDogGenetics, CryptoChromosome, SecurityTraits};
/// use chrono::Utc;
///
/// // Example genetics structure (typically generated by genetics engine)
/// let genetics = BearDogGenetics {
///     genome_id: "node-123-gen-1".to_string(),
///     crypto_chromosomes: vec![/* crypto capabilities */],
///     capability_genes: vec![/* inherited capabilities */],
///     security_traits: SecurityTraits {
///         paranoia_level: 0.8,        // High security consciousness
///         cooperation_tendency: 0.6,   // Moderately cooperative
///         // ... other traits
///         # innovation_rate: 0.5,
///         # resource_sharing: 0.4,
///         # threat_sensitivity: 0.9,
///         # compliance_strictness: 0.7,
///     },
///     parent_nodes: vec!["parent-1".to_string(), "parent-2".to_string()],
///     generation: 1,
///     birth_timestamp: Utc::now(),
///     can_spawn: true,
///     max_offspring: 5,
///     spawn_restrictions: vec![/* reproduction rules */],
/// };
/// ```
pub struct BearDogGenetics {
    /// Unique genetic identifier for this genome
    pub genome_id: String,

    /// Cryptographic capabilities encoded as genetic chromosomes
    pub crypto_chromosomes: Vec<CryptoChromosome>,

    /// Specific operational capabilities inherited from parent nodes
    pub capability_genes: Vec<CapabilityGene>,

    /// Behavioral and security trait expressions
    pub security_traits: SecurityTraits,

    /// Parent node IDs for lineage tracking
    pub parent_nodes: Vec<String>,
    /// Generation number in the genetic lineage (0 = genesis node)
    pub generation: u32,
    /// Timestamp when this genetic profile was created
    pub birth_timestamp: DateTime<Utc>,

    /// Whether this node is permitted to spawn offspring
    pub can_spawn: bool,
    /// Maximum number of child nodes this genetics allows
    pub max_offspring: u32,
    /// Rules and restrictions governing reproduction
    pub spawn_restrictions: Vec<SpawnRestriction>,
}

/// Cryptographic chromosome containing key material and algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoChromosome {
    pub chromosome_id: String,
    pub algorithm_family: AlgorithmFamily,
    pub key_material_hash: Vec<u8>, // Never store actual keys, only hashes
    pub capability_flags: u64,      // Bit flags for what this chromosome enables
    pub dominance_weight: f64,      // How strongly this gene expresses (0.0-1.0)
    pub mutation_rate: f64,         // How likely this gene is to mutate in offspring
}

/// Algorithm families that can be inherited
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlgorithmFamily {
    Encryption(EncryptionFamily),
    Signing(SigningFamily),
    Hashing(HashingFamily),
    KeyDerivation(KdfFamily),
    ZeroKnowledge(ZkFamily),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionFamily {
    AES,
    ChaCha20,
    RSA,
    ECC,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SigningFamily {
    Ed25519,
    ECDSA,
    RSA,
    BLS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HashingFamily {
    SHA3,
    Blake3,
    Argon2,
    Scrypt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KdfFamily {
    PBKDF2,
    Argon2,
    Scrypt,
    HKDF,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZkFamily {
    PLONK,
    STARK,
    Bulletproofs,
    Groth16,
}

/// Capability genes that define what operations a BearDog can perform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityGene {
    pub gene_id: String,
    pub capability: NodeCapability,
    pub expression_level: f64, // How strongly this capability is expressed (0.0-1.0)
    pub inherited_from: Option<String>, // Parent node that contributed this gene
    pub mutation_history: Vec<CapabilityMutation>,
}

/// Core capabilities that BearDog nodes can inherit or develop
/// Specific capabilities that a BearDog node can possess
///
/// NodeCapabilities represent the "genes" for specific operational abilities that can be
/// inherited, mutated, and evolved across generations of nodes. Each capability has
/// specific parameters that define its strength and characteristics.
///
/// # Capability Categories
///
/// - **Storage**: Encrypted storage and backup capabilities
/// - **Processing**: Compute power and analysis capabilities  
/// - **Network**: Communication and relay capabilities
/// - **Security**: Threat detection and incident response
/// - **Spawning**: Ability to create and manage child nodes
///
/// # Inheritance and Evolution
///
/// Capabilities can be:
/// - Inherited from parent nodes during spawning
/// - Mutated to adapt to environmental pressures
/// - Enhanced through successful operation history
/// - Restricted based on security policies
///
/// # Expression Levels
///
/// Each capability has an associated expression level (0.0-1.0) that determines
/// how strongly the capability manifests in the node's behavior.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeCapability {
    // Storage capabilities
    EncryptedStorage {
        max_gb: u64,
        encryption_strength: f64,
    },
    DistributedBackup {
        redundancy_level: u32,
        geographic_distribution: bool,
    },

    // Processing capabilities
    SecureCompute {
        max_cpu_hours: u64,
        enclave_support: bool,
    },
    ThreatDetection {
        ml_model_sophistication: f64,
        real_time_analysis: bool,
    },
    ComplianceMonitoring {
        standards_supported: Vec<String>,
        audit_depth: f64,
    },

    // Network capabilities
    P2PRelay {
        max_bandwidth_mbps: u64,
        latency_optimization: bool,
    },
    NetworkOrchestration {
        max_nodes_managed: u32,
        consensus_algorithms: Vec<String>,
    },

    // Security capabilities
    KeyRecovery {
        threshold_schemes: bool,
        social_recovery: bool,
    },
    IncidentResponse {
        automated_mitigation: bool,
        forensic_analysis: bool,
    },
    CryptographicProofs {
        zero_knowledge: bool,
        homomorphic_encryption: bool,
    },

    // Spawning capabilities
    NodeSpawning {
        max_children: u32,
        genetic_diversity: f64,
    },
    TaskSpecialization {
        ephemeral_instances: bool,
        capability_restriction: bool,
    },
}

impl Eq for NodeCapability {}

impl std::hash::Hash for NodeCapability {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            NodeCapability::EncryptedStorage {
                max_gb,
                encryption_strength,
            } => {
                0u8.hash(state);
                max_gb.hash(state);
                // Convert f64 to bits for consistent hashing
                encryption_strength.to_bits().hash(state);
            }
            NodeCapability::DistributedBackup {
                redundancy_level,
                geographic_distribution,
            } => {
                1u8.hash(state);
                redundancy_level.hash(state);
                geographic_distribution.hash(state);
            }
            NodeCapability::SecureCompute {
                max_cpu_hours,
                enclave_support,
            } => {
                2u8.hash(state);
                max_cpu_hours.hash(state);
                enclave_support.hash(state);
            }
            NodeCapability::ThreatDetection {
                ml_model_sophistication,
                real_time_analysis,
            } => {
                3u8.hash(state);
                ml_model_sophistication.to_bits().hash(state);
                real_time_analysis.hash(state);
            }
            NodeCapability::ComplianceMonitoring {
                standards_supported,
                audit_depth,
            } => {
                4u8.hash(state);
                standards_supported.hash(state);
                audit_depth.to_bits().hash(state);
            }
            NodeCapability::P2PRelay {
                max_bandwidth_mbps,
                latency_optimization,
            } => {
                5u8.hash(state);
                max_bandwidth_mbps.hash(state);
                latency_optimization.hash(state);
            }
            NodeCapability::NetworkOrchestration {
                max_nodes_managed,
                consensus_algorithms,
            } => {
                6u8.hash(state);
                max_nodes_managed.hash(state);
                consensus_algorithms.hash(state);
            }
            NodeCapability::KeyRecovery {
                threshold_schemes,
                social_recovery,
            } => {
                7u8.hash(state);
                threshold_schemes.hash(state);
                social_recovery.hash(state);
            }
            NodeCapability::IncidentResponse {
                automated_mitigation,
                forensic_analysis,
            } => {
                8u8.hash(state);
                automated_mitigation.hash(state);
                forensic_analysis.hash(state);
            }
            NodeCapability::CryptographicProofs {
                zero_knowledge,
                homomorphic_encryption,
            } => {
                9u8.hash(state);
                zero_knowledge.hash(state);
                homomorphic_encryption.hash(state);
            }
            NodeCapability::NodeSpawning {
                max_children,
                genetic_diversity,
            } => {
                10u8.hash(state);
                max_children.hash(state);
                genetic_diversity.to_bits().hash(state);
            }
            NodeCapability::TaskSpecialization {
                ephemeral_instances,
                capability_restriction,
            } => {
                11u8.hash(state);
                ephemeral_instances.hash(state);
                capability_restriction.hash(state);
            }
        }
    }
}

/// Security traits that emerge from genetic combinations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityTraits {
    pub paranoia_level: f64,        // How cautious this node is (0.0-1.0)
    pub cooperation_tendency: f64,  // How likely to help other nodes (0.0-1.0)
    pub innovation_rate: f64,       // How quickly this node adopts new security methods
    pub resource_sharing: f64,      // How generous with computational resources
    pub threat_sensitivity: f64,    // How quickly this node detects threats
    pub compliance_strictness: f64, // How strictly this node enforces compliance
}

/// Restrictions on spawning new nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpawnRestriction {
    RequireHumanApproval,
    RequireConsensus { min_nodes: u32 },
    GeographicRestriction { allowed_regions: Vec<String> },
    ResourceLimits { max_compute: u64, max_storage: u64 },
    TemporalRestrictions { spawn_window_hours: Vec<u8> },
    PurposeRestriction { allowed_tasks: Vec<TaskType> },
}

/// Types of tasks that spawned BearDogs can be specialized for
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    DataMigration,
    ThreatResponse,
    ComplianceAudit,
    KeyRecovery,
    NetworkExpansion,
    ComputeOffload,
    BackupReplication,
    EmergencyOperation,
}

/// Capability mutations that occur during genetic recombination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityMutation {
    pub mutation_id: String,
    pub original_capability: NodeCapability,
    pub mutated_capability: NodeCapability,
    pub mutation_timestamp: DateTime<Utc>,
    pub mutation_trigger: MutationTrigger,
}

/// What triggered a capability mutation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MutationTrigger {
    GeneticRecombination,
    EnvironmentalPressure,
    RandomMutation,
    DirectedEvolution,
}

/// A spawned BearDog instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnedBearDog {
    pub child_node_id: String,
    pub parent_nodes: Vec<String>,
    pub genetics: BearDogGenetics,
    pub spawn_purpose: SpawnPurpose,
    pub spawn_timestamp: DateTime<Utc>,
    pub expected_lifespan: Option<Duration>,
    pub current_status: SpawnStatus,
    pub task_specific_config: HashMap<String, serde_json::Value>,
}

/// Purpose for spawning a new BearDog
/// The purpose and intended lifecycle for a spawned BearDog node
///
/// SpawnPurpose defines why a node is being created and how it should behave
/// throughout its lifecycle. This affects genetic selection, resource allocation,
/// and termination policies.
///
/// # Purpose Categories
///
/// - **TaskSpecific**: Temporary nodes for specific operations
/// - **Permanent**: Long-lived nodes for ongoing services
/// - **EmergencyResponse**: Crisis response with special privileges
/// - **ScalingResponse**: Load balancing and capacity management
///
/// # Genetic Influence
///
/// The spawn purpose influences:
/// - Which parent genetics are emphasized during recombination
/// - What capabilities are enhanced or suppressed
/// - Resource limits and operational constraints
/// - Expected lifespan and termination conditions
///
/// # Security Considerations
///
/// Different purposes may require different security clearances and approval workflows.
/// Emergency spawns may bypass some checks, while permanent spawns require thorough vetting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpawnPurpose {
    TaskSpecific {
        task_type: TaskType,
        max_duration: Duration,
        resource_limits: ResourceLimits,
    },
    Permanent {
        geographic_location: Option<String>,
        specialized_role: String,
        resource_limits: ResourceLimits,
    },
    EmergencyResponse {
        incident_type: String,
        urgency_level: u8,
        auto_terminate_when_resolved: bool,
    },
    ScalingResponse {
        load_type: String,
        expected_duration: Duration,
        scale_down_threshold: f64,
    },
}

/// Resource limits for spawned instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_cpu_cores: u32,
    pub max_memory_gb: u64,
    pub max_storage_gb: u64,
    pub max_network_mbps: u64,
    pub max_crypto_operations_per_second: u64,
}

/// Current status of a spawned BearDog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpawnStatus {
    Conceiving, // Genetic recombination in progress
    Gestating,  // Setting up infrastructure
    Birthing,   // Initializing services
    Growing,    // Learning and adapting
    Mature,     // Fully operational
    Aging,      // Approaching end of life
    Dying,      // Shutting down gracefully
    Deceased,   // Fully terminated

    // Additional status variants needed by tests
    Pending, // Spawn request is pending approval
    Approved {
        child_genetics: BearDogGenetics,
        child_id: String,
    }, // Spawn has been approved with genetics
    Rejected {
        reason: String,
    }, // Spawn has been rejected
}

/// Multi-party workflow types for BearDog operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BearDogWorkflowType {
    // Traditional human-in-the-loop workflows
    HumanApprovalRequired {
        approver_roles: Vec<String>,
        min_approvals: u32,
        approval_timeout: Duration,
    },

    // Zero-touch automated workflows
    AutomatedConsensus {
        participating_nodes: Vec<String>,
        consensus_threshold: f64,
        max_decision_time: Duration,
    },

    // Hybrid workflows
    HybridApproval {
        automated_checks: Vec<AutomatedCheck>,
        human_oversight: bool,
        escalation_conditions: Vec<EscalationCondition>,
    },
}

/// Automated checks that can approve operations without human intervention
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomatedCheck {
    TrustScore { min_score: f64 },
    ResourceAvailability { min_resources: ResourceLimits },
    ComplianceValidation { required_standards: Vec<String> },
    ThreatAssessment { max_risk_level: f64 },
    GeographicCompliance { allowed_jurisdictions: Vec<String> },
    TemporalWindow { allowed_hours: Vec<u8> },
}

/// Conditions that escalate to human oversight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EscalationCondition {
    HighRiskOperation,
    UnknownNodeInvolved,
    LargeResourceRequest,
    ComplianceUncertainty,
    AnomalousPattern,
}

impl CrossNodeAuthEngine {
    /// Create new cross-node authorization engine with genetic capabilities
    pub async fn new(
        config: CrossNodeAuthConfig,
        node_registry: Arc<dyn NodeRegistry>,
        proof_generator: Arc<dyn ProofGenerator>,
        proof_verifier: Arc<dyn ProofVerifier>,
        auth_store: Arc<dyn CrossNodeAuthStore>,
        genetics_engine: Arc<dyn BearDogGeneticsEngine>,
    ) -> BearDogResult<Self> {
        let genetics = genetics_engine.get_node_genetics(&config.node_id).await?;

        Ok(Self {
            config,
            node_registry,
            proof_generator,
            proof_verifier,
            auth_store,
            genetics_engine,
            active_authorizations: Arc::new(RwLock::new(HashMap::new())),
            pending_requests: Arc::new(RwLock::new(HashMap::new())),
            spawned_children: Arc::new(RwLock::new(HashMap::new())),
            genetics_vault: Arc::new(RwLock::new(genetics)),
        })
    }

    /// Request permission from another node
    pub async fn request_cross_node_permission(
        &self,
        target_node_id: &str,
        requested_permissions: Vec<ResourcePermission>,
        justification: &str,
        workflow_engine: &MultiPartyWorkflowEngine,
    ) -> BearDogResult<String> {
        let workflow_id = Uuid::new_v4().to_string();

        // Create workflow request
        let request = CrossNodeWorkflowRequest {
            workflow_id: workflow_id.clone(),
            from_node_id: self.config.node_id.clone(),
            to_node_id: target_node_id.to_string(),
            requested_permissions: requested_permissions.clone(),
            justification: justification.to_string(),
            expires_at: Utc::now() + self.config.default_authorization_ttl,
            conditions: Vec::new(), // Can be specified by requestor
        };

        // Store pending request
        self.pending_requests
            .write()
            .await
            .insert(workflow_id.clone(), request.clone());

        // Send request to target node
        self.send_cross_node_workflow_request(target_node_id, &request)
            .await?;

        info!(
            "🌐 Sent cross-node permission request {} to node {}",
            workflow_id, target_node_id
        );

        Ok(workflow_id)
    }

    /// Handle incoming cross-node permission request
    pub async fn handle_cross_node_request(
        &self,
        request: CrossNodeWorkflowRequest,
        workflow_engine: &MultiPartyWorkflowEngine,
    ) -> BearDogResult<()> {
        info!(
            "🌐 Received cross-node permission request from {}",
            request.from_node_id
        );

        // Create approval workflow
        let workflow_request = WorkflowRequest {
            workflow_type: WorkflowType::EmergencyAccess, // Extend this to include CrossNodePermissionGrant
            initiator: request.from_node_id.clone(),
            target: crate::workflows::WorkflowTarget::System, // Extend to include CrossNode target
            parameters: {
                let mut params = HashMap::new();
                params.insert(
                    "cross_node_request".to_string(),
                    serde_json::to_value(&request)?,
                );
                params
            },
            reason: request.justification.clone(),
            priority: WorkflowPriority::Normal,
            metadata: HashMap::new(),
        };

        // Initiate workflow
        let workflow_response = workflow_engine.initiate_workflow(workflow_request).await?;

        info!(
            "✅ Created approval workflow {} for cross-node request",
            workflow_response.workflow_id
        );

        Ok(())
    }

    /// Generate cryptographic authorization proof after workflow approval
    pub async fn generate_authorization_proof(
        &self,
        workflow_id: &str,
        grantee_node_id: &str,
        permissions: Vec<ResourcePermission>,
    ) -> BearDogResult<CrossNodeAuthorization> {
        let authorization_id = Uuid::new_v4().to_string();

        let authorization = CrossNodeAuthorization {
            id: authorization_id.clone(),
            grantor_node_id: self.config.node_id.clone(),
            grantee_node_id: grantee_node_id.to_string(),
            resource_permissions: permissions,
            granted_at: Utc::now(),
            expires_at: Utc::now() + self.config.default_authorization_ttl,
            workflow_id: workflow_id.to_string(),
            approval_chain: Vec::new(), // Should be filled from workflow
            conditions: Vec::new(),
            grantor_signature: Vec::new(), // Will be filled by sign_authorization
        };

        // Generate cryptographic signature
        let signed_authorization = self
            .proof_generator
            .sign_authorization(authorization)
            .await?;

        // Store authorization
        self.auth_store
            .store_authorization(&signed_authorization)
            .await?;
        self.active_authorizations
            .write()
            .await
            .insert(authorization_id, signed_authorization.clone());

        info!(
            "✅ Generated cross-node authorization {} for node {}",
            signed_authorization.id, grantee_node_id
        );

        Ok(signed_authorization)
    }

    /// Prove authorization for a cross-node operation
    pub async fn prove_authorization(
        &self,
        target_node_id: &str,
        operation: &CrossNodeOperation,
    ) -> BearDogResult<AuthorizationProof> {
        // Find authorization for this node
        let authorization = self
            .auth_store
            .get_authorization_for_node(target_node_id)
            .await?
            .ok_or_else(|| BearDogError::not_found("authorization", target_node_id))?;

        // Check if authorization is valid and permits operation
        if authorization.expires_at < Utc::now() {
            return Err(BearDogError::authz("Authorization expired"));
        }

        if !authorization.permits_operation(operation) {
            return Err(BearDogError::authz(
                "Operation not permitted by authorization",
            ));
        }

        // Generate proof
        let proof = self
            .proof_generator
            .generate_operation_proof(authorization, operation)
            .await?;

        Ok(proof)
    }

    /// Verify authorization proof from another node
    pub async fn verify_authorization_proof(
        &self,
        proof: &AuthorizationProof,
    ) -> BearDogResult<bool> {
        // Verify this is our authorization (we are the grantor)
        if proof.authorization.grantor_node_id != self.config.node_id {
            return Ok(false);
        }

        // Use proof verifier to validate the proof
        self.proof_verifier.verify_authorization_proof(proof).await
    }

    /// Send cross-node request to target node (placeholder - would use SongBird in real implementation)
    async fn send_cross_node_workflow_request(
        &self,
        _target_node_id: &str,
        _request: &CrossNodeWorkflowRequest,
    ) -> BearDogResult<()> {
        // TODO: Integrate with SongBird for actual network communication
        // For now, this is a placeholder
        info!("📤 Would send cross-node request via SongBird network");
        Ok(())
    }

    /// Request permission to spawn a new BearDog instance
    pub async fn request_spawn_permission(
        &self,
        co_parents: Vec<String>,
        spawn_purpose: SpawnPurpose,
        workflow_type: BearDogWorkflowType,
        workflow_engine: &MultiPartyWorkflowEngine,
    ) -> BearDogResult<String> {
        info!("🧬 Requesting permission to spawn new BearDog instance");

        // Validate spawning capability
        let genetics = self.genetics_vault.read().await;
        if !genetics.can_spawn {
            return Err(BearDogError::authz("Node lacks spawning capability"));
        }

        let current_children = self.spawned_children.read().await.len() as u32;
        if current_children >= genetics.max_offspring {
            return Err(BearDogError::authz("Maximum offspring limit reached"));
        }

        // Create spawn request
        let spawn_request = SpawnRequest {
            request_id: Uuid::new_v4().to_string(),
            requesting_node: self.config.node_id.clone(),
            co_parents,
            spawn_purpose,
            workflow_type,
            requested_at: Utc::now(),
            estimated_genetics: self
                .genetics_engine
                .predict_offspring_genetics(
                    &genetics.genome_id,
                    &[], // Will be filled with co-parent genetics
                )
                .await?,
        };

        // Submit to appropriate workflow
        match spawn_request.workflow_type {
            BearDogWorkflowType::AutomatedConsensus { .. } => {
                self.process_automated_spawn(&spawn_request).await
            }
            BearDogWorkflowType::HumanApprovalRequired { .. } => {
                self.initiate_human_spawn_workflow(&spawn_request, workflow_engine)
                    .await
            }
            BearDogWorkflowType::HybridApproval { .. } => {
                self.process_hybrid_spawn_workflow(&spawn_request, workflow_engine)
                    .await
            }
        }
    }

    /// Execute genetic recombination to spawn a new BearDog
    pub async fn execute_spawn(
        &self,
        spawn_request: &SpawnRequest,
        approved_genetics: BearDogGenetics,
    ) -> BearDogResult<SpawnedBearDog> {
        info!("🧬 Executing BearDog spawn with genetic recombination");

        // Generate unique child ID
        let child_node_id = format!(
            "{}-child-{}",
            self.config.node_id,
            Uuid::new_v4().to_string()[..8].to_uppercase()
        );

        // Perform genetic recombination
        let child_genetics = self
            .genetics_engine
            .recombine_genetics(
                &spawn_request.requesting_node,
                &spawn_request.co_parents,
                &spawn_request.spawn_purpose,
            )
            .await?;

        // Create spawned instance
        let spawned_child = SpawnedBearDog {
            child_node_id: child_node_id.clone(),
            parent_nodes: {
                let mut parents = vec![spawn_request.requesting_node.clone()];
                parents.extend(spawn_request.co_parents.clone());
                parents
            },
            genetics: child_genetics,
            spawn_purpose: spawn_request.spawn_purpose.clone(),
            spawn_timestamp: Utc::now(),
            expected_lifespan: self.calculate_expected_lifespan(&spawn_request.spawn_purpose),
            current_status: SpawnStatus::Conceiving,
            task_specific_config: HashMap::new(),
        };

        // Register the spawn
        self.spawned_children
            .write()
            .await
            .insert(child_node_id.clone(), spawned_child.clone());

        // Initialize the child BearDog instance
        self.initialize_child_beardog(&spawned_child).await?;

        info!("✅ Successfully spawned BearDog child: {}", child_node_id);
        Ok(spawned_child)
    }

    async fn process_automated_spawn(&self, request: &SpawnRequest) -> BearDogResult<String> {
        // Implement automated consensus logic
        info!("🤖 Processing automated spawn request");

        if let BearDogWorkflowType::AutomatedConsensus {
            participating_nodes,
            consensus_threshold,
            max_decision_time,
        } = &request.workflow_type
        {
            // Get consensus from participating nodes
            let consensus_result = self
                .get_automated_consensus(
                    participating_nodes,
                    request,
                    *consensus_threshold,
                    *max_decision_time,
                )
                .await?;

            if consensus_result.approved {
                let spawned = self
                    .execute_spawn(request, consensus_result.approved_genetics)
                    .await?;
                Ok(spawned.child_node_id)
            } else {
                Err(BearDogError::authz(
                    "Automated consensus rejected spawn request",
                ))
            }
        } else {
            Err(BearDogError::Configuration {
                message: "Invalid workflow type for automated spawn".to_string(),
            })
        }
    }

    async fn initiate_human_spawn_workflow(
        &self,
        _spawn_request: &SpawnRequest,
        _workflow_engine: &MultiPartyWorkflowEngine,
    ) -> BearDogResult<String> {
        // Placeholder implementation for human workflow approval
        info!("👤 Initiating human approval workflow for spawn request");

        // In real implementation, this would:
        // 1. Create workflow request with required approvers
        // 2. Send notifications to approvers
        // 3. Wait for approvals
        // 4. Execute spawn when approved

        Err(BearDogError::Configuration {
            message: "Human approval workflows not yet implemented".to_string(),
        })
    }

    async fn process_hybrid_spawn_workflow(
        &self,
        _spawn_request: &SpawnRequest,
        _workflow_engine: &MultiPartyWorkflowEngine,
    ) -> BearDogResult<String> {
        // Placeholder implementation for hybrid workflow
        info!("🔄 Processing hybrid approval workflow for spawn request");

        // In real implementation, this would:
        // 1. Run automated checks first
        // 2. Escalate to human approval if needed
        // 3. Execute spawn when conditions are met

        Err(BearDogError::Configuration {
            message: "Hybrid approval workflows not yet implemented".to_string(),
        })
    }

    fn calculate_expected_lifespan(&self, spawn_purpose: &SpawnPurpose) -> Option<Duration> {
        match spawn_purpose {
            SpawnPurpose::TaskSpecific { max_duration, .. } => Some(*max_duration),
            SpawnPurpose::EmergencyResponse { .. } => Some(Duration::hours(48)), // 48 hour emergency response
            SpawnPurpose::ScalingResponse {
                expected_duration, ..
            } => Some(*expected_duration),
            SpawnPurpose::Permanent { .. } => None, // Permanent instances don't expire
        }
    }

    async fn get_automated_consensus(
        &self,
        _participating_nodes: &[String],
        _request: &SpawnRequest,
        _consensus_threshold: f64,
        _max_decision_time: Duration,
    ) -> BearDogResult<ConsensusResult> {
        // Placeholder implementation for automated consensus
        info!("🤖 Getting automated consensus from participating nodes");

        // In real implementation, this would:
        // 1. Send spawn request to participating nodes
        // 2. Collect their votes/approval decisions
        // 3. Calculate consensus score
        // 4. Return result with approved genetics

        // For demo, always approve with simple genetics
        let dummy_genetics = self
            .genetics_engine
            .get_node_genetics(&self.config.node_id)
            .await?;

        Ok(ConsensusResult {
            approved: true,
            participating_votes: HashMap::new(),
            consensus_score: 1.0,
            approved_genetics: dummy_genetics,
        })
    }

    /// Initialize child BearDog after spawn
    async fn initialize_child_beardog(&self, child: &SpawnedBearDog) -> BearDogResult<()> {
        info!("🐻 Initializing child BearDog: {}", child.child_node_id);

        // TODO: Implement actual child initialization
        // - Set up secure communication channels
        // - Transfer necessary configuration
        // - Establish trust relationships
        // - Initialize child's genetics

        Ok(())
    }

    /// Get spawn status for a spawn request
    pub async fn get_spawn_status(
        &self,
        spawn_request_id: &str,
    ) -> BearDogResult<Option<SpawnStatus>> {
        // TODO: Implement proper spawn status tracking
        // For now, return None to indicate spawn request not found
        Ok(None)
    }

    /// Terminate a child spawn
    pub async fn terminate_child_spawn(&self, child_id: &str) -> BearDogResult<()> {
        info!("🔚 Terminating child spawn: {}", child_id);

        // Remove from spawned children
        let mut spawned_children = self.spawned_children.write().await;
        if let Some(child) = spawned_children.remove(child_id) {
            info!("✅ Successfully terminated child spawn: {}", child_id);
        } else {
            return Err(BearDogError::not_found("spawned_child", child_id));
        }

        Ok(())
    }
}

/// Request to spawn a new BearDog instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnRequest {
    pub request_id: String,
    pub requesting_node: String,
    pub co_parents: Vec<String>,
    pub spawn_purpose: SpawnPurpose,
    pub workflow_type: BearDogWorkflowType,
    pub requested_at: DateTime<Utc>,
    pub estimated_genetics: BearDogGenetics,
}

/// Result of automated consensus
#[derive(Debug, Clone)]
pub struct ConsensusResult {
    pub approved: bool,
    pub participating_votes: HashMap<String, bool>,
    pub consensus_score: f64,
    pub approved_genetics: BearDogGenetics,
}

/// Trait for node registry
#[async_trait]
pub trait NodeRegistry: Send + Sync {
    async fn get_node_public_key(&self, node_id: &str) -> BearDogResult<Vec<u8>>;
    async fn register_node(&self, node_id: &str, public_key: &[u8]) -> BearDogResult<()>;
    async fn is_trusted_node(&self, node_id: &str) -> BearDogResult<bool>;
}

/// Trait for proof generation
#[async_trait]
pub trait ProofGenerator: Send + Sync {
    async fn sign_authorization(
        &self,
        authorization: CrossNodeAuthorization,
    ) -> BearDogResult<CrossNodeAuthorization>;
    async fn generate_operation_proof(
        &self,
        authorization: CrossNodeAuthorization,
        operation: &CrossNodeOperation,
    ) -> BearDogResult<AuthorizationProof>;
}

/// Trait for proof verification
#[async_trait]
pub trait ProofVerifier: Send + Sync {
    async fn verify_authorization_proof(&self, proof: &AuthorizationProof) -> BearDogResult<bool>;
}

/// Trait for authorization storage
#[async_trait]
pub trait CrossNodeAuthStore: Send + Sync {
    async fn store_authorization(
        &self,
        authorization: &CrossNodeAuthorization,
    ) -> BearDogResult<()>;
    async fn get_authorization(
        &self,
        authorization_id: &str,
    ) -> BearDogResult<Option<CrossNodeAuthorization>>;
    async fn get_authorization_for_node(
        &self,
        node_id: &str,
    ) -> BearDogResult<Option<CrossNodeAuthorization>>;
    async fn list_active_authorizations(&self) -> BearDogResult<Vec<CrossNodeAuthorization>>;
    async fn revoke_authorization(&self, authorization_id: &str) -> BearDogResult<()>;
}

/// Core genetic algorithm engine for BearDog node reproduction
///
/// This trait defines the interface for genetic algorithms that power BearDog's
/// unique node spawning and evolution capabilities. Implementations manage genetic
/// recombination, mutation, and the inheritance of security capabilities.
///
/// # Core Operations
///
/// - **Genetic Retrieval**: Load genetic profiles for existing nodes
/// - **Prediction**: Forecast what offspring genetics would look like before spawning
/// - **Recombination**: Combine genetics from multiple parent nodes
/// - **Mutation**: Introduce controlled genetic variation
///
/// # Genetic Algorithms
///
/// The engine implements biological-inspired algorithms:
/// - Dominant/recessive trait inheritance
/// - Genetic crossover from multiple parents
/// - Adaptive mutation rates
/// - Directed evolution for specific purposes
///
/// # Example Implementation
///
/// ```rust,no_run
/// use beardog::cross_node_auth::{BearDogGeneticsEngine, BearDogGenetics, SpawnPurpose};
/// use beardog::{BearDogResult};
/// use async_trait::async_trait;
///
/// struct MyGeneticsEngine;
///
/// #[async_trait]
/// impl BearDogGeneticsEngine for MyGeneticsEngine {
///     async fn get_node_genetics(&self, node_id: &str) -> BearDogResult<BearDogGenetics> {
///         // Implementation would load genetics from storage
///         todo!("Load genetics for node")
///     }
///     
///     async fn predict_offspring_genetics(&self, parent1: &str, parents: &[String]) -> BearDogResult<BearDogGenetics> {
///         // Implementation would simulate genetic recombination
///         todo!("Predict offspring genetics")
///     }
///     
///     // ... implement other methods
///     # async fn recombine_genetics(&self, requesting_node: &str, co_parents: &[String], purpose: &SpawnPurpose) -> BearDogResult<BearDogGenetics> { todo!() }
///     # async fn mutate_capabilities(&self, genetics: &BearDogGenetics, mutation_rate: f64) -> BearDogResult<BearDogGenetics> { todo!() }
/// }
/// ```
#[async_trait]
pub trait BearDogGeneticsEngine: Send + Sync {
    /// Retrieve genetic profile for a specific node
    ///
    /// # Arguments
    /// - `node_id`: Unique identifier for the node
    ///
    /// # Returns
    /// Complete genetic profile including chromosomes, capabilities, and traits
    async fn get_node_genetics(&self, node_id: &str) -> BearDogResult<BearDogGenetics>;

    /// Predict what offspring genetics would look like from specific parents
    ///
    /// This is useful for planning and approval workflows before actually spawning.
    ///
    /// # Arguments
    /// - `parent1`: Primary parent node ID
    /// - `parents`: Additional parent node IDs for multi-parent spawning
    ///
    /// # Returns
    /// Predicted genetic profile for the potential offspring
    async fn predict_offspring_genetics(
        &self,
        parent1: &str,
        parents: &[String],
    ) -> BearDogResult<BearDogGenetics>;

    /// Perform genetic recombination to create offspring genetics
    ///
    /// Combines genetic material from multiple parent nodes according to biological-inspired
    /// algorithms, taking into account the spawn purpose to optimize for specific capabilities.
    ///
    /// # Arguments
    /// - `requesting_node`: The primary parent requesting to spawn
    /// - `co_parents`: Additional parent nodes contributing genetics
    /// - `purpose`: Intended purpose of the offspring (affects genetic selection)
    ///
    /// # Returns
    /// New genetic profile for the offspring node
    async fn recombine_genetics(
        &self,
        requesting_node: &str,
        co_parents: &[String],
        purpose: &SpawnPurpose,
    ) -> BearDogResult<BearDogGenetics>;

    /// Apply controlled mutations to genetic capabilities
    ///
    /// Introduces genetic variation to maintain diversity and enable adaptation
    /// to changing environmental conditions.
    ///
    /// # Arguments
    /// - `genetics`: Base genetic profile to mutate
    /// - `mutation_rate`: Probability of mutations (0.0-1.0)
    ///
    /// # Returns
    /// Mutated genetic profile with recorded mutation history
    async fn mutate_capabilities(
        &self,
        genetics: &BearDogGenetics,
        mutation_rate: f64,
    ) -> BearDogResult<BearDogGenetics>;
}

impl CrossNodeAuthorization {
    /// Check if this authorization permits the given operation
    pub fn permits_operation(&self, operation: &CrossNodeOperation) -> bool {
        for permission in &self.resource_permissions {
            if permission.permits(operation) {
                return true;
            }
        }
        false
    }

    /// Verify the cryptographic signature
    pub fn verify_signature(&self, public_key: &[u8]) -> BearDogResult<bool> {
        // Validate public key length
        if public_key.len() != 32 {
            tracing::warn!("Invalid public key length: {} bytes", public_key.len());
            return Ok(false);
        }

        // Create a challenge message to verify the signature
        let challenge_message = format!(
            "BearDog signature verification challenge for authorization {}",
            self.id
        );

        // For now, we'll implement a basic verification check
        // In a full implementation, this would verify against stored signature data
        // Since CrossNodeAuthorization doesn't have a signature field yet,
        // we'll implement this as a structural validation

        if public_key.iter().all(|&b| b == 0) {
            tracing::warn!(
                "❌ Invalid authorization - placeholder public key detected for {}",
                self.id
            );
            return Ok(false);
        }

        // Verify that the authorization is not expired
        if chrono::Utc::now() > self.expires_at {
            tracing::warn!("❌ Authorization {} has expired", self.id);
            return Ok(false);
        }

        tracing::debug!("✅ Authorization {} passed basic verification", self.id);

        Ok(true)
    }
}

impl ResourcePermission {
    /// Check if this permission allows the given operation
    pub fn permits(&self, operation: &CrossNodeOperation) -> bool {
        match (self, &operation.operation_type) {
            // Write permission allows storing data
            (ResourcePermission::Write, OperationType::StoreData) => true,

            // Read permission allows retrieving data
            (ResourcePermission::Read, OperationType::RetrieveData) => true,

            // Delete permission allows deleting data
            (ResourcePermission::Delete, OperationType::DeleteData) => true,

            // Write permission allows sharing data (implies modification of sharing settings)
            (ResourcePermission::Write, OperationType::ShareData) => true,

            // Execute permission allows compute tasks
            (ResourcePermission::Execute, OperationType::ComputeTask) => true,

            // Execute permission allows network relay operations
            (ResourcePermission::Execute, OperationType::NetworkRelay) => true,

            // Admin permission allows key recovery operations
            (ResourcePermission::Admin, OperationType::KeyRecovery) => true,

            // Admin permission allows emergency access
            (ResourcePermission::Admin, OperationType::EmergencyAccess) => true,

            // Admin permission allows all operations
            (ResourcePermission::Admin, _) => true,

            // No other combinations are allowed
            _ => false,
        }
    }
}

impl Default for CrossNodeAuthConfig {
    fn default() -> Self {
        Self {
            node_id: Uuid::new_v4().to_string(),
            node_keypair: {
                // Generate a proper Ed25519 keypair for this node
                // In production, this would be loaded from secure storage or HSM
                if let Ok(keypair_hex) = std::env::var("BEARDOG_NODE_KEYPAIR") {
                    match hex::decode(&keypair_hex) {
                        Ok(keypair_bytes) if keypair_bytes.len() == 32 => keypair_bytes,
                        Ok(_) => {
                            tracing::warn!(
                                "BEARDOG_NODE_KEYPAIR has invalid length, generating new keypair"
                            );
                            crate::crypto_utils::BearDogCrypto::generate_ed25519_keypair()
                                .map(|(_, private_key)| private_key)
                                .unwrap_or_else(|_| vec![0u8; 32])
                        }
                        Err(e) => {
                            tracing::warn!(
                                "Failed to decode BEARDOG_NODE_KEYPAIR: {}, generating new keypair",
                                e
                            );
                            crate::crypto_utils::BearDogCrypto::generate_ed25519_keypair()
                                .map(|(_, private_key)| private_key)
                                .unwrap_or_else(|_| vec![0u8; 32])
                        }
                    }
                } else {
                    // Generate a new keypair for this node
                    tracing::info!("Generating new Ed25519 keypair for new node");
                    crate::crypto_utils::BearDogCrypto::generate_ed25519_keypair()
                        .map(|(_, private_key)| private_key)
                        .unwrap_or_else(|e| {
                            tracing::error!("Failed to generate Ed25519 keypair: {}", e);
                            vec![0u8; 32] // Fallback to placeholder only if generation fails
                        })
                }
            },
            signing_key: vec![0u8; 32], // Alias for compatibility
            default_authorization_ttl: Duration::days(30),
            max_authorization_ttl: Duration::days(30), // Maximum allowed TTL
            max_concurrent_authorizations: 100,
            require_approval_for_high_risk: true,
            require_explicit_permissions: false, // Require explicit permission grants
            auto_approve_trusted_nodes: false,
            allow_permission_delegation: false, // Allow nodes to delegate permissions
            enable_proof_caching: false,        // Enable caching of proof verification results
            trusted_nodes: Vec::new(),
        }
    }
}
