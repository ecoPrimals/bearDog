//! Genetic Operators for Genetic Algorithms
//!
//! This module handles genetic operator configurations.

use serde::{Deserialize, Serialize};

/// Genetic operators configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticOperatorsConfig {
    /// Crossover probability
    pub crossover_probability: f64,
    /// Mutation probability
    pub mutation_probability: f64,
    /// Selection pressure
    pub selection_pressure: f64,
    /// Elitism enabled
    pub elitism: bool,
    /// Niching enabled
    pub niching: bool,
    /// Repair mechanisms enabled
    pub repair_mechanisms: bool,
}

impl Default for GeneticOperatorsConfig {
    fn default() -> Self {
        Self {
            crossover_probability: 0.8,
            mutation_probability: 0.1,
            selection_pressure: 1.0,
            elitism: true,
            niching: false,
            repair_mechanisms: false,
        }
    }
}

impl GeneticOperatorsConfig {
    /// Create production genetic operators configuration
    pub fn production() -> Self {
        Self {
            crossover_probability: 0.9,
            mutation_probability: 0.05,
            selection_pressure: 1.5,
            elitism: true,
            niching: true,
            repair_mechanisms: true,
        }
    }

    /// Create development genetic operators configuration
    pub fn development() -> Self {
        Self {
            crossover_probability: 0.7,
            mutation_probability: 0.2,
            selection_pressure: 0.8,
            elitism: false,
            niching: false,
            repair_mechanisms: false,
        }
    }
}
