// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Canonical Genetics Types
///
/// **UNIFIED GENETICS CONFIGURATION** ✅
/// This module provides the canonical genetics configuration that consolidates
/// all scattered GeneticsConfig definitions across the codebase into a single,
/// comprehensive configuration structure.
/// ## Consolidation Complete
/// **ELIMINATED DUPLICATE CONFIGS:**
/// - `beardog-genetics/src/genetics/api.rs::GeneticsConfig` (4 fields)
/// - `beardog-genetics/src/genetics/types.rs::GeneticsConfig` (6 fields) 
/// - `beardog-genetics/src/genetics/mod.rs::GeneticsConfig` (8 fields)
/// - `beardog-genetics/src/genetics/spawning/engine.rs::GeneticsConfig` (3 fields)
/// **UNIFIED INTO:** Single canonical configuration with all useful fields

use serde::{Deserialize, Serialize};
/// **CANONICAL GENETICS CONFIGURATION** - Single source of truth
/// Comprehensive genetics configuration that combines all useful fields from
/// the scattered GeneticsConfig definitions throughout the codebase.
/// # Population Management
/// - `max_population_size`: Maximum number of individuals in population
/// - `population_size`: Current/target population size
/// - `max_generations`: Maximum number of generations to evolve
/// # Evolution Parameters
/// - `mutation_rate`: Base probability of mutations (0.0-1.0)
/// - `crossover_rate`: Probability of crossover during reproduction (0.0-1.0)
/// - `selection_pressure`: Intensity of natural selection (0.0-1.0)
/// # Diversity and Quality Control
/// - `max_genetic_diversity`: Maximum allowed genetic drift (0.0-1.0)
/// - `diversity_threshold`: Minimum diversity to maintain (0.0-1.0)
/// - `min_security_threshold`: Minimum security level for offspring (0.0-1.0)
/// - `fitness_threshold`: Minimum fitness required for survival (0.0-1.0)
/// # Inheritance and Blending
/// - `capability_inheritance_weight`: Weight for inheriting vs. generating capabilities (0.0-1.0)
/// - `trait_blending_factor`: Factor for blending vs. dominant inheritance (0.0-1.0)
/// # Advanced Features
/// - `enable_directed_evolution`: Allow purpose-specific genetic optimization
/// - `enable_adaptive_mutations`: Enable mutation rate adaptation
/// - `parallel_processing`: Enable parallel genetic operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsConfig {
    // Population Management
    /// Maximum number of individuals in the population
    pub max_population_size: usize,
    /// Current/target population size for genetic operations
    pub population_size: usize,
    /// Maximum number of generations to evolve
    pub max_generations: u32,
    
    // Evolution Parameters
    /// Base probability of genetic mutations occurring during reproduction (0.0-1.0)
    pub mutation_rate: f64,
    /// Probability of crossover during reproduction (0.0-1.0)
    pub crossover_rate: f64,
    /// Intensity of natural selection pressure (0.0-1.0)
    pub selection_pressure: f64,
    // Diversity and Quality Control
    /// Maximum genetic diversity allowed to prevent excessive drift (0.0-1.0)
    pub max_genetic_diversity: f64,
    /// Minimum diversity threshold to maintain (0.0-1.0)
    pub diversity_threshold: f64,
    /// Minimum security threshold that all offspring must meet (0.0-1.0)
    pub min_security_threshold: f64,
    /// Minimum fitness required for survival (0.0-1.0)
    pub fitness_threshold: f64,
    // Inheritance and Blending
    /// Weight given to inheriting parent capabilities vs. generating new ones (0.0-1.0)
    pub capability_inheritance_weight: f64,
    /// Factor for blending parent traits vs. dominant inheritance patterns (0.0-1.0)
    pub trait_blending_factor: f64,
    // Advanced Features
    /// Enable directed evolution for specific spawn purposes
    pub enable_directed_evolution: bool,
    /// Enable adaptive mutation rates based on population fitness
    pub enable_adaptive_mutations: bool,
    /// Enable parallel processing for genetic operations
    pub parallel_processing: bool,
}
impl Default for GeneticsConfig {
    fn default() -> Self {
        Self {
            // Population Management - Conservative defaults
            max_population_size: 1000,
            population_size: 100,
            max_generations: 1000,
            
            // Evolution Parameters - Balanced for security and innovation
            mutation_rate: 0.05,      // Conservative for security
            crossover_rate: 0.8,      // High for diversity
            selection_pressure: 0.7,   // Moderate selection
            
            // Diversity and Quality Control - Security-first
            max_genetic_diversity: 0.8,        // Allow significant diversity
            diversity_threshold: 0.3,          // Maintain minimum diversity
            min_security_threshold: 0.7,       // High security requirement
            fitness_threshold: 0.5,            // Moderate fitness requirement
            
            // Inheritance and Blending - Favor proven traits
            capability_inheritance_weight: 0.8,  // Favor inheritance
            trait_blending_factor: 0.6,          // Moderate blending
            
            // Advanced Features - Enable modern capabilities
            enable_directed_evolution: true,
            enable_adaptive_mutations: true,
            parallel_processing: true,
        }
    }
}
impl GeneticsConfig {
    /// Create a configuration optimized for security-focused evolution
    pub fn security_focused() -> Self {
        Self {
            mutation_rate: 0.02,                    // Very conservative mutations
            min_security_threshold: 0.9,            // Very high security requirement
            capability_inheritance_weight: 0.9,     // Heavily favor proven capabilities
            enable_directed_evolution: true,        // Enable security-directed evolution
            ..Default::default()
        }
    }

    /// Create a configuration optimized for performance evolution
    pub fn performance_focused() -> Self {
        Self {
            mutation_rate: 0.1,                     // Higher mutation for innovation
            selection_pressure: 0.9,                // Strong performance selection
            fitness_threshold: 0.7,                 // High fitness requirement
            parallel_processing: true,              // Enable performance optimizations
            ..Default::default()
        }
    }

    /// Create a configuration optimized for rapid experimentation
    pub fn experimental() -> Self {
        Self {
            population_size: 50,                    // Smaller for speed
            max_generations: 100,                   // Fewer generations
            mutation_rate: 0.15,                    // High mutation for diversity
            min_security_threshold: 0.5,            // Lower security for experimentation
            ..Default::default()
        }
    }

    /// Validate configuration parameters
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
