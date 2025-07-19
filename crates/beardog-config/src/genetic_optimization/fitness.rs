//! Fitness Evaluation for Genetic Algorithms
//!
//! This module handles fitness evaluation configurations.

use serde::{Deserialize, Serialize};

/// Fitness evaluation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitnessEvaluationConfig {
    /// Fitness evaluation method
    pub evaluation_method: FitnessEvaluationMethod,
    /// Fitness scaling enabled
    pub fitness_scaling: bool,
    /// Fitness sharing enabled
    pub fitness_sharing: bool,
    /// Fitness caching enabled
    pub fitness_caching: bool,
    /// Penalty-based evaluation enabled
    pub penalty_based: bool,
    /// Surrogate modeling enabled
    pub surrogate_modeling: bool,
}

/// Fitness evaluation method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FitnessEvaluationMethod {
    /// Direct evaluation
    Direct,
    /// Surrogate-based evaluation
    Surrogate,
    /// Hybrid evaluation
    Hybrid,
    /// Approximate evaluation
    Approximate,
}

impl Default for FitnessEvaluationConfig {
    fn default() -> Self {
        Self {
            evaluation_method: FitnessEvaluationMethod::Direct,
            fitness_scaling: false,
            fitness_sharing: false,
            fitness_caching: true,
            penalty_based: false,
            surrogate_modeling: false,
        }
    }
}

impl FitnessEvaluationConfig {
    /// Create production fitness evaluation configuration
    pub fn production() -> Self {
        Self {
            evaluation_method: FitnessEvaluationMethod::Hybrid,
            fitness_scaling: true,
            fitness_sharing: true,
            fitness_caching: true,
            penalty_based: true,
            surrogate_modeling: true,
        }
    }

    /// Create development fitness evaluation configuration
    pub fn development() -> Self {
        Self {
            evaluation_method: FitnessEvaluationMethod::Direct,
            fitness_scaling: false,
            fitness_sharing: false,
            fitness_caching: false,
            penalty_based: false,
            surrogate_modeling: false,
        }
    }
}
