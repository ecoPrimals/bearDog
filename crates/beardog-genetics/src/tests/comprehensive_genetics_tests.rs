// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive genetics tests exercising production genetic-algorithm and key-exchange code.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use crate::genetics::advanced_algorithms::{
    CrossoverEngine, EvolutionConfig, FitnessEvaluator, GeneticEvolutionEngine, GeneticIndividual,
    GeneticSignature, MutationEngine, PopulationManager, SelectionEngine,
};
use crate::genetics::key_exchange::{GeneticKeyExchange, KeyExchangeConfig};

#[cfg(test)]
mod genetic_algorithm_tests {
    use super::*;

    #[test]
    fn test_population_initialization() {
        let engine = GeneticEvolutionEngine::new(EvolutionConfig {
            population_size: 6,
            ..Default::default()
        });
        let population = engine
            .initialize_population(vec![GeneticSignature::default(); 6])
            .expect("initialize population");
        assert_eq!(population.len(), 6);
        assert!(population.iter().all(|ind| !ind.id.is_empty()));
    }

    #[test]
    fn test_fitness_calculation() {
        let evaluator = FitnessEvaluator::new();
        let mut high = GeneticIndividual::default();
        high.genetic_signature.quality_score = 0.9;
        high.performance_metrics.authorization_success_rate = 1.0;

        let mut low = GeneticIndividual::default();
        low.genetic_signature.quality_score = 0.2;
        low.performance_metrics.authorization_success_rate = 0.0;

        let high_fitness = evaluator.evaluate_fitness(&high).expect("high fitness");
        let low_fitness = evaluator.evaluate_fitness(&low).expect("low fitness");
        assert!(high_fitness > low_fitness);
        assert!(high_fitness <= 1.0);
    }

    #[test]
    fn test_selection_algorithm() {
        let engine = SelectionEngine::new();
        let population = vec![
            GeneticIndividual {
                fitness_score: 0.9,
                ..Default::default()
            },
            GeneticIndividual {
                fitness_score: 0.1,
                ..Default::default()
            },
        ];
        let survivors = engine
            .select_survivors(&population, 1)
            .expect("select survivors");
        assert_eq!(survivors, vec![0]);
    }

    #[test]
    fn test_crossover_operation() {
        let engine = CrossoverEngine::new();
        let parent_a = GeneticIndividual::default();
        let parent_b = GeneticIndividual::default();
        let parent_a_id = parent_a.id.clone();
        let parent_b_id = parent_b.id.clone();
        let children = engine
            .crossover(&parent_a, &parent_b)
            .expect("crossover produces offspring");
        assert_eq!(children.len(), 2);
        assert_eq!(children[0].parent_ids, vec![parent_a_id, parent_b_id]);
        assert_ne!(children[0].id, parent_a.id);
    }

    #[test]
    fn test_mutation_operation() {
        let engine = MutationEngine::new();
        let mut individual = GeneticIndividual::default();
        individual.fitness_score = 0.5;
        let changed = engine
            .apply_mutation(&mut individual, 1.0)
            .expect("mutation at rate 1.0");
        assert!(changed);
        assert!(!individual.mutation_history.is_empty());
    }

    #[test]
    fn test_generation_evolution() {
        let engine = GeneticEvolutionEngine::new(EvolutionConfig {
            population_size: 4,
            ..Default::default()
        });
        engine
            .initialize_population(vec![GeneticSignature::default(); 4])
            .expect("seed population");
        let snapshot = engine.evolve_generation().expect("evolve one generation");
        assert_eq!(snapshot.population_size, 4);
        assert!(snapshot.average_fitness >= 0.0);
        assert!(snapshot.best_fitness >= 0.0);
    }

    #[test]
    fn test_elitism_strategy() {
        let engine = GeneticEvolutionEngine::new(EvolutionConfig {
            population_size: 4,
            elitism_percentage: 0.5,
            ..Default::default()
        });
        engine
            .initialize_population(vec![GeneticSignature::default(); 4])
            .expect("seed population");
        let snapshot = engine.evolve_generation().expect("evolve with elitism");
        assert_eq!(snapshot.population_size, 4);
        assert!(snapshot.best_fitness >= snapshot.average_fitness);
    }
}

#[cfg(test)]
mod key_evolution_tests {
    use super::*;

    fn exchange() -> GeneticKeyExchange {
        GeneticKeyExchange::new(KeyExchangeConfig::default()).expect("key exchange engine")
    }

    #[test]
    fn test_key_generation() {
        let exchange = exchange();
        let key = exchange
            .create_delegated_key("peer-a", 3600, &["encrypt".to_string()])
            .expect("delegated key");
        assert!(!key.key_id.is_empty());
        assert_eq!(key.public_key.len(), 32);
    }

    #[test]
    fn test_key_rotation() {
        let exchange = exchange();
        let delegated = exchange
            .create_delegated_key("peer-b", 3600, &[])
            .expect("delegated key");
        let first = exchange
            .perform_key_exchange("peer-b", &delegated)
            .expect("first exchange");
        let second = exchange
            .perform_key_exchange("peer-b", &delegated)
            .expect("second exchange");
        assert!(second.lineage.generation > first.lineage.generation);
    }

    #[test]
    fn test_key_strength_evaluation() {
        let exchange = exchange();
        let key = exchange
            .create_delegated_key("peer-c", 1800, &["sign".to_string()])
            .expect("delegated key");
        assert!(key.lineage_info.genetic_fingerprint.iter().any(|b| *b != 0));
        assert!(key.constraints.time_constraint.is_some());
    }

    #[test]
    fn test_key_history_tracking() {
        let exchange = exchange();
        let delegated = exchange
            .create_delegated_key("peer-d", 3600, &[])
            .expect("delegated key");
        let result = exchange
            .perform_key_exchange("peer-d", &delegated)
            .expect("exchange");
        assert_eq!(result.lineage.peer_id, "peer-d");
        assert!(!result.lineage.lineage_id.is_empty());
        assert!(
            !exchange
                .should_evolve("peer-d")
                .expect("lineage lookup after exchange")
        );
    }
}

#[cfg(test)]
mod mutation_strategy_tests {
    use super::*;

    #[test]
    fn test_mutation_rate() {
        let engine = MutationEngine::new();
        let mut individual = GeneticIndividual::default();
        individual.fitness_score = 0.42;

        let unchanged = engine
            .apply_mutation(&mut individual, 0.0)
            .expect("zero mutation rate");
        assert!(!unchanged);
        assert!(individual.mutation_history.is_empty());
    }
}

#[cfg(test)]
mod fitness_evaluation_tests {
    use super::*;

    #[test]
    fn test_entropy_measurement() {
        let signature = GeneticSignature::default();
        assert!(signature.quality_score > 0.0 && signature.quality_score <= 1.0);
    }

    #[test]
    fn test_key_quality_score() {
        let evaluator = FitnessEvaluator::new();
        let mut individual = GeneticIndividual::default();
        individual.genetic_signature.quality_score = 0.75;
        let fitness = evaluator
            .evaluate_fitness(&individual)
            .expect("quality fitness");
        assert!(fitness >= 0.75);
    }

    #[test]
    fn test_security_strength_metric() {
        let evaluator = FitnessEvaluator::new();
        let mut strong = GeneticIndividual::default();
        strong.performance_metrics.security_rating = 1.0;
        strong.performance_metrics.authorization_success_rate = 1.0;

        let mut weak = GeneticIndividual::default();
        weak.performance_metrics.security_rating = 0.1;
        weak.performance_metrics.authorization_success_rate = 0.0;

        let strong_fitness = evaluator.evaluate_fitness(&strong).expect("strong fitness");
        let weak_fitness = evaluator.evaluate_fitness(&weak).expect("weak fitness");
        assert!(strong_fitness > weak_fitness);
    }

    #[test]
    fn test_diversity_measurement() {
        let engine = GeneticEvolutionEngine::new(EvolutionConfig {
            population_size: 5,
            ..Default::default()
        });
        engine
            .initialize_population(vec![GeneticSignature::default(); 5])
            .expect("seed population");
        let snapshot = engine.evolve_generation().expect("evolve");
        assert!(snapshot.diversity_index >= 0.0 && snapshot.diversity_index <= 1.0);
    }

    #[test]
    fn test_fitness_comparison() {
        let evaluator = FitnessEvaluator::new();
        let a = GeneticIndividual {
            genetic_signature: GeneticSignature {
                quality_score: 0.8,
                ..Default::default()
            },
            ..Default::default()
        };
        let b = GeneticIndividual {
            genetic_signature: GeneticSignature {
                quality_score: 0.3,
                ..Default::default()
            },
            ..Default::default()
        };
        let fitness_a = evaluator.evaluate_fitness(&a).expect("fitness a");
        let fitness_b = evaluator.evaluate_fitness(&b).expect("fitness b");
        assert!(fitness_a > fitness_b);
    }
}

#[cfg(test)]
mod population_management_tests {
    use super::*;

    #[test]
    fn test_population_size_control() {
        let manager = PopulationManager::new(8);
        assert_eq!(manager.population_size(), 0);
        manager.set_population(vec![GeneticIndividual::default(); 8]);
        assert_eq!(manager.population_size(), 8);
        assert_eq!(manager.get_population().len(), 8);
    }

    #[test]
    fn test_diversity_maintenance() {
        let engine = GeneticEvolutionEngine::new(EvolutionConfig {
            population_size: 6,
            diversity_preservation: true,
            ..Default::default()
        });
        engine
            .initialize_population(vec![GeneticSignature::default(); 6])
            .expect("seed population");
        let snapshot = engine.evolve_generation().expect("evolve");
        assert!(snapshot.diversity_index > 0.0);
    }
}

#[cfg(test)]
mod genetics_integration_tests {
    use super::*;
    use crate::assess_genetics_health;

    #[test]
    fn test_performance_metrics() {
        let mut individual = GeneticIndividual::default();
        individual.performance_metrics.operation_efficiency = 0.95;
        individual.performance_metrics.resource_utilization = 0.85;
        let fitness = FitnessEvaluator::new()
            .evaluate_fitness(&individual)
            .expect("metrics contribute to fitness");
        assert!(fitness > 0.0);
    }

    #[test]
    fn test_end_to_end_evolution() {
        let engine = GeneticEvolutionEngine::new(EvolutionConfig {
            population_size: 4,
            max_generations: 3,
            ..Default::default()
        });
        engine
            .initialize_population(vec![GeneticSignature::default(); 4])
            .expect("seed population");

        let mut last_generation = None;
        for _ in 0..3 {
            let snapshot = engine.evolve_generation().expect("generation");
            if let Some(prev) = last_generation {
                assert!(snapshot.generation > prev);
            }
            last_generation = Some(snapshot.generation);
        }

        let best = engine
            .get_best_individual()
            .expect("best individual after evolution");
        assert!(best.fitness_score >= 0.0);
        assert_eq!(assess_genetics_health(), "Healthy");
    }
}
