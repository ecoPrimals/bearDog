// Canonical Genetics Configuration

use serde::{Deserialize, Serialize};

/// Canonical genetics algorithm configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalGeneticsConfig {
    /// Whether genetic algorithms are enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Size of the genetic algorithm population
    /// Number of `population_size`
    pub population_size: usize,
    /// The mutation rate value
    pub mutation_rate: f64,
    /// The crossover rate value
    pub crossover_rate: f64,
    /// Maximum number of generations to run
    /// Number of `max_generations`
    pub max_generations: u32,
}

pub type GeneticsConfig = CanonicalGeneticsConfig;
