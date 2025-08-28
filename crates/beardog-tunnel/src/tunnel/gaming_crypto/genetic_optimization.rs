

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticParameters {

    pub population_size: usize,

    pub mutation_rate: f64,

    pub crossover_rate: f64,

    pub generations: usize,

    pub elite_percentage: f64,
}

pub struct EvolutionResult {
    pub best_fitness: f64,
    pub generations_evolved: usize,
    pub improvement_percentage: f64,
    pub optimized_params: Vec<u8>,
}

impl Default for GeneticParameters {
    fn default() -> Self {
        Self {
            population_size: 100,
            mutation_rate: 0.05,
            crossover_rate: 0.8,
            generations: 50,
            elite_percentage: 0.1,
        }
    }
}

impl Default for EvolutionResult {
    fn default() -> Self {
        Self {
            best_fitness: 0.85,
            generations_evolved: 50,
            improvement_percentage: 15.0,
            optimized_params: vec![],
        }
    }
}
