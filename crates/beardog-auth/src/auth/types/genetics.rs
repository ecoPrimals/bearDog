

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearDogGenetics {

    pub id: String,

    pub crypto_chromosomes: Vec<CryptoChromosome>,

    pub security_traits: SecurityTraits,

    pub capabilities: Vec<NodeCapability>,

    pub spawn_restrictions: Vec<SpawnRestriction>,

    pub generation: u32,

    pub parent_genetics: Option<Vec<String>>,

    pub mutations: Vec<CapabilityMutation>,

    pub fitness_score: f64,

    pub security_clearance: SecurityClearance,

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

    pub algorithm_family: AlgorithmFamily,

    pub strength_bits: u32,

    pub compatibility_score: f64,

    pub performance_factor: f64,

    pub security_level: u8,
}

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
    Aes,
    ChaCha,
    Blowfish,
    Rsa,
    Ecc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SigningFamily {

    Ed25519,

    Ecdsa,

    Dilithium,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HashingFamily {
    Sha2,
    Sha3,
    Blake,
    Argon2,
    Scrypt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KdfFamily {
    Pbkdf2,
    Hkdf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZkFamily {
    Bulletproofs,
    Zksnarks,
    Zkstarks,
    Plonk,
}

pub struct CapabilityGene {

    pub capability: NodeCapability,

    pub expression_level: f64,

    pub dominant: bool,

    pub mutable: bool,

    pub inheritance_weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeCapability {

    StorageProvider,

    ComputeProvider,

    NetworkRelay,

    SecurityAnalysis,

    EncryptionStrength(u32),

    QuantumResistant,

    MultiPartyComputation,

    HomomorphicEncryption,

    ZeroKnowledgeProofs,

    BlockchainIntegration,

    BiometricProcessing,

    AiModelTraining,

    DistributedConsensus,

    CryptographicAuditing,

    ThreatDetection,

    ToadStoolCompute,

    SongBirdDiscovery,

    NestGateStorage,

    SquirrelPlugins,

    HighThroughput,

    LowLatency,

    EnergyEfficient,

    FaultTolerant,

    SelfHealing,

    PenetrationTesting,

    VulnerabilityScanning,

    IncidentResponse,

    ForensicAnalysis,

    ComplianceAuditing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityTraits {

    pub trust_threshold: f64,

    pub paranoia_level: u8,

    pub consensus_requirement: bool,

    pub isolation_preference: f64,

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
    MaxConcurrentSpawns(u32),
    RequiredCapabilities(Vec<NodeCapability>),
    ForbiddenCapabilities(Vec<NodeCapability>),
    MinimumTrustLevel(f64),
    GeographicRestriction(String),
    ResourceLimits(crate::auth::types::spawning::ResourceLimits),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityMutation {

    pub trigger: MutationTrigger,

    pub mutation_type: String,

    pub affected_capabilities: Vec<NodeCapability>,

    pub fitness_impact: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MutationTrigger {
    EnvironmentalStress,
    SecurityThreat,
    PerformanceOptimization,
    EcosystemIntegration,
    UserRequirement,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SecurityClearance {

    Basic,

    Medium,

    High,

    Maximum,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
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
