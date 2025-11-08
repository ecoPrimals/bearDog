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

#[cfg(test)]
mod tests {
    use super::*;

    // BearDogGenetics tests
    #[test]
    fn test_beardog_genetics_default() {
        let genetics = BearDogGenetics::default();

        assert_eq!(genetics.id, "default-genetics");
        assert_eq!(genetics.generation, 0);
        assert_eq!(genetics.fitness_score, 0.5);
        assert_eq!(genetics.security_clearance, SecurityClearance::Basic);
        assert!(genetics.crypto_chromosomes.is_empty());
        assert!(genetics.capabilities.is_empty());
        assert!(genetics.parent_genetics.is_none());
    }

    #[test]
    fn test_beardog_genetics_creation() {
        let genetics = BearDogGenetics {
            id: "gen-001".to_string(),
            crypto_chromosomes: vec![],
            security_traits: SecurityTraits::default(),
            capabilities: vec![NodeCapability::SecurityAnalysis],
            spawn_restrictions: vec![],
            generation: 1,
            parent_genetics: Some(vec!["parent-gen".to_string()]),
            mutations: vec![],
            fitness_score: 0.85,
            security_clearance: SecurityClearance::High,
            specializations: vec![NodeSpecialization::HighPerformanceCrypto],
        };

        assert_eq!(genetics.id, "gen-001");
        assert_eq!(genetics.generation, 1);
        assert_eq!(genetics.fitness_score, 0.85);
        assert_eq!(genetics.security_clearance, SecurityClearance::High);
        assert_eq!(genetics.capabilities.len(), 1);
    }

    // AlgorithmFamily tests
    #[test]
    fn test_algorithm_family_variants() {
        let families = [
            AlgorithmFamily::EllipticCurve,
            AlgorithmFamily::RSA,
            AlgorithmFamily::PostQuantum,
            AlgorithmFamily::Symmetric,
            AlgorithmFamily::Hash,
        ];

        assert_eq!(families.len(), 5);
    }

    // NodeCapability tests
    #[test]
    fn test_node_capability_security_variants() {
        let caps = [
            NodeCapability::SecurityAnalysis,
            NodeCapability::ThreatDetection,
            NodeCapability::CryptographicAuditing,
            NodeCapability::QuantumResistant,
        ];

        assert_eq!(caps.len(), 4);
    }

    #[test]
    fn test_node_capability_encryption_strength() {
        let cap256 = NodeCapability::EncryptionStrength(256);
        let cap512 = NodeCapability::EncryptionStrength(512);

        match cap256 {
            NodeCapability::EncryptionStrength(strength) => {
                assert_eq!(strength, 256);
            }
            _ => panic!("Expected EncryptionStrength variant"),
        }

        match cap512 {
            NodeCapability::EncryptionStrength(strength) => {
                assert_eq!(strength, 512);
            }
            _ => panic!("Expected EncryptionStrength variant"),
        }
    }

    #[test]
    fn test_node_capability_compute_variants() {
        let caps = [
            NodeCapability::ComputeProvider,
            NodeCapability::AICapable,
            NodeCapability::HighThroughput,
            NodeCapability::LowLatency,
        ];

        assert_eq!(caps.len(), 4);
    }

    // SecurityTraits tests
    #[test]
    fn test_security_traits_default() {
        let traits = SecurityTraits::default();

        assert_eq!(traits.trust_threshold, 0.5);
        assert_eq!(traits.paranoia_level, 5);
        assert!(!traits.consensus_requirement);
        assert_eq!(traits.isolation_preference, 0.3);
        assert_eq!(traits.audit_frequency, 24);
    }

    #[test]
    fn test_security_traits_high_security() {
        let traits = SecurityTraits {
            trust_threshold: 0.9,
            paranoia_level: 10,
            consensus_requirement: true,
            isolation_preference: 0.8,
            audit_frequency: 1,
        };

        assert!(traits.trust_threshold > 0.8);
        assert!(traits.paranoia_level > 8);
        assert!(traits.consensus_requirement);
    }

    // SecurityClearance tests
    #[test]
    fn test_security_clearance_ordering() {
        assert!(SecurityClearance::Basic < SecurityClearance::Medium);
        assert!(SecurityClearance::Medium < SecurityClearance::High);
        assert!(SecurityClearance::High < SecurityClearance::Maximum);
    }

    #[test]
    fn test_security_clearance_equality() {
        assert_eq!(SecurityClearance::High, SecurityClearance::High);
        assert_ne!(SecurityClearance::High, SecurityClearance::Basic);
    }

    // SpawnRestriction tests
    #[test]
    fn test_spawn_restriction_max_concurrent() {
        let restriction = SpawnRestriction::MaxConcurrentSpawns(10);

        match restriction {
            SpawnRestriction::MaxConcurrentSpawns(limit) => {
                assert_eq!(limit, 10);
            }
            _ => panic!("Expected MaxConcurrentSpawns variant"),
        }
    }

    #[test]
    fn test_spawn_restriction_min_trust() {
        let restriction = SpawnRestriction::MinimumTrustLevel(0.75);

        match restriction {
            SpawnRestriction::MinimumTrustLevel(level) => {
                assert_eq!(level, 0.75);
            }
            _ => panic!("Expected MinimumTrustLevel variant"),
        }
    }

    // MutationTrigger tests
    #[test]
    fn test_mutation_trigger_variants() {
        let triggers = [
            MutationTrigger::EnvironmentalStress,
            MutationTrigger::SecurityThreat,
            MutationTrigger::PerformanceOptimization,
            MutationTrigger::EcosystemIntegration,
            MutationTrigger::UserRequirement,
        ];

        assert_eq!(triggers.len(), 5);
    }

    // NodeSpecialization tests
    #[test]
    fn test_node_specialization_variants() {
        let specs = [
            NodeSpecialization::GeneralPurpose,
            NodeSpecialization::HighPerformanceCrypto,
            NodeSpecialization::SecurityResponse,
        ];

        assert_eq!(specs.len(), 3);
    }

    #[test]
    fn test_node_specialization_ordering() {
        assert!(NodeSpecialization::GeneralPurpose < NodeSpecialization::HighPerformanceCrypto);
    }

    #[test]
    fn test_node_specialization_hash() {
        use std::collections::HashSet;

        let mut specs = HashSet::new();
        specs.insert(NodeSpecialization::GeneralPurpose);
        specs.insert(NodeSpecialization::SecurityResponse);

        assert_eq!(specs.len(), 2);
        assert!(specs.contains(&NodeSpecialization::GeneralPurpose));
    }

    // CryptoChromosome tests
    #[test]
    fn test_crypto_chromosome_creation() {
        let chromosome = CryptoChromosome {
            algorithm_family: AlgorithmFamily::EllipticCurve,
            strength_bits: 256,
            compatibility_score: 0.9,
            performance_factor: 0.8,
            security_level: 5,
        };

        assert_eq!(chromosome.strength_bits, 256);
        assert_eq!(chromosome.security_level, 5);
        assert!(chromosome.compatibility_score > 0.8);
    }

    // CapabilityMutation tests
    #[test]
    fn test_capability_mutation_creation() {
        let mutation = CapabilityMutation {
            mutation_trigger: MutationTrigger::PerformanceOptimization,
            mutation_type: "AddCapability".to_string(),
            affected_capabilities: vec![NodeCapability::HighThroughput],
            fitness_impact: 0.1,
        };

        assert_eq!(mutation.mutation_type, "AddCapability");
        assert_eq!(mutation.fitness_impact, 0.1);
        assert_eq!(mutation.affected_capabilities.len(), 1);
    }

    #[test]
    fn test_beardog_genetics_serialization() {
        let genetics = BearDogGenetics::default();
        let json = serde_json::to_string(&genetics);
        assert!(json.is_ok(), "Should be able to serialize BearDogGenetics");
    }
}
