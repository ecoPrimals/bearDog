//! Population Management for Genetic Algorithms
//!
//! This module handles population management configurations.

use serde::{Deserialize, Serialize};

/// Population management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationManagementConfig {
    /// Initial population size
    pub initial_population_size: u32,
    /// Maximum population size
    pub max_population_size: u32,
    /// Population initialization strategy
    pub initialization_strategy: PopulationInitializationStrategy,
    /// Population replacement strategy
    pub replacement_strategy: PopulationReplacementStrategy,
    /// Diversity maintenance enabled
    pub diversity_maintenance: bool,
    /// Population scaling enabled
    pub population_scaling: bool,
}

/// Population initialization strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PopulationInitializationStrategy {
    /// Random initialization
    Random,
    /// Uniform initialization
    Uniform,
    /// Heuristic initialization
    Heuristic,
    /// Seeded initialization
    Seeded,
}

/// Population replacement strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PopulationReplacementStrategy {
    /// Generational replacement
    Generational,
    /// Steady-state replacement
    SteadyState,
    /// Elitist replacement
    Elitist,
    /// Tournament replacement
    Tournament,
}

impl Default for PopulationManagementConfig {
    fn default() -> Self {
        Self {
            initial_population_size: 100,
            max_population_size: 1000,
            initialization_strategy: PopulationInitializationStrategy::Random,
            replacement_strategy: PopulationReplacementStrategy::Generational,
            diversity_maintenance: true,
            population_scaling: false,
        }
    }
}

impl PopulationManagementConfig {
    /// Create production population management configuration
    pub fn production() -> Self {
        Self {
            initial_population_size: 500,
            max_population_size: 5000,
            initialization_strategy: PopulationInitializationStrategy::Heuristic,
            replacement_strategy: PopulationReplacementStrategy::Elitist,
            diversity_maintenance: true,
            population_scaling: true,
        }
    }

    /// Create development population management configuration
    pub fn development() -> Self {
        Self {
            initial_population_size: 50,
            max_population_size: 100,
            initialization_strategy: PopulationInitializationStrategy::Random,
            replacement_strategy: PopulationReplacementStrategy::Generational,
            diversity_maintenance: false,
            population_scaling: false,
        }
    }
}
