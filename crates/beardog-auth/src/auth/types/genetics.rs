// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearDogGenetics {
    pub id: String,
    /// Collection of crypto chromosomes
    pub crypto_chromosomes: Vec<CryptoChromosome>,
    /// The security traits value
    pub security_traits: SecurityTraits,
    /// Collection of capabilities
    pub capabilities: Vec<NodeCapability>,
    /// Collection of spawn restrictions
    pub spawn_restrictions: Vec<SpawnRestriction>,
    /// Number of generation
    pub generation: u32,
    /// Optional parent genetics
    pub parent_genetics: Option<Vec<String>>,
    /// Collection of mutations
    pub mutations: Vec<CapabilityMutation>,
    /// The fitness score value
    pub fitness_score: f64,
    /// The security clearance value
    pub security_clearance: SecurityClearance,
    /// Collection of specializations
    pub specializations: Vec<NodeSpecialization>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoChromosome {
    /// The algorithm family value
    pub algorithm_family: AlgorithmFamily,
    /// Number of `strength_bits`
    pub strength_bits: u32,
    /// The compatibility score value
    pub compatibility_score: f64,
    pub performance_factor: f64,
    /// Number of `security_level`
    pub security_level: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlgorithmFamily {
    /// Represents elliptic curve variant
    EllipticCurve,
    /// Represents r s a variant
    RSA,
    /// Represents post quantum variant
    PostQuantum,
    /// Represents symmetric variant
    Symmetric,
    /// Represents hash variant
    Hash,
}

/// Node capabilities representing the functional abilities of a node
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NodeCapability {
    // Security capabilities
    /// Represents security analysis variant
    SecurityAnalysis,
    /// Represents threat detection variant
    ThreatDetection,
    /// Currently cryptographicauditing
    CryptographicAuditing,
    /// Represents quantum resistant variant
    QuantumResistant,
    /// Represents encryption strength variant
    EncryptionStrength(u32),

    // Compute capabilities
    /// Represents compute provider variant
    ComputeProvider,
    /// Represents compute service compute variant
    ComputeServiceCompute,
    /// Represents compute capable variant
    ComputeCapable,
    /// Currently aimodeltraining
    AiModelTraining,
    /// Represents a i capable variant
    AICapable,
    /// Represents high throughput variant
    HighThroughput,
    /// Represents low latency variant
    LowLatency,

    // Storage capabilities
    /// Represents storage provider variant
    StorageProvider,
    /// Represents storage capable variant
    StorageCapable,
    /// Represents data storage variant
    DataStorage,
    /// Represents data retrieval variant
    DataRetrieval,

    // Network capabilities
    /// Represents network communication variant
    NetworkCommunication,
    /// Represents service mesh capable variant
    ServiceMeshCapable,
    /// Represents distributed consensus variant
    DistributedConsensus,

    // HSM and security operations
    /// Represents hsm operations variant
    HsmOperations,
    /// Represents key generation variant
    KeyGeneration,
    /// Currently digitalsigning
    DigitalSigning,

    // System capabilities
    /// Currently selfhealing
    SelfHealing,
    /// Represents fault tolerant variant
    FaultTolerant,
    /// Represents energy efficient variant
    EnergyEfficient,
    /// Represents universal adapter variant
    UniversalAdapter,
    /// Represents basic operations variant
    BasicOperations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityTraits {
    /// The trust threshold value
    pub trust_threshold: f64,
    /// Number of `paranoia_level`
    pub paranoia_level: u8,
    /// Whether `consensus_requirement` is enabled
    pub consensus_requirement: bool,
    /// The isolation preference value
    pub isolation_preference: f64,
    /// Number of `audit_frequency`
    pub audit_frequency: u32,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpawnRestriction {
    /// Represents max concurrent spawns variant
    MaxConcurrentSpawns(u32),
    /// Represents required capabilities variant
    RequiredCapabilities(Vec<NodeCapability>),
    ForbiddenCapabilities(Vec<NodeCapability>),
    /// Represents minimum trust level variant
    MinimumTrustLevel(f64),
    /// Represents geographic restriction variant
    GeographicRestriction(String),
    /// Represents resource limits variant
    ResourceLimits(crate::auth::types::spawning::ResourceLimits),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityMutation {
    /// The mutation trigger value
    pub mutation_trigger: MutationTrigger,
    /// The mutation type value
    pub mutation_type: String,
    /// Collection of affected capabilities
    pub affected_capabilities: Vec<NodeCapability>,
    /// The fitness impact value
    pub fitness_impact: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MutationTrigger {
    /// Represents environmental stress variant
    EnvironmentalStress,
    /// Represents security threat variant
    SecurityThreat,
    PerformanceOptimization,
    /// Represents ecosystem integration variant
    EcosystemIntegration,
    /// Represents user requirement variant
    UserRequirement,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SecurityClearance {
    /// Represents basic variant
    Basic,
    /// Represents medium variant
    Medium,
    /// Represents high variant
    High,
    /// Represents maximum variant
    Maximum,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum NodeSpecialization {
    /// Represents general purpose variant
    GeneralPurpose,
    HighPerformanceCrypto,
    /// State indicating gamingoptimized
    GamingOptimized,
    /// Currently lowlatencynetworking
    LowLatencyNetworking,
    /// Represents resource efficient variant
    ResourceEfficient,
    /// Represents server workloads variant
    ServerWorkloads,
    /// Currently edgecomputing
    EdgeComputing,
    /// State indicating networkoptimized
    NetworkOptimized,
    /// Represents security response variant
    SecurityResponse,
}
