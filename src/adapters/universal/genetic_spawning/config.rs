//! Genetic spawning configuration and statistics
//!
//! This module contains configuration types and statistics tracking
//! for the genetic spawning system.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use super::genetics::HybridCapability;

/// Genetic algorithm configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticAlgorithmConfig {
    /// Mutation rate for genetic algorithm (0.0 to 1.0)
    pub mutation_rate: f64,
    /// Crossover rate for genetic recombination (0.0 to 1.0)
    pub crossover_rate: f64,
    /// Selection pressure for parent selection
    pub selection_pressure: f64,
    /// Maximum number of generations to evolve
    pub max_generations: u32,
    /// Size of the population in each generation
    pub population_size: u32,
    /// Fitness threshold for acceptable solutions
    pub fitness_threshold: f64,
    /// Minimum diversity requirement to prevent convergence
    pub diversity_requirement: f64,
    /// Tolerance for convergence detection
    pub convergence_tolerance: f64,
}

impl Default for GeneticAlgorithmConfig {
    fn default() -> Self {
        Self {
            mutation_rate: 0.05,
            crossover_rate: 0.8,
            selection_pressure: 1.5,
            max_generations: 100,
            population_size: 50,
            fitness_threshold: 0.9,
            diversity_requirement: 0.3,
            convergence_tolerance: 0.01,
        }
    }
}

/// Spawning statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpawningStatistics {
    /// Total number of spawning operations attempted
    pub total_spawns: u32,
    /// Number of successful spawning operations
    pub successful_spawns: u32,
    /// Number of failed spawning operations
    pub failed_spawns: u32,
    /// Number of currently active spawning operations
    pub active_spawns: u32,
    /// Total number of hybrid nodes created
    pub total_hybrid_nodes: u32,
    /// Average time taken for spawning operations (milliseconds)
    pub average_spawn_time_ms: u64,
    /// Most commonly requested hybrid capabilities
    pub most_common_hybrid_capabilities: Vec<(HybridCapability, u32)>,
    /// Statistics on ecosystem combinations
    pub ecosystem_combination_stats: HashMap<String, u32>,
    /// Last time statistics were updated
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl SpawningStatistics {
    /// Create new spawning statistics
    pub fn new() -> Self {
        Self {
            last_updated: chrono::Utc::now(),
            ..Default::default()
        }
    }

    /// Calculate success rate as percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_spawns == 0 {
            0.0
        } else {
            (self.successful_spawns as f64 / self.total_spawns as f64) * 100.0
        }
    }

    /// Calculate failure rate as percentage
    pub fn failure_rate(&self) -> f64 {
        if self.total_spawns == 0 {
            0.0
        } else {
            (self.failed_spawns as f64 / self.total_spawns as f64) * 100.0
        }
    }

    /// Update statistics after a spawning operation
    pub fn update_spawn_result(&mut self, success: bool, spawn_time_ms: u64) {
        self.total_spawns += 1;
        
        if success {
            self.successful_spawns += 1;
            self.total_hybrid_nodes += 1;
        } else {
            self.failed_spawns += 1;
        }
        
        // Update average spawn time
        if self.total_spawns == 1 {
            self.average_spawn_time_ms = spawn_time_ms;
        } else {
            self.average_spawn_time_ms = (self.average_spawn_time_ms * (self.total_spawns - 1) as u64 + spawn_time_ms) / self.total_spawns as u64;
        }
        
        self.last_updated = chrono::Utc::now();
    }

    /// Record usage of a hybrid capability
    pub fn record_capability_usage(&mut self, capability: HybridCapability) {
        // Find existing entry or create new one
        if let Some(entry) = self.most_common_hybrid_capabilities.iter_mut().find(|(cap, _)| cap == &capability) {
            entry.1 += 1;
        } else {
            self.most_common_hybrid_capabilities.push((capability, 1));
        }
        
        // Sort by usage count (descending)
        self.most_common_hybrid_capabilities.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Keep only top 10
        self.most_common_hybrid_capabilities.truncate(10);
    }

    /// Record usage of an ecosystem combination
    pub fn record_ecosystem_combination(&mut self, ecosystem_ids: Vec<String>) {
        let combination_key = ecosystem_ids.join("+");
        *self.ecosystem_combination_stats.entry(combination_key).or_insert(0) += 1;
    }

    /// Get most popular ecosystem combinations
    pub fn get_popular_combinations(&self, limit: usize) -> Vec<(String, u32)> {
        let mut combinations: Vec<_> = self.ecosystem_combination_stats.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        
        combinations.sort_by(|a, b| b.1.cmp(&a.1));
        combinations.truncate(limit);
        combinations
    }

    /// Check if statistics are healthy (success rate > 70%)
    pub fn is_healthy(&self) -> bool {
        self.success_rate() > 70.0
    }
}

impl GeneticAlgorithmConfig {
    /// Create a configuration optimized for speed
    pub fn fast() -> Self {
        Self {
            mutation_rate: 0.1,
            crossover_rate: 0.9,
            selection_pressure: 2.0,
            max_generations: 50,
            population_size: 30,
            fitness_threshold: 0.8,
            diversity_requirement: 0.2,
            convergence_tolerance: 0.05,
        }
    }

    /// Create a configuration optimized for quality
    pub fn high_quality() -> Self {
        Self {
            mutation_rate: 0.03,
            crossover_rate: 0.7,
            selection_pressure: 1.2,
            max_generations: 200,
            population_size: 100,
            fitness_threshold: 0.95,
            diversity_requirement: 0.4,
            convergence_tolerance: 0.005,
        }
    }

    /// Create a configuration optimized for diversity
    pub fn diverse() -> Self {
        Self {
            mutation_rate: 0.08,
            crossover_rate: 0.6,
            selection_pressure: 1.0,
            max_generations: 150,
            population_size: 80,
            fitness_threshold: 0.85,
            diversity_requirement: 0.5,
            convergence_tolerance: 0.02,
        }
    }

    /// Validate configuration parameters
    pub fn validate(&self) -> Result<(), String> {
        if self.mutation_rate < 0.0 || self.mutation_rate > 1.0 {
            return Err("Mutation rate must be between 0.0 and 1.0".to_string());
        }
        
        if self.crossover_rate < 0.0 || self.crossover_rate > 1.0 {
            return Err("Crossover rate must be between 0.0 and 1.0".to_string());
        }
        
        if self.selection_pressure <= 0.0 {
            return Err("Selection pressure must be positive".to_string());
        }
        
        if self.max_generations == 0 {
            return Err("Max generations must be positive".to_string());
        }
        
        if self.population_size == 0 {
            return Err("Population size must be positive".to_string());
        }
        
        if self.fitness_threshold < 0.0 || self.fitness_threshold > 1.0 {
            return Err("Fitness threshold must be between 0.0 and 1.0".to_string());
        }
        
        if self.diversity_requirement < 0.0 || self.diversity_requirement > 1.0 {
            return Err("Diversity requirement must be between 0.0 and 1.0".to_string());
        }
        
        if self.convergence_tolerance <= 0.0 {
            return Err("Convergence tolerance must be positive".to_string());
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genetic_algorithm_config_default() {
        let config = GeneticAlgorithmConfig::default();
        assert_eq!(config.mutation_rate, 0.05);
        assert_eq!(config.crossover_rate, 0.8);
        assert_eq!(config.max_generations, 100);
        assert_eq!(config.population_size, 50);
    }

    #[test]
    fn test_genetic_algorithm_config_validation() {
        let mut config = GeneticAlgorithmConfig::default();
        assert!(config.validate().is_ok());

        config.mutation_rate = 1.5;
        assert!(config.validate().is_err());

        config.mutation_rate = 0.05;
        config.max_generations = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_spawning_statistics_success_rate() {
        let mut stats = SpawningStatistics::new();
        assert_eq!(stats.success_rate(), 0.0);

        stats.update_spawn_result(true, 1000);
        assert_eq!(stats.success_rate(), 100.0);

        stats.update_spawn_result(false, 500);
        assert_eq!(stats.success_rate(), 50.0);
    }

    #[test]
    fn test_spawning_statistics_average_time() {
        let mut stats = SpawningStatistics::new();
        
        stats.update_spawn_result(true, 1000);
        assert_eq!(stats.average_spawn_time_ms, 1000);

        stats.update_spawn_result(true, 2000);
        assert_eq!(stats.average_spawn_time_ms, 1500);
    }

    #[test]
    fn test_spawning_statistics_health() {
        let mut stats = SpawningStatistics::new();
        
        // Add successful spawns
        for _ in 0..8 {
            stats.update_spawn_result(true, 1000);
        }
        
        // Add failed spawns
        for _ in 0..2 {
            stats.update_spawn_result(false, 500);
        }
        
        assert!(stats.is_healthy()); // 80% success rate
    }
} 