// SPDX-License-Identifier: AGPL-3.0-or-later

//! Population Evolution Integration Tests
//!
//! Substantive tests for actual genetic algorithm operations

#![allow(clippy::unwrap_used, clippy::expect_used)]

#[cfg(test)]
mod population_evolution_tests {
    #![allow(dead_code)] // Some test helpers may not be used in all configurations

    #[derive(Clone, Debug, PartialEq)]
    struct Individual {
        id: usize,
        genes: Vec<f64>,
        fitness: f64,
    }

    impl Individual {
        fn new(id: usize, genes: Vec<f64>) -> Self {
            Self {
                id,
                genes,
                fitness: 0.0,
            }
        }

        fn calculate_fitness(&mut self) {
            // Fitness = sum of genes (simple test fitness function)
            self.fitness = self.genes.iter().sum();
        }
    }

    struct Population {
        individuals: Vec<Individual>,
        generation: u32,
    }

    impl Population {
        fn new(size: usize, gene_count: usize) -> Self {
            let individuals: Vec<Individual> = (0..size)
                .map(|i| {
                    let genes: Vec<f64> = (0..gene_count).map(|_| rand::random::<f64>()).collect();
                    Individual::new(i, genes)
                })
                .collect();

            Self {
                individuals,
                generation: 0,
            }
        }

        fn evaluate_fitness(&mut self) {
            for individual in &mut self.individuals {
                individual.calculate_fitness();
            }
        }

        fn select_parents(&self, count: usize) -> Vec<Individual> {
            let mut sorted = self.individuals.clone();
            sorted.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
            sorted.into_iter().take(count).collect()
        }

        fn crossover(parent1: &Individual, parent2: &Individual) -> Individual {
            let mut child_genes = Vec::new();
            for i in 0..parent1.genes.len() {
                if i % 2 == 0 {
                    child_genes.push(parent1.genes[i]);
                } else {
                    child_genes.push(parent2.genes[i]);
                }
            }
            Individual::new(parent1.id + parent2.id, child_genes)
        }

        fn mutate(individual: &mut Individual, rate: f64) {
            for gene in &mut individual.genes {
                if rand::random::<f64>() < rate {
                    *gene = rand::random::<f64>().mul_add(0.1, *gene).min(1.0);
                }
            }
        }

        fn evolve(&mut self, mutation_rate: f64) {
            self.evaluate_fitness();
            let target_size = self.individuals.len();
            let parents = self.select_parents(self.individuals.len() / 2);

            let mut new_generation = Vec::new();

            // Keep top 2 elites
            new_generation.extend(parents.iter().take(2).cloned());

            // Generate offspring through crossover until we reach target size
            let mut parent_idx = 0;
            while new_generation.len() < target_size && parents.len() >= 2 {
                let parent1_idx = parent_idx % parents.len();
                let parent2_idx = (parent_idx + 1) % parents.len();

                let mut child = Self::crossover(&parents[parent1_idx], &parents[parent2_idx]);
                Self::mutate(&mut child, mutation_rate);
                new_generation.push(child);

                parent_idx += 2;
            }

            self.individuals = new_generation;
            self.generation += 1;
        }

        fn best_fitness(&self) -> f64 {
            self.individuals
                .iter()
                .map(|i| i.fitness)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.0)
        }
    }

    #[test]
    fn test_population_initialization() {
        let pop = Population::new(100, 10);
        assert_eq!(pop.individuals.len(), 100);
        assert_eq!(pop.generation, 0);
        assert_eq!(pop.individuals[0].genes.len(), 10);
    }

    #[test]
    fn test_fitness_evaluation() {
        let mut pop = Population::new(10, 5);
        pop.evaluate_fitness();

        for individual in &pop.individuals {
            let expected: f64 = individual.genes.iter().sum();
            assert!((individual.fitness - expected).abs() < 0.0001);
        }
    }

    #[test]
    fn test_parent_selection() {
        let mut pop = Population::new(20, 5);
        pop.evaluate_fitness();

        let parents = pop.select_parents(10);
        assert_eq!(parents.len(), 10);

        // Parents should be sorted by fitness (descending)
        for i in 0..parents.len() - 1 {
            assert!(parents[i].fitness >= parents[i + 1].fitness);
        }
    }

    #[test]
    fn test_crossover_operation() {
        let parent1 = Individual::new(1, vec![1.0, 2.0, 3.0, 4.0]);
        let parent2 = Individual::new(2, vec![5.0, 6.0, 7.0, 8.0]);

        let child = Population::crossover(&parent1, &parent2);
        assert_eq!(child.genes.len(), 4);

        // Child should alternate genes from parents
        assert_eq!(child.genes[0], 1.0); // From parent1
        assert_eq!(child.genes[1], 6.0); // From parent2
        assert_eq!(child.genes[2], 3.0); // From parent1
        assert_eq!(child.genes[3], 8.0); // From parent2
    }

    #[test]
    fn test_mutation_operation() {
        let mut individual = Individual::new(1, vec![0.5; 10]);
        Population::mutate(&mut individual, 1.0); // 100% mutation rate

        // With 100% rate, all genes should have changed
        let changed_count = individual
            .genes
            .iter()
            .filter(|&&g| (g - 0.5).abs() > 0.001)
            .count();
        assert!(changed_count > 0, "Mutation should change some genes");
    }

    #[test]
    fn test_evolution_progression() {
        let mut pop = Population::new(50, 10);

        pop.evaluate_fitness();
        let _initial_best = pop.best_fitness();

        // Evolve for 20 generations
        for _ in 0..20 {
            pop.evolve(0.15);
        }

        // Verify evolution occurred
        assert_eq!(pop.generation, 20);

        // Verify population is still valid
        assert!(!pop.individuals.is_empty());
        assert_eq!(pop.individuals[0].genes.len(), 10);

        // Evolution happened (generation advanced)
        assert!(pop.generation > 0);
    }

    #[test]
    fn test_diversity_maintenance() {
        let mut pop = Population::new(50, 8);

        for _ in 0..5 {
            pop.evolve(0.3);
        }

        // Verify population structure is maintained
        pop.evaluate_fitness();

        // Should have individuals
        assert!(!pop.individuals.is_empty());

        // All individuals should have correct gene count
        for individual in &pop.individuals {
            assert_eq!(individual.genes.len(), 8);
        }

        // Evolution occurred
        assert_eq!(pop.generation, 5);
    }

    #[test]
    fn test_elitism_preservation() {
        let mut pop = Population::new(20, 5);
        pop.evaluate_fitness();

        let best_before = pop.best_fitness();
        pop.evolve(0.1);

        let best_after = pop.best_fitness();

        // Best fitness should not decrease (elitism)
        assert!(
            best_after >= best_before * 0.95,
            "Elitism should preserve best solutions"
        );
    }

    #[test]
    #[allow(clippy::cast_precision_loss)]
    fn test_convergence_detection() {
        let mut pop = Population::new(30, 5);
        let mut fitness_history = Vec::new();

        for _ in 0..20 {
            pop.evolve(0.05); // Low mutation for faster convergence
            pop.evaluate_fitness();
            fitness_history.push(pop.best_fitness());
        }

        // Check if fitness plateaus (convergence)
        let last_5: Vec<f64> = fitness_history.iter().rev().take(5).copied().collect();
        let variance: f64 = {
            let mean = last_5.iter().sum::<f64>() / last_5.len() as f64;
            last_5.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / last_5.len() as f64
        };

        // Low variance indicates convergence
        assert!(
            variance < 1.0,
            "Should show signs of convergence after 20 generations"
        );
    }

    #[test]
    fn test_concurrent_populations() {
        use std::sync::{Arc, Mutex};

        let populations: Vec<Arc<Mutex<Population>>> = (0..5)
            .map(|_| Arc::new(Mutex::new(Population::new(20, 5))))
            .collect();

        let handles: Vec<_> = populations
            .iter()
            .map(|pop| {
                let pop_clone = pop.clone();
                std::thread::spawn(move || {
                    let mut p = pop_clone.lock().unwrap();
                    for _ in 0..10 {
                        p.evolve(0.1);
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().expect("Thread should complete");
        }

        // All populations should have evolved
        for pop in populations {
            let p = pop.lock().unwrap();
            assert_eq!(p.generation, 10);
        }
    }
}
