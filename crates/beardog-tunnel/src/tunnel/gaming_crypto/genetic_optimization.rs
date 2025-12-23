

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
    /// The mutation rate value
    pub mutation_rate: f64,

    /// The crossover rate value
    pub crossover_rate: f64,

    /// Number of generations
    pub generations: usize,

    /// The elite percentage value
    pub elite_percentage: f64,
}

pub struct EvolutionResult {
    /// The best fitness value
    pub best_fitness: f64,
    /// Number of generations_evolved
    pub generations_evolved: usize,
    /// The improvement percentage value
    pub improvement_percentage: f64,
    /// Collection of optimized params
    pub optimized_params: Vec<u8>,
}

impl Default for GeneticParameters {
    fn default(100,
            mutation_rate: 0.05,
            crossover_rate: 0.8,
            generations: 50,
            elite_percentage: 0.1,
        }
    }
}

impl Default for EvolutionResult {
    fn default(0.85,
            generations_evolved: 50,
            improvement_percentage: 15.0,
            optimized_params: vec![],
        }
    }
}
