// SPDX-License-Identifier: AGPL-3.0-only

//! # Advanced Genetic Algorithms
//!
//! This module provides advanced genetic algorithm implementations for
//! evolutionary optimization in the BearDog ecosystem.

mod engines;
mod evolution_engine;
mod metrics;
mod types;

pub use engines::{
    CrossoverEngine, FitnessEvaluator, MutationEngine, PopulationManager, SelectionEngine,
};
pub use evolution_engine::GeneticEvolutionEngine;
pub use metrics::{DiversityMetrics, EvolutionMetrics};
pub use types::{
    EvolutionConfig, FitnessCriterion, GenerationSnapshot, GeneticIndividual, GeneticSignature,
    MutationRecord, MutationType, OptimizationConstraint, OptimizationGoal, PerformanceMetrics,
};
