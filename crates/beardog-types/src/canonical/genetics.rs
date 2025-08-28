use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsConfig {
    pub max_population_size: usize,

    pub population_size: usize,

    pub max_generations: u32,

    pub mutation_rate: f64,

    pub crossover_rate: f64,

    pub selection_pressure: f64,

    pub max_genetic_diversity: f64,

    pub diversity_threshold: f64,

    pub min_security_threshold: f64,

    pub fitness_threshold: f64,

    pub capability_inheritance_weight: f64,

    pub trait_blending_factor: f64,

    pub enable_directed_evolution: bool,

    pub enable_adaptive_mutations: bool,

    pub parallel_processing: bool,
}
impl Default for GeneticsConfig {
    fn default() -> Self {
        Self {
            max_population_size: 1000,
            population_size: 100,
            max_generations: 1000,

            mutation_rate: 0.05,     // Conservative for security
            crossover_rate: 0.8,     // High for diversity
            selection_pressure: 0.7, // Moderate selection

            max_genetic_diversity: 0.8,  // Allow significant diversity
            diversity_threshold: 0.3,    // Maintain minimum diversity
            min_security_threshold: 0.7, // High security requirement
            fitness_threshold: 0.5,      // Moderate fitness requirement

            capability_inheritance_weight: 0.8, // Favor inheritance
            trait_blending_factor: 0.6,         // Moderate blending

            enable_directed_evolution: true,
            enable_adaptive_mutations: true,
            parallel_processing: true,
        }
    }
}
impl GeneticsConfig {
    pub fn security_focused() -> Self {
        Self {
            mutation_rate: 0.02,                // Very conservative mutations
            min_security_threshold: 0.9,        // Very high security requirement
            capability_inheritance_weight: 0.9, // Heavily favor proven capabilities
            enable_directed_evolution: true,    // Enable security-directed evolution
            ..Default::default()
        }
    }

    pub fn performance_focused() -> Self {
        Self {
            mutation_rate: 0.1,        // Higher mutation for innovation
            selection_pressure: 0.9,   // Strong performance selection
            fitness_threshold: 0.7,    // High fitness requirement
            parallel_processing: true, // Enable performance optimizations
            ..Default::default()
        }
    }

    pub fn experimental() -> Self {
        Self {
            population_size: 50,         // Smaller for speed
            max_generations: 100,        // Fewer generations
            mutation_rate: 0.15,         // High mutation for diversity
            min_security_threshold: 0.5, // Lower security for experimentation
            ..Default::default()
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.mutation_rate < 0.0 || self.mutation_rate > 1.0 {
            return Err("mutation_rate must be between 0.0 and 1.0".to_string());
        }
        if self.crossover_rate < 0.0 || self.crossover_rate > 1.0 {
            return Err("crossover_rate must be between 0.0 and 1.0".to_string());
        }
        if self.population_size == 0 {
            return Err("population_size must be greater than 0".to_string());
        }
        if self.max_population_size < self.population_size {
            return Err("max_population_size must be >= population_size".to_string());
        }
        Ok(())
    }
}
