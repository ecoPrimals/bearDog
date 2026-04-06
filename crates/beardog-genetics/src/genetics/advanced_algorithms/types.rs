// SPDX-License-Identifier: AGPL-3.0-or-later

//! Data types for advanced genetic algorithms (configuration, individuals, fitness, optimization).

use crate::genetics::entropy_hierarchy::{EntropyClass, MachineEntropySource, MachineSourceType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Genetic signature carried by evolved individuals (aligned with biome genetics shape).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticSignature {
    /// Biome identifier
    pub biome_id: String,
    /// Signature hash
    pub signature_hash: String,
    /// Entropy classification
    pub entropy_class: EntropyClass,
    /// Quality score (0.0–1.0)
    pub quality_score: f64,
    /// Creation time
    pub created_at: DateTime<Utc>,
    /// Lineage depth
    pub lineage_depth: u32,
    /// Mixed signature material
    pub mixed_signatures: Vec<String>,
}

impl Default for GeneticSignature {
    fn default() -> Self {
        let entropy_class = EntropyClass::StoreBoughtMachine {
            quality_score: 0.5,
            source_type: MachineEntropySource {
                source_type: MachineSourceType::CSPRNG {
                    algorithm: "chacha20".to_string(),
                    seed_source: "os".to_string(),
                },
                algorithm: "chacha20".to_string(),
                seed_source: "os".to_string(),
                quality_metrics: HashMap::new(),
            },
            generation_timestamp: Utc::now(),
            reproducibility_index: 0.0,
        };
        Self {
            biome_id: String::new(),
            signature_hash: String::new(),
            entropy_class,
            quality_score: 0.5,
            created_at: Utc::now(),
            lineage_depth: 0,
            mixed_signatures: Vec::new(),
        }
    }
}

/// Evolution configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionConfig {
    /// Population size
    pub population_size: usize,

    /// Mutation rate (0.0 - 1.0)
    pub mutation_rate: f64,

    /// Crossover rate (0.0 - 1.0)
    pub crossover_rate: f64,

    /// Elitism percentage (0.0 - 1.0)
    pub elitism_percentage: f64,

    /// Maximum generations
    pub max_generations: u32,

    /// Fitness threshold for convergence
    pub fitness_threshold: f64,

    /// Enable diversity preservation
    pub diversity_preservation: bool,

    /// Enable adaptive parameters
    pub adaptive_parameters: bool,
}

impl Default for EvolutionConfig {
    fn default() -> Self {
        Self {
            population_size: 100,
            mutation_rate: 0.05,
            crossover_rate: 0.8,
            elitism_percentage: 0.1,
            max_generations: 1000,
            fitness_threshold: 0.95,
            diversity_preservation: true,
            adaptive_parameters: true,
        }
    }
}

impl EvolutionConfig {
    /// Load evolution parameters from `BEARDOG_GENETICS_*` environment variables (see source for names).
    pub fn from_env() -> Self {
        Self {
            population_size: std::env::var("BEARDOG_GENETICS_POPULATION_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(100),
            mutation_rate: std::env::var("BEARDOG_GENETICS_MUTATION_RATE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.05),
            crossover_rate: std::env::var("BEARDOG_GENETICS_CROSSOVER_RATE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.8),
            elitism_percentage: std::env::var("BEARDOG_GENETICS_ELITISM_PERCENTAGE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.1),
            max_generations: std::env::var("BEARDOG_GENETICS_MAX_GENERATIONS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1000),
            fitness_threshold: std::env::var("BEARDOG_GENETICS_FITNESS_THRESHOLD")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.95),
            diversity_preservation: true,
            adaptive_parameters: std::env::var("BEARDOG_GENETICS_ADAPTIVE_PARAMETERS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(true),
        }
    }
}

/// Genetic individual
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticIndividual {
    /// Unique ID
    pub id: String,

    /// Genetic signature
    pub genetic_signature: GeneticSignature,

    /// Fitness score (0.0 - 1.0)
    pub fitness_score: f64,

    /// Age in generations
    pub age: u32,

    /// Parent IDs
    pub parent_ids: Vec<String>,

    /// Mutation history
    pub mutation_history: Vec<MutationRecord>,

    /// Performance metrics
    pub performance_metrics: PerformanceMetrics,

    /// Specialization traits
    pub specialization_traits: Vec<String>,
}

impl Default for GeneticIndividual {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            genetic_signature: GeneticSignature::default(),
            fitness_score: 0.0,
            age: 0,
            parent_ids: Vec::new(),
            mutation_history: Vec::new(),
            performance_metrics: PerformanceMetrics::default(),
            specialization_traits: Vec::new(),
        }
    }
}

/// Generation snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationSnapshot {
    /// Generation number
    pub generation: u32,

    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// Population size
    pub population_size: usize,

    /// Average fitness
    pub average_fitness: f64,

    /// Best fitness
    pub best_fitness: f64,

    /// Diversity index
    pub diversity_index: f64,

    /// Convergence rate
    pub convergence_rate: f64,
}

impl Default for GenerationSnapshot {
    fn default() -> Self {
        Self {
            generation: 0,
            timestamp: Utc::now(),
            population_size: 0,
            average_fitness: 0.0,
            best_fitness: 0.0,
            diversity_index: 0.0,
            convergence_rate: 0.0,
        }
    }
}

/// Mutation record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationRecord {
    /// Mutation type
    pub mutation_type: MutationType,

    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// Fitness impact
    pub fitness_impact: f64,

    /// Success rate
    pub success_rate: f64,
}

impl Default for MutationRecord {
    fn default() -> Self {
        Self {
            mutation_type: MutationType::Random,
            timestamp: Utc::now(),
            fitness_impact: 0.0,
            success_rate: 0.0,
        }
    }
}

/// Mutation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum MutationType {
    /// Random mutation
    #[default]
    Random,
    /// Entropy quality adjustment
    EntropyQualityAdjustment,
    /// Trust level shift
    TrustLevelShift,
    /// Specialization change
    SpecializationChange,
    /// Performance optimization
    PerformanceOptimization,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Authorization success rate
    pub authorization_success_rate: f64,

    /// Operation efficiency
    pub operation_efficiency: f64,

    /// Collaboration score
    pub collaboration_score: f64,

    /// Security rating
    pub security_rating: f64,

    /// Adaptability index
    pub adaptability_index: f64,

    /// Resource utilization
    pub resource_utilization: f64,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            authorization_success_rate: 0.5,
            operation_efficiency: 0.5,
            collaboration_score: 0.5,
            security_rating: 0.5,
            adaptability_index: 0.5,
            resource_utilization: 0.5,
        }
    }
}

/// Fitness criterion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitnessCriterion {
    /// Criterion name
    pub name: String,

    /// Weight (0.0 - 1.0)
    pub weight: f64,

    /// Evaluation function identifier
    pub evaluation_function: String,

    /// Target value
    pub target_value: f64,

    /// Tolerance
    pub tolerance: f64,
}

impl Default for FitnessCriterion {
    fn default() -> Self {
        Self {
            name: String::new(),
            weight: 1.0,
            evaluation_function: String::new(),
            target_value: 1.0,
            tolerance: 0.1,
        }
    }
}

/// Optimization goal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationGoal {
    /// Goal name
    pub name: String,

    /// Target value
    pub target_value: f64,

    /// Optimization strategy
    pub optimization_strategy: String,

    /// Constraints
    pub constraints: Vec<OptimizationConstraint>,
}

impl Default for OptimizationGoal {
    fn default() -> Self {
        Self {
            name: String::new(),
            target_value: 1.0,
            optimization_strategy: "maximize".to_string(),
            constraints: Vec::new(),
        }
    }
}

/// Optimization constraint
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OptimizationConstraint {
    /// Constraint name
    pub name: String,

    /// Minimum value
    pub min_value: Option<f64>,

    /// Maximum value
    pub max_value: Option<f64>,

    /// Fixed value
    pub fixed_value: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genetics::entropy_hierarchy::{
        EntropyClass, MachineEntropySource, MachineSourceType,
    };
    use chrono::Utc;

    #[test]
    fn genetic_signature_serde_roundtrip() {
        let sig = GeneticSignature::default();
        let json = serde_json::to_string(&sig).expect("serialize GeneticSignature");
        let back: GeneticSignature = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.biome_id, sig.biome_id);
    }

    #[test]
    fn evolution_config_serde_roundtrip() {
        let c = EvolutionConfig::default();
        let json = serde_json::to_string(&c).expect("ser");
        let back: EvolutionConfig = serde_json::from_str(&json).expect("de");
        assert_eq!(back.population_size, c.population_size);
        assert_eq!(back.mutation_rate, c.mutation_rate);
    }

    #[test]
    fn evolution_config_from_env_returns_valid_structure() {
        let c = EvolutionConfig::from_env();
        assert!(c.population_size > 0);
        assert!(c.mutation_rate >= 0.0 && c.mutation_rate <= 1.0);
        assert!(c.max_generations > 0);
    }

    #[test]
    fn genetic_individual_generation_snapshot_and_mutation_record_serde() {
        let ind = GeneticIndividual::default();
        let js = serde_json::to_string(&ind).expect("ind ser");
        let _: GeneticIndividual = serde_json::from_str(&js).expect("ind de");

        let snap = GenerationSnapshot::default();
        let s = serde_json::to_string(&snap).expect("snap ser");
        let _: GenerationSnapshot = serde_json::from_str(&s).expect("snap de");

        let m = MutationRecord::default();
        let ms = serde_json::to_string(&m).expect("mut ser");
        let _: MutationRecord = serde_json::from_str(&ms).expect("mut de");
    }

    #[test]
    fn mutation_type_roundtrip_json() {
        let variants = [
            MutationType::Random,
            MutationType::EntropyQualityAdjustment,
            MutationType::TrustLevelShift,
            MutationType::SpecializationChange,
            MutationType::PerformanceOptimization,
        ];
        for v in variants {
            let j = serde_json::to_string(&v).expect("serde");
            let back: MutationType = serde_json::from_str(&j).expect("de");
            assert_eq!(back, v);
        }
    }

    #[test]
    fn performance_metrics_and_fitness_criterion_defaults() {
        let pm = PerformanceMetrics::default();
        assert_eq!(pm.authorization_success_rate, 0.5);
        let fc = FitnessCriterion::default();
        assert_eq!(fc.weight, 1.0);
        let og = OptimizationGoal::default();
        assert_eq!(og.optimization_strategy, "maximize");
        let oc = OptimizationConstraint::default();
        assert!(oc.name.is_empty());
    }

    #[test]
    fn genetic_signature_custom_entropy_class_roundtrip() {
        let sig = GeneticSignature {
            biome_id: "b".to_string(),
            signature_hash: "h".to_string(),
            entropy_class: EntropyClass::StoreBoughtMachine {
                quality_score: 0.9,
                source_type: MachineEntropySource {
                    source_type: MachineSourceType::CSPRNG {
                        algorithm: "x".to_string(),
                        seed_source: "y".to_string(),
                    },
                    algorithm: "x".to_string(),
                    seed_source: "y".to_string(),
                    quality_metrics: std::collections::HashMap::new(),
                },
                generation_timestamp: Utc::now(),
                reproducibility_index: 0.1,
            },
            quality_score: 0.8,
            created_at: Utc::now(),
            lineage_depth: 3,
            mixed_signatures: vec!["a".to_string()],
        };
        let json = serde_json::to_string(&sig).expect("ser");
        let back: GeneticSignature = serde_json::from_str(&json).expect("de");
        assert_eq!(back.biome_id, "b");
        assert_eq!(back.lineage_depth, 3);
    }

    #[test]
    fn optimization_constraint_serde_with_bounds() {
        let c = OptimizationConstraint {
            name: "cpu".to_string(),
            min_value: Some(0.0),
            max_value: Some(1.0),
            fixed_value: None,
        };
        let j = serde_json::to_string(&c).expect("ser");
        let back: OptimizationConstraint = serde_json::from_str(&j).expect("de");
        assert_eq!(back.name, "cpu");
        assert_eq!(back.min_value, Some(0.0));
    }
}
