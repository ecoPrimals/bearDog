//! BearDog Genetics Engine
//!
//! Implements genetic algorithms for BearDog node reproduction and evolution.
//! Nodes can spawn offspring by combining their cryptographic "genetics" -
//! capabilities, security traits, and cryptographic material.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

use crate::cross_node_auth::{
    AlgorithmFamily, BearDogGenetics, BearDogGeneticsEngine, CapabilityGene, CapabilityMutation,
    CryptoChromosome, MutationTrigger, NodeCapability, SecurityTraits, SpawnPurpose,
    SpawnRestriction, TaskType,
};
use crate::{BearDogError, BearDogResult};

/// Default implementation of the BearDog genetics engine
///
/// This is the core genetic spawning engine that enables BearDog nodes to reproduce
/// and evolve their security capabilities. The engine manages:
///
/// - **Genetic Recombination**: Combining cryptographic "genetics" from multiple parent nodes
/// - **Mutation Operations**: Introducing controlled variations to maintain genetic diversity
/// - **Capability Evolution**: Allowing nodes to adapt their security capabilities over time
/// - **Spawn Restrictions**: Enforcing genetic lineage rules and security policies
///
/// The genetic system works by treating each node's security configuration as a genetic
/// code that can be inherited, mutated, and recombined. This enables organic growth
/// of the security network while maintaining cryptographic integrity.
///
/// # Example Usage
///
/// ```rust,no_run
/// use beardog::genetics_engine::{DefaultBearDogGeneticsEngine, GeneticsConfig, InMemoryGeneticsStore};
/// use std::sync::Arc;
///
/// async fn spawn_child_node() -> Result<(), Box<dyn std::error::Error>> {
///     let store = Arc::new(InMemoryGeneticsStore::new());
///     let config = GeneticsConfig::default();
///     let engine = DefaultBearDogGeneticsEngine::new(store, config);
///     
///     // Generate initial genetics for a new node
///     let genetics = engine.generate_genesis_genetics("node-1").await?;
///     println!("Generated genetics with {} crypto chromosomes",
///              genetics.crypto_chromosomes.len());
///     Ok(())
/// }
pub struct DefaultBearDogGeneticsEngine {
    /// Storage for node genetics
    genetics_store: Arc<dyn GeneticsStore>,

    /// Random seed for genetic operations  
    genetic_seed: u64,

    /// Configuration for genetic operations
    config: GeneticsConfig,
}

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

impl DefaultBearDogGeneticsEngine {
    pub fn new(genetics_store: Arc<dyn GeneticsStore>, config: GeneticsConfig) -> Self {
        let seed = {
            let mut hasher = Sha256::new();
            hasher.update(b"BearDog-Genetics-Seed");
            hasher.update(Utc::now().timestamp().to_le_bytes());
            let hash = hasher.finalize();
            // Safe conversion with fallback
            if hash.len() >= 8 {
                let bytes: [u8; 8] = hash[0..8].try_into().unwrap_or([1, 2, 3, 4, 5, 6, 7, 8]);
                u64::from_le_bytes(bytes)
            } else {
                12345678_u64 // Safe fallback
            }
        };

        Self {
            genetics_store,
            genetic_seed: seed,
            config,
        }
    }

    fn derive_node_seed(&self, node_id: &str) -> u64 {
        let mut hasher = Sha256::new();
        hasher.update(self.genetic_seed.to_le_bytes());
        hasher.update(node_id.as_bytes());
        let hash = hasher.finalize();
        // Safe conversion with fallback
        if hash.len() >= 8 {
            let bytes: [u8; 8] = hash[0..8].try_into().unwrap_or([1, 2, 3, 4, 5, 6, 7, 8]);
            u64::from_le_bytes(bytes)
        } else {
            node_id.len() as u64 * 13579 // Safe fallback based on node ID
        }
    }

    fn pseudo_random_f64(&self, seed: u64, index: usize) -> f64 {
        let mut hasher = Sha256::new();
        hasher.update(seed.to_le_bytes());
        hasher.update(index.to_le_bytes());
        let hash = hasher.finalize();
        let bytes = if hash.len() >= 8 {
            let array: [u8; 8] = hash[0..8].try_into().unwrap_or([0, 1, 2, 3, 4, 5, 6, 7]);
            u64::from_le_bytes(array)
        } else {
            (seed + index as u64) * 12345 // Safe fallback
        };
        (bytes as f64) / (u64::MAX as f64)
    }

    fn pseudo_random_range(&self, seed: u64, index: usize, min: u64, max: u64) -> u64 {
        let rand_val = self.pseudo_random_f64(seed, index);
        min + ((rand_val * (max - min) as f64) as u64)
    }

    fn pseudo_random_bool(&self, seed: u64, index: usize, probability: f64) -> bool {
        self.pseudo_random_f64(seed, index) < probability
    }

    /// Generate initial genetics for a new BearDog node
    ///
    /// Creates the foundational genetic profile for a new node that has no parents.
    /// This "genesis" genetics serves as the starting point for the node's genetic lineage.
    ///
    /// # Process
    ///
    /// 1. **Deterministic Generation**: Uses the node ID and genetic seed to ensure
    ///    consistent genetics for the same node across restarts
    /// 2. **Crypto Chromosomes**: Generates chromosomes for encryption, signing, and hashing
    /// 3. **Capability Genes**: Creates genes for storage, compute, and spawning capabilities
    /// 4. **Security Traits**: Establishes paranoia level, cooperation tendency, and other traits
    /// 5. **Spawn Restrictions**: Sets initial policies for genetic inheritance
    ///
    /// # Arguments
    ///
    /// - `node_id`: Unique identifier for the node (affects genetic determinism)
    ///
    /// # Returns
    ///
    /// Complete `BearDogGenetics` profile ready for storage and use in spawning operations.
    ///
    /// # Security
    ///
    /// Genesis genetics are generated with strong security defaults and high paranoia levels
    /// to ensure new nodes start with robust security postures.
    pub async fn generate_genesis_genetics(&self, node_id: &str) -> BearDogResult<BearDogGenetics> {
        info!("🧬 Generating genesis genetics for node: {}", node_id);

        // Use deterministic randomness based on node_id and genetic seed
        let node_seed = self.derive_node_seed(node_id);

        // Generate crypto chromosomes with diverse algorithm families
        let crypto_chromosomes = vec![
            CryptoChromosome {
                chromosome_id: format!("{}-crypto-enc", node_id),
                algorithm_family: AlgorithmFamily::Encryption(
                    crate::cross_node_auth::EncryptionFamily::AES,
                ),
                key_material_hash: self.generate_key_hash(node_seed, 0),
                capability_flags: 0b11110000, // High encryption capabilities
                dominance_weight: 0.7 + (self.pseudo_random_f64(node_seed, 1) * 0.2),
                mutation_rate: self.config.base_mutation_rate,
            },
            CryptoChromosome {
                chromosome_id: format!("{}-crypto-sign", node_id),
                algorithm_family: AlgorithmFamily::Signing(
                    crate::cross_node_auth::SigningFamily::Ed25519,
                ),
                key_material_hash: self.generate_key_hash(node_seed, 2),
                capability_flags: 0b11001100, // Strong signing capabilities
                dominance_weight: 0.8 + (self.pseudo_random_f64(node_seed, 3) * 0.15),
                mutation_rate: self.config.base_mutation_rate * 0.5, // Signing more stable
            },
            CryptoChromosome {
                chromosome_id: format!("{}-crypto-hash", node_id),
                algorithm_family: AlgorithmFamily::Hashing(
                    crate::cross_node_auth::HashingFamily::Blake3,
                ),
                key_material_hash: self.generate_key_hash(node_seed, 4),
                capability_flags: 0b10101010, // Balanced hashing capabilities
                dominance_weight: 0.6 + (self.pseudo_random_f64(node_seed, 5) * 0.2),
                mutation_rate: self.config.base_mutation_rate * 1.2,
            },
        ];

        // Generate capability genes
        let capability_genes = vec![
            CapabilityGene {
                gene_id: format!("{}-cap-storage", node_id),
                capability: NodeCapability::EncryptedStorage {
                    max_gb: self.pseudo_random_range(node_seed, 6, 100, 1000),
                    encryption_strength: 0.8 + (self.pseudo_random_f64(node_seed, 7) * 0.2),
                },
                expression_level: 0.7 + (self.pseudo_random_f64(node_seed, 8) * 0.25),
                inherited_from: None, // Genesis node
                mutation_history: Vec::new(),
            },
            CapabilityGene {
                gene_id: format!("{}-cap-compute", node_id),
                capability: NodeCapability::SecureCompute {
                    max_cpu_hours: self.pseudo_random_range(node_seed, 9, 1000, 10000),
                    enclave_support: self.pseudo_random_bool(node_seed, 10, 0.8),
                },
                expression_level: 0.6 + (self.pseudo_random_f64(node_seed, 11) * 0.3),
                inherited_from: None,
                mutation_history: Vec::new(),
            },
            CapabilityGene {
                gene_id: format!("{}-cap-spawn", node_id),
                capability: NodeCapability::NodeSpawning {
                    max_children: self.pseudo_random_range(node_seed, 12, 5, 20) as u32,
                    genetic_diversity: 0.5 + (self.pseudo_random_f64(node_seed, 13) * 0.4),
                },
                expression_level: 0.7 + (self.pseudo_random_f64(node_seed, 14) * 0.2),
                inherited_from: None,
                mutation_history: Vec::new(),
            },
        ];

        // Generate security traits
        let security_traits = SecurityTraits {
            paranoia_level: 0.6 + (self.pseudo_random_f64(node_seed, 15) * 0.3),
            cooperation_tendency: 0.5 + (self.pseudo_random_f64(node_seed, 16) * 0.3),
            innovation_rate: 0.4 + (self.pseudo_random_f64(node_seed, 17) * 0.3),
            resource_sharing: 0.3 + (self.pseudo_random_f64(node_seed, 18) * 0.4),
            threat_sensitivity: 0.7 + (self.pseudo_random_f64(node_seed, 19) * 0.25),
            compliance_strictness: 0.8 + (self.pseudo_random_f64(node_seed, 20) * 0.15),
        };

        // Generate spawn restrictions based on security traits
        let spawn_restrictions = self.generate_spawn_restrictions(&security_traits, node_seed);

        let genetics = BearDogGenetics {
            genome_id: format!("{}-genome-{}", node_id, Uuid::new_v4()),
            crypto_chromosomes,
            capability_genes,
            security_traits,
            parent_nodes: Vec::new(), // Genesis node has no parents
            generation: 0,
            birth_timestamp: Utc::now(),
            can_spawn: true,
            max_offspring: self.pseudo_random_range(node_seed, 21, 10, 50) as u32,
            spawn_restrictions,
        };

        // Store the genetics
        self.genetics_store
            .store_genetics(node_id, &genetics)
            .await?;

        info!("✅ Generated genesis genetics for node: {}", node_id);
        Ok(genetics)
    }

    /// Perform genetic recombination between parents
    async fn perform_recombination(
        &self,
        parent_genetics: &[BearDogGenetics],
        spawn_purpose: &SpawnPurpose,
    ) -> BearDogResult<BearDogGenetics> {
        info!(
            "🧬 Performing genetic recombination between {} parents",
            parent_genetics.len()
        );

        let child_id = Uuid::new_v4().to_string();
        let recombination_seed = self.derive_node_seed(&child_id);

        // Combine crypto chromosomes from parents
        let combined_chromosomes =
            self.combine_crypto_chromosomes(parent_genetics, recombination_seed)?;

        // Combine capability genes with potential mutations
        let combined_capabilities =
            self.combine_capability_genes(parent_genetics, spawn_purpose, recombination_seed)?;

        // Blend security traits
        let blended_traits = self.blend_security_traits(parent_genetics, recombination_seed)?;

        // Determine generation
        let max_generation = parent_genetics
            .iter()
            .map(|g| g.generation)
            .max()
            .unwrap_or(0);

        // Generate spawn restrictions based on purpose and traits
        let spawn_restrictions = self.generate_purpose_specific_restrictions(
            spawn_purpose,
            &blended_traits,
            recombination_seed,
        );

        let can_spawn =
            self.determine_spawn_capability(&blended_traits, spawn_purpose, recombination_seed);
        let max_offspring =
            self.calculate_max_offspring(&blended_traits, spawn_purpose, recombination_seed);

        let child_genetics = BearDogGenetics {
            genome_id: format!("child-genome-{}", child_id),
            crypto_chromosomes: combined_chromosomes,
            capability_genes: combined_capabilities,
            security_traits: blended_traits,
            parent_nodes: parent_genetics
                .iter()
                .map(|g| g.genome_id.clone())
                .collect(),
            generation: max_generation + 1,
            birth_timestamp: Utc::now(),
            can_spawn,
            max_offspring,
            spawn_restrictions,
        };

        info!(
            "✅ Genetic recombination complete - generated child genome: {}",
            child_genetics.genome_id
        );
        Ok(child_genetics)
    }

    fn combine_crypto_chromosomes(
        &self,
        parent_genetics: &[BearDogGenetics],
        seed: u64,
    ) -> BearDogResult<Vec<CryptoChromosome>> {
        let mut combined = Vec::new();
        let mut chromosome_map: HashMap<String, Vec<&CryptoChromosome>> = HashMap::new();

        // Group chromosomes by algorithm family
        for parent in parent_genetics {
            for chromosome in &parent.crypto_chromosomes {
                let key = format!("{:?}", chromosome.algorithm_family);
                chromosome_map.entry(key).or_default().push(chromosome);
            }
        }

        // Combine chromosomes of each type
        for (algo_family, chromosomes) in chromosome_map {
            if chromosomes.is_empty() {
                continue;
            }

            // Select dominant chromosome based on dominance weight
            let dominant = chromosomes
                .iter()
                .max_by(|a, b| a.dominance_weight.partial_cmp(&b.dominance_weight).unwrap())
                .unwrap();

            // Create new chromosome with combined traits
            let mut new_chromosome = (*dominant).clone();
            new_chromosome.chromosome_id = format!("child-{}-{}", algo_family, Uuid::new_v4());

            // Blend capabilities from all parents
            new_chromosome.capability_flags = chromosomes
                .iter()
                .fold(0u64, |acc, c| acc | c.capability_flags);

            // Average dominance weights
            new_chromosome.dominance_weight =
                chromosomes.iter().map(|c| c.dominance_weight).sum::<f64>()
                    / chromosomes.len() as f64;

            // Mutate if random chance
            if self.pseudo_random_bool(seed, combined.len(), new_chromosome.mutation_rate) {
                new_chromosome = self.mutate_chromosome(new_chromosome, seed, combined.len());
            }

            combined.push(new_chromosome);
        }

        Ok(combined)
    }

    fn combine_capability_genes(
        &self,
        parent_genetics: &[BearDogGenetics],
        spawn_purpose: &SpawnPurpose,
        seed: u64,
    ) -> BearDogResult<Vec<CapabilityGene>> {
        let mut combined = Vec::new();
        let mut capability_map: HashMap<String, Vec<&CapabilityGene>> = HashMap::new();

        // Group genes by capability type
        for parent in parent_genetics {
            for gene in &parent.capability_genes {
                let key = format!("{:?}", std::mem::discriminant(&gene.capability));
                capability_map.entry(key).or_default().push(gene);
            }
        }

        // Combine genes of each type
        for (cap_type, genes) in capability_map {
            if genes.is_empty() {
                continue;
            }

            // Select most expressed gene as base
            let dominant = genes
                .iter()
                .max_by(|a, b| a.expression_level.partial_cmp(&b.expression_level).unwrap())
                .unwrap();

            let mut new_gene = (*dominant).clone();
            new_gene.gene_id = format!("child-{}-{}", cap_type, Uuid::new_v4());

            // Blend expression levels
            new_gene.expression_level =
                genes.iter().map(|g| g.expression_level).sum::<f64>() / genes.len() as f64;

            // Apply inheritance weight
            new_gene.expression_level *= self.config.capability_inheritance_weight;

            // Record inheritance
            new_gene.inherited_from = Some(dominant.gene_id.clone());

            // Check for mutations
            if self.pseudo_random_bool(seed, combined.len(), self.config.base_mutation_rate) {
                let (mutated_capability, mutation) = self.mutate_capability(
                    &new_gene.capability,
                    spawn_purpose,
                    seed,
                    combined.len(),
                );
                new_gene.capability = mutated_capability;
                new_gene.mutation_history.push(mutation);
            }

            combined.push(new_gene);
        }

        // Add purpose-specific capabilities
        if let Some(specialized_gene) =
            self.generate_purpose_specific_capability(spawn_purpose, seed)
        {
            combined.push(specialized_gene);
        }

        Ok(combined)
    }

    fn blend_security_traits(
        &self,
        parent_genetics: &[BearDogGenetics],
        seed: u64,
    ) -> BearDogResult<SecurityTraits> {
        if parent_genetics.is_empty() {
            return Err(BearDogError::Configuration {
                message: "Cannot blend traits from empty parent list".to_string(),
            });
        }

        let blend_factor = self.config.trait_blending_factor;
        let mutation_chance = self.config.base_mutation_rate * 2.0; // Traits mutate more

        // Average traits across parents
        let paranoia_level = parent_genetics
            .iter()
            .map(|g| g.security_traits.paranoia_level)
            .sum::<f64>()
            / parent_genetics.len() as f64;

        let cooperation_tendency = parent_genetics
            .iter()
            .map(|g| g.security_traits.cooperation_tendency)
            .sum::<f64>()
            / parent_genetics.len() as f64;

        let innovation_rate = parent_genetics
            .iter()
            .map(|g| g.security_traits.innovation_rate)
            .sum::<f64>()
            / parent_genetics.len() as f64;

        let resource_sharing = parent_genetics
            .iter()
            .map(|g| g.security_traits.resource_sharing)
            .sum::<f64>()
            / parent_genetics.len() as f64;

        let threat_sensitivity = parent_genetics
            .iter()
            .map(|g| g.security_traits.threat_sensitivity)
            .sum::<f64>()
            / parent_genetics.len() as f64;

        let compliance_strictness = parent_genetics
            .iter()
            .map(|g| g.security_traits.compliance_strictness)
            .sum::<f64>()
            / parent_genetics.len() as f64;

        // Apply blending factor and potential mutations
        let mut blended = SecurityTraits {
            paranoia_level: self.apply_trait_mutation(
                paranoia_level * blend_factor,
                mutation_chance,
                seed,
                0,
            ),
            cooperation_tendency: self.apply_trait_mutation(
                cooperation_tendency * blend_factor,
                mutation_chance,
                seed,
                1,
            ),
            innovation_rate: self.apply_trait_mutation(
                innovation_rate * blend_factor,
                mutation_chance,
                seed,
                2,
            ),
            resource_sharing: self.apply_trait_mutation(
                resource_sharing * blend_factor,
                mutation_chance,
                seed,
                3,
            ),
            threat_sensitivity: self.apply_trait_mutation(
                threat_sensitivity * blend_factor,
                mutation_chance,
                seed,
                4,
            ),
            compliance_strictness: self.apply_trait_mutation(
                compliance_strictness * blend_factor,
                mutation_chance,
                seed,
                5,
            ),
        };

        // Ensure all traits are within valid bounds
        blended.paranoia_level = blended.paranoia_level.clamp(0.0, 1.0);
        blended.cooperation_tendency = blended.cooperation_tendency.clamp(0.0, 1.0);
        blended.innovation_rate = blended.innovation_rate.clamp(0.0, 1.0);
        blended.resource_sharing = blended.resource_sharing.clamp(0.0, 1.0);
        blended.threat_sensitivity = blended.threat_sensitivity.clamp(0.0, 1.0);
        blended.compliance_strictness = blended.compliance_strictness.clamp(0.0, 1.0);

        Ok(blended)
    }

    fn apply_trait_mutation(
        &self,
        base_value: f64,
        mutation_chance: f64,
        seed: u64,
        index: usize,
    ) -> f64 {
        if self.pseudo_random_bool(seed, index, mutation_chance) {
            let mutation_strength = (self.pseudo_random_f64(seed, index + 100) - 0.5) * 0.2; // +/- 0.1
            base_value + mutation_strength
        } else {
            base_value
        }
    }

    // Helper methods
    fn generate_key_hash(&self, seed: u64, index: usize) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(seed.to_le_bytes());
        hasher.update(index.to_le_bytes());
        hasher.update(b"key-material");
        let hash = hasher.finalize();
        hash[0..32].to_vec()
    }

    fn generate_spawn_restrictions(
        &self,
        traits: &SecurityTraits,
        seed: u64,
    ) -> Vec<SpawnRestriction> {
        let mut restrictions = Vec::new();

        // High paranoia nodes require human approval
        if traits.paranoia_level > 0.8 {
            restrictions.push(SpawnRestriction::RequireHumanApproval);
        }

        // Low cooperation nodes require consensus
        if traits.cooperation_tendency < 0.4 {
            restrictions.push(SpawnRestriction::RequireConsensus {
                min_nodes: self.pseudo_random_range(seed, 100, 3, 8) as u32,
            });
        }

        // High compliance strictness adds resource limits
        if traits.compliance_strictness > 0.8 {
            restrictions.push(SpawnRestriction::ResourceLimits {
                max_compute: self.pseudo_random_range(seed, 101, 1000, 5000),
                max_storage: self.pseudo_random_range(seed, 102, 100, 1000),
            });
        }

        restrictions
    }

    fn mutate_chromosome(
        &self,
        mut chromosome: CryptoChromosome,
        seed: u64,
        index: usize,
    ) -> CryptoChromosome {
        // Mutate capability flags
        let mutation_mask: u64 = self.pseudo_random_range(seed, index, 0, 255);
        chromosome.capability_flags ^= mutation_mask & 0xFF; // Only flip lower 8 bits

        // Slightly adjust dominance weight
        let weight_change = (self.pseudo_random_f64(seed, index + 50) - 0.5) * 0.1; // +/- 0.05
        chromosome.dominance_weight = (chromosome.dominance_weight + weight_change).clamp(0.0, 1.0);

        chromosome
    }

    fn mutate_capability(
        &self,
        capability: &NodeCapability,
        _spawn_purpose: &SpawnPurpose,
        seed: u64,
        index: usize,
    ) -> (NodeCapability, CapabilityMutation) {
        let original = capability.clone();

        let mutated = match capability {
            NodeCapability::EncryptedStorage {
                max_gb,
                encryption_strength,
            } => {
                let gb_factor = 0.8 + (self.pseudo_random_f64(seed, index) * 0.4); // 0.8 to 1.2
                let strength_factor = 0.95 + (self.pseudo_random_f64(seed, index + 1) * 0.1); // 0.95 to 1.05
                NodeCapability::EncryptedStorage {
                    max_gb: (*max_gb as f64 * gb_factor) as u64,
                    encryption_strength: (*encryption_strength * strength_factor).clamp(0.0, 1.0),
                }
            }
            NodeCapability::SecureCompute {
                max_cpu_hours,
                enclave_support,
            } => {
                let cpu_factor = 0.9 + (self.pseudo_random_f64(seed, index) * 0.2); // 0.9 to 1.1
                NodeCapability::SecureCompute {
                    max_cpu_hours: (*max_cpu_hours as f64 * cpu_factor) as u64,
                    enclave_support: if self.pseudo_random_bool(seed, index + 1, 0.1) {
                        !enclave_support
                    } else {
                        *enclave_support
                    },
                }
            }
            // Add more capability mutations as needed
            _ => capability.clone(),
        };

        let mutation = CapabilityMutation {
            mutation_id: Uuid::new_v4().to_string(),
            original_capability: original,
            mutated_capability: mutated.clone(),
            mutation_timestamp: Utc::now(),
            mutation_trigger: MutationTrigger::GeneticRecombination,
        };

        (mutated, mutation)
    }

    fn generate_purpose_specific_capability(
        &self,
        spawn_purpose: &SpawnPurpose,
        seed: u64,
    ) -> Option<CapabilityGene> {
        match spawn_purpose {
            SpawnPurpose::TaskSpecific { task_type, .. } => {
                let capability = match task_type {
                    TaskType::ThreatResponse => NodeCapability::IncidentResponse {
                        automated_mitigation: true,
                        forensic_analysis: self.pseudo_random_bool(seed, 200, 0.8),
                    },
                    TaskType::ComplianceAudit => NodeCapability::ComplianceMonitoring {
                        standards_supported: vec!["SOX".to_string(), "GDPR".to_string()],
                        audit_depth: 0.8 + (self.pseudo_random_f64(seed, 201) * 0.2),
                    },
                    _ => return None,
                };

                Some(CapabilityGene {
                    gene_id: format!("specialized-{}-{}", task_type.to_string(), Uuid::new_v4()),
                    capability,
                    expression_level: 0.8 + (self.pseudo_random_f64(seed, 202) * 0.15),
                    inherited_from: None,
                    mutation_history: Vec::new(),
                })
            }
            _ => None,
        }
    }

    fn generate_purpose_specific_restrictions(
        &self,
        spawn_purpose: &SpawnPurpose,
        traits: &SecurityTraits,
        seed: u64,
    ) -> Vec<SpawnRestriction> {
        let mut restrictions = self.generate_spawn_restrictions(traits, seed);

        match spawn_purpose {
            SpawnPurpose::TaskSpecific { task_type, .. } => {
                restrictions.push(SpawnRestriction::PurposeRestriction {
                    allowed_tasks: vec![task_type.clone()],
                });
            }
            SpawnPurpose::EmergencyResponse { .. } => {
                restrictions.push(SpawnRestriction::TemporalRestrictions {
                    spawn_window_hours: (0..24).collect(), // Can spawn any time during emergency
                });
            }
            _ => {}
        }

        restrictions
    }

    fn determine_spawn_capability(
        &self,
        traits: &SecurityTraits,
        spawn_purpose: &SpawnPurpose,
        seed: u64,
    ) -> bool {
        // Base probability based on cooperation tendency
        let base_probability = traits.cooperation_tendency * 0.8;

        let purpose_modifier = match spawn_purpose {
            SpawnPurpose::TaskSpecific { .. } => 0.1,
            SpawnPurpose::EmergencyResponse { .. } => -0.2, // Emergency spawns less likely to spawn further
            _ => 0.0,
        };

        let final_probability = (base_probability + purpose_modifier).clamp(0.0, 1.0);
        self.pseudo_random_bool(seed, 300, final_probability)
    }

    fn calculate_max_offspring(
        &self,
        traits: &SecurityTraits,
        spawn_purpose: &SpawnPurpose,
        seed: u64,
    ) -> u32 {
        let base_offspring = (traits.resource_sharing * 20.0) as u32;

        let purpose_modifier = match spawn_purpose {
            SpawnPurpose::TaskSpecific { .. } => -5,
            SpawnPurpose::EmergencyResponse { .. } => -10,
            _ => 0,
        };

        ((base_offspring as i32 + purpose_modifier).max(1) as u32).min(50)
    }

    /// Generate deterministic pseudorandom u64 from hash
    fn generate_pseudo_random_u64(&self, seed: &str) -> BearDogResult<u64> {
        let mut hasher = Sha256::new();
        hasher.update(seed.as_bytes());
        let hash = hasher.finalize();

        // Safe byte conversion with proper error handling
        if hash.len() < 8 {
            return Err(BearDogError::InvalidInput {
                message: "Hash too short for u64 conversion".to_string(),
            });
        }

        let bytes: [u8; 8] = hash[0..8]
            .try_into()
            .map_err(|_| BearDogError::InvalidInput {
                message: "Failed to convert hash bytes to u64".to_string(),
            })?;

        Ok(u64::from_le_bytes(bytes))
    }

    /// Generate deterministic pseudorandom f64 from hash
    fn generate_pseudo_random_f64(&self, seed: &str) -> BearDogResult<f64> {
        let mut hasher = Sha256::new();
        hasher.update(seed.as_bytes());
        let hash = hasher.finalize();

        // Safe byte conversion with proper error handling
        if hash.len() < 8 {
            return Err(BearDogError::InvalidInput {
                message: "Hash too short for f64 conversion".to_string(),
            });
        }

        let bytes: [u8; 8] = hash[0..8]
            .try_into()
            .map_err(|_| BearDogError::InvalidInput {
                message: "Failed to convert hash bytes to f64".to_string(),
            })?;

        let raw_value = u64::from_le_bytes(bytes);
        Ok((raw_value as f64) / (u64::MAX as f64))
    }

    /// Generate bounded pseudorandom f64 from hash
    fn generate_bounded_pseudo_random_f64(
        &self,
        seed: &str,
        min: f64,
        max: f64,
    ) -> BearDogResult<f64> {
        let base_value = self.generate_pseudo_random_f64(seed)?;
        Ok(min + (max - min) * base_value)
    }
}

#[async_trait]
impl BearDogGeneticsEngine for DefaultBearDogGeneticsEngine {
    async fn get_node_genetics(&self, node_id: &str) -> BearDogResult<BearDogGenetics> {
        if let Some(genetics) = self.genetics_store.load_genetics(node_id).await? {
            Ok(genetics)
        } else {
            // Generate genesis genetics for new nodes
            self.generate_genesis_genetics(node_id).await
        }
    }

    async fn predict_offspring_genetics(
        &self,
        parent1: &str,
        parents: &[String],
    ) -> BearDogResult<BearDogGenetics> {
        let mut all_parents = vec![parent1.to_string()];
        all_parents.extend_from_slice(parents);

        let mut parent_genetics = Vec::new();
        for parent_id in &all_parents {
            let genetics = self.get_node_genetics(parent_id).await?;
            parent_genetics.push(genetics);
        }

        // Create a temporary spawn purpose for prediction
        let temp_purpose = SpawnPurpose::TaskSpecific {
            task_type: TaskType::DataMigration,
            max_duration: Duration::hours(24),
            resource_limits: crate::cross_node_auth::ResourceLimits {
                max_cpu_cores: 4,
                max_memory_gb: 16,
                max_storage_gb: 100,
                max_network_mbps: 100,
                max_crypto_operations_per_second: 1000,
            },
        };

        self.perform_recombination(&parent_genetics, &temp_purpose)
            .await
    }

    async fn recombine_genetics(
        &self,
        requesting_node: &str,
        co_parents: &[String],
        purpose: &SpawnPurpose,
    ) -> BearDogResult<BearDogGenetics> {
        let mut all_parents = vec![requesting_node.to_string()];
        all_parents.extend_from_slice(co_parents);

        let mut parent_genetics = Vec::new();
        for parent_id in &all_parents {
            let genetics = self.get_node_genetics(parent_id).await?;
            parent_genetics.push(genetics);
        }

        self.perform_recombination(&parent_genetics, purpose).await
    }

    async fn mutate_capabilities(
        &self,
        genetics: &BearDogGenetics,
        mutation_rate: f64,
    ) -> BearDogResult<BearDogGenetics> {
        let mut mutated = genetics.clone();
        let mutation_seed = self.derive_node_seed(&genetics.genome_id);

        // Mutate capability genes
        for (i, gene) in mutated.capability_genes.iter_mut().enumerate() {
            if self.pseudo_random_bool(mutation_seed, i, mutation_rate) {
                let temp_purpose = SpawnPurpose::TaskSpecific {
                    task_type: TaskType::DataMigration,
                    max_duration: Duration::hours(1),
                    resource_limits: crate::cross_node_auth::ResourceLimits {
                        max_cpu_cores: 1,
                        max_memory_gb: 1,
                        max_storage_gb: 1,
                        max_network_mbps: 1,
                        max_crypto_operations_per_second: 1,
                    },
                };
                let (mutated_cap, mutation) =
                    self.mutate_capability(&gene.capability, &temp_purpose, mutation_seed, i);
                gene.capability = mutated_cap;
                gene.mutation_history.push(mutation);
            }
        }

        // Mutate security traits
        let trait_mutation_rate = mutation_rate * 0.5; // Traits mutate less frequently
        mutated.security_traits.paranoia_level = self
            .apply_trait_mutation(
                mutated.security_traits.paranoia_level,
                trait_mutation_rate,
                mutation_seed,
                400,
            )
            .clamp(0.0, 1.0);

        mutated.security_traits.cooperation_tendency = self
            .apply_trait_mutation(
                mutated.security_traits.cooperation_tendency,
                trait_mutation_rate,
                mutation_seed,
                401,
            )
            .clamp(0.0, 1.0);

        Ok(mutated)
    }
}

/// Storage abstraction for genetics data
///
/// This trait defines the interface for persisting and retrieving genetic information
/// for BearDog nodes. Implementations can use various storage backends while maintaining
/// the same genetic operations interface.
///
/// # Security Requirements
///
/// - All genetic data must be stored securely to prevent genetic tampering
/// - Access control should be enforced at the storage level
/// - Genetic lineage must be preserved for audit trails
/// - Storage should support atomic operations for genetic updates
///
/// # Implementation Examples
///
/// - `InMemoryGeneticsStore`: For testing and development
/// - Database-backed stores: For production persistence
/// - Encrypted stores: For sensitive genetic material
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

/// In-memory implementation of genetics store
///
/// A simple, thread-safe storage implementation that keeps all genetic data in memory.
/// This is suitable for testing, development, and small-scale deployments where
/// persistence across restarts is not required.
///
/// # Thread Safety
///
/// Uses `Arc<RwLock<_>>` to provide safe concurrent access across multiple threads.
/// Read operations can occur concurrently, while write operations are exclusive.
///
/// # Memory Usage
///
/// All genetic data is kept in memory and will be lost when the process terminates.
/// For production use, consider implementing a database-backed storage solution.
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
    pub fn new() -> Self {
        Self {
            genetics: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl GeneticsStore for InMemoryGeneticsStore {
    async fn store_genetics(&self, node_id: &str, genetics: &BearDogGenetics) -> BearDogResult<()> {
        self.genetics
            .write()
            .await
            .insert(node_id.to_string(), genetics.clone());
        Ok(())
    }

    async fn load_genetics(&self, node_id: &str) -> BearDogResult<Option<BearDogGenetics>> {
        Ok(self.genetics.read().await.get(node_id).cloned())
    }

    async fn list_all_genetics(&self) -> BearDogResult<Vec<(String, BearDogGenetics)>> {
        Ok(self
            .genetics
            .read()
            .await
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect())
    }

    async fn delete_genetics(&self, node_id: &str) -> BearDogResult<()> {
        self.genetics.write().await.remove(node_id);
        Ok(())
    }
}

// Helper trait implementations
impl ToString for TaskType {
    fn to_string(&self) -> String {
        match self {
            TaskType::DataMigration => "data_migration".to_string(),
            TaskType::ThreatResponse => "threat_response".to_string(),
            TaskType::ComplianceAudit => "compliance_audit".to_string(),
            TaskType::KeyRecovery => "key_recovery".to_string(),
            TaskType::NetworkExpansion => "network_expansion".to_string(),
            TaskType::ComputeOffload => "compute_offload".to_string(),
            TaskType::BackupReplication => "backup_replication".to_string(),
            TaskType::EmergencyOperation => "emergency_operation".to_string(),
        }
    }
}
