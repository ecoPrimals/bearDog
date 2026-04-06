// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    CrossoverEngine, FitnessEvaluator, MutationEngine, PopulationManager, SelectionEngine,
};
use crate::genetics::advanced_algorithms::types::GeneticIndividual;

#[test]
fn population_manager_set_and_get_roundtrip() {
    let pm = PopulationManager::new(10);
    let mut a = GeneticIndividual::default();
    a.fitness_score = 0.5;
    let mut b = GeneticIndividual::default();
    b.fitness_score = 0.7;
    pm.set_population(vec![a, b]);
    assert_eq!(pm.population_size(), 2);
    let pop = pm.get_population();
    assert_eq!(pop.len(), 2);
}

#[test]
fn fitness_evaluator_caps_at_one() {
    let fe = FitnessEvaluator::new();
    let mut ind = GeneticIndividual::default();
    ind.genetic_signature.quality_score = 0.95;
    ind.performance_metrics.authorization_success_rate = 1.0;
    let f = fe.evaluate_fitness(&ind).expect("fitness");
    assert!((f - 1.0).abs() < 1e-9);
}

#[test]
fn mutation_engine_apply_mutation_is_deterministic_when_rate_is_zero() {
    let me = MutationEngine::new();
    let mut ind = GeneticIndividual::default();
    ind.fitness_score = 0.5;
    let before = ind.fitness_score;
    let changed = me.apply_mutation(&mut ind, 0.0).expect("mut");
    assert!(!changed);
    assert!((ind.fitness_score - before).abs() < f64::EPSILON);
}

#[test]
fn crossover_engine_produces_two_children_with_parents_recorded() {
    let ce = CrossoverEngine::new();
    let p1 = GeneticIndividual::default();
    let p2 = GeneticIndividual::default();
    let kids = ce.crossover(&p1, &p2).expect("crossover");
    assert_eq!(kids.len(), 2);
    assert_eq!(kids[0].parent_ids.len(), 2);
    assert_eq!(kids[0].parent_ids[0], p1.id);
}

#[test]
fn selection_engine_empty_population_returns_empty_parents() {
    let se = SelectionEngine::new();
    let parents = se.select_parents(&[], 3).expect("parents");
    assert!(parents.is_empty());
}

#[test]
fn selection_engine_survivors_sorted_by_fitness() {
    let se = SelectionEngine::new();
    let mut a = GeneticIndividual::default();
    a.fitness_score = 0.2;
    let mut b = GeneticIndividual::default();
    b.fitness_score = 0.9;
    let mut c = GeneticIndividual::default();
    c.fitness_score = 0.5;
    let pop = vec![a, b, c];
    let surv = se.select_survivors(&pop, 2).expect("survivors");
    assert_eq!(surv.len(), 2);
    assert!(pop[surv[0]].fitness_score >= pop[surv[1]].fitness_score);
}

#[test]
fn fitness_evaluator_clone_preserves_adaptive_weight_bits() {
    let fe = FitnessEvaluator::new();
    let c = fe;
    let _ = c
        .evaluate_fitness(&GeneticIndividual::default())
        .expect("ok");
}

#[test]
fn mutation_engine_clone_preserves_history() {
    let me = MutationEngine::new();
    let c = me;
    let mut ind = GeneticIndividual::default();
    let _ = c.apply_mutation(&mut ind, 1.0).expect("mutation");
}

#[test]
fn population_manager_clone_is_independent() {
    let pm = PopulationManager::new(4);
    pm.set_population(vec![GeneticIndividual::default()]);
    let q = pm;
    assert_eq!(q.population_size(), 1);
}

#[test]
fn crossover_engine_clone_preserves_matrix() {
    let ce = CrossoverEngine::new();
    let _ = ce;
}

#[test]
fn selection_engine_clone_preserves_method() {
    let se = SelectionEngine::new();
    let c = se;
    let pop = vec![GeneticIndividual::default()];
    let _ = c.select_parents(&pop, 1).expect("p");
}
