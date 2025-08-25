// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Genetic system types for BearDog nodes
///
/// This module contains all types related to the genetic system,
/// including genetics, chromosomes, capabilities, mutations, and traits.

use serde::{Deserialize, Serialize};
/// BearDog genetic information
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
impl Default for BearDogGenetics {}


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
/// Cryptographic chromosome
/// Represents a genetic element that defines cryptographic capabilities
/// and performance characteristics of a BearDog node.
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
/// Cryptographic algorithm families
/// Categorizes different types of cryptographic algorithms that can be
/// part of a BearDog's genetic makeup.
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
/// Encryption algorithm families}


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
/// Digital signature algorithm families
pub enum SigningFamily {
    /// Ed25519 signature algorithm
    Ed25519,
    /// ECDSA signature algorithm
    Ecdsa,
    /// RSA signature algorithm
    /// Dilithium post-quantum signature
    Dilithium,
/// Hash function algorithm families}


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
/// Key derivation function families
pub enum KdfFamily {
    /// PBKDF2 key derivation
    Pbkdf2,
    /// scrypt key derivation
    /// Argon2 key derivation
    /// HKDF key derivation
    Hkdf,
/// Zero-knowledge proof algorithm families}


pub enum ZkFamily {
    /// Bulletproofs
    Bulletproofs,
    /// zk-SNARKs
    Zksnarks,
    /// zk-STARKs
    Zkstarks,
    /// PLONK
    Plonk,
/// Capability gene expression
/// Represents how a specific capability is expressed in a BearDog's genetics,
/// including inheritance patterns and mutability.
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
/// Node capability types
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
/// Security traits configuration
/// Defines behavioral security characteristics of a BearDog node,
/// including trust levels, paranoia, and operational preferences.}


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
    pub audit_frequency: u32,}


impl Default for SecurityTraits {
            trust_threshold: 0.5,
            paranoia_level: 5,
            consensus_requirement: false,
            isolation_preference: 0.3,
            audit_frequency: 24, // hours
/// Spawn restriction types
/// Defines various restrictions that can be placed on spawning
/// new BearDog instances based on genetics and security policies.}


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
    ResourceLimits(crate::auth::types::spawning::ResourceLimits),
/// Capability mutation record
/// Records a genetic mutation that occurred in a BearDog's capabilities,
/// including the trigger, type, and impact on fitness.}


pub struct CapabilityMutation {
    /// What triggered this mutation
    pub trigger: MutationTrigger,
    /// Type of mutation that occurred
    pub mutation_type: String,
    /// Which capabilities were affected
    pub affected_capabilities: Vec<NodeCapability>,
    /// Impact on fitness score
    pub fitness_impact: f64,
/// Mutation trigger types
/// Defines the various conditions that can trigger genetic mutations
/// in BearDog capabilities.
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
/// Security clearance levels for nodes
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]}


pub enum SecurityClearance {
    /// Basic security clearance
    Basic,
    /// Medium security clearance
    Medium,
    /// High security clearance
    High,
    /// Maximum security clearance
    Maximum,
/// Node specialization types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum NodeSpecialization {
    /// General purpose node
    GeneralPurpose,
    /// High performance cryptography specialized node
    HighPerformanceCrypto,
    /// Gaming optimized node
    GamingOptimized,
    /// Low latency networking specialized node
    LowLatencyNetworking,
    /// Resource efficient node
    ResourceEfficient,
    /// Server workloads optimized node
    ServerWorkloads,
    /// Edge computing specialized node
    EdgeComputing,
    /// Network optimized node
    NetworkOptimized,
    /// Security response specialized node
    SecurityResponse,
