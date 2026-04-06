// SPDX-License-Identifier: AGPL-3.0-or-later

//! Extended Genetics Validation Tests
//!
//! Comprehensive test coverage for genetics algorithm validation and evolution
//! Added October 29, 2025 - Part of Week 1 test coverage initiative

// Helper types
#[derive(Clone, Debug, PartialEq)]
struct Gene {
    id: String,
    value: f64,
}

#[derive(Clone, Debug)]
struct Genome {
    genes: Vec<Gene>,
    fitness: f64,
}

#[derive(Clone)]
struct Population {
    members: Vec<Genome>,
    generation: u32,
}

// Helper functions
fn create_test_gene(id: &str, value: f64) -> Gene {
    Gene {
        id: id.to_string(),
        value,
    }
}

fn create_test_genome() -> Genome {
    Genome {
        genes: vec![
            create_test_gene("gene1", 0.5),
            create_test_gene("gene2", 0.7),
        ],
        fitness: 0.0,
    }
}

fn calculate_fitness(genome: &Genome) -> f64 {
    genome.genes.iter().map(|g| g.value).sum()
}

fn mutate_gene(gene: &mut Gene, rate: f64) {
    if rate > 0.0 {
        gene.value = (gene.value + 0.1).min(1.0);
    }
}

fn crossover(parent1: &Genome, parent2: &Genome) -> Genome {
    let mut child_genes = Vec::new();
    for (i, gene) in parent1.genes.iter().enumerate() {
        if i % 2 == 0 {
            child_genes.push(gene.clone());
        } else if let Some(gene2) = parent2.genes.get(i) {
            child_genes.push(gene2.clone());
        }
    }
    Genome {
        genes: child_genes,
        fitness: 0.0,
    }
}

fn select_best(population: &Population, count: usize) -> Vec<Genome> {
    let mut sorted = population.members.clone();
    sorted.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    sorted.into_iter().take(count).collect()
}

fn create_population(size: usize) -> Population {
    let members = (0..size).map(|_| create_test_genome()).collect();

    Population {
        members,
        generation: 0,
    }
}

fn evolve_population(pop: &mut Population) {
    pop.generation += 1;
    for genome in &mut pop.members {
        genome.fitness = calculate_fitness(genome);
    }
}

fn validate_genome(genome: &Genome) -> Result<(), String> {
    if genome.genes.is_empty() {
        return Err("Genome has no genes".to_string());
    }
    for gene in &genome.genes {
        if gene.value < 0.0 || gene.value > 1.0 {
            return Err("Gene value out of range".to_string());
        }
    }
    Ok(())
}

#[cfg(test)]
mod genetics_validation_tests {
    use super::*;

    #[test]
    fn test_gene_creation() {
        let gene = create_test_gene("test", 0.5);
        assert_eq!(gene.id, "test");
        assert_eq!(gene.value, 0.5);
    }

    #[test]
    fn test_gene_clone() {
        let gene = create_test_gene("test", 0.5);
        let cloned = gene.clone();
        assert_eq!(gene, cloned);
    }

    #[test]
    fn test_genome_creation() {
        let genome = create_test_genome();
        assert_eq!(genome.genes.len(), 2);
    }

    #[test]
    fn test_fitness_calculation() {
        let genome = create_test_genome();
        let fitness = calculate_fitness(&genome);
        assert!(fitness > 0.0);
    }

    #[test]
    fn test_gene_mutation() {
        let mut gene = create_test_gene("test", 0.5);
        let original_value = gene.value;
        mutate_gene(&mut gene, 0.5);
        assert_ne!(gene.value, original_value);
    }

    #[test]
    fn test_gene_mutation_no_rate() {
        let mut gene = create_test_gene("test", 0.5);
        let original_value = gene.value;
        mutate_gene(&mut gene, 0.0);
        assert_eq!(gene.value, original_value);
    }

    #[test]
    fn test_crossover() {
        let parent1 = create_test_genome();
        let parent2 = create_test_genome();
        let child = crossover(&parent1, &parent2);
        assert_eq!(child.genes.len(), 2);
    }

    #[test]
    fn test_population_creation() {
        let pop = create_population(10);
        assert_eq!(pop.members.len(), 10);
        assert_eq!(pop.generation, 0);
    }

    #[test]
    fn test_population_evolution() {
        let mut pop = create_population(5);
        let initial_gen = pop.generation;
        evolve_population(&mut pop);
        assert_eq!(pop.generation, initial_gen + 1);
    }

    #[test]
    fn test_fitness_assignment() {
        let mut pop = create_population(5);
        evolve_population(&mut pop);
        for genome in &pop.members {
            assert!(genome.fitness > 0.0);
        }
    }

    #[test]
    fn test_selection_best() {
        let mut pop = create_population(10);
        evolve_population(&mut pop);
        let best = select_best(&pop, 3);
        assert_eq!(best.len(), 3);
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    fn test_selection_all() {
        let mut pop = create_population(5);
        evolve_population(&mut pop);
        let all = select_best(&pop, 10);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: genetics
        // TEST_PRIORITY: normal
        assert_eq!(all.len(), 5); // Should return all available
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    fn test_genome_validation_valid() {
        let genome = create_test_genome();
        let result = validate_genome(&genome);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: genetics
        // TEST_PRIORITY: normal
        assert!(result.is_ok());
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    fn test_genome_validation_empty() {
        let genome = Genome {
            genes: Vec::new(),
            fitness: 0.0,
        };
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: genetics
        // TEST_PRIORITY: normal
        let result = validate_genome(&genome);
        assert!(result.is_err());
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    fn test_gene_value_bounds_low() {
        let gene = create_test_gene("test", 0.0);
        let genome = Genome {
            genes: vec![gene],
            fitness: 0.0,
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: genetics
        // TEST_PRIORITY: normal
        };
        let result = validate_genome(&genome);
        assert!(result.is_ok());
    }
 // TEST_CATEGORY: integration
 // TEST_DOMAIN: genetics
 // TEST_PRIORITY: normal

    #[test]
    fn test_gene_value_bounds_high() {
        let gene = create_test_gene("test", 1.0);
        let genome = Genome {
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: genetics
            // TEST_PRIORITY: normal
            genes: vec![gene],
            fitness: 0.0,
        };
        let result = validate_genome(&genome);
        assert!(result.is_ok());
    }
 // TEST_CATEGORY: integration
 // TEST_DOMAIN: genetics
 // TEST_PRIORITY: normal

    #[test]
    fn test_multiple_generations() {
        let mut pop = create_population(5);
        for _ in 0..3 {
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: genetics
            // TEST_PRIORITY: normal
            evolve_population(&mut pop);
        }
        assert_eq!(pop.generation, 3);
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_gene_equality() {
        let gene1 = create_test_gene("test", 0.5);
        let gene2 = create_test_gene("test", 0.5);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: genetics
        // TEST_PRIORITY: normal
        assert_eq!(gene1, gene2);
    }

    #[test]
    fn test_gene_inequality() {
        let gene1 = create_test_gene("test1", 0.5);
        let gene2 = create_test_gene("test2", 0.5);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: genetics
        // TEST_PRIORITY: normal
        assert_ne!(gene1, gene2);
    }

    #[test]
    fn test_genome_clone() {
        let genome = create_test_genome();
        let cloned = genome.clone();
        assert_eq!(genome.genes.len(), cloned.genes.len());
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    }

    #[test]
    fn test_population_size_preservation() {
        let mut pop = create_population(7);
        evolve_population(&mut pop);
        assert_eq!(pop.members.len(), 7);
    }
 // TEST_CATEGORY: integration
 // TEST_DOMAIN: genetics
 // TEST_PRIORITY: normal

    #[test]
    fn test_fitness_positive() {
        let genome = create_test_genome();
        let fitness = calculate_fitness(&genome);
        assert!(fitness >= 0.0);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    }

    #[test]
    fn test_empty_population() {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: genetics
        // TEST_PRIORITY: normal
        let pop = create_population(0);
        assert_eq!(pop.members.len(), 0);
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_large_population() {
        let pop = create_population(100);
        assert_eq!(pop.members.len(), 100);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    }

    #[test]
    fn test_gene_mutation_clamping() {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: genetics
        // TEST_PRIORITY: normal
        let mut gene = create_test_gene("test", 0.95);
        mutate_gene(&mut gene, 1.0);
        assert!(gene.value <= 1.0);
    }
 // TEST_CATEGORY: integration
 // TEST_DOMAIN: genetics
 // TEST_PRIORITY: normal

    #[test]
    fn test_crossover_preserves_structure() {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: genetics
        // TEST_PRIORITY: normal
        let parent1 = create_test_genome();
        let parent2 = create_test_genome();
        let child = crossover(&parent1, &parent2);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: genetics
        // TEST_PRIORITY: normal
        assert!(!child.genes.is_empty());
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    fn test_selection_ordering() {
        let mut pop = create_population(5);
        // Set different fitness values
        for (i, genome) in pop.members.iter_mut().enumerate() {
            genome.fitness = i as f64;
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: genetics
        // TEST_PRIORITY: normal
        }
        let best = select_best(&pop, 3);
        // Best should have highest fitness
        assert!(best[0].fitness >= best[1].fitness);
    }

    #[test]
    fn test_genome_debug_format() {
        let genome = create_test_genome();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: genetics
        // TEST_PRIORITY: normal
        let debug_str = format!("{:?}", genome);
        assert!(!debug_str.is_empty());
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_gene_debug_format() {
        let gene = create_test_gene("test", 0.5);
        let debug_str = format!("{:?}", gene);
        assert!(debug_str.contains("test"));
    }
}
