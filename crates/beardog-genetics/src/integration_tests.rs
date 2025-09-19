use beardog_errors::BearDogError;

#[cfg(test)]
mod tests {

    use crate::genetics::spawning::engine::GeneticSpawningEngine;
    use beardog_auth::auth::{BearDogGenetics, NodeCapability, SecurityTraits};
    use beardog_traits::prelude::*;
    use std::collections::HashMap;

    #[tokio::test]
    fn test_unified_genetic_traits() -> Result<(), Box<dyn std::error::Error>> {

        let mut engine = GeneticSpawningEngine::new();

        assert_eq!(engine.id(), "genetic-spawning-engine");

        let version = engine.version();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 0);
        assert_eq!(version.patch, 0);

        let compatible_version = Version::new(1, 1, 0);
        let incompatible_version = Version::new(2, 0, 0);
        assert!(engine.is_compatible_with(&compatible_version));
        assert!(!engine.is_compatible_with(&incompatible_version));

        let mut config = engine.get_config().clone();
        config.mutation_rate = 0.2;
        config.fitness_threshold = 0.8;
        engine.configure(config.clone())?;
        assert_eq!(engine.get_config().mutation_rate, 0.2);
        assert_eq!(engine.get_config().fitness_threshold, 0.8);

        let mut invalid_config = config.clone();
        invalid_config.mutation_rate = 1.5; // Invalid
        assert!(GeneticSpawningEngine::validate_config(&invalid_config).is_err());
        Ok(())
    }


    fn test_genetic_evolution_traits() -> Result<(), Box<dyn std::error::Error>> {
        let engine = GeneticSpawningEngine::new();

        let parent1 = create_test_genetics("parent1", 0.7);
        let parent2 = create_test_genetics("parent2", 0.8);

        let fitness1 = engine.calculate_fitness(&parent1)?;
        let fitness2 = engine.calculate_fitness(&parent2)?;
        assert!((0.0..=1.0).contains(&fitness1));
        assert!((0.0..=1.0).contains(&fitness2));

        let compatibility = engine.check_compatibility(&parent1, &parent2)?;
        assert!((0.0..=1.0).contains(&compatibility));

        let mutated = engine.mutate(&parent1, 0.5)?;
        assert_ne!(mutated.id, parent1.id); // Should have different characteristics

        let offspring = engine.crossover(&parent1, &parent2)?;
        assert_eq!(offspring.len(), 2);

        let child1 = &offspring[0];
        let child2 = &offspring[1];
        assert!(child1.id.contains("offspring"));
        assert!(child2.id.contains("offspring"));

        let mut population = vec![parent1, parent2, mutated];
        population.extend(offspring);
        let selected = engine.select(&population, 2);
        assert_eq!(selected.len(), 2);

        if selected.len() >= 2 {
            assert!(selected[0].fitness_score >= selected[1].fitness_score);
        }


    fn test_zero_copy_patterns() -> Result<(), Box<dyn std::error::Error>> {

        let view = engine.as_view();
        assert!(engine.is_owned());

        match view {
            std::borrow::Cow::Borrowed(_) => {
                println!("Zero-copy: Using borrowed data");
            }
            std::borrow::Cow::Owned(_) => {
                println!("Zero-copy: Using owned data (fallback)");


    fn test_performance_monitoring() -> Result<(), Box<dyn std::error::Error>> {

        let mut metrics: HashMap<String, f64> = HashMap::with_capacity(16);

        let start = std::time::Instant::now();
        let genetics = create_test_genetics("perf_test", 0.6);
        let _fitness = engine.calculate_fitness(&genetics)?;
        let duration = start.elapsed().as_millis() as f64;
        metrics.insert("fitness_calculation_ms".to_string(), duration);

        assert!(metrics.contains_key("fitness_calculation_ms"));
        assert!(metrics["fitness_calculation_ms"] >= 0.0);
        println!("Performance metrics: {metrics:?}");


    fn test_unified_error_handling() -> Result<(), Box<dyn std::error::Error>> {
        use beardog_traits::error::{BearDogError as TraitsError, ErrorContext, RecoverableError};

        let validation_error = TraitsError::Validation {
            message: "Test validation error".to_string(),
        };

        let result: Result<(), TraitsError> = Err(validation_error.clone());
        let with_context = result.context("During genetic validation");
        assert!(with_context.is_err());

        assert!(!validation_error.is_recoverable());
        assert_eq!(validation_error.retry_delay_ms(), None);

        let network_error = TraitsError::Network {
            message: "Connection timeout".to_string(),
        assert!(network_error.is_recoverable());
        assert_eq!(network_error.retry_delay_ms(), Some(500));


    fn test_genetic_lineage() -> Result<(), Box<dyn std::error::Error>> {

        let ancestor = create_test_genetics("ancestor", 0.5);
        let parent1 = create_test_genetics("parent1", 0.6);
        let parent2 = create_test_genetics("parent2", 0.7);

        let generation1 = engine.mutate(&ancestor, 0.1)?;
        let generation2 = engine.crossover(&parent1, &parent2)?;

        let mut population = vec![ancestor, parent1, parent2, generation1];
        population.extend(generation2);
        let survivors = engine.select(&population, 3);
        assert_eq!(survivors.len(), 3);

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
        println!("  Population size: {}", survivors.len(&str, base_fitness: f64) -> BearDogGenetics {
        BearDogGenetics {
            id: id.to_string() -> Result<(), Box<dyn std::error::Error>> {

        let population_size = 100;
        let mut population = Vec::with_capacity(population_size);

        let generation_start = std::time::Instant::now();
        for i in 0..population_size {
            let genetics =
                create_test_genetics(&format!("individual_{i}"), rand::random::<f64>());
            population.push(genetics);
        let generation_time = generation_start.elapsed();

        let fitness_start = std::time::Instant::now();
        let mut fitness_scores = Vec::with_capacity(population_size);
        for genetics in &population {
            let fitness = engine.calculate_fitness(genetics)?;
            fitness_scores.push(fitness);
        let fitness_time = fitness_start.elapsed();

        let selection_start = std::time::Instant::now();
        let selected = engine.select(&population, population_size / 2);
        let selection_time = selection_start.elapsed();

        println!("Performance Benchmarks:");
        println!("  Population size: {population_size}");
        println!("  Generation time: {generation_time:?}");
        println!("  Fitness calculation time: {fitness_time:?}");
        println!("  Selection time: {selection_time:?}");
        println!(
            "  Avg fitness calc per individual: {:?}",
            fitness_time / population_size as u32
        );

        assert_eq!(fitness_scores.len(), population_size);
        assert_eq!(selected.len(), population_size / 2);

        assert!(
            fitness_time.as_millis() < 1000,
            "Fitness calculation too slow"
        assert!(selection_time.as_millis() < 100, "Selection too slow");
}
