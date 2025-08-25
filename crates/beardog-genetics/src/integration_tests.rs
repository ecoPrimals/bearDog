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


/// Integration tests for unified trait system
///
/// Demonstrates the power of our trait-based architecture

#[cfg(test)]
mod tests {
    // Removed unused import
    use crate::genetics::spawning::engine::GeneticSpawningEngine;
    use beardog_auth::auth::{BearDogGenetics, NodeCapability, SecurityTraits};
    use beardog_traits::prelude::*;
    use std::collections::HashMap;
    /// Test trait-based genetic operations
    #[tokio::test]
    async fn test_unified_genetic_traits() -> Result<(), Box<dyn std::error::Error>> {
        // Create engine using unified traits
        let mut engine = GeneticSpawningEngine::new();
        // Test Identifiable trait
        assert_eq!(engine.id(), "genetic-spawning-engine");
        // Test Versioned trait
        let version = engine.version();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 0);
        assert_eq!(version.patch, 0);
        // Test version compatibility
        let compatible_version = Version::new(1, 1, 0);
        let incompatible_version = Version::new(2, 0, 0);
        assert!(engine.is_compatible_with(&compatible_version));
        assert!(!engine.is_compatible_with(&incompatible_version));
        // Test Configurable trait
        let mut config = engine.get_config().clone();
        config.mutation_rate = 0.2;
        config.fitness_threshold = 0.8;
        engine.configure(config.clone())?;
        assert_eq!(engine.get_config().mutation_rate, 0.2);
        assert_eq!(engine.get_config().fitness_threshold, 0.8);
        // Test configuration validation
        let mut invalid_config = config.clone();
        invalid_config.mutation_rate = 1.5; // Invalid
        assert!(GeneticSpawningEngine::validate_config(&invalid_config).is_err());
        Ok(())
    }
    /// Test genetic spawning and evolution traits
    async fn test_genetic_evolution_traits() -> Result<(), Box<dyn std::error::Error>> {
        let engine = GeneticSpawningEngine::new();
        // Create test genetics
        let parent1 = create_test_genetics("parent1", 0.7);
        let parent2 = create_test_genetics("parent2", 0.8);
        // Test GeneticSpawner trait - fitness calculation
        let fitness1 = engine.calculate_fitness(&parent1).await?;
        let fitness2 = engine.calculate_fitness(&parent2).await?;
        assert!((0.0..=1.0).contains(&fitness1));
        assert!((0.0..=1.0).contains(&fitness2));
        // Test compatibility checking
        let compatibility = engine.check_compatibility(&parent1, &parent2).await?;
        assert!((0.0..=1.0).contains(&compatibility));
        // Test GeneticEvolution trait - mutation
        let mutated = engine.mutate(&parent1, 0.5).await?;
        assert_ne!(mutated.id, parent1.id); // Should have different characteristics
        // Test crossover
        let offspring = engine.crossover(&parent1, &parent2).await?;
        assert_eq!(offspring.len(), 2);
        // Offspring should have mixed traits
        let child1 = &offspring[0];
        let child2 = &offspring[1];
        assert!(child1.id.contains("offspring"));
        assert!(child2.id.contains("offspring"));
        // Test selection
        let mut population = vec![parent1, parent2, mutated];
        population.extend(offspring);
        let selected = engine.select(&population, 2);
        assert_eq!(selected.len(), 2);
        // Selected should be sorted by fitness (highest first)
        if selected.len() >= 2 {
            assert!(selected[0].fitness_score >= selected[1].fitness_score);
        }
    /// Test zero-copy optimizations
    async fn test_zero_copy_patterns() -> Result<(), Box<dyn std::error::Error>> {
        // Test ZeroCopy trait
        let view = engine.as_view();
        assert!(engine.is_owned());
        // In a real implementation, this would demonstrate actual zero-copy benefits
        // For now, we're testing the trait interface
        match view {
            std::borrow::Cow::Borrowed(_) => {
                println!("Zero-copy: Using borrowed data");
            }
            std::borrow::Cow::Owned(_) => {
                println!("Zero-copy: Using owned data (fallback)");
    /// Test performance monitoring integration
    async fn test_performance_monitoring() -> Result<(), Box<dyn std::error::Error>> {
        // Create a simple performance monitor
        let mut metrics: HashMap<String, f64> = HashMap::new();
        // Simulate performance tracking
        let start = std::time::Instant::now();
        let genetics = create_test_genetics("perf_test", 0.6);
        let _fitness = engine.calculate_fitness(&genetics).await?;
        let duration = start.elapsed().as_millis() as f64;
        metrics.insert("fitness_calculation_ms".to_string(), duration);
        // Verify we captured metrics
        assert!(metrics.contains_key("fitness_calculation_ms"));
        assert!(metrics["fitness_calculation_ms"] >= 0.0);
        println!("Performance metrics: {metrics:?}");
    /// Test error handling and recovery
    async fn test_unified_error_handling() -> Result<(), Box<dyn std::error::Error>> {
        use beardog_traits::error::{BearDogError as TraitsError, ErrorContext, RecoverableError};
        // Test error conversion and context
        let validation_error = TraitsError::Validation {
            message: "Test validation error".to_string(),
        };
        // Test error context trait
        let result: Result<(), TraitsError> = Err(validation_error.clone());
        let with_context = result.context("During genetic validation");
        assert!(with_context.is_err());
        // Test recoverable error trait
        assert!(!validation_error.is_recoverable());
        assert_eq!(validation_error.retry_delay_ms(), None);
        // Test recoverable error
        let network_error = TraitsError::Network {
            message: "Connection timeout".to_string(),
        assert!(network_error.is_recoverable());
        assert_eq!(network_error.retry_delay_ms(), Some(500));
    /// Test genetic lineage tracking
    async fn test_genetic_lineage() -> Result<(), Box<dyn std::error::Error>> {
        // Create a lineage of genetics
        let ancestor = create_test_genetics("ancestor", 0.5);
        let parent1 = create_test_genetics("parent1", 0.6);
        let parent2 = create_test_genetics("parent2", 0.7);
        // Simulate evolution over generations
        let generation1 = engine.mutate(&ancestor, 0.1).await?;
        let generation2 = engine.crossover(&parent1, &parent2).await?;
        // Test selection pressure
        let mut population = vec![ancestor, parent1, parent2, generation1];
        population.extend(generation2);
        let survivors = engine.select(&population, 3);
        assert_eq!(survivors.len(), 3);
        // Calculate diversity (simplified)
        let avg_fitness: f64 =
            survivors.iter().map(|g| g.fitness_score).sum::<f64>() / survivors.len() as f64;
        let fitness_variance: f64 = survivors
            .iter()
            .map(|g| (g.fitness_score - avg_fitness).powi(2))
            .sum::<f64>()
            / survivors.len() as f64;
        println!("Population diversity metrics:");
        println!("  Average fitness: {avg_fitness:.3}");
        println!("  Fitness variance: {fitness_variance:.3}");
        println!("  Population size: {}", survivors.len());
        // Higher variance indicates more genetic diversity
        assert!(fitness_variance >= 0.0);
    /// Helper function to create test genetics
    fn create_test_genetics(id: &str, base_fitness: f64) -> BearDogGenetics {
        BearDogGenetics {
            id: id.to_string(),
            crypto_chromosomes: vec![],
            security_traits: SecurityTraits {
                trust_threshold: base_fitness,
                paranoia_level: (base_fitness * 10.0) as u8,
                consensus_requirement: base_fitness > 0.5,
                isolation_preference: base_fitness * 0.8,
                audit_frequency: (base_fitness * 24.0) as u32,
            },
            capabilities: vec![
                NodeCapability::StorageProvider,
                NodeCapability::ComputeProvider,
            ],
            spawn_restrictions: vec![],
            fitness_score: base_fitness,
            generation: 0,
            parent_genetics: None,
            mutations: vec![],
            specializations: vec![beardog_auth::auth::NodeSpecialization::GeneralPurpose],
            security_clearance: beardog_auth::auth::SecurityClearance::Basic,
    /// Benchmark test for performance validation
    async fn test_performance_benchmarks() -> Result<(), Box<dyn std::error::Error>> {
        // Create a population for benchmarking
        let population_size = 100;
        let mut population = Vec::with_capacity(population_size);
        // Generate population
        let generation_start = std::time::Instant::now();
        for i in 0..population_size {
            let genetics =
                create_test_genetics(&format!("individual_{i}"), rand::random::<f64>());
            population.push(genetics);
        let generation_time = generation_start.elapsed();
        // Benchmark fitness calculation
        let fitness_start = std::time::Instant::now();
        let mut fitness_scores = Vec::with_capacity(population_size);
        for genetics in &population {
            let fitness = engine.calculate_fitness(genetics).await?;
            fitness_scores.push(fitness);
        let fitness_time = fitness_start.elapsed();
        // Benchmark selection
        let selection_start = std::time::Instant::now();
        let selected = engine.select(&population, population_size / 2);
        let selection_time = selection_start.elapsed();
        // Print performance metrics
        println!("Performance Benchmarks:");
        println!("  Population size: {population_size}");
        println!("  Generation time: {generation_time:?}");
        println!("  Fitness calculation time: {fitness_time:?}");
        println!("  Selection time: {selection_time:?}");
        println!(
            "  Avg fitness calc per individual: {:?}",
            fitness_time / population_size as u32
        );
        // Verify results
        assert_eq!(fitness_scores.len(), population_size);
        assert_eq!(selected.len(), population_size / 2);
        // Performance assertions (these would be tuned based on requirements)
        assert!(
            fitness_time.as_millis() < 1000,
            "Fitness calculation too slow"
        assert!(selection_time.as_millis() < 100, "Selection too slow");
}
